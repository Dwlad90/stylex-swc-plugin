/*!
CSS filter function parser.
*/

use crate::token_parser::TokenParser;

/// A CSS filter function
#[derive(Debug, Clone, PartialEq)]
pub enum FilterFunction {
    Blur(super::length::Length),
    Brightness(super::common_types::Percentage),
    // More filter functions will be added
}

impl FilterFunction {
    pub fn parse() -> TokenParser<FilterFunction> {
        todo!("Implementation pending")
    }
}
