//! Folds a self-contained method call by evaluating it, rather than by
//! matching its name against a table of methods.
//!
//! A table is finite by construction, so the method it does not list is the
//! next bug report. Handing the call to a JavaScript engine instead covers
//! `String.prototype`, `Array.prototype` and `Object.prototype` at once, along
//! with the `Math` and `Object` statics, and covers a chain for free: the
//! receiver of a call is itself a candidate, so
//! `["1px","solid"].concat(["red"]).join(" ")` is printed and evaluated once
//! instead of being split across two tables that have to agree.
//!
//! Everything therefore rests on the guard in front of the engine. What it
//! lets through is answered by the language; what it holds back is a boundary
//! with a reason, and each one is named where it is applied below.
//!
//! The guard asks whether every leaf of an expression *resolves to a value the
//! bridge can carry*, not whether it is written out — so giving a value a name
//! does not change whether the call on it folds. A name the guard resolved
//! becomes a parameter of a printed arrow and its value an argument to it, so
//! `s.toLowerCase()` is handed over as `(s) => s.toLowerCase()` called with the
//! string `s` holds. See [`Transport`] for why the value travels beside the
//! source rather than inside it.
//!
//! A fold answers one of two things: the value, or the rule that refused it.
//! There is no silent refusal — a call the guard recognised and declined says
//! which rule declined it, rather than falling through to the caller's
//! `Unsupported expression: CallExpression`. A call the guard never recognised
//! is not a refusal at all: it is simply not this module's, and the dispatch
//! below it decides what happens instead.

use std::{borrow::Cow, cell::RefCell, mem::ManuallyDrop};

use boa_engine::{
  Context, JsError, JsObject, JsResult, JsString, JsValue, Source, js_string,
  object::builtins::JsArray, property::PropertyKey,
};
use swc_core::{
  atoms::{Atom, Wtf8Atom},
  common::DUMMY_SP,
  ecma::{
    ast::{
      ArrayLit, ArrowExpr, BlockStmtOrExpr, CallExpr, Callee, Expr, ExprOrSpread, ExprStmt, Ident,
      KeyValueProp, Lit, MemberExpr, MemberProp, Module, ModuleItem, Null, ObjectLit, Pat, Prop,
      PropName, PropOrSpread, Stmt, Tpl,
    },
    codegen::Config,
  },
};

use stylex_ast::ast::convertors::atom_utf16_length;
use stylex_ast::ast::factories::{
  create_arrow_expression_with_params, create_binding_ident, create_ident,
  create_ident_key_value_prop, create_object_lit,
};
use stylex_constants::constants::evaluation_errors::{
  SPREAD_ELEMENT, amplification_inside_a_callback, array_length_too_large,
  bound_value_has_too_many_entries, bound_value_too_large, engine_did_not_start, engine_threw,
  escaping_property, expression_too_deep, folded_string_too_large, locale_sensitive_method,
  not_a_function, numeric_literal_receiver, object_size_too_large, unbounded_amplified_length,
  uncallable_printed_fold, uncoercible_value, unfoldable_fold_result, unfoldable_static,
};
use stylex_js::coercions::{self, is_global_spelled_as_an_identifier};
use stylex_js::helpers::{is_invalid_method, is_valid_callee};
use stylex_utils::number::to_js_string;

use super::{evaluate_cached, evaluate_result_vec_to_array_expr, helpers::get_binding};
use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{functions::FunctionMap, state::EvaluationState, state_manager::StateManager},
  utils::{
    common::order_own_keys,
    log::build_code_frame_error::{CodeFrame, print_module},
  },
};

/// Methods whose answer depends on locale data the engine does not carry.
///
/// It resolves them against the root locale, so `"i".toLocaleUpperCase("tr")`
/// comes back `I` where the language says `İ`, and a collation comparison
/// answers by code point. Folding those would put a wrong value in the
/// stylesheet, which is worse than not folding at all.
///
/// `toLocaleString` is here for a receiver the name cannot be separated from.
/// On an object it delegates to `toString` and carries no locale at all, and
/// refusing it costs one of the object methods that would otherwise fold. On a
/// number it formats — `(1234.5).toLocaleString("de-DE")` is `1.234,5` in the
/// language and `1234.5` here — and which receiver a chain will produce is not
/// knowable before evaluating it. One name cannot be both admitted and refused,
/// so it is refused, and the object case is the price.
const LOCALE_SENSITIVE_METHODS: [&str; 4] = [
  "localeCompare",
  "toLocaleLowerCase",
  "toLocaleUpperCase",
  "toLocaleString",
];

/// Property names that walk off the value that was written and onto the
/// language's function graph.
///
/// `constructor` on a literal is `String`, whose own `constructor` is
/// `Function`, which compiles a string into a function. So
/// `"".constructor.constructor("return Date.now()").call()` is a chain every
/// other rule here admits — two named property reads on a literal, then a call
/// — whose answer is a different number on every build, and whose body can
/// assign to `String.prototype` in an engine every later fold shares. `call`,
/// `apply` and `bind` are what turn an unapplied function back into a call, so
/// they are refused with it.
const ESCAPING_PROPERTIES: [&str; 4] = ["constructor", "call", "apply", "bind"];

/// The one property name that is not a property when it is written as one.
///
/// `{ __proto__: x }` sets the object's prototype, so the object the source
/// describes carries no own property called `__proto__`. This evaluator's own
/// object form keeps it as an ordinary key, which is why the two directions of
/// this bridge have to agree on it explicitly: an expression written out reaches
/// the engine as text and the language drops it, and a value the guard resolved
/// is built property by property and would keep it.
const PROTOTYPE_KEY: &str = "__proto__";

/// Methods whose result length is set by an argument rather than by the
/// receiver, and so are the only ones a single small argument can blow up.
const LENGTH_AMPLIFYING_METHODS: [&str; 3] = ["repeat", "padStart", "padEnd"];

/// How long a string one of those may be asked to build.
///
/// The engine bounds recursion and VM stack depth by default, and this module
/// bounds loop iterations itself ([`MAX_LOOP_ITERATIONS`]), because Boa's own
/// default for that is `u64::MAX`. What none of them bound is allocation:
/// growth inside a native builtin is not a counted loop. So
/// `"x".repeat(200000000)` is a typo that folds — agreeing with the language —
/// at several gigabytes of resident memory, and a compiler that dies there is
/// worse than one that declines the fold. A million UTF-16 code units is two
/// megabytes in the engine and instant, and no CSS value is a megabyte long.
const MAX_AMPLIFIED_LENGTH: f64 = 1_000_000.0;

/// How many entries a folded array or object may carry back.
///
/// [`MAX_AMPLIFIED_LENGTH`] bounds a string, and a string can still be turned
/// into one element per code unit: `"x".repeat(999999).split("")` folds to an
/// array literal of a million AST nodes, which costs far more as a tree than it
/// did as text. An object pays the same price per property, so it is bounded by
/// the same number rather than by one of its own. A fallback list in a real
/// declaration holds a handful of values, and a nested style object a handful of
/// conditions, so this is generous by three orders of magnitude and still
/// refuses that.
const MAX_FOLDED_ENTRIES: u64 = 10_000;

/// How deeply nested an expression this module will hand to the engine, and how
/// deeply nested a value it will carry back.
///
/// Nesting is not free for whoever parses it. The engine's parser descends
/// through a nested array literal recursively and, measured on a debug build,
/// overflows its stack somewhere around a hundred levels — which aborts the
/// process from inside an evaluation whose whole contract is that it may fail.
/// Refusing deeper input is what turns that crash back into a diagnostic, and
/// the same bound applies on the way out because a loop inside the engine can
/// nest a value deeper than any expression that reached it.
///
/// One level is spent per step of the walk, and a leaf spends one too, so this
/// admits an expression nested 31 levels and refuses the 32nd. The number is
/// deliberately the same as
/// `stylex_structures::evaluation_depth::DEFAULT_MAX_EVALUATION_DEPTH`, for the
/// same reason and with a wide margin over the measured limit rather than an
/// exact match to it. It is *not* that setting: raising the ceiling on the
/// evaluator's own recursion says nothing about the engine's stack, and a
/// project that raised it must not lose the diagnostic here.
const MAX_ENGINE_NESTING: usize = 32;

