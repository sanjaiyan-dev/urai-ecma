use ast::TailwindMode;
use clap::{Parser, Subcommand, ValueHint};
use ignore::WalkBuilder;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use tiktoken::CoreBpe;

mod ast;
mod markdown;
mod ollama;

#[derive(Parser, Debug)]
#[command(
    name = "urai-ecma",
    version = "1.0",
    about = "AST-aware JS/TS code to prompt tool"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short = 'i', long, value_name = "Input PROJECT_PATH", value_hint = ValueHint::AnyPath)]
    input_project: Option<PathBuf>,

    #[arg(short = 'o', long, value_name = "Output FILE_PATH", value_hint = ValueHint::FilePath)]
    output_file: Option<PathBuf>,

    #[arg(short = 'e', long, env = "OLLAMA_ENDPOINT", value_name = "Ollama URL Endpoint", value_hint = ValueHint::Url)]
    ollama_endpoint: Option<String>,

    #[arg(short = 'm', long, value_name = "Ollama Model Name")]
    ollama_modelname: Option<String>,

    #[arg(
        long,
        value_name = "MODE",
        help = "Tailwind CSS mode: remove, remove_aggr, summarize, preserve"
    )]
    tailwind_mode: Option<String>,

    #[arg(
        long,
        value_name = "CHARS",
        help = "Character count threshold for Tailwind pruning (default: 96)"
    )]
    tailwind_threshold: Option<usize>,

    #[arg(long, help = "Summarize function blocks using Ollama or JSDoc")]
    summarize_functions: Option<bool>,

    #[arg(
        long,
        value_name = "LINES",
        help = "Line count threshold to trigger function summarization (default: 5)"
    )]
    summarize_functions_threshold: Option<usize>,

    #[arg(long, help = "Generate route table for backend frameworks")]
    generate_route_table: Option<bool>,

    #[arg(long, help = "Analyze React components")]
    analyze_react_components: Option<bool>,

    #[arg(long, help = "Generate project file tree & module graph")]
    generate_file_graph: Option<bool>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Creates urai.config.jsonc file
    Create,
}

#[derive(Deserialize, Default, Debug)]
pub struct UraiConfig {
    input_project: Option<PathBuf>,
    output_file: Option<PathBuf>,
    ollama_endpoint: Option<String>,
    ollama_modelname: Option<String>,
    tailwind_mode: Option<TailwindMode>,
    tailwind_threshold: Option<usize>,
    summarize_functions: Option<bool>,
    summarize_functions_threshold: Option<usize>,
    generate_route_table: Option<bool>,
    analyze_react_components: Option<bool>,
    generate_file_graph: Option<bool>,
}

pub struct OllamaContext {
    pub ollama_model_name: Option<String>,
    pub ollama_endpoint: Option<String>,
    pub ollama_cache_folder: PathBuf,
}

pub struct UraiContext {
    pub input_project: PathBuf,
    pub output_filename: PathBuf,
    pub ollama_endpoint: OllamaContext,
    pub tailwind_mode: TailwindMode,
    pub tailwind_threshold: usize,
    pub summarize_functions: bool,
    pub summarize_functions_threshold: usize,
    pub generate_route_table: bool,
    pub analyze_react_components: bool,
    pub generate_file_graph: bool,
}

