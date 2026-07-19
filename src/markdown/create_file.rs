use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Arc,
};

use anyhow::{Context, Result};

use crate::{UraiContext, markdown::MarkdownUrai};

struct MarkdownFileData {
    pub markdown_writer: File,
}

impl MarkdownUrai {
    pub fn new(ctx: Arc<UraiContext>) -> Self {
        Self { ctx }
    }

    pub fn create_markdown_file(&self) -> Result<MarkdownFileData> {
        let markdown_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.ctx.output_filename)
            .with_context(|| {
                format!(
                    "Failed to create or open file '{}' in append mode",
                    self.ctx.output_filename.display()
                )
            })?;

        Ok(MarkdownFileData {
            markdown_writer: markdown_file,
        })
    }
    pub fn markdown_content_writer(&self, txt_content: &str) -> Result<()> {
        let mut file_data = self.create_markdown_file()?;

        file_data
            .markdown_writer
            .write_all(txt_content.as_bytes())
            .context("Failed to write content bytes to the markdown file")?;
        file_data
            .markdown_writer
            .write_all(b"\n")
            .context("Failed to write newline")?;

        Ok(())
    }

    pub fn clear_markdown_content(&self) -> Result<String> {
        File::create(&self.ctx.output_filename).with_context(|| {
            format!(
                "Failed to clear the contents of file '{}'",
                self.ctx.output_filename.display()
            )
        })?;

        Ok(self.ctx.output_filename.display().to_string())
    }
}
