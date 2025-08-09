/*!
CSS basic shape parser.
*/

use crate::token_parser::TokenParser;

/// A CSS basic shape
#[derive(Debug, Clone, PartialEq)]
pub enum BasicShape {
    Circle,
    Ellipse,
    // More shapes will be added
}

impl BasicShape {
    pub fn parse() -> TokenParser<BasicShape> {
        todo!("Implementation pending")
    }
}
