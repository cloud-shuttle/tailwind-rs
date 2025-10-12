//! High-level browser automation APIs

use crate::chrome_devtools::ChromeDevTools;
use crate::error::{DevToolsError, Result};
use crate::{BrowserTestConfig, Viewport};
use std::process::Stdio;
use tokio::process::Command;

/// Browser automation session
pub struct BrowserSession {
    devtools: Option<ChromeDevTools>,
    config: BrowserTestConfig,
    chrome_process: Option<tokio::process::Child>,
}

impl BrowserSession {
    /// Start a new browser session
    pub async fn start(config: BrowserTestConfig) -> Result<Self> {
        log::info!("Starting browser session with config: {:?}", config);

        // Start Chrome with remote debugging enabled
        let chrome_process = Self::launch_chrome(&config).await?;

        // Give Chrome time to start
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Connect to DevTools
        let devtools_url = "ws://127.0.0.1:9222/devtools/browser";
        let devtools = Some(ChromeDevTools::connect(devtools_url).await?);

        Ok(Self {
            devtools,
            config,
            chrome_process: Some(chrome_process),
        })
    }

    /// Launch Chrome with remote debugging
    async fn launch_chrome(config: &BrowserTestConfig) -> Result<tokio::process::Child> {
        let mut args = vec![
            "--remote-debugging-port=9222".to_string(),
            "--no-first-run".to_string(),
            "--no-default-browser-check".to_string(),
            "--disable-background-timer-throttling".to_string(),
            "--disable-backgrounding-occluded-windows".to_string(),
            "--disable-renderer-backgrounding".to_string(),
        ];

        if config.headless {
            args.push("--headless".to_string());
        }

        args.push(format!("--window-size={},{}", config.viewport.width, config.viewport.height));

        log::debug!("Launching Chrome with args: {:?}", args);

        let child = Command::new("google-chrome")
            .args(&args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| DevToolsError::Generic(format!("Failed to launch Chrome: {}", e)))?;

        Ok(child)
    }

    /// Navigate to a URL
    pub async fn navigate(&mut self, url: &str) -> Result<()> {
        if let Some(devtools) = &mut self.devtools {
            devtools.navigate(url).await?;
            // Wait for page to load
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
        Ok(())
    }

    /// Take a screenshot
    pub async fn screenshot(&mut self, filename: Option<&str>) -> Result<Vec<u8>> {
        if let Some(devtools) = &mut self.devtools {
            let data = devtools.take_screenshot().await?;

            if let Some(filename) = filename {
                if let Some(dir) = &self.config.screenshots_dir {
                    std::fs::create_dir_all(dir)?;
                    let path = format!("{}/{}", dir, filename);
                    std::fs::write(&path, &data)?;
                    log::info!("Screenshot saved to: {}", path);
                }
            }

            Ok(data)
        } else {
            Err(DevToolsError::Generic("DevTools not connected".to_string()))
        }
    }

    /// Execute JavaScript and return result
    pub async fn execute_js(&mut self, script: &str) -> Result<serde_json::Value> {
        if let Some(devtools) = &mut self.devtools {
            devtools.evaluate_js(script).await
        } else {
            Err(DevToolsError::Generic("DevTools not connected".to_string()))
        }
    }

    /// Get page title
    pub async fn get_title(&mut self) -> Result<String> {
        if let Some(devtools) = &mut self.devtools {
            devtools.get_title().await
        } else {
            Err(DevToolsError::Generic("DevTools not connected".to_string()))
        }
    }

    /// Wait for element to appear
    pub async fn wait_for_element(&mut self, selector: &str) -> Result<()> {
        if let Some(devtools) = &mut self.devtools {
            devtools.wait_for_element(selector, 10000).await
        } else {
            Err(DevToolsError::Generic("DevTools not connected".to_string()))
        }
    }

    /// Check if element exists
    pub async fn element_exists(&mut self, selector: &str) -> Result<bool> {
        let script = format!(
            "document.querySelector('{}') !== null",
            selector
        );
        let result = self.execute_js(&script).await?;
        result.as_bool()
            .ok_or_else(|| DevToolsError::Generic("Result is not a boolean".to_string()))
    }

    /// Get element text content
    pub async fn get_element_text(&mut self, selector: &str) -> Result<String> {
        let script = format!(
            "document.querySelector('{}')?.textContent || ''",
            selector
        );
        let result = self.execute_js(&script).await?;
        result.as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| DevToolsError::Generic("Result is not a string".to_string()))
    }

