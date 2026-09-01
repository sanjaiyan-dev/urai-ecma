use anyhow::Result;
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use swc_common::SourceMap;
use swc_common::sync::Lrc;
use swc_ecma_codegen::{Emitter, text_writer::JsWriter};
use swc_ecma_visit::{VisitMutWith, VisitWith};

use crate::UraiContext;
use crate::ast::PackageJsonUrai;
use crate::ast::graph::generate_project_graph;
use crate::ast::parser::parse_file;
use crate::ast::visitor::class_summarizer::ClassMethodSummarizerVisitor;
use crate::ast::visitor::function_summarizer::FunctionSummarizerVisitor;
use crate::ast::visitor::react::{ReactComponentAnalyzer, ReactJsxPruner};
use crate::ast::visitor::routes::RouteVisitor;
use crate::ast::{FileAnalysisResult, RouteInfo};
use crate::markdown::MarkdownUrai;
use crate::markdown::markdown_content::MarkdownContentBuilder;
use crate::ollama::OllamaUrai;

pub fn run_project_analysis(ctx: Arc<UraiContext>) -> Result<()> {
    let ollama_client = if ctx.ollama_endpoint.ollama_endpoint.is_some() {
        OllamaUrai::new(ctx.clone()).ok()
    } else {
        None
    };

    let files = collect_source_files(&ctx.input_project);
    println!("🔍 Found {} source file(s) for analysis.", files.len());

    let ollama_ref = ollama_client.as_ref();

    let processed_data: Vec<(FileAnalysisResult, Vec<RouteInfo>)> = files
        .par_iter()
        .filter_map(|file_path| {
            let rel_path = file_path
                .strip_prefix(&ctx.input_project)
                .unwrap_or(file_path)
                .display()
                .to_string();

            let raw_content = match fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!(
                        "⚠️ Warning: Failed to read file '{}': {}",
                        file_path.display(),
                        e
                    );
                    return None;
                }
            };

            let parse_res = parse_file(&raw_content, file_path.to_str().unwrap_or_default());

            match parse_res {
                Ok((mut module, comments, cm)) => {
                    let mut routes = Vec::new();

                    // Route Extraction
                    if ctx.generate_route_table {
                        let mut route_vis = RouteVisitor::new(rel_path.as_str(), &cm);
                        module.visit_with(&mut route_vis);
                        routes = route_vis.routes;
                    }

                    // React Component Analysis
                    let mut react_components = Vec::new();
                    if ctx.analyze_react_components {
                        let mut react_analyzer = ReactComponentAnalyzer::new();
                        module.visit_with(&mut react_analyzer);
                        react_components = react_analyzer.components;
                    }

                    // Function Summaries
                    if ctx.summarize_functions {
                        let mut fn_vis = FunctionSummarizerVisitor::new(
                            &cm,
                            &raw_content,
                            ollama_ref,
                            comments.to_owned(),
                            ctx.summarize_functions_threshold,
                        );
                        module.visit_mut_with(&mut fn_vis);
                    }

                    // Class Method Summaries
                    if ctx.summarize_functions {
                        let mut fn_vis = ClassMethodSummarizerVisitor::new(
                            &cm,
                            &raw_content,
                            ollama_ref,
                            comments,
                            ctx.summarize_functions_threshold,
                        );
                        module.visit_mut_with(&mut fn_vis);
                    }

                    // JSX Pruning
                    if ctx.analyze_react_components {
                        let mut pruner = ReactJsxPruner {
                            mode: ctx.tailwind_mode,
                            ollama: ollama_ref,
                            tailwind_threshold: ctx.tailwind_threshold,
                        };
                        module.visit_mut_with(&mut pruner);
                    }

                    let processed_code = emit_code(&module, cm);

                    let result = FileAnalysisResult {
                        file_path: file_path.clone(),
                        relative_path: rel_path,
                        raw_content,
                        processed_content: processed_code,
                        routes: Vec::new(),
                        react_components,
                    };

                    Some((result, routes))
                }
                Err(e) => {
                    eprintln!(
                        "⚠️  Warning: Failed to parse JS/TS file '{}': {}",
                        file_path.display(),
                        e
                    );
                    let result = FileAnalysisResult {
                        file_path: file_path.clone(),
                        relative_path: rel_path,
                        raw_content: raw_content.clone(),
                        processed_content: raw_content,
                        routes: Vec::new(),
                        react_components: Vec::new(),
                    };

                    Some((result, Vec::new()))
                }
            }
        })
        .collect();

    let mut file_results = Vec::with_capacity(processed_data.len());
    let mut all_routes = Vec::new();

    for (file_res, routes) in processed_data {
        file_results.push(file_res);
        all_routes.extend(routes);
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

/// Collects source files using `WalkBuilder` from the `ignore` crate.
/// Respects `.gitignore`, `.ignore`, `.git/info/exclude`, and skips hidden directories.
fn collect_source_files(path: &Path) -> Vec<PathBuf> {
    if path.is_file() {
        if is_supported_extension(path) {
            return vec![path.to_path_buf()];
        }
        return Vec::new();
    }

    WalkBuilder::new(path)
        .hidden(true)
        .git_ignore(true)
        .ignore(true)
        .filter_entry(|entry| {
            // Prune heavy directories early before recursing into them
            if let Some(file_name) = entry.file_name().to_str()
                && (file_name == "node_modules"
                    || file_name == "dist"
                    || file_name == "build"
                    || file_name == "target")
            {
                return false;
            }
            true
        })
        .build()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_some_and(|ft| ft.is_file()) && is_supported_extension(entry.path())
        })
        .map(|entry| entry.into_path())
        .collect()
}

pub fn is_supported_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| matches!(ext, "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs"))
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
