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

    let url = format!("https://api.tailscale.com/api/v2/tailnet/{tailnet}/keys");
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
    let status = Command::new("ssh")
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

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
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
            unsafe { std::env::set_var("TS_API_KEY", value) };
        }
        if let Some(value) = prior_tailnet {
            unsafe { std::env::set_var("TS_TAILNET", value) };
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
}
