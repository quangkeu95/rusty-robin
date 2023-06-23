use derive_more::Deref;
use std::path::PathBuf;

use serde::Deserialize;

use crate::errors::WizardError;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub version: String,
    pub tasks: Vec<Tasks>,
}

pub fn parse_main_config(content: &str) -> Result<Config, WizardError> {
    serde_yaml::from_str(content).map_err(WizardError::from)
}

#[derive(Debug, Deserialize)]
pub struct Tasks {
    pub name: TaskName,
    pub id: TaskID,
    pub description: Option<String>,
    pub steps: Vec<Step>,
}

#[derive(Debug, Deserialize, Deref)]
pub struct TaskName(String);

#[derive(Debug, Deserialize, Deref)]
pub struct TaskID(String);

#[derive(Debug, Deserialize)]
pub struct Step {}
