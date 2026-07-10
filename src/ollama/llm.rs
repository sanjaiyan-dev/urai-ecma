use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub struct ProgramConciseInfoParams {
    ollama_endpoint: String,
    program_code: String,
    program_lang: String,
    netowrk_reqwest: &'static reqwest::blocking::Client,
    model_name: String,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

pub fn program_concise_info(params: ProgramConciseInfoParams) -> Result<String> {
    let payload = OllamaRequest {
        model: params.model_name,
        stream: false,
        prompt: format!("{}", params.program_code),
    };
    let response = params
        .netowrk_reqwest
        .post(params.ollama_endpoint)
        .json(&payload)
        .send()
        .context("Failed to connect to ollama server")?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Ollama server returned an error: {}",
            response.status()
        ));
    }

    let res_body: OllamaResponse = response
        .json()
        .context("Failed to parse the response JSON from Ollama")?;

    Ok(res_body.response.trim().to_string())
}
