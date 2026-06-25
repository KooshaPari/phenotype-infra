//! KVirtualDesktop Core MCP Protocol Implementation
//!
//! This crate provides the core MCP (Model Context Protocol) types and schema
//! definitions based on the 2025-06-18 specification. It is the canonical
//! protocol layer for the KVirtualDesktop ecosystem and is consumed by
//! `kvd`, `kvirtualdesktop-mcp-server`, and external MCP clients.
//!
//! # Crate layout
//!
//! - [`protocol`] — MCP method names and protocol-level constants.
//! - [`types`] — Wire-format structs (`Message`, `Tool`, `Resource`,
//!   `PromptTemplate`, `ServerCapabilities`, …).
//! - [`transport`] — The [`Transport`](transport::Transport) trait plus
//!   [`StdioTransport`](transport::StdioTransport) and
//!   [`HttpSseTransport`](transport::HttpSseTransport) implementations.
//! - [`error`] — The error hierarchy rooted at
//!   [`McpError`](error::McpError), including transport, security, desktop
//!   automation, VM, CLI, TTS, and credential sub-enums.
//! - [`security`] — OAuth2 / JWT helpers and security-policy types.
//!
//! # Quickstart
//!
//! Build a [`Message`](types::Message), send it over stdio, and read the
//! response:
//!
//! ```
//! use kvirtualdesktop_core::{Message, transport::{Transport, StdioTransport}};
//! use chrono::Utc;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let mut t = StdioTransport::new();
//! t.start().await?;
//!
//! let msg = Message {
//!     id: "1".to_string(),
//!     method: "ping".to_string(),
//!     params: None,
//!     result: None,
//!     error: None,
//!     session_id: None,
//!     timestamp: Utc::now(),
//! };
//!
//! // In a real client, send the request and await the response.
//! let _ = t.send_message(msg).await;
//! t.stop().await?;
//! # Ok(()) }
//! ```
//!
//! # Protocol version
//!
//! The MCP version pinned by this crate is exposed as
//! [`MCP_VERSION`]. The crate's own version is
//! [`KVIRTUALDESKTOP_VERSION`].

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

pub mod protocol;
pub mod transport;
pub mod types;
pub mod error;
pub mod security;

pub use protocol::*;
pub use transport::*;
pub use types::*;
pub use error::*;
pub use security::*;

/// MCP Protocol Version
///
/// The wire-protocol version this crate targets. Bump in lock-step with the
/// upstream MCP specification; downstream consumers can read this constant
/// at runtime to negotiate capabilities.
pub const MCP_VERSION: &str = "2025-06-18";

/// KVirtualDesktop MCP Implementation Version
///
/// The semver of this crate. Independent from [`MCP_VERSION`]: a crate
/// release may advance the implementation without changing the wire
/// protocol.
pub const KVIRTUALDESKTOP_VERSION: &str = "0.1.0";