//! Spike vehicle for issue 05: fold a static method call by handing it to a
//! JavaScript engine instead of matching its name against a table.
//!
//! This exists to be measured, not merged. It prints the call node back to
//! source, evaluates it in a lazily created engine reused for the whole
//! process, and converts the result back to an AST literal. Everything it
//! refuses is refused for a reason that keeps the measurement honest, not
//! because the shape is unfoldable in principle.

use std::{cell::RefCell, mem::ManuallyDrop};

use boa_engine::{Context, JsValue, Source, js_string};
use swc_core::{
  common::DUMMY_SP,
  ecma::{
    ast::{
      ArrayLit, ArrowExpr, BlockStmtOrExpr, CallExpr, Callee, Expr, ExprOrSpread, KeyValueProp,
      Lit, MemberExpr, MemberProp, Null, ObjectLit, Pat, Prop, PropName, PropOrSpread,
    },
    codegen::Config,
  },
};

use crate::shared::utils::log::build_code_frame_error::{CodeFrame, create_module, print_module};

thread_local! {
  /// One engine per thread, created on the first fold that needs it and reused
  /// for every later one. A file with no foldable method call never builds it.
  ///
  /// `ManuallyDrop` is not a convenience: the engine's garbage collector lives
  /// in a thread-local of its own, and the order two thread-locals are dropped
  /// in is not defined. Dropping this one after the collector's underflows a
  /// reference count, and that panic runs inside a destructor, which aborts the
  /// process instead of unwinding. Leaking one engine per thread at exit is the
  /// price of not aborting.
  static ENGINE: RefCell<Option<ManuallyDrop<Context>>> = const { RefCell::new(None) };
}

/// Folds `call` through the engine, or answers `None` to leave the existing
/// path in charge.
pub(crate) fn try_fold(call: &CallExpr) -> Option<Expr> {
  // Only a method call on a self-contained receiver is a candidate. Anything
  // else would need the surrounding scope, which the engine does not have.
  if !is_foldable_call(call) {
    return None;
  }

  let source = print_expr(&Expr::Call(call.clone()));

  ENGINE.with_borrow_mut(|slot| {
    let engine = slot.get_or_insert_with(|| ManuallyDrop::new(Context::default()));

    match engine.eval(Source::from_bytes(&source)) {
      Ok(value) => to_expr(&value, engine),
      // A throw is the reference implementation's behaviour too, and the
      // caller's existing refusal already reports it. Nothing to fold.
      Err(_) => None,
    }
  })
}

/// Whether an expression carries its whole value with it, so printing it
/// yields source the engine can evaluate on its own.
fn is_self_contained(expr: &Expr) -> bool {
  match expr {
    // A regex literal has no static value the evaluator can carry, and the
    // reference implementation refuses one as well.
    Expr::Lit(lit) => !matches!(lit, Lit::Regex(_)),
    Expr::Paren(paren) => is_self_contained(&paren.expr),
    Expr::Array(ArrayLit { elems, .. }) => elems.iter().all(|elem| match elem {
      Some(ExprOrSpread { spread: None, expr }) => is_self_contained(expr),
      // A hole is `undefined` and a spread needs the scope; both stay out.
      _ => false,
    }),
    Expr::Object(ObjectLit { props, .. }) => props.iter().all(|prop| match prop {
      PropOrSpread::Prop(prop) => match prop.as_ref() {
        Prop::KeyValue(KeyValueProp { key, value }) => {
          matches!(
            key,
            PropName::Ident(_) | PropName::Str(_) | PropName::Num(_)
          ) && is_self_contained(value)
        },
        _ => false,
      },
      PropOrSpread::Spread(_) => false,
    }),
    Expr::Unary(unary) => is_self_contained(&unary.arg),
    // A chained call: the receiver is itself a fold. Printing the whole chain
    // and evaluating it once is what makes `[…].map(…).join('-')` work, which
    // two separate method tables cannot agree on.
    Expr::Call(call) => is_foldable_call(call),
    _ => false,
  }
}

/// Whether a call is a method call this module can hand to the engine whole.
fn is_foldable_call(call: &CallExpr) -> bool {
  let Callee::Expr(callee) = &call.callee else {
    return false;
  };

  let Expr::Member(MemberExpr { obj, prop, .. }) = callee.as_ref() else {
    return false;
  };

  matches!(prop, MemberProp::Ident(_))
    && is_self_contained(obj)
    && call.args.iter().all(is_argument_foldable)
}

