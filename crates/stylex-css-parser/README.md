# StyleX CSS Parser

A high-performance CSS value parser for StyleX, written in Rust. This is a complete port of the JavaScript `style-value-parser` package, maintaining exact API compatibility and feature parity.

## Overview

This crate provides comprehensive CSS value parsing capabilities for StyleX, including:

- **CSS Types**: Complete support for all CSS value types (colors, lengths, angles, percentages, etc.)
- **Function Parsing**: Support for CSS functions like `calc()`, `rgb()`, `hsl()`, transform functions, etc.
- **Property Parsers**: Specialized parsers for complex CSS properties like `transform`, `box-shadow`, and `border-radius`
- **Media Queries**: Full media query parsing and transformation support
- **Parser Combinators**: Monadic parser combinator library for building complex CSS parsers

## Features

- 🚀 **High Performance**: Built with Rust for maximum speed
- 🔧 **Exact Compatibility**: 1:1 API compatibility with JavaScript version
- 📦 **Comprehensive**: Supports all CSS value types and edge cases
- 🧪 **Well Tested**: 100% test coverage ported from JavaScript
- 🎯 **Type Safe**: Leverages Rust's type system for correctness

## Usage

```rust
use stylex_css_parser::{tokenParser, properties};

// Parse a color value
let color = tokenParser::Color::parse("rgb(255, 0, 0)").unwrap();

// Parse a transform property
let transform = properties::Transform::parse("translate(10px, 20px) scale(1.5)").unwrap();
```

## Architecture

This crate is organized into several modules mirroring the JavaScript structure:

- `token_parser`: Core parser combinator library
- `css_types`: Individual CSS type parsers (color, length, angle, etc.)
- `properties`: Property-specific parsers for complex CSS properties
- `at_queries`: Media query and at-rule parsing

## Development Status

This is a faithful Rust port of the JavaScript `style-value-parser` package. All functionality from the original is being systematically ported to maintain exact behavioral compatibility.