/// How many loop iterations an evaluation may run.
///
/// Boa's own default is `u64::MAX` — `RuntimeLimits::loop_iteration` documents
/// it as no limit — so a loop is unbounded until it is said to be bounded. The
/// guard refuses the shapes that reach a loop body, and this is the second
/// answer behind that one: a bound the engine enforces whatever the guard let
/// through. Ten million iterations of an empty loop is well under a second, and
/// no folded CSS value is reached by counting that far.
const MAX_LOOP_ITERATIONS: u64 = 10_000_000;

/// What the engine answers when anything asks a function for its source text.
///
/// A function's `ToString` is its source, and this compiler has none to give: an
/// arrow reaches the engine as this module's own minified printing of it, not as
/// the text the author wrote, so `String(() => 'x')` would fold to a spelling no
/// other build produces — and a class name is a hash of the declaration text.
/// The reference compiler answers such a call with the source of a wrapper from
/// inside its own evaluator, which is no better.
///
/// So the source is taken away and every conversion that would read one throws,
/// which the fold reports as a refusal. A function reached only to be *called*
/// is untouched, which is what `String({ toString: () => 'red' })` needs, and is
/// the whole of the difference between a function used as a value and one used
/// as a method.
///
/// Assigned rather than defined, because `Function.prototype.toString` is a
/// writable property; and assigned once when the engine is built, because a fold
/// cannot reach it afterwards — every route from a value to a prototype is a
/// refused property read.
const NO_FUNCTION_SOURCE: &str = concat!(
  "Function.prototype.toString = function () {",
  " throw new TypeError('A function has no source text at compile time.')",
  " };"
);

/// Whether `name` is one of the names in `list`.
///
/// The lists above are three to four entries, so a scan beats any structure
/// that would have to hash first, and one function keeps every call site from
/// spelling the double reference its own way. Takes a `&str` so a name read off
/// a member property and one read out of a string literal ask the same question
/// rather than one of them reaching past the helper.
fn lists(list: &[&str], name: &str) -> bool {
  list.contains(&name)
}

thread_local! {
  /// One engine per thread, created on the first fold that needs it and reused
  /// for every later one. A file with no foldable method call never builds it.
  ///
  /// Reuse is what makes the guard load-bearing rather than merely tidy: a fold
  /// that reached a prototype would be read by every later fold in the build,
  /// including one in another file, so the boundaries below are what keeps one
  /// engine safe to share. Reuse also costs: the engine interns each distinct
  /// source it is handed and never reclaims it, measured at roughly half a
  /// kilobyte per distinct folded call site, which a real corpus keeps in the
  /// low megabytes for the life of the process.
  ///
  /// `ManuallyDrop` is not a convenience: the engine's garbage collector lives
  /// in a thread-local of its own, and the order two thread-locals are dropped
  /// in is not defined. Dropping this one after the collector's underflows a
  /// reference count, and that panic runs inside a destructor, which aborts the
  /// process instead of unwinding. Leaking one engine per thread at exit is the
  /// price of not aborting.
  static ENGINE: RefCell<Option<ManuallyDrop<Context>>> = const { RefCell::new(None) };
}

/// The words a [refused fold](../../../../../CONTEXT.md) hands the caller,
/// ready to be raised as a deopt.
///
/// Borrowed where the rule has one fixed sentence and owned where it names the
/// method or the limit it refused on, so the common path allocates nothing.
pub(crate) type Refusal = Cow<'static, str>;

/// Why the guard did not hand a call to the engine — the outcome, where
/// [`Refusal`] is the half of it an author reads.
///
/// Only one of the two arms is a refusal in the glossary's sense. A call that
/// is not a candidate was decided from syntax before any rule ran: nothing
/// declined it, so nothing is reported and the dispatch below this module owns
/// the call. A rule that fired declined a call the guard did recognise, and
/// carries the words for it.
enum Decline {
  NotACandidate,
  Rule(Refusal),
}

impl Decline {
  fn rule(reason: impl Into<Refusal>) -> Self {
    Self::Rule(reason.into())
  }
}

/// Where a bare identifier in an expression is allowed to get its value from.
#[derive(Clone, Copy)]
enum Scope<'a> {
  /// The module the expression was written in, read through the evaluator. A
  /// name it resolves to a carryable value becomes a parameter of the printed
  /// arrow; a name it cannot is not this module's call.
  Module,
  /// A callback's own parameters, which the engine binds itself when it invokes
  /// the callback with values from the receiver. A name that is not one of them
  /// is still resolved against the module.
  Params(&'a [Atom]),
}

/// How much nesting is left, and the one refusal spent at the bottom of it.
///
/// Every walk across the bridge counts the same budget for the same reason —
/// the guard's walk in, the conversion of a value it resolved, and the
/// conversion out all recurse on the bare thread stack — so they share the
/// counter and the sentence rather than keeping three that could drift apart.
///
/// Reaching the bottom is a refusal and not a "not mine", which is the one
/// place the two are worth telling apart. This bound is the engine parser's
/// stack, so it does not move when a project raises the evaluator's ceiling,
/// and under a raised ceiling the older path would fold what this declines —
/// so the refusal costs a fold. It is taken because the two ceilings no longer
/// carry the same number, and a bound this module owns has to answer in this
/// module's words: handing the shape back instead makes which sentence an
/// author reads depend on which of two disagreeing ceilings they crossed.
/// Handing it back is at least safe now — the nested array that reached the
/// older `join` refuses rather than panicking — so what remains is the
/// diagnostic, and Ticket 11 owns unifying the two ceilings.
#[derive(Clone, Copy)]
struct Depth(usize);

impl Depth {
  /// A full budget, at the start of a walk or a conversion.
  const FULL: Self = Self(MAX_ENGINE_NESTING);

  /// One level in, or the depth refusal at the bound — so depth is answered the
  /// same way as any other rule the guard applies.
  fn descend(self) -> Result<Self, Decline> {
    match self.0 {
      0 => Err(Decline::rule(expression_too_deep(MAX_ENGINE_NESTING))),
      left => Ok(Self(left - 1)),
    }
  }
}

/// What the guard carries as it walks: where a bare identifier may come from,
/// and how much nesting is left before the expression is refused as too deep.
///
/// Nothing here records *where* in the expression the walk is. Every rule below
/// reads the call in front of it and nothing else, so a static, a chain link and
/// the call the caller asked about are all answered the same way — which is what
/// one guard walk is for.
#[derive(Clone, Copy)]
struct Guard<'a> {
  scope: Scope<'a>,
  depth: Depth,
}

impl<'a> Guard<'a> {
  /// The guard one level in.
  fn descend(self) -> Result<Self, Decline> {
    Ok(Self {
      depth: self.depth.descend()?,
      ..self
    })
  }

  /// The same remaining depth, with a callback's parameters now in scope.
  fn binding(self, params: &'a [Atom]) -> Self {
    Self {
      scope: Scope::Params(params),
      ..self
    }
  }
}

/// The names the guard resolved and the values it resolved them to, ready to
/// cross the bridge inward.
///
/// The value travels *beside* the printed source rather than inside it: the
/// expression is printed as an arrow taking these names as parameters, and the
/// values are passed as arguments. Substituting a literal into the text instead
/// would reprint and reparse the whole value at every use site, and a value with
/// no literal spelling could not cross at all — where an argument keeps the
/// printed text the size of the expression however large the value is.
///
/// The author's own name is the parameter name. Nothing has to be rewritten for
/// that, and a callback parameter of the same name shadows it in the printed
/// arrow exactly as it does in the module the author wrote.
///
/// Chosen over registering the names on the engine because the engine is one
/// leaked instance per thread, shared by every file that thread compiles: a name
/// left behind or shadowed there would be a cross-file correctness bug.
#[derive(Default)]
struct Transport {
  params: Vec<Atom>,
  values: Vec<Carried>,
  totals: Totals,
}

