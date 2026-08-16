//! The predicate behind the unprefixed-custom-property rejection, asserted
//! against hand-built token lists.
//!
//! The rule's *behaviour* is asserted at the public entry point, in
//! `css/tests/unprefixed_custom_properties_test.rs`; what is here is the one
//! thing that seam cannot reach — node shapes the scanner never produces. A
//! function node carrying no argument list at all, a word whose text is empty,
//! a name spelled by a node kind that is not a word: none of them can be
//! written in CSS, so none of them can be tested through a value, and each is
//! a branch that would otherwise be held up only by reading.
//!
//! Source:
//! `crates/stylex-css/src/css/normalizers/unprefixed_custom_properties.rs`

use postcss_value_parser::{Node, NodeKind, ValueParser};

use super::{detect_unprefixed_custom_properties, names_unprefixed_property};

/// A node of `kind` spelling `value`, with source offsets a caller never reads.
fn node(kind: NodeKind, value: &str) -> Node {
  Node::new(kind, value.to_string(), 0, value.len())
}

/// A function node named `name` over `args`.
fn function(name: &str, args: Vec<Node>) -> Node {
  let mut func = node(NodeKind::Function, name);
  func.nodes = Some(args);
  func
}

#[test]
fn a_reference_naming_a_bare_word_is_unprefixed() {
  assert!(names_unprefixed_property(&function(
    "var",
    vec![node(NodeKind::Word, "foo")]
  )));
}

#[test]
fn a_reference_naming_a_prefixed_word_is_not() {
  assert!(!names_unprefixed_property(&function(
    "var",
    vec![node(NodeKind::Word, "--foo")]
  )));
}

/// One dash is not two. `-foo` is a legal CSS identifier and an illegal custom
/// property name, which is exactly the typo this rule is for.
#[test]
fn a_single_leading_dash_is_unprefixed() {
  assert!(names_unprefixed_property(&function(
    "var",
    vec![node(NodeKind::Word, "-foo")]
  )));
}

/// The scanner cannot produce this — a function node always carries an
/// argument list, empty at worst. Reached only by hand, and it must not be
/// mistaken for a reference to a property named nothing.
#[test]
fn a_function_with_no_argument_list_names_nothing() {
  let mut func = node(NodeKind::Function, "var");
  func.nodes = None;

  assert!(!names_unprefixed_property(&func));
}

#[test]
fn a_function_with_an_empty_argument_list_names_nothing() {
  assert!(!names_unprefixed_property(&function("var", vec![])));
}

/// Also unreachable through a value: the word scan never emits an empty word.
/// An empty name is not a prefixed one, so the predicate reports it — which is
/// the safe direction, since the value it would come from is not a reference.
#[test]
fn an_empty_first_word_is_unprefixed() {
  assert!(names_unprefixed_property(&function(
    "var",
    vec![node(NodeKind::Word, "")]
  )));
}

/// Every kind other than `Word` is something the author wrote in place of a
/// name, not a name spelled wrong.
#[test]
fn a_first_argument_that_is_not_a_word_names_nothing() {
  let kinds = [
    NodeKind::String,
    NodeKind::Div,
    NodeKind::Space,
    NodeKind::Comment,
    NodeKind::UnicodeRange,
  ];

  for kind in kinds {
    assert!(
      !names_unprefixed_property(&function("var", vec![node(kind, "foo")])),
      "a first argument of kind `{}` should not be read as a property name",
      kind.as_str()
    );
  }
}

#[test]
fn a_nested_reference_as_the_first_argument_names_nothing() {
  let inner = function("var", vec![node(NodeKind::Word, "foo")]);

  assert!(!names_unprefixed_property(&function("var", vec![inner])));
}

#[test]
fn a_function_that_is_not_a_reference_is_never_reported() {
  for name in ["calc", "rgb", "url", "attr", "--var", "Var", "VAR", ""] {
    assert!(
      !names_unprefixed_property(&function(name, vec![node(NodeKind::Word, "foo")])),
      "`{name}()` should not be read as a custom-property reference"
    );
  }
}

#[test]
fn a_node_that_is_not_a_function_is_never_reported() {
  for kind in [
    NodeKind::Word,
    NodeKind::String,
    NodeKind::Div,
    NodeKind::Space,
    NodeKind::Comment,
    NodeKind::UnicodeRange,
  ] {
    assert!(
      !names_unprefixed_property(&node(kind, "var")),
      "a node of kind `{}` should not be read as a reference",
      kind.as_str()
    );
  }
}

/// The pass reads the top level of the list only, so a reference the scanner
/// nested inside another function is out of its reach whatever it names.
#[test]
fn the_pass_reads_the_top_level_only() {
  let mut ast = ValueParser::new("calc(var(foo) + 1px)");

  detect_unprefixed_custom_properties(&mut ast, "width");
}

/// An empty value scans to an empty list. Nothing to walk is not a rejection.
#[test]
fn an_empty_token_list_is_accepted() {
  let mut ast = ValueParser::new("");

  detect_unprefixed_custom_properties(&mut ast, "color");
}
