/*!
CSS time type parser.
*/

use crate::token_parser::TokenParser;

/// A CSS time value
#[derive(Debug, Clone, PartialEq)]
pub struct Time {
    pub value: f64,
    pub unit: String, // s, ms
}

impl Time {
    pub fn parse() -> TokenParser<Time> {
        todo!("Implementation pending")
    }
}
