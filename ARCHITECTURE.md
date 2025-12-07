# memory-rs Architecture

## Overview

memory-rs is a Rust implementation of long-term memory for AI agents. It provides:

1. **Memory Management**: Store and retrieve user/agent memories
2. **Vector Search**: Semantic search using in-memory vector store
3. **Local Embeddings**: SHA256-based hash embeddings
4. **Persistent Storage**: SQLite database for metadata
5. **Async Operations**: Non-blocking I/O with Tokio
6. **MCP Server**: Model Context Protocol server for AI assistant integration
7. **CLI Tools**: Command-line interface for memory management

## Binaries

- **memory-mcp**: MCP server for AI assistant integration (STDIO transport)
- **memory-cli**: CLI for memory management operations

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

**InMemoryStore**: In-memory implementation
- Stores vectors in memory with RwLock for thread-safety
- Manages collections per user/agent
- Implements cosine similarity search

### Embeddings Module (`src/embeddings/`)

**EmbedderBase Trait**: Abstract embedder interface
- `embed()` - Generate embeddings

**LocalEmbedder**: Local implementation
- Uses SHA256-based hashing
- No external dependencies
- Deterministic embeddings
- Default embedder

## Data Flow

1. **Add Memory**:
   - User provides text
   - Generate embeddings (local)
   - Store in vector database with metadata

2. **Search Memory**:
   - User provides query
   - Generate query embedding
   - Search vector store
   - Return relevant memories with scores

3. **Update Memory**:
   - Retrieve existing memory
   - Update vector store with new embedding

## Design Patterns

- **Trait-Based Abstraction**: All major components use traits for testability
- **Async-First**: All I/O operations are async
- **Error Handling**: Comprehensive error types with context
- **Configuration**: Externalized configuration for flexibility

## Testing Strategy

- Unit tests for core logic
- In-memory vector store tests
- Snapshot tests with insta
- Mock implementations for LLM/embeddings

### MCP Module (`src/mcp/`)

**MemoryMcpServer**: MCP server implementation
- Exposes memory operations as MCP tools
- Uses STDIO transport for communication
- JSON Schema generation for tool inputs

### Utils Module (`src/utils/`)

**Shared Utilities**:
- `compute_hash()` - SHA256 hash computation
- `cosine_similarity()` - Vector similarity calculation

## Future Enhancements

- Additional vector store backends (Milvus, PostgreSQL)
- REST/GraphQL API
- Migration utilities
- Performance profiling
