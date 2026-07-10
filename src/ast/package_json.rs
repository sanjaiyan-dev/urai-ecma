use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub main: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
    pub dependencies: Option<HashMap<String, String>>,
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub author: Option<serde_json::Value>,
    pub license: Option<String>,
}

fn read_person_from_file(path: &str) -> Result<PackageJson> {
    let file =
        File::open(path).with_context(|| format!("Failed to open file at path: {}", path))?;

    let reader = BufReader::new(file);

    let package_json: PackageJson =
        serde_json::from_reader(reader).context("Failed to parse JSON structure")?;

    Ok(package_json)
}

pub fn parse_package_json() -> Result<String> {
    let pkg_json = read_person_from_file("package.json")?;

    let pkg_json_name_len = pkg_json.name.len();
    let pkg_json_desc_len = pkg_json.description.as_ref().map_or(0, |d| d.len());
    let pkg_json_content_est_size = pkg_json_name_len + pkg_json_desc_len + 100;
    let mut pkg_json_content: String = String::with_capacity(pkg_json_content_est_size);

    let _ = writeln!(pkg_json_content, "# Project Title: {} \n", &pkg_json.name);
    if let Some(desc) = &pkg_json.description {
        let _ = writeln!(pkg_json_content, "## Project Description\n\n{desc}\n");
    }

    if let Some(dependencies) = pkg_json.dependencies.as_ref().filter(|m| !m.is_empty()) {
        pkg_json_content.reserve(35 + dependencies.len() * 12);
        let _ = writeln!(pkg_json_content, "Dependencies used in this project: ");

        for (name, version) in dependencies.iter() {
            let _ = writeln!(pkg_json_content, "   - **{name}** : `{version}`");
        }
    }

    if let Some(dev_dependencies) = pkg_json.dev_dependencies.as_ref().filter(|m| !m.is_empty()) {
        pkg_json_content.reserve(40 + dev_dependencies.len() * 12);
        let _ = writeln!(pkg_json_content, "Dev dependencies used in this project: ");

        for (name, version) in dev_dependencies.iter() {
            let _ = writeln!(pkg_json_content, "   - **{name}** : `{version}`");
        }

        let _ = writeln!(pkg_json_content);
    }
    
    let _ = writeln!(
        pkg_json_content,
        "#### Project Version: {} \n",
        pkg_json.version
    );

    Ok(pkg_json_content)
}
