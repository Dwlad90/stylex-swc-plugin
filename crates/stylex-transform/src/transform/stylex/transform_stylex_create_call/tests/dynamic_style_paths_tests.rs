//! What a dynamic style's key and path say about each other.
//!
//! The legacy shorthand expansion rewrites a path by replacing the key at its
//! head, and it can only do that because the key is a prefix of the path. The
//! rule is stated on `dynamic_styles_of_namespace` and measured here.

use indexmap::IndexMap;
use stylex_ast::ast::convertors::create_string_expr;
use stylex_enums::style_resolution::StyleResolution;
use stylex_state::types::TInlineStyles;
use stylex_structures::inline_style::InlineStyle;

use super::super::dynamic_style_functions::dynamic_styles_of_namespace;

/// One inline style, written under `path`.
fn inline_styles_of(paths: &[&[&str]]) -> TInlineStyles {
  let mut inline_styles: TInlineStyles = IndexMap::new();

  for (index, path) in paths.iter().enumerate() {
    inline_styles.insert(
      format!("--x-{index}"),
      Box::new(InlineStyle {
        path: path.iter().map(|step| (*step).to_string()).collect(),
        original_expression: create_string_expr("red"),
        expression: create_string_expr("red"),
      }),
    );
  }

  inline_styles
}

#[test]
fn a_key_is_a_prefix_of_its_path_that_ends_at_a_property() {
  // A property on its own, a property under one selector and under two, a
  // property with steps below it, and an at-rule above a pseudo selector.
  let paths: &[&[&str]] = &[
    &["color"],
    &[":hover", "color"],
    &["@media (min-width: 1px)", ":hover", "color"],
    &["color", "default"],
    &["marginInline", "default"],
    &["@media (min-width: 1px)", "color", ":hover"],
  ];

  let dynamic_styles =
    dynamic_styles_of_namespace(&inline_styles_of(paths), &StyleResolution::ApplicationOrder);

  assert_eq!(dynamic_styles.len(), paths.len());

  for dynamic_style in &dynamic_styles {
    assert!(
      !dynamic_style.key.is_empty(),
      "a key that names no property: {dynamic_style:?}"
    );

    assert!(
      dynamic_style.path == dynamic_style.key
        || dynamic_style
          .path
          .starts_with(&(dynamic_style.key.clone() + "_")),
      "a key that is no prefix of its path: {dynamic_style:?}"
    );
  }
}

/// A path made only of selectors has no property to cut at, and the walk that
/// writes one never produces it: a declaration is written under a property.
#[test]
fn a_path_of_selectors_alone_is_not_written() {
  let dynamic_styles = dynamic_styles_of_namespace(
    &inline_styles_of(&[&[":hover", "@media (min-width: 1px)"]]),
    &StyleResolution::ApplicationOrder,
  );

  // Read as the rule reads it: with no property, the cut is empty. The case is
  // here to say that such a path is not one the compiler writes, rather than to
  // bless the answer.
  assert_eq!(dynamic_styles[0].key, "");
}
