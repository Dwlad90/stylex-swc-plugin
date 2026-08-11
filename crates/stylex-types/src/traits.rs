use std::{any::Any, rc::Rc};

use indexmap::IndexMap;
use rustc_hash::FxHashMap;

use stylex_structures::stylex_state_options::StyleXStateOptions;

use crate::{
  enums::data_structures::injectable_style::InjectableStyleKind, structures::style_key::RuleKey,
};

/// Type alias for injectable styles map, moved here to be available at the
/// types level.
pub type InjectableStylesMap = IndexMap<RuleKey, Rc<InjectableStyleKind>>;

/// Tier 1: Minimal interface for CSS generation, PreRule, and function pointer
/// signatures.
///
/// Object-safe — used as `dyn StyleOptions` in function pointer signatures
/// (e.g., `StylexExprFn`, `FunctionType::ArrayArgs`).
///
/// `StateManager` implements this trait in the `stylex-transform` crate.
pub trait StyleOptions {
  /// Access the StyleX configuration options.
  fn options(&self) -> &StyleXStateOptions;

  /// Map of CSS properties already processed, used to avoid duplicates.
  fn css_property_seen(&self) -> &FxHashMap<String, String>;

  /// Mutable access to the CSS properties map.
  fn css_property_seen_mut(&mut self) -> &mut FxHashMap<String, String>;

  /// Access to injected CSS rules for keyframes, position-try, etc.
  fn other_injected_css_rules(&self) -> &InjectableStylesMap;

  /// Mutable access to injected CSS rules.
  fn other_injected_css_rules_mut(&mut self) -> &mut InjectableStylesMap;

  /// Downcast to concrete type for bridge during migration.
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// The value accepted in the second slot of the `stylex.when.*` functions.
///
/// That slot holds either a custom marker or the StyleX options, and the
/// marker itself is either a class-name string, an import proxy, or a
/// compiled `$$css` style object. Each accessor below performs one of those
/// type tests and returns `None` when it does not apply, so `when`'s
/// resolution chain stays a direct translation of its JavaScript original.
///
/// Object-safe — used as `dyn WhenMarkerValue` by `stylex-css`, which sits
/// below the evaluator and cannot name the evaluated-value types itself.
/// `StyleXStateOptions` is implemented here; `EvaluateResultValue`
/// implements it in the `stylex-transform` crate.
pub trait WhenMarkerValue {
  /// The `typeof options === 'string'` test: a marker passed as a literal
  /// class name, used verbatim.
  fn as_str_value(&self) -> Option<&str>;

  /// The `__IS_PROXY === true` test on its own, without resolving the proxy.
  /// Callers that only need to know whether a value *is* a marker should ask
  /// this rather than discarding an `as_proxy_string` result, which costs a
  /// hash and two allocations.
  fn is_proxy(&self) -> bool;

  /// The `__IS_PROXY === true` test: an import proxy standing in for a
  /// marker defined in another file, resolved through its `toString`.
  fn as_proxy_string(&self) -> Option<String>;

  /// The `$$css === true` test, yielding the first key that is not `$$css`
  /// — the class name a compiled marker object carries.
  fn first_css_key(&self) -> Option<&str>;

  /// The `classNamePrefix` property, absent on every marker shape and
  /// present only on the options. `None` reproduces JavaScript's `!= null`
  /// check, which decides whether the default marker gains a prefix.
  fn class_name_prefix(&self) -> Option<&str>;
}

impl WhenMarkerValue for StyleXStateOptions {
  fn as_str_value(&self) -> Option<&str> {
    None
  }

  fn is_proxy(&self) -> bool {
    false
  }

  fn as_proxy_string(&self) -> Option<String> {
    None
  }

  fn first_css_key(&self) -> Option<&str> {
    None
  }

  fn class_name_prefix(&self) -> Option<&str> {
    Some(&self.class_name_prefix)
  }
}
