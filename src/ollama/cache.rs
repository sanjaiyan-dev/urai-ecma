use crate::ollama::{OllamaResponse, OllamaUrai};
use anyhow::{Result, bail};
use foyer::{BlockEngineConfig, DeviceBuilder, FsDeviceBuilder, HybridCache, HybridCacheBuilder};
use sha2::{Digest, Sha512_256};
use std::path::Path;

pub fn init_cache(
    cache_folder: &Path,
    rt: &tokio::runtime::Runtime,
) -> anyhow::Result<HybridCache<String, OllamaResponse>> {
    std::fs::create_dir_all(cache_folder)?;

    rt.block_on(async {
        let device = FsDeviceBuilder::new(cache_folder)
            .with_capacity(128 * 1024 * 1024)
            .build()?;

        let hybrid = HybridCacheBuilder::new()
            .memory(64 * 1024 * 1024)
            .storage()
            .with_engine_config(BlockEngineConfig::new(device))
            .build()
            .await?;

        Ok(hybrid)
    })
}

impl OllamaUrai {
    pub fn generate_cache_key(&self, prompt: &String) -> String {
        let cache_key = Sha512_256::digest(prompt.as_bytes());

        const_hex::encode(cache_key)
    }

    pub fn get_cache_res(&self, cache_key: &String) -> Result<OllamaResponse> {
        let cached_entry = self
            .rt
            .block_on(async { self.cache.get(cache_key).await })?;

        if let Some(entry) = cached_entry {
            return Ok(entry.value().clone());
        } else {
            bail!("Cache miss")
        }
    }

    pub fn insert_res_cache(&self, cache_key: String, response: OllamaResponse) {
        self.cache.insert(cache_key, response);
    }
}
impl Drop for OllamaUrai {
    fn drop(&mut self) {
        let _ = self.rt.block_on(async { self.cache.close().await });
    }
}
