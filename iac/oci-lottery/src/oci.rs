//! Thin wrapper over the `oci` CLI. We deliberately shell out rather than
//! pull the OCI Rust SDK — keeps auth handling delegated to the user's
//! existing `~/.oci/config` and avoids surface area.

use anyhow::{Context, Result, anyhow};
use serde_json::Value;
use tokio::process::Command;

use crate::config::Config;

#[derive(Debug)]
pub struct LaunchOutcome {
    /// True when capacity was granted (instance OCID returned).
    pub success: bool,
    pub instance_ocid: Option<String>,
    pub raw_stdout: String,
    pub raw_stderr: String,
    /// Set when the CLI returned a structured error string we recognized as
    /// "out of capacity" — so the caller can keep looping silently.
    pub out_of_capacity: bool,
}

/// Discover the AD names for a given region by listing them via `oci`.
pub async fn list_availability_domains(cfg: &Config, region: &str) -> Result<Vec<String>> {
    let compartment = cfg
        .compartment_ocid
        .as_deref()
        .ok_or_else(|| anyhow!("compartment_ocid not set"))?;

    let out = Command::new("oci")
        .args([
            "iam",
            "availability-domain",
            "list",
            "--compartment-id",
            compartment,
            "--region",
            region,
            "--profile",
            &cfg.profile,
        ])
        .output()
        .await
        .context("invoking `oci iam availability-domain list`")?;

    if !out.status.success() {
        return Err(anyhow!(
            "oci availability-domain list failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    let v: Value = serde_json::from_slice(&out.stdout)?;
    let names = parse_ad_names(&v);
    Ok(names)
}

fn parse_ad_names(v: &Value) -> Vec<String> {
    v.get("data")
        .and_then(|d| d.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|e| e.get("name").and_then(|n| n.as_str().map(String::from)))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

/// Attempt to launch an A1.Flex instance in the given region/AD.
pub async fn try_launch(cfg: &Config, region: &str, ad_name: &str) -> Result<LaunchOutcome> {
    let compartment = cfg
        .compartment_ocid
        .as_deref()
        .ok_or_else(|| anyhow!("compartment_ocid not set"))?;
    let image = cfg
        .image_ocid
        .as_deref()
        .ok_or_else(|| anyhow!("image_ocid not set"))?;
    let subnet = cfg
        .subnet_ocid
        .as_deref()
        .ok_or_else(|| anyhow!("subnet_ocid not set"))?;

    let shape_config = serde_json::json!({
        "ocpus": cfg.ocpus,
        "memoryInGBs": cfg.memory_gb
    })
    .to_string();

    let ssh_key = tokio::fs::read_to_string(&cfg.ssh_authorized_keys_path)
        .await
        .with_context(|| {
            format!(
                "reading ssh public key {}",
                cfg.ssh_authorized_keys_path.display()
            )
        })?;
    let metadata = serde_json::json!({ "ssh_authorized_keys": ssh_key.trim() }).to_string();

    let out = Command::new("oci")
        .args([
            "compute",
            "instance",
            "launch",
            "--region",
            region,
            "--profile",
            &cfg.profile,
            "--availability-domain",
            ad_name,
            "--compartment-id",
            compartment,
            "--shape",
            &cfg.shape,
            "--shape-config",
            &shape_config,
            "--image-id",
            image,
            "--subnet-id",
            subnet,
            "--display-name",
            &cfg.display_name,
            "--metadata",
            &metadata,
            "--wait-for-state",
            "RUNNING",
            "--max-wait-seconds",
            "600",
        ])
        .output()
        .await
        .context("invoking `oci compute instance launch`")?;

    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

    if out.status.success() {
        let v: Value = serde_json::from_str(&stdout).unwrap_or(Value::Null);
        let ocid = parse_instance_id(&v);
        return Ok(LaunchOutcome {
            success: ocid.is_some(),
            instance_ocid: ocid,
            raw_stdout: stdout,
            raw_stderr: stderr,
            out_of_capacity: false,
        });
    }

    let out_of_capacity = is_out_of_capacity(stderr.as_str());

    Ok(LaunchOutcome {
        success: false,
        instance_ocid: None,
        raw_stdout: stdout,
        raw_stderr: stderr,
        out_of_capacity,
    })
}

/// Best-effort lookup of the public IP of a freshly-launched instance.
pub async fn instance_public_ip(
    cfg: &Config,
    region: &str,
    instance_ocid: &str,
) -> Result<Option<String>> {
    let compartment = cfg
        .compartment_ocid
        .as_deref()
        .ok_or_else(|| anyhow!("compartment_ocid not set"))?;

    let out = Command::new("oci")
        .args([
            "compute",
            "instance",
            "list-vnics",
            "--instance-id",
            instance_ocid,
            "--compartment-id",
            compartment,
            "--region",
            region,
            "--profile",
            &cfg.profile,
        ])
        .output()
        .await?;

    if !out.status.success() {
        return Ok(None);
    }
    let v: Value = serde_json::from_slice(&out.stdout)?;
    let ip = parse_public_ip(&v);
    Ok(ip)
}

fn parse_instance_id(v: &Value) -> Option<String> {
    v.pointer("/data/id")
        .and_then(|n| n.as_str())
        .map(String::from)
}

fn parse_public_ip(v: &Value) -> Option<String> {
    v.pointer("/data/0/public-ip")
        .and_then(|n| n.as_str())
        .map(String::from)
}

fn is_out_of_capacity(stderr: &str) -> bool {
    let lower = stderr.to_ascii_lowercase();
    lower.contains("out of host capacity")
        || lower.contains("outofhostcapacity")
        || lower.contains("internalerror")
        || lower.contains("toomanyrequests")
        || lower.contains("limitexceeded")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ad_names_keeps_named_entries_only() {
        let v = serde_json::json!({
            "data": [
                {"name": "AD-1"},
                {"id": "without-name"},
                {"name": 7},
                {"name": "AD-3"}
            ]
        });
        assert_eq!(parse_ad_names(&v), vec!["AD-1", "AD-3"]);
        assert!(parse_ad_names(&serde_json::json!({"data": null})).is_empty());
        assert!(parse_ad_names(&serde_json::json!({})).is_empty());
    }

    #[test]
    fn parse_instance_id_and_public_ip_require_expected_shapes() {
        let v = serde_json::json!({
            "data": {
                "id": "ocid1.instance.test"
            }
        });
        assert_eq!(
            parse_instance_id(&v).as_deref(),
            Some("ocid1.instance.test")
        );
        assert_eq!(parse_instance_id(&serde_json::json!({})), None);

        let v = serde_json::json!({"data": [{"public-ip": "198.51.100.7"}]});
        assert_eq!(parse_public_ip(&v).as_deref(), Some("198.51.100.7"));
        assert_eq!(parse_public_ip(&serde_json::json!({"data": []})), None);
    }

    #[test]
    fn capacity_error_classifier_matches_known_cli_errors() {
        for message in [
            "Out of Host Capacity",
            "OutOfHostCapacity",
            "InternalError while launching",
            "TooManyRequests",
            "LimitExceeded",
        ] {
            assert!(is_out_of_capacity(message), "not classified: {message}");
        }
        assert!(!is_out_of_capacity("invalid image OCID"));
    }

    #[tokio::test]
    async fn required_launch_configuration_is_validated_before_cli() {
        let cfg = Config::default();
        assert!(list_availability_domains(&cfg, "us-test-1").await.is_err());
        assert!(try_launch(&cfg, "us-test-1", "AD-1").await.is_err());
        assert!(
            instance_public_ip(&cfg, "us-test-1", "ocid1.test")
                .await
                .is_err()
        );
    }
}
