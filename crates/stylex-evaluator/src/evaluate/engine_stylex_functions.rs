//! The StyleX functions the compile-time engine is allowed to call.
//!
//! The [folded function map] is not JavaScript: its values are this compiler's
//! own Rust functions. Two kinds of them can run in the engine.
//!
//! `firstThatWorks` answers from its arguments and nothing else. It reorders
//! the fallbacks it was handed and folds the CSS variables among them into one
//! `var()` chain, and it touches no state. It travels as a function of the
//! engine's own, built over the same [shared
//! core](crate::stylex_first_that_works) the evaluator's own path calls, so the
//! two cannot come to answer differently.
//!
//! A **compiler function** is a map entry that takes one evaluated argument and
//! the build state: `keyframes`, `positionTry` and the functions like them. It
//! writes a rule into the build and answers the name it hashed. It crosses as a
//! native function that converts its argument back to the evaluator's own
//! value, calls the Rust function with the live state, and converts the answer
//! into the engine. The live state reaches it through a scope that the fold
//! opens only while the engine runs. See ADR 0011 for the decision and for why
//! the scope is sound.
//!
//! What that buys is a callback: `a.map(x => firstThatWorks(x, 'serif'))` and
//! `a.map(() => keyframes({ … }))` are one JavaScript call per element, which is
//! the engine's job and not this compiler's. A rule is filed each time the
//! callback calls `keyframes`, and a callback that never runs files nothing.
//!
//! The namespace object also holds the `env` object, so `stylex.env` reads in
//! the engine as it does outside it.
//!
//! [folded function map]: https://github.com/Dwlad90/stylex-swc-plugin/blob/develop/crates/stylex-evaluator/CONTEXT.md

use std::rc::Rc;

use boa_engine::{
  Context, JsResult, JsString, JsValue, native_function::NativeFunctionPointer,
  object::builtins::JsArray,
};
use indexmap::IndexMap;
use swc_core::{
  atoms::Atom,
  ecma::ast::{Expr, Ident, IdentName, MemberExpr, MemberProp},
};

use stylex_ast::ast::convertors::normalize_expr;
use stylex_constants::constants::api_names::STYLEX_FIRST_THAT_WORKS;
use stylex_structures::named_import_source::ImportSources;
use stylex_structures::stylex_env::EnvEntry;

use crate::stylex_first_that_works::{
  Fallbacks, cut_to_css_variable_name, fold_fallback_chain, plan_fallbacks,
};
use stylex_state::{
  functions::{FunctionConfig, FunctionConfigType, FunctionMap, FunctionType, StylexExprFn},
  state_manager::{ImportKind, StateManager},
};

/// One StyleX function the engine may call without the build state: how the
/// module can name it, and what the engine runs in the compiler's place.
struct Callable {
  /// The import this function is, which is how a bare name is recognised — the
  /// module's import record says which StyleX function a name is, where a
  /// function map entry only says that some Rust function stands behind it and
  /// every one of those looks alike from here.
  kind: ImportKind,
  /// The property a namespace exposes it under, which is how `stylex.…` is
  /// recognised.
  property: &'static str,
  call: NativeFunctionPointer,
}

/// Every StyleX function the engine may call without the build state.
///
/// A set rather than a table standing in for the language: what it holds is the
/// answer to one question — does this function answer from its arguments alone —
/// and that question has no source to read it off. Adding a name to it is a
/// claim about that function.
const CALLABLE: [Callable; 1] = [Callable {
  kind: ImportKind::FirstThatWorks,
  property: STYLEX_FIRST_THAT_WORKS,
  call: first_that_works,
}];

/// A StyleX function the engine may call, and the one name the printed source
/// needs a value for to reach it.
pub(super) struct EngineCallable {
  /// The name that becomes a parameter of the printed arrow — the local name of
  /// an imported function, or the namespace it is read off.
  pub(super) name: Atom,
  /// What crosses under that name.
  pub(super) carried: Carried,
  /// The function the call itself names, which is what a refusal names: the
  /// namespace in front of it is not the half an author has to change.
  function_name: Atom,
  /// Whether the function is a compiler function, which reads its first
  /// argument and so needs one.
  pub(super) needs_an_argument: bool,
}

