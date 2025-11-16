//! Error types for mem0-rs

use thiserror::Error;

/// Result type for mem0-rs operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for mem0-rs
#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Vector store error: {0}")]
    VectorStoreError(String),

    #[error("LLM error: {0}")]
    LlmError(String),

    #[error("Embedding error: {0}")]
    EmbeddingError(String),

    #[error("Memory error: {0}")]
    MemoryError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Qdrant error: {0}")]
    QdrantError(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),
}

impl Error {
    /// Create a configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Error::ConfigError(msg.into())
    }

    /// Create a vector store error
    pub fn vector_store(msg: impl Into<String>) -> Self {
        Error::VectorStoreError(msg.into())
    }

    /// Create an LLM error
    pub fn llm(msg: impl Into<String>) -> Self {
        Error::LlmError(msg.into())
    }

    /// Create an embedding error
    pub fn embedding(msg: impl Into<String>) -> Self {
        Error::EmbeddingError(msg.into())
    }

    /// Create a memory error
    pub fn memory(msg: impl Into<String>) -> Self {
        Error::MemoryError(msg.into())
    }

    /// Create an invalid argument error
    pub fn invalid_arg(msg: impl Into<String>) -> Self {
        Error::InvalidArgument(msg.into())
    }

    /// Create a not found error
    pub fn not_found(msg: impl Into<String>) -> Self {
        Error::NotFound(msg.into())
    }

    /// Create an internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Error::InternalError(msg.into())
    }

    /// Create a timeout error
    pub fn timeout(msg: impl Into<String>) -> Self {
        Error::Timeout(msg.into())
    }

    /// Create an authentication error
    pub fn auth(msg: impl Into<String>) -> Self {
        Error::AuthenticationError(msg.into())
    }
}
