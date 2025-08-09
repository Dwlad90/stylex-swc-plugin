/*!
CSS calc constants parser.
*/

use crate::token_parser::TokenParser;

/// A CSS calc constant
#[derive(Debug, Clone, PartialEq)]
pub enum CalcConstant {
    Pi,
    E,
    // More constants will be added
}

impl CalcConstant {
    pub fn parse() -> TokenParser<CalcConstant> {
        todo!("Implementation pending")
    }
}
