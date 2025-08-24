/*!
At-rule parsing functionality.

This module contains parsers for CSS at-rules, particularly media queries.
Includes support for complex media query parsing, validation, and transformation.
*/

pub mod media_query;
pub mod media_query_transform;
pub mod messages;

pub use media_query::{validate_media_query, MediaQuery, MediaQueryRule};
pub use media_query_transform::lastMediaQueryWinsTransform;
pub use messages::MediaQueryErrors;
