use super::pre_rule::{CompiledResult, PreRule, PreRuleValue};

#[derive(Debug, Clone)]
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
    fn compiled(&mut self, _prefix: &str) -> CompiledResult {
        CompiledResult::Null
    }
    fn equals(&self, _other: &dyn PreRule) -> bool {
        false
    }
}
