//! Mesh-state commit — flips OCI to ✅ in compute-mesh-state.md and commits.

use crate::InstanceFile;
use anyhow::{Context, Result, anyhow};
use chrono::Utc;
use oci_helpers::expand_home;
use tokio::process::Command;
use tracing::info;

pub async fn commit_state(repo: &str, inst: &InstanceFile) -> Result<()> {
    let repo_path = expand_home(repo);
    let doc = repo_path.join("docs/governance/compute-mesh-state.md");
    let original = tokio::fs::read_to_string(&doc)
        .await
        .with_context(|| format!("read {}", doc.display()))?;

    let timestamp = Utc::now().format("%Y-%m-%d %H:%M UTC");
    let marker = format!(
        "\n\n<!-- oci-post-acquire: AUTO-INSERTED {timestamp} -->\n\
         ## OCI Status: ✅ ACQUIRED\n\n\
         - Region: `{}`\n- AD: `{}`\n- Public IP: `{}`\n- Instance OCID: `{}`\n- Acquired: `{}`\n",
        inst.region, inst.ad, inst.public_ip, inst.instance_ocid, inst.acquired_at,
    );

    // Idempotent: replace any prior auto-insert block.
    let updated = if let Some(idx) = original.find("<!-- oci-post-acquire: AUTO-INSERTED") {
        let mut s = original[..idx].trim_end().to_string();
        s.push_str(&marker);
        s
    } else {
        format!("{}{marker}", original.trim_end())
    };
    tokio::fs::write(&doc, updated).await?;

    let run = |args: &[&str]| {
        let cwd = repo_path.clone();
        let owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        async move {
            let status = Command::new("git")
                .args(&owned)
                .current_dir(&cwd)
                .status()
                .await?;
            if !status.success() {
                return Err(anyhow!("git {:?} exited {status}", owned));
            }
            anyhow::Ok(())
        }
    };
    run(&["add", "docs/governance/compute-mesh-state.md"]).await?;
    run(&[
        "commit",
        "-m",
        &format!("chore(mesh): OCI acquired {timestamp} ({})", inst.region),
    ])
    .await?;
    info!("mesh-state commit landed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_repo() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock before unix epoch")
            .as_nanos();
        let repo = std::env::temp_dir().join(format!("oci-post-mesh-{nonce}"));
        std::fs::create_dir_all(repo.join("docs/governance")).expect("create mesh fixture");
        let init = Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(&repo)
            .status()
            .expect("spawn git init");
        assert!(init.success(), "git init failed: {init}");
        for (key, value) in [
            ("user.name", "coverage-test"),
            ("user.email", "coverage@example.test"),
        ] {
            let configured = Command::new("git")
                .args(["config", key, value])
                .current_dir(&repo)
                .status()
                .expect("spawn git config");
            assert!(configured.success(), "git config failed: {configured}");
        }
        repo
    }

    fn fixture(region: &str) -> InstanceFile {
        InstanceFile {
            instance_ocid: "ocid1.test".into(),
            region: region.into(),
            ad: "AD-1".into(),
            public_ip: "198.51.100.40".into(),
            acquired_at: "2026-08-02T00:00:00Z".into(),
        }
    }

    #[tokio::test]
    async fn writes_and_replaces_auto_inserted_mesh_state() {
        let repo = temp_repo();
        let doc = repo.join("docs/governance/compute-mesh-state.md");
        tokio::fs::write(&doc, "# Compute Mesh State\n")
            .await
            .unwrap();

        commit_state(repo.to_str().unwrap(), &fixture("us-test-1"))
            .await
            .unwrap();
        commit_state(repo.to_str().unwrap(), &fixture("us-test-2"))
            .await
            .unwrap();

        let content = tokio::fs::read_to_string(&doc).await.unwrap();
        assert_eq!(
            content
                .matches("<!-- oci-post-acquire: AUTO-INSERTED")
                .count(),
            1
        );
        assert!(content.contains("Region: `us-test-2`"));
        assert!(!content.contains("Region: `us-test-1`"));
        let _ = tokio::fs::remove_dir_all(repo).await;
    }
}
