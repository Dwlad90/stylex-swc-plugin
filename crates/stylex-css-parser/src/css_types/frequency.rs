/*!
CSS frequency type parser.
*/

use crate::token_parser::TokenParser;

/// A CSS frequency value
#[derive(Debug, Clone, PartialEq)]
pub struct Frequency {
    pub value: f64,
    pub unit: String, // Hz, kHz
}

impl Frequency {
    pub fn parse() -> TokenParser<Frequency> {
        todo!("Implementation pending")
    }
}
