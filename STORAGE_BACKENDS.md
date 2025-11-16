# Storage Backends Guide - mem0-rs

## Overview

mem0-rs supports multiple vector store backends for semantic memory storage. Each backend has different characteristics, trade-offs, and use cases.

## Supported Backends

### 1. Qdrant (Built-in)

**Status**: ✅ Implemented (Placeholder)

**Characteristics**:
- Open-source vector database
- Excellent for development and testing
- Local deployment option
- REST and gRPC APIs
- Filtering and metadata support

**Setup**:
```bash
docker run -p 6334:6334 qdrant/qdrant:latest
```

**Usage**:
```rust
use mem0_rs::vector_store::qdrant::QdrantStore;

let store = QdrantStore::new("http://localhost:6334").await?;
```

**Best For**:
- Development and prototyping
- Self-hosted deployments
- Full control over infrastructure

### 2. Pinecone

**Status**: ✅ Implemented

**Characteristics**:
- Managed vector database service
- Serverless, fully managed
- High availability and scalability
- Metadata filtering
- Real-time indexing

**Setup**:
```bash
# Create account at https://www.pinecone.io
# Get API key and index name
```

**Usage**:
```rust
use mem0_rs::vector_store::pinecone::PineconeStore;

let store = PineconeStore::new(
    "your-api-key".to_string(),
    "your-index-name".to_string(),
    "https://api.pinecone.io".to_string(),
).await?;
```

**Configuration**:
```rust
use mem0_rs::vector_store::{BackendType, BackendConfig};

let config = BackendConfig::new(
    BackendType::Pinecone,
    "https://api.pinecone.io".to_string(),
)
.with_api_key("your-api-key".to_string())
.with_config("dimension".to_string(), "1536".to_string());
```

**Best For**:
- Production deployments
- Managed infrastructure
- High availability requirements
- Scalability needs

**Pricing**: Pay-as-you-go based on storage and queries

### 3. Weaviate

**Status**: ✅ Implemented

**Characteristics**:
- Open-source vector search engine
- GraphQL API
- Modular architecture
- Hybrid search (vector + keyword)
- Flexible deployment

**Setup**:
```bash
docker run -p 8080:8080 semitechnologies/weaviate:latest
```

**Usage**:
```rust
use mem0_rs::vector_store::weaviate::WeaviateStore;

let store = WeaviateStore::new("http://localhost:8080".to_string()).await?;
```

**With API Key**:
```rust
let store = WeaviateStore::with_api_key(
    "http://localhost:8080".to_string(),
    "your-api-key".to_string(),
).await?;
```

**Best For**:
- Hybrid search requirements
- Custom vectorization
- GraphQL preference
- Open-source deployments

### 4. Milvus

**Status**: 📋 Planned

**Characteristics**:
- Open-source vector database
- High performance
- Scalable architecture
- Multiple index types
- Cloud and self-hosted options

**Planned Usage**:
```rust
use mem0_rs::vector_store::milvus::MilvusStore;

let store = MilvusStore::new("localhost:19530").await?;
```

### 5. Chroma

**Status**: 📋 Planned

**Characteristics**:
- Lightweight vector database
- Easy to use
- Built-in embeddings
- Persistent storage
- Great for small to medium deployments

**Planned Usage**:
```rust
use mem0_rs::vector_store::chroma::ChromaStore;

let store = ChromaStore::new("http://localhost:8000").await?;
```

### 6. PostgreSQL/pgvector

**Status**: 📋 Planned

**Characteristics**:
- SQL database with vector extension
- Familiar SQL interface
- ACID transactions
- Hybrid queries
- Cost-effective for small scale

**Planned Usage**:
```rust
use mem0_rs::vector_store::postgres::PostgresStore;

let store = PostgresStore::new("postgresql://user:pass@localhost/db").await?;
```

## Backend Comparison

| Feature | Qdrant | Pinecone | Weaviate | Milvus | Chroma | PostgreSQL |
|---------|--------|----------|----------|--------|--------|------------|
| Open Source | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ |
| Managed | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ |
| Self-Hosted | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ |
| Filtering | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Metadata | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Scalability | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |
| Ease of Use | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ |
| Cost | Free | Paid | Free | Free | Free | Free |

## Choosing a Backend

### Development
- **Recommended**: Qdrant or Chroma
- **Reason**: Easy to set up locally, no costs

### Small Production
- **Recommended**: Chroma or PostgreSQL
- **Reason**: Simple, cost-effective, sufficient performance

### Medium Production
- **Recommended**: Weaviate or Milvus
- **Reason**: Good balance of features and scalability

### Large Scale Production
- **Recommended**: Pinecone
- **Reason**: Fully managed, high availability, proven scalability

### Hybrid Search
- **Recommended**: Weaviate
- **Reason**: Native support for vector + keyword search

## Backend Configuration

### Using BackendConfig

