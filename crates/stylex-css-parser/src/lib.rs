/*!
# StyleX CSS Parser

A high-performance CSS value parser for StyleX, written in Rust.

This crate is a complete port of the JavaScript `style-value-parser` package, providing
comprehensive CSS value parsing capabilities with exact API compatibility.

## Features

- **Token Parser Combinators**: Monadic parser combinators for building complex parsers
- **CSS Type Parsers**: Support for all CSS value types (colors, lengths, angles, calc expressions, etc.)
- **Property Parsers**: Specialized parsers for complex CSS properties (transform, box-shadow, border-radius)
- **Media Query Support**: Complete media query parsing and transformation
- **Type Safety**: Full Rust type safety with comprehensive error handling
- **Performance**: High-performance parsing with minimal allocations

## API Compatibility

This crate mirrors the JavaScript API exactly:

```rust
use stylex_css_parser::{tokenParser, properties, lastMediaQueryWinsTransform};
use stylex_css_parser::css_types::{Color, Length, Calc};

// Parse CSS values
let color = Color::parser(); // CSS color parser
let length = Length::parser(); // CSS length parser
let calc = Calc::parse(); // CSS calc() expression parser

// Property parsers
let transform = properties::Transform::parser();
let box_shadow = properties::BoxShadow::parser();
let border_radius = properties::BorderRadiusShorthand::parser();

// Media query transformation
let queries = vec![/* MediaQuery instances */];
let transformed = lastMediaQueryWinsTransform(queries);
```

## Module Organization

- [`tokenParser`] - Core parser combinator library (mirrors JS `tokenParser`)
- [`properties`] - Property-specific parsers (mirrors JS `properties`)
- [`css_types`] - Individual CSS type parsers (color, length, angle, etc.)
- [`at_queries`] - Media query and at-rule parsing
- [`base_types`] - Base utility types (SubString, etc.)
- [`token_types`] - Tokenization utilities

## JavaScript Compatibility

This crate provides the exact same exports as the JavaScript package:

**JavaScript:**
```javascript
import { tokenParser, properties, lastMediaQueryWinsTransform } from 'style-value-parser';
```

**Rust:**
```rust
use stylex_css_parser::{tokenParser, properties, lastMediaQueryWinsTransform};
```

*/

pub mod at_queries;
pub mod base_types;
pub mod css_types;
pub mod properties;
pub mod token_parser;
pub mod token_types;

#[cfg(test)]
pub mod tests;

// Re-export main APIs to match JavaScript structure exactly
// Mirrors: export * as tokenParser from './token-parser';
pub use token_parser as tokenParser;

// Note: properties module is already publicly exported via `pub mod properties;`
// Mirrors: export * as properties from './properties';

// Mirrors: export { lastMediaQueryWinsTransform } from './at-queries/media-query-transform.js';
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

#[cfg(test)]
mod lib_tests {
  use super::*;

  #[test]
  fn test_api_exports_available() {
    // Test that all main API exports are accessible
    // This ensures the JavaScript-compatible API is working

    // Test tokenParser module access
    let _token_parser = tokenParser::TokenParser::<i32>::never();

    // Test properties module access
    let _transform = properties::Transform::new(vec![]);
    let _box_shadow = properties::BoxShadow::new(
      css_types::Length::new(0.0, "px".to_string()),
      css_types::Length::new(0.0, "px".to_string()),
      css_types::Length::new(0.0, "px".to_string()),
      css_types::Length::new(0.0, "px".to_string()),
      css_types::Color::Named(css_types::NamedColor::new("black".to_string())),
      false,
    );
    let _border_radius = properties::BorderRadiusIndividual::new(
      css_types::LengthPercentage::Length(css_types::Length::new(0.0, "px".to_string())),
      None,
    );

    // Test lastMediaQueryWinsTransform function access
    let styles = serde_json::json!({});
    let _transformed = lastMediaQueryWinsTransform(styles);

    // Test css_types module access
    let _color = css_types::Color::parser();
    let _length = css_types::Length::parser();
    let _calc = css_types::Calc::parse();
  }

  #[test]
  fn test_javascript_compatible_imports() {
    // Test that we can import in a JavaScript-compatible way
    use crate::css_types::{Calc, Color, Length};
    use crate::{lastMediaQueryWinsTransform, properties};

    // Test that imports work and types are accessible
    let _transform = properties::Transform::new(vec![]);
    let _transform_fn = lastMediaQueryWinsTransform;
    let _color_parser = Color::parser();
    let _length_parser = Length::parser();
    let _calc_parser = Calc::parse();
  }

  #[test]
  fn test_crate_level_types() {
    // Test that crate-level types are working
    let error = CssParseError::ParseError {
      message: "test error".to_string(),
    };

    let result: CssResult<i32> = Err(error);
    assert!(result.is_err());
  }

  #[test]
  fn test_module_accessibility() {
    // Test that all modules are accessible
    let _substring = base_types::SubString::new("test");
    let _token_list = token_types::TokenList::new("test");
    let _token_parser = token_parser::TokenParser::<()>::never();
    let _color = css_types::Color::parser();
    let _transform = properties::Transform::new(vec![]);
    let _media_query = at_queries::MediaQuery::new("@media screen".to_string());
  }

  #[test]
  fn test_error_types_comprehensive() {
    // Test all error variants
    let errors = vec![
      CssParseError::ParseError {
        message: "parse error".to_string(),
      },
      CssParseError::InvalidValue {
        value: "invalid".to_string(),
      },
      CssParseError::UnexpectedEndOfInput,
      CssParseError::InvalidToken {
        token: "bad".to_string(),
      },
    ];

    for error in errors {
      // Test that errors can be cloned and debugged
      let _cloned = error.clone();
      let _debug_str = format!("{:?}", error);
      let _display_str = format!("{}", error);
    }
  }

  #[test]
  fn test_readme_example_compatibility() {
    // Test examples from the README/documentation work
    use crate::css_types::{Calc, Color, Length};
    use crate::{lastMediaQueryWinsTransform, properties};

    // Parse CSS values
    let _color = Color::parser(); // CSS color parser
    let _length = Length::parser(); // CSS length parser
    let _calc = Calc::parse(); // CSS calc() expression parser

    // Property parsers
    let _transform = properties::Transform::parse();
    let _box_shadow = properties::BoxShadow::parse();
    let _border_radius = properties::BorderRadiusShorthand::parse();

    // Media query transformation
    let styles = serde_json::json!({});
    let _transformed = lastMediaQueryWinsTransform(styles);
  }
}
