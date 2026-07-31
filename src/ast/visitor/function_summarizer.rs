use swc_common::comments::{Comment, Comments, SingleThreadedComments};
use swc_common::{BytePos, SourceMap};
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

use jsdoc::{
    Input as JsDocInput,
    ast::{Tag, TagItem},
    parse as parse_jsdoc,
};

use crate::ast::FunctionSummary;
use crate::ollama::OllamaUrai;

pub struct FunctionSummarizerVisitor<'a> {
    pub cm: &'a SourceMap,
    pub file_content: &'a str,
    pub ollama: Option<&'a OllamaUrai>,
    pub summaries: Vec<FunctionSummary>,
    pub comments: SingleThreadedComments,
    pub line_threshold: usize,
    pub current_export_lo: Option<BytePos>,
}

impl<'a> FunctionSummarizerVisitor<'a> {
    pub fn new(
        cm: &'a SourceMap,
        file_content: &'a str,
        ollama: Option<&'a OllamaUrai>,
        comments: SingleThreadedComments,
        line_threshold: usize,
    ) -> Self {
        Self {
            cm,
            file_content,
            ollama,
            summaries: Vec::new(),
            comments,
            line_threshold,
            current_export_lo: None,
        }
    }
}

impl<'a> Visit for FunctionSummarizerVisitor<'a> {
    fn visit_export_decl(&mut self, export_decl: &ExportDecl) {
        let prev = self.current_export_lo;
        self.current_export_lo = Some(export_decl.span.lo);
        export_decl.visit_children_with(self);
        self.current_export_lo = prev;
    }

    fn visit_export_default_decl(&mut self, export_default: &ExportDefaultDecl) {
        let prev = self.current_export_lo;
        self.current_export_lo = Some(export_default.span.lo);
        export_default.visit_children_with(self);
        self.current_export_lo = prev;
    }
    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        let fn_name = fn_decl.ident.sym.to_string();
        let lo_pos = self.cm.lookup_char_pos(fn_decl.function.span.lo);
        let hi_pos = self.cm.lookup_char_pos(fn_decl.function.span.hi);
        let line_count = hi_pos.line.saturating_sub(lo_pos.line) + 1;

        // First Preference: Check for JSDoc comments (@description, @param, @return)
        let jsdoc_summary = self.extract_jsdoc_summary(fn_decl);

        let summary = if let Some(jsdoc_text) = jsdoc_summary {
            // Found JSDoc documentation -> Skip Ollama execution
            jsdoc_text
        } else if line_count >= self.line_threshold {
            // Fallback to Ollama if no JSDoc is present AND line count >= threshold
            if let Some(ollama) = self.ollama {
                let fn_snippet = extract_snippet(self.file_content, lo_pos.line, hi_pos.line);
                ollama
                    .summarize_function(&fn_name, &fn_snippet)
                    .unwrap_or_else(|_| format!("Executes logic for function {}", fn_name))
            } else {
                format!("Executes logic for function {}", fn_name)
            }
        } else {
            // Fallback for short functions without JSDoc
            format!("Executes logic for function {}", fn_name)
        };

        self.summaries.push(FunctionSummary {
            function_name: fn_name,
            line_number: lo_pos.line,
            concise_summary: summary,
        });

        fn_decl.visit_children_with(self);
    }
}

impl<'a> FunctionSummarizerVisitor<'a> {
    /// Extract and format JSDoc annotations (@description, @param, @return)
    fn extract_jsdoc_summary(&self, fn_decl: &FnDecl) -> Option<String> {
        let fn_lo = fn_decl.function.span.lo;
        let ident_lo = fn_decl.ident.span.lo;

        let positions_to_check = [self.current_export_lo, Some(fn_lo), Some(ident_lo)];
        for pos in positions_to_check.into_iter().flatten() {
            if let Some(leading_comments) = self.comments.get_leading(pos) {
                for comment in &leading_comments {
                    if let Some(info) = parse_jsdoc_comment(comment) {
                        return Some(format_jsdoc_summary(&info));
                    }
                }
            }
        }

        let (leading_map, _trailing_map) = self.comments.borrow_all();

        for (pos, comment_list) in leading_map.iter() {
            let distance = pos.0.abs_diff(fn_lo.0);

            if distance <= 300 {
                for cmt in comment_list {
                    if let Some(info) = parse_jsdoc_comment(cmt) {
                        return Some(format_jsdoc_summary(&info));
                    }
                }
            }
        }

        None
    }
}

