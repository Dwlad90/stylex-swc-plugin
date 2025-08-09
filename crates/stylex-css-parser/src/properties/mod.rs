/*!
Property-specific CSS parsers.

This module contains parsers for complex CSS properties that require specialized parsing logic,
mirroring the JavaScript properties directory.
*/

pub mod transform;
pub mod box_shadow;
pub mod border_radius;

// Re-export main types
pub use transform::Transform;
pub use box_shadow::{BoxShadow, BoxShadowList};
pub use border_radius::{BorderRadiusIndividual, BorderRadiusShorthand};
