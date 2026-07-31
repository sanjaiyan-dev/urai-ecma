use swc_common::SourceMap;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

use crate::ast::FunctionSummary;
use crate::ollama::OllamaUrai;

pub struct FunctionSummarizerVisitor<'a> {
    pub cm: &'a SourceMap,
    pub file_content: &'a str,
    pub ollama: Option<&'a OllamaUrai>,
    pub summaries: Vec<FunctionSummary>,
}

impl<'a> FunctionSummarizerVisitor<'a> {
    pub fn new(cm: &'a SourceMap, file_content: &'a str, ollama: Option<&'a OllamaUrai>) -> Self {
        Self {
            cm,
            file_content,
            ollama,
            summaries: Vec::new(),
        }
    }
}

impl<'a> Visit for FunctionSummarizerVisitor<'a> {
    
    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        let fn_name = fn_decl.ident.sym.to_string();
        let lo_pos = self.cm.lookup_char_pos(fn_decl.function.span.lo);
        let hi_pos = self.cm.lookup_char_pos(fn_decl.function.span.hi);
        let line_count = hi_pos.line.saturating_sub(lo_pos.line);

        if line_count >= 4 {
            let fn_snippet = extract_snippet(self.file_content, lo_pos.line, hi_pos.line);
            let summary = if let Some(ollama) = self.ollama {
                ollama
                    .summarize_function(&fn_name, &fn_snippet)
                    .unwrap_or_else(|_| format!("Executes logic for function {}", fn_name))
            } else {
                format!("Executes logic for function {}", fn_name)
            };

            self.summaries.push(FunctionSummary {
                function_name: fn_name,
                line_number: lo_pos.line,
                concise_summary: summary,
            });
        }

        fn_decl.visit_children_with(self);
    }
}

fn extract_snippet(file_content: &str, start_line: usize, end_line: usize) -> String {
    file_content
        .lines()
        .skip(start_line.saturating_sub(1))
        .take(end_line.saturating_sub(start_line) + 1)
        .collect::<Vec<_>>()
        .join("\n")
}
