//! # tailwind-rs-devtools
//!
//! Chrome DevTools MCP integration for tailwind-rs. Provides browser automation,
//! visual testing, and debugging capabilities for Tailwind CSS generation.

pub mod chrome_devtools;
pub mod mcp_client;
pub mod browser_automation;
pub mod visual_testing;
pub mod error;

pub use chrome_devtools::*;
pub use browser_automation::*;
pub use visual_testing::*;
pub use error::*;

/// Initialize the devtools environment
pub fn init_devtools() -> crate::error::Result<()> {
    log::info!("Initializing Tailwind-RS DevTools");
    Ok(())
}

/// Test configuration for browser automation
#[derive(Debug, Clone)]
pub struct BrowserTestConfig {
    pub headless: bool,
    pub viewport: Viewport,
    pub timeout: std::time::Duration,
    pub screenshots_dir: Option<String>,
}

/// Viewport configuration
#[derive(Debug, Clone)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
        }
    }
}

impl Default for BrowserTestConfig {
    fn default() -> Self {
        Self {
            headless: true,
            viewport: Viewport::default(),
            timeout: std::time::Duration::from_secs(30),
            screenshots_dir: Some("test-screenshots".to_string()),
        }
    }
}
