use oci_helpers::home_or_fallback;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Runtime configuration for the OCI lottery daemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Regions to attempt, in priority order.
    pub regions: Vec<String>,
    /// Availability domain index to try (1-based, OCI-style "AD-1"). Optional;
    /// if `None`, daemon will iterate ADs 1..=3 per region.
    pub availability_domains: Option<Vec<u8>>,
    /// Compatibility shape — defaults to VM.Standard.A1.Flex (Always-Free Ampere).
    pub shape: String,
    /// OCPUs to request (Always-Free max = 4 across all A1 instances).
    pub ocpus: u8,
    /// Memory GB (Always-Free max = 24 across all A1 instances).
    pub memory_gb: u8,
    /// Image OCID (Ubuntu 22.04 ARM by default in your tenancy — must be set).
    pub image_ocid: Option<String>,
    /// Subnet OCID for the new instance.
    pub subnet_ocid: Option<String>,
    /// Display name for the instance.
    pub display_name: String,
    /// SSH public key path.
    pub ssh_authorized_keys_path: PathBuf,
    /// OCI CLI profile name in `~/.oci/config`.
    pub profile: String,
    /// Compartment OCID.
    pub compartment_ocid: Option<String>,
    /// Min/max backoff seconds between attempts.
    pub backoff_min_secs: u64,
    pub backoff_max_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            regions: vec![
                "ap-tokyo-1".into(),
                "ap-osaka-1".into(),
                "eu-frankfurt-1".into(),
                "us-ashburn-1".into(),
                "sa-saopaulo-1".into(),
            ],
            availability_domains: None,
            shape: "VM.Standard.A1.Flex".into(),
            ocpus: 4,
            memory_gb: 24,
            image_ocid: None,
            subnet_ocid: None,
            display_name: "phenotype-arm-mesh-node".into(),
            ssh_authorized_keys_path: home_or_fallback().join(".ssh").join("id_ed25519.pub"),
            profile: "DEFAULT".into(),
            compartment_ocid: None,
            backoff_min_secs: 60,
            backoff_max_secs: 180,
        }
    }
}

impl Config {
    pub async fn load_or_default(path: &PathBuf) -> anyhow::Result<Self> {
        if tokio::fs::try_exists(path).await.unwrap_or(false) {
            let raw = tokio::fs::read_to_string(path).await?;
            let cfg: Config = serde_json::from_str(&raw)?;
            Ok(cfg)
        } else {
            Ok(Self::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("oci-lottery-{name}-{nonce}.json"))
    }

    #[tokio::test]
    async fn load_or_default_uses_default_when_missing() {
        let path = temp_path("missing");
        let cfg = Config::load_or_default(&path).await.unwrap();
        assert_eq!(cfg.profile, "DEFAULT");
        assert!(cfg.availability_domains.is_none());
    }

    #[tokio::test]
    async fn load_or_default_reads_json() {
        let path = temp_path("valid");
        let expected = Config {
            regions: vec!["test-region".into()],
            display_name: "test-node".into(),
            ..Config::default()
        };
        tokio::fs::write(&path, serde_json::to_vec(&expected).unwrap())
            .await
            .unwrap();
        let actual = Config::load_or_default(&path).await.unwrap();
        assert_eq!(actual.regions, expected.regions);
        assert_eq!(actual.display_name, expected.display_name);
        let _ = tokio::fs::remove_file(path).await;
    }

    #[tokio::test]
    async fn load_or_default_rejects_invalid_json() {
        let path = temp_path("invalid");
        tokio::fs::write(&path, b"not-json").await.unwrap();
        assert!(Config::load_or_default(&path).await.is_err());
        let _ = tokio::fs::remove_file(path).await;
    }
}
