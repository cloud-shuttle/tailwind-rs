# CSS Generator Architecture Design

## Overview
**Purpose**: Refactor the monolithic `css_generator.rs` into a modular, maintainable architecture  
**Target**: Reduce file size from ~3000 lines to <300 lines per module  
**Priority**: High

## Current Architecture Issues

### Problems
- **Monolithic File**: Single file with ~3000 lines
- **Multiple Responsibilities**: Parsing, generation, optimization in one file
- **Maintenance Difficulty**: Hard to navigate and modify
- **LLM Processing**: Difficult for AI tools to process effectively

### Impact
- Code maintainability
- Development velocity
- Code review efficiency
- Testing and debugging

## Proposed Architecture

### 1. Core Structure
```
css_generator/
├── lib.rs                 # Main interface and exports
├── core.rs                # Core generation logic
├── variants.rs            # Variant parsing and handling
├── utils.rs               # Utility functions
└── parsers/
    ├── mod.rs             # Parser module exports
    ├── spacing.rs         # Spacing utilities (padding, margin, gap)
    ├── color.rs           # Color utilities (text, bg, border)
    ├── typography.rs      # Typography utilities (font, text)
    ├── layout.rs          # Layout utilities (display, position)
    ├── flexbox.rs         # Flexbox utilities
    ├── grid.rs            # Grid utilities
    ├── borders.rs         # Border utilities
    ├── effects.rs         # Effects utilities (shadow, opacity)
    ├── transforms.rs      # Transform utilities
    └── animations.rs      # Animation utilities
```

### 2. Module Responsibilities

#### `lib.rs` - Main Interface
```rust
//! CSS Generator - Main Interface
//! 
//! This module provides the main CSS generation interface,
//! orchestrating the various parser modules to generate CSS.

pub mod core;
pub mod variants;
pub mod utils;
pub mod parsers;

pub use core::CssGenerator;
pub use variants::VariantParser;
pub use parsers::*;
```

#### `core.rs` - Core Generation Logic
```rust
//! Core CSS Generation Logic
//! 
//! This module contains the main CSS generation logic,
//! orchestrating parsers and generating final CSS output.

use crate::parsers::*;
use crate::variants::VariantParser;

pub struct CssGenerator {
    // Core generation state
}

impl CssGenerator {
    // Main generation methods
}
```

#### `variants.rs` - Variant Parsing
```rust
//! Variant Parsing and Handling
//! 
//! This module handles the parsing and processing of CSS variants,
//! including responsive, state, and custom variants.

pub struct VariantParser {
    // Variant parsing state
}

impl VariantParser {
    // Variant parsing methods
}
```

#### `parsers/spacing.rs` - Spacing Utilities
```rust
//! Spacing Utilities Parser
//! 
//! This module handles parsing of spacing-related utilities:
//! - Padding (p-*, px-*, py-*, etc.)
//! - Margin (m-*, mx-*, my-*, etc.)
//! - Gap (gap-*, gap-x-*, gap-y-*, etc.)

pub struct SpacingParser;

impl SpacingParser {
    pub fn parse_padding_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    pub fn parse_margin_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    pub fn parse_gap_class(&self, class: &str) -> Option<Vec<CssProperty>>;
}
```

### 3. Parser Module Structure

#### Common Parser Interface
```rust
//! Common Parser Interface
//! 
//! This module defines the common interface for all utility parsers.

pub trait UtilityParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>>;
    fn get_supported_patterns(&self) -> Vec<&'static str>;
    fn get_priority(&self) -> u32;
}
```

#### Parser Implementation Pattern
```rust
//! Example Parser Implementation
//! 
//! This shows the pattern for implementing utility parsers.

pub struct ExampleParser;

impl UtilityParser for ExampleParser {
    fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Parser implementation
    }
    
    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec!["example-*", "ex-*"]
    }
    
    fn get_priority(&self) -> u32 {
        100
    }
}
```

## Implementation Plan

### Phase 1: Core Refactoring
1. **Create Module Structure**
   - Create `css_generator/` directory
   - Create `parsers/` subdirectory
   - Set up module files

2. **Extract Core Logic**
   - Move main generation logic to `core.rs`
   - Move variant parsing to `variants.rs`
   - Move utilities to `utils.rs`

### Phase 2: Parser Extraction
1. **Spacing Parser** (Day 1)
   - Extract spacing-related parsing
   - Create `parsers/spacing.rs`
   - Update imports and exports

2. **Color Parser** (Day 1)
   - Extract color-related parsing
   - Create `parsers/color.rs`
   - Update imports and exports

3. **Typography Parser** (Day 2)
   - Extract typography-related parsing
   - Create `parsers/typography.rs`
   - Update imports and exports

4. **Layout Parser** (Day 2)
   - Extract layout-related parsing
   - Create `parsers/layout.rs`
   - Update imports and exports

5. **Remaining Parsers** (Day 3)
   - Extract remaining utility parsers
   - Create remaining parser modules
   - Update imports and exports

### Phase 3: Testing and Validation
1. **Update Tests**
   - Split tests across new modules
   - Ensure all tests pass
   - Update test documentation

2. **API Compatibility**
   - Maintain existing public APIs
   - Update internal interfaces
   - Document changes

## Success Criteria
- [ ] All files under 300 lines
- [ ] All tests passing
- [ ] API compatibility maintained
- [ ] Performance maintained
- [ ] Documentation updated

## Risk Mitigation
- **API Compatibility**: Maintain existing public APIs
- **Test Coverage**: Ensure all tests pass after refactoring
- **Performance**: Monitor performance impact
- **Documentation**: Update all documentation

## Dependencies
- No external dependencies
- Internal refactoring only
- Maintains backward compatibility
