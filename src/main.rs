use clap::{Parser, Subcommand, ValueHint};
use serde::Deserialize;
use std::{fs, path::PathBuf, sync::Arc};

mod ast;
mod markdown;
mod ollama;

#[derive(Parser, Debug)]
#[command(name = "urai-ecma", version = "1.0", about = "")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short = 'i', long, value_name = "Input PROJECT_PATH", value_hint = ValueHint::AnyPath)]
    input_project: Option<PathBuf>,

    #[arg(short = 'o', long, value_name = "Output FILE_PATH", value_hint = ValueHint::FilePath)]
    output_file: Option<PathBuf>,

    #[arg(short = 'e', long, env = "OLLAMA_ENDPOINT", value_name="Ollama URL Endpoint", value_hint= ValueHint::Url)]
    ollama_endpoint: Option<String>,

    #[arg(short = 'm', long, value_name = "Ollama Model Name")]
    ollama_modelname: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Create,
}

#[derive(Deserialize, Default, Debug)]
struct UraiConfig {
    input_project: Option<PathBuf>,
    output_file: Option<PathBuf>,
    ollama_endpoint: Option<String>,
    ollama_modelname: Option<String>,
}

struct OllamaContext {
    ollama_model_name: Option<String>,
    ollama_endpoint: Option<String>,
    ollama_cache_folder: PathBuf,
}

pub struct UraiContext {
    input_project: PathBuf, // Renamed to match
    output_filename: PathBuf,
    ollama_endpoint: OllamaContext,
}

const DEFAULT_CONFIG: &str = r#"{
    // The input project directory or file path
    // "input_project": "./src",

    // The output file path
    // "output_file": "./output.md",

    // Ollama URL Endpoint (e.g., "http://localhost:11434")
    // "ollama_endpoint": "http://localhost:11434",

    // Ollama Model Name (e.g., "ornith")
    // "ollama_modelname": "gemma4"
}
"#;

fn main() {
    let cli = Cli::parse();

    if let Some(Commands::Create) = cli.command {
        let config_path = PathBuf::from("urai.config.jsonc");
        if config_path.exists() {
            eprintln!("Error: urai.config.jsonc already exists in this directory!");
            std::process::exit(1);
        }

        if let Err(e) = fs::write(&config_path, DEFAULT_CONFIG) {
            eprintln!("Error writing config file: {}", e);
            std::process::exit(1);
        }

        println!("Successfully created urai.config.jsonc with default comments.");
        return;
    }

    let config_path = [
        PathBuf::from("urai.config.jsonc"),
        PathBuf::from("urai.config.json"),
    ]
    .into_iter()
    .find(|p| p.exists());

    let config: UraiConfig = if let Some(path) = config_path {
        let content = fs::read_to_string(&path).unwrap_or_default();
        json5::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Error parsing {}: {}", path.display(), e);
            std::process::exit(1);
        })
    } else {
        UraiConfig::default()
    };

    let input_project_path = cli.input_project.or(config.input_project).unwrap_or_else(|| {
        eprintln!("Error: `input_project` is required. Provide it via CLI (--input-project) or urai.config.json");
        std::process::exit(1);
    });

    let output_file_path = cli.output_file.or(config.output_file).unwrap_or_else(|| {
        eprintln!("Error: `output_file` is required. Provide it via CLI (--output-file) or urai.config.json");
        std::process::exit(1);
    });

    let ollama_endpoint = cli.ollama_endpoint.or(config.ollama_endpoint);
    let ollama_model_name = cli.ollama_modelname.or(config.ollama_modelname);

    let ollama_cache_folder = if input_project_path.is_dir() {
        input_project_path.join(".urai-cache")
    } else {
        input_project_path
            .parent()
            .map(|parent| parent.join(".urai-cache"))
            .unwrap_or_else(|| std::path::PathBuf::from(".urai-cache"))
    };

    let ctx = UraiContext {
        input_project: input_project_path,
        output_filename: output_file_path,
        ollama_endpoint: OllamaContext {
            ollama_model_name,
            ollama_endpoint,
            ollama_cache_folder,
        },
    };

    let urai_ctx = Arc::new(ctx);

    println!("Hello, Sanjaiyan!");
}
