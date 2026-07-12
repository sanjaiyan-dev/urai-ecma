use anyhow::{Context, Result};

use crate::ast::package_json::parse_package_json;

struct SuccessCreated {
    file_name: String,
    token_size: u32,
}

struct MarkdownFileData {
    file_name: String,
}

fn create_markdown_output() -> Result<SuccessCreated> {
    let package_json_markdown_content = parse_package_json().unwrap_or("".to_string());
    unimplemented!()
}