/// What crosses under the name of an [`EngineCallable`].
pub(super) enum Carried {
  /// `firstThatWorks(…)` through its own name: the function itself.
  Native(NativeFunctionPointer),
  /// `keyframes(…)` through its own name: a compiler function, which needs the
  /// live build state when it runs.
  Compiler(StylexExprFn),
  /// `stylex.firstThatWorks(…)` or `stylex.keyframes(…)`: the namespace, as an
  /// object the printed call reads its function off.
  ///
  /// The object holds every function of [`CALLABLE`], every compiler function
  /// of the namespace and its `env` object, not only the one this call named.
  /// So a fold that names two of them carries one object that holds both: the
  /// transport keeps one value per name, and a per-call object would let the
  /// second naming be dropped. Every other property of the real namespace is a
  /// function the engine may not call, and a call naming one is declined before
  /// anything is printed.
  Namespace(NamespaceMembers),
}

/// What the object a namespace crosses as holds, apart from the functions of
/// [`CALLABLE`].
pub(super) struct NamespaceMembers {
  /// The compiler functions, under the property each is read as.
  pub(super) functions: Vec<(Atom, StylexExprFn)>,
  /// The `env` object, under the property it is read as, where the namespace
  /// has one.
  pub(super) env: Option<(Atom, EnvMap)>,
}

/// The entries of the `env` option.
pub(super) type EnvMap = Rc<IndexMap<String, EnvEntry>>;

impl EngineCallable {
  /// The function's own spelling, which is what a refusal names.
  pub(super) fn function_name(&self) -> &str {
    &self.function_name
  }

  /// Every function of [`CALLABLE`], under the property a namespace holds it
  /// as — see [`Carried::Namespace`].
  pub(super) fn namespace_properties() -> impl Iterator<Item = (&'static str, NativeFunctionPointer)>
  {
    CALLABLE
      .iter()
      .map(|callable| (callable.property, callable.call))
  }
}

/// The StyleX function `callee` names, or `None` where it names none.
///
/// `functions` is the function map of the evaluation, which is where a
/// compiler function is found. It is keyed by name, as the dispatch below the
/// fold reads it.
pub(super) fn engine_callable(
  callee: &Expr,
  traversal_state: &StateManager,
  functions: &FunctionMap,
) -> Option<EngineCallable> {
  match callee_name(callee)? {
    CalleeName::Bare(ident) => {
      if let Some(callable) = native_by_name(&ident.sym, traversal_state) {
        return Some(EngineCallable {
          name: ident.sym.clone(),
          carried: Carried::Native(callable.call),
          function_name: callable.property.into(),
          needs_an_argument: false,
        });
      }

      let call = compiler_by_name(&ident.sym, functions)?;

      Some(EngineCallable {
        name: ident.sym.clone(),
        carried: Carried::Compiler(call),
        function_name: ident.sym.clone(),
        needs_an_argument: true,
      })
    },
    CalleeName::Member(namespace, property) => {
      let native = is_native_member(&namespace.sym, &property.sym, traversal_state);

      (native || is_compiler_member(&namespace.sym, &property.sym, functions)).then(|| {
        EngineCallable {
          name: namespace.sym.clone(),
          carried: Carried::Namespace(namespace_members(functions, &namespace.sym)),
          function_name: property.sym.clone(),
          needs_an_argument: !native,
        }
      })
    },
  }
}

/// Whether `callee` names a compiler function, as [`engine_callable`] reads
/// it.
///
/// The same answer as an [`EngineCallable`] that needs an argument, but it
/// builds nothing. The guard asks it of every call it searches, and most of
/// them name no StyleX function at all.
pub(super) fn is_compiler_callable(
  callee: &Expr,
  traversal_state: &StateManager,
  functions: &FunctionMap,
) -> bool {
  callee_name(callee).is_some_and(|name| match name {
    CalleeName::Bare(ident) => {
      native_by_name(&ident.sym, traversal_state).is_none()
        && compiler_by_name(&ident.sym, functions).is_some()
    },
    CalleeName::Member(namespace, property) => {
      !is_native_member(&namespace.sym, &property.sym, traversal_state)
        && is_compiler_member(&namespace.sym, &property.sym, functions)
    },
  })
}

/// The two ways a callee can name a StyleX function.
enum CalleeName<'e> {
  /// `keyframes(…)`: the name the function was imported under.
  Bare(&'e Ident),
  /// `stylex.keyframes(…)`: the namespace, and the property read off it.
  Member(&'e Ident, &'e IdentName),
}

/// How `callee` names a StyleX function, or `None` where its shape cannot.
///
/// A parenthesis is not a different callee, at either level: `(stylex.types)`
/// and `(stylex).types` name what `stylex.types` names. Only names are read
/// here, so nothing is printed back and unwrapping costs no spelling.
fn callee_name(callee: &Expr) -> Option<CalleeName<'_>> {
  match normalize_expr(callee) {
    Expr::Ident(ident) => Some(CalleeName::Bare(ident)),
    Expr::Member(MemberExpr {
      obj,
      prop: MemberProp::Ident(property),
      ..
    }) => Some(CalleeName::Member(
      normalize_expr(obj).as_ident()?,
      property,
    )),
    _ => None,
  }
}

