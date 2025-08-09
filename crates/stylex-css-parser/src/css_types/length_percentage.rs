/*!
CSS length-percentage type parser.
*/

use crate::token_parser::TokenParser;

/// A CSS length-percentage value
#[derive(Debug, Clone, PartialEq)]
pub enum LengthPercentage {
    Length(super::length::Length),
    Percentage(super::common_types::Percentage),
}

impl LengthPercentage {
    pub fn parse() -> TokenParser<LengthPercentage> {
        todo!("Implementation pending")
    }
}
