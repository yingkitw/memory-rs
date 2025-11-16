# Graph Memory Guide - mem0-rs

## Overview

Graph memory enables mem0-rs to store and traverse relationships between memories, creating a knowledge graph that captures how different pieces of information relate to each other.

## Concepts

### Graph Nodes

Nodes represent individual memories in the graph. Each node contains:
- **ID**: Unique identifier
- **Content**: Memory text
- **Labels**: Categories (e.g., Memory, Fact, Preference)
- **Properties**: Additional metadata

### Graph Relationships

Relationships connect nodes and represent how memories relate to each other:
- **RelatedTo**: General relationship
- **Contradicts**: Conflicting information
- **Supports**: Supporting evidence
- **PartOf**: Hierarchical relationship
- **Contains**: Containment relationship
- **Custom**: User-defined relationships

### Knowledge Graph

A knowledge graph is a collection of nodes and relationships that form a network of interconnected memories.

## Supported Backends

### 1. Neo4j

**Status**: ✅ Implemented

**Characteristics**:
- Industry-leading graph database
- Cypher query language
- Excellent performance
- ACID transactions
- Enterprise support

**Setup**:
```bash
docker run -p 7474:7474 -p 7687:7687 neo4j:latest
```

**Usage**:
```rust
use mem0_rs::graph::neo4j::Neo4jStore;

let graph = Neo4jStore::new(
    "http://localhost:7474".to_string(),
    "neo4j".to_string(),
    "password".to_string(),
).await?;
```

**Best For**:
- Complex relationship queries
- Large-scale knowledge graphs
- Enterprise deployments
- Advanced graph analytics

### 2. Memgraph

**Status**: 📋 Planned

**Characteristics**:
- Open-source graph database
- Cypher compatible
- High performance
- In-memory processing
- Lightweight

**Planned Usage**:
```rust
use mem0_rs::graph::memgraph::MemgraphStore;

let graph = MemgraphStore::new("bolt://localhost:7687".to_string()).await?;
```

## Core Operations

### Creating Nodes

```rust
use mem0_rs::graph::{GraphNode, GraphStoreBase};

let node = GraphNode {
    id: "memory_1".to_string(),
    content: "User likes coffee".to_string(),
    labels: vec!["Memory".to_string(), "Preference".to_string()],
    properties: std::collections::HashMap::new(),
};

graph.create_node(node).await?;
```

### Creating Relationships

```rust
use mem0_rs::graph::{GraphRelationship, RelationType};

let relationship = GraphRelationship {
    source_id: "memory_1".to_string(),
    target_id: "memory_2".to_string(),
    rel_type: RelationType::RelatedTo,
    properties: std::collections::HashMap::new(),
};

graph.create_relationship(relationship).await?;
```

### Querying Relationships

```rust
// Get all relationships for a node
let relationships = graph.get_relationships("memory_1").await?;

for rel in relationships {
    println!("Related to: {}", rel.target_id);
}
```

### Finding Paths

```rust
// Find shortest path between two memories
let path = graph.find_path("memory_1", "memory_5", 3).await?;

println!("Path: {:?}", path);
```

### Updating Nodes

```rust
use std::collections::HashMap;

let mut properties = HashMap::new();
properties.insert("updated".to_string(), "true".to_string());

graph.update_node("memory_1", properties).await?;
```

### Deleting Nodes

```rust
graph.delete_node("memory_1").await?;
```

## Relationship Types

### Built-in Relationships

```rust
use mem0_rs::graph::RelationType;

// General relationship
RelationType::RelatedTo

// Conflicting information
RelationType::Contradicts

// Supporting evidence
RelationType::Supports

// Hierarchical
RelationType::PartOf
RelationType::Contains

// Custom relationship
RelationType::Custom("my_relation".to_string())
```

## Integration with Memory

### Adding Memories with Relationships

```rust
use mem0_rs::{memory::Memory, graph::GraphStoreBase};

// Add memory
let item = memory.add("user_123", "I like coffee", Some("preference")).await?;

// Create graph node
let node = GraphNode {
    id: item.id.clone(),
    content: item.content.clone(),
    labels: vec!["Memory".to_string()],
    properties: std::collections::HashMap::new(),
};

graph.create_node(node).await?;

// Create relationship to related memory
let rel = GraphRelationship {
    source_id: item.id,
    target_id: "memory_2".to_string(),
    rel_type: RelationType::RelatedTo,
    properties: std::collections::HashMap::new(),
};

graph.create_relationship(rel).await?;
```

### Searching with Graph Context