impl Transport {
  /// Records what `name` resolves to, or does nothing where the name is already
  /// carried.
  ///
  /// One parameter per name however often the expression reads it, because a
  /// repeated parameter is a syntax error in the arrow this is printed into. The
  /// second reading resolves to the same value — the evaluator memoises it — so
  /// dropping it loses nothing. A scan rather than a set, because an expression
  /// carries a handful of names and hashing them would cost more than comparing
  /// them.
  ///
  /// The value is converted here rather than when the engine is in hand, so a
  /// value past a bound refuses before anything is printed or built. What comes
  /// out is [`Carried`]: the shapes the bridge carries, measured, and not yet
  /// the engine's own values.
  fn bind(&mut self, name: &Atom, value: &EvaluateResultValue) -> Result<(), Decline> {
    if self.params.contains(name) {
      return Ok(());
    }

    let carried = Inward {
      name,
      totals: &mut self.totals,
    }
    .value(value, Depth::FULL)?;

    self.params.push(name.clone());
    self.values.push(carried);

    Ok(())
  }

  /// The carried values as the arguments the printed arrow is called with.
  ///
  /// Built with the engine in hand, because an array and an object are engine
  /// objects and there is no way to make one without it. Everything that could
  /// refuse was answered when the value was carried, so this step only builds.
  fn arguments(&self, engine: &mut Context, method: &Atom) -> Result<Vec<JsValue>, Decline> {
    let mut arguments = Vec::with_capacity(self.values.len());

    for value in &self.values {
      arguments.push(to_js(value, engine, method)?);
    }

    Ok(arguments)
  }
}

/// A resolved value on its way into the engine, in the shapes the bridge
/// carries.
///
/// A tree of the bridge's own rather than the evaluator's value or the engine's.
/// The evaluator's can be walked with no engine in hand, which is what lets
/// every bound be answered before one is built; the engine's cannot exist until
/// it is. So a value crosses in two steps, and this is what stands between them.
///
/// Refused in both directions, as stated rules rather than omissions: function
/// configurations, callbacks, the environment object, an unresolved theme
/// reference, and the AST-keyed map variants. A theme reference therefore
/// crosses only as the `var(--…)` string it already resolved to, because
/// resolving it is what mutates compiler state and that happens before the
/// bridge.
enum Carried {
  Str(Wtf8Atom),
  Num(f64),
  Bool(bool),
  Null,
  List(Vec<Carried>),
  Object(Vec<(Wtf8Atom, Carried)>),
}

/// How much one fold has already promised to copy into the engine.
///
/// Counted across every name a fold carries rather than per name, because what
/// is about to be copied is all of them: eight names each one unit under the
/// limit is eight times the limit, and a per-value check would wave every one of
/// them through.
///
/// Two counts, because a value costs in two ways that do not stand in for each
/// other. A thousand empty arrays hold no text at all and are still a thousand
/// values to build; a single string is one entry and can be a megabyte.
#[derive(Default)]
struct Totals {
  /// UTF-16 code units of every string and key carried — the unit the engine's
  /// own strings are measured in, and the one that bounds a folded string on the
  /// way out.
  units: u64,
  /// Array elements and object properties carried.
  entries: u64,
}

/// What the inward conversion carries: the name whose value is crossing, so a
/// bound can say which binding to shorten, and the totals it counts against.
struct Inward<'a> {
  name: &'a Atom,
  totals: &'a mut Totals,
}

impl Inward<'_> {
  /// One value the evaluator answered as the bridge's own, or the reason it does
  /// not cross.
  ///
  /// A shape the bridge does not carry is not a refusal. The dispatch below the
  /// fold owns those values and answers for them, and a rule fired here would
  /// stop it ever being asked.
  fn value(&mut self, value: &EvaluateResultValue, depth: Depth) -> Result<Carried, Decline> {
    match value {
      EvaluateResultValue::Expr(expr) => self.expr(expr, depth),
      // The evaluator answers an array either as a list of its own or as the
      // array literal it was written as. Both are one array here, which is the
      // whole of why the two dispatch arms that carried the array methods could
      // disagree about which names they knew: they were answering for the same
      // value in two shapes.
      EvaluateResultValue::Vec(items) => {
        let inner = depth.descend()?;

        self.count(items.len())?;

        let mut list = Vec::with_capacity(items.len());

        for item in items {
          list.push(self.value(item, inner)?);
        }

        Ok(Carried::List(list))
      },
      _ => Err(Decline::NotACandidate),
    }
  }

  /// One evaluated expression as the bridge's own value.
  ///
  /// One level of the budget per node, leaves included, exactly as the guard's
  /// own walk spends it — the two walk the same shapes and would otherwise
  /// disagree about how deep the same value is.
  fn expr(&mut self, expr: &Expr, depth: Depth) -> Result<Carried, Decline> {
    let inner = depth.descend()?;

    match expr {
      Expr::Lit(Lit::Str(text)) => Ok(Carried::Str(self.text(&text.value)?)),
      Expr::Lit(Lit::Num(number)) => Ok(Carried::Num(number.value)),
      Expr::Lit(Lit::Bool(truth)) => Ok(Carried::Bool(truth.value)),
      Expr::Lit(Lit::Null(_)) => Ok(Carried::Null),
      Expr::Array(ArrayLit { elems, .. }) => {
        self.count(elems.len())?;

        let mut list = Vec::with_capacity(elems.len());

        for elem in elems {
          // A hole is `undefined` and a spread was refused where it was
          // written, so neither reaches a fold: the reference compiler refuses
          // a method call on both, and folding a hole as anything at all would
          // write a value the source does not describe.
          let Some(ExprOrSpread { spread: None, expr }) = elem else {
            return Err(Decline::NotACandidate);
          };

          list.push(self.expr(expr, inner)?);
        }

        Ok(Carried::List(list))
      },
      Expr::Object(ObjectLit { props, .. }) => {
        self.count(props.len())?;

        let mut entries = Vec::with_capacity(props.len());

        for prop in props {
          let PropOrSpread::Prop(prop) = prop else {
            return Err(Decline::NotACandidate);
          };

          let Prop::KeyValue(KeyValueProp { key, value }) = prop.as_ref() else {
            return Err(Decline::NotACandidate);
          };

          let key = self.key(key)?;

          // `__proto__` written as a plain key sets the prototype rather than a
          // member, so the object the source describes has no own property of
          // that name. The evaluator keeps it as one, so it is dropped here —
          // where an expression written out reaches the engine as text and the
          // language drops it for us. Both paths then answer alike, and alike is
          // what the reference compiler answers.
          if key == PROTOTYPE_KEY {
            continue;
          }

          entries.push((key, self.expr(value, inner)?));
        }

        Ok(Carried::Object(entries))
      },
      _ => Err(Decline::NotACandidate),
    }
  }

  /// A property name as the string the language reads it as.
  fn key(&mut self, key: &PropName) -> Result<Wtf8Atom, Decline> {
    let name = match key {
      PropName::Ident(name) => Wtf8Atom::from(&*name.sym),
      PropName::Str(name) => name.value.clone(),
      // A numeric key names the property its own string form spells, read by
      // the conversion every other number-to-string in this compiler uses
      // rather than by a spelling of its own.
      PropName::Num(number) => Wtf8Atom::from(to_js_string(number.value).as_str()),
      // A computed key was evaluated before it reached a value, and a BigInt is
      // not a value this bridge carries in any position.
      _ => return Err(Decline::NotACandidate),
    };

    self.text(&name)
  }

  /// One string, counted against what this fold may copy in.
  fn text(&mut self, text: &Wtf8Atom) -> Result<Wtf8Atom, Decline> {
    // Saturating because the sum exists to be refused on, and a wrapped one
    // would admit.
    self.totals.units = self
      .totals
      .units
      .saturating_add(atom_utf16_length(text) as u64);

    if self.totals.units as f64 > MAX_AMPLIFIED_LENGTH {
      return Err(Decline::rule(bound_value_too_large(
        self.name,
        MAX_AMPLIFIED_LENGTH,
      )));
    }

    Ok(text.clone())
  }

  /// Some elements or properties, counted against the same budget.
  ///
  /// Counted before they are walked, so a list past the bound refuses without
  /// first converting every entry in it.
  fn count(&mut self, entries: usize) -> Result<(), Decline> {
    self.totals.entries = self.totals.entries.saturating_add(entries as u64);

    if self.totals.entries > MAX_FOLDED_ENTRIES {
      return Err(Decline::rule(bound_value_has_too_many_entries(
        self.name,
        MAX_FOLDED_ENTRIES,
      )));
    }

    Ok(())
  }
}

