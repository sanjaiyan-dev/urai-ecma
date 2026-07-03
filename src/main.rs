use std::path::PathBuf;

use clap::{Parser, ValueHint};
mod ast;
mod markdown;
mod ollama;
#[derive(Parser, Debug)]
#[command(name = "urai-ecma", version = "1.0", about = "")]
struct Args {
    #[arg(short, long, value_name = "FILE_PATH", value_hint = ValueHint::FilePath)]
    input_file: PathBuf,
    #[arg(short, long, value_name="Ollama EndPoint", value_hint= ValueHint::Url)]
    ollama_endpoint: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello, Sanjaiyan!");
}
