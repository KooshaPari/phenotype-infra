//! Screen and audio recording

use crate::error::Result;

pub struct Recorder;

impl Recorder {
    pub fn new() -> Self {
        Self
    }

    pub async fn start_recording(&self, _output: &str) -> Result<()> {
        tracing::info!("Recording start — not yet implemented");
        Ok(())
    }

    pub async fn stop_recording(&self) -> Result<()> {
        Ok(())
    }

    pub async fn take_screenshot(&self, _path: Option<&str>) -> Result<String> {
        Ok("screenshot.png".to_string())
    }

    pub async fn start_audio(&self, _output: &str) -> Result<()> {
        tracing::info!("Audio recording — not yet implemented");
        Ok(())
    }
}
