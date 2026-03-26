// ABOUTME: Unified CLI entry point for dravr-riviere server
// ABOUTME: Supports serve (REST+MCP) and MCP stdio transport modes
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use std::sync::Arc;

use clap::{Parser, Subcommand};
use tokio::sync::RwLock;
use tracing::info;

use dravr_riviere_mcp::state::ServerState;
use dravr_riviere_mcp::{build_tool_registry, McpServer};
use dravr_riviere_server::router::build_router;

/// Default host to bind to when none is specified.
const DEFAULT_HOST: &str = "127.0.0.1";
/// Default port to listen on when none is specified.
const DEFAULT_PORT: u16 = 3200;

#[derive(Parser)]
#[command(
    name = "dravr-riviere-server",
    version,
    about = "Time-series storage server"
)]
struct Cli {
    /// Transport mode for MCP (when no subcommand)
    #[arg(long, default_value = "http")]
    transport: String,

    /// HTTP host
    #[arg(long)]
    host: Option<String>,

    /// HTTP port
    #[arg(long)]
    port: Option<u16>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Start the REST + MCP HTTP server
    Serve {
        /// HTTP host
        #[arg(long)]
        host: Option<String>,
        /// HTTP port
        #[arg(long)]
        port: Option<u16>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    dravr_tronc::server::tracing_init::init(&cli.transport);

    let state = Arc::new(RwLock::new(ServerState::new()));
    let tools = build_tool_registry();
    let mcp_server = Arc::new(McpServer::new(
        "dravr-riviere",
        env!("CARGO_PKG_VERSION"),
        tools,
        state,
    ));

    match cli.command {
        Some(Command::Serve { host, port }) => {
            let host = host.unwrap_or_else(|| DEFAULT_HOST.to_owned());
            let port = port.unwrap_or(DEFAULT_PORT);
            serve_http(mcp_server, &host, port).await?;
        }
        None => {
            let host = cli.host.unwrap_or_else(|| DEFAULT_HOST.to_owned());
            let port = cli.port.unwrap_or(DEFAULT_PORT);
            if cli.transport == "stdio" {
                dravr_tronc::mcp::transport::stdio::run(mcp_server).await?;
            } else {
                serve_http(mcp_server, &host, port).await?;
            }
        }
    }

    Ok(())
}

async fn serve_http(
    mcp_server: Arc<McpServer<ServerState>>,
    host: &str,
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = build_router(mcp_server);
    let addr = format!("{host}:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("dravr-riviere server listening on {addr}");
    info!("  Health: GET http://{addr}/health");
    info!("  MCP:    POST http://{addr}/mcp");

    axum::serve(listener, app).await?;
    Ok(())
}
