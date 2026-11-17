//! A compiler function the engine calls with no fold running.
//!
//! No source reaches this: the native function exists only in a fold, and the
//! guard admits no shape that keeps it past the fold. The case builds it
//! directly.

use super::*;

use boa_engine::{Context, NativeFunction};
use stylex_structures::stylex_options::StyleXOptions;

/// A native function, standing for the variable group the answer is built
/// with. No answer here is a group.
fn no_group(context: &mut Context) -> JsFunction {
  NativeFunction::from_fn_ptr(|_, _, _| Ok(JsValue::undefined())).to_js_function(context.realm())
}

/// The ceilings and depth of a default build.
fn bounds() -> (Ceilings, Depth) {
  let state = StateManager::new(StyleXOptions::default());

  (
    Ceilings {
      characters: state.character_ceiling() as u64,
      entries: state.entry_ceiling() as u64,
    },
    Depth::full(state.evaluation_ceiling()),
  )
}

/// Calls the identity compiler function on `"b"`.
fn called(context: &mut Context) -> JsResult<JsValue> {
  called_with(&[JsValue::from(JsString::from("b"))], context)
}

/// Calls the identity compiler function on `args`.
fn called_with(args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
  let (ceilings, depth) = bounds();
  let group = no_group(context);
  let native = compiler_native(
    |expr, _| expr,
    &"own".into(),
    ceilings,
    depth,
    &group,
    context,
  );

  let Some(function) = native.as_callable() else {
    panic!("the compiler function is not callable");
  };

  function.call(&JsValue::undefined(), args, context)
}

#[test]
fn a_call_with_no_fold_running_throws() {
  let mut context = Context::default();

  match called(&mut context) {
    Ok(value) => panic!("expected a throw, got {value:?}"),
    Err(error) => assert!(error.to_string().contains(NO_BUILD_STATE), "got {error}"),
  }
}

/// With a scope open, the same call answers, and the scope closes when it is
/// dropped.
#[test]
fn a_scope_makes_the_state_reachable_until_it_is_dropped() {
  let mut context = Context::default();
  let mut state = StateManager::new(StyleXOptions::default());

  {
    let _in_scope = LiveState::enter(&mut state);

    match called(&mut context) {
      Ok(value) => assert_eq!(
        value
          .as_string()
          .map(|s| s.to_std_string_lossy())
          .as_deref(),
        Some("b")
      ),
      Err(error) => panic!("expected an answer, got {error}"),
    }
  }

  assert!(called(&mut context).is_err());
}

/// The guard admits no call with no argument, so only a direct call reaches
/// this throw.
#[test]
fn a_call_with_no_argument_throws() {
  let mut context = Context::default();
  let mut state = StateManager::new(StyleXOptions::default());
  let _in_scope = LiveState::enter(&mut state);

  match called_with(&[], &mut context) {
    Ok(value) => panic!("expected a throw, got {value:?}"),
    Err(error) => assert!(error.to_string().contains(NO_ARGUMENT), "got {error}"),
  }
}

/// Calls the native function of the `env` entry `key` on `"b"`.
fn env_called(key: &str, context: &mut Context) -> JsResult<JsValue> {
  let (ceilings, depth) = bounds();
  let group = no_group(context);
  let native = env_native(key, ceilings, depth, &group, context);

  let Some(function) = native.as_callable() else {
    panic!("the env function is not callable");
  };

  function.call(
    &JsValue::undefined(),
    &[JsValue::from(JsString::from("b"))],
    context,
  )
}

/// An `env` function entry called with no fold running has no build state to
/// read the function from, and throws. So does one whose key the options in
/// scope hold no function under.
#[test]
fn an_env_function_with_no_function_in_scope_throws() {
  let mut context = Context::default();

  match env_called("tint", &mut context) {
    Ok(value) => panic!("expected a throw, got {value:?}"),
    Err(error) => assert!(error.to_string().contains(NO_ENV_FUNCTION), "got {error}"),
  }

  let mut state = StateManager::new(StyleXOptions::default());
  let _in_scope = LiveState::enter(&mut state);

  match env_called("tint", &mut context) {
    Ok(value) => panic!("expected a throw, got {value:?}"),
    Err(error) => assert!(error.to_string().contains(NO_ENV_FUNCTION), "got {error}"),
  }
}