/// The function of [`CALLABLE`] that the module imported as `name`.
fn native_by_name(name: &Atom, traversal_state: &StateManager) -> Option<&'static Callable> {
  CALLABLE
    .iter()
    .find(|callable| traversal_state.any_stylex_api_import_contains(&[callable.kind], name))
}

/// The compiler function the function map holds under `name`.
fn compiler_by_name(name: &Atom, functions: &FunctionMap) -> Option<StylexExprFn> {
  compiler_function(functions.identifiers.get(name)?)
}

/// Whether `namespace.property` is a function of [`CALLABLE`] read off a
/// StyleX import.
fn is_native_member(namespace: &Atom, property: &Atom, traversal_state: &StateManager) -> bool {
  traversal_state.is_regular_stylex_import(namespace)
    && CALLABLE
      .iter()
      .any(|callable| *property == callable.property)
}

/// Whether `namespace.property` is a compiler function of the namespace.
fn is_compiler_member(namespace: &Atom, property: &Atom, functions: &FunctionMap) -> bool {
  namespace_entries(functions, namespace)
    .any(|(name, entry)| name == property && compiler_function(entry).is_some())
}

/// The compiler functions and the `env` object a namespace holds, under the
/// property each is read as.
fn namespace_members(functions: &FunctionMap, namespace: &Atom) -> NamespaceMembers {
  let mut members = NamespaceMembers {
    functions: Vec::new(),
    env: None,
  };

  for (name, entry) in namespace_entries(functions, namespace) {
    match entry {
      FunctionConfigType::EnvObject(env) => members.env = Some((name.clone(), Rc::clone(env))),
      entry => members
        .functions
        .extend(compiler_function(entry).map(|call| (name.clone(), call))),
    }
  }

  members
}

/// The namespace name and what its object holds, where `member` reads the
/// `env` object off a namespace, as `stylex.env` does.
///
/// The engine reads the entries off the object, so a key the option does not
/// carry is `undefined`, as on any object.
///
/// Asked of every member read the guard walks, so the namespace is searched
/// for the one entry before anything is collected.
pub(super) fn an_env_read(
  member: &MemberExpr,
  functions: &FunctionMap,
) -> Option<(Atom, NamespaceMembers)> {
  let namespace = normalize_expr(&member.obj).as_ident()?;
  let MemberProp::Ident(property) = &member.prop else {
    return None;
  };

  namespace_entries(functions, &namespace.sym)
    .any(|(name, entry)| *name == property.sym && matches!(entry, FunctionConfigType::EnvObject(_)))
    .then(|| {
      (
        namespace.sym.clone(),
        namespace_members(functions, &namespace.sym),
      )
    })
}

