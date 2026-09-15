//! Tests for the flattening of one authored style object into the rules it
//! stands for.

use indexmap::IndexMap;
use stylex_evaluator::state::EvaluationState;
use stylex_state::{functions::FunctionMap, state_manager::StateManager};
use swc_core::ecma::ast::{KeyValueProp, ModuleItem, Stmt};

use super::{PreRules, flatten_raw_style_object_logic};
use crate::tests::support::{module, object};

/// The properties the object `code` spells, as the flattener is given them.
fn pairs(code: &str) -> Vec<KeyValueProp> {
  object(code)
    .props
    .into_iter()
    .filter_map(|prop| prop.prop().and_then(|prop| prop.key_value()))
    .collect()
}

/// The rules the object `code` flattens to, in the order it wrote them.
fn flatten(code: &str) -> IndexMap<String, PreRules> {
  flatten_in(code, &mut StateManager::default())
}

fn flatten_in(code: &str, traversal_state: &mut StateManager) -> IndexMap<String, PreRules> {
  flatten_raw_style_object_logic(
    &pairs(code),
    &mut vec![],
    &mut EvaluationState::default(),
    traversal_state,
    &FunctionMap::default(),
  )
}

/// A state that has read the declarations `code` spells.
fn state_declaring(code: &str) -> StateManager {
  let mut state = StateManager::default();

  for item in module(code).body {
    if let ModuleItem::Stmt(Stmt::Decl(swc_core::ecma::ast::Decl::Var(declaration))) = item {
      for declarator in declaration.decls {
        state.push_declaration(declarator);
      }
    }
  }

  state
}

fn keys_of(rules: &IndexMap<String, PreRules>) -> Vec<String> {
  rules.keys().cloned().collect()
}

#[test]
fn flattens_a_plain_declaration() {
  assert_eq!(keys_of(&flatten("{ color: 'red' }")), ["color"]);
}

/// A key spelled as a variable reference names the variable, which is how a
/// constant's placeholder reaches the declaration it belongs to.
#[test]
fn reads_a_variable_reference_key_as_the_variable_it_names() {
  assert_eq!(keys_of(&flatten("{ 'var(--x)': 'red' }")), ["--x"]);
}

/// A fallback list becomes one rule holding every value that survives.
#[test]
fn flattens_a_fallback_list_into_one_rule() {
  assert_eq!(keys_of(&flatten("{ color: ['red', 'blue'] }")), ["color"]);
}

/// A repeat in a fallback list says nothing the first one did not, so it goes.
#[test]
fn drops_a_repeat_from_a_fallback_list() {
  let rules = flatten("{ color: ['red', 'red'] }");

  assert_eq!(format!("{:?}", rules["color"]).matches("red").count(), 1);
}

/// A list holding only absent values declares nothing, and the property
/// survives carrying that absence so a later declaration of it is unset rather
/// than shadowed.
#[test]
fn flattens_a_list_that_empties_into_an_absence() {
  let rules = flatten("{ color: [null, null] }");

  assert!(matches!(rules["color"], PreRules::NullPreRule(_)));
}

/// Only a literal can stand in a fallback list: every entry becomes a
/// declaration of its own, and there is nothing to declare for anything else.
#[test]
#[should_panic(expected = "A style array value can only contain strings or numbers.")]
fn refuses_a_fallback_entry_that_is_not_a_literal() {
  flatten("{ color: ['red', name] }");
}

/// A template literal is read as the text it spells.
#[test]
fn reads_a_template_literal_as_its_text() {
  let rules = flatten("{ width: `10px` }");

  assert_eq!(keys_of(&rules), ["width"]);
  assert!(format!("{:?}", rules["width"]).contains("10px"));
}

/// A name is read as the value it was declared with.
#[test]
fn reads_a_name_as_the_value_it_was_declared_with() {
  let mut state = state_declaring("const red = 'red';");
  let rules = flatten_in("{ color: red }", &mut state);

  assert_eq!(keys_of(&rules), ["color"]);
  assert!(format!("{:?}", rules["color"]).contains("red"));
}

/// A name the compiler cannot resolve names no value, so there is nothing to
/// declare and the call is refused.
#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex")]
fn refuses_a_name_it_cannot_resolve() {
  flatten("{ color: red }");
}

/// `undefined` is a value rather than a name that failed to resolve, and a
/// style value position refuses it for not being a style value.
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn refuses_an_undefined_value() {
  flatten("{ color: undefined }");
}

/// An arithmetic expression is read as the number it comes to.
#[test]
fn reads_an_arithmetic_expression_as_its_number() {
  let rules = flatten("{ width: 5 + 5 }");

  assert_eq!(keys_of(&rules), ["width"]);
  assert!(format!("{:?}", rules["width"]).contains("10"));
}

/// A call names no static value.
#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex")]
fn refuses_a_call_as_a_value() {
  flatten("{ color: makeColour() }");
}

/// A condition holds the declarations that apply under it, keyed by the
/// property they declare.
#[test]
fn flattens_a_condition_into_the_property_it_declares() {
  assert_eq!(
    keys_of(&flatten("{ color: { default: 'red', ':hover': 'blue' } }")),
    ["color"]
  );
}

/// An empty object declares nothing at all, and nothing after it does either.
#[test]
fn flattens_an_empty_object_into_nothing() {
  assert!(flatten("{ color: {} }").is_empty());
}

/// A spread inside a condition names no property, and a property that is not a
/// key-value pair declares no value. Neither is a condition the compiler can
/// read.
#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex")]
fn refuses_a_spread_inside_a_condition() {
  flatten("{ color: { ...rest } }");
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a stylex")]
fn refuses_a_method_inside_a_condition() {
  flatten("{ color: { default() { return 'red' } } }");
}
