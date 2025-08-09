# Contributing to StyleX CSS Parser

Thank you for your interest in contributing to the StyleX CSS Parser! This document provides guidelines and information for contributors.

## 🎯 Overview

This project is a **complete Rust port** of the JavaScript `style-value-parser` package. The primary goal is maintaining **100% API compatibility** while delivering superior performance through Rust's type safety and performance characteristics.

## 🏗️ Development Principles

### 1. API Compatibility First
- **Exact JavaScript Mirroring**: Every module, function, and type must match the JavaScript version exactly
- **Behavioral Parity**: All edge cases, error conditions, and parsing behavior must be identical
- **Export Structure**: Public API exports must mirror the JavaScript package structure
- **No Breaking Changes**: Any API changes must maintain backward compatibility

### 2. Performance and Safety
- **Performance**: Rust implementation should meet or exceed JavaScript performance
- **Memory Safety**: Leverage Rust's ownership system for zero-cost safety
- **Type Safety**: Use Rust's type system to prevent runtime errors
- **Concurrency**: Ensure thread-safe usage where applicable

### 3. Testing and Quality
- **Comprehensive Testing**: All features must have corresponding tests
- **JavaScript Test Parity**: Tests should mirror JavaScript test patterns
- **Benchmark Coverage**: Performance-critical code should have benchmarks
- **Documentation**: All public APIs must be documented

## 🚀 Getting Started

### Prerequisites

- **Rust 1.84+**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: For version control
- **Editor**: VS Code with rust-analyzer recommended

### Setting Up Development Environment

1. **Clone the Repository**
   ```bash
   git clone <repository-url>
   cd custom-css-parser/crates/stylex-css-parser
   ```

2. **Install Dependencies**
   ```bash
   cargo build
   ```

3. **Run Tests**
   ```bash
   cargo test
   ```

4. **Run Benchmarks**
   ```bash
   cargo bench
   ```

### Development Workflow

1. **Create a Branch**
   ```bash
   git checkout -b feature/new-css-type
   ```

2. **Make Changes**
   - Follow the coding standards below
   - Add tests for new functionality
   - Update documentation as needed

3. **Test Your Changes**
   ```bash
   cargo test
   cargo bench
   cargo clippy
   cargo fmt
   ```

4. **Submit Pull Request**
   - Ensure all tests pass
   - Include comprehensive description
   - Reference any related issues

## 📁 Project Structure

Understanding the codebase structure is crucial for effective contributions:

```
crates/stylex-css-parser/
├── src/
│   ├── lib.rs                    # Main entry point, JavaScript-compatible exports
│   ├── base_types.rs            # SubString utility (mirrors base-types.js)
│   ├── token_types.rs           # TokenList, tokenization (mirrors token-types.js)
│   ├── token_parser.rs          # Parser combinators (mirrors token-parser.js)
│   ├── css_types/               # CSS value types (mirrors css-types/)
│   │   ├── mod.rs               # Module exports and organization
│   │   ├── common_types.rs      # Number, Percentage, CssWideKeyword
│   │   ├── color.rs             # All color parsing functionality
│   │   ├── length.rs            # Length values and units
│   │   ├── calc.rs              # calc() expression parsing
│   │   └── ...                  # Additional CSS types
│   ├── properties/              # CSS properties (mirrors properties/)
│   │   ├── mod.rs               # Property exports
│   │   ├── transform.rs         # CSS transform property
│   │   ├── box_shadow.rs        # CSS box-shadow property
│   │   └── border_radius.rs     # CSS border-radius property
│   ├── at_queries/              # Media queries (mirrors at-queries/)
│   │   ├── media_query.rs       # Media query parsing
│   │   └── media_query_transform.rs # Query transformation
│   └── tests/                   # Integration tests
├── benches/                     # Performance benchmarks
├── tests/                       # Integration tests (if needed)
└── docs/                        # Additional documentation
```

### File Naming Conventions

- **Rust modules**: Use `snake_case` (e.g., `custom_ident.rs`)
- **JavaScript mirrors**: Name should clearly indicate the JS file it mirrors
- **Comments**: Include `// Mirrors: path/to/javascript/file.js` at the top

## 🧪 Testing Guidelines

### Test Categories

1. **Unit Tests**
   - Test individual functions and methods
   - Located in the same file as the implementation (`#[cfg(test)] mod tests`)
   - Focus on edge cases and error conditions

2. **Integration Tests**
   - Test module interactions and public APIs
   - Located in `src/tests/` directory
   - Mirror JavaScript test patterns

