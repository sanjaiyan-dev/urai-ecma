use std::sync::Arc;

use crate::UraiContext;

pub mod create_file;
pub mod markdown_content;

pub struct MarkdownUrai {
    ctx: Arc<UraiContext>,
}
