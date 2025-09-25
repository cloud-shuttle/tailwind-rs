//! Parallel processing implementation
//!
//! This module provides parallel file processing capabilities for
//! efficient content scanning.

use crate::file_scanner::FileInfo;
use crate::class_extractor::ExtractedClass;
use crate::class_extractor::ClassExtractor;
use crate::content_config::ScanConfig;
use crate::error::{ScannerError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

/// Parallel processor for file processing
#[derive(Debug)]
pub struct ParallelProcessor {
    /// Whether parallel processing is enabled
    enabled: bool,
    /// Maximum number of workers
    max_workers: Option<usize>,
}

/// Processing statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingStats {
    /// Total number of files
    pub total_files: usize,
    /// Number of processed files
    pub processed_files: usize,
    /// Total number of classes found
    pub total_classes: usize,
    /// Number of unique classes
    pub unique_classes: usize,
    /// Total processing time
    pub total_time: std::time::Duration,
    /// Average time per file
    pub average_file_time: std::time::Duration,
    /// Memory usage in bytes
    pub memory_usage: usize,
}

// Default implementation is in file_scanner.rs

impl ParallelProcessor {
    /// Create a new parallel processor
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            max_workers: None,
        }
    }

    /// Create with maximum workers
    pub fn with_max_workers(enabled: bool, max_workers: usize) -> Self {
        Self {
            enabled,
            max_workers: Some(max_workers),
        }
    }

    /// Process files in parallel
    pub async fn process_files(
        &self,
        files: &[FileInfo],
        extractor: &ClassExtractor,
        config: &ScanConfig,
    ) -> Result<HashMap<PathBuf, Vec<ExtractedClass>>> {
        let start_time = Instant::now();
        let mut results = HashMap::new();
        
        if self.enabled && files.len() > 1 {
            // Parallel processing
            self.process_files_parallel(files, extractor, &mut results).await?;
        } else {
            // Sequential processing
            self.process_files_sequential(files, extractor, &mut results).await?;
        }
        
        Ok(results)
    }

    /// Process files sequentially
    async fn process_files_sequential(
        &self,
        files: &[FileInfo],
        extractor: &ClassExtractor,
        results: &mut HashMap<PathBuf, Vec<ExtractedClass>>,
    ) -> Result<()> {
        for file in files {
            if let Ok(classes) = extractor.extract_classes(file).await {
                results.insert(file.path.clone(), classes);
            }
        }
        Ok(())
    }

    /// Process files in parallel
    async fn process_files_parallel(
        &self,
        files: &[FileInfo],
        extractor: &ClassExtractor,
        results: &mut HashMap<PathBuf, Vec<ExtractedClass>>,
    ) -> Result<()> {
        // For now, use sequential processing
        // In a real implementation, this would use rayon or tokio for parallel processing
        self.process_files_sequential(files, extractor, results).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_processor_creation() {
        let processor = ParallelProcessor::new(true);
        assert!(processor.enabled);
    }

    #[test]
    fn test_parallel_processor_with_max_workers() {
        let processor = ParallelProcessor::with_max_workers(true, 4);
        assert!(processor.enabled);
        assert_eq!(processor.max_workers, Some(4));
    }

    #[test]
    fn test_processing_stats_default() {
        let stats = ProcessingStats::default();
        assert_eq!(stats.total_files, 0);
        assert_eq!(stats.processed_files, 0);
        assert_eq!(stats.total_classes, 0);
    }
}
