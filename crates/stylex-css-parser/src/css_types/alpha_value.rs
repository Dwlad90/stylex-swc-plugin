/*!
CSS alpha value type parser.
*/

use crate::token_parser::TokenParser;

/// A CSS alpha value (0-1 or percentage)
#[derive(Debug, Clone, PartialEq)]
pub struct AlphaValue {
    pub value: f64,
}

impl AlphaValue {
    pub fn parse() -> TokenParser<AlphaValue> {
        todo!("Implementation pending")
    }
}
