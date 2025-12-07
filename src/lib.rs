//! memory-rs: Long-term memory for AI Agents
//!
//! This library provides a Rust implementation of long-term memory, enabling AI agents
//! and assistants to maintain persistent, personalized memory across interactions.
//!
//! ## MCP Server
//!
//! This library includes an MCP (Model Context Protocol) server that exposes memory
//! operations as tools. Run the server with:
//!
//! ```bash
//! cargo run --bin memory-mcp
//! ```

pub mod config;
pub mod distributed;
pub mod embeddings;
pub mod error;
pub mod filtering;
pub mod graph;
pub mod mcp;
pub mod memory;
pub mod utils;
pub mod vector_store;

pub use config::MemoryConfig;
pub use distributed::{DistributedConfig, DistributedStoreBase, NodeRole, ShardingStrategy};
pub use embeddings::EmbedderBase;
pub use error::{Error, Result};
pub use filtering::{AggregationQuery, FilterQuery, QueryBuilder, TimeFilter};
pub use graph::GraphStoreBase;
pub use mcp::MemoryMcpServer;
pub use memory::{Memory, MemoryBase};
pub use vector_store::VectorStoreBase;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
