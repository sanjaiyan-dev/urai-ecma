use std::sync::Arc;

use foyer::HybridCache;
use serde::{Deserialize, Serialize};

use crate::UraiContext;

pub mod cache;
pub mod llm;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct OllamaResponse {
    response: String,
}

pub struct OllamaUrai {
    ctx: Arc<UraiContext>,
    cache: HybridCache<String, OllamaResponse>,
    rt: tokio::runtime::Runtime,
}
