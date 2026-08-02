//! Tailscale enroll — mints an ephemeral auth-key via the Tailscale API and
//! installs+brings up `tailscale` on the freshly-booted instance over SSH.

use crate::InstanceFile;
use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tracing::info;

#[derive(Debug, Serialize)]
struct CreateKeyReq<'a> {
    capabilities: Caps<'a>,
    #[serde(rename = "expirySeconds")]
    expiry_seconds: u64,
}

#[derive(Debug, Serialize)]
struct Caps<'a> {
    devices: Devices<'a>,
}

#[derive(Debug, Serialize)]
struct Devices<'a> {
    create: DeviceCreate<'a>,
}

#[derive(Debug, Serialize)]
struct DeviceCreate<'a> {
    reusable: bool,
    ephemeral: bool,
    preauthorized: bool,
    tags: &'a [&'a str],
}

#[derive(Debug, Deserialize)]
struct CreateKeyResp {
    key: String,
}

pub async fn enroll(inst: &InstanceFile) -> Result<()> {
    let api_key = std::env::var("TS_API_KEY").context("TS_API_KEY env missing")?;
    let tailnet = std::env::var("TS_TAILNET").context("TS_TAILNET env missing")?;

    let body = CreateKeyReq {
        capabilities: Caps {
            devices: Devices {
                create: DeviceCreate {
                    reusable: false,
                    ephemeral: true,
                    preauthorized: true,
                    tags: &["tag:oci", "tag:phenotype-mesh"],
                },
            },
        },
        expiry_seconds: 600,
    };

    let api_base =
        std::env::var("TS_API_BASE").unwrap_or_else(|_| "https://api.tailscale.com".to_string());
    let url = format!(
        "{}/api/v2/tailnet/{tailnet}/keys",
        api_base.trim_end_matches('/')
    );
    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .basic_auth(&api_key, Some(""))
        .json(&body)
        .send()
        .await?;
    if !resp.status().is_success() {
        let s = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(anyhow!("tailscale create-key {s}: {text}"));
    }
    let parsed: CreateKeyResp = resp.json().await?;
    info!("tailscale ephemeral auth-key minted");

    // SSH in, install + up.
    let remote_cmd = format!(
        "set -e; curl -fsSL https://tailscale.com/install.sh | sudo sh; \
         sudo tailscale up --auth-key={} --ssh --hostname={}-oci --accept-routes",
        parsed.key,
        inst.region.replace('_', "-")
    );
    let ssh_bin = std::env::var_os("TS_SSH_BIN").unwrap_or_else(|| "ssh".into());
    let status = Command::new(ssh_bin)
        .arg("-o")
        .arg("StrictHostKeyChecking=accept-new")
        .arg("-o")
        .arg("ConnectTimeout=10")
        .arg(format!("ubuntu@{}", inst.public_ip))
        .arg(&remote_cmd)
        .status()
        .await
        .context("spawn ssh for tailscale install")?;
    if !status.success() {
        return Err(anyhow!("ssh tailscale install exited {status}"));
    }
    info!(host = %inst.public_ip, "tailscale up complete");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    fn restore_env(name: &str, value: Option<std::ffi::OsString>) {
        unsafe {
            if let Some(value) = value {
                std::env::set_var(name, value);
            } else {
                std::env::remove_var(name);
            }
        }
    }

    fn instance() -> InstanceFile {
        InstanceFile {
            instance_ocid: "ocid1.test".into(),
            region: "us_test_1".into(),
            ad: "AD-1".into(),
            public_ip: "198.51.100.31".into(),
            acquired_at: "2026-08-02T00:00:00Z".into(),
        }
    }

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("oci-tail-{name}-{nonce}"));
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    #[tokio::test]
    async fn enroll_requires_api_key_and_tailnet_configuration() {
        let _guard = env_lock();
        let prior_key = std::env::var_os("TS_API_KEY");
        let prior_tailnet = std::env::var_os("TS_TAILNET");
        unsafe {
            std::env::remove_var("TS_API_KEY");
            std::env::remove_var("TS_TAILNET");
        }
        let err = enroll(&instance()).await.unwrap_err();
        assert!(err.to_string().contains("TS_API_KEY"));

        unsafe { std::env::set_var("TS_API_KEY", "test-key") };
        let err = enroll(&instance()).await.unwrap_err();
        assert!(err.to_string().contains("TS_TAILNET"));

        if let Some(value) = prior_key {
            restore_env("TS_API_KEY", Some(value));
        } else {
            restore_env("TS_API_KEY", None);
        }
        if let Some(value) = prior_tailnet {
            restore_env("TS_TAILNET", Some(value));
        } else {
            restore_env("TS_TAILNET", None);
        }
    }

    #[test]
    fn request_shape_is_ephemeral_and_pre_authorized() {
        let body = CreateKeyReq {
            capabilities: Caps {
                devices: Devices {
                    create: DeviceCreate {
                        reusable: false,
                        ephemeral: true,
                        preauthorized: true,
                        tags: &["tag:oci", "tag:phenotype-mesh"],
                    },
                },
            },
            expiry_seconds: 600,
        };
        let value = serde_json::to_value(body).unwrap();
        assert_eq!(value["expirySeconds"], 600);
        assert_eq!(
            value["capabilities"]["devices"]["create"]["ephemeral"],
            true
        );
        assert_eq!(
            value["capabilities"]["devices"]["create"]["tags"][1],
            "tag:phenotype-mesh"
        );
    }

    #[tokio::test]
    async fn enroll_creates_key_and_runs_ssh_bootstrap() {
        let _guard = env_lock();
        let old_key = std::env::var_os("TS_API_KEY");
        let old_tailnet = std::env::var_os("TS_TAILNET");
        let old_base = std::env::var_os("TS_API_BASE");
        let old_ssh = std::env::var_os("TS_SSH_BIN");
        let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = [0u8; 8192];
            let read = stream.read(&mut request).await.unwrap();
            let request = String::from_utf8_lossy(&request[..read]);
            assert!(request.starts_with("POST /api/v2/tailnet/example.test/keys"));
            let body = r#"{"key":"tskey-auth-test"}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });
        let dir = temp_dir("ssh");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let ssh = dir.join("ssh");
            std::fs::write(&ssh, b"#!/bin/sh\nexit 0\n").unwrap();
            std::fs::set_permissions(&ssh, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
        #[cfg(windows)]
        let ssh = {
            let ssh = dir.join("ssh.cmd");
            std::fs::write(&ssh, b"@exit /b 0\r\n").unwrap();
            ssh
        };
        #[cfg(unix)]
        let ssh = dir.join("ssh");

        unsafe {
            std::env::set_var("TS_API_KEY", "test-api-key");
            std::env::set_var("TS_TAILNET", "example.test");
            std::env::set_var("TS_API_BASE", &base);
            std::env::set_var("TS_SSH_BIN", &ssh);
        }
        enroll(&instance()).await.unwrap();
        server.await.unwrap();
        restore_env("TS_API_KEY", old_key);
        restore_env("TS_TAILNET", old_tailnet);
        restore_env("TS_API_BASE", old_base);
        restore_env("TS_SSH_BIN", old_ssh);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[tokio::test]
    async fn enroll_reports_api_failure_before_ssh() {
        let _guard = env_lock();
        let old_key = std::env::var_os("TS_API_KEY");
        let old_tailnet = std::env::var_os("TS_TAILNET");
        let old_base = std::env::var_os("TS_API_BASE");
        let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = [0u8; 4096];
            let _ = stream.read(&mut request).await.unwrap();
            let body = "denied";
            let response = format!(
                "HTTP/1.1 403 Forbidden\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });
        unsafe {
            std::env::set_var("TS_API_KEY", "test-api-key");
            std::env::set_var("TS_TAILNET", "example.test");
            std::env::set_var("TS_API_BASE", &base);
        }
        let err = enroll(&instance()).await.unwrap_err();
        assert!(err.to_string().contains("tailscale create-key"));
        server.await.unwrap();
        restore_env("TS_API_KEY", old_key);
        restore_env("TS_TAILNET", old_tailnet);
        restore_env("TS_API_BASE", old_base);
    }
}
