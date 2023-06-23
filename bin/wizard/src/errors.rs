use thiserror::Error;

#[derive(Debug, Error)]
pub enum WizardError {
    #[error(transparent)]
    YamlParsingError(#[from] serde_yaml::Error),
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
}
