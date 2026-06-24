//! Desktop automation primitives

use crate::error::Result;
use async_trait::async_trait;
use crate::automation::Automator;

pub struct DesktopAutomator;

#[async_trait]
impl Automator for DesktopAutomator {
    async fn click(&mut self, x: i32, y: i32) -> Result<()> {
        tracing::info!("Click at ({}, {})", x, y);
        Ok(())
    }

    async fn type_text(&mut self, text: &str) -> Result<()> {
        tracing::info!("Type text: {}", text);
        Ok(())
    }

    async fn key_press(&mut self, key: &str) -> Result<()> {
        tracing::info!("Key press: {}", key);
        Ok(())
    }

    async fn screenshot(&mut self, path: Option<&str>) -> Result<String> {
        let p = path.unwrap_or("screenshot.png");
        tracing::info!("Screenshot -> {}", p);
        Ok(p.to_string())
    }

    async fn find_element(&mut self, selector: &str) -> Result<crate::automation::ElementHandle> {
        tracing::info!("Find element: {}", selector);
        Err(crate::error::KvdError::ElementNotFound(selector.to_string()))
    }

    async fn wait_for_element(&mut self, selector: &str, _timeout: Option<u64>) -> Result<crate::automation::ElementHandle> {
        tracing::info!("Wait for element: {}", selector);
        Err(crate::error::KvdError::ElementNotFound(selector.to_string()))
    }
}
