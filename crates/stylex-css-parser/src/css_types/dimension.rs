/*!
CSS Dimension type parsing.

Handles dimensional values that can be lengths, times, frequencies, or resolutions.
Mirrors: packages/style-value-parser/src/css-types/dimension.js
*/

use crate::token_parser::TokenParser;

/// Union type for all dimensional CSS values
#[derive(Debug, Clone, PartialEq)]
pub enum Dimension {
    // TODO: Add Length, Time, Resolution, Frequency when implemented
    Placeholder,
}

impl Dimension {
    /// Parser for dimensional values
    /// Mirrors: dimension in dimension.js
    pub fn parser() -> TokenParser<Dimension> {
        // TODO: Implement when Length, Time, Resolution, Frequency are ready
        TokenParser::<Dimension>::never()
    }
}
