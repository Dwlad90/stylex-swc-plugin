use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::pre_rule::{CompiledResult, ComputedStyle, PreRule, Styles};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct NullPreRule {}

impl NullPreRule {
    pub(crate) fn new() -> Self {
        NullPreRule {}
    }
}

impl PreRule for NullPreRule {
    fn get_value(&self) -> Option<String> {
        None
    }
    fn compiled(&mut self, prefix: &str) -> CompiledResult {
        return CompiledResult::ComputedStyles(vec![]);
    }
    fn equals(&self, other: &dyn PreRule) -> bool {
        false
    }
}
