use thiserror::Error;

/// Erreurs communes à tous les bounded contexts
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid value: {0}")]
    InvalidValue(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}
