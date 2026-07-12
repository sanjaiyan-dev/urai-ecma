use std::{path::PathBuf, sync::Arc};

use clap::{Parser, ValueHint};
mod ast;
mod markdown;
mod ollama;
#[derive(Parser, Debug)]
#[command(name = "urai-ecma", version = "1.0", about = "")]
struct Args {
    #[arg(short, long, value_name = "Input FILE_PATH", value_hint = ValueHint::FilePath)]
    input_file: PathBuf,
    #[arg(short, long, value_name = "Output FILE_PATH", value_hint = ValueHint::FilePath)]
    output_file: PathBuf,

    #[arg(short, long, env = "OLLAMA_ENDPOINT", value_name="Ollama URL Endpoint", value_hint= ValueHint::Url)]
    ollama_endpoint: Option<String>,
    #[arg(short, long, value_name = "Ollama Model Name")]
    ollama_modelname: Option<String>,
}
struct OllamaContext {
    ollama_model_name: Option<String>,
    ollama_endpoint: Option<String>,
}
pub struct UraiContext {
    input_filename: PathBuf,
    output_filename: PathBuf,

    ollama_endpoint: OllamaContext,
}

fn main() {
    let args = Args::parse();
    let ctx = UraiContext {
        input_filename: args.input_file,
        output_filename: args.output_file,
        ollama_endpoint: OllamaContext {
            ollama_model_name: args.ollama_modelname,
            ollama_endpoint: args.ollama_endpoint,
        },
    };
    let urai_ctx = Arc::new(ctx);

    println!("Hello, Sanjaiyan!");
}
