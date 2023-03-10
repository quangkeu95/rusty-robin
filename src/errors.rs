use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Parse JSON error: {0}")]
    ParseJsonError(String),
}
