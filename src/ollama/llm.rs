use anyhow::{Context, Result, bail};
use serde::Serialize;
use std::sync::Arc;

use crate::{
    UraiContext,
    ollama::{OllamaResponse, OllamaUrai, cache::init_cache},
};

#[derive(Serialize)]
pub struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    system: String,
}

const FUNCTION_SYSTEM_PROMPT: &str = "# Role
You are an elite, highly precise Program Semantic Analyst. Your sole objective is to summarize the core behavioral purpose of any given source code function block in 1 concise sentence.

# Instructions
1. Silent Reasoning: Mentally trace logic and output ONLY a 1-sentence explanation of what the function does.
2. Length: Exactly 1 sentence.
3. No Preamble: Do NOT say 'This function...', 'Here is...', or wrap output in code blocks. Start directly with the active description verb (e.g. 'Calculates user permissions based on role matrix.').";

const TAILWIND_SYSTEM_PROMPT: &str = "# Role
You are a UI Design & CSS Expert. Your objective is to summarize complex Tailwind CSS utility class names into a concise single-line description of the visual layout and component style.

# Instructions
1. Length: Under 12 words (e.g., 'Flex card layout with dark mode background and rounded borders').
2. Output ONLY the short natural language style description. Do NOT include quotes, backticks, or intro text.";

impl OllamaUrai {
    pub fn new(ctx: Arc<UraiContext>) -> Result<Self> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;

        let cache_folder = &ctx.ollama_endpoint.ollama_cache_folder;
        let cache = init_cache(cache_folder, &rt)?;

        Ok(Self { ctx, cache, rt })
    }

    pub fn summarize_function(&self, fn_name: &str, fn_code: &str) -> Result<String> {
        let prompt = format!("Function Name: {}\nCode:\n{}", fn_name, fn_code);
        self.generate_completion(&prompt, FUNCTION_SYSTEM_PROMPT)
    }

    pub fn summarize_tailwind_classes(&self, class_names: &str) -> Result<String> {
        let prompt = format!("Tailwind CSS Classes:\n{}", class_names);
        self.generate_completion(&prompt, TAILWIND_SYSTEM_PROMPT)
    }

    fn generate_completion(&self, prompt: &str, system: &str) -> Result<String> {
        let endpoint_url = match &self.ctx.ollama_endpoint.ollama_endpoint {
            Some(ep) => ep.as_str(),
            None => bail!("Ollama endpoint not configured"),
        };

        let model_name = self
            .ctx
            .ollama_endpoint
            .ollama_model_name
            .as_deref()
            .unwrap_or("gemma2");

        let cache_key = self.generate_cache_key(&prompt.to_string());
        if let Ok(cached_response) = self.get_cache_res(&cache_key) {
            if cached_response.response != "URAI_OLLAMA_CACHE_MISS" {
                return Ok(cached_response.response);
            }
        }

        let payload = OllamaRequest {
            model: model_name.to_string(),
            prompt: prompt.to_string(),
            stream: false,
            system: system.to_string(),
        };

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        let response = client
            .post(format!("{}/api/generate", endpoint_url))
            .json(&payload)
            .send()
            .context("Failed to send HTTP request to Ollama endpoint")?;

        if !response.status().is_success() {
            bail!("Ollama endpoint returned HTTP status {}", response.status());
        }

        let res_body: OllamaResponse = response
            .json()
            .context("Failed to parse JSON response from Ollama")?;

        let cleaned = res_body.response.trim().to_string();
        self.insert_res_cache(
            cache_key,
            OllamaResponse {
                response: cleaned.clone(),
            },
        );

        Ok(cleaned)
    }
}
