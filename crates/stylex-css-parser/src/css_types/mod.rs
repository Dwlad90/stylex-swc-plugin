/*!
CSS type parsers.

This module contains parsers for all CSS value types, mirroring the JavaScript css-types directory.
Each type has its own module with comprehensive parsing support.
*/

// Basic types
pub mod common_types;
pub mod number;
pub mod dimension;

// Color types
pub mod alpha_value;
pub mod color;

// Length and position types
pub mod length;
pub mod length_percentage;
pub mod angle;
pub mod angle_percentage;
pub mod position;

// Time and frequency types
pub mod time;
pub mod frequency;
pub mod resolution;

// Calculation types
pub mod calc;
pub mod calc_constant;

// Identifier types
pub mod custom_ident;
pub mod dashed_ident;

// Function types
pub mod easing_function;
pub mod filter_function;
pub mod transform_function;

// Shape and styling types
pub mod basic_shape;
pub mod blend_mode;
pub mod flex;

// Re-export commonly used types
pub use common_types::{CssWideKeyword, CssVariable, Number, Percentage, NumberOrPercentage};
pub use dimension::Dimension;
pub use length::Length;
pub use time::Time;
pub use frequency::Frequency;
pub use resolution::Resolution;
pub use color::{Color, NamedColor, HashColor, Rgb, Rgba, Hsl, Hsla, Lch, Oklch, Oklab};
pub use angle::Angle;
pub use alpha_value::AlphaValue;
pub use length_percentage::{LengthPercentage, length_percentage_parser};
pub use angle_percentage::{AnglePercentage, angle_percentage_parser};
pub use custom_ident::CustomIdentifier;
pub use dashed_ident::DashedIdentifier;
pub use calc_constant::CalcConstant;
pub use calc::{Calc, CalcValue, CalcDimension, BinaryOp, BinaryOperation, CalcGroup};
pub use blend_mode::BlendMode;
pub use flex::Flex;
pub use position::{Position, Horizontal, Vertical, HorizontalKeyword, VerticalKeyword};
