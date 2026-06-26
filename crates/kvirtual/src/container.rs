//! Container management (Docker/bollard)

use crate::error::Result;

pub struct ContainerManager;

impl ContainerManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        tracing::info!("Listing containers");
        Ok(vec![])
    }

    pub async fn create(&self, _name: &str, _image: &str) -> Result<()> {
        tracing::info!("Container create not yet implemented");
        Ok(())
    }

    pub async fn start(&self, _name: &str) -> Result<()> {
        Ok(())
    }

    pub async fn stop(&self, _name: &str) -> Result<()> {
        Ok(())
    }
}
