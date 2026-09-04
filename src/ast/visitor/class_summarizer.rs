use swc_common::comments::{Comment, Comments, SingleThreadedComments};
use swc_common::{BytePos, DUMMY_SP, SourceMap};
use swc_ecma_ast::*;
use swc_ecma_visit::{VisitMut, VisitMutWith};

use jsdoc::{Input as JsDocInput, ast::Tag, parse as parse_jsdoc};

use crate::ast::FunctionSummary;
use crate::ollama::OllamaUrai;

pub struct ClassMethodSummarizerVisitor<'a> {
    pub cm: &'a SourceMap,
    pub file_content: &'a str,
    pub ollama: Option<&'a OllamaUrai>,
    pub summaries: Vec<FunctionSummary>,
    pub comments: SingleThreadedComments,
    pub line_threshold: usize,
    pub current_export_lo: Option<BytePos>,
    pub fn_name_stack: Vec<String>,
    pub class_name_stack: Vec<String>,
    pub sumarize_class_method_enabled: bool,
}

impl<'a> ClassMethodSummarizerVisitor<'a> {
    pub fn new(
        cm: &'a SourceMap,
        file_content: &'a str,
        ollama: Option<&'a OllamaUrai>,
        comments: SingleThreadedComments,
        line_threshold: usize,
        sumarize_class_method_enabled: bool,
    ) -> Self {
        Self {
            cm,
            file_content,
            ollama,
            summaries: Vec::new(),
            comments,
            line_threshold,
            current_export_lo: None,
            fn_name_stack: Vec::new(),
            class_name_stack: Vec::new(),
            sumarize_class_method_enabled,
        }
    }
}

fn is_structural_stub_stmt(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Decl(Decl::Fn(_)) => true,

        Stmt::Decl(Decl::Var(var_decl)) => var_decl.decls.iter().any(|decl| {
            if let Some(init) = &decl.init {
                matches!(**init, Expr::Arrow(_) | Expr::Fn(_))
            } else {
                false
            }
        }),

        Stmt::Expr(expr_stmt) => {
            if let Expr::Call(call_expr) = &*expr_stmt.expr
                && let Callee::Expr(callee_expr) = &call_expr.callee
                && let Expr::Ident(ident) = &**callee_expr
            {
                let name = ident.sym.as_ref();

                return name.starts_with("use")
                    || name == "setTimeout"
                    || name == "setInterval"
                    || name.contains("addEventListener")
                    || name.contains("requestIdleCallback");
            }
            false
        }

        Stmt::Return(ret_stmt) => {
            if let Some(arg) = &ret_stmt.arg {
                matches!(
                    &**arg,
                    Expr::JSXElement(_) | Expr::JSXFragment(_) | Expr::Paren(_)
                )
            } else {
                false
            }
        }

        _ => false,
    }
}

impl<'a> VisitMut for ClassMethodSummarizerVisitor<'a> {
    fn visit_mut_export_decl(&mut self, export_decl: &mut ExportDecl) {
        let prev = self.current_export_lo;
        self.current_export_lo = Some(export_decl.span.lo);
        export_decl.visit_mut_children_with(self);
        self.current_export_lo = prev;
    }

    fn visit_mut_export_default_decl(&mut self, export_default: &mut ExportDefaultDecl) {
        let prev = self.current_export_lo;
        self.current_export_lo = Some(export_default.span.lo);
        export_default.visit_mut_children_with(self);
        self.current_export_lo = prev;
    }

    fn visit_mut_class_decl(&mut self, class_decl: &mut ClassDecl) {
        let class_name = class_decl.ident.sym.to_string();
        self.class_name_stack.push(class_name);
        class_decl.visit_mut_children_with(self);
        self.class_name_stack.pop();
    }

    fn visit_mut_class_expr(&mut self, class_expr: &mut ClassExpr) {
        let class_name = class_expr
            .ident
            .as_ref()
            .map(|id| id.sym.to_string())
            .unwrap_or_else(|| "anonymous_class".to_string());
        self.class_name_stack.push(class_name);
        class_expr.visit_mut_children_with(self);
        self.class_name_stack.pop();
    }

    fn visit_mut_constructor(&mut self, constructor: &mut Constructor) {
        constructor.visit_mut_children_with(self);

        let class_name = self
            .class_name_stack
            .last()
            .cloned()
            .unwrap_or_else(|| "UnknownClass".to_string());
        let full_name = format!("{}.constructor", class_name);

        let lo_pos = self.cm.lookup_char_pos(constructor.span.lo);
        let hi_pos = self.cm.lookup_char_pos(constructor.span.hi);
        let line_count = hi_pos.line.saturating_sub(lo_pos.line) + 1;

        let jsdoc_summary = self.extract_jsdoc_summary_by_pos(constructor.span.lo, None);

        let summary = if let Some(jsdoc_text) = jsdoc_summary {
            jsdoc_text
        } else if line_count >= self.line_threshold && self.sumarize_class_method_enabled {
            if let Some(ollama) = self.ollama {
                let fn_snippet = extract_snippet(self.file_content, lo_pos.line, hi_pos.line);
                ollama
                    .summarize_function(&full_name, &fn_snippet)
                    .unwrap_or_else(|_| format!("Executes logic for constructor {}", full_name))
            } else {
                format!("Executes logic for constructor {}", full_name)
            }
        } else {
            format!("Executes logic for constructor {}", full_name)
        };

        if let Some(function_body) = &mut constructor.body {
            function_body.stmts.retain(is_structural_stub_stmt);
        }

        self.summaries.push(FunctionSummary {
            function_name: full_name,
            line_number: lo_pos.line,
            concise_summary: summary,
        });
    }

