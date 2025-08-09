/*!
CSS resolution type parser.
*/

use crate::token_parser::TokenParser;

/// A CSS resolution value
#[derive(Debug, Clone, PartialEq)]
pub struct Resolution {
    pub value: f64,
    pub unit: String, // dpi, dpcm, dppx
}

impl Resolution {
    pub fn parse() -> TokenParser<Resolution> {
        todo!("Implementation pending")
    }
}
