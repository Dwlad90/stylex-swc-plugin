//! Tests for the refusals the compiler makes before it reads a StyleX call.

use stylex_state::{evaluate_result_value::EvaluateResultValue, state_manager::StateManager};
use swc_core::ecma::ast::{CallExpr, Expr};

use super::{
  assert_valid_keyframes, assert_valid_position_try, assert_valid_properties,
  assert_valid_view_transition_class, contains_call, is_bound_create_expr,
  validate_conditional_styles, validate_theme_variables,
};
use crate::tests::support::{expr, ts_expr};

/// The folded value `code` spells, as the evaluator would hand it over.
fn folded(code: &str) -> EvaluateResultValue {
  EvaluateResultValue::Expr(expr(code))
}

/// The call `code` spells, and the expression it was read out of.
///
/// Read as TypeScript, because one of the shapes a call is reached through --
/// a non-null assertion -- is spelled only there.
fn call_of(code: &str) -> (Expr, CallExpr) {
  let parsed = ts_expr(code);

  let call = match find_call(&parsed) {
    Some(call) => call.clone(),
    None => panic!("the fixture {code} holds no call"),
  };

  (parsed, call)
}

fn find_call(expr: &Expr) -> Option<&CallExpr> {
  match expr {
    Expr::Call(call) => Some(call),
    Expr::Member(member) => find_call(&member.obj),
    Expr::Array(array) => array
      .elems
      .iter()
      .flatten()
      .find_map(|element| find_call(&element.expr)),
    Expr::TsNonNull(inner) => find_call(&inner.expr),
    Expr::Paren(paren) => find_call(&paren.expr),
    Expr::OptChain(chain) => match chain.base.as_ref() {
      swc_core::ecma::ast::OptChainBase::Member(member) => find_call(&member.obj),
      swc_core::ecma::ast::OptChainBase::Call(call) => find_call(&call.callee),
    },
    Expr::Seq(seq) => seq.exprs.iter().find_map(|expr| find_call(expr)),
    _ => None,
  }
}

#[test]
fn a_keyframe_is_an_object_of_objects() {
  assert_valid_keyframes(
    &folded("{ from: { opacity: 0 }, to: { opacity: 1 } }"),
    &mut StateManager::default(),
  );
  assert_valid_keyframes(&folded("{}"), &mut StateManager::default());
}

/// Each step of a keyframe holds the declarations of that step, so a step that
/// is not an object declares nothing.
#[test]
#[should_panic(expected = "Every frame within a keyframes() call must be an object")]
fn refuses_a_keyframe_step_that_is_not_an_object() {
  assert_valid_keyframes(&folded("{ from: 'red' }"), &mut StateManager::default());
}

#[test]
#[should_panic(expected = "keyframes() can only accept an object")]
fn refuses_a_keyframes_argument_that_is_not_an_object() {
  assert_valid_keyframes(&folded("'red'"), &mut StateManager::default());
}

/// A value the compiler could not fold is not a keyframe it can read.
#[test]
#[should_panic(expected = "Only static values are allowed inside of a keyframes() call.")]
fn refuses_a_keyframes_argument_that_did_not_fold() {
  assert_valid_keyframes(&EvaluateResultValue::Null, &mut StateManager::default());
}

#[test]
fn a_position_try_argument_is_an_object() {
  assert_valid_position_try(&folded("{ top: 0 }"), &mut StateManager::default());
  assert_valid_view_transition_class(&folded("{}"), &mut StateManager::default());
}

