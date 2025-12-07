//! MCP (Model Context Protocol) server implementation for memory-rs
//!
//! This module provides an MCP server that exposes memory operations as tools.

use std::sync::Arc;
use tokio::sync::RwLock;

use rmcp::{
    handler::server::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::*,
    schemars,
    tool, tool_handler, tool_router,
    ErrorData as McpError,
};
use serde::{Deserialize, Serialize};

use crate::config::MemoryConfig;
use crate::embeddings::LocalEmbedder;
use crate::memory::{Memory, MemoryBase, MemoryItem, SearchResultItem};
use crate::vector_store::InMemoryStore;

/// MCP Memory Server
#[derive(Clone)]
pub struct MemoryMcpServer {
    memory: Arc<RwLock<Memory>>,
    tool_router: ToolRouter<Self>,
}

/// Input for adding a memory
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct AddMemoryInput {
    /// User ID to associate the memory with
    #[schemars(description = "User ID to associate the memory with")]
    pub user_id: String,
    /// Content of the memory
    #[schemars(description = "Content of the memory to store")]
    pub content: String,
    /// Optional memory type (e.g., "fact", "preference", "insight")
    #[schemars(description = "Optional memory type (e.g., 'fact', 'preference', 'insight')")]
    pub memory_type: Option<String>,
}

/// Input for searching memories
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SearchMemoryInput {
    /// User ID to search memories for
    #[schemars(description = "User ID to search memories for")]
    pub user_id: String,
    /// Search query
    #[schemars(description = "Search query for semantic similarity matching")]
    pub query: String,
    /// Maximum number of results (default: 5)
    #[schemars(description = "Maximum number of results to return (default: 5)")]
    pub limit: Option<usize>,
}

/// Input for updating a memory
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct UpdateMemoryInput {
    /// Memory ID to update
    #[schemars(description = "Memory ID to update")]
    pub memory_id: String,
    /// New content for the memory
    #[schemars(description = "New content for the memory")]
    pub content: String,
}

/// Input for deleting a memory
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DeleteMemoryInput {
    /// Memory ID to delete
    #[schemars(description = "Memory ID to delete")]
    pub memory_id: String,
}

/// Input for getting all memories
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetAllMemoriesInput {
    /// User ID to get memories for
    #[schemars(description = "User ID to get all memories for")]
    pub user_id: String,
}

/// Memory response for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryResponse {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub memory_type: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<MemoryItem> for MemoryResponse {
    fn from(item: MemoryItem) -> Self {
        Self {
            id: item.id,
            user_id: item.user_id,
            content: item.content,
            memory_type: item.memory_type,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

/// Search result response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub memory: MemoryResponse,
    pub score: f32,
}

impl From<SearchResultItem> for SearchResponse {
    fn from(item: SearchResultItem) -> Self {
        Self {
            memory: item.memory.into(),
            score: item.score,
        }
    }
}

#[tool_router]
impl MemoryMcpServer {
    /// Create a new MCP Memory Server with default configuration
    pub fn new() -> Self {
        let config = MemoryConfig::new("memory.db".to_string());
        let vector_store = Arc::new(InMemoryStore::new());
        let embedder = Arc::new(LocalEmbedder::with_defaults());
        let memory = Memory::new(config, vector_store, embedder);

        Self {
            memory: Arc::new(RwLock::new(memory)),
            tool_router: Self::tool_router(),
        }
    }

    /// Create with custom memory instance
    pub fn with_memory(memory: Memory) -> Self {
        Self {
            memory: Arc::new(RwLock::new(memory)),
            tool_router: Self::tool_router(),
        }
    }

    /// Add a new memory for a user
    #[tool(description = "Add a new memory for a user. Stores content with semantic embedding for later retrieval.")]
    async fn add_memory(
        &self,
        input: Parameters<AddMemoryInput>,
    ) -> Result<CallToolResult, McpError> {
        let memory = self.memory.read().await;
        match memory
            .add(&input.0.user_id, &input.0.content, input.0.memory_type.as_deref())
            .await
        {
            Ok(item) => {
                let response: MemoryResponse = item.into();
                let json = serde_json::to_string_pretty(&response)
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    /// Search memories by semantic similarity
    #[tool(description = "Search memories for a user using semantic similarity. Returns the most relevant memories matching the query.")]
    async fn search_memory(
        &self,
        input: Parameters<SearchMemoryInput>,
    ) -> Result<CallToolResult, McpError> {
        let memory = self.memory.read().await;
        let limit = input.0.limit.unwrap_or(5);
        match memory.search(&input.0.user_id, &input.0.query, limit).await {
            Ok(results) => {
                let responses: Vec<SearchResponse> =
                    results.into_iter().map(|r| r.into()).collect();
                let json = serde_json::to_string_pretty(&responses)
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    /// Update an existing memory
    #[tool(description = "Update the content of an existing memory by its ID.")]
    async fn update_memory(
        &self,
        input: Parameters<UpdateMemoryInput>,
    ) -> Result<CallToolResult, McpError> {
        let memory = self.memory.read().await;
        match memory.update(&input.0.memory_id, &input.0.content).await {
            Ok(item) => {
                let response: MemoryResponse = item.into();
                let json = serde_json::to_string_pretty(&response)
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    /// Delete a memory
    #[tool(description = "Delete a memory by its ID.")]
    async fn delete_memory(
        &self,
        input: Parameters<DeleteMemoryInput>,
    ) -> Result<CallToolResult, McpError> {
        let memory = self.memory.read().await;
        match memory.delete(&input.0.memory_id).await {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Memory {} deleted successfully",
                input.0.memory_id
            ))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    /// Get all memories for a user
    #[tool(description = "Retrieve all memories stored for a specific user.")]
    async fn get_all_memories(
        &self,
        input: Parameters<GetAllMemoriesInput>,
    ) -> Result<CallToolResult, McpError> {
        let memory = self.memory.read().await;
        match memory.get_all(&input.0.user_id).await {
            Ok(items) => {
                let responses: Vec<MemoryResponse> =
                    items.into_iter().map(|i| i.into()).collect();
                let json = serde_json::to_string_pretty(&responses)
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }
}

impl Default for MemoryMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_handler]
impl rmcp::ServerHandler for MemoryMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Memory MCP Server - Long-term memory for AI Agents. \
                 Use this server to store, search, update, and delete memories for users. \
                 Memories are stored with semantic embeddings for intelligent retrieval."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::ServerHandler;

    #[tokio::test]
    async fn test_mcp_server_creation() {
        let server = MemoryMcpServer::new();
        let info = server.get_info();
        assert!(info.instructions.is_some());
    }
}
