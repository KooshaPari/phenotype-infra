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
    #[cfg(unix)]
    use std::path::PathBuf;
    #[cfg(unix)]
    use std::time::{SystemTime, UNIX_EPOCH};

    fn fixture() -> InstanceFile {
        InstanceFile {
            instance_ocid: "ocid1.test".into(),
            region: "us-test-1".into(),
            ad: "AD-1".into(),
            public_ip: "198.51.100.40".into(),
            acquired_at: "2026-08-02T00:00:00Z".into(),
        }
    }

    #[cfg(unix)]
    fn temp_dir(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock before unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("oci-post-hooks-{name}-{nonce}"));
        std::fs::create_dir_all(&path).expect("create hook fixture directory");
        path
    }

    #[tokio::test]
    async fn missing_hooks_directory_is_a_noop() {
        let path = std::env::temp_dir().join("oci-post-hooks-missing");
        let _ = tokio::fs::remove_dir_all(&path).await;
        assert!(
            run_dropins(path.to_str().unwrap(), &fixture())
                .await
                .is_ok()
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn runs_sorted_hooks_and_reports_failures_without_aborting() {
        use std::os::unix::fs::PermissionsExt;

        let dir = temp_dir("ordered");
        let marker = dir.join("marker");
        let marker_literal = marker.to_string_lossy().replace('\'', "'\\''");
        let ok = dir.join("01-ok");
        std::fs::write(
            &ok,
            format!(
                "#!/bin/sh\nprintf '%s|%s|%s|%s' \"$OCI_INSTANCE_OCID\" \"$OCI_REGION\" \"$OCI_AD\" \"$OCI_PUBLIC_IP\" > '{}'\n",
                marker_literal
            ),
        )
        .expect("write successful hook");
        std::fs::set_permissions(&ok, std::fs::Permissions::from_mode(0o755))
            .expect("make successful hook executable");

        let fail = dir.join("02-fail");
        std::fs::write(&fail, b"#!/bin/sh\nexit 7\n").expect("write failing hook");
        std::fs::set_permissions(&fail, std::fs::Permissions::from_mode(0o755))
            .expect("make failing hook executable");

        let result = run_dropins(dir.to_str().unwrap(), &fixture()).await;
        let error = result
            .expect_err("failing hook should be reported")
            .to_string();
        assert!(error.contains("02-fail"), "unexpected error: {error}");
        assert_eq!(
            tokio::fs::read_to_string(&marker).await.unwrap(),
            "ocid1.test|us-test-1|AD-1|198.51.100.40"
        );
        let _ = tokio::fs::remove_dir_all(dir).await;
    }
}
