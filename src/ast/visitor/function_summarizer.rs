use crate::ast::FunctionSummary;
use crate::ollama::OllamaUrai;
use jsdoc::{Input as JsDocInput, parse as parse_jsdoc};
use swc_common::SourceMap;
use swc_common::comments::{Comment, Comments, SingleThreadedComments};
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct FunctionSummarizerVisitor<'a> {
    pub cm: &'a SourceMap,
    pub file_content: &'a str,
    pub ollama: Option<&'a OllamaUrai>,
    pub summaries: Vec<FunctionSummary>,
    pub comments: SingleThreadedComments,
}

fn extract_jsdoc_description(comment: &Comment) -> Option<String> {
    if let Ok((_, jsdoc_ast)) = parse_jsdoc(JsDocInput::from(comment)) {
        for item in jsdoc_ast.tags {
            if let jsdoc::ast::Tag::Description(desc_tag) = item.tag {
                let description = desc_tag.text.value.to_string();
                if !description.trim().is_empty() {
                    return Some(description.trim().to_string());
                }
            }
        }
    }

    
    for line in comment.text.lines() {
        let cleaned = line.trim().trim_start_matches('*').trim();
        if let Some(desc) = cleaned.strip_prefix("@description") {
            let desc_trimmed = desc.trim();
            if !desc_trimmed.is_empty() {
                return Some(desc_trimmed.to_string());
            }
        }
    }

    None
}
impl<'a> FunctionSummarizerVisitor<'a> {
    pub fn new(
        cm: &'a SourceMap,
        file_content: &'a str,
        ollama: Option<&'a OllamaUrai>,
        comments: SingleThreadedComments,
    ) -> Self {
        Self {
            cm,
            file_content,
            ollama,
            summaries: Vec::new(),
            comments: comments,
        }
    }
    fn find_description_comment(&self, fn_decl: &FnDecl) -> Option<String> {
        let leading_comments = self
            .comments
            .get_leading(fn_decl.function.span.lo)
            .or_else(|| self.comments.get_leading(fn_decl.ident.span.lo))?;
        print!("Comment Detected {:?}", { &leading_comments });

        for comment in leading_comments {
            if let Some(desc) = extract_jsdoc_description(&comment) {
                return Some(desc);
            }
        }

        None
    }
}

impl<'a> Visit for FunctionSummarizerVisitor<'a> {
    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        let fn_name = fn_decl.ident.sym.to_string();
        let lo_pos = self.cm.lookup_char_pos(fn_decl.function.span.lo);
        let hi_pos = self.cm.lookup_char_pos(fn_decl.function.span.hi);
        let line_count = hi_pos.line.saturating_sub(lo_pos.line);

        if line_count >= 4 {
            let existing_description = self.find_description_comment(fn_decl);
            let fn_snippet = extract_snippet(self.file_content, lo_pos.line, hi_pos.line);

            let summary = if let Some(description) = existing_description {
                println!("{description}");
                description
            } else if let Some(ollama) = self.ollama {
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
