/*!
CSS angle type parser.
*/

use crate::token_parser::TokenParser;

/// A CSS angle value
#[derive(Debug, Clone, PartialEq)]
pub struct Angle {
    pub value: f64,
    pub unit: String,
}

impl Angle {
    pub fn new(value: f64, unit: String) -> Self {
        Self { value, unit }
    }

    pub fn parse() -> TokenParser<Angle> {
        todo!("Implementation pending")
    }
}
