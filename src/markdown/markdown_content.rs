use anyhow::Context;

use crate::ast::package_json::PackageJson;
pub struct MarkdownContent {
    package_json_content: Option<PackageJson>,
}
fn markdown_content(params: MarkdownContent) -> String {
    let package_json = params.package_json_content;
    format!(
        "
{:?}
    ",
        package_json.unwrap().name
    );
    unimplemented!()
}
