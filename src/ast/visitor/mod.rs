use std::sync::Arc;

use swc_ecma_ast::Stmt;

use crate::UraiContext;

pub mod react;

pub struct VisitorCode {
    pub ctx: Arc<UraiContext>,
    pub extracted_handlers: Vec<Stmt>,
    pub counter: usize,
}
