//! What one `positionTry` fallback compiles to.

#[cfg(test)]
mod stylex_position_try {
  use crate::shared::transformers::stylex_position_try::stylex_position_try;
  use crate::tests::support::expr;
  use stylex_enums::style_resolution::StyleResolution;
  use stylex_state::{evaluate_result_value::EvaluateResultValue, state_manager::StateManager};
  use stylex_types::{
    enums::data_structures::injectable_style::InjectableStyleKind,
    structures::injectable_style::InjectableStyle,
  };

  /// The name the fallback is written under, and the rule it stands for.
  fn compile_with(code: &str, state: &mut StateManager) -> (String, InjectableStyle) {
    let (name, rule) = stylex_position_try(&EvaluateResultValue::Expr(expr(code)), state);

    match rule {
      InjectableStyleKind::Regular(style) => (name, style),
      other => panic!("a fallback is a regular rule: {other:?}"),
    }
  }

  fn compile(code: &str) -> (String, InjectableStyle) {
    compile_with(code, &mut StateManager::default())
  }

  /// A state that resolves logical properties to physical ones.
  fn legacy_polyfill_state() -> StateManager {
    let mut state = StateManager::default();
    state.options.style_resolution = StyleResolution::LegacyExpandShorthands;
    state.options.enable_logical_styles_polyfill = true;
    state
  }

  #[test]
  fn writes_each_declaration_once() {
    let (name, style) = compile("{ top: '10px', marginTop: '4px' }");

    assert_eq!(name, "--x15ai9df");
    assert_eq!(
      style.ltr,
      format!("@position-try {name} {{margin-top:4px;top:10px;}}")
    );
    assert_eq!(style.rtl, None);
  }

  /// A logical value reads as a different physical value in each direction, so
  /// each direction carries the value it resolves to.
  ///
  /// `float` is refused by `assert_valid_position_try` before a module reaches
  /// this fold. It is used here because it is the shortest declaration the
  /// direction resolvers give a different value in each direction.
  #[test]
  fn carries_the_value_each_direction_resolves_a_logical_one_to() {
    let (name, style) = compile("{ float: 'start' }");

    assert_eq!(style.ltr, format!("@position-try {name} {{float:left;}}"));
    assert_eq!(
      style.rtl,
      Some(format!("@position-try {name} {{float:right;}}"))
    );
  }

  /// The direction resolvers can rename a property, and the body is written
  /// with the name they give.
  #[test]
  fn writes_the_property_name_each_direction_resolves_to() {
    let (name, style) = compile_with(
      "{ insetInlineStart: '10px', top: '0' }",
      &mut legacy_polyfill_state(),
    );

    assert_eq!(name, "--x62e4de");
    assert_eq!(
      style.ltr,
      format!("@position-try {name} {{left:10px;top:0;}}")
    );
    assert_eq!(
      style.rtl,
      Some(format!("@position-try {name} {{right:10px;top:0;}}"))
    );
  }

  /// When two declarations resolve to the same property name, the later one
  /// wins.
  #[test]
  fn keeps_the_later_declaration_when_a_physical_one_comes_last() {
    let (name, style) = compile_with(
      "{ insetInlineStart: '2px', left: '1px' }",
      &mut legacy_polyfill_state(),
    );

    assert_eq!(name, "--x1eefps4");
    assert_eq!(style.ltr, format!("@position-try {name} {{left:1px;}}"));
    assert_eq!(
      style.rtl,
      Some(format!("@position-try {name} {{left:1px;right:2px;}}"))
    );
  }

  #[test]
  fn keeps_the_later_declaration_when_a_logical_one_comes_last() {
    let (name, style) = compile_with(
      "{ left: '1px', insetInlineStart: '2px' }",
      &mut legacy_polyfill_state(),
    );

    assert_eq!(name, "--xt3jgm");
    assert_eq!(style.ltr, format!("@position-try {name} {{left:2px;}}"));
    assert_eq!(
      style.rtl,
      Some(format!("@position-try {name} {{left:1px;right:2px;}}"))
    );
  }

  /// A fallback that declares nothing is still a named rule, with an empty
  /// body.
  #[test]
  fn writes_an_empty_body_for_a_fallback_that_declares_nothing() {
    let (name, style) = compile("{}");

    assert_eq!(style.ltr, format!("@position-try {name} {{}}"));
    assert_eq!(style.rtl, None);
  }

  /// A value that spells no CSS text declares nothing, so the body is the one
  /// the other declarations write alone.
  #[test]
  fn drops_a_declaration_whose_value_spells_nothing() {
    let (name, style) = compile("{ positionAnchor: '--a', top: ' ' }");

    assert_eq!(name, "--xsgi8eb");
    assert_eq!(
      style.ltr,
      format!("@position-try {name} {{position-anchor:--a;}}")
    );
  }

  /// The declarations are written in the order their property names sort in,
  /// whatever order the author wrote them.
  #[test]
  fn sorts_the_declarations_by_property_name() {
    let (name, style) = compile("{ top: '1px', bottom: '2px', left: '3px' }");

    assert_eq!(name, "--x14uyb4i");
    assert_eq!(
      style.ltr,
      format!("@position-try {name} {{bottom:2px;left:3px;top:1px;}}")
    );
  }

  /// Names sort by UTF-16 code unit, as JavaScript sorts strings. An emoji is
  /// a surrogate pair, whose first unit sorts before a full-width character,
  /// although its UTF-8 bytes sort after it.
  #[test]
  fn sorts_the_property_names_by_utf16_code_unit() {
    let (name, style) = compile("{ '--\u{1F600}': 1, '--\u{FF01}': 2 }");

    assert_eq!(name, "--x1hfnf9");
    assert_eq!(
      style.ltr,
      format!("@position-try {name} {{--\u{1F600}:1;--\u{FF01}:2;}}")
    );
  }
}
