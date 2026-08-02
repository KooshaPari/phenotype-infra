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
    use std::process::Command as StdCommand;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("oci-mesh-{name}-{nonce}"))
    }

    fn instance() -> InstanceFile {
        InstanceFile {
            instance_ocid: "ocid1.test".into(),
            region: "us-test-1".into(),
            ad: "AD-1".into(),
            public_ip: "198.51.100.21".into(),
            acquired_at: "2026-08-02T00:00:00Z".into(),
        }
    }

    async fn git_repo() -> std::path::PathBuf {
        let repo = temp_dir("repo");
        tokio::fs::create_dir_all(repo.join("docs/governance"))
            .await
            .unwrap();
        tokio::fs::write(
            repo.join("docs/governance/compute-mesh-state.md"),
            "# Mesh\n",
        )
        .await
        .unwrap();
        assert!(
            StdCommand::new("git")
                .args(["init", "-q"])
                .arg(&repo)
                .status()
                .unwrap()
                .success()
        );
        for (key, value) in [
            ("user.name", "coverage-test"),
            ("user.email", "coverage@test.invalid"),
        ] {
            assert!(
                StdCommand::new("git")
                    .args(["-C", repo.to_str().unwrap(), "config", key, value])
                    .status()
                    .unwrap()
                    .success()
            );
        }
        assert!(
            StdCommand::new("git")
                .args(["-C", repo.to_str().unwrap(), "add", "."])
                .status()
                .unwrap()
                .success()
        );
        assert!(
            StdCommand::new("git")
                .args(["-C", repo.to_str().unwrap(), "commit", "-qm", "initial"])
                .status()
                .unwrap()
                .success()
        );
        repo
    }

    #[tokio::test]
    async fn commit_state_inserts_and_replaces_its_marker_idempotently() {
        let repo = git_repo().await;
        commit_state(repo.to_str().unwrap(), &instance())
            .await
            .unwrap();
        let doc = repo.join("docs/governance/compute-mesh-state.md");
        let first = tokio::fs::read_to_string(&doc).await.unwrap();
        assert_eq!(first.matches("AUTO-INSERTED").count(), 1);
        assert!(first.contains("ocid1.test"));

        let mut changed = instance();
        changed.instance_ocid = "ocid1.changed".into();
        commit_state(repo.to_str().unwrap(), &changed)
            .await
            .unwrap();
        let second = tokio::fs::read_to_string(&doc).await.unwrap();
        assert_eq!(second.matches("AUTO-INSERTED").count(), 1);
        assert!(!second.contains("ocid1.test"));
        assert!(second.contains("ocid1.changed"));
        let _ = tokio::fs::remove_dir_all(repo).await;
    }
}
