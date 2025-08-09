/*!
CSS border-radius property parser.
*/

use crate::token_parser::TokenParser;

/// An individual border-radius value
#[derive(Debug, Clone, PartialEq)]
pub struct BorderRadiusIndividual {
    // Implementation details will be added later
}

/// A shorthand border-radius value
#[derive(Debug, Clone, PartialEq)]
pub struct BorderRadiusShorthand {
    // Implementation details will be added later
}

impl BorderRadiusIndividual {
    pub fn parse() -> TokenParser<BorderRadiusIndividual> {
        todo!("Implementation pending")
    }
}

impl BorderRadiusShorthand {
    pub fn parse() -> TokenParser<BorderRadiusShorthand> {
        todo!("Implementation pending")
    }
}