/// One carried value as the engine's own, under the name of the method whose
/// fold it is about to be an argument to.
///
/// Needs no bound of its own: what it walks was bounded when it was carried, in
/// text, in entries and in nesting, and nothing has been added to it since.
fn to_js(carried: &Carried, engine: &mut Context, method: &Atom) -> Result<JsValue, Decline> {
  let value = match carried {
    Carried::Str(text) => JsValue::from(carry_string(text)),
    Carried::Num(number) => JsValue::from(*number),
    Carried::Bool(truth) => JsValue::from(*truth),
    Carried::Null => JsValue::null(),
    Carried::List(items) => {
      let mut values = Vec::with_capacity(items.len());

      for item in items {
        values.push(to_js(item, engine, method)?);
      }

      JsValue::from(JsArray::from_iter(values, engine))
    },
    Carried::Object(entries) => {
      let object = JsObject::with_object_proto(engine.intrinsics());

      for (key, value) in entries {
        let value = to_js(value, engine, method)?;

        // A fresh ordinary object takes a data property without complaint, so
        // the throw is unreachable — and answered rather than asserted, because
        // this runs inside an evaluation whose whole contract is that it may
        // fail.
        read(method, || {
          object.create_data_property_or_throw(carry_string(key), value, engine)
        })?;
      }

      JsValue::from(object)
    },
  };

  Ok(value)
}

/// What the walk needs that the expression does not carry: the evaluator, so a
/// name can be resolved to the value it holds, and the transport the resolved
/// values are collected into.
struct Reader<'a> {
  state: &'a mut EvaluationState,
  traversal_state: &'a mut StateManager,
  fns: &'a FunctionMap,
  transport: Transport,
}

impl Reader<'_> {
  /// The value `ident` resolves to, or `None` where it resolves to nothing.
  ///
  /// Resolved through the evaluator's own memoised entry point rather than by a
  /// second reading of its own, so a binding this fold reads is the binding
  /// every other position reads — including the disqualifications that live
  /// there: a reassigned binding, one mutated in place, and one read above its
  /// own declaration all answer nothing here because they answer nothing there.
  ///
  /// The read is a speculation and is marked as one, so nothing it refuses is
  /// left behind: the evaluation's confidence and deopt are put back, and the
  /// memo withholds the refusal. A name this module could not read is not a
  /// refusal — the dispatch below owns the call, evaluates the same name itself,
  /// and has to find both the state and the sentence it would have had.
  fn resolve(&mut self, ident: &Ident) -> Option<EvaluateResultValue> {
    let reference = Expr::Ident(ident.clone());
    let Reader {
      state,
      traversal_state,
      fns,
      ..
    } = self;

    // The putting-back is the whole contract, so it belongs to the two states
    // that own the fields rather than to a sequence written out here that a
    // later edit could return past.
    speculate(state, traversal_state, |state, traversal_state| {
      evaluate_cached(&reference, state, traversal_state, fns)
    })
  }
}

/// Runs `read` as a [speculative
/// read](../../../../../CONTEXT.md#speculative-read), and puts back everything it
/// refused.
///
/// `None` where the read refused, so a caller cannot mistake a refusal for a
/// value: the confidence that says which it was is gone by the time the caller
/// sees the answer, which is the point.
///
/// The two flags are saved and restored rather than cleared, so a fold reached
/// from inside another speculation stays inside it for as long as that one lasts.
/// Written as one function taking a closure rather than as a save, a call and a
/// restore at the call site, because the restore is the contract and a `return`
/// added between the halves would silently drop it.
fn speculate(
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  read: impl FnOnce(&mut EvaluationState, &mut StateManager) -> Option<EvaluateResultValue>,
) -> Option<EvaluateResultValue> {
  let confident = state.confident;
  let deopt_path = state.deopt_path.take();
  let deopt_reason = state.deopt_reason.take();
  let speculating = traversal_state.speculating;

  traversal_state.speculating = true;

  let read = read(state, traversal_state);

  let value = match state.confident {
    true => read,
    false => None,
  };

  traversal_state.speculating = speculating;
  state.confident = confident;
  state.deopt_path = deopt_path;
  state.deopt_reason = deopt_reason;

  value
}

/// One resolved string as the engine's own string type.
///
/// Two readings of the same atom, because a JavaScript string literal can hold
/// an unpaired surrogate and no Rust `str` can. The engine's strings are UTF-16,
/// so the ill-formed reading carries such a value across *exactly* — where the
/// outward direction has to substitute the replacement character, since a
/// `Lit::Str` is what it has to land in. A valid value takes the direct reading
/// rather than being taken apart into code units it would only be rebuilt from.
fn carry_string(value: &Wtf8Atom) -> JsString {
  match value.as_str() {
    Some(text) => JsString::from(text),
    None => JsString::from(&value.to_ill_formed_utf16().collect::<Vec<u16>>()[..]),
  }
}

/// Whether a *name* may hold this value — a question of its own, and narrower
/// than what [`Carried`] carries. Narrower, too, than the set of receivers the
/// dispatch below hands straight back to a refusal: that one is about which
/// prototypes this module now owns whole, and an object is not among them
/// because an object receiver is still where a function map's own methods are
/// looked up.
///
/// A string, an array, a plain object, a number and a boolean — the primitives
/// and the two composites the bridge was proved on, in the one shape that
/// matters to the guard: a value the engine can be handed and the source can go
/// on reading as itself.
///
/// A number is here because the statics need it. `Math.round(BASE / 4)` is the
/// ordinary way to write one, and `BASE` is a name; refusing a name that holds a
/// number would have refused every arithmetic fold whose operands were not
/// written out. It follows that `const n = 255; n.toString(16)` folds too, which
/// is what the reference compiler does. The refusal that has to survive is about
/// how the receiver was *written*, not what it holds — a number literal in the
/// source is still refused, in [`receiver_is_a_written_number`], because the
/// reference compiler throws on one.
///
/// A boolean comes with it rather than for a case of its own. Once a name may
/// hold a number, a boolean is the only primitive left outside, and there is no
/// sentence that would say why: it prints, it crosses, and its prototype folds
/// like any other. Leaving it out would be a table of one — the shape this whole
/// module exists to delete.
fn is_a_carryable_receiver(value: &EvaluateResultValue) -> bool {
  matches!(
    value,
    EvaluateResultValue::Vec(_)
      | EvaluateResultValue::Expr(
        Expr::Lit(Lit::Str(_) | Lit::Num(_) | Lit::Bool(_)) | Expr::Array(_) | Expr::Object(_)
      )
  )
}

/// Folds `call` through the engine.
///
/// `None` leaves the existing path in charge: the call is not one this module
/// handles, which is a question of syntax and of what the module's names hold,
/// not a refusal. `Some` is the fold's own answer — the value, or the rule that
/// declined it.
pub(crate) fn try_fold(
  call: &CallExpr,
  state: &mut EvaluationState,
  traversal_state: &mut StateManager,
  fns: &FunctionMap,
) -> Option<Result<EvaluateResultValue, Refusal>> {
  let guard = Guard {
    scope: Scope::Module,
    depth: Depth::FULL,
  };

  let mut reader = Reader {
    state,
    traversal_state,
    fns,
    transport: Transport::default(),
  };

  match fold(call, guard, &mut reader) {
    Ok(value) => Some(Ok(value)),
    Err(Decline::NotACandidate) => None,
    Err(Decline::Rule(reason)) => Some(Err(reason)),
  }
}