3. **Compatibility Tests**
   - Ensure JavaScript API compatibility
   - Located in `lib.rs` and module `tests`
   - Test export structure and function signatures

4. **Benchmark Tests**
   - Performance measurement and regression detection
   - Located in `benches/` directory
   - Use `criterion` crate for statistical analysis

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Test basic usage
        let result = YourType::new("input");
        assert_eq!(result.to_string(), "expected");
    }

    #[test]
    fn test_edge_cases() {
        // Test edge cases that might break
        assert!(YourType::new("").is_err());
        assert!(YourType::new("invalid").is_err());
    }

    #[test]
    fn test_javascript_compatibility() {
        // Ensure behavior matches JavaScript exactly
        let parser = YourType::parser();
        assert_eq!(parser.parse("test"), expected_js_behavior);
    }
}
```

### Test Naming

- Use descriptive test names: `test_color_parsing_with_alpha`
- Group related tests in modules: `mod color_tests { ... }`
- Mirror JavaScript test structure when possible

## 🎨 Coding Standards

### Rust Style

- **Formatting**: Use `cargo fmt` for consistent formatting
- **Linting**: Address all `cargo clippy` warnings
- **Naming**: Follow Rust naming conventions (snake_case, PascalCase)
- **Documentation**: Document all public APIs with `///` comments

### Code Organization

1. **Imports**: Group and organize imports logically
   ```rust
   use std::collections::HashMap;
   use crate::token_parser::TokenParser;
   use super::common_types::Number;
   ```

2. **Structure**: Organize code in logical sections
   ```rust
   // Type definitions
   pub struct YourType { ... }

   // Implementation
   impl YourType { ... }

   // Display trait
   impl Display for YourType { ... }

   // Tests
   #[cfg(test)]
   mod tests { ... }
   ```

3. **Comments**: Include helpful comments for complex logic
   ```rust
   /// Parses a CSS color value supporting all CSS color formats.
   ///
   /// This function handles named colors, hex values, rgb(), rgba(),
   /// hsl(), hsla(), and other CSS color specifications.
   ///
   /// # Examples
   ///
   /// ```rust
   /// let red = Color::parse("red").unwrap();
   /// let hex = Color::parse("#ff0000").unwrap();
   /// ```
   pub fn parse(input: &str) -> Result<Self, ParseError> {
       // Implementation
   }
   ```

## 🔄 JavaScript Compatibility

### Porting JavaScript Code

When porting JavaScript functionality:

1. **Study the Original**: Understand the JavaScript implementation thoroughly
2. **Maintain Behavior**: Ensure identical behavior, especially edge cases
3. **Preserve API**: Keep function signatures and return types equivalent
4. **Mirror Tests**: Port corresponding JavaScript tests

### API Mapping Guidelines

| JavaScript | Rust |
|------------|------|
| `camelCase` functions | `snake_case` with camelCase re-export |
| `PascalCase` classes | `PascalCase` structs |
| `function()` | `fn function_name()` |
| `static method()` | `impl Type { fn method() }` |
| `instance.method()` | `impl Type { fn method(&self) }` |

### Error Handling

```rust
// JavaScript: throw new Error("message")
// Rust: Err(CssParseError::ParseError { message: "message".to_string() })

pub type CssResult<T> = Result<T, CssParseError>;

#[derive(Debug, Clone, thiserror::Error)]
pub enum CssParseError {
    #[error("Parse error: {message}")]
    ParseError { message: String },

    #[error("Invalid CSS value: {value}")]
    InvalidValue { value: String },
}
```

## 📋 Contribution Types

### 1. CSS Type Implementation

Adding a new CSS type (e.g., `grid-template`):

1. **Create the module**: `src/css_types/grid_template.rs`
2. **Implement the struct**: With parsing and display logic
3. **Add parser method**: Using token parser combinators
4. **Write comprehensive tests**: Including edge cases
5. **Update module exports**: In `css_types/mod.rs`
6. **Add benchmarks**: In `benches/css_types_bench.rs`

### 2. CSS Property Implementation

Adding a new CSS property (e.g., `filter`):

1. **Create the module**: `src/properties/filter.rs`
2. **Implement property parsing**: Handle all syntax variations
3. **Support shorthand/longhand**: As appropriate
4. **Add comprehensive tests**: Cover all syntax forms
5. **Update module exports**: In `properties/mod.rs`
6. **Add benchmarks**: In `benches/properties_bench.rs`

### 3. Parser Combinator Enhancement

Improving the parser combinator system:

1. **Maintain API compatibility**: Don't break existing usage
2. **Add comprehensive tests**: Test all combinator combinations
3. **Benchmark performance**: Ensure no regressions
4. **Document new functionality**: With clear examples

### 4. Performance Optimization

Improving performance:

1. **Measure first**: Use benchmarks to identify bottlenecks
2. **Optimize carefully**: Maintain correctness and compatibility
3. **Test thoroughly**: Ensure optimizations don't break functionality
4. **Document changes**: Explain the optimization approach

## 🐛 Bug Reports

### Before Reporting

1. **Check existing issues**: Avoid duplicates
2. **Test with latest version**: Ensure bug still exists
3. **Minimal reproduction**: Create smallest possible test case
4. **Compare with JavaScript**: Verify it's actually a bug vs. intended behavior

### Bug Report Template

```markdown
## Bug Description
Brief description of the issue.

