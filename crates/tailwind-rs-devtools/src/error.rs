//! Error types for tailwind-rs-devtools

use std::fmt;

/// Errors that can occur during browser automation and DevTools operations
#[derive(Debug, thiserror::Error)]
pub enum DevToolsError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Chrome DevTools Protocol error: {0}")]
    Cdp(String),

    #[error("MCP communication error: {0}")]
    Mcp(String),

    #[error("Browser automation error: {0}")]
    Automation(String),

    #[error("Visual testing error: {0}")]
    VisualTesting(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Image processing error: {0}")]
    Image(String),

    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Generic error: {0}")]
    Generic(String),
}

/// Result type alias for DevTools operations
pub type Result<T> = std::result::Result<T, DevToolsError>;
