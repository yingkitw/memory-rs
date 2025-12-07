//! CLI for memory-rs management
//!
//! Usage:
//!   memory-cli add --user <USER_ID> --content <CONTENT> [--type <TYPE>]
//!   memory-cli search --user <USER_ID> --query <QUERY> [--limit <N>]
//!   memory-cli list --user <USER_ID>
//!   memory-cli delete --id <MEMORY_ID>
//!   memory-cli export --user <USER_ID> [--output <FILE>]
//!   memory-cli import --user <USER_ID> --input <FILE>

use std::sync::Arc;

use clap::{Parser, Subcommand};
use memory_rs::{
    config::MemoryConfig,
    embeddings::LocalEmbedder,
    memory::{Memory, MemoryBase},
    vector_store::InMemoryStore,
};

#[derive(Parser)]
#[command(name = "memory-cli")]
#[command(about = "CLI for memory-rs - Long-term memory for AI Agents")]
#[command(version)]
struct Cli {
    /// Database path
    #[arg(short, long, default_value = "memory.db")]
    database: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new memory
    Add {
        /// User ID
        #[arg(short, long)]
        user: String,

        /// Memory content
        #[arg(short, long)]
        content: String,

        /// Memory type (e.g., fact, preference, insight)
        #[arg(short = 't', long, default_value = "general")]
        memory_type: String,
    },

    /// Search memories
    Search {
        /// User ID
        #[arg(short, long)]
        user: String,

        /// Search query
        #[arg(short, long)]
        query: String,

        /// Maximum results
        #[arg(short, long, default_value = "5")]
        limit: usize,
    },

    /// List all memories for a user
    List {
        /// User ID
        #[arg(short, long)]
        user: String,
    },

    /// Delete a memory
    Delete {
        /// Memory ID
        #[arg(short, long)]
        id: String,
    },

    /// Export memories to JSON
    Export {
        /// User ID
        #[arg(short, long)]
        user: String,

        /// Output file (default: stdout)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Import memories from JSON
    Import {
        /// User ID
        #[arg(short, long)]
        user: String,

        /// Input file
        #[arg(short, long)]
        input: String,
    },

    /// Show memory statistics
    Stats {
        /// User ID (optional, shows all if not specified)
        #[arg(short, long)]
        user: Option<String>,
    },
}

fn create_memory(db_path: &str) -> Memory {
    let config = MemoryConfig::new(db_path.to_string());
    let vector_store = Arc::new(InMemoryStore::new());
    let embedder = Arc::new(LocalEmbedder::with_defaults());
    Memory::new(config, vector_store, embedder)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let memory = create_memory(&cli.database);

    match cli.command {
        Commands::Add {
            user,
            content,
            memory_type,
        } => {
            let item = memory.add(&user, &content, Some(&memory_type)).await?;
            println!("Added memory:");
            println!("  ID: {}", item.id);
            println!("  User: {}", item.user_id);
            println!("  Type: {}", item.memory_type);
            println!("  Content: {}", item.content);
        }

        Commands::Search { user, query, limit } => {
            let results = memory.search(&user, &query, limit).await?;
            if results.is_empty() {
                println!("No memories found for query: {}", query);
            } else {
                println!("Found {} memories:", results.len());
                for (i, result) in results.iter().enumerate() {
                    println!("\n{}. [Score: {:.3}]", i + 1, result.score);
                    println!("   ID: {}", result.memory.id);
                    println!("   Type: {}", result.memory.memory_type);
                    println!("   Content: {}", result.memory.content);
                }
            }
        }

        Commands::List { user } => {
            let memories = memory.get_all(&user).await?;
            if memories.is_empty() {
                println!("No memories found for user: {}", user);
            } else {
                println!("Found {} memories for user {}:", memories.len(), user);
                for (i, mem) in memories.iter().enumerate() {
                    println!("\n{}. {}", i + 1, mem.id);
                    println!("   Type: {}", mem.memory_type);
                    println!("   Content: {}", mem.content);
                    println!("   Created: {}", mem.created_at);
                }
            }
        }

        Commands::Delete { id } => {
            memory.delete(&id).await?;
            println!("Deleted memory: {}", id);
        }

        Commands::Export { user, output } => {
            let memories = memory.get_all(&user).await?;
            let json = serde_json::to_string_pretty(&memories)?;

            if let Some(path) = output {
                std::fs::write(&path, &json)?;
                println!("Exported {} memories to {}", memories.len(), path);
            } else {
                println!("{}", json);
            }
        }

        Commands::Import { user, input } => {
            let json = std::fs::read_to_string(&input)?;
            let items: Vec<serde_json::Value> = serde_json::from_str(&json)?;

            let mut count = 0;
            for item in items {
                if let (Some(content), memory_type) = (
                    item.get("content").and_then(|v| v.as_str()),
                    item.get("memory_type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("general"),
                ) {
                    memory.add(&user, content, Some(memory_type)).await?;
                    count += 1;
                }
            }
            println!("Imported {} memories for user {}", count, user);
        }

        Commands::Stats { user } => {
            if let Some(user_id) = user {
                let memories = memory.get_all(&user_id).await?;
                println!("Statistics for user {}:", user_id);
                println!("  Total memories: {}", memories.len());

                // Count by type
                let mut type_counts: std::collections::HashMap<&str, usize> =
                    std::collections::HashMap::new();
                for mem in &memories {
                    *type_counts.entry(&mem.memory_type).or_insert(0) += 1;
                }
                println!("  By type:");
                for (t, count) in type_counts {
                    println!("    {}: {}", t, count);
                }
            } else {
                println!("Stats for all users not yet implemented.");
                println!("Use --user <USER_ID> to see stats for a specific user.");
            }
        }
    }

    Ok(())
}