fn fold(
  call: &CallExpr,
  guard: Guard,
  reader: &mut Reader,
) -> Result<EvaluateResultValue, Decline> {
  let admitted = admit_call(call, guard, reader)?;
  let method = admitted.name();

  let source = print_fold(call, &reader.transport.params);

  ENGINE.with_borrow_mut(|slot| {
    // Taken, not borrowed in place. A panic unwinding out of the engine is
    // caught at the NAPI boundary and the process carries on, so an engine left
    // in the slot would be reused by every later fold with its VM stack
    // abandoned mid-frame. Taking it means an unwind leaves the slot empty and
    // the next fold builds a fresh engine; the abandoned one leaks, which is
    // what `ManuallyDrop` already makes it do at thread exit.
    let mut engine = match slot.take() {
      Some(engine) => engine,
      None => new_engine()?,
    };

    let outward = Outward {
      method,
      depth: Depth::FULL,
    };

    let applied = match admitted {
      Admitted::Global(global) => admit_an_applied_global(global, &mut engine),
      Admitted::Method(_) => Ok(()),
    };

    let folded = applied
      .and_then(|()| reader.transport.arguments(&mut engine, method))
      .and_then(|arguments| apply(&source, &arguments, &mut engine, outward))
      .and_then(|value| to_value(&value, &mut engine, outward));

    *slot = Some(engine);

    folded
  })
}

/// Compiles the printed arrow and calls it with the transported values.
///
/// Two steps rather than one evaluation *when there is something to pass*,
/// because the values cross as arguments rather than as text. Every step can
/// throw and all of them are answered the same way: a throw is an answer, not a
/// failure of this module — the language throws on `[].reduce(f)` too — so the
/// engine's own sentence is what the author reads rather than a generic refusal
/// standing in for it.
///
/// A fold that resolved no name is evaluated directly, which is the whole of why
/// this branches. Wrapping it in an arrow and invoking that arrow costs a
/// function object and a VM frame on top of the expression itself: measured, it
/// is +44% on the cheapest leg of the benchmark and +24% on the chain, paid by
/// exactly the folds that gained nothing from the transport, since every
/// expression that folded before this work resolves no name. The branch is on one
/// question — did the guard resolve anything — and both arms hand the same
/// expression to the same engine, so it is not the two-tables-that-must-agree
/// this module exists to remove.
///
/// The compiled value is a function by construction, so the refusal for one that
/// is not stands in for a broken invariant rather than for anything an author can
/// write. It is a refusal all the same: this runs inside an evaluation whose
/// whole contract is that it may fail, where an assertion would abort a build
/// that a deopt would only leave to the runtime.
fn apply(
  source: &str,
  arguments: &[JsValue],
  engine: &mut Context,
  outward: Outward,
) -> Result<JsValue, Decline> {
  let evaluated = engine
    .eval(Source::from_bytes(source))
    .map_err(|error| threw(outward.method, &error))?;

  if arguments.is_empty() {
    return Ok(evaluated);
  }

  let Some(callable) = evaluated.as_callable() else {
    return Err(Decline::rule(uncallable_printed_fold(outward.method)));
  };

  callable
    .call(&JsValue::undefined(), arguments, engine)
    .map_err(|error| threw(outward.method, &error))
}

/// Whether this thread is holding an engine — the observable half of "built on
/// first use and never before".
///
/// Test-only, and reading the slot rather than counting constructions, because
/// what the claim is about is whether an engine exists after an input the fold
/// declined. Paired with [`forget_engine`], since a test asserting an engine was
/// *not* built has to start from a thread that has none.
#[cfg(test)]
pub(super) fn holds_an_engine() -> bool {
  ENGINE.with_borrow(|slot| slot.is_some())
}

/// Drops this thread's engine reference without dropping the engine, which is
/// what the slot's `ManuallyDrop` already does at thread exit and for the same
/// reason: the collector lives in a thread-local of its own and the drop order
/// between the two is not defined.
#[cfg(test)]
pub(super) fn forget_engine() {
  ENGINE.with_borrow_mut(|slot| {
    slot.take();
  });
}

/// A context with the one runtime limit its default leaves open, and without
/// the one thing the language provides that this compiler cannot: function
/// source text.
///
/// Answers a refusal rather than asserting, because the assignment runs inside
/// an evaluation whose whole contract is that it may fail — and because an
/// engine that kept function source would fold a spelling no other build
/// produces, which is worse than declining the fold.
fn new_engine() -> Result<ManuallyDrop<Context>, Decline> {
  let mut engine = Context::default();

  engine
    .runtime_limits_mut()
    .set_loop_iteration_limit(MAX_LOOP_ITERATIONS);

  engine
    .eval(Source::from_bytes(NO_FUNCTION_SOURCE))
    .map_err(|error| Decline::rule(engine_did_not_start(&error.to_string())))?;

  Ok(ManuallyDrop::new(engine))
}

/// What the outward bridge carries as it converts a value back: the method
/// whose answer it is reading, so a refusal can name it, and how much nesting
/// is left before the value is refused as too deep.
///
/// The method is carried because the engine's own sentence does not always
/// name it — `"abc".unsupported()` throws `not a callable function`, which
/// tells an author nothing the code frame has not already shown them.
#[derive(Clone, Copy)]
struct Outward<'a> {
  method: &'a Atom,
  depth: Depth,
}

impl Outward<'_> {
  /// The bridge one level in.
  ///
  /// A value nested deeper than the guard admits on the way in can still be
  /// built on the way out, by a loop the engine ran rather than by syntax the
  /// author wrote. Bounded for the reason the input is bounded: the conversion
  /// recurses on the bare thread stack.
  fn descend(self) -> Result<Self, Decline> {
    Ok(Self {
      depth: self.depth.descend()?,
      ..self
    })
  }
}

/// A throw, in the engine's own words under this compiler's naming of the call
/// that produced it.
///
/// Takes the method rather than a direction, because both directions throw: a
/// getter runs while a value is read back out, and a property is written while
/// one is carried in. What an author needs from either is the same two things,
/// and neither of them is which way the value was going.
fn threw(method: &Atom, error: &JsError) -> Decline {
  Decline::rule(engine_threw(method, &error.to_string()))
}

