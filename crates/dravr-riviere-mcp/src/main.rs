// ABOUTME: CLI entry point for the dravr-riviere MCP server
// ABOUTME: Supports stdio and HTTP transport modes via dravr-tronc shared infrastructure
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

use std::error::Error;
use std::process;
use std::sync::Arc;

use clap::Parser;
use dravr_tronc::mcp::transport::{http, stdio};
use dravr_tronc::server::cli::McpArgs;
use dravr_tronc::server::tracing_init;
use tracing::info;

use dravr_riviere_mcp::state::ServerState;
use dravr_riviere_mcp::{build_tool_registry, McpServer};

#[derive(Parser)]
#[command(
    name = "dravr-riviere-mcp",
    version,
    about = "Time-series storage MCP server"
)]
struct Cli {
    #[command(flatten)]
    server: McpArgs,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let cli = Cli::parse();

    tracing_init::init(&cli.server.transport);

    let state = Arc::new(ServerState::new());
    let tools = build_tool_registry();
    let server = Arc::new(McpServer::new(
        "dravr-riviere-mcp",
        env!("CARGO_PKG_VERSION"),
        tools,
        state,
    ));

    info!(
        "Starting dravr-riviere MCP server (transport: {})",
        cli.server.transport
    );

    match cli.server.transport.as_str() {
        "stdio" => stdio::run(server).await?,
        "http" => {
            http::serve(server, &cli.server.host, cli.server.port).await?;
        }
        other => {
            eprintln!("Unknown transport: {other}. Use 'stdio' or 'http'.");
            process::exit(1);
        }
    }

    Ok(())
}
