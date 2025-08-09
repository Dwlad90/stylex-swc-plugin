/*!
CSS box-shadow property parser.
*/

use crate::token_parser::TokenParser;

/// A single CSS box-shadow value
#[derive(Debug, Clone, PartialEq)]
pub struct BoxShadow {
    // Implementation details will be added later
}

/// A list of CSS box-shadow values
#[derive(Debug, Clone, PartialEq)]
pub struct BoxShadowList {
    pub shadows: Vec<BoxShadow>,
}

impl BoxShadow {
    pub fn parse() -> TokenParser<BoxShadow> {
        todo!("Implementation pending")
    }
}

impl BoxShadowList {
    pub fn parse() -> TokenParser<BoxShadowList> {
        todo!("Implementation pending")
    }
}
