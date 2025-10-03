//! Enhanced State Machine Architecture
//!
//! Specialized state machines for complex parsing scenarios,
//! inspired by the official Tailwind Oxide implementation.

use crate::css_generator::types::CssProperty;
use crate::cursor::Cursor;

/// Result of a state machine operation
#[derive(Debug, Clone, PartialEq)]
pub enum MachineResult<T> {
    /// Continue processing
    Continue,
    /// Successfully completed with result
    Complete(T),
    /// Error occurred during processing
    Error(String),
}

/// Core state machine trait
pub trait ParsingMachine: Default {
    type Output;

    /// Reset the machine to initial state
    fn reset(&mut self);

    /// Process next character and return result
    fn next(&mut self, cursor: &mut Cursor) -> MachineResult<Self::Output>;

    /// Check if machine is in a complete state
    fn is_complete(&self) -> bool;

    /// Get the result if complete
    fn get_result(&self) -> Option<Self::Output>;
}

/// Gradient parsing state machine
#[derive(Debug, Clone)]
pub struct GradientMachine {
    state: GradientParseState,
    direction: Option<String>,
    stops: Vec<GradientStop>,
    current_stop: Option<PartialStop>,
}

#[derive(Debug, Clone)]
enum GradientParseState {
    ExpectingDirection,
    ExpectingStops,
    ParsingStop,
    Complete,
}

#[derive(Debug, Clone)]
pub struct GradientStop {
    pub color: String,
    pub position: Option<String>,
}

#[derive(Debug, Clone)]
struct PartialStop {
    color_part: String,
    position_part: Option<String>,
}

impl Default for GradientMachine {
    fn default() -> Self {
        Self {
            state: GradientParseState::ExpectingDirection,
            direction: None,
            stops: Vec::new(),
            current_stop: None,
        }
    }
}

impl ParsingMachine for GradientMachine {
    type Output = GradientDefinition;

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn next(&mut self, cursor: &mut Cursor) -> MachineResult<Self::Output> {
        match self.state {
            GradientParseState::ExpectingDirection => {
                // Look for gradient direction (to-r, to-l, etc.)
                if let Some(direction) = self.extract_direction(cursor) {
                    self.direction = Some(direction);
                    self.state = GradientParseState::ExpectingStops;
                    MachineResult::Continue
                } else {
                    MachineResult::Error("Invalid gradient direction".to_string())
                }
            }
            GradientParseState::ExpectingStops => {
                // Look for gradient stops (from-, via-, to-)
                if let Some(stop) = self.extract_stop(cursor) {
                    self.stops.push(stop);
                    self.state = GradientParseState::ParsingStop;
                    MachineResult::Continue
                } else {
                    MachineResult::Error("Expected gradient stop".to_string())
                }
            }
            GradientParseState::ParsingStop => {
                // Continue parsing additional stops or complete
                if let Some(stop) = self.extract_stop(cursor) {
                    self.stops.push(stop);
                    MachineResult::Continue
                } else if self.is_complete() {
                    self.state = GradientParseState::Complete;
                    MachineResult::Complete(self.get_result().unwrap())
                } else {
                    MachineResult::Continue
                }
            }
            GradientParseState::Complete => {
                MachineResult::Error("Machine already complete".to_string())
            }
        }
    }

    fn is_complete(&self) -> bool {
        matches!(self.state, GradientParseState::Complete)
    }

    fn get_result(&self) -> Option<Self::Output> {
        if self.is_complete() && !self.stops.is_empty() {
            Some(GradientDefinition {
                direction: self.direction.clone()?,
                stops: self.stops.clone(),
            })
        } else {
            None
        }
    }
}

impl GradientMachine {
    fn extract_direction(&self, cursor: &mut Cursor) -> Option<String> {
        // Look for patterns like "bg-gradient-to-r", "bg-gradient-to-br", etc.
        let input = std::str::from_utf8(cursor.remaining()).ok()?;

        if let Some(start) = input.find("bg-gradient-to-") {
            let after_prefix = &input[start + 15..]; // Skip "bg-gradient-to-"
            if let Some(end) = after_prefix.find(' ') {
                Some(format!("to-{}", &after_prefix[..end]))
            } else if let Some(end) = after_prefix.find('\n') {
                Some(format!("to-{}", &after_prefix[..end]))
            } else {
                Some(format!("to-{}", after_prefix))
            }
        } else {
            None
        }
    }

    fn extract_stop(&self, cursor: &mut Cursor) -> Option<GradientStop> {
        let input = std::str::from_utf8(cursor.remaining()).ok()?;

        // Look for from-, via-, to- patterns
        let patterns = ["from-", "via-", "to-"];

        for &pattern in &patterns {
            if let Some(start) = input.find(pattern) {
                let after_pattern = &input[start + pattern.len()..];
                if let Some(end) = after_pattern.find(' ') {
                    let color = after_pattern[..end].to_string();
                    return Some(GradientStop {
                        color,
                        position: None,
                    });
                }
            }
        }

        None
    }
}

