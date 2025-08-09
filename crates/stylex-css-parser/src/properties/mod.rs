/*!
CSS property parsers.

This module contains parsers for specific CSS properties that require complex parsing logic.
Each property has its own module with specialized parsing for that property's syntax.

Mirrors: packages/style-value-parser/src/properties/
*/

pub mod border_radius;
pub mod box_shadow;
pub mod transform;

// Re-export commonly used property types
pub use border_radius::{BorderRadiusIndividual, BorderRadiusShorthand};
pub use box_shadow::{BoxShadow, BoxShadowList};
pub use transform::Transform;
