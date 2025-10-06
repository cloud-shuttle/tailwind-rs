//! File Watching Utilities
//!
//! Provides file system monitoring capabilities for the watch command,
//! detecting changes to source files and triggering rebuilds.

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::time::{Duration, SystemTime};
use tokio::time;
use glob::Pattern;

/// File watcher for monitoring source file changes
pub struct FileWatcher {
    /// File watcher instance
    watcher: RecommendedWatcher,
    /// Receiver for file change events
    receiver: Receiver<notify::Result<Event>>,
    /// Content patterns to monitor
    patterns: Vec<String>,
    /// File modification times cache
    file_times: HashMap<PathBuf, SystemTime>,
    /// Polling interval
    poll_interval: Duration,
}

impl FileWatcher {
    /// Create a new file watcher
    pub fn new(patterns: Vec<String>, poll_interval: Duration) -> Result<Self, FileWatcherError> {
        let (tx, receiver) = channel();

        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default(),
        )?;

        Ok(Self {
            watcher,
            receiver,
            patterns,
            file_times: HashMap::new(),
            poll_interval,
        })
    }

    /// Start watching files matching the configured patterns
    pub fn start(&mut self) -> Result<(), FileWatcherError> {
        // Find all files matching patterns and start watching them
        let files_to_watch = self.find_files_to_watch()?;

        for file_path in files_to_watch {
            // Watch the file's directory for changes
            if let Some(parent) = file_path.parent() {
                self.watcher.watch(parent, RecursiveMode::NonRecursive)?;
            }

            // Store initial modification time
            if let Ok(metadata) = std::fs::metadata(&file_path) {
                if let Ok(modified) = metadata.modified() {
                    self.file_times.insert(file_path.clone(), modified);
                }
            }
        }

        Ok(())
    }

    /// Wait for file changes and return changed files
    pub async fn wait_for_changes(&mut self) -> Result<Vec<PathBuf>, FileWatcherError> {
        // In a real implementation, we'd use the notify events
        // For now, we'll use polling as a simple implementation

        loop {
            // Check for file changes by polling
            let changed_files = self.check_for_changes()?;

            if !changed_files.is_empty() {
                // Update modification times
                for file in &changed_files {
                    if let Ok(metadata) = std::fs::metadata(file) {
                        if let Ok(modified) = metadata.modified() {
                            self.file_times.insert(file.clone(), modified);
                        }
                    }
                }

                return Ok(changed_files);
            }

            // Wait before next check
            time::sleep(self.poll_interval).await;
        }
    }

    /// Check for file changes using polling
    fn check_for_changes(&self) -> Result<Vec<PathBuf>, FileWatcherError> {
        let mut changed_files = Vec::new();

        // Find all current files matching patterns
        let current_files = self.find_files_to_watch()?;

        // Check each file for changes
        for file_path in current_files {
            let current_modified = std::fs::metadata(&file_path)
                .and_then(|m| m.modified())
                .unwrap_or(SystemTime::UNIX_EPOCH);

            if let Some(&last_modified) = self.file_times.get(&file_path) {
                if current_modified > last_modified {
                    changed_files.push(file_path);
                }
            } else {
                // New file
                changed_files.push(file_path);
            }
        }

        Ok(changed_files)
    }

    /// Find all files that match the configured patterns
    fn find_files_to_watch(&self) -> Result<Vec<PathBuf>, FileWatcherError> {
        let mut files = Vec::new();

        for pattern in &self.patterns {
            let matching_files = self.find_files_matching_pattern(pattern)?;
            files.extend(matching_files);
        }

        // Remove duplicates
        files.sort();
        files.dedup();

        Ok(files)
    }

    /// Find files matching a specific pattern
    fn find_files_matching_pattern(&self, pattern: &str) -> Result<Vec<PathBuf>, FileWatcherError> {
        let mut files = Vec::new();

        if pattern.contains('*') {
            // Glob pattern - use walkdir to find matches
            let glob_pattern = Pattern::new(pattern)?;

            for entry in walkdir::WalkDir::new(".")
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() {
                    let path_str = path.to_string_lossy();
                    if glob_pattern.matches(&path_str) {
                        files.push(path.to_path_buf());
                    }
                }
            }
        } else {
            // Direct file path
            let path = Path::new(pattern);
            if path.exists() && path.is_file() {
                files.push(path.to_path_buf());
            }
        }

        Ok(files)
    }
}

/// File watcher error types
#[derive(Debug, thiserror::Error)]
pub enum FileWatcherError {
    #[error("Notify watcher error: {0}")]
    Notify(#[from] notify::Error),

    #[error("Glob pattern error: {0}")]
    Glob(#[from] glob::PatternError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("File watching failed: {0}")]
    WatchFailed(String),
}