/// Arbitrary value parsing state machine
#[derive(Debug, Clone)]
pub struct ArbitraryValueMachine {
    state: ArbitraryParseState,
    bracket_depth: usize,
    content: String,
    value_type: ArbitraryValueType,
}

#[derive(Debug, Clone)]
enum ArbitraryParseState {
    ExpectingOpenBracket,
    ParsingContent,
    Validating,
    Complete,
}

#[derive(Debug, Clone)]
pub enum ArbitraryValueType {
    Color,
    Length,
    Percentage,
    Url,
    Unknown,
}

impl Default for ArbitraryValueMachine {
    fn default() -> Self {
        Self {
            state: ArbitraryParseState::ExpectingOpenBracket,
            bracket_depth: 0,
            content: String::new(),
            value_type: ArbitraryValueType::Unknown,
        }
    }
}

impl ParsingMachine for ArbitraryValueMachine {
    type Output = ArbitraryValue;

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn next(&mut self, cursor: &mut Cursor) -> MachineResult<Self::Output> {
        match self.state {
            ArbitraryParseState::ExpectingOpenBracket => {
                if cursor.curr == b'[' {
                    self.state = ArbitraryParseState::ParsingContent;
                    cursor.advance();
                    MachineResult::Continue
                } else {
                    MachineResult::Error("Expected '['".to_string())
                }
            }
            ArbitraryParseState::ParsingContent => {
                if cursor.curr == b'[' {
                    self.bracket_depth += 1;
                } else if cursor.curr == b']' {
                    if self.bracket_depth == 0 {
                        self.state = ArbitraryParseState::Validating;
                        cursor.advance();
                        return MachineResult::Continue;
                    } else {
                        self.bracket_depth -= 1;
                    }
                }

                if cursor.curr != b']' {
                    self.content.push(cursor.curr as char);
                }

                cursor.advance();
                MachineResult::Continue
            }
            ArbitraryParseState::Validating => {
                if self.validate_content() {
                    self.state = ArbitraryParseState::Complete;
                    MachineResult::Complete(self.get_result().unwrap())
                } else {
                    MachineResult::Error("Invalid arbitrary value content".to_string())
                }
            }
            ArbitraryParseState::Complete => {
                MachineResult::Error("Machine already complete".to_string())
            }
        }
    }

    fn is_complete(&self) -> bool {
        matches!(self.state, ArbitraryParseState::Complete)
    }

    fn get_result(&self) -> Option<Self::Output> {
        if self.is_complete() {
            Some(ArbitraryValue {
                content: self.content.clone(),
                value_type: self.value_type.clone(),
            })
        } else {
            None
        }
    }
}

impl ArbitraryValueMachine {
    fn validate_content(&mut self) -> bool {
        // Determine value type based on content
        if self.content.starts_with('#') && (self.content.len() == 4 || self.content.len() == 7) {
            self.value_type = ArbitraryValueType::Color;
        } else if self.content.ends_with("px") || self.content.ends_with("rem") || self.content.ends_with("em") {
            self.value_type = ArbitraryValueType::Length;
        } else if self.content.ends_with('%') {
            self.value_type = ArbitraryValueType::Percentage;
        } else if self.content.starts_with("url(") {
            self.value_type = ArbitraryValueType::Url;
        } else {
            self.value_type = ArbitraryValueType::Unknown;
        }

        // Basic validation - content should not be empty and should not contain invalid characters
        !self.content.is_empty() && !self.content.contains('\n') && !self.content.contains('\t')
    }
}

/// Variant combination parsing state machine
#[derive(Debug, Clone)]
pub struct VariantCombinationMachine {
    variants: Vec<ParsedVariant>,
    current_variant: Option<String>,
    state: VariantParseState,
}

#[derive(Debug, Clone)]
enum VariantParseState {
    ExpectingVariant,
    ParsingVariant,
    Complete,
}

#[derive(Debug, Clone)]
pub struct ParsedVariant {
    pub name: String,
    pub kind: VariantKind,
}

#[derive(Debug, Clone)]
pub enum VariantKind {
    Responsive,
    State,
    DarkMode,
    Custom,
}

impl Default for VariantCombinationMachine {
    fn default() -> Self {
        Self {
            variants: Vec::new(),
            current_variant: None,
            state: VariantParseState::ExpectingVariant,
        }
    }
}

impl ParsingMachine for VariantCombinationMachine {
    type Output = VariantCombination;

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn next(&mut self, cursor: &mut Cursor) -> MachineResult<Self::Output> {
        match self.state {
            VariantParseState::ExpectingVariant => {
                if cursor.curr == b':' {
                    self.state = VariantParseState::ParsingVariant;
                    cursor.advance();
                    MachineResult::Continue
                } else {
                    // No more variants, complete
                    self.state = VariantParseState::Complete;
                    MachineResult::Complete(self.get_result().unwrap())
                }
            }
            VariantParseState::ParsingVariant => {
                let start_pos = cursor.pos;

                // Find the next colon or end of class
                while cursor.pos < cursor.input.len() && cursor.curr != b':' {
                    cursor.advance();
                }

                let variant_str = std::str::from_utf8(&cursor.input[start_pos..cursor.pos])
                    .unwrap_or("")
                    .to_string();

                if !variant_str.is_empty() {
                    if let Some(variant) = self.parse_variant(&variant_str) {
                        self.variants.push(variant);
                    }
                }

                if cursor.pos < cursor.input.len() && cursor.curr == b':' {
                    self.state = VariantParseState::ExpectingVariant;
                    cursor.advance();
                    MachineResult::Continue
                } else {
                    self.state = VariantParseState::Complete;
                    MachineResult::Complete(self.get_result().unwrap())
                }
            }
            VariantParseState::Complete => {
                MachineResult::Error("Machine already complete".to_string())
            }
        }
    }

