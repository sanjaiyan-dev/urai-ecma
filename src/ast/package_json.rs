use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt::Write;
use std::fs;
use std::sync::Arc;

use crate::UraiContext;
use crate::ast::PackageJsonUrai;

#[derive(Deserialize, Debug)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub main: Option<String>,
    pub scripts: Option<BTreeMap<String, String>>,
    pub dependencies: Option<BTreeMap<String, String>>,
    pub dev_dependencies: Option<BTreeMap<String, String>>,
    pub author: Option<serde_json::Value>,
    pub license: Option<String>,
}

impl PackageJsonUrai {
    pub fn new(ctx: Arc<UraiContext>) -> Self {
        Self { ctx }
    }

    fn read_package_json_from_file(&self) -> Result<PackageJson> {
        let input_path = &self.ctx.input_project;
        let is_dir = self.ctx.input_project.is_dir();
        let package_json_path = if is_dir {
            let package_json_path = input_path.join("package.json");
            if !package_json_path.exists() {
                bail!(
                    "Directory found, but 'package.json' does not exist at: {}",
                    package_json_path.display()
                );
            }
            package_json_path
        } else if input_path.is_file() {
            input_path.to_path_buf()
        } else {
            bail!(
                "The path '{}' does not exist or is not a valid file or directory",
                input_path.display()
            );
        };

        let content = fs::read_to_string(&package_json_path).with_context(|| {
            format!(
                "Failed to read file at path: {}",
                package_json_path.display()
            )
        })?;

        let package_json: PackageJson =
            json5::from_str(&content).context("Failed to parse JSON/JSON5 structure")?;

        Ok(package_json)
    }

    pub fn parse_package_json(&self) -> Result<String> {
        let pkg_json = self.read_package_json_from_file()?;

        let pkg_json_name_len = pkg_json.name.len();
        let pkg_json_desc_len = pkg_json.description.as_ref().map_or(0, |d| d.len());
        let mut pkg_json_content_est_size = pkg_json_name_len + pkg_json_desc_len + 212;
        if let Some(deps) = &pkg_json.dependencies {
            pkg_json_content_est_size += 50 + (deps.len() * 30);
        }
        if let Some(dev_deps) = &pkg_json.dev_dependencies {
            pkg_json_content_est_size += 50 + (dev_deps.len() * 30);
        }

        let mut pkg_json_content = String::with_capacity(pkg_json_content_est_size);

        let _ = writeln!(pkg_json_content, "# Project Title: {} \n", &pkg_json.name);
        if let Some(desc) = &pkg_json.description {
            let _ = writeln!(pkg_json_content, "## Project Description\n\n{desc}\n");
        }

        if let Some(dependencies) = pkg_json.dependencies.as_ref().filter(|m| !m.is_empty()) {
            pkg_json_content.push_str("Dependencies used in this project: ");

            for (name, version) in dependencies.iter() {
                let _ = writeln!(pkg_json_content, "   - **{name}** : `{version}`");
            }
        }

        if let Some(dev_dependencies) = pkg_json.dev_dependencies.as_ref().filter(|m| !m.is_empty())
        {
            pkg_json_content.push_str("Dev dependencies used in this project: ");

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
}