```rust
// Search for memory
let results = memory.search("user_123", "coffee", 5).await?;

// Get related memories from graph
for result in results {
    let relationships = graph.get_relationships(&result.memory.id).await?;
    
    for rel in relationships {
        println!("Related memory: {}", rel.target_id);
    }
}
```

## Advanced Queries

### Finding Similar Memories

```rust
// Find all memories related to a specific memory
let relationships = graph.get_relationships("memory_1").await?;

for rel in relationships {
    if rel.rel_type == RelationType::RelatedTo {
        let related = graph.get_node(&rel.target_id).await?;
        println!("Similar: {:?}", related);
    }
}
```

### Detecting Contradictions

```rust
// Find contradicting memories
let relationships = graph.get_relationships("memory_1").await?;

for rel in relationships {
    if rel.rel_type == RelationType::Contradicts {
        println!("Contradiction found: {}", rel.target_id);
    }
}
```

### Building Knowledge Hierarchies

```rust
// Create hierarchical relationships
let parent_rel = GraphRelationship {
    source_id: "memory_1".to_string(),
    target_id: "memory_2".to_string(),
    rel_type: RelationType::Contains,
    properties: std::collections::HashMap::new(),
};

graph.create_relationship(parent_rel).await?;
```

## Performance Characteristics

### Query Latency (ms)
- **Node creation**: 1-5ms
- **Relationship creation**: 1-5ms
- **Path finding**: 10-100ms
- **Relationship query**: 5-20ms

### Scalability
- **Nodes**: Millions
- **Relationships**: Tens of millions
- **Query depth**: Unlimited (with timeout)

### Storage
- **Per node**: ~500 bytes
- **Per relationship**: ~300 bytes

## Best Practices

1. **Use Meaningful Labels**
   - Categorize nodes clearly
   - Use consistent naming
   - Enable efficient filtering

2. **Index Important Properties**
   - Index frequently queried fields
   - Improves query performance
   - Reduces memory usage

3. **Manage Relationship Density**
   - Avoid creating too many relationships
   - Use relationship types effectively
   - Clean up stale relationships

4. **Optimize Path Finding**
   - Limit search depth
   - Use intermediate nodes
   - Cache common paths

5. **Monitor Graph Health**
   - Track node/relationship counts
   - Monitor query performance
   - Regular maintenance

## Troubleshooting

### Connection Issues
- Verify Neo4j is running
- Check connection URI
- Verify credentials
- Check network connectivity

### Query Performance
- Add indexes to frequently queried properties
- Limit path search depth
- Use relationship type filtering
- Monitor query execution time

### Memory Issues
- Limit relationship density
- Archive old memories
- Use graph pruning
- Monitor storage usage

## Example: Building a Knowledge Graph

```rust
use mem0_rs::graph::{GraphNode, GraphRelationship, RelationType, GraphStoreBase};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let graph = Neo4jStore::new(
        "http://localhost:7474".to_string(),
        "neo4j".to_string(),
        "password".to_string(),
    ).await?;

    // Create nodes
    let coffee_node = GraphNode {
        id: "coffee".to_string(),
        content: "Coffee is a beverage".to_string(),
        labels: vec!["Concept".to_string()],
        properties: HashMap::new(),
    };

    let preference_node = GraphNode {
        id: "user_preference".to_string(),
        content: "User prefers coffee".to_string(),
        labels: vec!["Preference".to_string()],
        properties: HashMap::new(),
    };

    graph.create_node(coffee_node).await?;
    graph.create_node(preference_node).await?;

    // Create relationship
    let rel = GraphRelationship {
        source_id: "user_preference".to_string(),
        target_id: "coffee".to_string(),
        rel_type: RelationType::RelatedTo,
        properties: HashMap::new(),
    };

    graph.create_relationship(rel).await?;

    // Query relationships
    let relationships = graph.get_relationships("user_preference").await?;
    println!("Found {} relationships", relationships.len());

    Ok(())
}
```

## Future Enhancements

### Planned Features
- Memgraph integration
- Graph analytics
- Community detection
- Centrality analysis
- Recommendation engine

### Advanced Capabilities
- Temporal graphs
- Weighted relationships
- Graph embeddings
- Pattern matching
- Graph algorithms

## See Also

- [README.md](README.md) - Main documentation
- [STORAGE_BACKENDS.md](STORAGE_BACKENDS.md) - Storage backends
- [LLM_PROVIDERS.md](LLM_PROVIDERS.md) - LLM providers
- [ADVANCED_FEATURES.md](ADVANCED_FEATURES.md) - Advanced features