struct JsDocInfo {
    description: Option<String>,
    params: Vec<String>,
    returns: Option<String>,
}

/// Parses JSDoc annotations with `jsdoc` crate & robust string fallback
fn parse_jsdoc_comment(raw_comment: &Comment) -> Option<JsDocInfo> {
    if let Ok((_, ast)) = parse_jsdoc(JsDocInput::from(raw_comment)) {
        let mut description_lines = Vec::new();
        let mut params = Vec::new();
        let mut returns = None;

        if !ast.description.value.trim().is_empty() {
            description_lines.push(ast.description.value.trim().to_string());
        }

        for item in ast.tags {
            match item.tag {
                Tag::Description(d) => {
                    let desc = d.text.value.trim();
                    if !desc.is_empty() && !description_lines.contains(&desc.to_string()) {
                        description_lines.push(desc.to_string());
                    }
                }
                Tag::Parameter(p) => {
                    let name_str = p.name.as_ref().map(|n| n.value.as_str()).unwrap_or("param");
                    let type_str =
                        p.ty.as_ref()
                            .map(|t| format!(" ({})", t.value))
                            .unwrap_or_default();
                    let desc_str = if !p.desc.value.as_str().trim().is_empty() {
                        format!(" - {}", p.desc.value.as_str().trim())
                    } else {
                        String::new()
                    };

                    params.push(format!("{}{}{}", name_str, type_str, desc_str));
                }
                Tag::Return(r) => {
                    let ret_type =
                        r.ty.as_ref()
                            .map(|t| t.value.to_string())
                            .unwrap_or_default();
                    let ret_str = format!("{} {}", ret_type, r.description.value)
                        .trim()
                        .to_string();
                    if !ret_str.is_empty() {
                        returns = Some(ret_str);
                    }
                }
                _ => {}
            }
        }

        if !description_lines.is_empty() || !params.is_empty() || returns.is_some() {
            return Some(JsDocInfo {
                description: if description_lines.is_empty() {
                    None
                } else {
                    Some(description_lines.join(" "))
                },
                params,
                returns,
            });
        }
    }

    // Manual line-by-line fallback parser for non-standard or line comments
    let mut description_lines = Vec::new();
    let mut params = Vec::new();
    let mut returns = None;

    for line in raw_comment.text.lines() {
        let trimmed = line
            .trim()
            .trim_start_matches("/*")
            .trim_start_matches("//")
            .trim_end_matches("*/")
            .trim();

        let cleaned = trimmed.trim_start_matches('*').trim();
        if cleaned.is_empty() {
            continue;
        }

        if let Some(desc) = cleaned.strip_prefix("@description") {
            let desc_text = desc.trim().to_string();
            if !desc_text.is_empty() && !description_lines.contains(&desc_text) {
                description_lines.push(desc_text);
            }
        } else if let Some(param) = cleaned.strip_prefix("@param") {
            let param_text = param.trim();
            if !param_text.is_empty() {
                params.push(param_text.to_string());
            }
        } else if let Some(ret) = cleaned
            .strip_prefix("@return")
            .or_else(|| cleaned.strip_prefix("@returns"))
        {
            let ret_text = ret.trim();
            if !ret_text.is_empty() {
                returns = Some(ret_text.to_string());
            }
        } else if !cleaned.starts_with('@') {
            description_lines.push(cleaned.to_string());
        }
    }

    if description_lines.is_empty() && params.is_empty() && returns.is_none() {
        return None;
    }

    Some(JsDocInfo {
        description: if description_lines.is_empty() {
            None
        } else {
            Some(description_lines.join(" "))
        },
        params,
        returns,
    })
}

fn format_jsdoc_summary(info: &JsDocInfo) -> String {
    let mut parts = Vec::new();

    if let Some(desc) = &info.description {
        parts.push(desc.clone());
    }

    if !info.params.is_empty() {
        parts.push(format!("Params: {}", info.params.join("; ")));
    }

    if let Some(ret) = &info.returns {
        parts.push(format!("Returns: {}", ret));
    }

    parts.join(" | ")
}

fn extract_snippet(file_content: &str, start_line: usize, end_line: usize) -> String {
    file_content
        .lines()
        .skip(start_line.saturating_sub(1))
        .take(end_line.saturating_sub(start_line) + 1)
        .collect::<Vec<_>>()
        .join("\n")
}
