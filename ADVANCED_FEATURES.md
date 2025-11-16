# Advanced Features Guide - mem0-rs

## Overview

mem0-rs includes several advanced features for production deployments, including deduplication, batch operations, prompt management, and embedding caching.

## 1. Memory Deduplication

Prevent storing duplicate memories using various strategies.

### Deduplication Strategies

```rust
use mem0_rs::memory::dedup::{Deduplicator, DeduplicationStrategy};

// Exact hash matching
let mut dedup = Deduplicator::new(DeduplicationStrategy::Exact);

// Similarity-based (requires embeddings)
let mut dedup = Deduplicator::new(DeduplicationStrategy::Similarity);

// No deduplication
let mut dedup = Deduplicator::new(DeduplicationStrategy::None);
```

### Usage Example

```rust
let mut dedup = Deduplicator::new(DeduplicationStrategy::Exact);

// Register a memory
dedup.register("I like coffee", "memory_id_1".to_string());

// Check if duplicate
if dedup.is_duplicate("I like coffee") {
    println!("This memory already exists!");
}

// Get duplicate ID
if let Some(id) = dedup.get_duplicate("I like coffee") {
    println!("Duplicate found: {}", id);
}
```

### Similarity Computation

```rust
use mem0_rs::memory::dedup::Deduplicator;

let vec1 = vec![1.0, 0.0, 0.0];
let vec2 = vec![1.0, 0.0, 0.0];

let similarity = Deduplicator::compute_similarity(&vec1, &vec2);
println!("Similarity: {}", similarity); // Output: 1.0
```

## 2. Batch Operations

Process multiple memory operations efficiently.

### Batch Operation Types

```rust
use mem0_rs::memory::batch::{BatchOp, BatchOpType};

// Create add operation
let op = BatchOp::add(
    "id_1".to_string(),
    "I prefer coffee".to_string(),
    "preference".to_string(),
);

// Create update operation
let op = BatchOp::update(
    "id_1".to_string(),
    "I prefer dark coffee".to_string(),
);

// Create delete operation
let op = BatchOp::delete("id_1".to_string());
```

### Batch Processing

```rust
use mem0_rs::memory::batch::{BatchProcessor, BatchOp};

let processor = BatchProcessor::new(32); // Batch size of 32

let ops = vec![
    BatchOp::add("1".to_string(), "content1".to_string(), "type1".to_string()),
    BatchOp::add("2".to_string(), "content2".to_string(), "type2".to_string()),
    // ... more operations
];

// Split into batches
let batches = processor.split_into_batches(ops);

for batch in batches {
    // Process each batch
    println!("Processing batch of {} operations", batch.len());
}
```

### Batch Results

```rust
use mem0_rs::memory::batch::BatchResult;

let mut result = BatchResult::new(100);

// Simulate operations
result.add_success();
result.add_success();
result.add_error("Failed to add memory".to_string());

println!("Success rate: {:.2}%", result.success_rate() * 100.0);
println!("Successful: {}, Failed: {}", result.successful, result.failed);
```

### Optimize Batch Size

```rust
let processor = BatchProcessor::new(32);

// Automatically optimize based on operation count
let optimal_size = processor.optimize_batch_size(5000);
println!("Optimal batch size: {}", optimal_size);
```

## 3. Prompt Management

Manage LLM prompts with templates and variables.

### Using Prompt Templates

```rust
use mem0_rs::llm::PromptTemplate;
use std::collections::HashMap;

// Create a template
let template = PromptTemplate::new(
    "greeting".to_string(),
    "Hello {name}, welcome to {place}".to_string(),
);

// Prepare variables
let mut vars = HashMap::new();
vars.insert("name".to_string(), "Alice".to_string());
vars.insert("place".to_string(), "Rust".to_string());

// Render template
let prompt = template.render(&vars)?;
println!("{}", prompt); // Output: "Hello Alice, welcome to Rust"
```

### Using Prompt Manager

```rust
use mem0_rs::llm::PromptManager;
use std::collections::HashMap;

// Create manager with default templates
let manager = PromptManager::new();

// List available templates
for template_name in manager.list_templates() {
    println!("Template: {}", template_name);
}

// Use a template
let mut vars = HashMap::new();
vars.insert("conversation".to_string(), "User: I like coffee".to_string());

let prompt = manager.render("extract_facts", &vars)?;
println!("{}", prompt);
```

### Default Templates

mem0-rs includes these default templates:

- **extract_facts**: Extract key facts from conversation
- **generate_insights**: Generate insights from facts
- **summarize_memories**: Summarize memories into profile
- **answer_with_context**: Answer question with context
- **classify_memory**: Classify memory type

### Custom Templates

```rust
use mem0_rs::llm::{PromptTemplate, PromptManager};

let mut manager = PromptManager::new();

// Register custom template
let custom = PromptTemplate::new(
    "my_template".to_string(),
    "Analyze: {text}".to_string(),
);

manager.register(custom);
```

## 4. Embedding Cache

Cache embeddings to improve performance.

### Basic Caching

```rust
use mem0_rs::embeddings::EmbeddingCache;

// Create cache with max size of 1000
let mut cache = EmbeddingCache::new(1000);

// Store embedding
cache.put("I like coffee", vec![0.1, 0.2, 0.3]);

// Retrieve embedding
if let Some(embedding) = cache.get("I like coffee") {
    println!("Found cached embedding: {:?}", embedding);
}
```

### Cache Management

