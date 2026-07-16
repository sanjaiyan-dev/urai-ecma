use std::sync::Arc;

use crate::UraiContext;

pub mod package_json;
pub mod parser;
pub mod visitor;

pub struct PackageJsonUrai {
    ctx: Arc<UraiContext>,
}

pub struct CodeAnalyze {
    ctx: Arc<UraiContext>,
    file_name: String,
    content: String,
}
