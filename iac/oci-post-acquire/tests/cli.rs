use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("oci-post-cli-{name}-{nonce}"));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn fake_ansible(dir: &Path) -> PathBuf {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let path = dir.join("ansible-playbook");
        std::fs::write(&path, b"#!/bin/sh\nexit 0\n").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        path
    }
    #[cfg(windows)]
    {
        let path = dir.join("ansible-playbook.cmd");
        std::fs::write(&path, b"@exit /b 0\r\n").unwrap();
        path
    }
}

// The fake ansible command and loopback harness are POSIX-only.  The hosted
// coverage gate runs on Ubuntu; avoid accidentally invoking a host ansible
// binary during Windows test runs.
#[cfg(unix)]
#[tokio::test]
async fn dry_run_reads_instance_waits_for_ssh_and_runs_ansible() {
    let temp = TempDir::new("dry-run");
    let fake_dir = temp.path().join("bin");
    std::fs::create_dir_all(&fake_dir).unwrap();
    let _ansible = fake_ansible(&fake_dir);
    let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
    let ssh_port = listener.local_addr().unwrap().port().to_string();
    let accept = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let _ = stream.shutdown().await;
    });
    let instance = temp.path().join("instance.json");
    std::fs::write(
        &instance,
        serde_json::to_vec(&json!({
            "instance_ocid": "ocid1.test",
            "region": "us-test-1",
            "ad": "AD-1",
            "public_ip": "127.0.0.1",
            "acquired_at": "2026-08-02T00:00:00Z"
        }))
        .unwrap(),
    )
    .unwrap();
    let repo = temp.path().join("repo");
    std::fs::create_dir_all(&repo).unwrap();
    let mut path = fake_dir.to_string_lossy().into_owned();
    #[cfg(windows)]
    {
        path.push(';');
        path.push_str(r"C:\Windows\System32");
    }
    #[cfg(unix)]
    {
        path.push(':');
        path.push_str("/usr/bin:/bin");
    }
    let output = Command::new(env!("CARGO_BIN_EXE_oci-post-acquire"))
        .args([
            "--instance-file",
            instance.to_str().unwrap(),
            "--repo",
            repo.to_str().unwrap(),
            "--playbook",
            "coverage-playbook.yml",
            "--ssh-port",
            &ssh_port,
            "--hooks-dir",
            temp.path().join("hooks").to_str().unwrap(),
            "--format",
            "text",
            "--dry-run",
        ])
        .env("PATH", path)
        .env_remove("TS_API_KEY")
        .env_remove("TS_TAILNET")
        .output()
        .unwrap();
    accept.await.unwrap();
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(unix)]
#[test]
fn invalid_instance_file_fails_at_step_one() {
    let temp = TempDir::new("invalid");
    let instance = temp.path().join("invalid.json");
    std::fs::write(&instance, b"not-json").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_oci-post-acquire"))
        .args([
            "--instance-file",
            instance.to_str().unwrap(),
            "--format",
            "text",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
}
