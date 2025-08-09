/*!
Common CSS types used throughout the parser.
*/

use crate::token_parser::TokenParser;

/// A CSS number value
#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    pub value: f64,
}

impl Number {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn parse() -> TokenParser<Number> {
        todo!("Implementation pending")
    }
}

/// A CSS percentage value
#[derive(Debug, Clone, PartialEq)]
pub struct Percentage {
    pub value: f64,
}

impl Percentage {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn parse() -> TokenParser<Percentage> {
        todo!("Implementation pending")
    }
}
