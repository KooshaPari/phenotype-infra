//! Credential management helpers
//!
//! These stubs reference the `credential-manager` workspace crate for full functionality.
//! Enable via `cargo check --features credential-manager` once wired.

pub mod credential_manager {
    /// Stub — integrate with `credential-manager` crate.
    /// The `kvirtual` binary delegates credential operations to
    /// `crates/credential-manager` directly (no re-export needed).
    pub fn placeholder() -> &'static str {
        "credential-manager is its own crate at crates/credential-manager"
    }
}
