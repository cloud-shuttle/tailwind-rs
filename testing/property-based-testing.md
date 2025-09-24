# Property-Based Testing and API Stability

This document describes the comprehensive property-based testing and API stability testing implemented in the tailwind-rs library.

## Overview

We have implemented two key testing enhancements:

1. **Property-Based Testing** - Using `proptest` to generate random valid inputs and test system invariants
2. **API Stability Testing** - Ensuring public APIs maintain backward compatibility and don't break between versions

## Property-Based Testing

### What is Property-Based Testing?

Property-based testing generates random valid inputs and tests that certain properties (invariants) hold true for all possible inputs. This helps catch edge cases and ensures our systems maintain their expected behavior under all conditions.

### Implementation

#### Core Library Property Tests (`tailwind-rs-core/src/property_tests.rs`)

**Class Generation Properties:**
- All input classes should be present in output
- Result should not be empty
- Result should be a valid CSS class string
- Class builder should be associative (order-independent)

**Validation Properties:**
- Validation should be deterministic
- Valid classes should pass validation
- Invalid classes should fail validation appropriately

**Color System Properties:**
- Colors should have valid CSS representations
- Colors should be round-trip serializable
- Color operations should maintain invariants

**Spacing System Properties:**
- Spacing values should have valid CSS representations
- Spacing should be round-trip serializable
- Spacing operations should maintain invariants

**Responsive System Properties:**
- Set values should be retrievable
- Unset breakpoints should return base value
- Responsive operations should maintain ordering

**Theme System Properties:**
- Set values should be retrievable
- Theme should be serializable
- Theme operations should maintain invariants

**Performance Properties:**
- Operations should terminate (no infinite loops)
- Complex inputs should not cause hangs
- Performance should be reasonable

#### Testing Library Property Tests (`tailwind-rs-testing/src/property_tests.rs`)

**Class Testing Properties:**
- Class testing should be deterministic
- Same input should produce same output
- Edge cases should be handled appropriately

**Component Testing Properties:**
- Generated HTML should contain all classes
- Generated HTML should contain original content
- HTML rendering should be consistent

**Theme Testing Properties:**
- Theme testing should be deterministic
- Theme testing should produce consistent results

**Responsive Testing Properties:**
- Correct breakpoint values should pass
- Matching responsive values should pass
- Responsive testing should be deterministic

**Mock Component Properties:**
- HTML should contain all classes and properties
- Mock components should be consistent
- Complex scenarios should not break

### Usage

Property-based tests run automatically with regular tests:

```bash
# Run all tests including property-based tests
cargo test

# Run only property-based tests
cargo test property_tests

# Run with more test cases (slower but more thorough)
PROPTEST_CASES=1000 cargo test
```

### Configuration

Property-based tests use the following configuration:

- **Default test cases**: 256 per property
- **Max shrink attempts**: 1000
- **Timeout**: 30 seconds per test
- **Thread safety**: Tests run with `--test-threads=1` to avoid flakiness

## API Stability Testing

### What is API Stability Testing?

API stability testing ensures that public APIs maintain their contracts and don't break between versions. This includes:

- Function signatures remain stable
- Serialization formats don't change
- Error types remain consistent
- Default values stay the same
- Performance characteristics remain stable

### Implementation

#### API Signature Stability (`api_stability.rs`)

**ClassBuilder API:**
- All public methods exist and work as expected
- API hasn't changed in breaking ways

**ClassSet API:**
- All public methods exist and work as expected
- API maintains consistency

**Color API:**
- All public constructors exist
- All public methods exist and work as expected
- API maintains consistency

**ResponsiveValue API:**
- All public methods exist and work as expected
- API maintains consistency

**Theme API:**
- All public methods exist and work as expected
- API maintains consistency

**ClassValidator API:**
- All public methods exist and work as expected
- API maintains consistency

#### Serialization Stability

**JSON Serialization:**
- All types can be serialized to JSON
- Serialization is deterministic
- Deserialization produces equivalent objects
- Format doesn't change between versions

**Round-trip Testing:**
- Serialize → Deserialize → Compare
- Ensures data integrity across versions

#### Error Stability

**ValidationError Variants:**
- All error variants exist and work
- Error messages are stable
- Error serialization works correctly

#### Default Value Stability

**Color Defaults:**
- Default colors remain consistent
- Color values don't change unexpectedly

**Spacing Defaults:**
- Default spacing values remain consistent
- Spacing representations don't change

**Theme Defaults:**
- Default theme values remain consistent
- Theme structure doesn't change

**Breakpoint Defaults:**
- Breakpoint min widths remain consistent
- Breakpoint ordering doesn't change

#### Performance Stability

