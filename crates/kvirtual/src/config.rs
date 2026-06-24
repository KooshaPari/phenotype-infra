use crate::error::{KvdError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub desktop: DesktopConfig,
    pub container: ContainerConfig,
    pub vm: VmConfig,
    pub recording: RecordingConfig,
    pub session: SessionConfig,
    pub credentials: CredentialsConfig,
    pub scripting: ScriptingConfig,
    pub tui: TuiConfig,
    pub mcp: McpConfig,

    // Paths
    pub data_dir: PathBuf,
    pub session_dir: PathBuf,
    pub script_dir: PathBuf,
    pub recording_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub config_file: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub log_level: String,
    pub timeout: u64,
    pub max_history: usize,
    pub auto_save: bool,
    pub backup_on_start: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopConfig {
    pub click_delay: u64,
    pub type_delay: u64,
    pub screenshot_format: String,
    pub screenshot_quality: u8,
    pub element_timeout: u64,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerConfig {
    pub docker_socket: String,
    pub default_image: String,
    pub auto_remove: bool,
    pub network_mode: String,
    pub resource_limits: ResourceLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmConfig {
    pub hypervisor: String,
    pub default_template: String,
    pub auto_start: bool,
    pub snapshot_on_create: bool,
    pub connection_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingConfig {
    pub video_format: String,
    pub video_quality: String,
    pub audio_format: String,
    pub frame_rate: u32,
    pub max_duration: u64,
    pub auto_compress: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub auto_save_interval: u64,
    pub max_sessions: usize,
    pub session_timeout: u64,
    pub persist_history: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialsConfig {
    pub keyring_service: String,
    pub encryption_key: Option<String>,
    pub auto_lock_timeout: u64,
    pub require_confirmation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptingConfig {
    pub default_interpreter: String,
    pub timeout: u64,
    pub max_memory: u64,
    pub allowed_modules: Vec<String>,
    pub sandbox_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    pub theme: String,
    pub key_bindings: HashMap<String, String>,
    pub refresh_rate: u64,
    pub show_line_numbers: bool,
    pub word_wrap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    pub enabled: bool,
    pub server_port: u16,
    pub server_host: String,
    pub tools: Vec<String>,
    pub max_connections: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub memory: Option<u64>,
    pub cpu: Option<f64>,
    pub disk: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let data_dir = home_dir.join(".kvirtualdesktop");

        Self {
            general: GeneralConfig {
                log_level: "info".to_string(),
                timeout: 30000,
                max_history: 1000,
                auto_save: true,
                backup_on_start: true,
            },
            desktop: DesktopConfig {
                click_delay: 100,
                type_delay: 50,
                screenshot_format: "png".to_string(),
                screenshot_quality: 90,
                element_timeout: 5000,
                retry_count: 3,
            },
            container: ContainerConfig {
                docker_socket: "unix:///var/run/docker.sock".to_string(),
                default_image: "ubuntu:latest".to_string(),
                auto_remove: false,
                network_mode: "bridge".to_string(),
                resource_limits: ResourceLimits {
                    memory: Some(1024 * 1024 * 1024),
                    cpu: Some(1.0),
                    disk: Some(10 * 1024 * 1024 * 1024),
                },
            },
            vm: VmConfig {
                hypervisor: "qemu:///system".to_string(),
                default_template: "ubuntu20.04".to_string(),
                auto_start: false,
                snapshot_on_create: true,
                connection_timeout: 30000,
            },
            recording: RecordingConfig {
                video_format: "mp4".to_string(),
                video_quality: "high".to_string(),
                audio_format: "mp3".to_string(),
                frame_rate: 30,
                max_duration: 3600,
                auto_compress: true,
            },
            session: SessionConfig {
                auto_save_interval: 300,
                max_sessions: 10,
                session_timeout: 7200,
                persist_history: true,
            },
            credentials: CredentialsConfig {
                keyring_service: "kvirtualdesktop".to_string(),
                encryption_key: None,
                auto_lock_timeout: 1800,
                require_confirmation: true,
            },
            scripting: ScriptingConfig {
                default_interpreter: "python3".to_string(),
                timeout: 300000,
                max_memory: 512 * 1024 * 1024,
                allowed_modules: vec![
                    "os".to_string(),
                    "sys".to_string(),
                    "json".to_string(),
                    "time".to_string(),
                ],
                sandbox_mode: true,
            },
            tui: TuiConfig {
                theme: "default".to_string(),
                key_bindings: HashMap::new(),
                refresh_rate: 60,
                show_line_numbers: true,
                word_wrap: true,
            },
            mcp: McpConfig {
                enabled: false,
                server_port: 8080,
                server_host: "127.0.0.1".to_string(),
                tools: vec![],
                max_connections: 10,
            },
            data_dir: data_dir.clone(),
            session_dir: data_dir.join("sessions"),
            script_dir: data_dir.join("scripts"),
            recording_dir: data_dir.join("recordings"),
            cache_dir: data_dir.join("cache"),
            config_file: data_dir.join("config.toml"),
        }
    }
}

impl Config {
    pub fn load_default() -> Result<Self> {
        let config = Self::default();

        crate::utils::ensure_dir_exists(&config.data_dir)?;
        crate::utils::ensure_dir_exists(&config.session_dir)?;
        crate::utils::ensure_dir_exists(&config.script_dir)?;
        crate::utils::ensure_dir_exists(&config.recording_dir)?;
        crate::utils::ensure_dir_exists(&config.cache_dir)?;

        if config.config_file.exists() {
            Self::load_from_file(&config.config_file)
        } else {
            config.save()?;
            Ok(config)
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)
            .map_err(|e| KvdError::Config(format!("TOML parse: {}", e)))?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| KvdError::Config(format!("TOML serialize: {}", e)))?;
        std::fs::write(&self.config_file, content)?;
        Ok(())
    }

    pub fn get_value(&self, key: &str) -> Result<String> {
        match key {
            "general.log_level" => Ok(self.general.log_level.clone()),
            "general.timeout" => Ok(self.general.timeout.to_string()),
            "desktop.click_delay" => Ok(self.desktop.click_delay.to_string()),
            "desktop.type_delay" => Ok(self.desktop.type_delay.to_string()),
            "recording.video_format" => Ok(self.recording.video_format.clone()),
            "session.auto_save_interval" => Ok(self.session.auto_save_interval.to_string()),
            _ => Err(KvdError::Config(format!("Unknown config key: {}", key))),
        }
    }

    pub fn set_value(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "general.log_level" => self.general.log_level = value.to_string(),
            "general.timeout" => self.general.timeout = value.parse()
                .map_err(|_| KvdError::Config("Invalid timeout value".into()))?,
            "desktop.click_delay" => self.desktop.click_delay = value.parse()
                .map_err(|_| KvdError::Config("Invalid click delay value".into()))?,
            "desktop.type_delay" => self.desktop.type_delay = value.parse()
                .map_err(|_| KvdError::Config("Invalid type delay value".into()))?,
            "recording.video_format" => self.recording.video_format = value.to_string(),
            "session.auto_save_interval" => self.session.auto_save_interval = value.parse()
                .map_err(|_| KvdError::Config("Invalid auto save interval value".into()))?,
            _ => return Err(KvdError::Config(format!("Unknown config key: {}", key))),
        }
        Ok(())
    }
}
