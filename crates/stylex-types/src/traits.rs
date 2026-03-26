use std::any::Any;
use std::rc::Rc;

use indexmap::IndexMap;
use rustc_hash::FxHashMap;

use stylex_structures::stylex_state_options::StyleXStateOptions;

use crate::enums::data_structures::injectable_style::InjectableStyleKind;

/// Type alias for injectable styles map, moved here to be available at the types level.
pub type InjectableStylesMap = IndexMap<String, Rc<InjectableStyleKind>>;

/// Tier 1: Minimal interface for CSS generation, PreRule, and function pointer signatures.
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
