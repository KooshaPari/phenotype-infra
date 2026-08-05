//! `oci-helpers` — shared utilities extracted from oci-lottery and
//! oci-post-acquire to avoid duplication across the `iac/` workspace.
//!
//! Per the DAG unit B14 mandate, this crate serves as the canonical home
//! for path helpers, OCI CLI wrappers, and other common patterns that
//! were previously copy-pasted between sibling crates.

use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Path helpers
// ---------------------------------------------------------------------------

/// Resolve `~` prefix to `$HOME`. If the path does not start with `~/`,
/// returns it unchanged.
pub fn expand_home(p: &str) -> PathBuf {
    if let Some(rest) = p.strip_prefix("~/")
        && let Some(home) = home_dir()
    {
        return home.join(rest);
    }
    PathBuf::from(p)
}

/// Return `$HOME` as a `PathBuf`, or `None` if unset.
pub fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

/// Convenience wrapper: returns `$HOME` or a fallback (default `/tmp`).
pub fn home_or_fallback() -> PathBuf {
    home_dir().unwrap_or_else(|| PathBuf::from("/tmp"))
}

// ---------------------------------------------------------------------------
// File I/O helpers
// ---------------------------------------------------------------------------

/// Idempotent JSON load: reads and deserializes a JSON file at `path`.
/// If the file does not exist, returns `Ok(default)`.
pub async fn load_json_or<T>(path: &PathBuf, default: T) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    if tokio::fs::try_exists(path).await.unwrap_or(false) {
        let raw = tokio::fs::read_to_string(path).await?;
        Ok(serde_json::from_str(&raw)?)
    } else {
        Ok(default)
    }
}

/// Idempotent JSON save: writes `value` as pretty-printed JSON to `path`,
/// creating parent directories as needed.
pub async fn save_json<T>(path: &PathBuf, value: &T) -> anyhow::Result<()>
where
    T: serde::Serialize,
{
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let raw = serde_json::to_string_pretty(value)?;
    tokio::fs::write(path, raw).await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Process helpers
// ---------------------------------------------------------------------------

/// Check whether a binary is on `$PATH` by invoking `command -v` via the
/// shell. Respects shell functions, aliases, and per-shell PATH munging.
pub async fn which_on_path(bin: &str) -> bool {
    tokio::process::Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {bin} >/dev/null 2>&1"))
        .status()
        .await
        .map(|s| s.success())
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Fixture {
        value: String,
    }

    fn temp_path(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("oci-helpers-{name}-{nonce}"))
    }

    #[test]
    fn test_home_or_fallback() {
        let h = home_or_fallback();
        assert!(h.is_absolute());
    }

    #[test]
    fn expand_home_preserves_non_tilde_paths() {
        assert_eq!(expand_home("relative/path"), PathBuf::from("relative/path"));
        assert_eq!(expand_home("/absolute/path"), PathBuf::from("/absolute/path"));
    }

    #[tokio::test]
    async fn save_and_load_json_roundtrip_and_missing_default() {
        let path = temp_path("nested/data.json");
        let fixture = Fixture {
            value: "covered".into(),
        };

        assert_eq!(load_json_or(&path, Fixture { value: "default".into() })
            .await
            .unwrap(), Fixture { value: "default".into() });
        save_json(&path, &fixture).await.unwrap();
        assert_eq!(load_json_or(&path, Fixture { value: "other".into() }).await.unwrap(), fixture);
        let _ = tokio::fs::remove_dir_all(path.parent().unwrap()).await;
    }

    #[tokio::test]
    async fn load_json_reports_invalid_content_and_which_checks_path() {
        let path = temp_path("invalid.json");
        tokio::fs::write(&path, b"not-json").await.unwrap();
        assert!(load_json_or::<Fixture>(&path, Fixture { value: "default".into() })
            .await
            .is_err());
        assert!(which_on_path("sh").await);
        assert!(!which_on_path("oci-helper-command-that-is-absent").await);
        let _ = tokio::fs::remove_file(path).await;
    }
}
