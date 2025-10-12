//! Visual testing utilities for comparing screenshots and detecting visual regressions

use crate::error::{DevToolsError, Result};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::path::Path;

/// Visual comparison result
#[derive(Debug)]
pub struct VisualComparison {
    pub pixel_difference: u64,
    pub total_pixels: u64,
    pub difference_percentage: f64,
    pub diff_image: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
}

impl VisualComparison {
    /// Compare two images and return the difference
    pub fn compare_images(img1: &DynamicImage, img2: &DynamicImage) -> Result<Self> {
        let (width1, height1) = img1.dimensions();
        let (width2, height2) = img2.dimensions();

        if width1 != width2 || height1 != height2 {
            return Err(DevToolsError::VisualTesting(
                format!("Image dimensions don't match: {}x{} vs {}x{}", width1, height1, width2, height2)
            ));
        }

        let mut diff_pixels = 0u64;
        let total_pixels = (width1 * height1) as u64;
        let mut diff_image = ImageBuffer::new(width1, height1);

        for (x, y, pixel1) in img1.pixels() {
            let pixel2 = img2.get_pixel(x, y);
            let diff = Self::pixel_difference(pixel1, pixel2);

            if diff > 10 { // Threshold for considering pixels different
                diff_pixels += 1;

                // Create red highlight for differences
                diff_image.put_pixel(x, y, Rgba([255, 0, 0, 255]));
            } else {
                // Copy original pixel
                diff_image.put_pixel(x, y, pixel1);
            }
        }

        let difference_percentage = (diff_pixels as f64 / total_pixels as f64) * 100.0;

        Ok(Self {
            pixel_difference: diff_pixels,
            total_pixels,
            difference_percentage,
            diff_image: Some(diff_image),
        })
    }

    /// Calculate the difference between two pixels
    fn pixel_difference(p1: Rgba<u8>, p2: Rgba<u8>) -> u32 {
        let r_diff = (p1[0] as i32 - p2[0] as i32).abs() as u32;
        let g_diff = (p1[1] as i32 - p2[1] as i32).abs() as u32;
        let b_diff = (p1[2] as i32 - p2[2] as i32).abs() as u32;
        let a_diff = (p1[3] as i32 - p2[3] as i32).abs() as u32;

        r_diff + g_diff + b_diff + a_diff
    }

    /// Check if the comparison passes the threshold
    pub fn passes_threshold(&self, max_difference_percentage: f64) -> bool {
        self.difference_percentage <= max_difference_percentage
    }
}

/// Visual test suite for running multiple comparisons
pub struct VisualTestSuite {
    reference_dir: String,
    test_dir: String,
    diff_dir: String,
}

impl VisualTestSuite {
    /// Create a new visual test suite
    pub fn new(reference_dir: &str, test_dir: &str, diff_dir: &str) -> Result<Self> {
        std::fs::create_dir_all(reference_dir)?;
        std::fs::create_dir_all(test_dir)?;
        std::fs::create_dir_all(diff_dir)?;

        Ok(Self {
            reference_dir: reference_dir.to_string(),
            test_dir: test_dir.to_string(),
            diff_dir: diff_dir.to_string(),
        })
    }

    /// Run a visual test comparison
    pub fn compare_screenshots(&self, test_name: &str) -> Result<VisualComparison> {
        let reference_path = Path::new(&self.reference_dir).join(format!("{}.png", test_name));
        let test_path = Path::new(&self.test_dir).join(format!("{}.png", test_name));

        if !reference_path.exists() {
            return Err(DevToolsError::VisualTesting(
                format!("Reference screenshot not found: {:?}", reference_path)
            ));
        }

        if !test_path.exists() {
            return Err(DevToolsError::VisualTesting(
                format!("Test screenshot not found: {:?}", test_path)
            ));
        }

        let reference_img = image::open(&reference_path)
            .map_err(|e| DevToolsError::Image(format!("Failed to open reference image: {}", e)))?;

        let test_img = image::open(&test_path)
            .map_err(|e| DevToolsError::Image(format!("Failed to open test image: {}", e)))?;

        let comparison = VisualComparison::compare_images(&reference_img, &test_img)?;

        // Save diff image if there are differences
        if comparison.pixel_difference > 0 {
            if let Some(diff_img) = &comparison.diff_image {
                let diff_path = Path::new(&self.diff_dir).join(format!("{}_diff.png", test_name));
                diff_img.save(&diff_path)
                    .map_err(|e| DevToolsError::Image(format!("Failed to save diff image: {}", e)))?;
                log::info!("Diff image saved to: {:?}", diff_path);
            }
        }

        Ok(comparison)
    }

    /// Update reference screenshots
    pub fn update_references(&self, test_name: &str) -> Result<()> {
        let test_path = Path::new(&self.test_dir).join(format!("{}.png", test_name));
        let reference_path = Path::new(&self.reference_dir).join(format!("{}.png", test_name));

        if test_path.exists() {
            std::fs::copy(&test_path, &reference_path)
                .map_err(|e| DevToolsError::Io(e))?;
            log::info!("Updated reference screenshot: {:?}", reference_path);
            Ok(())
        } else {
            Err(DevToolsError::VisualTesting(
                format!("Test screenshot not found: {:?}", test_path)
            ))
        }
    }

    /// Check color accuracy in a screenshot
    pub fn analyze_colors(&self, screenshot_path: &str) -> Result<ColorAnalysis> {
        let img = image::open(screenshot_path)
            .map_err(|e| DevToolsError::Image(format!("Failed to open screenshot: {}", e)))?;

        let mut color_counts = std::collections::HashMap::new();
        let mut total_pixels = 0u64;
        let mut vibrant_pixels = 0u64;

        for (_, _, pixel) in img.pixels() {
            total_pixels += 1;
            *color_counts.entry(pixel).or_insert(0u64) += 1;

            // Consider a pixel vibrant if it has high saturation
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;

            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            let saturation = if max > 0.0 { (max - min) / max } else { 0.0 };

            if saturation > 0.3 && max > 0.2 { // Thresholds for vibrant colors
                vibrant_pixels += 1;
            }
        }

        let vibrant_percentage = (vibrant_pixels as f64 / total_pixels as f64) * 100.0;
        let unique_colors = color_counts.len();

        Ok(ColorAnalysis {
            total_pixels,
            vibrant_pixels,
            vibrant_percentage,
            unique_colors,
        })
    }
}

/// Analysis of colors in an image
#[derive(Debug)]
pub struct ColorAnalysis {
    pub total_pixels: u64,
    pub vibrant_pixels: u64,
    pub vibrant_percentage: f64,
    pub unique_colors: usize,
}

impl ColorAnalysis {
    /// Check if the image has sufficient color variety
    pub fn has_good_color_variety(&self) -> bool {
        self.vibrant_percentage > 20.0 && self.unique_colors > 100
    }

    pub fn summary(&self) -> String {
        format!(
            "Color Analysis:\n\
             - Total pixels: {}\n\
             - Vibrant pixels: {} ({:.1}%)\n\
             - Unique colors: {}\n\
             - Color variety: {}",
            self.total_pixels,
            self.vibrant_pixels,
            self.vibrant_percentage,
            self.unique_colors,
            if self.has_good_color_variety() { "✅ Good" } else { "❌ Poor" }
        )
    }
}
