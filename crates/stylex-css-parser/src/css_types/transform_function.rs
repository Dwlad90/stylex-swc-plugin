/*!
CSS transform function parser.
*/

use crate::token_parser::TokenParser;

/// A CSS transform function
#[derive(Debug, Clone, PartialEq)]
pub enum TransformFunction {
    Translate(super::length::Length, super::length::Length),
    Scale(f64, f64),
    // More transform functions will be added
}

impl TransformFunction {
    pub fn parse() -> TokenParser<TransformFunction> {
        todo!("Implementation pending")
    }
}