/// Whether every value `expr` needs is written into it, bound by the guard's
/// scope, or resolvable from the module — so the printed arrow and the values
/// beside it are together something the engine can evaluate alone.
///
/// One walk serves both questions this module asks — the receiver's and the
/// arguments' — so a shape accepted in one position cannot silently be refused
/// in the other. Deliberately narrow: a shape it does not recognise is not a
/// candidate.
fn admit_value(expr: &Expr, guard: Guard, reader: &mut Reader) -> Result<(), Decline> {
  let inner = guard.descend()?;

  match expr {
    // A name in the callback's own parameters is bound by the engine when it
    // invokes the callback. Any other name is asked of the module, and becomes a
    // parameter of the printed arrow carrying the value it resolved to.
    Expr::Ident(ident) => {
      if let Scope::Params(params) = guard.scope
        && params.contains(&ident.sym)
      {
        return Ok(());
      }

      // `undefined`, `NaN` and `Infinity` are values the grammar has no literal
      // for, so an author writes them as names and they reach the guard as
      // names. The engine holds them, so they are printed and the language
      // answers — the same arrangement a global receiver takes, and for the same
      // reason. Only where the module bound nothing of the name, in which case
      // the binding is resolved like any other below.
      if is_global_spelled_as_an_identifier(ident)
        && get_binding(expr, reader.traversal_state).is_none()
      {
        return Ok(());
      }

      match reader.resolve(ident) {
        Some(value) if is_a_carryable_receiver(&value) => reader.transport.bind(&ident.sym, &value),
        _ => Err(Decline::NotACandidate),
      }
    },
    // A regular expression and a BigInt have no value this evaluator carries,
    // and neither does the reference implementation fold one.
    Expr::Lit(Lit::Regex(_) | Lit::BigInt(_)) => Err(Decline::NotACandidate),
    Expr::Lit(_) => Ok(()),
    Expr::Paren(paren) => admit_value(&paren.expr, inner, reader),
    Expr::Unary(unary) => admit_value(&unary.arg, inner, reader),
    Expr::Bin(bin) => {
      admit_value(&bin.left, inner, reader)?;
      admit_value(&bin.right, inner, reader)
    },
    Expr::Cond(cond) => {
      admit_value(&cond.test, inner, reader)?;
      admit_value(&cond.cons, inner, reader)?;
      admit_value(&cond.alt, inner, reader)
    },
    // A template literal is written-out syntax whose holes are values in their
    // own right, so it is walked like any other composite and printed back
    // exactly as it was written. A tagged one is a different node and is not
    // here: its tag is a call this module cannot see the body of.
    Expr::Tpl(Tpl { exprs, .. }) => {
      for hole in exprs {
        admit_value(hole, inner, reader)?;
      }

      Ok(())
    },
    // A property read: `x.length` inside a callback, `({a:1}).a` as a receiver,
    // and `p[0]` on an element a callback was handed.
    //
    // The property is answered before the receiver is walked, because the name
    // alone decides it and the walk now resolves bindings — so a read that no
    // receiver could make safe must not cost a resolution first.
    Expr::Member(MemberExpr { obj, prop, .. }) => {
      match prop {
        MemberProp::Ident(name) => {
          if lists(&ESCAPING_PROPERTIES, &name.sym) {
            return Err(Decline::rule(escaping_property(&name.sym)));
          }
        },
        // A computed key is a value in its own right, so it is walked as one.
        // The escaping-property rule is applied to a key written as a string,
        // because `x['constructor']` spells the read `x.constructor` spells.
        //
        // A key whose value the guard cannot read is still admitted, and that
        // is a boundary rather than a hole: what such a read can reach is a
        // function, which is refused on the way out and cannot be applied on
        // the way in — a call whose method name is computed is not a candidate
        // at all, so there is no step from the function to its result.
        MemberProp::Computed(key) => {
          if let Expr::Lit(Lit::Str(name)) = key.expr.as_ref()
            && let Some(name) = name.value.as_str()
            && lists(&ESCAPING_PROPERTIES, name)
          {
            return Err(Decline::rule(escaping_property(name)));
          }

          admit_value(&key.expr, inner, reader)?;
        },
        // A private name belongs to a class body, which no value a fold carries
        // has.
        MemberProp::PrivateName(_) => return Err(Decline::NotACandidate),
      }

      admit_value(obj, inner, reader)
    },
    Expr::Array(ArrayLit { elems, .. }) => {
      for elem in elems {
        match elem {
          Some(ExprOrSpread { spread: None, expr }) => admit_value(expr, inner, reader)?,
          // A hole is `undefined` and a spread needs the scope; both stay out.
          _ => return Err(Decline::NotACandidate),
        }
      }

      Ok(())
    },
    // A key and a value written out, which is a value the engine can be handed
    // whole. `__proto__` is the one key that is not: written as a plain
    // property it sets the prototype rather than a member, so the receiver the
    // engine sees is not the object the source appears to describe. It is left
    // in because the reference implementation folds it identically and every
    // route off the prototype is refused above — not because the walk models
    // it.
    Expr::Object(ObjectLit { props, .. }) => {
      for prop in props {
        let PropOrSpread::Prop(prop) = prop else {
          // A spread is a value in its own right and the language does the
          // spreading, so the operand is walked and the printed source keeps the
          // spread exactly as it was written.
          let PropOrSpread::Spread(spread) = prop else {
            return Err(Decline::NotACandidate);
          };

          admit_value(&spread.expr, inner, reader)?;

          continue;
        };

        let Prop::KeyValue(KeyValueProp { key, value }) = prop.as_ref() else {
          return Err(Decline::NotACandidate);
        };

        if !matches!(
          key,
          PropName::Ident(_) | PropName::Str(_) | PropName::Num(_)
        ) {
          return Err(Decline::NotACandidate);
        }

        admit_value(value, inner, reader)?;
      }

      Ok(())
    },
    // A chained call: the receiver is itself a fold. Printing the whole chain
    // and evaluating it once is what makes `[…].map(…).join('-')` work, which
    // two separate method tables cannot agree on.
    Expr::Call(call) => admit_call(call, inner, reader).map(|_| ()),
    // An arrow is a value the language can hold and call: the callback `map` and
    // `filter` take, and the own `toString` an object converts through. It has
    // no *string* form here — the engine is built without function source text,
    // so a conversion that would read one refuses. See [`NO_FUNCTION_SOURCE`].
    Expr::Arrow(arrow) => admit_arrow(arrow, inner, reader),
    _ => Err(Decline::NotACandidate),
  }
}

/// What the guard admitted, and the name a refusal or a throw is reported
/// under.
///
/// The two arms are the two ways a native function is reached: a method on a
/// receiver, and a global applied as a function. They are told apart because
/// only the second can name something that is not a function at all — `Math` is
/// a valid callee because its methods fold, which says nothing about whether the
/// name itself can be applied.
#[derive(Clone, Copy)]
enum Admitted<'a> {
  Method(&'a Atom),
  Global(&'a Atom),
}

impl<'a> Admitted<'a> {
  /// The method or global the call names.
  fn name(self) -> &'a Atom {
    match self {
      Admitted::Method(name) | Admitted::Global(name) => name,
    }
  }
}

/// Refuses an applied global that is not a function.
///
/// Asked of the language rather than of a list of names: the global object holds
/// the value, and the value says whether it can be applied. The engine's own
/// sentence for applying one is `not a callable function`, which names neither
/// the global nor the mistake, so the refusal is this compiler's.
///
/// Asked with the engine in hand and so after the guard, because it is the one
/// rule here that cannot be answered from the source. Asked of the outermost call
/// only: a global applied in the middle of a chain reads the engine's own throw,
/// and there is no sentence about a chain worth preferring to it.
fn admit_an_applied_global(name: &Atom, engine: &mut Context) -> Result<(), Decline> {
  let global_object = engine.global_object();
  let value = read(name, || {
    global_object.get(JsString::from(name.as_str()), engine)
  })?;

  match value.is_callable() {
    true => Ok(()),
    false => Err(Decline::rule(not_a_function(name))),
  }
}

/// The name a bare identifier names as a global the module never bound, or
/// `None` where it is not one.
///
/// One question asked in the two positions a global appears in — the receiver of
/// a static and the callee of an applied global — so a name that is the global
/// in one cannot be a binding in the other.
///
/// A locally-declared shadow is the module's own value and is resolved like any
/// other name: measured, `const String = 'abc'; String.toUpperCase()` folds to
/// `ABC` in the reference compiler, so treating the name as the global would
/// refuse an input it compiles. The lookup is one map read and no evaluation, so
/// it stays in front of the walk with the other cheap answers.
fn unshadowed_global<'a>(expr: &'a Expr, reader: &Reader) -> Option<&'a Atom> {
  match expr.as_ident() {
    Some(name) if is_valid_callee(expr) && get_binding(expr, reader.traversal_state).is_none() => {
      Some(&name.sym)
    },
    _ => None,
  }
}

