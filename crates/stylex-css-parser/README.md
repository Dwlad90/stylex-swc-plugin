# StyleX CSS Parser

A high-performance CSS value parser written in Rust, providing comprehensive parsing and validation for CSS properties, types, and at-rules.

## Features

- **CSS Type Parsers**: Complete support for colors, lengths, angles, calc expressions, transform functions, and more
- **Property Parsers**: Specialized parsers for `transform`, `box-shadow`, `border-radius`, and other complex properties
- **Media Query Support**: Full media query parsing, validation, and transformation with "last media query wins" logic
- **Parser Combinators**: Flexible, composable parsing system with backtracking support
- **Comprehensive Testing**: 480+ tests ensuring robust CSS parsing behavior
- **Zero Dependencies**: Pure Rust implementation with no external runtime dependencies

## Usage

```rust
use stylex_css_parser::{parse_color, parse_length, parse_transform};

// Parse CSS colors
let color = parse_color("rgba(255, 0, 0, 0.5)").unwrap();

// Parse CSS lengths
let length = parse_length("10px").unwrap();

// Parse transform functions
let transform = parse_transform("translateX(50px) rotate(45deg)").unwrap();
```

## CSS Types Supported

- Colors (hex, rgb, rgba, hsl, hsla, named colors)
- Lengths (px, em, rem, vh, vw, etc.)
- Angles (deg, rad, grad, turn)
- Calc expressions with full arithmetic
- Transform functions (translate, rotate, scale, etc.)
- Position values
- Basic shapes
- Media queries

## License

MIT licensed. See [LICENSE](../../LICENSE) for details.
