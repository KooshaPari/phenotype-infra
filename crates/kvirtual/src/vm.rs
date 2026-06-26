//! VM management (libvirt/QEMU)

use crate::error::Result;

pub struct VmManager;

impl VmManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn list(&self) -> Result<Vec<String>> {
        tracing::info!("Listing VMs");
        Ok(vec![])
    }

    pub async fn create(&self, _name: &str, _template: &str) -> Result<()> {
        Ok(())
    }

    pub async fn start(&self, _name: &str) -> Result<()> {
        Ok(())
    }

    pub async fn stop(&self, _name: &str) -> Result<()> {
        Ok(())
    }
}
