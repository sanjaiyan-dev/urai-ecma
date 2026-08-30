use anyhow::{Result, bail};
use ignore::WalkBuilder;
use petgraph::graph::UnGraph;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::ast::analyze::is_supported_extension;

pub struct ProjectGraph {
    pub ascii_tree: String,
    pub dependency_graph: String,
}

pub fn generate_project_graph(root_path: &Path) -> Result<ProjectGraph> {
    let mut tree_out = String::new();
    tree_out.push_str(&format!(
        "📁 {}\n",
        root_path.file_name().unwrap_or_default().to_string_lossy()
    ));

    if root_path.is_dir() {
        build_ascii_tree(root_path, "", &mut tree_out)?;
    } else {
        tree_out.push_str(&format!(" └── 📄 {}\n", root_path.display()));
    }

    let dep_graph_out = build_petgraph_dependencies(root_path)?;

    Ok(ProjectGraph {
        ascii_tree: tree_out,
        dependency_graph: dep_graph_out,
    })
}

fn build_ascii_tree(dir: &Path, prefix: &str, out: &mut String) -> Result<()> {
    let entries = match fs::read_dir(dir) {
        Ok(read) => read,
        Err(_) => return Ok(()),
    };

    let mut paths: Vec<PathBuf> = entries
        .filter_map(|e| e.ok().map(|entry| entry.path()))
        .filter(|p| {
            let name = p.file_name().unwrap_or_default().to_string_lossy();
            !name.starts_with('.')
                && name != "node_modules"
                && name != "dist"
                && name != "build"
                && name != "target"
        })
        .collect();

    paths.sort();

    let count = &paths.len();
    for (i, path) in paths.iter().enumerate() {
        let is_last = (i == count - 1) && (i < 1000);
        let connector = if is_last { "└── " } else { "├── " };
        let name = path.file_name().unwrap_or_default().to_string_lossy();

        if path.is_dir() {
            out.push_str(&format!("{}{}{}/\n", prefix, connector, name));
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            build_ascii_tree(path, &new_prefix, out)?;
        } else {
            out.push_str(&format!("{}{}{}\n", prefix, connector, name));
        }
    }

    Ok(())
}

fn build_petgraph_dependencies(root_path: &Path) -> Result<String> {
    let mut files = Vec::new();
    collect_js_ts_files(root_path, &mut files)?;

    if files.is_empty() {
        return Ok("No JS/TS source files found for dependency graph.".to_string());
    }

    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut node_indices = HashMap::new();

    for file in &files {
        let rel_path = file
            .strip_prefix(root_path)
            .unwrap_or(file)
            .display()
            .to_string();
        let idx = graph.add_node(rel_path.clone());
        node_indices.insert(rel_path, idx);
    }

    let mut dep_summary = String::with_capacity(25 + (files.len() * 12));
    dep_summary.push_str("```mermaid\ngraph LR;\n");

    for file in &files {
        let source_rel = file
            .strip_prefix(root_path)
            .unwrap_or(file)
            .display()
            .to_string();
        if let Ok(content) = fs::read_to_string(file) {
            for line in content.lines() {
                let trimmed = line.trim();
                if (trimmed.starts_with("import ") || trimmed.starts_with("export "))
                    && trimmed.contains("from ")
                    && let Some(import_spec) = trimmed.split("from ").nth(1)
                {
                    let clean_spec = import_spec
                        .trim()
                        .trim_matches(';')
                        .trim_matches('\'')
                        .trim_matches('"');
                    if clean_spec.starts_with('.') {
                        dep_summary.push_str(&format!("    {} --> {};\n", source_rel, clean_spec));
                    }
                }
            }
        }
    }

    dep_summary.push_str("```\n");
    Ok(dep_summary)
}

fn collect_js_ts_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    if dir.is_file() {
        let ext = dir.extension().and_then(|e| e.to_str()).unwrap_or("");
        if matches!(ext, "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs") {
            files.push(dir.to_path_buf());
        }
        return Ok(());
    }
    if dir.is_dir() {
        let current_folder_files: Vec<PathBuf> = WalkBuilder::new(dir)
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
                entry.file_type().is_some_and(|ft| ft.is_file())
                    && is_supported_extension(entry.path())
            })
            .map(|entry| entry.into_path())
            .collect();
        files.extend(current_folder_files);

        Ok(())
    } else {
        bail!("This is not directory")
    }
}
