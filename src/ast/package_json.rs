use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
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

pub fn parse_package_json() -> Result<PackageJson> {
    read_person_from_file("package.json").context("Failed to retrieve package.json configuration")
}
