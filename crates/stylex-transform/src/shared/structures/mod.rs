// Kept locally (the evaluator's own result types, and the pre-rule chain)
pub mod evaluate_result;
pub(crate) mod null_pre_rule;
pub(crate) mod pre_rule;
pub(crate) mod pre_rule_set;
pub mod state;
#[cfg(test)]
pub(crate) mod tests;
