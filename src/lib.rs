//! memory-rs: Long-term memory for AI Agents
//!
//! This library provides a Rust implementation of long-term memory, enabling AI agents
//! and assistants to maintain persistent, personalized memory across interactions.

pub mod error;
pub mod memory;
pub mod vector_store;
pub mod embeddings;
pub mod config;
pub mod graph;
pub mod filtering;
pub mod distributed;

pub use memory::{Memory, MemoryBase};
pub use vector_store::VectorStoreBase;
pub use embeddings::EmbedderBase;
pub use graph::GraphStoreBase;
pub use filtering::{FilterQuery, AggregationQuery, TimeFilter, QueryBuilder};
pub use distributed::{DistributedStoreBase, DistributedConfig, NodeRole, ShardingStrategy};
pub use error::{Error, Result};
pub use config::MemoryConfig;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
