/*!
CSS custom identifier parser.
*/

use crate::token_parser::TokenParser;

/// A CSS custom identifier
#[derive(Debug, Clone, PartialEq)]
pub struct CustomIdent {
    pub value: String,
}

impl CustomIdent {
    pub fn parse() -> TokenParser<CustomIdent> {
        todo!("Implementation pending")
    }
}