/// Every entry a namespace holds, from both places the function map keeps
/// them: a namespace map registered as a name, and the member entries of an
/// import.
///
/// The member entries come last. Where both hold a property, the object the
/// engine gets keeps the last one, and the dispatch below the fold also reads
/// the member entry first. The import is found by a scan rather than by a key,
/// because a key would need an owned copy of the name for each call the guard
/// reads, and a module imports StyleX under one or two names.
fn namespace_entries<'f>(
  functions: &'f FunctionMap,
  namespace: &'f Atom,
) -> impl Iterator<Item = (&'f Atom, &'f FunctionConfigType)> {
  let folded = match functions.identifiers.get(namespace).map(Box::as_ref) {
    Some(FunctionConfigType::Map(map)) => Some(map),
    _ => None,
  };

  let members = functions
    .member_expressions
    .iter()
    .find(
      |(source, _)| matches!(source, ImportSources::Regular(name) if name == namespace.as_str()),
    )
    .map(|(_, members)| members);

  folded.into_iter().flat_map(|map| map.iter()).chain(
    members
      .into_iter()
      .flat_map(|map| map.iter().map(|(name, entry)| (name, entry.as_ref()))),
  )
}

/// The Rust function behind `entry`, where it is a compiler function.
///
/// Only a function that takes one evaluated argument qualifies. A function that
/// takes the argument as written has no value form the engine can hand it, and
/// the other kinds take arguments of their own shapes.
fn compiler_function(entry: &FunctionConfigType) -> Option<StylexExprFn> {
  match entry {
    FunctionConfigType::Regular(FunctionConfig {
      fn_ptr: FunctionType::StylexExprFn(call),
      takes_path: false,
    }) => Some(*call),
    _ => None,
  }
}

/// `firstThatWorks` over the engine's own values.
///
/// The ordering is [`plan_fallbacks`]'s, so this and the evaluator's own path
/// cannot disagree about which argument falls back to which. What differs is
/// only how a value is read: an argument is a variable reference when it *is* a
/// string matching one, where the evaluator's path reads every argument as text
/// first. So a number or an array among the fallbacks keeps its own form and
/// reaches CSS the way the language writes it, which is what the reference
/// implementation does too.
///
/// Text is read out of the engine's UTF-16 strings, which can hold an unpaired
/// surrogate that no Rust string can. Such a unit becomes the replacement
/// character here, as it does everywhere a folded string crosses back.
fn first_that_works(_this: &JsValue, args: &[JsValue], engine: &mut Context) -> JsResult<JsValue> {
  // Read once per argument rather than per question. Reading a string out of the
  // engine costs nothing and cannot fail, so there is no reason to be lazy about
  // it the way the evaluator's own path has to be.
  let names = args
    .iter()
    .map(|value| {
      let mut text = value.as_string()?.to_std_string_lossy();

      cut_to_css_variable_name(&mut text).then_some(text)
    })
    .collect::<Vec<_>>();

  // The chain's text is read the same way whichever shape holds it.
  let folded = |chain: &[usize], engine: &mut Context| -> JsResult<JsValue> {
    let mut parts = Vec::with_capacity(chain.len());

    for &index in chain {
      parts.push(match &names[index] {
        Some(name) => name.clone(),
        // The value the chain bottoms out on, read as the language reads it —
        // which is what puts it inside `var(…, here)`.
        None => args[index].to_string(engine)?.to_std_string_lossy(),
      });
    }

    Ok(JsValue::from(JsString::from(fold_fallback_chain(parts))))
  };

  match plan_fallbacks(args.len(), |index| names[index].is_some()) {
    Fallbacks::Reversed => Ok(JsValue::from(JsArray::from_iter(
      args.iter().rev().cloned(),
      engine,
    ))),
    Fallbacks::Chain(chain) => folded(&chain, engine),
    Fallbacks::ChainAndRest(chain, rest) => {
      let chain = folded(&chain, engine)?;
      let values = std::iter::once(chain)
        .chain(rest.iter().map(|&index| args[index].clone()))
        .collect::<Vec<_>>();

      Ok(JsValue::from(JsArray::from_iter(values, engine)))
    },
  }
}

// What the call answers for a fallback that throws when it is read, which no
// transported value can be.
#[cfg(test)]
#[path = "tests/throwing_fallback_tests.rs"]
mod throwing_fallback_tests;
