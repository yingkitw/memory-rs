//! MCP Server binary for memory-rs
//!
//! This binary runs the memory MCP server using STDIO transport.

use memory_rs::mcp::MemoryMcpServer;
use rmcp::{transport::stdio, ServiceExt};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing to stderr (stdout is used for MCP communication)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "memory_mcp=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();

    tracing::info!("Starting Memory MCP Server");

    // Create and run the server with STDIO transport
    let server = MemoryMcpServer::new();
    let service = server.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("Error starting server: {}", e);
    })?;

    tracing::info!("Memory MCP Server running");

    // Wait for the service to complete
    service.waiting().await?;

    tracing::info!("Memory MCP Server stopped");
    Ok(())
}
