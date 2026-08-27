use crate::ast::graph::ProjectGraph;
use std::fmt::Write;

use crate::ast::{FileAnalysisResult, RouteInfo};

pub struct MarkdownContentBuilder {
    pub package_info: String,
    pub all_routes: Vec<RouteInfo>,
    pub file_results: Vec<FileAnalysisResult>,
    pub graph_data: Option<ProjectGraph>,
}

impl MarkdownContentBuilder {
    pub fn build_markdown_prompt(&self) -> String {
        let mut out = String::new();

        if !self.package_info.is_empty() {
            out.push_str(&self.package_info);
            out.push_str("\n---\n\n");
        } else {
            out.push_str("# Project Technical Context & LLM Prompt\n\n");
        };

        // Project File Structure & Dependency Graph
        if let Some(ref graph) = self.graph_data {
            out.push_str("## Project File Structure & PEG Graph\n\n");
            out.push_str("```\n");
            out.push_str(&graph.ascii_tree);
            out.push_str("```\n\n");

            if !graph.dependency_graph.is_empty() {
                out.push_str("### Module Dependency Graph\n\n");
                out.push_str(&graph.dependency_graph);
                out.push('\n');
            }
            out.push_str("---\n\n");
        }

        //  Backend Route Table
        if !self.all_routes.is_empty() {
            out.push_str("## Backend API Route Table\n\n");
            out.push_str("| Framework | Method | Path | Handler | File Location |\n");
            out.push_str("| :--- | :--- | :--- | :--- | :--- |\n");
            for r in &self.all_routes {
                let _ = writeln!(
                    out,
                    "| **{}** | `{}` | `{}` | `{}` | `{}:{}` |",
                    r.framework, r.method, r.path, r.handler_name, r.file_path, r.line_number
                );
            }
            out.push_str("\n---\n\n");
        }

        //  React Component Detailed Explanations
        let has_react = self
            .file_results
            .iter()
            .any(|f| !f.react_components.is_empty());
        if has_react {
            out.push_str("## React Component Architecture & Explanations\n\n");
            for file in &self.file_results {
                for comp in &file.react_components {
                    out.push_str(&comp.detailed_explanation);
                    if !comp.rendered_elements.is_empty() {
                        out.push_str(&format!(
                            "- **Rendered JSX Tree**: `<{}>` \n\n",
                            comp.rendered_elements.join(">, <")
                        ));
                    }
                }
            }
            out.push_str("---\n\n");
        }

        //  Source Code Section
        out.push_str("## AST-Pruned Source Code Repository\n\n");
        out.push_str("> Note: Tailwind classNames and static styles have been pruned according to mode to maximize token efficiency.\n\n");

        for file in &self.file_results {
            out.push_str(&format!("### File: `{}`\n\n", file.relative_path));

            let lang = match file.file_path.extension().and_then(|e| e.to_str()) {
                Some("ts") | Some("tsx") => "typescript",
                Some("jsx") => "jsx",
                _ => "javascript",
            };

            out.push_str(&format!("```{}\n", lang));
            out.push_str(&file.processed_content);
            out.push_str("\n```\n\n");
        }

        out
    }
}
