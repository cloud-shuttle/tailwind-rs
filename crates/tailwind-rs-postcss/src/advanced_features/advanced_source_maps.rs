//! Advanced Source Maps
//!
//! This module provides comprehensive source map generation with
//! detailed mappings, transformation tracking, and source resolution.

use super::types::*;

/// Advanced source map generator
pub struct AdvancedSourceMapGenerator {
    mappings: Vec<SourceMapping>,
    sources: Vec<SourceFile>,
    names: Vec<String>,
    transformations: Vec<Transformation>,
}

impl AdvancedSourceMapGenerator {
    /// Create new advanced source map generator
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
            sources: Vec::new(),
            names: Vec::new(),
            transformations: Vec::new(),
        }
    }

    /// Generate comprehensive source map
    pub fn generate_source_map(
        &mut self,
        css: &str,
        source_files: &[SourceFile],
    ) -> Result<SourceMap, AdvancedFeatureError> {
        // Clear previous mappings
        self.mappings.clear();
        self.sources.clear();
        self.names.clear();

        // Add source files
        for source_file in source_files {
            self.sources.push(source_file.clone());
        }

        // Generate mappings for CSS
        self.generate_css_mappings(css)?;

        // Generate source map
        let source_map = SourceMap {
            version: 3,
            file: "output.css".to_string(),
            source_root: String::new(),
            sources: self.sources.iter().map(|s| s.path.clone()).collect(),
            names: self.names.clone(),
            mappings: self.encode_mappings(&self.mappings)?,
            sources_content: self.sources.iter().map(|s| s.content.clone()).collect(),
        };

        Ok(source_map)
    }

    /// Add transformation mapping
    pub fn add_transformation(
        &mut self,
        transformation: Transformation,
    ) -> Result<(), AdvancedFeatureError> {
        let transformation_clone = transformation.clone();
        self.transformations.push(transformation);

        // Generate mappings for transformation
        self.generate_transformation_mappings(transformation_clone)?;

        Ok(())
    }

    /// Resolve source location
    pub fn resolve_source_location(
        &self,
        line: usize,
        column: usize,
    ) -> Result<SourceLocation, AdvancedFeatureError> {
        // Find mapping for position
        for mapping in &self.mappings {
            if mapping.generated_line == line && mapping.generated_column == column {
                let source_file = if let Some(source_index) = mapping.source_file {
                    if source_index < self.sources.len() {
                        &self.sources[source_index]
                    } else {
                        return Err(AdvancedFeatureError::SourceMapFailed {
                            error: "Invalid source file index".to_string(),
                        });
                    }
                } else {
                    return Err(AdvancedFeatureError::SourceMapFailed {
                        error: "No source file for mapping".to_string(),
                    });
                };

                return Ok(SourceLocation {
                    file: source_file.path.clone(),
                    line: mapping.source_line,
                    column: mapping.source_column,
                    content: self.get_source_content(source_file, mapping.source_line)?,
                });
            }
        }

        Err(AdvancedFeatureError::SourceMapFailed {
            error: "No mapping found for position".to_string(),
        })
    }

    /// Generate CSS mappings
    fn generate_css_mappings(&mut self, css: &str) -> Result<(), AdvancedFeatureError> {
        let lines: Vec<&str> = css.split('\n').collect();

        for (line_num, line) in lines.iter().enumerate() {
            for (col_num, _) in line.char_indices() {
                // Create 1:1 mapping for each character
                self.mappings.push(SourceMapping {
                    generated_line: line_num + 1,
                    generated_column: col_num,
                    source_line: line_num + 1,
                    source_column: col_num,
                    source_file: Some(0), // First source file
                    name: None,
                });
            }
        }

        Ok(())
    }

    /// Generate transformation mappings
    fn generate_transformation_mappings(
        &mut self,
        transformation: Transformation,
    ) -> Result<(), AdvancedFeatureError> {
        // Find source file index
        let source_index = self
            .sources
            .iter()
            .position(|s| s.path == transformation.source_file)
            .unwrap_or(0);

        // Generate mappings for transformation
        for line in transformation.input_range.start..transformation.output_range.end {
            self.mappings.push(SourceMapping {
                generated_line: line,
                generated_column: 0,
                source_line: line,
                source_column: 0,
                source_file: Some(source_index),
                name: Some(self.names.len()),
            });

            // Add transformation name
            self.names.push(transformation.name.clone());
        }

        Ok(())
    }

    /// Encode mappings using VLQ
    fn encode_mappings(&self, mappings: &[SourceMapping]) -> Result<String, AdvancedFeatureError> {
        let mut encoded = String::new();
        let mut previous_generated_line = 0;
        let mut previous_generated_column = 0;
        let mut previous_source_line = 0;
        let mut previous_source_column = 0;
        let mut previous_source_file = 0;
        let mut previous_name = 0;

        for mapping in mappings {
            // Calculate deltas
            let generated_line_delta = (mapping.generated_line - previous_generated_line) as isize;
            let generated_column_delta =
                (mapping.generated_column - previous_generated_column) as isize;
            let source_line_delta = (mapping.source_line - previous_source_line) as isize;
            let source_column_delta = (mapping.source_column - previous_source_column) as isize;
            let source_file_delta =
                (mapping.source_file.unwrap_or(0) - previous_source_file) as isize;
            let name_delta = (mapping.name.unwrap_or(0) - previous_name) as isize;

            // Encode deltas
            if !encoded.is_empty() {
                encoded.push(';');
            }

            encoded.push_str(&self.encode_vlq(generated_line_delta));
            encoded.push(',');
            encoded.push_str(&self.encode_vlq(generated_column_delta));
            encoded.push(',');
            encoded.push_str(&self.encode_vlq(source_file_delta));
            encoded.push(',');
            encoded.push_str(&self.encode_vlq(source_line_delta));
            encoded.push(',');
            encoded.push_str(&self.encode_vlq(source_column_delta));

            if mapping.name.is_some() {
                encoded.push(',');
                encoded.push_str(&self.encode_vlq(name_delta));
            }

            // Update previous values
            previous_generated_line = mapping.generated_line;
            previous_generated_column = mapping.generated_column;
            previous_source_line = mapping.source_line;
            previous_source_column = mapping.source_column;
            previous_source_file = mapping.source_file.unwrap_or(0);
            previous_name = mapping.name.unwrap_or(0);
        }

        Ok(encoded)
    }

    /// Encode number using VLQ
    fn encode_vlq(&self, mut num: isize) -> String {
        let mut result = String::new();

        if num < 0 {
            num = (-num << 1) | 1;
        } else {
            num <<= 1;
        }

        loop {
            let mut digit = (num & 0x1f) as u8;
            num >>= 5;

            if num > 0 {
                digit |= 0x20;
            }

            result.push(self.vlq_to_char(digit));

            if num == 0 {
                break;
            }
        }

        result
    }

    /// Convert VLQ digit to character
    fn vlq_to_char(&self, digit: u8) -> char {
        const VLQ_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        VLQ_CHARS.chars().nth(digit as usize).unwrap_or('A')
    }

    /// Get source content for line
    fn get_source_content(
        &self,
        source_file: &SourceFile,
        line: usize,
    ) -> Result<String, AdvancedFeatureError> {
        let lines: Vec<&str> = source_file.content.split('\n').collect();

        if line > 0 && line <= lines.len() {
            Ok(lines[line - 1].to_string())
        } else {
            Ok(String::new())
        }
    }
}

