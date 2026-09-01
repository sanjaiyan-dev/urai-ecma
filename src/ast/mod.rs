use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Arc;

use crate::UraiContext;

pub mod analyze;
pub mod graph;
pub mod package_json;
pub mod parser;
pub mod visitor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TailwindMode {
    #[default]
    Remove,
    #[serde(alias = "remove_aggressive", alias = "aggressive")]
    RemoveAggr,
    Summarize,
    Preserve,
}
pub struct PackageJsonUrai {
    pub ctx: Arc<UraiContext>,
}

#[derive(Debug, Clone)]
pub struct RouteInfo {
    pub framework: String, // Express, Fastify, Next.js, NestJS
    pub method: String,    // GET, POST, PUT, DELETE, ALL
    pub path: String,      // /api/users, /login
    pub handler_name: String,
    pub file_path: String,
    pub line_number: usize,
}

#[derive(Debug, Clone, Default)]
pub struct ReactComponentAnalysis {
    pub component_name: String,
    pub is_export_default: bool,
    pub props: Vec<PropDetail>,
    pub state_vars: Vec<StateDetail>,
    pub effect_count: usize,
    pub hooks_used: Vec<String>,
    pub event_handlers: Vec<String>,
    pub rendered_elements: Vec<String>,
    pub pruned_jsx_code: String,
    pub detailed_explanation: String,
}

#[derive(Debug, Clone)]
pub struct PropDetail {
    pub name: String,
    pub prop_type: String,
    pub is_optional: bool,
}

#[derive(Debug, Clone)]
pub struct StateDetail {
    pub state_name: String,
    pub setter_name: String,
    pub initial_value: Option<String>,
    pub state_type: String,
}

#[derive(Debug, Clone)]
pub struct FileAnalysisResult {
    pub file_path: PathBuf,
    pub relative_path: String,
    pub raw_content: String,
    pub processed_content: String,
    pub routes: Vec<RouteInfo>,
    pub react_components: Vec<ReactComponentAnalysis>,
}

#[derive(Debug, Clone)]
pub struct FunctionSummary {
    pub function_name: String,
    pub line_number: usize,
    pub concise_summary: String,
}
