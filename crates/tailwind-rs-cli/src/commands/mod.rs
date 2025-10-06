//! CLI Command Implementations
//!
//! This module contains the implementation of all CLI commands:
//! - build: Generate CSS from source files
//! - init: Initialize a new Tailwind-RS project
//! - watch: Watch files and rebuild on changes

pub mod build;
pub mod init;
pub mod watch;

// Re-export command functions for easier access
pub use build::execute as build_execute;
pub use init::execute as init_execute;
pub use watch::execute as watch_execute;
