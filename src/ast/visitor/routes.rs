use std::path::Path;
use swc_common::SourceMap;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

use crate::ast::RouteInfo;

pub struct RouteVisitor<'a> {
    pub file_path: &'a str,
    pub cm: &'a SourceMap,
    pub routes: Vec<RouteInfo>,
    pub nest_controller_prefix: Option<String>,
}

impl<'a> RouteVisitor<'a> {
    pub fn new(file_path: &'a str, cm: &'a SourceMap) -> Self {
        Self {
            file_path,
            cm,
            routes: Vec::new(),
            nest_controller_prefix: None,
        }
    }
}

impl<'a> Visit for RouteVisitor<'a> {
    fn visit_export_decl(&mut self, export: &ExportDecl) {
        // Next.js App Router API route check (export async function GET(req))
        if (self.file_path.contains("route.ts")
            || self.file_path.contains("route.js")
            || self.file_path.contains("/api/"))
            && let Decl::Fn(fn_decl) = &export.decl {
                let fn_name = fn_decl.ident.sym.to_string();
                let upper_name = fn_name.to_uppercase();
                if matches!(
                    upper_name.as_str(),
                    "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" | "OPTIONS"
                ) {
                    let derived_path = derive_nextjs_route_path(self.file_path);
                    let line = self.cm.lookup_char_pos(fn_decl.function.span.lo).line;
                    self.routes.push(RouteInfo {
                        framework: "Next.js".to_string(),
                        method: upper_name,
                        path: derived_path,
                        handler_name: fn_name,
                        file_path: self.file_path.to_string(),
                        line_number: line,
                    });
                }
            }
        export.visit_children_with(self);
    }

    fn visit_class_decl(&mut self, class_decl: &ClassDecl) {
        // NestJS Controller check
        let mut controller_prefix = None;
        for decorator in &class_decl.class.decorators {
            if let Expr::Call(call) = &*decorator.expr
                && let Callee::Expr(callee_expr) = &call.callee
                    && let Expr::Ident(ident) = &**callee_expr
                        && ident.sym == "Controller" {
                            let prefix = if let Some(arg) = call.args.first() {
                                match &*arg.expr {
                                    Expr::Lit(Lit::Str(s)) => {
                                        s.value.as_str().unwrap_or_default().to_string()
                                    }
                                    _ => "".to_string(),
                                }
                            } else {
                                "".to_string()
                            };
                            controller_prefix = Some(prefix);
                        }
        }

        if let Some(ref base_path) = controller_prefix {
            let normalized_base = if base_path.starts_with('/') || base_path.is_empty() {
                base_path.clone()
            } else {
                format!("/{}", base_path)
            };

            for member in &class_decl.class.body {
                if let ClassMember::Method(method) = member
                    && let PropName::Ident(method_ident) = &method.key {
                        for decorator in &method.function.decorators {
                            if let Expr::Call(call) = &*decorator.expr
                                && let Callee::Expr(callee_expr) = &call.callee
                                    && let Expr::Ident(ident) = &**callee_expr {
                                        let http_method = ident.sym.to_string().to_uppercase();
                                        if matches!(
                                            http_method.as_str(),
                                            "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "ALL"
                                        ) {
                                            let subpath = if let Some(arg) = call.args.first() {
                                                match &*arg.expr {
                                                    Expr::Lit(Lit::Str(s)) => s
                                                        .value
                                                        .as_str()
                                                        .unwrap_or_default()
                                                        .to_string(),
                                                    _ => "".to_string(),
                                                }
                                            } else {
                                                "".to_string()
                                            };

                                            let full_path = format!(
                                                "{}/{}",
                                                normalized_base.trim_end_matches('/'),
                                                subpath.trim_start_matches('/')
                                            );
                                            let line = self.cm.lookup_char_pos(method.span.lo).line;

                                            self.routes.push(RouteInfo {
                                                framework: "NestJS".to_string(),
                                                method: http_method,
                                                path: if full_path.is_empty() {
                                                    "/".to_string()
                                                } else {
                                                    full_path
                                                },
                                                handler_name: format!(
                                                    "{}::{}",
                                                    class_decl.ident.sym, method_ident.sym
                                                ),
                                                file_path: self.file_path.to_string(),
                                                line_number: line,
                                            });
                                        }
                                    }
                        }
                    }
            }
        }

        class_decl.visit_children_with(self);
    }

    fn visit_call_expr(&mut self, call: &CallExpr) {
        // Express & Fastify endpoint check
        if let Callee::Expr(callee_expr) = &call.callee
            && let Expr::Member(member) = &**callee_expr
                && let MemberProp::Ident(prop_ident) = &member.prop {
                    let method_name = prop_ident.sym.to_string().to_lowercase();
                    if matches!(
                        method_name.as_str(),
                        "get" | "post" | "put" | "delete" | "patch" | "all" | "route"
                    ) {
                        let is_express_or_fastify = if let Expr::Ident(obj_ident) = &*member.obj {
                            let obj_name = obj_ident.sym.as_str();
                            matches!(
                                obj_name,
                                "app"
                                    | "router"
                                    | "express"
                                    | "fastify"
                                    | "server"
                                    | "instance"
                                    | "api"
                            )
                        } else {
                            false
                        };

                        if is_express_or_fastify && let Some(first_arg) = call.args.first() {
                            let path = match &*first_arg.expr {
                                Expr::Lit(Lit::Str(s)) => {
                                    s.value.as_str().unwrap_or_default().to_string()
                                }
                                Expr::Tpl(tpl) => {
                                    let raw: String = tpl
                                        .quasis
                                        .iter()
                                        .map(|q| q.raw.as_str().to_string())
                                        .collect::<Vec<_>>()
                                        .join("${...}");
                                    raw
                                }
                                _ => "".to_string(),
                            };

                            if !path.is_empty() {
                                let framework = if let Expr::Ident(obj) = &*member.obj {
                                    if obj.sym.contains("fastify") {
                                        "Fastify"
                                    } else {
                                        "Express"
                                    }
                                } else {
                                    "Express"
                                };

                                let line = self.cm.lookup_char_pos(call.span.lo).line;
                                self.routes.push(RouteInfo {
                                    framework: framework.to_string(),
                                    method: method_name.to_uppercase(),
                                    path,
                                    handler_name: "anonymous_handler".to_string(),
                                    file_path: self.file_path.to_string(),
                                    line_number: line,
                                });
                            }
                        }
                    }
                }

        call.visit_children_with(self);
    }
}

fn derive_nextjs_route_path(file_path: &str) -> String {
    let p = Path::new(file_path);
    let mut components = Vec::new();
    let mut recording = false;

    for comp in p.components() {
        let name = comp.as_os_str().to_string_lossy();
        if name == "app" || name == "pages" || name == "api" {
            recording = true;
            if name == "api" {
                components.push(name.to_string());
            }
            continue;
        }
        if recording {
            if name == "route.ts" || name == "route.js" || name == "index.ts" || name == "index.js"
            {
                continue;
            }
            let clean_name = name.trim_end_matches(".ts").trim_end_matches(".js");
            components.push(clean_name.to_string());
        }
    }

    if components.is_empty() {
        "/api".to_string()
    } else {
        format!("/{}", components.join("/"))
    }
}
