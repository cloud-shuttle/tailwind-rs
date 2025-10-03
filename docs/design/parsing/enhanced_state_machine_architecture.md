# Enhanced State Machine Architecture Design

## Overview

Implement specialized state machines for complex parsing scenarios, following the pattern used in the official Tailwind Oxide implementation with dedicated machines for candidates, utilities, variants, and arbitrary values.

## Problem Statement

Current parsing uses individual parser functions that don't maintain state across related classes. Complex scenarios like gradient parsing and arbitrary values require stateful parsing that our current architecture doesn't support efficiently.

## Solution Architecture

### Core Components

#### 1. State Machine Trait

```rust
pub trait ParsingMachine: Default {
    type Output;

    fn reset(&mut self);
    fn next(&mut self, cursor: &mut Cursor) -> MachineState<Self::Output>;
    fn is_complete(&self) -> bool;
    fn get_result(&self) -> Option<Self::Output>;
}

#[derive(Debug)]
pub enum MachineState<T> {
    Continue,           // Continue processing
    Complete(T),        // Successfully completed with result
    Error(String),      // Error occurred
    Incomplete,         // Needs more input
}
```

#### 2. Specialized Machines

##### Gradient State Machine
```rust
pub struct GradientMachine {
    state: GradientParseState,
    direction: Option<String>,
    stops: Vec<GradientStop>,
    current_stop: Option<PartialStop>,
}

#[derive(Debug)]
enum GradientParseState {
    ExpectingDirection,
    ExpectingStops,
    ExpectingFrom,
    ExpectingVia,
    ExpectingTo,
    Complete,
}

impl ParsingMachine for GradientMachine {
    type Output = GradientDefinition;

    fn next(&mut self, cursor: &mut Cursor) -> MachineState<Self::Output> {
        match self.state {
            GradientParseState::ExpectingDirection => {
                if let Some(direction) = self.parse_direction(cursor) {
                    self.direction = Some(direction);
                    self.state = GradientParseState::ExpectingStops;
                    MachineState::Continue
                } else {
                    MachineState::Error("Invalid gradient direction".to_string())
                }
            }
            // ... other states
        }
    }
}
```

##### Arbitrary Value Machine
```rust
pub struct ArbitraryValueMachine {
    state: ArbitraryParseState,
    bracket_depth: usize,
    content: String,
    value_type: ArbitraryValueType,
}

#[derive(Debug)]
enum ArbitraryParseState {
    ExpectingOpenBracket,
    ParsingContent,
    ExpectingCloseBracket,
    Validating,
    Complete,
}

#[derive(Debug)]
enum ArbitraryValueType {
    Color,
    Length,
    Percentage,
    Url,
    Unknown,
}
```

##### Variant Combination Machine
```rust
pub struct VariantCombinationMachine {
    variants: Vec<ParsedVariant>,
    current_variant: Option<PartialVariant>,
    state: VariantParseState,
}

impl VariantCombinationMachine {
    pub fn add_class(&mut self, class: &str) -> Result<(), String> {
        // Parse variants from class name
        // Handle complex combinations like hover:focus:dark:bg-blue-500
        // Maintain state across multiple related classes
    }

    pub fn get_combined_selector(&self) -> String {
        // Generate CSS selector from variant combination
    }
}
```

### Integration Points

#### 1. Parser Registry Enhancement

```rust
pub struct ParserRegistry {
    simple_parsers: HashMap<String, Box<dyn SimpleParser>>,
    state_machines: HashMap<String, Box<dyn ParsingMachine>>,
}

impl ParserRegistry {
    pub fn register_machine<T: ParsingMachine + 'static>(
        &mut self,
        prefix: &str,
        machine: T,
    ) {
        self.state_machines.insert(prefix.to_string(), Box::new(machine));
    }

    pub fn parse_with_machine(&self, input: &str) -> Option<CssOutput> {
        for (prefix, machine) in &self.state_machines {
            if input.starts_with(prefix) {
                let mut cursor = Cursor::new(input.as_bytes());
                let mut machine = machine.clone_box();

                while let MachineState::Continue = machine.next(&mut cursor) {
                    // Continue processing
                }

                if let Some(result) = machine.get_result() {
                    return Some(self.convert_to_css(result));
                }
            }
        }
        None
    }
}
```