/// Source map structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SourceMap {
    pub version: u32,
    pub file: String,
    pub source_root: String,
    pub sources: Vec<String>,
    pub names: Vec<String>,
    pub mappings: String,
    pub sources_content: Vec<String>,
}

impl SourceMap {
    /// Create new source map
    pub fn new() -> Self {
        Self {
            version: 3,
            file: String::new(),
            source_root: String::new(),
            sources: Vec::new(),
            names: Vec::new(),
            mappings: String::new(),
            sources_content: Vec::new(),
        }
    }

    /// Get source map as JSON
    pub fn to_json(&self) -> Result<String, AdvancedFeatureError> {
        serde_json::to_string_pretty(self).map_err(|e| AdvancedFeatureError::SourceMapFailed {
            error: format!("Failed to serialize source map: {}", e),
        })
    }

    /// Load source map from JSON
    pub fn from_json(json: &str) -> Result<Self, AdvancedFeatureError> {
        serde_json::from_str(json).map_err(|e| AdvancedFeatureError::SourceMapFailed {
            error: format!("Failed to parse source map: {}", e),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_map_generation() {
        let mut generator = AdvancedSourceMapGenerator::new();
        let css = ".test { color: red; }";
        let source_files = vec![SourceFile {
            path: "input.css".to_string(),
            content: css.to_string(),
            mtime: std::time::SystemTime::now(),
            size: css.len(),
        }];

        let result = generator.generate_source_map(css, &source_files);
        assert!(result.is_ok());

        let source_map = result.unwrap();
        assert_eq!(source_map.version, 3);
        assert!(!source_map.sources.is_empty());
        assert!(!source_map.mappings.is_empty());
    }

    #[test]
    fn test_transformation_mapping() {
        let mut generator = AdvancedSourceMapGenerator::new();
        let transformation = Transformation {
            name: "autoprefixer".to_string(),
            input_range: TextRange::new(0, 10),
            output_range: TextRange::new(0, 15),
            source_file: "input.css".to_string(),
        };

        let result = generator.add_transformation(transformation);
        assert!(result.is_ok());
    }

    #[test]
    fn test_source_location_resolution() {
        let mut generator = AdvancedSourceMapGenerator::new();
        let css = ".test { color: red; }";
        let source_files = vec![SourceFile {
            path: "input.css".to_string(),
            content: css.to_string(),
            mtime: std::time::SystemTime::now(),
            size: css.len(),
        }];

        generator.generate_source_map(css, &source_files).unwrap();

        let result = generator.resolve_source_location(1, 0);
        assert!(result.is_ok());

        let location = result.unwrap();
        assert_eq!(location.file, "input.css");
        assert_eq!(location.line, 1);
    }

    #[test]
    fn test_vlq_encoding() {
        let generator = AdvancedSourceMapGenerator::new();
        let encoded = generator.encode_vlq(123);
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_source_map_json() {
        let source_map = SourceMap::new();
        let json = source_map.to_json();
        assert!(json.is_ok());

        let json_str = json.unwrap();
        let parsed = SourceMap::from_json(&json_str);
        assert!(parsed.is_ok());
    }
}
