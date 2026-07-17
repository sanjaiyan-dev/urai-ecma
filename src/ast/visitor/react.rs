use swc_ecma_ast::{Callee, Expr, ImportSpecifier, ModuleExportName, Pat};

use swc_ecma_visit::{Visit, VisitMut};

use crate::ast::visitor::{VisitorCode, resolve_ts_type};
pub struct ReactCodeVisitor {}

impl Visit for VisitorCode {
    fn visit_import_decl(&mut self, node: &swc_ecma_ast::ImportDecl) {
        if node.src.value == "react" {
            for specifier in &node.specifiers {
                if let ImportSpecifier::Named(named) = specifier {
                    let import_as_name = named.local.sym.to_string();

                    let imported_original_name = match &named.imported {
                        Some(ModuleExportName::Ident(ident)) => ident.sym.to_string(),
                        Some(ModuleExportName::Str(str_lit)) => {
                            str_lit.value.to_string_lossy().into_owned()
                        }
                        None => import_as_name.clone(),
                    };

                    match imported_original_name.as_str() {
                        "useState" => {
                            self.local_to_canonical_track_imports
                                .insert(import_as_name, "useState".to_string());
                        }
                        "startTransition" => {
                            self.local_to_canonical_track_imports
                                .insert(import_as_name, "startTransition".to_string());
                        }
                        _ => {}
                    };
                }
            }
        }
    }
    fn visit_var_decl(&mut self, decls: &swc_ecma_ast::VarDecl) {
        for decl in &decls.decls {
            if let Pat::Array(array_pat) = &decl.name {
                if let Some(Some(init_expr)) = &decl.init.as_deref().map(|e| match e {
                    Expr::Call(call) => Some(call),
                    _ => None,
                }) {
                    if let Callee::Expr(callee_expr) = &init_expr.callee {
                        if let Expr::Ident(callee_id) = &**callee_expr {
                            if callee_id.sym == "useState" {
                                let state_name = match array_pat.elems.get(0) {
                                    Some(Some(Pat::Ident(bind_id))) => bind_id.id.sym.to_string(),
                                    _ => "unknown".to_string(),
                                };
                                let setter_name = match array_pat.elems.get(1) {
                                    Some(Some(Pat::Ident(bind_id))) => bind_id.id.sym.to_string(),
                                    _ => "unknown".to_string(),
                                };

                                let state_type = if let Some(type_args) = &init_expr.type_args {
                                    if let Some(first_type) = type_args.params.get(0) {
                                        resolve_ts_type(&**first_type) // <--- Calls our recursive helper!
                                    } else {
                                        "any".to_string()
                                    }
                                } else {
                                    "any".to_string()
                                };
                            }
                        }
                    }
                }
            }
        }
    }
}
