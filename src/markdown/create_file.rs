use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
    sync::Arc,
};

use anyhow::{Result, bail};
use std::io::ErrorKind;

use crate::{UraiContext, markdown::MarkdownUrai};

struct MarkdownFileData {
    file_name: PathBuf,
    pub markdown_writter: BufWriter<File>,
}

impl MarkdownUrai {
    pub fn new(ctx: Arc<UraiContext>) -> Self {
        Self { ctx }
    }

    pub fn create_markdown_file(&self) -> Result<MarkdownFileData> {
        let outputfile_result = File::create_new(&self.ctx.output_filename);

        let markdown_file = match outputfile_result {
            Ok(file_info) => file_info,
            Err(err) => match err.kind() {
                ErrorKind::AlreadyExists => OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&self.ctx.output_filename)?,

                _ => {
                    bail!(
                        "Error: Failed to create '{}' due to: {err}",
                        self.ctx.output_filename.display()
                    );
                }
            },
        };

        Ok(MarkdownFileData {
            file_name: self.ctx.output_filename.clone(),
            markdown_writter: BufWriter::new(markdown_file),
        })
    }

    pub fn markdown_content_writer(&self, txt_content: String) -> Result<()> {
        let mut markdown_file = self.create_markdown_file()?;
        writeln!(markdown_file.markdown_writter, "{txt_content}")?;
        markdown_file.markdown_writter.flush()?;
        Ok(())
    }
}