/// Whether a call is one this module can hand to the engine whole.
///
/// Every boundary is checked here rather than at the outermost call, because a
/// chain hides its middle links: `"AB".toLocaleLowerCase().trim()` is a `trim`
/// whose receiver needs a locale.
///
/// Nearly everything answerable from syntax is answered before the walk, because
/// the walk resolves bindings and resolution is the only expensive thing here. So
/// the shape of the callee, the spelling of the method name, a receiver written
/// as a number and a length nothing bounds are all settled while they are still
/// free, and only an expression this module intends to fold pays to have its
/// names read. The one exception is named where it is applied.
fn admit_call<'a>(
  call: &'a CallExpr,
  guard: Guard,
  reader: &mut Reader,
) -> Result<Admitted<'a>, Decline> {
  let Callee::Expr(callee) = &call.callee else {
    return Err(Decline::NotACandidate);
  };

  // `String(x)`, `Number(x)`, `Array(n)` and `Object(x)` are native JavaScript
  // functions, so they are folded by being called rather than by a conversion
  // written out here. A name the module bound is not one of them and is left to
  // the dispatch below, which calls the author's own function.
  if let Some(global) = unshadowed_global(callee, reader) {
    return admit_applied_global(global, call, guard, reader);
  }

  let Expr::Member(MemberExpr { obj, prop, .. }) = callee.as_ref() else {
    return Err(Decline::NotACandidate);
  };

  // A computed method name is a lookup the guard cannot resolve, even when it
  // is written as a literal.
  let MemberProp::Ident(method) = prop else {
    return Err(Decline::NotACandidate);
  };

  // A receiver naming one of the globals the engine provides itself — `Math`,
  // `Object`, `String`, `Number`, `Array` — carries no value across the bridge:
  // the printed source names it and the language answers. That is the whole of
  // what folding a static needs, so the surface is the language's rather than a
  // list of names this compiler chose, and where a static is written no longer
  // decides whether it folds.
  let global = unshadowed_global(obj, reader);

  // The statics the reference compiler refuses by name, refused here for the
  // reason it refuses them: each answers by changing what it was handed, or
  // answers something new on every build, and either way a fold of it is not a
  // function of the source. `INVALID_METHODS` is that compiler's own set.
  if let Some(global) = global
    && is_invalid_method(prop)
  {
    return Err(Decline::rule(unfoldable_static(global, &method.sym)));
  }

  if lists(&LOCALE_SENSITIVE_METHODS, &method.sym) {
    return Err(Decline::rule(locale_sensitive_method(&method.sym)));
  }

  if receiver_is_a_written_number(obj) {
    return Err(Decline::rule(numeric_literal_receiver(&method.sym)));
  }

  admit_amplification(&method.sym, obj, &call.args, guard.scope)?;

  // A global the engine provides itself carries no value across the bridge: the
  // printed source names it and the language answers. It is admitted here, as a
  // receiver, and nowhere else — a global's *name* is not a value this fold
  // carries, and admitting it as one would let `['a'].concat(String)` fold a
  // function's own source text into a declaration.
  if global.is_none() {
    admit_value(obj, guard, reader)?;
  }

  for arg in &call.args {
    admit_argument(arg, guard, reader)?;
  }

  // The one rule left behind the walk, and deliberately. It is a name check like
  // the three above and would cost nothing in front of them, but a chain of
  // escaping reads is refused outermost-first there — so
  // `''.constructor.constructor('return 1').call()` would be named for its
  // `call` rather than for the `constructor` that is the whole of the reason.
  // The walk reaches the receiver's reads first, which is the sentence worth
  // reading, and the resolution it costs is one binding on a call already
  // certain to refuse.
  if lists(&ESCAPING_PROPERTIES, &method.sym) {
    return Err(Decline::rule(escaping_property(&method.sym)));
  }

  Ok(Admitted::Method(&method.sym))
}

/// Whether a call applying a global is one the engine can answer.
///
/// The arguments are walked as values and nothing else: none of the globals is a
/// higher-order function, so an arrow among them is a value like any other.
///
/// A name the bridge cannot carry is a refusal here rather than a shape handed
/// back, because the fold owns every call to an unbound global — nothing below
/// it folds one, so handing the call back would end it at the catch-all's
/// `Unsupported expression` with the reason lost. The one thing the guard does
/// not answer is whether the global is a function at all: that is the language's
/// answer and is read off the engine in [`Admitted::callable`].
fn admit_applied_global<'a>(
  global: &'a Atom,
  call: &CallExpr,
  guard: Guard,
  reader: &mut Reader,
) -> Result<Admitted<'a>, Decline> {
  for arg in &call.args {
    admit_argument(arg, guard, reader).map_err(|declined| match declined {
      Decline::NotACandidate => Decline::rule(uncoercible_value(global)),
      rule => rule,
    })?;
  }

  Ok(Admitted::Global(global))
}

/// Whether the receiver is a number written into the source as a literal.
///
/// Every `Number.prototype` method throws upstream on one of those — the
/// reference implementation applies the method without a receiver, so
/// `(5).toFixed(2)` reports that `toFixed` requires a Number. Refusing keeps
/// both compilers rejecting the same input. A number that a fold *produced* is
/// a different shape and folds in both, as does a negated literal, which is a
/// unary expression rather than a literal.
fn receiver_is_a_written_number(receiver: &Expr) -> bool {
  match receiver {
    Expr::Paren(paren) => receiver_is_a_written_number(&paren.expr),
    Expr::Lit(lit) => matches!(lit, Lit::Num(_)),
    _ => false,
  }
}

/// Whether a length-amplifying call is bounded well enough to evaluate.
///
/// The length asked for has to be written into the source as a number under
/// [`MAX_AMPLIFIED_LENGTH`], and the receiver must not itself be a call:
/// per-link bounds alone would let `"x".repeat(1000000).repeat(1000000)`
/// multiply two allowed lengths into one that is not. And the call must not sit
/// inside a callback, which runs once per element of a receiver nothing here
/// measured, so a bound written once is multiplied by a count the source never
/// states. With all three in place the most a fold can build is one bounded
/// string per amplifying call written, so the source file bounds the total.
fn admit_amplification(
  method: &Atom,
  receiver: &Expr,
  args: &[ExprOrSpread],
  scope: Scope,
) -> Result<(), Decline> {
  if !lists(&LENGTH_AMPLIFYING_METHODS, method) {
    return Ok(());
  }

  // A written bound bounds one evaluation. A callback body is evaluated once
  // per element of a receiver this guard never measured, so the same bound is
  // multiplied by a count the source never states: `"x".repeat(999999)
  // .split("").map(() => "y".repeat(999999))` is two calls, each inside the
  // bound, building a terabyte between them. The count cannot be read here, so
  // an amplifying call inside a callback is refused whatever its argument says.
  if matches!(scope, Scope::Params(_)) {
    return Err(Decline::rule(amplification_inside_a_callback(method)));
  }

  let unbounded = || Decline::rule(unbounded_amplified_length(method, MAX_AMPLIFIED_LENGTH));

  if matches!(receiver, Expr::Call(_)) {
    return Err(unbounded());
  }

  match args.first() {
    // `"x".padStart()` amplifies nothing, so there is no length to bound.
    None => Ok(()),
    Some(ExprOrSpread { spread: None, expr }) => match expr.as_ref() {
      Expr::Lit(Lit::Num(length)) if length.value <= MAX_AMPLIFIED_LENGTH => Ok(()),
      _ => Err(unbounded()),
    },
    Some(_) => Err(unbounded()),
  }
}

/// An argument is admitted when it is a value the walk carries — an arrow among
/// them, which is how a callback and an own conversion method reach the engine.
fn admit_argument(arg: &ExprOrSpread, guard: Guard, reader: &mut Reader) -> Result<(), Decline> {
  // A spread needs the scope, and is refused rather than handed back: the
  // receiver is walked before the arguments, so a call reaching here is one this
  // module owns, and the sentence for a spread is the same one every other
  // position gives it.
  if arg.spread.is_some() {
    return Err(Decline::rule(SPREAD_ELEMENT));
  }

  admit_value(&arg.expr, guard, reader)
}

/// Whether an arrow reads nothing but its own parameters and names the module
/// resolves. Anything else would need a scope the engine does not have.
///
/// A block body is refused rather than analysed: statements bind, assign and
/// loop, and none of that is modelled here.
fn admit_arrow(arrow: &ArrowExpr, guard: Guard, reader: &mut Reader) -> Result<(), Decline> {
  let mut params = Vec::with_capacity(arrow.params.len());

  for param in &arrow.params {
    match param {
      Pat::Ident(ident) => params.push(ident.sym.clone()),
      _ => return Err(Decline::NotACandidate),
    }
  }

  let BlockStmtOrExpr::Expr(body) = arrow.body.as_ref() else {
    return Err(Decline::NotACandidate);
  };

  admit_value(body, guard.binding(&params), reader)
}

