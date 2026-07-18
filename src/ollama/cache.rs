use std::{path::Path, sync::Arc};

use foyer::{BlockEngineConfig, DeviceBuilder, FsDeviceBuilder, HybridCache, HybridCacheBuilder};
use sha2::{Digest, Sha512_256};

use crate::{UraiContext, ollama::OllamaUrai};

pub struct CacheEntry {
    pub modified_time: u64,
    pub size: u64,
    pub pruned_code: String,
    pub token_count: usize,
    pub ctx: Arc<UraiContext>,
}

impl OllamaUrai {
    pub fn new(ctx: Arc<UraiContext>) -> anyhow::Result<Self> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;

        let cache_folder = &ctx.ollama_endpoint.ollama_cache_folder;
        std::fs::create_dir_all(cache_folder)?;

        let cache = rt.block_on(async {
            let device = FsDeviceBuilder::new(cache_folder)
                .with_capacity(50 * 1024 * 1024)
                .build()?;

            let hybrid = HybridCacheBuilder::new()
                .memory(64 * 1024 * 1024) // 64 MB RAM cache
                .storage()
                .with_engine_config(BlockEngineConfig::new(device))
                .build()
                .await?;

            Ok::<_, anyhow::Error>(hybrid)
        })?;

        Ok(Self { ctx, cache, rt })
    }

    fn generate_cache_key(prompt: &String) -> String {
        let cache_key = Sha512_256::digest(prompt);

        format!("{:?}", cache_key)
    }

    pub fn fetch_from_cache(&self, cache_key: &String) {}
}

impl Drop for OllamaUrai {
    fn drop(&mut self) {
        let _ = self.rt.block_on(async {
            let _ = self.cache.close().await;
        });
    }
}
