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

#[cfg(all(test, unix))]
mod tests {
    use super::run_dropins;
    use crate::InstanceFile;

    #[tokio::test]
    async fn runs_successful_hooks_and_reports_failures() {
        use std::os::unix::fs::PermissionsExt;

        let dir =
            std::env::temp_dir().join(format!("oci-post-acquire-hooks-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        for (name, body) in [
            ("10-ok.sh", "#!/bin/sh\nexit 0\n"),
            ("20-bad.sh", "#!/bin/sh\nexit 7\n"),
        ] {
            let path = dir.join(name);
            std::fs::write(&path, body).unwrap();
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
        let inst = InstanceFile {
            instance_ocid: "ocid".into(),
            region: "region".into(),
            ad: "ad".into(),
            public_ip: "127.0.0.1".into(),
            acquired_at: "now".into(),
        };

        let err = run_dropins(dir.to_str().unwrap(), &inst)
            .await
            .expect_err("a failing hook should be reported");
        assert!(err.to_string().contains("1 hook(s) failed"));
        std::fs::remove_file(dir.join("20-bad.sh")).unwrap();
        run_dropins(dir.to_str().unwrap(), &inst)
            .await
            .expect("successful hooks should complete");
        std::fs::remove_dir_all(dir).unwrap();
    }
}
