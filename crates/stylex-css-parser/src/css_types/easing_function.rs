/*!
CSS easing function parser.
*/

use crate::token_parser::TokenParser;

/// A CSS easing function
#[derive(Debug, Clone, PartialEq)]
pub enum EasingFunction {
    Linear,
    CubicBezier(f64, f64, f64, f64),
    // More easing functions will be added
}

impl EasingFunction {
    pub fn parse() -> TokenParser<EasingFunction> {
        todo!("Implementation pending")
    }
}
