//! mem0-rs: Long-term memory for AI Agents
//!
//! This library provides a Rust implementation of Mem0, enabling AI agents
//! and assistants to maintain persistent, personalized memory across interactions.

pub mod error;
pub mod memory;
pub mod vector_store;
pub mod llm;
pub mod embeddings;
pub mod config;
pub mod graph;
pub mod filtering;

pub use memory::{Memory, MemoryBase};
pub use vector_store::VectorStoreBase;
pub use llm::LlmBase;
pub use embeddings::EmbedderBase;
pub use graph::GraphStoreBase;
pub use filtering::{FilterQuery, AggregationQuery, TimeFilter, QueryBuilder};
pub use error::{Error, Result};
pub use config::MemoryConfig;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