/// Converts an engine value into the evaluator's own value type, or declines
/// with the rule that refused it.
///
/// The evaluator's type rather than a bare syntax node, so a folded value
/// reaches every place a value the author wrote reaches: an array answers the
/// `Vec` an array literal answers, and an object the `Expr::Object` an object
/// literal answers. Answering a syntax node instead is what left a folded array
/// and an evaluated one in two dispatch arms that disagree about which methods
/// they carry.
fn to_value(
  value: &JsValue,
  engine: &mut Context,
  outward: Outward,
) -> Result<EvaluateResultValue, Decline> {
  // Read through the accessors rather than matching variants: the engine's
  // value is nan-boxed by default and an enum only under a feature, and both
  // answer these.
  if let Some(number) = value.as_number() {
    // Spelled rather than written straight into a `Number` node: `NaN` and the
    // infinities have no numeric literal, so the emitter would write `0 / 0` and
    // a numeral no author wrote. A class name is a hash of the declaration text,
    // so the spelling is the value.
    return Ok(EvaluateResultValue::Expr(coercions::js_number_expr(number)));
  }

  let literal = if let Some(truth) = value.as_boolean() {
    Lit::Bool(truth.into())
  } else if value.is_null() {
    Lit::Null(Null { span: DUMMY_SP })
  } else if let Some(string) = value.as_string() {
    // The bound on an amplifying argument bounds what one written call may be
    // asked to build; this bounds what actually came back, whatever produced
    // it. The array arm below has had such a bound from the start, and a string
    // is the other shape a fold can return at size.
    if string.len() as f64 > MAX_AMPLIFIED_LENGTH {
      return Err(Decline::rule(folded_string_too_large(MAX_AMPLIFIED_LENGTH)));
    }

    // The engine's strings are UTF-16 and `Lit::Str`'s atom is UTF-8, so an
    // unpaired surrogate cannot survive this step. Substituting the replacement
    // character keeps the declaration text identical to what the reference
    // implementation writes to disk, and diverges only in the class name.
    Lit::Str(string.to_std_string_lossy().into())
  } else {
    return to_object_value(value, engine, outward);
  };

  Ok(EvaluateResultValue::Expr(Expr::Lit(literal)))
}

/// The half of [`to_value`] that needs the engine to read the value back:
/// arrays and plain objects, and the refusal for everything else.
fn to_object_value(
  value: &JsValue,
  engine: &mut Context,
  outward: Outward,
) -> Result<EvaluateResultValue, Decline> {
  // `typeof` names the kind rather than a word list of this module's own: the
  // engine already answers this question, exhaustively, and its answer is the
  // one an author would use for the value they wrote.
  let Some(object) = value.as_object() else {
    return Err(Decline::rule(unfoldable_fold_result(value.type_of())));
  };

  let inner = outward.descend()?;

  if object.is_array() {
    let length = read_length(&object, engine, outward)?;
    let mut items = Vec::with_capacity(length as usize);

    for index in 0..length {
      let element = read(outward.method, || object.get(index, engine))?;

      // A hole and an element written `undefined` are the same value, and it
      // crosses back as the name the language spells it with — the one value
      // with no literal at all. `Array(3)` is three of them, and refusing
      // instead would refuse the array rather than the holes in it, where the
      // style-array check is what an author should hear from, on both compilers.
      items.push(match element.is_undefined() {
        true => EvaluateResultValue::Expr(Expr::Ident(create_ident(&Atom::from("undefined")))),
        false => to_value(&element, engine, inner)?,
      });
    }

    return Ok(EvaluateResultValue::Vec(items));
  }

  if !object.is_ordinary() {
    return Err(Decline::rule(unfoldable_fold_result(value.type_of())));
  }

  let keys = read(outward.method, || object.own_property_keys(engine))?;

  if keys.len() as u64 > MAX_FOLDED_ENTRIES {
    return Err(Decline::rule(object_size_too_large(MAX_FOLDED_ENTRIES)));
  }

  let mut props = Vec::with_capacity(keys.len());

  for key in keys {
    // A symbol key has no spelling in an object literal, so an object carrying
    // one cannot be written back out whole — and writing it out partly would
    // fold a value the source does not describe. `PropertyKey` has three
    // variants and all three are answered, as the crate's error policy asks.
    let name = match &key {
      PropertyKey::String(string) => string.to_std_string_lossy(),
      PropertyKey::Index(index) => index.get().to_string(),
      PropertyKey::Symbol(_) => {
        return Err(Decline::rule(unfoldable_fold_result(
          "object with a symbol key",
        )));
      },
    };

    let element = read(outward.method, || object.get(key.clone(), engine))?;
    let expr = as_property_value(to_value(&element, engine, inner)?)?;

    props.push(create_ident_key_value_prop(&name, expr));
  }

  // Ordered by the same rule an object the author wrote is ordered by, rather
  // than by trusting two implementations of own-key order to agree.
  Ok(EvaluateResultValue::Expr(Expr::Object(create_object_lit(
    order_own_keys(props),
  ))))
}

/// One folded value as the expression an object property carries.
///
///
/// An array is the one case that has to be rebuilt rather than moved: a `Vec`
/// is the shape the evaluator wants at the top of a value, and an object
/// literal wants a nested array literal in the same position. Rebuilt by the
/// evaluator's own conversion rather than by a second copy of it here, so a
/// folded property and an evaluated one cannot come to disagree about what an
/// array element may be.
fn as_property_value(value: EvaluateResultValue) -> Result<Expr, Decline> {
  let expr = match &value {
    EvaluateResultValue::Expr(expr) => Some(expr.clone()),
    EvaluateResultValue::Vec(items) => evaluate_result_vec_to_array_expr(items),
    // Every arm of `to_value` answers one of the two above, so nothing else is
    // reachable by construction — and answers a refusal rather than panicking
    // if that ever stops holding.
    _ => None,
  };

  expr.ok_or_else(|| Decline::rule(unfoldable_fold_result("value of an unreadable kind")))
}

/// An array's `length`, bounded: the count the conversion loop below reads.
///
/// The two ways it can fail say different things, because they are different
/// faults. A length past [`MAX_FOLDED_ENTRIES`] is the bound, and names it. A
/// `length` that is not a count at all — not a number, or negative — is not the
/// bound and must not claim to be; it is a value the bridge cannot read, and is
/// refused as one.
fn read_length(object: &JsObject, engine: &mut Context, outward: Outward) -> Result<u64, Decline> {
  let length = read(outward.method, || object.get(js_string!("length"), engine))?;

  let Some(length) = length.as_number().filter(|length| *length >= 0.0) else {
    return Err(Decline::rule(unfoldable_fold_result(
      "array with no readable length",
    )));
  };

  if length > MAX_FOLDED_ENTRIES as f64 {
    return Err(Decline::rule(array_length_too_large(
      MAX_FOLDED_ENTRIES as usize,
    )));
  }

  Ok(length as u64)
}

/// A read across the bridge, with a throw carried in the engine's words.
///
/// Reading a property runs a getter and writing one can be refused, so both
/// directions need the same answer the evaluation itself gets rather than a
/// second one of their own.
fn read<T>(method: &Atom, read: impl FnOnce() -> JsResult<T>) -> Result<T, Decline> {
  read().map_err(|error| threw(method, &error))
}

/// The call as the minified source the engine is handed: an arrow over the names
/// the guard resolved, whose values [`apply`] passes to it as arguments — or the
/// call alone where it resolved none.
///
/// The bare form is not a second path so much as the absence of one: an arrow
/// over no parameters, invoked immediately, is the same expression with a
/// function object and a VM frame added, and [`apply`] carries the measurement
/// that says what those cost. Printing the call itself is what lets an expression
/// that names nothing pay nothing.
///
/// The module is assembled here rather than by `create_module`, which takes
/// `&Expr` and clones it — so going through it means cloning the subtree once
/// to build the `Expr` and once more inside. The printer needs an owned tree
/// either way, because it drops the spans in place before emitting.
fn print_fold(call: &CallExpr, params: &[Atom]) -> String {
  let folded = Expr::Call(call.clone());

  let printed = match params.is_empty() {
    true => folded,
    false => create_arrow_expression_with_params(
      params
        .iter()
        .map(|name| Pat::Ident(create_binding_ident(create_ident(name))))
        .collect(),
      folded,
    ),
  };

  let module = Module {
    span: DUMMY_SP,
    body: vec![ModuleItem::Stmt(Stmt::Expr(ExprStmt {
      span: DUMMY_SP,
      expr: Box::new(printed),
    }))],
    shebang: None,
  };

  print_module(
    &CodeFrame::new(),
    module,
    Some(
      Config::default()
        .with_minify(true)
        .with_omit_last_semi(true),
    ),
  )
}