    fn is_complete(&self) -> bool {
        matches!(self.state, VariantParseState::Complete)
    }

    fn get_result(&self) -> Option<Self::Output> {
        if self.is_complete() {
            Some(VariantCombination {
                variants: self.variants.clone(),
            })
        } else {
            None
        }
    }
}

impl VariantCombinationMachine {
    fn parse_variant(&self, variant_str: &str) -> Option<ParsedVariant> {
        let kind = if variant_str.starts_with("sm:") || variant_str.starts_with("md:") || variant_str.starts_with("lg:") {
            VariantKind::Responsive
        } else if variant_str.starts_with("hover:") || variant_str.starts_with("focus:") {
            VariantKind::State
        } else if variant_str == "dark" {
            VariantKind::DarkMode
        } else {
            VariantKind::Custom
        };

        Some(ParsedVariant {
            name: variant_str.to_string(),
            kind,
        })
    }
}

/// Output types
#[derive(Debug, Clone)]
pub struct GradientDefinition {
    pub direction: String,
    pub stops: Vec<GradientStop>,
}

#[derive(Debug, Clone)]
pub struct ArbitraryValue {
    pub content: String,
    pub value_type: ArbitraryValueType,
}

#[derive(Debug, Clone)]
pub struct VariantCombination {
    pub variants: Vec<ParsedVariant>,
}

/// State machine registry for managing different machine types
#[derive(Debug)]
pub struct StateMachineRegistry {
    gradient_machine: GradientMachine,
    arbitrary_machine: ArbitraryValueMachine,
    variant_machine: VariantCombinationMachine,
}

impl Default for StateMachineRegistry {
    fn default() -> Self {
        Self {
            gradient_machine: GradientMachine::default(),
            arbitrary_machine: ArbitraryValueMachine::default(),
            variant_machine: VariantCombinationMachine::default(),
        }
    }
}

impl StateMachineRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_gradient(&mut self, input: &str) -> Result<GradientDefinition, String> {
        let mut cursor = Cursor::new(input.as_bytes());
        self.gradient_machine.reset();

        loop {
            match self.gradient_machine.next(&mut cursor) {
                MachineResult::Continue => continue,
                MachineResult::Complete(result) => return Ok(result),
                MachineResult::Error(err) => return Err(err),
            }
        }
    }

    pub fn process_arbitrary_value(&mut self, input: &str) -> Result<ArbitraryValue, String> {
        let mut cursor = Cursor::new(input.as_bytes());
        self.arbitrary_machine.reset();

        loop {
            match self.arbitrary_machine.next(&mut cursor) {
                MachineResult::Continue => continue,
                MachineResult::Complete(result) => return Ok(result),
                MachineResult::Error(err) => return Err(err),
            }
        }
    }

    pub fn process_variant_combination(&mut self, input: &str) -> Result<VariantCombination, String> {
        let mut cursor = Cursor::new(input.as_bytes());
        self.variant_machine.reset();

        loop {
            match self.variant_machine.next(&mut cursor) {
                MachineResult::Continue => continue,
                MachineResult::Complete(result) => return Ok(result),
                MachineResult::Error(err) => return Err(err),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_machine() {
        let mut registry = StateMachineRegistry::new();

        // Test basic gradient
        let result = registry.process_gradient("bg-gradient-to-r from-blue-500 to-red-500");
        assert!(result.is_ok());

        let gradient = result.unwrap();
        assert_eq!(gradient.direction, "to-r");
        assert_eq!(gradient.stops.len(), 2);
    }

    #[test]
    fn test_arbitrary_value_machine() {
        let mut registry = StateMachineRegistry::new();

        // Test color arbitrary value
        let result = registry.process_arbitrary_value("[#ff0000]");
        assert!(result.is_ok());

        let arbitrary = result.unwrap();
        assert_eq!(arbitrary.content, "#ff0000");
        matches!(arbitrary.value_type, ArbitraryValueType::Color);
    }

    #[test]
    fn test_variant_combination_machine() {
        let mut registry = StateMachineRegistry::new();

        // Test variant combination
        let result = registry.process_variant_combination("hover:focus:bg-blue-500");
        assert!(result.is_ok());

        let combination = result.unwrap();
        assert_eq!(combination.variants.len(), 2);
        assert_eq!(combination.variants[0].name, "hover");
        assert_eq!(combination.variants[1].name, "focus");
    }
}