**ClassBuilder Performance:**
- Performance remains within acceptable bounds
- No performance regressions

**ClassValidator Performance:**
- Validation performance remains stable
- No performance regressions

**Serialization Performance:**
- Serialization remains fast
- No performance regressions

#### Migration Stability

**Legacy API Patterns:**
- Old API patterns still work
- Backward compatibility maintained

**New API Compatibility:**
- New features don't break old usage
- Forward compatibility maintained

### Usage

API stability tests run automatically with regular tests:

```bash
# Run all tests including API stability tests
cargo test

# Run only API stability tests
cargo test api_stability
```

## Benefits

### Property-Based Testing Benefits

1. **Edge Case Discovery**: Finds bugs that unit tests might miss
2. **Invariant Validation**: Ensures system properties hold under all conditions
3. **Regression Prevention**: Catches regressions in complex scenarios
4. **Documentation**: Tests serve as living documentation of system behavior
5. **Confidence**: High confidence that system works correctly

### API Stability Testing Benefits

1. **Backward Compatibility**: Ensures APIs don't break between versions
2. **Migration Safety**: Safe to upgrade library versions
3. **Contract Enforcement**: Ensures APIs maintain their contracts
4. **Performance Monitoring**: Catches performance regressions
5. **Quality Assurance**: High confidence in API quality

## Integration with CI/CD

### Pre-commit Hooks

Property-based tests are integrated into pre-commit hooks:

```bash
# Pre-commit hook runs:
# 1. Regular tests
# 2. Property-based tests (with --test-threads=1)
# 3. Code formatting
# 4. Clippy linting
# 5. Documentation checks
```

### GitHub Actions

Property-based tests run in CI/CD pipelines:

```yaml
- name: Run property-based tests
  run: |
    cargo test --package tailwind-rs-core --package tailwind-rs-testing -- --test-threads=1
```

## Best Practices

### Property-Based Testing Best Practices

1. **Start Simple**: Begin with basic properties and add complexity
2. **Focus on Invariants**: Test properties that should always hold
3. **Use Appropriate Strategies**: Generate valid inputs for your domain
4. **Handle Edge Cases**: Explicitly test edge cases and error conditions
5. **Document Properties**: Clearly document what each property tests

### API Stability Testing Best Practices

1. **Test Public APIs**: Focus on public, stable APIs
2. **Version Compatibility**: Test compatibility across versions
3. **Performance Monitoring**: Monitor performance characteristics
4. **Error Handling**: Ensure error types remain stable
5. **Documentation**: Keep API documentation up to date

## Troubleshooting

### Property-Based Testing Issues

**Tests are flaky:**
- Use `--test-threads=1` to avoid race conditions
- Increase `PROPTEST_CASES` for more thorough testing
- Check for non-deterministic behavior in code

**Tests are slow:**
- Reduce `PROPTEST_CASES` for faster feedback
- Use more specific strategies to avoid invalid inputs
- Profile tests to find performance bottlenecks

**Tests find too many failures:**
- Review property definitions
- Check if properties are too strict
- Consider if the system behavior has changed

### API Stability Testing Issues

**Serialization failures:**
- Check if serialization format has changed
- Verify that all fields are serializable
- Check for new required fields

**Performance regressions:**
- Profile the specific operations
- Check for algorithmic changes
- Verify that performance requirements are realistic

**API signature changes:**
- Review if changes are intentional
- Check if changes break backward compatibility
- Consider deprecation strategies

## Future Enhancements

### Planned Improvements

1. **More Property Tests**: Add property tests for framework integrations
2. **Performance Benchmarks**: Add performance benchmarking
3. **Contract Testing**: Add contract testing for external APIs
4. **Mutation Testing**: Add mutation testing for test quality
5. **Fuzz Testing**: Add fuzz testing for security

### Integration Opportunities

1. **Property-Based Testing for Frameworks**: Extend to Leptos, Yew, Dioxus
2. **API Versioning**: Add support for API versioning
3. **Compatibility Matrix**: Test compatibility across framework versions
4. **Performance Regression Detection**: Automated performance monitoring
5. **Security Testing**: Add security-focused property tests

## Conclusion

Property-based testing and API stability testing significantly enhance the quality and reliability of the tailwind-rs library. They provide:

- **Comprehensive Coverage**: Test scenarios that manual tests might miss
- **Regression Prevention**: Catch issues before they reach production
- **API Confidence**: Ensure APIs remain stable and reliable
- **Documentation**: Serve as living documentation of system behavior
- **Quality Assurance**: High confidence in system correctness

These testing strategies align with our TDD approach and ADRs, ensuring that the tailwind-rs library maintains high quality standards and provides a reliable foundation for Tailwind CSS integration in Rust applications.
