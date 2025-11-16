//! Graph memory abstraction and implementations

use async_trait::async_trait;
use crate::Result;

pub mod neo4j;

pub use neo4j::Neo4jStore;

/// Graph relationship type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationType {
    /// Related to another memory
    RelatedTo,
    /// Contradicts another memory
    Contradicts,
    /// Supports another memory
    Supports,
    /// Part of another memory
    PartOf,
    /// Contains another memory
    Contains,
    /// Custom relationship
    Custom(String),
}

impl RelationType {
    /// Get relationship name
    pub fn name(&self) -> String {
        match self {
            Self::RelatedTo => "RELATED_TO".to_string(),
            Self::Contradicts => "CONTRADICTS".to_string(),
            Self::Supports => "SUPPORTS".to_string(),
            Self::PartOf => "PART_OF".to_string(),
            Self::Contains => "CONTAINS".to_string(),
            Self::Custom(name) => name.to_uppercase(),
        }
    }
}

/// Graph node representing a memory
#[derive(Debug, Clone)]
pub struct GraphNode {
    /// Node ID
    pub id: String,
    /// Memory content
    pub content: String,
    /// Node labels
    pub labels: Vec<String>,
    /// Node properties
    pub properties: std::collections::HashMap<String, String>,
}

/// Graph relationship
#[derive(Debug, Clone)]
pub struct GraphRelationship {
    /// Source node ID
    pub source_id: String,
    /// Target node ID
    pub target_id: String,
    /// Relationship type
    pub rel_type: RelationType,
    /// Relationship properties
    pub properties: std::collections::HashMap<String, String>,
}

/// Base trait for graph store implementations
#[async_trait]
pub trait GraphStoreBase: Send + Sync {
    /// Create a node
    async fn create_node(&self, node: GraphNode) -> Result<()>;

    /// Get a node by ID
    async fn get_node(&self, id: &str) -> Result<Option<GraphNode>>;

    /// Update a node
    async fn update_node(&self, id: &str, properties: std::collections::HashMap<String, String>) -> Result<()>;

    /// Delete a node
    async fn delete_node(&self, id: &str) -> Result<()>;

    /// Create a relationship
    async fn create_relationship(&self, relationship: GraphRelationship) -> Result<()>;

    /// Get relationships for a node
    async fn get_relationships(&self, node_id: &str) -> Result<Vec<GraphRelationship>>;

    /// Delete a relationship
    async fn delete_relationship(&self, source_id: &str, target_id: &str, rel_type: RelationType) -> Result<()>;

    /// Find nodes by label
    async fn find_nodes_by_label(&self, label: &str) -> Result<Vec<GraphNode>>;

    /// Find path between two nodes
    async fn find_path(&self, source_id: &str, target_id: &str, max_depth: usize) -> Result<Vec<String>>;

    /// Get node count
    async fn node_count(&self) -> Result<usize>;

    /// Get relationship count
    async fn relationship_count(&self) -> Result<usize>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relation_type_names() {
        assert_eq!(RelationType::RelatedTo.name(), "RELATED_TO");
        assert_eq!(RelationType::Contradicts.name(), "CONTRADICTS");
        assert_eq!(RelationType::Supports.name(), "SUPPORTS");
        assert_eq!(RelationType::PartOf.name(), "PART_OF");
        assert_eq!(RelationType::Contains.name(), "CONTAINS");
    }

    #[test]
    fn test_custom_relation_type() {
        let custom = RelationType::Custom("my_relation".to_string());
        assert_eq!(custom.name(), "MY_RELATION");
    }

    #[test]
    fn test_graph_node_creation() {
        let node = GraphNode {
            id: "node_1".to_string(),
            content: "Test content".to_string(),
            labels: vec!["Memory".to_string()],
            properties: std::collections::HashMap::new(),
        };

        assert_eq!(node.id, "node_1");
        assert_eq!(node.labels.len(), 1);
    }

    #[test]
    fn test_graph_relationship_creation() {
        let rel = GraphRelationship {
            source_id: "node_1".to_string(),
            target_id: "node_2".to_string(),
            rel_type: RelationType::RelatedTo,
            properties: std::collections::HashMap::new(),
        };

        assert_eq!(rel.source_id, "node_1");
        assert_eq!(rel.target_id, "node_2");
    }
}