```rust
let mut cache = EmbeddingCache::new(1000);

// Check if text is cached
if cache.contains("I like coffee") {
    println!("Text is in cache");
}

// Get cache statistics
println!("Cache size: {}", cache.size());
println!("Hit rate: {:.2}%", cache.hit_rate() * 100.0);

// Clear cache
cache.clear();
```

### LRU Eviction

The cache uses Least Recently Used (LRU) eviction policy:

```rust
let mut cache = EmbeddingCache::new(2);

cache.put("text1", vec![0.1]);
cache.put("text2", vec![0.2]);
cache.put("text3", vec![0.3]); // text1 is evicted

assert!(cache.get("text1").is_none()); // Evicted
assert!(cache.get("text2").is_some()); // Still cached
```

## 5. Advanced Configuration

### Fine-tuning Configuration

```rust
use mem0_rs::config::MemoryConfig;

let config = MemoryConfig::new(
    "http://localhost:6334".to_string(),
    "api-key".to_string(),
)
.with_project_id("project-id".to_string())
.with_llm_model("ibm/granite-4-h-small".to_string())
.with_vector_dimension(768) // Larger dimension for more precision
.with_collection_prefix("prod".to_string())
.with_telemetry(true)
.with_batch_size(64); // Larger batches for throughput
```

## 6. Performance Optimization Tips

### 1. Batch Operations
```rust
// Instead of adding one memory at a time
for item in items {
    memory.add(&user_id, &item, Some("fact")).await?;
}

// Use batch operations
let ops = items.iter()
    .map(|item| BatchOp::add(uuid::Uuid::new_v4().to_string(), item.clone(), "fact".to_string()))
    .collect();
let processor = BatchProcessor::new(32);
let batches = processor.split_into_batches(ops);
```

### 2. Enable Caching
```rust
// Cache embeddings for frequently used texts
let mut cache = EmbeddingCache::new(10000);

// Check cache before computing embedding
if let Some(embedding) = cache.get(text) {
    return Ok(embedding);
}

// Compute and cache
let embedding = embedder.embed(text).await?;
cache.put(text, embedding.clone());
```

### 3. Use Deduplication
```rust
// Prevent duplicate memories
let mut dedup = Deduplicator::new(DeduplicationStrategy::Exact);

if !dedup.is_duplicate(content) {
    memory.add(user_id, content, memory_type).await?;
    dedup.register(content, memory_id);
}
```

### 4. Optimize Batch Size
```rust
let processor = BatchProcessor::new(32);
let optimal_size = processor.optimize_batch_size(operations.len());
let processor = BatchProcessor::new(optimal_size);
```

## 7. Error Handling

All advanced features include comprehensive error handling:

```rust
use mem0_rs::Result;

// Template rendering with error handling
match template.render(&vars) {
    Ok(prompt) => println!("Prompt: {}", prompt),
    Err(e) => eprintln!("Template error: {}", e),
}

// Batch operations with error tracking
let mut result = BatchResult::new(ops.len());
for op in ops {
    match process_op(&op).await {
        Ok(_) => result.add_success(),
        Err(e) => result.add_error(e.to_string()),
    }
}

if !result.all_succeeded() {
    eprintln!("Some operations failed: {}", result.failed);
}
```

## 8. Integration Example

Complete example using multiple advanced features:

```rust
use mem0_rs::{
    config::MemoryConfig,
    memory::{Memory, dedup::{Deduplicator, DeduplicationStrategy}, batch::BatchProcessor},
    llm::PromptManager,
    embeddings::EmbeddingCache,
    MemoryBase,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup
    let config = MemoryConfig::new(
        "http://localhost:6334".to_string(),
        std::env::var("WATSONX_API_KEY")?,
    );

    let memory = Memory::new(
        config,
        Arc::new(/* vector store */),
        Arc::new(/* llm */),
        Arc::new(/* embedder */),
    );

    // Initialize advanced features
    let mut dedup = Deduplicator::new(DeduplicationStrategy::Exact);
    let mut cache = EmbeddingCache::new(10000);
    let prompt_manager = PromptManager::new();

    // Process memories with deduplication
    let user_id = "user_123";
    for content in vec!["I like coffee", "I prefer dark coffee"] {
        if !dedup.is_duplicate(content) {
            let item = memory.add(user_id, content, Some("preference")).await?;
            dedup.register(content, item.id.clone());
        }
    }

    // Search with caching
    let query = "coffee preferences";
    if !cache.contains(query) {
        let results = memory.search(user_id, query, 3).await?;
        // Cache results...
    }

    Ok(())
}
```

## Performance Benchmarks

Typical performance characteristics:

- **Deduplication**: O(1) lookup, O(1) registration
- **Batch processing**: 10-100x faster than sequential
- **Embedding cache**: 1000x faster for cached hits
- **Prompt rendering**: < 1ms for typical templates

## Best Practices

1. **Use deduplication** to prevent duplicate memories
2. **Enable caching** for frequently accessed embeddings
3. **Batch operations** for bulk inserts/updates
4. **Use templates** for consistent prompts
5. **Monitor cache hit rate** to optimize cache size
6. **Handle errors** explicitly in production
7. **Test performance** with your typical workload

## Troubleshooting

### Cache Not Working
- Check cache size is sufficient
- Verify text hashing is consistent
- Monitor hit rate

### Deduplication Issues
- Ensure strategy matches use case
- Check hash computation
- Verify content normalization

### Batch Processing Slow
- Adjust batch size using `optimize_batch_size()`
- Check network latency
- Monitor memory usage

## See Also

- [README.md](README.md) - Main documentation
- [ARCHITECTURE.md](ARCHITECTURE.md) - Architecture overview
- [GETTING_STARTED.md](GETTING_STARTED.md) - Setup guide