## Steps to Reproduce
1. Parse this CSS: `"..."`
2. Call this method: `...`
3. Observe incorrect result

## Expected Behavior
What should happen (reference JavaScript behavior).

## Actual Behavior
What actually happens.

## Environment
- Rust version:
- Crate version:
- OS:

## Additional Context
Any additional information that might be helpful.
```

## 🚀 Feature Requests

### Before Requesting

1. **Check JavaScript version**: Ensure feature exists in original
2. **Search existing issues**: Avoid duplicates
3. **Consider scope**: Should align with project goals

### Feature Request Template

```markdown
## Feature Description
Brief description of the requested feature.

## JavaScript Reference
Link or reference to equivalent JavaScript functionality.

## Use Case
Why is this feature needed? What problem does it solve?

## Proposed Implementation
High-level approach for implementation.

## Additional Context
Any additional information that might be helpful.
```

## 📝 Documentation

### Documentation Standards

1. **API Documentation**: All public items must have doc comments
2. **Examples**: Include usage examples in doc comments
3. **Error Cases**: Document when functions can fail
4. **JavaScript References**: Reference original JavaScript behavior

### Documentation Format

```rust
/// Parses a CSS length value with unit validation.
///
/// Supports all CSS length units including absolute (px, pt, in, cm, mm, pc),
/// relative (em, rem, ex, ch, lh, rlh), viewport (vh, vw, vmin, vmax, svh, svw,
/// lvh, lvw, dvh, dvw), and container query units (cqw, cqh, cqi, cqb).
///
/// # Examples
///
/// ```rust
/// use stylex_css_parser::css_types::Length;
///
/// let px_length = Length::parse("10px").unwrap();
/// assert_eq!(px_length.value, 10.0);
/// assert_eq!(px_length.unit, "px");
///
/// let em_length = Length::parse("1.5em").unwrap();
/// assert_eq!(em_length.value, 1.5);
/// assert_eq!(em_length.unit, "em");
/// ```
///
/// # Errors
///
/// Returns `CssParseError::InvalidValue` if:
/// - Input contains invalid number format
/// - Unit is not a recognized CSS length unit
/// - Input is empty or malformed
///
/// # JavaScript Compatibility
///
/// This function maintains exact compatibility with the JavaScript
/// `Length.parse()` method, including edge case handling and error messages.
pub fn parse(input: &str) -> CssResult<Self> {
    // Implementation
}
```

## 🔒 Security

### Security Considerations

1. **Input Validation**: Always validate CSS input thoroughly
2. **Memory Safety**: Leverage Rust's ownership system
3. **No Unsafe Code**: Avoid `unsafe` blocks unless absolutely necessary
4. **Dependency Management**: Keep dependencies up to date

### Reporting Security Issues

For security-related issues, please email [security contact] rather than creating public issues.

## 📞 Getting Help

### Resources

1. **Documentation**: Check the comprehensive README and API docs
2. **Tests**: Look at existing tests for usage examples
3. **JavaScript Reference**: Consult the original JavaScript implementation
4. **Issues**: Search existing issues for similar problems

### Communication

- **GitHub Issues**: For bugs, features, and questions
- **Discussions**: For general questions and community interaction
- **Code Reviews**: Constructive feedback on pull requests

## 🙏 Recognition

Contributors will be recognized in:
- CHANGELOG.md for significant contributions
- README.md acknowledgments section
- Git commit history and pull request records

Thank you for contributing to the StyleX CSS Parser! Your contributions help make CSS parsing faster, safer, and more reliable for the entire community.
