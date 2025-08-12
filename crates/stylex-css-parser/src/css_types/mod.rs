/*!
CSS type parsers.

This module contains parsers for all CSS value types, mirroring the JavaScript css-types directory.
Each type has its own module with comprehensive parsing support.
*/

// Basic types
pub mod common_types;
pub mod dimension;
pub mod number;

// Color types
pub mod alpha_value;
pub mod color;

// Length and position types
pub mod angle;
pub mod angle_percentage;
pub mod length;
pub mod length_percentage;
pub mod position;

// Time and frequency types
pub mod frequency;
pub mod resolution;
pub mod time;

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
pub use alpha_value::AlphaValue;
pub use angle::Angle;
pub use angle_percentage::{angle_percentage_parser, AnglePercentage};
pub use blend_mode::BlendMode;
pub use calc::{
  calc_value_to_string, Addition, Calc, CalcDimension, CalcValue, Division, Group, Multiplication,
  Subtraction,
};
pub use calc_constant::CalcConstant;
pub use color::{Color, HashColor, Hsl, Hsla, Lch, NamedColor, Oklab, Oklch, Rgb, Rgba};
pub use common_types::{CssVariable, CssWideKeyword, Number, NumberOrPercentage, Percentage};
pub use custom_ident::CustomIdentifier;
pub use dashed_ident::DashedIdentifier;
pub use dimension::Dimension;
pub use flex::Flex;
pub use frequency::Frequency;
pub use length::Length;
pub use length_percentage::{length_percentage_parser, LengthPercentage};
pub use position::{Horizontal, HorizontalKeyword, Position, Vertical, VerticalKeyword};
pub use resolution::Resolution;
pub use time::Time;
