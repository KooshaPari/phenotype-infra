//! Downstream hook drop-ins — runs every executable in `oci-acquire-hooks.d/`
//! lexicographically, passing the instance file path as $1 and exporting key
//! fields as env vars. A failing hook does not abort the chain; we collect
//! errors and warn at the end.

use crate::InstanceFile;
use anyhow::{Result, anyhow};
use oci_helpers::expand_home;
use tokio::process::Command;
use tracing::{info, warn};

pub async fn run_dropins(dir: &str, inst: &InstanceFile) -> Result<()> {
    let p = expand_home(dir);
    if !p.exists() {
        info!(dir = %p.display(), "no hooks.d dir; skipping");
        return Ok(());
    }
    let mut entries: Vec<_> = std::fs::read_dir(&p)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();
    entries.sort();

    let mut errors = Vec::new();
    for hook in entries {
        let name = hook
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("?")
            .to_string();
        info!(hook = %name, "running drop-in hook");
        let status = Command::new(&hook)
            .env("OCI_INSTANCE_OCID", &inst.instance_ocid)
            .env("OCI_REGION", &inst.region)
            .env("OCI_AD", &inst.ad)
            .env("OCI_PUBLIC_IP", &inst.public_ip)
            .env("OCI_ACQUIRED_AT", &inst.acquired_at)
            .status()
            .await;
        match status {
            Ok(s) if s.success() => info!(hook = %name, "ok"),
            Ok(s) => {
                warn!(hook = %name, ?s, "hook failed");
                errors.push(format!("{name}: {s}"));
            }
            Err(e) => {
                warn!(hook = %name, error = ?e, "spawn failed");
                errors.push(format!("{name}: {e}"));
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(anyhow!("{} hook(s) failed: {:?}", errors.len(), errors))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("oci-dropins-{name}-{nonce}"))
    }

    fn instance() -> InstanceFile {
        InstanceFile {
            instance_ocid: "ocid1.test".into(),
            region: "us-test-1".into(),
            ad: "AD-1".into(),
            public_ip: "198.51.100.20".into(),
            acquired_at: "2026-08-02T00:00:00Z".into(),
        }
    }

    #[tokio::test]
    async fn missing_and_empty_hook_directories_are_noops() {
        let missing = temp_dir("missing");
        run_dropins(missing.to_str().unwrap(), &instance())
            .await
            .unwrap();
        let empty = temp_dir("empty");
        tokio::fs::create_dir_all(&empty).await.unwrap();
        run_dropins(empty.to_str().unwrap(), &instance())
            .await
            .unwrap();
        let _ = tokio::fs::remove_dir_all(empty).await;
    }

    #[tokio::test]
    async fn dropins_continue_after_spawn_failure_and_report_error() {
        let dir = temp_dir("failure");
        tokio::fs::create_dir_all(&dir).await.unwrap();
        // A regular text file is deliberately not executable on Unix and is
        // not a runnable image on Windows, exercising the fail-soft branch.
        tokio::fs::write(dir.join("01-not-runnable"), b"not a program")
            .await
            .unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let hook = dir.join("02-ok.sh");
            tokio::fs::write(&hook, b"#!/bin/sh\nexit 0\n")
                .await
                .unwrap();
            std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
        #[cfg(windows)]
        tokio::fs::write(dir.join("02-ok.cmd"), b"@exit /b 0\r\n")
            .await
            .unwrap();

        let err = run_dropins(dir.to_str().unwrap(), &instance())
            .await
            .unwrap_err();
        assert!(err.to_string().contains("hook(s) failed"));
        let _ = tokio::fs::remove_dir_all(dir).await;
    }
}
