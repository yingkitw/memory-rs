//! Vector store backend implementations

/// Backend type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendType {
    /// Qdrant backend
    Qdrant,
    /// Milvus backend
    Milvus,
    /// PostgreSQL/pgvector backend
    PostgreSQL,
}

impl BackendType {
    /// Get backend name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Qdrant => "qdrant",
            Self::Milvus => "milvus",
            Self::PostgreSQL => "postgresql",
        }
    }

    /// Get backend description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Qdrant => "Open-source vector database",
            Self::Milvus => "Open-source vector database",
            Self::PostgreSQL => "PostgreSQL with pgvector extension",
        }
    }
}

/// Backend configuration
#[derive(Debug, Clone)]
pub struct BackendConfig {
    /// Backend type
    pub backend_type: BackendType,
    /// API endpoint/URL
    pub endpoint: String,
    /// API key
    pub api_key: Option<String>,
    /// Additional configuration
    pub config: std::collections::HashMap<String, String>,
}

impl BackendConfig {
    /// Create a new backend configuration
    pub fn new(backend_type: BackendType, endpoint: String) -> Self {
        Self {
            backend_type,
            endpoint,
            api_key: None,
            config: std::collections::HashMap::new(),
        }
    }

    /// Set API key
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Add configuration parameter
    pub fn with_config(mut self, key: String, value: String) -> Self {
        self.config.insert(key, value);
        self
    }

    /// Get configuration parameter
    pub fn get_config(&self, key: &str) -> Option<&str> {
        self.config.get(key).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_type_names() {
        assert_eq!(BackendType::Qdrant.name(), "qdrant");
        assert_eq!(BackendType::Milvus.name(), "milvus");
        assert_eq!(BackendType::PostgreSQL.name(), "postgresql");
    }

    #[test]
    fn test_backend_config() {
        let config = BackendConfig::new(BackendType::Qdrant, "http://localhost:6334".to_string())
            .with_api_key("test-key".to_string())
            .with_config("dimension".to_string(), "384".to_string());

        assert_eq!(config.backend_type, BackendType::Qdrant);
        assert_eq!(config.api_key, Some("test-key".to_string()));
        assert_eq!(config.get_config("dimension"), Some("384"));
    }

    #[test]
    fn test_backend_descriptions() {
        assert!(!BackendType::Qdrant.description().is_empty());
        assert!(!BackendType::Milvus.description().is_empty());
        assert!(!BackendType::PostgreSQL.description().is_empty());
    }
}
