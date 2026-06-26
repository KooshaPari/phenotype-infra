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
        oci_helpers::load_json_or(path, Self::default()).await
    }

    pub async fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        oci_helpers::save_json(path, self).await
    }
}

pub async fn write_acquired(path: &PathBuf, inst: &AcquiredInstance) -> anyhow::Result<()> {
    oci_helpers::save_json(path, inst).await
}
