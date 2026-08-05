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
        std::env::temp_dir().join(format!("oci-lottery-state-{name}-{nonce}/state.json"))
    }

    #[tokio::test]
    async fn state_and_acquired_files_roundtrip_and_create_parents() {
        let state_path = temp_path("roundtrip");
        let state = LotteryState {
            attempts: 3,
            last_region: Some("test-region".into()),
            acquired: true,
            ..LotteryState::default()
        };
        let missing = LotteryState::load(&state_path).await.unwrap();
        assert_eq!(missing.attempts, 0);
        assert!(!missing.acquired);
        state.save(&state_path).await.unwrap();
        assert_eq!(LotteryState::load(&state_path).await.unwrap().attempts, 3);

        let acquired = AcquiredInstance {
            instance_ocid: "ocid1.test".into(),
            region: "test-region".into(),
            ad: 1,
            public_ip: None,
            acquired_at: Utc::now(),
        };
        let acquired_path = state_path.parent().unwrap().join("acquired.json");
        write_acquired(&acquired_path, &acquired).await.unwrap();
        assert_eq!(
            serde_json::from_slice::<AcquiredInstance>(&tokio::fs::read(acquired_path).await.unwrap())
                .unwrap()
                .instance_ocid,
            "ocid1.test"
        );
        let _ = tokio::fs::remove_dir_all(state_path.parent().unwrap()).await;
    }

    #[tokio::test]
    async fn invalid_state_json_falls_back_to_default() {
        let path = temp_path("invalid");
        tokio::fs::create_dir_all(path.parent().unwrap()).await.unwrap();
        tokio::fs::write(&path, b"not-json").await.unwrap();
        let recovered = LotteryState::load(&path).await.unwrap();
        assert_eq!(recovered.attempts, 0);
        assert!(!recovered.acquired);
        let _ = tokio::fs::remove_dir_all(path.parent().unwrap()).await;
    }
}