/// An argument is foldable when it is self-contained, or an arrow function
/// whose body only reads its own parameters — the callback shape `map`,
/// `filter` and `reduce` take.
fn is_argument_foldable(arg: &ExprOrSpread) -> bool {
  if arg.spread.is_some() {
    return false;
  }

  match arg.expr.as_ref() {
    Expr::Arrow(arrow) => is_closed_arrow(arrow),
    expr => is_self_contained(expr),
  }
}

/// Whether an arrow reads nothing but its own parameters. Anything else would
/// need a scope the engine does not have.
fn is_closed_arrow(arrow: &ArrowExpr) -> bool {
  let mut params = Vec::with_capacity(arrow.params.len());

  for param in &arrow.params {
    match param {
      Pat::Ident(ident) => params.push(ident.sym.to_string()),
      _ => return false,
    }
  }

  let BlockStmtOrExpr::Expr(body) = arrow.body.as_ref() else {
    return false;
  };

  free_identifiers_are_within(body, &params)
}

/// Walks an expression and answers whether every identifier it reads is one of
/// `params`. Deliberately narrow: the shapes it does not recognise are refused.
fn free_identifiers_are_within(expr: &Expr, params: &[String]) -> bool {
  match expr {
    Expr::Ident(ident) => params.contains(&ident.sym.to_string()),
    Expr::Lit(lit) => !matches!(lit, Lit::Regex(_)),
    Expr::Paren(paren) => free_identifiers_are_within(&paren.expr, params),
    Expr::Unary(unary) => free_identifiers_are_within(&unary.arg, params),
    Expr::Bin(bin) => {
      free_identifiers_are_within(&bin.left, params)
        && free_identifiers_are_within(&bin.right, params)
    },
    Expr::Cond(cond) => {
      free_identifiers_are_within(&cond.test, params)
        && free_identifiers_are_within(&cond.cons, params)
        && free_identifiers_are_within(&cond.alt, params)
    },
    // A member read on a parameter is the common `x.length` in a callback.
    Expr::Member(MemberExpr { obj, prop, .. }) => {
      free_identifiers_are_within(obj, params) && matches!(prop, MemberProp::Ident(_))
    },
    _ => false,
  }
}

/// Converts an engine value back to an AST literal, or answers `None` when it
/// has no representation the evaluator carries.
fn to_expr(value: &JsValue, engine: &mut Context) -> Option<Expr> {
  // Read through the accessors rather than matching variants: the engine's
  // value is nan-boxed by default and an enum only under a feature, and both
  // answer these.
  if let Some(truth) = value.as_boolean() {
    return Some(Expr::Lit(Lit::Bool(truth.into())));
  }

  if let Some(number) = value.as_number() {
    return Some(Expr::Lit(Lit::Num(number.into())));
  }

  if value.is_null() {
    return Some(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })));
  }

  if let Some(string) = value.as_string() {
    // The engine's strings are UTF-16 and `Lit::Str`'s atom is UTF-8, so an
    // unpaired surrogate cannot survive this step. Lossy conversion is the
    // rule issue 06 pins; the engine does not change it.
    return Some(Expr::Lit(Lit::Str(string.to_std_string_lossy().into())));
  }

  match value.as_object() {
    Some(object) if object.is_array() => {
      let length = object.get(js_string!("length"), engine).ok()?.as_number()? as u64;
      let mut elems = Vec::with_capacity(usize::try_from(length).ok()?);

      for index in 0..length {
        let element = object.get(index, engine).ok()?;

        elems.push(Some(ExprOrSpread {
          spread: None,
          expr: Box::new(to_expr(&element, engine)?),
        }));
      }

      Some(Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems,
      }))
    },
    // A plain object, a function, a symbol, `undefined` and a BigInt have no
    // literal the evaluator carries, so the existing path keeps them.
    _ => None,
  }
}

fn print_expr(expr: &Expr) -> String {
  print_module(
    &CodeFrame::new(),
    create_module(expr),
    Some(
      Config::default()
        .with_minify(true)
        .with_omit_last_semi(true),
    ),
  )
}
