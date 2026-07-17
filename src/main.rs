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

    ollama_cache_folder: PathBuf,
}
pub struct UraiContext {
    input_filename: PathBuf,
    output_filename: PathBuf,

    ollama_endpoint: OllamaContext,
}

fn main() {
    let args = Args::parse();
    let input_root_file = args.input_file;
    let ollama_cache_folder = if input_root_file.is_dir() {
        input_root_file.join(".urai-cache")
    } else {
        input_root_file
            .parent()
            .map(|parent| parent.join(".urai-cache"))
            .unwrap_or_else(|| std::path::PathBuf::from(".urai-cache"))
    };
    let ctx = UraiContext {
        input_filename: input_root_file,
        output_filename: args.output_file,
        ollama_endpoint: OllamaContext {
            ollama_model_name: args.ollama_modelname,
            ollama_endpoint: args.ollama_endpoint,
            ollama_cache_folder,
        },
    };
    let urai_ctx = Arc::new(ctx);

    println!("Hello, Sanjaiyan!");
}