#[test]
#[should_panic(expected = "positionTry() can only accept an object")]
fn refuses_a_position_try_argument_that_is_not_an_object() {
  assert_valid_position_try(&folded("'red'"), &mut StateManager::default());
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a positionTry() call.")]
fn refuses_a_position_try_argument_that_did_not_fold() {
  assert_valid_position_try(&EvaluateResultValue::Null, &mut StateManager::default());
}

#[test]
#[should_panic(expected = "viewTransitionClass() can only accept an object")]
fn refuses_a_view_transition_class_argument_that_is_not_an_object() {
  assert_valid_view_transition_class(&folded("1"), &mut StateManager::default());
}

#[test]
#[should_panic(expected = "Only static values are allowed inside of a viewTransitionClass() call.")]
fn refuses_a_view_transition_class_argument_that_did_not_fold() {
  assert_valid_view_transition_class(&EvaluateResultValue::Null, &mut StateManager::default());
}

#[test]
fn accepts_only_the_keys_a_call_names() {
  assert_valid_properties(
    &folded("{ top: 0 }"),
    &["top", "left"],
    "only top or left",
    &mut StateManager::default(),
  );
}

#[test]
#[should_panic(expected = "only top or left")]
fn refuses_a_key_a_call_does_not_name() {
  assert_valid_properties(
    &folded("{ bottom: 0 }"),
    &["top", "left"],
    "only top or left",
    &mut StateManager::default(),
  );
}

/// A value that is not an object names no keys to check, so there is nothing to
/// refuse here and the reader beside this one reports it.
#[test]
fn checks_no_key_of_a_value_that_names_none() {
  assert_valid_properties(
    &folded("'red'"),
    &["top"],
    "only top",
    &mut StateManager::default(),
  );
  assert_valid_properties(
    &EvaluateResultValue::Null,
    &["top"],
    "only top",
    &mut StateManager::default(),
  );
}

/// A call bound to a variable is bound however the author reached it: through a
/// namespace, through an optional read, through a non-null assertion, or from
/// inside an array of styles.
#[test]
fn reads_a_call_through_the_shapes_that_reach_it() {
  for code in [
    "create({}).root",
    "create({})?.root",
    "create({})!.root",
    "create({}).root.nested",
    "create({})?.root.nested",
    "create({})?.root?.nested",
  ] {
    let (expression, call) = call_of(code);

    assert!(
      is_bound_create_expr(&expression, &call),
      "{code} was not read as a bound call"
    );
  }

  let (expression, call) = call_of("[create({})]");

  assert!(is_bound_create_expr(&expression, &call));
}

/// An optional call is a call of its own, not a read on the one inside it.
#[test]
fn reads_no_call_through_the_shapes_that_do_not_bind_one() {
  for code in [
    "create({})",
    "create({})?.()",
    "(create({}) , 1)",
    "create({}), styles.root",
    "create({}), makeStyles?.().root",
  ] {
    let (expression, call) = call_of(code);

    assert!(
      !is_bound_create_expr(&expression, &call),
      "{code} was read as a bound call"
    );
  }
}

/// A read on some other call is not a read on this one, whichever way that
/// other call is written.
///
/// The expressions here stand beside the call rather than holding it, which is
/// what the module-level reader hands this predicate: every top-level
/// expression of the file is offered for one call. The case above asks about a
/// call each expression holds, so the two cannot share a fixture even where
/// they spell the same chain.
#[test]
fn reads_no_call_through_a_chain_rooted_elsewhere() {
  let (_, call) = call_of("create({}).root");

  for code in [
    // An optional call of another expression: a call of its own, and not the
    // one asked about.
    "makeStyles?.().root",
    // A name: the chain ends at something that is no call at all.
    "styles.root",
  ] {
    assert!(
      !is_bound_create_expr(&ts_expr(code), &call),
      "{code} was read as a bound call"
    );
  }
}

/// Identity is the place the call was written, not its shape. Two calls that
/// read the same are two calls, and only the one that is there is found.
#[test]
fn tells_two_calls_that_read_the_same_apart() {
  let pair = expr("[create({}).root, create({}).root]");

  let elements = match &pair {
    Expr::Array(array) => &array.elems,
    other => panic!("the fixture is not an array: {other:?}"),
  };

  let first = match elements.first().and_then(Option::as_ref) {
    Some(element) => match find_call(&element.expr) {
      Some(call) => call.clone(),
      None => panic!("the first element holds no call"),
    },
    None => panic!("the fixture holds no first element"),
  };

  let second = match elements.get(1).and_then(Option::as_ref) {
    Some(element) => (*element.expr).clone(),
    None => panic!("the fixture holds no second element"),
  };

  assert!(!contains_call(&second, &first));
}

/// The first property of the object `code` spells.
fn first_pair(code: &str) -> swc_core::ecma::ast::KeyValueProp {
  match crate::tests::support::object(code).props.into_iter().next() {
    Some(prop) => match prop.prop().and_then(|prop| prop.key_value()) {
      Some(key_value) => key_value,
      None => panic!("the fixture {code} holds no key-value pair"),
    },
    None => panic!("the fixture {code} holds no property"),
  }
}

/// A condition holds declarations, a nested condition, or a name the compiler
/// resolves later.
#[test]
fn a_condition_holds_a_value_or_a_nested_condition() {
  for code in [
    "{ ':hover': { default: 'red' } }",
    "{ default: 'red' }",
    "{ '@media print': { default: 'red' } }",
    "{ 'var(--x)': 'red' }",
    "{ default: name }",
    "{ default: ['red', 'blue'] }",
  ] {
    validate_conditional_styles(&first_pair(code), &[], &mut StateManager::default());
  }
}

/// A key that names neither a pseudo selector nor an at-rule names no
/// condition, so the declarations under it would never apply.
#[test]
#[should_panic(expected = "Invalid pseudo or at-rule")]
fn refuses_a_key_that_names_no_condition() {
  validate_conditional_styles(
    &first_pair("{ hover: 'red' }"),
    &[],
    &mut StateManager::default(),
  );
}

/// One condition written twice on one path is a declaration that shadows
/// itself, which is a mistake rather than a rule.
#[test]
#[should_panic(expected = "The same pseudo selector or at-rule cannot be used more than once.")]
fn refuses_a_condition_that_is_already_in_the_path() {
  validate_conditional_styles(
    &first_pair("{ ':hover': 'red' }"),
    &[":hover".to_owned()],
    &mut StateManager::default(),
  );
}

/// A condition holds a declaration, not a shape with no CSS value.
#[test]
#[should_panic(expected = "A style value can only contain an array, string or number.")]
fn refuses_a_condition_holding_a_value_that_is_not_a_style_value() {
  validate_conditional_styles(
    &first_pair("{ ':hover': () => 1 }"),
    &[],
    &mut StateManager::default(),
  );
}

/// A theme overrides a variable group, which is named by the hash the group
/// carries. A value that is not a group cannot be overridden.
#[test]
#[should_panic(expected = "Can only override variables theme created with defineVars().")]
fn refuses_a_theme_target_that_is_not_a_variable_group() {
  validate_theme_variables(&folded("1"), &StateManager::default());
}

#[test]
#[should_panic(expected = "Can only override variables theme created with defineVars().")]
fn refuses_a_theme_target_that_carries_no_group_hash() {
  validate_theme_variables(&folded("{ color: 'red' }"), &StateManager::default());
}

/// A group hash that spells no text names no group.
#[test]
#[should_panic(expected = "Can only override variables theme created with defineVars().")]
fn refuses_a_group_hash_that_is_empty() {
  validate_theme_variables(
    &folded("{ __varGroupHash__: '' }"),
    &StateManager::default(),
  );
}

/// A group hash the compiler could not fold names no group either.
#[test]
#[should_panic(expected = "Can only override variables theme created with defineVars().")]
fn refuses_a_group_hash_that_did_not_fold() {
  validate_theme_variables(
    &folded("{ __varGroupHash__: name }"),
    &StateManager::default(),
  );
}

#[test]
fn reads_the_group_hash_a_theme_target_carries() {
  let key_value = validate_theme_variables(
    &folded("{ __varGroupHash__: 'x568ih9' }"),
    &StateManager::default(),
  );

  assert_eq!(
    key_value.value.as_lit().and_then(|lit| match lit {
      swc_core::ecma::ast::Lit::Str(text) => text.value.as_str().map(str::to_owned),
      _ => None,
    }),
    Some("x568ih9".to_owned())
  );
}
