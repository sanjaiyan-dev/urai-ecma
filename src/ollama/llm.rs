use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    UraiContext,
    ollama::{OllamaResponse, OllamaUrai, cache::init_cache},
};

pub struct ProgramConciseInfoParams {
    ollama_endpoint: String,
    program_code: String,
    program_lang: String,
    netowrk_reqwest: &'static reqwest::blocking::Client,
    model_name: String,
}

#[derive(Serialize)]
pub struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    system: &'static str,
}

const SYSTEM_PROMPT: &str = "# Role
You are an elite, highly precise Program Semantic Analyst. Your sole objective is to summarize the core behavioral purpose of any given source code block with high semantic accuracy.

# Core Instructions
1. **Deep Code Analysis**: Internally analyze the code's control flow, input/output structures, logic branches, and side effects step-by-step. Trace the execution path mentally to determine its deep functional intent before formulating your response.
2. **Silent Reasoning Constraint**: You must perform your step-by-step logical analysis entirely in your internal, silent thinking process. Do NOT output your step-by-step thinking, code blocks, or intermediate analytical steps to the final output.
3. **Length Constraint**: Your final output must be exactly one to two sentences.
4. **Output Restrictions**: 
   - Output ONLY the natural language explanation of the code.
   - Do NOT include any preamble, introductory text, or conversational filler (such as 'This code...', 'Here is...', or 'The provided function...').
   - Do NOT wrap your output in markdown code blocks or backticks.
   - Start immediately with the first word of the explanation.";

impl OllamaUrai {
    pub fn new(ctx: Arc<UraiContext>) -> anyhow::Result<Self> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;

        let cache_folder = &ctx.ollama_endpoint.ollama_cache_folder;
        let cache = init_cache(cache_folder, &rt)?;

        Ok(Self { ctx, cache, rt })
    }
    fn summarize_code_block(&self, params: ProgramConciseInfoParams) -> Result<String> {
        let ctx = &self.ctx;

        match ctx.ollama_endpoint.ollama_endpoint {
            None => bail!("No need Ollama"),
            Some(_) => {
                println!("Ollama Process is begins ")
            }
        }

        let ollama_model_name = ctx
            .ollama_endpoint
            .ollama_model_name
            .clone()
            .unwrap_or("".to_string());
        let ollama_endpoint_url = ctx
            .ollama_endpoint
            .ollama_endpoint
            .as_deref()
            .unwrap_or("http://localhost:11234");

        let cache_key = self.generate_cache_key(&params.program_code);

        let cached_response = match self.get_cache_res(&cache_key) {
            Ok(res) => res,
            Err(_) => OllamaResponse {
                response: "URAI_OLLAMA_CACHE_MISS".to_string(),
            },
        };

        if cached_response.response != "URAI_OLLAMA_CACHE_MISS" {
            return Ok(cached_response.response);
        }

        let payload = OllamaRequest {
            model: ollama_model_name,
            stream: false,
            prompt: params.program_code,
            system: SYSTEM_PROMPT,
        };

        let response = params
            .netowrk_reqwest
            .post(format!("{}/api/generate", ollama_endpoint_url))
            .json(&payload)
            .send()
            .context("Failed to connect to ollama server")?;

        if !response.status().is_success() {
            bail!("Ollama server returned an error: {}", response.status());
        }

        let res_body: OllamaResponse = response
            .json()
            .context("Failed to parse the response JSON from Ollama")?;
        self.insert_res_cache(cache_key, res_body.clone());
        Ok(res_body.response.trim().to_string())
    }
}
