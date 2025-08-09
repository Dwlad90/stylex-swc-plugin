/*!
CSS color type parser.
*/

use crate::token_parser::TokenParser;

/// A CSS color value
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Named(String),
    Hex(String),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, f64),
    // More color types will be added
}

impl Color {
    pub fn parse() -> TokenParser<Color> {
        todo!("Implementation pending")
    }
}
