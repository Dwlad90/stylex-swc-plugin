/*!
CSS blend mode parser.
*/

use crate::token_parser::TokenParser;

/// A CSS blend mode
#[derive(Debug, Clone, PartialEq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    // More blend modes will be added
}

impl BlendMode {
    pub fn parse() -> TokenParser<BlendMode> {
        todo!("Implementation pending")
    }
}
