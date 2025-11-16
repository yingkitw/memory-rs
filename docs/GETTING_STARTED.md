# Getting Started with mem0-rs

## Overview

mem0-rs is a Rust implementation of Mem0, providing long-term memory capabilities for AI agents and assistants. It enables semantic memory storage, retrieval, and management with support for multiple memory types and user/agent scoping.

## Prerequisites

- Rust 1.70 or later
- Qdrant instance (for vector storage)
- Watsonx API credentials (for LLM and embeddings)

## Installation

### From Source

```bash
git clone <repo-url>
cd mem0-rs
cargo build --release
```

### In Your Project

Add to your `Cargo.toml`:

```toml
[dependencies]
mem0-rs = { path = "../mem0-rs" }
tokio = { version = "1.40", features = ["full"] }
```

## Quick Start

### 1. Setup Qdrant

```bash
# Using Docker
docker run -p 6334:6334 qdrant/qdrant:latest
```

### 2. Set Environment Variables

```bash
export WATSONX_API_KEY="your-api-key"
export WATSONX_PROJECT_ID="your-project-id"
```

### 3. Create a Memory Instance

```rust
use mem0_rs::{
    config::MemoryConfig,
    memory::Memory,
    vector_store::qdrant::QdrantStore,
    llm::watsonx::WatsonxLLM,
    embeddings::default::DefaultEmbedder,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure memory
    let config = MemoryConfig::new(
        "http://localhost:6334".to_string(),
        std::env::var("WATSONX_API_KEY")?,
    )
    .with_project_id(std::env::var("WATSONX_PROJECT_ID")?);

    // Initialize components
    let vector_store = Arc::new(
        QdrantStore::new(&config.vector_store_url).await?
    );

    let llm = Arc::new(
        WatsonxLLM::with_defaults(
            config.watsonx_api_key.clone(),
            config.watsonx_project_id.clone().unwrap_or_default(),
        )
    );

    let embedder = Arc::new(
        DefaultEmbedder::with_defaults(
            config.watsonx_api_key.clone(),
            config.watsonx_project_id.clone().unwrap_or_default(),
        )
    );

    // Create memory
    let memory = Memory::new(config, vector_store, llm, embedder);

    // Add memory
    let item = memory
        .add("user_123", "I like Rust programming", Some("preference"))
        .await?;

    println!("Memory added: {}", item.id);

    // Search memories
    let results = memory
        .search("user_123", "What programming languages does the user like?", 3)
        .await?;

    for result in results {
        println!("Found: {} (score: {})", result.memory.content, result.score);
    }

    Ok(())
}
```

## Core Concepts

### Memory Item

A `MemoryItem` represents a single memory with:

- **id**: Unique identifier (UUID)
- **user_id**: Associated user
- **agent_id**: Optional agent identifier
- **run_id**: Optional run identifier
- **content**: The memory text
- **memory_type**: Type of memory (fact, preference, insight, etc.)
- **hash**: Content hash for deduplication
- **created_at/updated_at**: Timestamps
- **metadata**: Custom key-value pairs

### Memory Operations

#### Add Memory

```rust
let memory_item = memory
    .add(user_id, "I prefer dark coffee", Some("preference"))
    .await?;
```

#### Search Memories

```rust
let results = memory
    .search(user_id, "coffee preferences", 5)
    .await?;

for result in results {
    println!("Score: {}, Content: {}", result.score, result.memory.content);
}
```

#### Update Memory

```rust
let updated = memory
    .update(&memory_id, "I prefer dark espresso")
    .await?;
```

#### Delete Memory

```rust
memory.delete(&memory_id).await?;
```

#### Get All Memories

```rust
let all_memories = memory.get_all(user_id).await?;
```

## Configuration

### MemoryConfig

```rust
let config = MemoryConfig::new(
    "http://localhost:6334".to_string(),
    "api-key".to_string(),
)
.with_project_id("project-id".to_string())
.with_llm_model("ibm/granite-4-h-small".to_string())
.with_vector_dimension(384)
.with_collection_prefix("mem0".to_string())
.with_telemetry(true)
.with_batch_size(32);
```

## Architecture

### Components

1. **Memory**: Main orchestrator
2. **VectorStore**: Semantic search backend (Qdrant)
3. **LLM**: Language model for processing (Watsonx)
4. **Embedder**: Text embedding generation

### Trait-Based Design

All components use traits for flexibility:

```rust
pub trait VectorStoreBase { /* ... */ }
pub trait LlmBase { /* ... */ }
pub trait EmbedderBase { /* ... */ }
pub trait MemoryBase { /* ... */ }
```

This allows easy substitution of implementations.

## Testing

Run tests:

```bash
cargo test
```

Run with output:

```bash
cargo test -- --nocapture
```

Run ignored tests (requires Qdrant):

```bash
cargo test -- --ignored
```

## Examples

See `examples/` directory:

```bash
cargo run --example basic_usage
```

## Troubleshooting

### Connection Errors

- Ensure Qdrant is running on `http://localhost:6334`
- Check firewall settings

### API Errors

- Verify Watsonx API key and project ID
- Check API rate limits

### Memory Not Found

- Verify user_id matches
- Check collection name format

## Performance Tips

1. **Batch Operations**: Use batch upsert for multiple vectors
2. **Caching**: Embeddings are cached by default
3. **Vector Dimension**: Adjust based on your needs (384 is default)
4. **Collection Prefix**: Use different prefixes for different applications

## Next Steps

- Read [ARCHITECTURE.md](ARCHITECTURE.md) for design details
- Check [examples/](examples/) for more use cases
- Review [TODO.md](TODO.md) for planned features

## Contributing

Contributions welcome! Please ensure:

- `cargo build` succeeds
- `cargo test` passes
- Code follows Rust conventions

## License

Apache 2.0 - See LICENSE file
