use crate::evaluate_result_value::EvaluateResultValue;

/// What one memoized evaluation answers for: a subtree, folded under one
/// import mode.
///
/// The structural hash alone used to be the key, and it is one question short.
/// An import binding folds to two different answers depending on the fold's
/// `FunctionMap::disable_imports`: the theme reference the imported file
/// exports where imports resolve, and a refusal where the fold is held inside
/// the module -- which is how a `stylex.props` argument is read, so that an
/// imported marker in it is handed to the runtime rather than merged. One entry
/// cannot serve both. A `create` call that resolved the marker first left the
/// reference in the memo, the props merge read it back as a confident value,
/// and a style list holding a theme reference aborted the build.
///
/// The reference implementation never meets the question: its memo lives on
/// one `evaluate()` call, so each mode asks it afresh. This memo lives for the
/// module, which is what makes it pay for itself, so the mode is part of the
/// key instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SeenKey {
  /// `stable_hash_unspanned` of the folded expression.
  pub subtree: u128,
  /// The fold's `FunctionMap::disable_imports`.
  pub imports_disabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SeenValue {
  pub value: Option<EvaluateResultValue>,
  pub resolved: bool,
}