```rust
use mem0_rs::vector_store::{BackendType, BackendConfig};

let config = BackendConfig::new(
    BackendType::Pinecone,
    "https://api.pinecone.io".to_string(),
)
.with_api_key("your-api-key".to_string())
.with_config("dimension".to_string(), "1536".to_string())
.with_config("metric".to_string(), "cosine".to_string());

// Use config to initialize store
```

### Backend Types

```rust
use mem0_rs::vector_store::BackendType;

// Available backends
let qdrant = BackendType::Qdrant;
let pinecone = BackendType::Pinecone;
let weaviate = BackendType::Weaviate;
let milvus = BackendType::Milvus;
let postgresql = BackendType::PostgreSQL;

// Get backend info
println!("Name: {}", qdrant.name());
println!("Description: {}", qdrant.description());
```

## Performance Characteristics

### Latency (ms)
- **Qdrant**: 10-50ms (local), 50-100ms (remote)
- **Pinecone**: 50-200ms (managed)
- **Weaviate**: 20-100ms (local), 100-300ms (remote)
- **Milvus**: 10-50ms (local)
- **Chroma**: 5-20ms (local)
- **PostgreSQL**: 20-100ms (depends on index)

### Throughput (ops/sec)
- **Qdrant**: 1000-10000
- **Pinecone**: 10000-100000+ (managed)
- **Weaviate**: 1000-5000
- **Milvus**: 5000-50000
- **Chroma**: 100-1000
- **PostgreSQL**: 1000-10000

### Storage Efficiency
- **Qdrant**: Excellent (compressed)
- **Pinecone**: Good (managed)
- **Weaviate**: Good
- **Milvus**: Excellent
- **Chroma**: Good
- **PostgreSQL**: Fair (depends on index)

## Migration Between Backends

### Export from One Backend

```rust
let source_store = QdrantStore::new("http://localhost:6334").await?;
let vectors = source_store.search("collection", vec![0.1; 384], 10000, None).await?;
```

### Import to Another Backend

```rust
let target_store = PineconeStore::new(
    "api-key".to_string(),
    "index".to_string(),
    "endpoint".to_string(),
).await?;

for result in vectors {
    target_store.upsert("collection", vec![(
        result.id,
        vec![0.1; 384],
        result.metadata,
    )]).await?;
}
```

## Troubleshooting

### Connection Issues
- Verify endpoint URL
- Check API keys
- Ensure network connectivity
- Check firewall rules

### Performance Issues
- Verify vector dimension matches
- Check batch size
- Monitor network latency
- Consider backend capacity

### Data Consistency
- Use transactions where available
- Implement retry logic
- Monitor sync status
- Regular backups

## Best Practices

1. **Choose Based on Requirements**
   - Start with development backend
   - Migrate to production backend as needed

2. **Monitor Performance**
   - Track query latency
   - Monitor storage usage
   - Watch error rates

3. **Implement Fallbacks**
   - Have backup backend
   - Implement retry logic
   - Cache frequently used vectors

4. **Optimize Configuration**
   - Right vector dimension
   - Appropriate batch sizes
   - Proper indexing strategy

5. **Security**
   - Use API keys securely
   - Encrypt data in transit
   - Implement access controls
   - Regular security audits

## Future Backends

Planned backends for future releases:
- Milvus (Phase 8)
- Chroma (Phase 8)
- PostgreSQL/pgvector (Phase 8)
- Elasticsearch (Phase 9)
- OpenSearch (Phase 9)
- Redis Stack (Phase 9)

## Examples

### Multi-Backend Setup

```rust
use mem0_rs::vector_store::{
    qdrant::QdrantStore,
    pinecone::PineconeStore,
    weaviate::WeaviateStore,
};

// Development
let dev_store = QdrantStore::new("http://localhost:6334").await?;

// Production
let prod_store = PineconeStore::new(
    std::env::var("PINECONE_API_KEY")?,
    "prod-index".to_string(),
    "https://api.pinecone.io".to_string(),
).await?;

// Fallback
let fallback_store = WeaviateStore::new("http://backup:8080".to_string()).await?;
```

### Backend Abstraction

```rust
use mem0_rs::vector_store::VectorStoreBase;
use std::sync::Arc;

async fn initialize_store(backend: &str) -> Result<Arc<dyn VectorStoreBase>> {
    match backend {
        "qdrant" => Ok(Arc::new(QdrantStore::new("http://localhost:6334").await?)),
        "pinecone" => Ok(Arc::new(PineconeStore::new(
            std::env::var("PINECONE_API_KEY")?,
            "index".to_string(),
            "https://api.pinecone.io".to_string(),
        ).await?)),
        "weaviate" => Ok(Arc::new(WeaviateStore::new("http://localhost:8080".to_string()).await?)),
        _ => Err("Unknown backend".into()),
    }
}
```

## See Also

- [README.md](README.md) - Main documentation
- [ADVANCED_FEATURES.md](ADVANCED_FEATURES.md) - Advanced features
- [ARCHITECTURE.md](ARCHITECTURE.md) - Architecture overview
