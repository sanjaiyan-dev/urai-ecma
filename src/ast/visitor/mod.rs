use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use swc_ecma_ast::{Stmt, TsEntityName, TsKeywordTypeKind, TsType};

use crate::UraiContext;

pub mod react;

pub fn resolve_ts_type(ts_type: &TsType) -> String {
    match ts_type {
        TsType::TsKeywordType(keyword) => match keyword.kind {
            TsKeywordTypeKind::TsStringKeyword => "string".to_string(),
            TsKeywordTypeKind::TsNumberKeyword => "number".to_string(),
            TsKeywordTypeKind::TsBigIntKeyword => "bigint".to_string(),
            TsKeywordTypeKind::TsBooleanKeyword => "boolean".to_string(),
            TsKeywordTypeKind::TsObjectKeyword => "object".to_string(),
            TsKeywordTypeKind::TsAnyKeyword => "any".to_string(),
            TsKeywordTypeKind::TsVoidKeyword => "void".to_string(),
            _ => "unknown_keyword".to_string(),
        },

        TsType::TsArrayType(array_type) => {
            let element_type_str = resolve_ts_type(&array_type.elem_type);

            format!("{}[]", element_type_str)
        }

        TsType::TsTypeRef(type_ref) => {
            if let TsEntityName::Ident(ident) = &type_ref.type_name {
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

pub struct VisitorCode {
    pub ctx: Arc<UraiContext>,
    pub extracted_handlers: Vec<Stmt>,
    pub counter: usize,

    pub local_to_canonical_track_imports: BTreeMap<String, String>,
}
