/*!
CSS flex value parser.
*/

use crate::token_parser::TokenParser;

/// A CSS flex value
#[derive(Debug, Clone, PartialEq)]
pub struct Flex {
    pub grow: f64,
    pub shrink: f64,
    pub basis: String, // TODO: proper type
}

impl Flex {
    pub fn parse() -> TokenParser<Flex> {
        todo!("Implementation pending")
    }
}
