use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::pre_rule::{CompiledResult, ComputedStyle, PreRule, PreRuleValue, Styles};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct NullPreRule {}

impl NullPreRule {
    pub(crate) fn new() -> Self {
        NullPreRule {}
    }
}

impl PreRule for NullPreRule {
    fn get_value(&self) -> Option<PreRuleValue> {
        None
    }
    fn compiled(&mut self, prefix: &str) -> CompiledResult {
        return CompiledResult::Null(Option::None);
    }
    fn equals(&self, other: &dyn PreRule) -> bool {
        false
    }
}
