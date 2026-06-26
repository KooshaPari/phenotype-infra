//! KVirtualDesktop MCP Server Library
//! 
//! This library provides the core MCP server implementation for KVirtualDesktop.

pub mod server;
pub mod config;
pub mod handlers;
pub mod session;
pub mod middleware;

pub use server::*;
pub use config::*;
pub use handlers::*;
pub use session::*;
pub use middleware::*;

use kvirtualdesktop_core::*;