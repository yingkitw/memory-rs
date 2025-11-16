# mem0-rs Architecture

## Overview

mem0-rs is a Rust implementation of the Mem0 memory layer for AI agents. It provides:

1. **Memory Management**: Store and retrieve user/agent memories
2. **Vector Search**: Semantic search using Qdrant
3. **LLM Integration**: Process memories with Watsonx
4. **Async Operations**: Non-blocking I/O with Tokio

## Core Components

### Memory Module (`src/memory/`)

**MemoryBase Trait**: Abstract interface for memory operations
- `add()` - Add new memories
- `search()` - Search memories semantically
- `update()` - Update existing memories
- `delete()` - Remove memories

**Memory Struct**: Main implementation
- Manages memory lifecycle
- Coordinates with vector store and LLM
- Handles metadata and filtering

### Vector Store Module (`src/vector_store/`)

**VectorStoreBase Trait**: Abstract vector store interface
- `create_collection()` - Initialize collection
- `upsert()` - Add/update vectors
- `search()` - Semantic search
- `delete()` - Remove vectors

**QdrantStore**: Qdrant implementation
- Uses Qdrant client for vector operations
- Manages collections per user/agent
- Handles vector embeddings

### LLM Module (`src/llm/`)

**LlmBase Trait**: Abstract LLM interface
- `generate()` - Generate text
- `generate_stream()` - Stream generation

**WatsonxLLM**: Watsonx implementation
- Uses watsonx-rs SDK
- Supports streaming responses
- Configurable model selection

### Embeddings Module (`src/embeddings/`)

**EmbedderBase Trait**: Abstract embedder interface
- `embed()` - Generate embeddings

**DefaultEmbedder**: Default implementation
- Uses Watsonx for embeddings
- Caches embeddings

## Data Flow

1. **Add Memory**:
   - User provides text
   - LLM extracts facts/insights
   - Generate embeddings
   - Store in vector database with metadata

2. **Search Memory**:
   - User provides query
   - Generate query embedding
   - Search vector store
   - Return relevant memories with scores

3. **Update Memory**:
   - Retrieve existing memory
   - LLM processes update
   - Update vector store

## Design Patterns

- **Trait-Based Abstraction**: All major components use traits for testability
- **Async-First**: All I/O operations are async
- **Error Handling**: Comprehensive error types with context
- **Configuration**: Externalized configuration for flexibility

## Testing Strategy

- Unit tests for core logic
- Integration tests with Qdrant
- Snapshot tests with insta
- Mock implementations for LLM/embeddings

## Future Enhancements

- Graph-based memory relationships
- Multiple vector store backends
- Advanced filtering and aggregation
- Distributed memory management
