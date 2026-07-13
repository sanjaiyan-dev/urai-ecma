use std::sync::Arc;

use crate::UraiContext;

pub mod cache;
pub mod llm;
pub struct OllamaUrai {
    ctx: Arc<UraiContext>,
}
