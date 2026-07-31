use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use swc_common::SourceMap;
use swc_common::sync::Lrc;
use swc_ecma_codegen::{Emitter, text_writer::JsWriter};
use swc_ecma_visit::VisitMutWith;
use swc_ecma_visit::VisitWith;

use crate::UraiContext;
use crate::ast::PackageJsonUrai;
use crate::ast::graph::generate_project_graph;
use crate::ast::parser::parse_file;
use crate::ast::visitor::function_summarizer::FunctionSummarizerVisitor;
use crate::ast::visitor::react::{ReactComponentAnalyzer, ReactJsxPruner};
use crate::ast::visitor::routes::RouteVisitor;
use crate::ast::{FileAnalysisResult, RouteInfo};
use crate::markdown::MarkdownUrai;
use crate::markdown::markdown_content::MarkdownContentBuilder;
use crate::ollama::OllamaUrai;

pub fn run_project_analysis(ctx: Arc<UraiContext>) -> Result<()> {
    let mut file_results = Vec::new();
    let mut all_routes: Vec<RouteInfo> = Vec::new();

    let ollama_client = if ctx.ollama_endpoint.ollama_endpoint.is_some() {
        OllamaUrai::new(ctx.clone()).ok()
    } else {
        None
    };
    println!("San Running");
    let files = collect_source_files(&ctx.input_project)?;

    println!("🔍 Found {} source file(s) for analysis.", files.len());

    for file_path in &files {
        let rel_path = file_path
            .strip_prefix(&ctx.input_project)
            .unwrap_or(file_path)
            .display()
            .to_string();

        let raw_content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        let parse_res = parse_file(&raw_content, file_path.to_str().unwrap_or_default());
        match parse_res {
            Ok((mut module, _comments, cm)) => {
                // Route Extraction
                if ctx.generate_route_table {
                    let mut route_vis = RouteVisitor::new(rel_path.as_str(), &cm);
                    module.visit_with(&mut route_vis);
                    all_routes.extend(route_vis.routes);
                }

                // React Component Analysis
                let mut react_components = Vec::new();
                if ctx.analyze_react_components {
                    let mut react_analyzer = ReactComponentAnalyzer::new(&cm);
                    module.visit_with(&mut react_analyzer);
                    react_components = react_analyzer.components;
                }

                // Function Summaries
                let mut fn_summaries = Vec::new();
                if ctx.summarize_functions {
                    let mut fn_vis =
                        FunctionSummarizerVisitor::new(&cm, &raw_content, ollama_client.as_ref());
                    module.visit_with(&mut fn_vis);
                    fn_summaries = fn_vis.summaries;
                }

                // JSX Pruning
                if ctx.analyze_react_components {
                    let mut pruner = ReactJsxPruner {
                        mode: ctx.tailwind_mode,
                        ollama: ollama_client.as_ref(),
                    };
                    module.visit_mut_with(&mut pruner);
                }

                let processed_code = emit_code(&module, cm);

                file_results.push(FileAnalysisResult {
                    file_path: file_path.clone(),
                    relative_path: rel_path,
                    raw_content,
                    processed_content: processed_code,
                    routes: Vec::new(),
                    react_components,
                    function_summaries: fn_summaries,
                });
            }
            Err(e) => {
                eprintln!(
                    "⚠️  Warning: Failed to parse JS/TS file '{}': {}",
                    file_path.display(),
                    e
                );
                file_results.push(FileAnalysisResult {
                    file_path: file_path.clone(),
                    relative_path: rel_path,
                    raw_content: raw_content.clone(),
                    processed_content: raw_content,
                    routes: Vec::new(),
                    react_components: Vec::new(),
                    function_summaries: Vec::new(),
                });
            }
        }
    }

    // Package.json parsing
    let pkg_json_urai = PackageJsonUrai::new(ctx.clone());
    let pkg_markdown = pkg_json_urai.parse_package_json().unwrap_or_default();

    // File structure & Graph
    let graph_data = if ctx.generate_file_graph {
        generate_project_graph(&ctx.input_project).ok()
    } else {
        None
    };

    // Assemble final Markdown output
    let markdown_builder = MarkdownContentBuilder {
        ctx: ctx.clone(),
        package_info: pkg_markdown,
        all_routes,
        file_results,
        graph_data,
    };

    let final_markdown = markdown_builder.build_markdown_prompt();

    let writer = MarkdownUrai::new(ctx.clone());
    writer.clear_markdown_content()?;
    writer.markdown_content_writer(&final_markdown)?;

    Ok(())
}

fn collect_source_files(path: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if path.is_file() {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if matches!(ext, "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs") {
            files.push(path.to_path_buf());
        }
        println!("{:?}", &files);

        return Ok(files);
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let p = entry.path();
            let name = p.file_name().unwrap_or_default().to_string_lossy();
            println!("{}", p.display());
            if name.starts_with('.')
                || name == "node_modules"
                || name == "dist"
                || name == "build"
                || name == "target"
            {
                continue;
            }
            println!("{}", p.display());
            if p.is_dir() {
                let mut sub = collect_source_files(&p)?;
                files.append(&mut sub);
            } else {
                let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
                if matches!(ext, "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs") {
                    files.push(p);
                }
            }
        }
    }

    Ok(files)
}

fn emit_code(module: &swc_ecma_ast::Module, cm: Lrc<SourceMap>) -> String {
    let mut buf = Vec::new();
    {
        let writer = JsWriter::new(cm.clone(), "\n", &mut buf, None);
        let mut emitter = Emitter {
            cfg: Default::default(),
            cm,
            comments: None,
            wr: writer,
        };
        let _ = emitter.emit_module(module);
    }
    String::from_utf8_lossy(&buf).to_string()
}
