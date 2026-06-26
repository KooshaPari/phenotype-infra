use thiserror::Error;

pub type Result<T> = std::result::Result<T, KvdError>;

#[derive(Error, Debug)]
pub enum KvdError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Session error: {0}")]
    Session(String),

    #[error("Desktop automation error: {0}")]
    Desktop(String),

    #[error("Container error: {0}")]
    Container(String),

    #[error("VM error: {0}")]
    Vm(String),

    #[error("Recording error: {0}")]
    Recording(String),

    #[error("Credential error: {0}")]
    Credential(String),

    #[error("Script error: {0}")]
    Script(String),

    #[error("TUI error: {0}")]
    Tui(String),

    #[error("MCP error: {0}")]
    Mcp(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Element not found: {0}")]
    ElementNotFound(String),

    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
