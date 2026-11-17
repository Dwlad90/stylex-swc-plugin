//! A compiler function, such as `keyframes`, called by the engine.
//!
//! A compiler function takes one evaluated argument and the build state, and
//! it writes into the build. The engine knows nothing of the build state, so
//! the fold opens a [scope](LiveState) that holds the state for as long as the
//! engine runs. The native function the engine calls reads the state from that
//! scope. Outside a scope, there is no state, and the native function throws.
//!
//! A function entry of the `env` option is called the same way. It reads
//! the option's function from the build state in scope.
//!
//! The decision and the safety argument are in ADR 0011.

use std::{cell::Cell, ptr};

use boa_engine::{
  Context, JsError, JsNativeError, JsResult, JsString, JsValue, NativeFunction,
  object::builtins::JsFunction,
};
use swc_core::atoms::Atom;

use stylex_state::{functions::StylexExprFn, state_manager::StateManager};

use super::outward::Outward;
use super::transport::answer_into_engine;
use super::{Ceilings, Decline, Depth};

/// What the native function throws when the engine calls it with no argument.
const NO_ARGUMENT: &str = "A StyleX function needs an argument.";

/// What the native function throws when its answer cannot go into the engine.
pub(in crate::evaluate) const NO_ENGINE_FORM: &str =
  "The answer of a StyleX function has no engine form.";

/// What the native function throws when it is called with no fold running.
const NO_BUILD_STATE: &str = "A StyleX function was called with no build state.";

/// What the native function of an `env` entry throws when the build state in
/// scope holds no function under its key.
const NO_ENV_FUNCTION: &str = "The env option has no function under this name in the build state.";

thread_local! {
  /// The build state of the fold that runs on this thread, or null.
  ///
  /// Set only by [`LiveState::enter`] and put back only by its drop.
  static LIVE_STATE: Cell<*mut StateManager> = const { Cell::new(ptr::null_mut()) };
}

/// The scope that makes the build state reachable from the engine.
///
/// It records the state that was in scope before, and puts it back when it is
/// dropped. So a fold that runs inside a compiler function (a nested fold)
/// opens its own scope, and the outer scope is in force again when the nested
/// fold ends. The drop also runs on an unwind.
pub(super) struct LiveState {
  previous: *mut StateManager,
}

impl LiveState {
  /// Makes `state` the build state of the engine until the scope is dropped.
  ///
  /// The caller must not use `state` while the scope is open, except through
  /// the scope. The fold keeps to this: it opens the scope before the engine
  /// runs and drops it before it reads its own state again.
  pub(super) fn enter(state: &mut StateManager) -> Self {
    let previous = LIVE_STATE.replace(state);

    Self { previous }
  }
}

impl Drop for LiveState {
  fn drop(&mut self) {
    LIVE_STATE.set(self.previous);
  }
}

/// Runs `run` on the build state in scope, or answers `None` where no scope is
/// open.
fn with_live_state<T>(run: impl FnOnce(&mut StateManager) -> T) -> Option<T> {
  let state = LIVE_STATE.get();

  // SAFETY: the pointer is null or it comes from `LiveState::enter`, which got
  // it from a `&mut StateManager` that the fold owns. All of this runs on one
  // thread, because the cell is thread-local. The fold does not use its own
  // borrow while the scope is open, so this reference is the only one in use.
  // The scope is dropped, and the pointer cleared, before that borrow ends. A
  // nested fold reborrows from this reference and puts the pointer back when
  // it ends, so two references are never in use at the same time.
  let state = unsafe { state.as_mut() };

  state.map(run)
}

/// The engine value for the compiler function `call`, read as `name`.
///
/// The name, the variable group and the bounds go with the function, because a
/// native function cannot reach the fold that made it.
pub(super) fn compiler_native(
  call: StylexExprFn,
  name: &Atom,
  ceilings: Ceilings,
  depth: Depth,
  var_group: &JsFunction,
  engine: &mut Context,
) -> JsValue {
  let captures = (JsString::from(name.as_str()), var_group.clone());

  let native = NativeFunction::from_copy_closure_with_captures(
    move |_, args, (name, var_group), engine| {
      run(call, name, ceilings, depth, var_group, args, engine)
    },
    captures,
  );

  JsValue::from(native.to_js_function(engine.realm()))
}

/// What the engine gets when it calls a compiler function.
///
/// The first argument goes back to the evaluator's own value, and the answer
/// goes back into the engine. Each step that fails throws, and the fold reads a
/// throw as a refusal.
fn run(
  call: StylexExprFn,
  name: &JsString,
  ceilings: Ceilings,
  depth: Depth,
  var_group: &JsFunction,
  args: &[JsValue],
  engine: &mut Context,
) -> JsResult<JsValue> {
  let method = Atom::from(name.to_std_string_lossy());

  let Some(argument) = args.first() else {
    return Err(thrown(Decline::rule(NO_ARGUMENT)));
  };

  let argument = Outward::new(&method, ceilings)
    .expr(argument, engine, depth)
    .map_err(thrown)?;

  let answer = with_live_state(|state| call(argument, state))
    .ok_or_else(|| thrown(Decline::rule(NO_BUILD_STATE)))?;

  answer_into_engine(&answer, engine, &method, depth, var_group).map_err(thrown)
}

/// The engine value for the function entry `key` of the `env` option.
///
/// The native function holds the key, not the function. The option's function
/// is a host closure, which the engine's collector cannot hold, so the native
/// function reads it from the options of the build state in scope each time it
/// is called. The `env` object a fold carries is always the one those options
/// hold.
pub(super) fn env_native(
  key: &str,
  ceilings: Ceilings,
  depth: Depth,
  var_group: &JsFunction,
  engine: &mut Context,
) -> JsValue {
  let captures = (JsString::from(key), var_group.clone());

  let native = NativeFunction::from_copy_closure_with_captures(
    move |_, args, (key, var_group), engine| run_env(key, ceilings, depth, var_group, args, engine),
    captures,
  );

  JsValue::from(native.to_js_function(engine.realm()))
}

/// What the engine gets when it calls a function entry of the `env` option.
///
/// Every argument goes back to the evaluator's own value, the option's function
/// runs on them, and its answer goes back into the engine. Each step that fails
/// throws, and the fold reads a throw as a refusal.
fn run_env(
  key: &JsString,
  ceilings: Ceilings,
  depth: Depth,
  var_group: &JsFunction,
  args: &[JsValue],
  engine: &mut Context,
) -> JsResult<JsValue> {
  let key = key.to_std_string_lossy();
  let method = Atom::from(key.as_str());

  let mut arguments = Vec::with_capacity(args.len());

  for argument in args {
    arguments.push(
      Outward::new(&method, ceilings)
        .expr(argument, engine, depth)
        .map_err(thrown)?,
    );
  }

  let function = with_live_state(|state| state.options.env.get(&key)?.as_function().cloned())
    .flatten()
    .ok_or_else(|| thrown(Decline::rule(NO_ENV_FUNCTION)))?;

  answer_into_engine(&function.call(arguments), engine, &method, depth, var_group).map_err(thrown)
}

/// A decline as the error the engine throws.
fn thrown(decline: Decline) -> JsError {
  let message = match decline {
    Decline::Rule(reason) => reason.into_owned(),
    Decline::NotACandidate => NO_ENGINE_FORM.to_string(),
  };

  JsError::from_native(JsNativeError::typ().with_message(message))
}

#[cfg(test)]
#[path = "tests/compiler_native_tests.rs"]
mod compiler_native_tests;
