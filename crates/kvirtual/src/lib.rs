//! KVirtual — Desktop Automation CLI (`kvd`)
//!
//! Absorbed from `_tmp_kvirtualstage/kvirtualdesktop` on 2026-06-24.
//! Provides a playwright-equivalent for desktop automation with
//! VM/container support, credential management, screen recording,
//! and an MCP protocol server.

pub mod cli;
pub mod config;
pub mod session;
pub mod tui;
pub mod desktop;
pub mod container;
pub mod vm;
pub mod record;
pub mod credentials;
pub mod scripting;
pub mod history;
pub mod completion;
pub mod error;

// Re-exports
pub use cli::{CliRunner, CommandContext};
pub use config::Config;
pub use session::SessionManager;
pub use tui::TuiRunner;
pub use error::{KvdError, Result};

// ─── CLI Subcommand Action Types ──────────────────────────────────────────────
// Shared between main.rs (binary) and cli.rs (library).

#[derive(Debug, Clone)]
pub enum DesktopAction {
    Click { x: i32, y: i32 },
    Type { text: String },
    Screenshot { path: Option<String> },
    Find { text: String },
    Wait { selector: String, timeout: Option<u64> },
    Drag { from_x: i32, from_y: i32, to_x: i32, to_y: i32 },
    Keys { keys: String },
    StartRecording { output: String },
    StopRecording,
}

#[derive(Debug, Clone)]
pub enum ContainerAction {
    List,
    Create { name: String, image: String },
    Start { name: String },
    Stop { name: String },
    Exec { name: String, command: String },
    Connect { name: String },
}

#[derive(Debug, Clone)]
pub enum VmAction {
    List,
    Create { name: String, template: String },
    Start { name: String },
    Stop { name: String },
    Connect { name: String },
    Snapshot { name: String, snapshot_name: String },
}

#[derive(Debug, Clone)]
pub enum RecordAction {
    Start { output: String },
    Stop,
    Screenshot { path: Option<String> },
    Audio { output: String },
    Tts { text: String },
}

#[derive(Debug, Clone)]
pub enum SessionAction {
    List,
    Create { name: String },
    Switch { name: String },
    Delete { name: String },
    Save { name: Option<String> },
    Load { name: String },
}

#[derive(Debug, Clone)]
pub enum ConfigAction {
    Show,
    Set { key: String, value: String },
    Get { key: String },
    Init,
    Edit,
}

#[derive(Debug, Clone)]
pub enum CredAction {
    Store { name: String, value: String },
    Get { name: String },
    List,
    Delete { name: String },
}

// ─── Core Automation Traits ───────────────────────────────────────────────────

pub mod automation {
    use async_trait::async_trait;
    use crate::error::Result;

    /// Core automation interface — click, type, screenshot, find elements.
    #[async_trait]
    pub trait Automator: Send {
        async fn click(&mut self, x: i32, y: i32) -> Result<()>;
        async fn type_text(&mut self, text: &str) -> Result<()>;
        async fn key_press(&mut self, key: &str) -> Result<()>;
        async fn screenshot(&mut self, path: Option<&str>) -> Result<String>;
        async fn find_element(&mut self, selector: &str) -> Result<ElementHandle>;
        async fn wait_for_element(&mut self, selector: &str, timeout: Option<u64>) -> Result<ElementHandle>;
    }

    #[derive(Debug, Clone)]
    pub struct ElementHandle {
        pub id: String,
        pub bounds: Rect,
        pub text: Option<String>,
        pub class_name: Option<String>,
    }

    #[derive(Debug, Clone)]
    pub struct Rect {
        pub x: i32,
        pub y: i32,
        pub width: i32,
        pub height: i32,
    }
}

// ─── MCP Interface Support ────────────────────────────────────────────────────

pub mod mcp {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct McpTool {
        pub name: String,
        pub description: String,
        pub input_schema: serde_json::Value,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct McpRequest {
        pub tool: String,
        pub arguments: serde_json::Value,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct McpResponse {
        pub success: bool,
        pub result: serde_json::Value,
        pub error: Option<String>,
    }
}

// ─── Utility Functions ────────────────────────────────────────────────────────

pub mod utils {
    use crate::error::Result;
    use std::path::Path;

    pub fn ensure_dir_exists(path: &Path) -> Result<()> {
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        Ok(())
    }

    pub fn get_timestamp() -> String {
        chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string()
    }

    pub fn sanitize_filename(name: &str) -> String {
        name.chars()
            .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
            .collect()
    }
}
