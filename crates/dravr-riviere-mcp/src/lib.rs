// ABOUTME: MCP server library exposing time-series storage via Model Context Protocol
// ABOUTME: JSON-RPC 2.0 protocol with stdio and HTTP transports via dravr-tronc shared infrastructure
//
// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 dravr.ai

//! # dravr-riviere-mcp
//!
//! MCP (Model Context Protocol) server for the dravr-riviere time-series engine.
//! Exposes storage operations as MCP tools via JSON-RPC 2.0.

/// Shared server state
pub mod state;
/// MCP tool definitions and registry
pub mod tools;

pub use dravr_tronc::McpServer;
pub use state::{ServerState, SharedState};
pub use tools::build_tool_registry;
