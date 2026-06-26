//! credential-manager — Secret vault credential manager for NVMS/KVirtualStage
//!
//! Zero-trust token rotation, OAuth2 refresh, credential caching with
//! hardware-backed keystore support. Designed for headless environments
//! (NVMS VMs, KVirtualStage Desktop) where no TTY is available for
//! interactive auth flows.
//!
//! # Phase 7 — Stub / Next Work
//! - [ ] Implement `CredentialVault` with file-backed encrypted storage
//! - [ ] Add OAuth2 refresh-token rotation (wraps kvirtualdesktop-core)
//! - [ ] Add hardware keystore backend (TPM, macOS Keychain, Linux Secret Service)
//! - [ ] Integrate with nvms-ffi for VM credential injection
//! - [ ] Rotation policy: auto-rotate before expiry, graceful fallback

use thiserror::Error;
use std::collections::HashMap;

/// A stored credential with metadata.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Credential {
    /// The credential identifier (e.g. "github-oauth", "vault-token")
    pub id: String,
    /// Provider name (e.g. "github", "aws", "azure")
    pub provider: String,
    /// Encrypted credential blob
    pub blob: Vec<u8>,
    /// Unix timestamp of last rotation
    pub rotated_at: u64,
    /// Unix timestamp when this credential expires
    pub expires_at: u64,
    /// Whether this credential supports auto-rotation
    pub rotatable: bool,
}

#[derive(Debug, Error)]
pub enum CredentialVaultError {
    #[error("credential not found: {0}")]
    NotFound(String),
    #[error("storage error: {0}")]
    Storage(String),
    #[error("crypto error: {0}")]
    Crypto(String),
    #[error("credential expired: {0}")]
    Expired(String),
}

/// A vault managing credential storage and rotation.
pub struct CredentialVault {
    credentials: HashMap<String, Credential>,
    storage_path: Option<String>,
}

impl CredentialVault {
    /// Create a new in-memory credential vault.
    pub fn new() -> Self {
        Self {
            credentials: HashMap::new(),
            storage_path: None,
        }
    }

    /// Store a credential.
    pub fn store(&mut self, cred: Credential) {
        self.credentials.insert(cred.id.clone(), cred);
    }

    /// Retrieve a credential by id.
    pub fn get(&self, id: &str) -> Result<&Credential, CredentialVaultError> {
        self.credentials.get(id).ok_or_else(|| CredentialVaultError::NotFound(id.to_string()))
    }

    /// Rotate (refresh) a credential — placeholder for token exchange.
    pub fn rotate(&mut self, id: &str) -> Result<(), CredentialVaultError> {
        let cred = self.credentials.get_mut(id).ok_or_else(|| CredentialVaultError::NotFound(id.to_string()))?;
        if !cred.rotatable {
            return Err(CredentialVaultError::Crypto("credential does not support rotation".into()));
        }
        // Placeholder: in production, exchange refresh token for new access token
        cred.rotated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Ok(())
    }

    /// Number of stored credentials.
    pub fn len(&self) -> usize {
        self.credentials.len()
    }
}

impl Default for CredentialVault {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vault() {
        let vault = CredentialVault::new();
        assert_eq!(vault.len(), 0);
    }

    #[test]
    fn test_store_and_retrieve() {
        let mut vault = CredentialVault::new();
        let cred = Credential {
            id: "test".into(),
            provider: "github".into(),
            blob: vec![1, 2, 3],
            rotated_at: 1000,
            expires_at: 2000,
            rotatable: true,
        };
        vault.store(cred);
        assert_eq!(vault.len(), 1);
        let retrieved = vault.get("test").unwrap();
        assert_eq!(retrieved.provider, "github");
    }

    #[test]
    fn test_not_found() {
        let vault = CredentialVault::new();
        let result = vault.get("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_rotate() {
        let mut vault = CredentialVault::new();
        vault.store(Credential {
            id: "rotatable".into(),
            provider: "test".into(),
            blob: vec![],
            rotated_at: 1000,
            expires_at: 2000,
            rotatable: true,
        });
        assert!(vault.rotate("rotatable").is_ok());
        assert_ne!(vault.get("rotatable").unwrap().rotated_at, 1000);
    }

    #[test]
    fn test_non_rotatable() {
        let mut vault = CredentialVault::new();
        vault.store(Credential {
            id: "static".into(),
            provider: "test".into(),
            blob: vec![],
            rotated_at: 1000,
            expires_at: 0,
            rotatable: false,
        });
        assert!(vault.rotate("static").is_err());
    }
}
