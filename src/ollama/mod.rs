use std::sync::Arc;

use foyer::HybridCache;
use serde::{Deserialize, Serialize};

use crate::UraiContext;

pub mod cache;
pub mod llm;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OllamaResponse {
    pub response: String,
}

pub struct OllamaUrai {
    pub ctx: Arc<UraiContext>,
    pub cache: HybridCache<String, OllamaResponse>,
    pub rt: tokio::runtime::Runtime,
}
