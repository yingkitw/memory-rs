# mem0-rs

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

Rust implementation of **Mem0** - Long-term memory for AI Agents and Assistants.

mem0-rs provides a high-performance, type-safe memory layer for AI systems, enabling semantic memory storage, retrieval, and management with support for multiple memory types and user/agent scoping.

## ✨ Features

- **Multi-Level Memory**: User, Session, and Agent state management
- **Semantic Search**: Qdrant vector store for intelligent memory retrieval
- **LLM Integration**: Watsonx for memory processing and generation
- **Async-First**: Built with Tokio for non-blocking I/O
- **Type-Safe**: Leverages Rust's type system for reliability and performance
- **Trait-Based**: Extensible architecture for custom implementations
- **Production-Ready**: Comprehensive error handling and logging

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- Qdrant instance (for vector storage)
- Watsonx API credentials

### Setup

1. **Start Qdrant**:
```bash
docker run -p 6334:6334 qdrant/qdrant:latest
```

2. **Set environment variables**:
```bash
export WATSONX_API_KEY="your-api-key"
export WATSONX_PROJECT_ID="your-project-id"
```

3. **Add to your `Cargo.toml`**:
```toml
[dependencies]
mem0-rs = { path = "." }
tokio = { version = "1.40", features = ["full"] }
```

### Basic Usage

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
    // Configure
    let config = MemoryConfig::new(
        "http://localhost:6334".to_string(),
        std::env::var("WATSONX_API_KEY")?,
    );

    // Initialize components
    let vector_store = Arc::new(QdrantStore::new(&config.vector_store_url).await?);
    let llm = Arc::new(WatsonxLLM::with_defaults(
        config.watsonx_api_key.clone(),
        config.watsonx_project_id.clone().unwrap_or_default(),
    ));
    let embedder = Arc::new(DefaultEmbedder::with_defaults(
        config.watsonx_api_key.clone(),
        config.watsonx_project_id.clone().unwrap_or_default(),
    ));

    // Create memory
    let memory = Memory::new(config, vector_store, llm, embedder);

    // Add memory
    memory.add("user_123", "I prefer coffee", Some("preference")).await?;

    // Search memories
    let results = memory.search("user_123", "coffee preferences", 3).await?;
    for result in results {
        println!("Found: {} (score: {})", result.memory.content, result.score);
    }

    Ok(())
}
```

## 📚 Documentation

- **[Getting Started](GETTING_STARTED.md)** - Comprehensive setup and usage guide
- **[Advanced Features](ADVANCED_FEATURES.md)** - Deduplication, batching, caching, prompts
- **[Storage Backends](STORAGE_BACKENDS.md)** - Vector store backends (Qdrant, Pinecone, Weaviate)
- **[LLM Providers](LLM_PROVIDERS.md)** - LLM integration (OpenAI, Claude, Watsonx)
- **[Graph Memory](GRAPH_MEMORY.md)** - Knowledge graph integration (Neo4j)
- **[Filtering Guide](FILTERING_GUIDE.md)** - Advanced filtering and queries
- **[Architecture](ARCHITECTURE.md)** - Design decisions and component overview
- **[Migration Summary](MIGRATION_SUMMARY.md)** - Python to Rust migration details
- **[TODO](TODO.md)** - Roadmap and planned features

## 🏗️ Project Structure

```
src/
├── lib.rs              # Main library entry point
├── config.rs           # Configuration types
├── error.rs            # Error handling
├── memory/             # Core memory implementation
│   ├── mod.rs          # Memory traits and types
│   ├── main.rs         # Memory struct
│   ├── dedup.rs        # Deduplication engine
│   └── batch.rs        # Batch operations
├── vector_store/       # Vector store abstraction
│   ├── mod.rs          # VectorStoreBase trait
│   ├── backends.rs     # Backend types and config
│   ├── qdrant.rs       # Qdrant implementation
│   ├── pinecone.rs     # Pinecone implementation
│   └── weaviate.rs     # Weaviate implementation
├── llm/                # LLM integration
│   ├── mod.rs          # LlmBase trait
│   ├── watsonx.rs      # Watsonx implementation
│   ├── openai.rs       # OpenAI implementation
│   ├── claude.rs       # Claude (Anthropic) implementation
│   └── prompts.rs      # Prompt templates
├── embeddings/         # Embedding models
│   ├── mod.rs          # EmbedderBase trait
│   ├── default.rs      # Default embedder
│   └── cache.rs        # Embedding cache (LRU)
├── graph/              # Graph memory
│   ├── mod.rs          # GraphStoreBase trait
│   └── neo4j.rs        # Neo4j implementation
└── filtering/          # Advanced filtering
    └── mod.rs          # Filter DSL and queries
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run ignored tests (requires Qdrant)
cargo test -- --ignored
```

## 📖 Examples

See `examples/` directory:

```bash
cargo run --example basic_usage
```

## 🔧 Configuration

```rust
let config = MemoryConfig::new(url, api_key)
    .with_project_id("project-id")
    .with_llm_model("ibm/granite-4-h-small")
    .with_vector_dimension(384)
    .with_collection_prefix("mem0")
    .with_telemetry(true)
    .with_batch_size(32);
```

## 🎯 Core Operations

### Add Memory
```rust
let item = memory.add(user_id, content, memory_type).await?;
```

### Search Memories
```rust
let results = memory.search(user_id, query, limit).await?;
```

### Update Memory
```rust
let updated = memory.update(memory_id, new_content).await?;
```

### Delete Memory
```rust
memory.delete(memory_id).await?;
```

## 🏃 Performance

- **Async-first**: Non-blocking I/O with Tokio
- **Batch operations**: Efficient vector store operations
- **Caching**: Embedding caching for frequently used texts
- **Type-safe**: Zero-cost abstractions

## 🤝 Contributing

Contributions welcome! Please ensure:

- `cargo build` succeeds
- `cargo test` passes
- Code follows Rust conventions

## 📄 License

Apache 2.0 - See [LICENSE](LICENSE) file

## 🔗 Related

- [Mem0 (Python)](https://github.com/mem0ai/mem0)
- [Qdrant](https://qdrant.tech/)
- [Watsonx](https://www.ibm.com/watsonx)
