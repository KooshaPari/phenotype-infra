use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LotteryState {
    pub attempts: u64,
    pub last_attempt: Option<DateTime<Utc>>,
    pub last_region: Option<String>,
    pub last_ad: Option<u8>,
    pub last_error: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub acquired: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquiredInstance {
    pub instance_ocid: String,
    pub region: String,
    pub ad: u8,
    pub public_ip: Option<String>,
    pub acquired_at: DateTime<Utc>,
}

impl LotteryState {
    pub async fn load(path: &PathBuf) -> anyhow::Result<Self> {
        if tokio::fs::try_exists(path).await.unwrap_or(false) {
            let raw = tokio::fs::read_to_string(path).await?;
            Ok(serde_json::from_str(&raw).unwrap_or_default())
        } else {
            Ok(Self::default())
        }
    }

    pub async fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let raw = serde_json::to_string_pretty(self)?;
        tokio::fs::write(path, raw).await?;
        Ok(())
    }
}

pub async fn write_acquired(path: &PathBuf, inst: &AcquiredInstance) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let raw = serde_json::to_string_pretty(inst)?;
    tokio::fs::write(path, raw).await?;
    Ok(())
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
        std::env::temp_dir().join(format!("oci-lottery-state-{name}-{nonce}.json"))
    }

    fn acquired() -> AcquiredInstance {
        AcquiredInstance {
            instance_ocid: "ocid1.test".into(),
            region: "us-test-1".into(),
            ad: 2,
            public_ip: None,
            acquired_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn load_missing_and_malformed_fall_back_to_default() {
        let missing = temp_path("missing");
        assert_eq!(LotteryState::load(&missing).await.unwrap().attempts, 0);

        let malformed = temp_path("malformed");
        tokio::fs::write(&malformed, b"broken").await.unwrap();
        assert_eq!(LotteryState::load(&malformed).await.unwrap().attempts, 0);
        let _ = tokio::fs::remove_file(malformed).await;
    }

    #[tokio::test]
    async fn save_and_load_round_trip() {
        let path = temp_path("roundtrip");
        let state = LotteryState {
            attempts: 4,
            last_region: Some("eu-test-1".into()),
            acquired: true,
            ..LotteryState::default()
        };
        state.save(&path).await.unwrap();
        let loaded = LotteryState::load(&path).await.unwrap();
        assert_eq!(loaded.attempts, 4);
        assert_eq!(loaded.last_region.as_deref(), Some("eu-test-1"));
        assert!(loaded.acquired);
        let _ = tokio::fs::remove_file(path).await;
    }

    #[tokio::test]
    async fn write_acquired_creates_parent_and_round_trips() {
        let path = temp_path("nested").join("instance.json");
        let inst = acquired();
        write_acquired(&path, &inst).await.unwrap();
        let raw = tokio::fs::read_to_string(&path).await.unwrap();
        let loaded: AcquiredInstance = serde_json::from_str(&raw).unwrap();
        assert_eq!(loaded.instance_ocid, inst.instance_ocid);
        assert_eq!(loaded.public_ip, None);
        let _ = tokio::fs::remove_dir_all(path.parent().unwrap()).await;
    }
}