const DEFAULT_CONFIG: &str = r#"{
    // Path to the project directory or single source file
    "input_project": "./src",

    // Target output Markdown file path
    "output_file": "./output.md",

    // Ollama local endpoint URL (Optional, e.g., "http://localhost:11434")
    "ollama_endpoint": "http://localhost:11434",

    // Ollama Model Name (e.g., "gemma4", "ornith")
    "ollama_modelname": "gemma4",

    // Tailwind CSS / className pruning mode: "remove" | "remove_aggr" | "summarize" | "preserve"
    // "remove": strips static class strings exceeding threshold while keeping dynamic expressions.
    // "remove_aggr": aggressively removes class strings even if below character threshold.
    // "summarize": sends class strings exceeding threshold to Ollama for 1-line style descriptions.
    // "preserve": keeps classNames untouched.
    "tailwind_mode": "remove",

    // Character length threshold for Tailwind pruning (default: 96 characters)
    "tailwind_threshold": 96,

    // Summarize function block bodies using local Ollama or fallback to JSDoc comments
    "summarize_functions": true,

    // Line count threshold to trigger function summarization (default: 5 lines)
    "summarize_functions_threshold": 5,

    // Extract and generate Express/Fastify/Next.js/NestJS API Route Table
    "generate_route_table": true,

    // Analyze React / React Native components and output detailed explanations
    "analyze_react_components": true,

    // Generate ASCII File Structure & Module Dependency Graph
    "generate_file_graph": true
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
        eprintln!("Error: `input_project` is required. Provide it via CLI (--input-project) or urai.config.jsonc");
        std::process::exit(1);
    });

    let output_file_path = cli.output_file.or(config.output_file).unwrap_or_else(|| {
        eprintln!("Error: `output_file` is required. Provide it via CLI (--output-file) or urai.config.jsonc");
        std::process::exit(1);
    });

    let ollama_endpoint = cli.ollama_endpoint.or(config.ollama_endpoint);
    let ollama_model_name = cli.ollama_modelname.or(config.ollama_modelname);

    let tailwind_mode = cli
        .tailwind_mode
        .as_deref()
        .map(|s| match s.to_lowercase().as_str() {
            "summarize" => TailwindMode::Summarize,
            "preserve" => TailwindMode::Preserve,
            "remove_aggr" | "remove_aggressive" | "aggressive" => TailwindMode::RemoveAggr,
            _ => TailwindMode::Remove,
        })
        .or(config.tailwind_mode)
        .unwrap_or(TailwindMode::Remove);

    let tailwind_threshold = cli
        .tailwind_threshold
        .or(config.tailwind_threshold)
        .unwrap_or(96);

    let summarize_functions = cli
        .summarize_functions
        .or(config.summarize_functions)
        .unwrap_or(true);

    let summarize_functions_threshold = cli
        .summarize_functions_threshold
        .or(config.summarize_functions_threshold)
        .unwrap_or(5);

    let generate_route_table = cli
        .generate_route_table
        .or(config.generate_route_table)
        .unwrap_or(true);
    let analyze_react_components = cli
        .analyze_react_components
        .or(config.analyze_react_components)
        .unwrap_or(true);
    let generate_file_graph = cli
        .generate_file_graph
        .or(config.generate_file_graph)
        .unwrap_or(true);

    let ollama_cache_folder = if input_project_path.is_dir() {
        input_project_path.join(".urai-cache")
    } else {
        input_project_path
            .parent()
            .map(|parent| parent.join(".urai-cache"))
            .unwrap_or_else(|| PathBuf::from(".urai-cache"))
    };

    let ctx = Arc::new(UraiContext {
        input_project: input_project_path,
        output_filename: output_file_path,
        ollama_endpoint: OllamaContext {
            ollama_model_name,
            ollama_endpoint,
            ollama_cache_folder,
        },
        tailwind_mode,
        tailwind_threshold,
        summarize_functions,
        summarize_functions_threshold,
        generate_route_table,
        analyze_react_components,
        generate_file_graph,
    });

    println!(
        "🚀 [urai-ecma] Starting AST Analysis on project: {}",
        ctx.input_project.display()
    );

    if let Err(e) = ast::analyze::run_project_analysis(ctx.clone()) {
        eprintln!("❌ Error running project analysis: {:#}", e);
        std::process::exit(1);
    }

    println!(
        "✅ [urai-ecma] Prompt successfully generated at: {}",
        ctx.output_filename.display()
    );
    if let Ok(content) = fs::read_to_string(&ctx.output_filename)
        && let Some(bpe) = tiktoken::get_encoding("llama3") {
            let token_count = bpe.encode(&content);
            println!(
                "📊 [urai-ecma] Estimated Tokens in {}: {} tokens",
                ctx.output_filename.display(),
                token_count.len()
            );
        }

    if let Some(bpe) = tiktoken::get_encoding("llama3") {
        let output_tokens = fs::read_to_string(&ctx.output_filename)
            .ok()
            .map(|content| bpe.encode(&content).len())
            .unwrap_or(0);

        let raw_tokens = calculate_raw_project_tokens(&ctx.input_project, bpe);

        if raw_tokens > 0 {
            let saved_tokens = raw_tokens.saturating_sub(output_tokens);
            let reduction_percentage = (saved_tokens as f64 / raw_tokens as f64) * 100.0;

            println!("\n============================================================");
            println!("📊 TOKEN SAVINGS & OPTIMIZATION REPORT");
            println!("============================================================");
            println!("📁 Raw Source Code (All JS/TS): {:>10} tokens", raw_tokens);
            println!(
                "⚡ Optimized Output (output.md): {:>10} tokens",
                output_tokens
            );
            println!("------------------------------------------------------------");
            if output_tokens <= raw_tokens {
                println!(
                    "🎉 Reduction: -{:.2}% tokens saved! (Saved ~{} tokens)",
                    reduction_percentage, saved_tokens
                );
            } else {
                println!(
                    "ℹ️ Output expanded by +{:.2}% tokens due to added AST context/graphs.",
                    ((output_tokens - raw_tokens) as f64 / raw_tokens as f64) * 100.0
                );
            }
            println!("============================================================\n");
        }
    } else {
        eprintln!("⚠️ Failed to load tiktoken encoding.");
    }
}

fn calculate_raw_project_tokens(path: &Path, tokenizer: &CoreBpe) -> usize {
    let mut total_tokens = 0;

    for entry in WalkBuilder::new(path).build().flatten() {
        let file_path = entry.path();

        if file_path.is_file() && is_source_file(file_path)
            && let Ok(content) = fs::read_to_string(file_path) {
                let encoding = tokenizer.encode(content.as_str());
                if let Some(file_name) = file_path.file_name() {
                    total_tokens += file_name.len();
                }
                total_tokens += encoding.len();
            }
    }

    total_tokens
}
fn is_source_file(path: &Path) -> bool {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    matches!(ext, "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" | "json")
}