#### 2. Context-Aware Parsing

```rust
pub struct ParseContext {
    machines: HashMap<String, Box<dyn ParsingMachine>>,
    shared_state: SharedParseState,
}

pub struct SharedParseState {
    pub gradient_context: Option<GradientContext>,
    pub variant_stack: Vec<ParsedVariant>,
    pub arbitrary_values: HashMap<String, ArbitraryValue>,
}

impl ParseContext {
    pub fn process_class(&mut self, class: &str) -> Vec<CssRule> {
        // Try state machines first
        if let Some(result) = self.try_state_machines(class) {
            return result;
        }

        // Fall back to simple parsers
        self.fallback_to_simple_parsers(class)
    }

    pub fn maintain_state(&mut self, class: &str) {
        // Update shared state based on class processing
        // Maintain gradient stops, variant combinations, etc.
    }
}
```

### Complex Parsing Scenarios

#### 1. Gradient Parsing with State
```rust
// Before: Stateless parsing
bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500

// After: Stateful parsing maintains context
let mut context = ParseContext::new();
context.process_class("bg-gradient-to-r");  // Sets direction
context.process_class("from-blue-500");     // Adds stop
context.process_class("via-purple-500");    // Adds stop
context.process_class("to-pink-500");       // Adds stop
// Result: Single gradient rule with all stops
```

#### 2. Complex Variant Combinations
```rust
// Complex variant parsing with state
hover:focus:dark:bg-blue-500/50

// Machine maintains state through each variant
- hover (state: active)
- focus (state: active + focus)
- dark (state: active + focus + dark)
- bg-blue-500/50 (applied with all variants)
```

### Implementation Plan

#### Phase 1: Core State Machine Infrastructure
- [ ] Define `ParsingMachine` trait and `MachineState` enum
- [ ] Create basic state machine utilities
- [ ] Add state machine registration system

#### Phase 2: Gradient State Machine
- [ ] Implement `GradientMachine` for complex gradients
- [ ] Add gradient context management
- [ ] Integrate with existing gradient parsers

#### Phase 3: Arbitrary Value Machine
- [ ] Implement `ArbitraryValueMachine` for complex arbitrary values
- [ ] Add validation and type inference
- [ ] Integrate with arbitrary value parsing

#### Phase 4: Variant Combination Machine
- [ ] Implement complex variant combination parsing
- [ ] Add selector generation for complex variants
- [ ] Integrate with variant system

#### Phase 5: Integration & Testing
- [ ] Update parser registry to support state machines
- [ ] Add comprehensive tests for stateful parsing
- [ ] Performance benchmarking and optimization

### Performance Characteristics

#### Memory Usage
- **State Machines**: Small per-instance memory footprint
- **Shared Context**: Minimal overhead for state sharing
- **Cleanup**: Automatic state cleanup after parsing completion

#### Processing Speed
- **Complex Scenarios**: Faster than stateless approaches
- **Simple Cases**: Minimal overhead compared to direct parsing
- **Caching**: State machine instances can be reused

### Compatibility Considerations

#### Backward Compatibility
- Existing simple parsers continue to work unchanged
- State machines are opt-in for complex scenarios
- Fallback to simple parsing when state machines fail

#### Error Handling
- Clear error messages for state machine failures
- Graceful degradation to simple parsing
- State recovery mechanisms for partial failures

### Risk Mitigation

#### Complexity Risk
- **Mitigation**: Clean abstractions and trait-based design
- **Documentation**: Extensive examples and state diagrams
- **Testing**: High coverage for state transitions

#### Performance Risk
- **Mitigation**: Benchmarking for all state machine operations
- **Optimization**: Lazy state machine instantiation
- **Monitoring**: Performance regression detection

#### Correctness Risk
- **Mitigation**: Formal state machine testing
- **Validation**: Cross-checking with simple parsers
- **Fuzzing**: Randomized input testing for edge cases

## Success Metrics

- **Functionality**: 100% coverage of complex parsing scenarios
- **Performance**: No regression for simple cases, speedup for complex cases
- **Maintainability**: Clear state machine abstractions
- **Reliability**: Robust error handling and state recovery
