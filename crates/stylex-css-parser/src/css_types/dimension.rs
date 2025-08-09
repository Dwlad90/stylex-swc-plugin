/*!
CSS dimension type parser.
*/

use crate::token_parser::TokenParser;

/// A CSS dimension value (number with unit)
#[derive(Debug, Clone, PartialEq)]
pub struct Dimension {
    pub value: f64,
    pub unit: String,
}

impl Dimension {
    pub fn new(value: f64, unit: String) -> Self {
        Self { value, unit }
    }

    pub fn parse() -> TokenParser<Dimension> {
        todo!("Implementation pending")
    }
}
