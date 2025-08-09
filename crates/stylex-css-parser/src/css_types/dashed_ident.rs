/*!
CSS dashed identifier parser.
*/

use crate::token_parser::TokenParser;

/// A CSS dashed identifier (starts with --)
#[derive(Debug, Clone, PartialEq)]
pub struct DashedIdent {
    pub value: String,
}

impl DashedIdent {
    pub fn parse() -> TokenParser<DashedIdent> {
        todo!("Implementation pending")
    }
}
