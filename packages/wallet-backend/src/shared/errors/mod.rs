use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid value: {0}")]
    InvalidValue(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("External service error: {0}")]
    ExternalService(String),
}
