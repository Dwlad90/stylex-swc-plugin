/*!
# StyleX CSS Parser

A high-performance CSS value parser for StyleX, written in Rust.

This crate provides comprehensive CSS value parsing capabilities, including support for all CSS value types,
function parsing, property-specific parsers, and media query parsing.

## Module Organization

- [`token_parser`] - Core parser combinator library
- [`css_types`] - Individual CSS type parsers (color, length, angle, etc.)
- [`properties`] - Property-specific parsers for complex CSS properties
- [`at_queries`] - Media query and at-rule parsing

## Quick Start

```rust
use stylex_css_parser::css_types::Color;

// Parse a color value
let color = Color::parse("rgb(255, 0, 0)").unwrap();
```
*/

pub mod base_types;
pub mod token_types;
pub mod token_parser;
pub mod css_types;
pub mod properties;
pub mod at_queries;

// Re-export main APIs to match JavaScript structure
pub use token_parser as tokenParser;
pub use at_queries::lastMediaQueryWinsTransform;

/// Main error type for CSS parsing operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum CssParseError {
    #[error("Parse error: {message}")]
    ParseError { message: String },

    #[error("Invalid CSS value: {value}")]
    InvalidValue { value: String },

    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("Invalid token: {token}")]
    InvalidToken { token: String },
}

/// Result type for CSS parsing operations
pub type CssResult<T> = std::result::Result<T, CssParseError>;
