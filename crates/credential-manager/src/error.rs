use thiserror::Error;

/// Errors returned by the credential manager
#[derive(Error, Debug)]
pub enum CredentialError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Vault error: {0}")]
    Vault(String),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("OAuth error: {0}")]
    OAuth(String),

    #[error("Session error: {0}")]
    Session(String),

    #[error("MFA error: {0}")]
    Mfa(String),

    #[error("Injection error: {0}")]
    Injection(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Base64 decode error: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Rate limited: retry after {0:?}")]
    RateLimited(std::time::Duration),

    #[error("Unsupported: {0}")]
    Unsupported(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Convenience result alias
pub type Result<T> = std::result::Result<T, CredentialError>;
