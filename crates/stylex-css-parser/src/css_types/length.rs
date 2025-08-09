/*!
CSS length type parser.
*/

use crate::token_parser::TokenParser;

/// A CSS length value
#[derive(Debug, Clone, PartialEq)]
pub struct Length {
    pub value: f64,
    pub unit: String,
}

impl Length {
    pub fn new(value: f64, unit: String) -> Self {
        Self { value, unit }
    }

    pub fn parse() -> TokenParser<Length> {
        todo!("Implementation pending")
    }
}