    fn visit_mut_class_method(&mut self, class_method: &mut ClassMethod) {
        class_method.visit_mut_children_with(self);

        let method_name = match &class_method.key {
            PropName::Ident(ident) => ident.sym.to_string(),
            PropName::Str(str_name) => str_name.value.to_string_lossy().to_string(),
            _ => "unknown_method".to_string(),
        };

        let class_name = self
            .class_name_stack
            .last()
            .cloned()
            .unwrap_or_else(|| "UnknownClass".to_string());
        let full_name = format!("{}.{}", class_name, method_name);

        let lo_pos = self.cm.lookup_char_pos(class_method.function.span.lo);
        let hi_pos = self.cm.lookup_char_pos(class_method.function.span.hi);
        let line_count = hi_pos.line.saturating_sub(lo_pos.line) + 1;

        let jsdoc_summary = self.extract_jsdoc_summary_by_pos(
            class_method.function.span.lo,
            Some(class_method.span.lo),
        );

        let summary = if let Some(jsdoc_text) = jsdoc_summary {
            jsdoc_text
        } else if line_count >= self.line_threshold {
            if let Some(ollama) = self.ollama {
                let fn_snippet = extract_snippet(self.file_content, lo_pos.line, hi_pos.line);

                ollama
                    .summarize_function(&full_name, &fn_snippet)
                    .unwrap_or_else(|_| format!("Executes logic for method {}", full_name))
            } else {
                format!("Executes logic for method {}", full_name)
            }
        } else {
            format!("Executes logic for method {}", full_name)
        };

        if let Some(function_body) = class_method.function.body.as_mut() {
            function_body.stmts.retain(is_structural_stub_stmt);

            function_body.stmts.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: Box::new(Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    value: format!("/* {:?} */", summary).into(),
                    raw: None,
                }))),
            }));
        }

        self.summaries.push(FunctionSummary {
            function_name: full_name,
            line_number: lo_pos.line,
            concise_summary: summary,
        });
    }

    // -- PRIVATE METHODS (#myMethod) --
    fn visit_mut_private_method(&mut self, private_method: &mut PrivateMethod) {
        private_method.visit_mut_children_with(self);

        let method_name = private_method.key.name.to_string();
        let class_name = self
            .class_name_stack
            .last()
            .cloned()
            .unwrap_or_else(|| "UnknownClass".to_string());
        let full_name = format!("{}.#{}", class_name, method_name);

        let lo_pos = self.cm.lookup_char_pos(private_method.function.span.lo);
        let hi_pos = self.cm.lookup_char_pos(private_method.function.span.hi);
        let line_count = hi_pos.line.saturating_sub(lo_pos.line) + 1;

        let jsdoc_summary = self.extract_jsdoc_summary_by_pos(
            private_method.function.span.lo,
            Some(private_method.span.lo),
        );

        let summary = if let Some(jsdoc_text) = jsdoc_summary {
            jsdoc_text
        } else if line_count >= self.line_threshold {
            if let Some(ollama) = self.ollama {
                let fn_snippet = extract_snippet(self.file_content, lo_pos.line, hi_pos.line);
                ollama
                    .summarize_function(&full_name, &fn_snippet)
                    .unwrap_or_else(|_| format!("Executes logic for method {}", full_name))
            } else {
                format!("Executes logic for method {}", full_name)
            }
        } else {
            format!("Executes logic for method {}", full_name)
        };

        if let Some(function_body) = &mut private_method.function.body {
            function_body.stmts.retain(is_structural_stub_stmt);
        }

        if let Some(function_body) = private_method.function.body.as_mut() {
            function_body.stmts.retain(is_structural_stub_stmt);

            function_body.stmts.push(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: Box::new(Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    value: format!("/* {:?} */", summary).into(),
                    raw: None,
                }))),
            }));
        }

        self.summaries.push(FunctionSummary {
            function_name: full_name,
            line_number: lo_pos.line,
            concise_summary: summary,
        });
    }

    fn visit_mut_var_declarator(&mut self, decl: &mut VarDeclarator) {
        let mut pushed = false;
        if let Pat::Ident(binding_ident) = &decl.name {
            self.fn_name_stack.push(binding_ident.id.sym.to_string());
            pushed = true;
        }

        decl.visit_mut_children_with(self);

        if pushed {
            self.fn_name_stack.pop();
        }
    }
}

impl<'a> ClassMethodSummarizerVisitor<'a> {
    /// Extract and format JSDoc annotations by generic Spans, replacing strict FnDecl requirements
    fn extract_jsdoc_summary_by_pos(
        &self,
        span_lo: BytePos,
        ident_lo: Option<BytePos>,
    ) -> Option<String> {
        let positions_to_check = [self.current_export_lo, Some(span_lo), ident_lo];
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
            let distance = pos.0.abs_diff(span_lo.0);

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
