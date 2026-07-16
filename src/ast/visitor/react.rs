use swc_ecma_ast::{Callee, Expr, Pat, TsEntityName, TsKeywordTypeKind, TsType};

use swc_ecma_visit::{Visit, VisitMut};

use crate::ast::CodeAnalyze;
pub struct ReactCodeVisitor {}
impl Visit for CodeAnalyze {
    fn visit_var_decl(&mut self, decls: &swc_ecma_ast::VarDecl) {
        for decl in &decls.decls {
            if let Pat::Array(array_pat) = &decl.name {
                if let Some(Some(init_expr)) = &decl.init.as_deref().map(|e| match e {
                    Expr::Call(call) => Some(call),
                    _ => None,
                }) {
                    if let Callee::Expr(callee_expr) = &init_expr.callee {
                        if let Expr::Ident(callee_id) = &**callee_expr {
                            if (callee_id.sym == "useState") {
                                let state_name = match array_pat.elems.get(0) {
                                    Some(Some(Pat::Ident(bind_id))) => bind_id.id.sym.to_string(),
                                    _ => "unknown".to_string(),
                                };
                                let setter_name = match array_pat.elems.get(1) {
                                    Some(Some(Pat::Ident(bind_id))) => bind_id.id.sym.to_string(),
                                    _ => "unknown".to_string(),
                                };

                                fn resolve_ts_type(ts_type: &TsType) -> String {
                                    match ts_type {
                                        TsType::TsKeywordType(keyword) => match keyword.kind {
                                            TsKeywordTypeKind::TsNumberKeyword
                                            | TsKeywordTypeKind::TsBigIntKeyword => {
                                                "number".to_string()
                                            }
                                            TsKeywordTypeKind::TsStringKeyword => {
                                                "string".to_string()
                                            }
                                            TsKeywordTypeKind::TsBooleanKeyword => {
                                                "boolean".to_string()
                                            }
                                            TsKeywordTypeKind::TsObjectKeyword => {
                                                "object".to_string()
                                            }
                                            TsKeywordTypeKind::TsAnyKeyword => "any".to_string(),
                                            TsKeywordTypeKind::TsVoidKeyword => "void".to_string(),
                                            _ => "unknown_keyword".to_string(),
                                        },

                                        TsType::TsArrayType(array_type) => {
                                            let element_type_str =
                                                resolve_ts_type(&array_type.elem_type);

                                            format!("{}[]", element_type_str)
                                        }

                                        TsType::TsTypeRef(type_ref) => {
                                            if let TsEntityName::Ident(ident) = &type_ref.type_name
                                            {
                                                ident.sym.to_string()
                                            } else {
                                                "object".to_string()
                                            }
                                        }

                                        TsType::TsUnionOrIntersectionType(union_or_intersect) => {
                                            if union_or_intersect.is_ts_union_type() {
                                                "union_type".to_string()
                                            } else {
                                                "intersection_type".to_string()
                                            }
                                        }

                                        _ => "object".to_string(),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
impl VisitMut for CodeAnalyze {}
