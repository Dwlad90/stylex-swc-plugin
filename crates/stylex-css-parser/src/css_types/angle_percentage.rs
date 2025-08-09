/*!
CSS angle-percentage type parser.
*/

use crate::token_parser::TokenParser;

/// A CSS angle-percentage value
#[derive(Debug, Clone, PartialEq)]
pub enum AnglePercentage {
    Angle(super::angle::Angle),
    Percentage(super::common_types::Percentage),
}

impl AnglePercentage {
    pub fn parse() -> TokenParser<AnglePercentage> {
        todo!("Implementation pending")
    }
}
