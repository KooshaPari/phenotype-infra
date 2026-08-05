#![cfg(unix)]

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("oci-lottery-cli-{name}-{nonce}"));
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

fn fake_oci(dir: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let path = dir.join("oci");
        std::fs::write(
            &path,
            r##"#!/bin/sh
case "$*" in
  *"availability-domain list"*) printf '%s\n' '{"data":[{"name":"AD-1"},{"name":"AD-2"}]}' ;;
  *"instance launch"*)
    if [ "${OCI_TEST_MODE:-success}" = "capacity" ]; then
      printf '%s\n' 'OutOfHostCapacity' >&2; exit 1
    fi
    printf '%s\n' '{"data":{"id":"ocid1.instance.test"}}' ;;
  *"list-vnics"*) printf '%s\n' '{"data":[{"public-ip":"198.51.100.40"}]}' ;;
  *) exit 2 ;;
esac
"##,
        )
        .unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }

    #[cfg(windows)]
    std::fs::write(
        dir.join("oci.cmd"),
        r##"@echo off
if "%1"=="iam" (
  echo {"data":[{"name":"AD-1"},{"name":"AD-2"}]}
  exit /b 0
)
if "%3"=="launch" (
  if "%OCI_TEST_MODE%"=="capacity" (
    echo OutOfHostCapacity 1>&2
    exit /b 1
  )
  echo {"data":{"id":"ocid1.instance.test"}}
  exit /b 0
)
if "%3"=="list-vnics" (
  echo {"data":[{"public-ip":"198.51.100.40"}]}
  exit /b 0
)
exit /b 2
"##,
    )
    .unwrap();
}

fn run_lottery(temp: &TempDir, mode: Option<&str>) -> std::process::Output {
    fake_oci(temp.path());
    let config_path = temp.path().join("config.json");
    let state_path = temp.path().join("state.json");
    let acquired_path = temp.path().join("acquired.json");
    let key_path = temp.path().join("id_ed25519.pub");
    std::fs::write(&key_path, "ssh-ed25519 AAAATEST").unwrap();
    let config = serde_json::json!({
        "regions": ["us-test-1"],
        "availability_domains": [1],
        "shape": "VM.Standard.A1.Flex",
        "ocpus": 1,
        "memory_gb": 2,
        "image_ocid": "ocid1.image.test",
        "subnet_ocid": "ocid1.subnet.test",
        "display_name": "coverage-node",
        "ssh_authorized_keys_path": key_path,
        "profile": "DEFAULT",
        "compartment_ocid": "ocid1.compartment.test",
        "backoff_min_secs": 0,
        "backoff_max_secs": 0
    });
    std::fs::write(&config_path, serde_json::to_vec(&config).unwrap()).unwrap();

    let mut path = temp.path().to_string_lossy().into_owned();
    if let Some(existing) = std::env::var_os("PATH") {
        path.push(if cfg!(windows) { ';' } else { ':' });
        path.push_str(&existing.to_string_lossy());
    }
    let mut command = Command::new(env!("CARGO_BIN_EXE_oci-lottery"));
    command
        .args([
            "--config",
            config_path.to_str().unwrap(),
            "--state-file",
            state_path.to_str().unwrap(),
            "--acquired-file",
            acquired_path.to_str().unwrap(),
            "--format",
            "text",
            "--once",
        ])
        .env("PATH", path)
        .env("HOME", temp.path());
    if let Some(mode) = mode {
        command.env("OCI_TEST_MODE", mode);
    }
    command.output().unwrap()
}

// The fake CLI is a POSIX shell script.  Keep these black-box tests scoped to
// the hosted Linux coverage lane rather than invoking a real OCI CLI.
#[test]
fn cli_once_acquires_capacity_and_runs_failsoft_hooks() {
    let temp = TempDir::new("success");
    let output = run_lottery(&temp, None);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let acquired: Value =
        serde_json::from_slice(&std::fs::read(temp.path().join("acquired.json")).unwrap()).unwrap();
    assert_eq!(acquired["instance_ocid"], "ocid1.instance.test");
    assert_eq!(acquired["public_ip"], "198.51.100.40");
    let state: Value =
        serde_json::from_slice(&std::fs::read(temp.path().join("state.json")).unwrap()).unwrap();
    assert_eq!(state["attempts"], 1);
    assert_eq!(state["acquired"], true);
}

#[cfg(unix)]
#[test]
fn cli_once_records_out_of_capacity_and_exits_cleanly() {
    let temp = TempDir::new("capacity");
    let output = run_lottery(&temp, Some("capacity"));
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let state: Value =
        serde_json::from_slice(&std::fs::read(temp.path().join("state.json")).unwrap()).unwrap();
    assert_eq!(state["attempts"], 1);
    assert!(
        state["last_error"] == "out-of-capacity"
            || state["last_error"]
                .as_str()
                .unwrap_or_default()
                .contains("OutOfHostCapacity")
    );
    assert!(!temp.path().join("acquired.json").exists());
}