    /// Get element CSS property
    pub async fn get_element_css(&mut self, selector: &str, property: &str) -> Result<String> {
        let script = format!(
            "getComputedStyle(document.querySelector('{}')).getPropertyValue('{}')",
            selector, property
        );
        let result = self.execute_js(&script).await?;
        result.as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| DevToolsError::Generic("Result is not a string".to_string()))
    }
}

impl Drop for BrowserSession {
    fn drop(&mut self) {
        if let Some(mut process) = self.chrome_process.take() {
            let _ = tokio::spawn(async move {
                let _ = process.kill().await;
            });
        }
    }
}

/// Visual testing utilities
pub struct VisualTester {
    session: BrowserSession,
}

impl VisualTester {
    /// Create a new visual tester
    pub fn new(session: BrowserSession) -> Self {
        Self { session }
    }

    /// Check if colors are properly applied to elements
    pub async fn check_colors(&mut self) -> Result<ColorTestResult> {
        log::info!("Checking color rendering in the demo");

        let mut results = ColorTestResult::default();

        // Check main background gradient
        let bg_color = self.session.get_element_css("body", "background").await?;
        results.has_background_gradient = !bg_color.is_empty() && bg_color.contains("gradient");

        // Check for gradient text
        let gradient_text_exists = self.session.element_exists(".gradient-text").await?;
        results.has_gradient_text = gradient_text_exists;

        if gradient_text_exists {
            let text_color = self.session.get_element_css(".gradient-text", "background-image").await?;
            results.gradient_text_works = text_color.contains("gradient");
        }

        // Check demo cards with gradients
        let demo_cards = self.session.execute_js("document.querySelectorAll('[class*=\"bg-gradient\"]').length").await?;
        let card_count = demo_cards.as_u64().unwrap_or(0);
        results.demo_cards_with_gradients = card_count > 0;

        // Check if buttons have proper colors
        let button_colors = self.session.execute_js("Array.from(document.querySelectorAll('button')).map(btn => getComputedStyle(btn).backgroundImage || getComputedStyle(btn).backgroundColor)").await?;
        if let Some(colors) = button_colors.as_array() {
            results.button_colors_present = colors.iter().any(|c| {
                c.as_str().map(|s| !s.is_empty() && s != "rgba(0, 0, 0, 0)" && s != "transparent").unwrap_or(false)
            });
        }

        Ok(results)
    }

    /// Take a screenshot for visual comparison
    pub async fn capture_visual_state(&mut self, name: &str) -> Result<Vec<u8>> {
        self.session.screenshot(Some(&format!("{}.png", name))).await
    }
}

/// Results of color testing
#[derive(Debug, Default)]
pub struct ColorTestResult {
    pub has_background_gradient: bool,
    pub has_gradient_text: bool,
    pub gradient_text_works: bool,
    pub demo_cards_with_gradients: bool,
    pub button_colors_present: bool,
}

impl ColorTestResult {
    pub fn is_successful(&self) -> bool {
        self.has_background_gradient &&
        self.has_gradient_text &&
        self.gradient_text_works &&
        self.demo_cards_with_gradients &&
        self.button_colors_present
    }

    pub fn summary(&self) -> String {
        format!(
            "Color Test Results:\n\
             - Background gradient: {}\n\
             - Gradient text: {} (works: {})\n\
             - Demo cards with gradients: {}\n\
             - Button colors: {}\n\
             - Overall: {}",
            self.has_background_gradient,
            self.has_gradient_text,
            self.gradient_text_works,
            self.demo_cards_with_gradients,
            self.button_colors_present,
            if self.is_successful() { "✅ PASS" } else { "❌ FAIL" }
        )
    }
}
