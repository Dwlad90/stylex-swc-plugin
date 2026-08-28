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
//! A name holding a *function* crosses by the other route, because a function
//! has no value an argument could carry: the declaration it came from is
//! printed as the parameter's default and nothing is passed for it. So a
//! callback given a name folds exactly as the same arrow written out in place
//! does, down to which spelling shadows which. Calling such a name is the same
//! carriage and a different question — see [`Position`] for the one thing on
//! this bridge that reads where a call sits.
//!
//! A fold answers one of two things: the value, or the rule that refused it.
//! There is no silent refusal — a call the guard recognised and declined says
//! which rule declined it, rather than falling through to the caller's
//! `Unsupported expression: CallExpression`. A call the guard never recognised
//! is not a refusal at all: it is simply not this module's, and the dispatch
//! below it decides what happens instead.

use std::{borrow::Cow, cell::RefCell, mem::ManuallyDrop};

use boa_engine::{
  Context, JsError, JsObject, JsResult, JsString, JsValue, NativeFunction, Script, Source,
  js_string, native_function::NativeFunctionPointer, object::builtins::JsArray,
  property::PropertyKey,
};
use swc_core::{
  atoms::{Atom, Wtf8Atom},
  common::DUMMY_SP,
  ecma::{
    ast::{
      ArrayLit, ArrowExpr, AssignPat, BlockStmtOrExpr, CallExpr, Callee, Decl, Expr, ExprOrSpread,
      ExprStmt, Ident, KeyValueProp, Lit, MemberExpr, MemberProp, Module, ModuleItem, Null,
      ObjectLit, ObjectPatProp, Pat, Prop, PropName, PropOrSpread, ReturnStmt, Stmt, Tpl,
    },
    codegen::Config,
  },
};

use rustc_hash::FxHashMap;
use stylex_ast::ast::convertors::atom_utf16_length;
use stylex_ast::ast::factories::{
  create_arrow_expression_with_params, create_binding_ident, create_ident,
  create_ident_key_value_prop, create_object_lit,
};
use stylex_constants::constants::evaluation_errors::{
  SPREAD_ELEMENT, amplification_inside_a_callback, amplified_entries_too_large,
  amplified_length_too_large, array_length_too_large, bound_value_has_too_many_entries,
  bound_value_too_large, engine_did_not_start, engine_threw, escaping_property,
  expression_too_deep, folded_string_too_large, locale_sensitive_method, not_a_function,
  numeric_literal_receiver, object_size_too_large, unbounded_amplified_length,
  uncallable_printed_fold, uncoercible_value, unfoldable_fold_result, unfoldable_function,
  unfoldable_statement, unfoldable_static,
};
use stylex_js::coercions::{self, is_global_spelled_as_an_identifier, to_js_number};
use stylex_js::helpers::{is_invalid_method, is_valid_callee};
use stylex_utils::number::to_js_string;
use stylex_utils::swc::get_stmt_node_kind;

use super::{
  engine_stylex_functions::{EngineCallable, Reached, engine_callable},
  evaluate_cached, evaluate_result_vec_to_array_expr, growable_stack,
  helpers::get_binding,
};
use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{functions::FunctionMap, state::EvaluationState, state_manager::StateManager},
  utils::{
    common::order_own_keys,
    js::check_declaration::DeclarationType,
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

/// Methods whose result *string* length is set by an argument, and so are the
/// only ones a single small argument can blow up into text.
///
/// Set by an argument is what makes them answerable *here*, in front of the
/// engine: the guard reads arguments. A method whose result length comes from its
/// receiver adds nothing to that receiver's own length, so it is bounded by
/// whatever set it — which is one call earlier and is [`EntryAmplifier`].
/// `Array(n).fill(0)` is `n` read from the other end, and `fill` itself is
/// innocent.
const LENGTH_AMPLIFYING_METHODS: [&str; 3] = ["repeat", "padStart", "padEnd"];

/// Methods whose callback the language evaluates at most once per element of the
/// receiver, and hands the element to as its first parameter.
///
/// Both halves are what a bound needs. *At most once per element* is what makes
/// the receiver's element count a factor the two amplification rules can
/// multiply by; *first parameter* is what makes the element's width the width of
/// the name a body reads it through.
///
/// A name not listed here leaves a callback unmeasured, which is the refusal
/// every callback used to get — so the list is safe by default and grows only
/// where both halves were checked. `sort` is left out because a comparator runs
/// more often than its array is long, and `reduce` and `reduceRight` with it
/// because the element they hand a callback is its second parameter, so a width
/// read off the receiver would name the accumulator.
const PER_ELEMENT_METHODS: [&str; 10] = [
  "map",
  "flatMap",
  "filter",
  "forEach",
  "some",
  "every",
  "find",
  "findIndex",
  "findLast",
  "findLastIndex",
];

/// The lengths an array may have, as the language's own range.
///
/// A number outside it declares no length at all: `Array(2 ** 32)` is a
/// `RangeError` rather than an array, raised before anything is allocated. Held
/// as a range of floats because that is what the number being checked is, and
/// comparing before any cast keeps a value the cast would saturate from reading
/// as a length.
const VALID_ARRAY_LENGTHS: std::ops::Range<f64> = 0.0..4_294_967_296.0;

/// What one fold may allocate, in the two units that cost separately.
///
/// The engine bounds recursion and VM stack depth by default, and this module
/// bounds loop iterations itself ([`MAX_LOOP_ITERATIONS`]), because Boa's own
/// default for that is `u64::MAX`. What none of them bound is allocation:
/// growth inside a native builtin is not a counted loop. So
/// `"x".repeat(200000000)` is a typo that folds — agreeing with the language —
/// at gigabytes of resident memory, and a compiler that dies there is worse than
/// one that declines the fold.
///
/// Two numbers rather than one, because a value costs in two ways that do not
/// stand in for each other: a bounded string can still become one element per
/// code unit, which costs far more as a tree than it did as text, and a thousand
/// empty arrays hold no text at all and are still a thousand values to build.
///
/// Carried rather than read at each of the six sites that spend them -- three
/// each, a resolved value on the way in and the answer on the way back -- for
/// the reason the depth ceiling is carried: the same number bounds both
/// directions, and sites that each reached for the option could come to
/// disagree about which number that is. Where the values
/// come from, and what each costs, is `stylex_structures::fold_ceilings`.
#[derive(Clone, Copy)]
struct Ceilings {
  /// UTF-16 code units of string — the unit the engine's own strings are
  /// measured in.
  characters: u64,
  /// Array elements and object properties.
  entries: u64,
}

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
  /// price of not aborting. It wraps the whole of [`Engine`] rather than the
  /// context alone, because the memo beside it holds engine values and dropping
  /// one of those late underflows the same count.
  static ENGINE: RefCell<Option<ManuallyDrop<Engine>>> = const { RefCell::new(None) };
}

/// A thread's engine and the [fold memo](../../../../../CONTEXT.md) that may only
/// live as long as it does.
///
/// The two are one value because their lifetimes are one lifetime. A compiled
/// script belongs to a particular engine's realm, so a memo that outlived its
/// engine would hand a later engine a script from a realm that no longer exists,
/// and a memo built before the engine would have nothing to parse against.
/// Holding them together is what makes both impossible to write.
struct Engine {
  context: Context,
  /// One compiled script per distinct printed expression.
  ///
  /// Keyed by the printed text because that is what the engine would otherwise
  /// re-parse: a file writing one shape a thousand times prints a thousand
  /// identical strings, and parsing them is most of what a warm fold costs.
  /// Reuse across files is safe for the reason a shared engine is — a printed
  /// expression carries no name it did not resolve, since every one the guard
  /// resolved became a parameter of an arrow and its value travels beside it as
  /// an argument.
  ///
  /// A compiled script rather than the value or the function it evaluates to,
  /// because those are two shapes and this is one. The bare form evaluates to
  /// the answer and the arrow form to a function still waiting for its
  /// arguments, so a memo of results would have to hold both and a memo of
  /// functions could not hold the first at all — and re-running compiled
  /// bytecode is what both of them wanted anyway.
  ///
  /// It grows for the life of the thread, alongside the source the engine
  /// already interns and never reclaims, and is bounded by the number of
  /// distinct folded call sites in the build rather than by the number of folds.
  memo: FxHashMap<String, Script>,
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
  /// Names a callback binds itself — its parameters, and whatever a block of
  /// its body declares — over the scope they were written inside. The engine
  /// binds them when it runs the callback, so the guard neither resolves one
  /// nor carries a value for it.
  ///
  /// A chain rather than one set, because an inner arrow does not replace the
  /// scope around it: `a.map(x => b.map(y => x + y))` reads `x` from the arrow
  /// outside the one that is being walked.
  Names {
    /// The names this scope introduces, in the order they were written.
    names: &'a [Atom],
    /// Which of them hold an element of a receiver the call around this scope
    /// measured, and how wide such an element is.
    elements: Elements,
    outer: &'a Scope<'a>,
  },
}

impl Scope<'_> {
  /// Whether this scope or any around it binds `name` — in which case the name
  /// is the engine's to resolve, not the module's.
  fn binds(&self, name: &Atom) -> bool {
    match self {
      Scope::Module => false,
      Scope::Names { names, outer, .. } => names.contains(name) || outer.binds(name),
    }
  }

  /// Whether the walk is inside a callback body, where a call runs once per
  /// element of a receiver the guard may not have measured.
  fn inside_a_callback(&self) -> bool {
    matches!(self, Scope::Names { .. })
  }

  /// How many characters `name` may hold, where it is an element of a receiver
  /// a call measured — and `None` for every other name, whose value nothing
  /// here bounded.
  ///
  /// The innermost scope binding the name answers, so a name shadowing an
  /// element is read as itself rather than borrowing the element's width.
  fn characters_of(&self, name: &Atom) -> Option<u64> {
    let Scope::Names {
      names,
      elements,
      outer,
    } = self
    else {
      return None;
    };

    match names.iter().position(|bound| bound == name) {
      Some(at) if at < elements.named => elements.characters,
      Some(_) => None,
      None => outer.characters_of(name),
    }
  }
}

/// What the guard knows about the values a scope's leading names hold.
///
/// Leading, because only a callback's *first* parameter is handed the element:
/// the parameters after it are the index and the receiver itself, and a name a
/// block of the body declares holds whatever the body built. Neither is bounded
/// by an element's width, so neither borrows it.
#[derive(Clone, Copy, Default)]
struct Elements {
  /// How many of the scope's names hold one, counted from the first.
  named: usize,
  /// The characters the widest of them renders to, or `None` where an element
  /// renders to a width the guard could not read.
  characters: Option<u64>,
}

/// How many times the expression under the walk is evaluated.
///
/// One at module scope. Inside a callback it is the product of every enclosing
/// receiver's element count, so nesting multiplies rather than resets — and a
/// callback over a receiver nothing counted is unmeasured, which is the blanket
/// refusal the two amplification rules used to give every callback body.
#[derive(Clone, Copy, Default)]
enum Repeats {
  /// Evaluations the guard counted.
  Times(u64),
  /// A callback over a receiver whose element count nothing here read.
  #[default]
  Unmeasured,
}

impl Repeats {
  /// The count a bound on one evaluation is multiplied by, or the refusal that
  /// there is no such count — which is what both amplification rules do with an
  /// unmeasured callback, so the sentence is written once here.
  ///
  /// `built` is the unit the refusal names, for the reason the message takes one:
  /// a call amplifies in one of the two a fold spends, and the two do not stand
  /// in for each other.
  fn counted(self, built: &str, call: &str) -> Result<u64, Decline> {
    match self {
      Self::Times(times) => Ok(times),
      Self::Unmeasured => Err(Decline::rule(amplification_inside_a_callback(built, call))),
    }
  }

  /// The repeats of a body evaluated once per element of a receiver holding
  /// `elements`, which multiplies what is already counted rather than replacing
  /// it — so a callback nested in a callback is the product of both receivers.
  ///
  /// Saturating for the reason the product inside one evaluation saturates: it
  /// exists to be refused on, and a wrapped one would admit.
  fn per_element(self, elements: u64) -> Self {
    match self {
      Self::Times(times) => Self::Times(times.saturating_mul(elements)),
      Self::Unmeasured => Self::Unmeasured,
    }
  }
}

/// What the call under the walk measured for an arrow it will run.
///
/// Two positions reach one, and they are measured differently. A callback among
/// the arguments runs once per element of the receiver and is handed that
/// element, so it carries both numbers. The callee of a call reached through a
/// name runs once per evaluation of the call itself and is handed the
/// arguments, whose width nothing here reads — so it carries the count alone.
///
/// One value rather than two fields on the guard, so the two things a body needs
/// — how often it runs, and how wide the value it is handed — are read off the
/// same measurement and cannot come to disagree.
#[derive(Clone, Copy, Default)]
struct Callback {
  /// How many times the call will run the body.
  repeats: Repeats,
  /// The characters the widest element of the receiver renders to.
  characters: Option<u64>,
}

/// How much nesting is left, and the one refusal spent at the bottom of it.
///
/// Every walk across the bridge counts the same budget for the same reason ---
/// the guard's walk in, the conversion of a value it resolved, and the
/// conversion out all recurse on the stack — so they share the counter and
/// the sentence rather than keeping three that could drift apart.
///
/// The budget is the project's configured evaluation depth, which is also what
/// the evaluator's own descent spends. One number, because both walks run on
/// the same grown stack for the same reason, and two would mean an author who
/// raised the configured depth still met a ceiling nobody set.
///
/// Reaching the bottom is a refusal and not a "not mine": the shape is one an
/// author can shorten, and saying so is more use than handing it back to a path
/// that would refuse it in vaguer words.
#[derive(Clone, Copy)]
struct Depth {
  /// Levels left before the walk refuses.
  left: usize,
  /// The ceiling the refusal names — kept beside the count because a walk
  /// that has spent its budget no longer knows what it started with.
  ceiling: usize,
}

impl Depth {
  /// A full budget, at the start of a walk or a conversion.
  fn full(ceiling: usize) -> Self {
    Self {
      left: ceiling,
      ceiling,
    }
  }

  /// A full budget again, for a walk that starts where this one stands.
  ///
  /// A value the guard resolved and a value the engine answered are each
  /// converted by a walk of their own, and neither is nested inside the
  /// expression that reached them — so each is measured against the whole
  /// ceiling rather than against what the walk before it had left.
  fn restart(self) -> Self {
    Self::full(self.ceiling)
  }

  /// One level in, or the depth refusal at the bound — so depth is answered
  /// the same way as any other rule the guard applies.
  fn descend(self) -> Result<Self, Decline> {
    match self.left {
      0 => Err(Decline::rule(expression_too_deep(self.ceiling))),
      left => Ok(Self {
        left: left - 1,
        ..self
      }),
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
  ceilings: Ceilings,
  /// How many times the expression under the walk is evaluated.
  repeats: Repeats,
  /// What the call under the walk measured for a callback among its arguments,
  /// and `None` where this position reaches no callback the guard counts.
  ///
  /// Set by a call for its own arguments and dropped everywhere else, so the
  /// arrow that reads it is the one written inside the call that measured it.
  callback: Option<Callback>,
}

impl<'a> Guard<'a> {
  /// The guard one level in.
  fn descend(self) -> Result<Self, Decline> {
    Ok(Self {
      depth: self.depth.descend()?,
      ..self
    })
  }

  /// The same remaining depth, with `names` bound over the scope this guard
  /// already carries.
  ///
  /// `elements` says which of the names hold a value the call around them
  /// measured, and `repeats` how many times the scope being entered runs. A
  /// block inside a callback passes neither on: what a block declares is a value
  /// the body built, and the body's own repeats are already counted.
  fn binding<'b>(&'b self, names: &'b [Atom], elements: Elements, repeats: Repeats) -> Guard<'b> {
    Guard {
      scope: Scope::Names {
        names,
        elements,
        outer: &self.scope,
      },
      depth: self.depth,
      ceilings: self.ceilings,
      repeats,
      // The scope being entered is not a call, and only the call an arrow was
      // written inside says how often that arrow runs.
      callback: None,
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
struct Transport {
  params: Vec<Atom>,
  values: Vec<Crossing>,
  totals: Totals,
  ceilings: Ceilings,
}

impl Transport {
  /// An empty transport, holding the ceilings every value it carries is counted
  /// against.
  fn new(ceilings: Ceilings) -> Self {
    Self {
      params: Vec::new(),
      values: Vec::new(),
      totals: Totals::default(),
      ceilings,
    }
  }

  /// Records what `name` resolves to, or does nothing where the name is already
  /// carried.
  ///
  /// One parameter per name however often the expression reads it, because a
  /// repeated parameter is a syntax error in the arrow this is printed into. The
  /// second reading resolves to the same value — the evaluator memoises it — so
  /// dropping it loses nothing.
  ///
  /// The value is converted here rather than when the engine is in hand, so a
  /// value past a bound refuses before anything is printed or built. What comes
  /// out is [`Carried`]: the shapes the bridge carries, measured, and not yet
  /// the engine's own values.
  fn bind(
    &mut self,
    name: &Atom,
    value: &EvaluateResultValue,
    depth: Depth,
  ) -> Result<(), Decline> {
    // Asked before the conversion rather than left to `carry`, so a name read
    // twice is not converted twice.
    if self.holds(name) {
      return Ok(());
    }

    let carried = Inward {
      name,
      totals: &mut self.totals,
      ceilings: self.ceilings,
    }
    .value(value, depth.restart())?;

    self.carry(name, Crossing::Value(carried));

    Ok(())
  }

  /// Records something the bridge already holds in its own shapes, under the
  /// name the printed source reads it through.
  ///
  /// Nothing here can refuse, which is the whole difference from [`bind`]: what
  /// it takes was not copied out of the module, so there is no text and no entry
  /// count to compare a ceiling to. A [StyleX function the engine may
  /// call](engine_stylex_functions) and a resolved arrow are what arrive this
  /// way.
  ///
  /// [`bind`]: Transport::bind
  fn carry(&mut self, name: &Atom, crossing: Crossing) {
    if self.holds(name) {
      return;
    }

    self.params.push(name.clone());
    self.values.push(crossing);
  }

  /// The names as the parameters of the printed arrow.
  ///
  /// A value's name is a bare parameter and its value arrives as an argument. A
  /// function's is a parameter with the expression it was declared from as its
  /// default, because a function has no argument form to arrive as — so the
  /// printed source carries it and the call passes nothing.
  ///
  /// A default is evaluated where the parameter stands, so the order the names
  /// were carried in is the order they have to be printed in: a name a default
  /// reads is carried before the name whose default reads it, which is what
  /// walking a declaration before recording it buys.
  fn parameters(&self) -> Vec<Pat> {
    self
      .params
      .iter()
      .zip(&self.values)
      .map(|(name, crossing)| {
        let bound = Pat::Ident(create_binding_ident(create_ident(name)));

        match crossing {
          Crossing::Value(_) => bound,
          Crossing::Source(source) => Pat::Assign(AssignPat {
            span: DUMMY_SP,
            left: Box::new(bound),
            right: source.clone(),
          }),
        }
      })
      .collect()
  }

  /// Whether a value is already travelling under `name`.
  ///
  /// A scan rather than a set, because an expression carries a handful of names
  /// and hashing them would cost more than comparing them.
  fn holds(&self, name: &Atom) -> bool {
    self.params.contains(name)
  }

  /// The carried values as the arguments the printed arrow is called with.
  ///
  /// Built with the engine in hand, because an array and an object are engine
  /// objects and there is no way to make one without it. Everything that could
  /// refuse was answered when the value was carried, so this step only builds.
  fn arguments(&self, engine: &mut Context, method: &Atom) -> Result<Vec<JsValue>, Decline> {
    let mut arguments = Vec::with_capacity(self.values.len());

    for value in &self.values {
      arguments.push(match value {
        Crossing::Value(carried) => to_js(carried, engine, method)?,
        // A function travels in the printed source as the parameter's default,
        // so passing nothing for it is what makes the default the value.
        Crossing::Source(_) => JsValue::undefined(),
      });
    }

    Ok(arguments)
  }
}

/// What travels under one parameter name of the printed arrow.
///
/// Two arms because a value and a function cross by different routes, and the
/// routes are not interchangeable. A value is copied into the engine and passed
/// as an argument, which keeps the printed text the size of the expression
/// however large the value is. A function has no such form — there is no engine
/// value a resolved arrow could be built into before the engine exists, and
/// nothing an argument could carry — so what crosses is the source it was
/// declared from, printed back where the parameter stands.
///
/// Printing it as a *default* rather than substituting it at each reading is
/// what keeps shadowing the language's answer instead of this walk's: a callback
/// parameter of the same name shadows the default exactly as it shadowed the
/// module binding, and a substitution would have had to work that out for
/// itself.
enum Crossing {
  Value(Carried),
  Source(Box<Expr>),
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
///
/// A *function* is the one thing here that is not copied out of the module. It
/// is one of this compiler's own, handed over for the engine to call rather than
/// to read, and only where nothing an author writes could reach it as a value —
/// see [`engine_stylex_functions`].
enum Carried {
  Str(Wtf8Atom),
  Num(f64),
  Bool(bool),
  Null,
  List(Vec<Carried>),
  Object(Vec<(Wtf8Atom, Carried)>),
  Function(NativeFunctionPointer),
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
  ceilings: Ceilings,
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

    if self.totals.units > self.ceilings.characters {
      return Err(Decline::rule(bound_value_too_large(
        self.name,
        self.ceilings.characters,
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

    if self.totals.entries > self.ceilings.entries {
      return Err(Decline::rule(bound_value_has_too_many_entries(
        self.name,
        self.ceilings.entries,
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
    // Built per fold rather than kept, because a function object belongs to the
    // realm that made it and costs one allocation — where keeping one would tie
    // a value's lifetime to the engine's, which is the arrangement the memo
    // beside it already had to be written around.
    Carried::Function(call) => {
      JsValue::from(NativeFunction::from_fn_ptr(*call).to_js_function(engine.realm()))
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
  /// The value `expr` resolves to, or `None` where it resolves to nothing.
  ///
  /// Resolved through the evaluator's own memoised entry point rather than by a
  /// second reading of its own, so a binding this fold reads is the binding
  /// every other position reads — including the disqualifications that live
  /// there: a reassigned binding, one mutated in place, and one read above its
  /// own declaration all answer nothing here because they answer nothing there.
  ///
  /// Takes an expression rather than a name because two questions need it: what
  /// a name holds, and what an amplifying call's count comes to. `2 * 2` is a
  /// count as surely as `4` is, and the evaluator is what already knows so.
  ///
  /// The read is a speculation and is marked as one, so nothing it refuses is
  /// left behind: the evaluation's confidence and deopt are put back, and the
  /// memo withholds the refusal. An expression this module could not read is not
  /// a refusal — the dispatch below owns the call, evaluates the same names
  /// itself, and has to find both the state and the sentence it would have had.
  fn resolve(&mut self, expr: &Expr) -> Option<EvaluateResultValue> {
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
      evaluate_cached(expr, state, traversal_state, fns)
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
  let ceiling = traversal_state.evaluation_ceiling();
  let ceilings = Ceilings {
    characters: traversal_state.character_ceiling() as u64,
    entries: traversal_state.entry_ceiling() as u64,
  };

  let guard = Guard {
    scope: Scope::Module,
    depth: Depth::full(ceiling),
    ceilings,
    // The expression the caller asked about is evaluated once.
    repeats: Repeats::Times(1),
    callback: None,
  };

  let mut reader = Reader {
    state,
    traversal_state,
    fns,
    transport: Transport::new(ceilings),
  };

  // Grown before the first recursive step rather than at every step: the
  // engine's parser descends through a nested literal without ever asking for
  // room, so the whole fold has to run on a stack that was already large enough
  // when it started.
  match growable_stack::grown_for_depth(ceiling, || fold(call, guard, &mut reader)) {
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
  let admitted = admit_call(call, Position::Outermost, guard, reader)?;
  let method = admitted.name();

  let source = print_fold(call, reader.transport.parameters());

  ENGINE.with_borrow_mut(|slot| {
    // Taken, not borrowed in place. A panic unwinding out of the engine is
    // caught at the NAPI boundary and the process carries on, so an engine left
    // in the slot would be reused by every later fold with its VM stack
    // abandoned mid-frame. Taking it means an unwind leaves the slot empty and
    // the next fold builds a fresh engine; the abandoned one leaks, which is
    // what `ManuallyDrop` already makes it do at thread exit.
    let mut engine = match slot.take() {
      Some(engine) => engine,
      None => Engine::new()?,
    };

    let outward = Outward {
      method,
      depth: guard.depth.restart(),
      ceilings: guard.ceilings,
    };

    let applied = match admitted {
      Admitted::Global(global) => admit_an_applied_global(global, &mut engine.context),
      Admitted::Method(_) | Admitted::Named(_) => Ok(()),
    };

    let folded = applied
      .and_then(|()| reader.transport.arguments(&mut engine.context, method))
      .and_then(|arguments| apply(&source, &arguments, &mut engine, outward))
      .and_then(|value| to_value(&value, &mut engine.context, outward));

    *slot = Some(engine);

    folded
  })
}

/// Evaluates the printed expression and, where there is something to pass,
/// calls what it evaluated to with the transported values.
///
/// Two steps rather than one *when there is something to pass*, because the
/// values cross as arguments rather than as text. Every step can throw and all
/// of them are answered the same way: a throw is an answer, not a failure of
/// this module — the language throws on `[].reduce(f)` too — so the engine's own
/// sentence is what the author reads rather than a generic refusal standing in
/// for it.
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
/// Both arms go through the memo, because what is memoised is the compiled
/// script and neither arm has to be a function for that.
///
/// The compiled value is a function by construction, so the refusal for one that
/// is not stands in for a broken invariant rather than for anything an author can
/// write. It is a refusal all the same: this runs inside an evaluation whose
/// whole contract is that it may fail, where an assertion would abort a build
/// that a deopt would only leave to the runtime.
fn apply(
  source: &str,
  arguments: &[JsValue],
  engine: &mut Engine,
  outward: Outward,
) -> Result<JsValue, Decline> {
  let evaluated = engine.eval(source, outward.method)?;

  if arguments.is_empty() {
    return Ok(evaluated);
  }

  let Some(callable) = evaluated.as_callable() else {
    return Err(Decline::rule(uncallable_printed_fold(outward.method)));
  };

  callable
    .call(&JsValue::undefined(), arguments, &mut engine.context)
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

/// How many distinct expressions this thread's engine has compiled, or none
/// where it holds no engine.
///
/// Test-only, and the observable half of "compiled once and reused after": the
/// answer alone cannot tell a memo hit from a fresh compile, because both
/// produce the same value — which is the whole point of the memo and also why it
/// needs a witness of its own.
#[cfg(test)]
pub(super) fn compiled_expressions() -> Option<usize> {
  ENGINE.with_borrow(|slot| slot.as_ref().map(|engine| engine.memo.len()))
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

impl Engine {
  /// A context with the one runtime limit its default leaves open, without the
  /// one thing the language provides that this compiler cannot — function source
  /// text — and with an empty memo.
  ///
  /// Answers a refusal rather than asserting, because the assignment runs inside
  /// an evaluation whose whole contract is that it may fail — and because an
  /// engine that kept function source would fold a spelling no other build
  /// produces, which is worse than declining the fold.
  fn new() -> Result<ManuallyDrop<Self>, Decline> {
    let mut context = Context::default();

    context
      .runtime_limits_mut()
      .set_loop_iteration_limit(MAX_LOOP_ITERATIONS);

    context
      .eval(Source::from_bytes(NO_FUNCTION_SOURCE))
      .map_err(|error| Decline::rule(engine_did_not_start(&error.to_string())))?;

    Ok(ManuallyDrop::new(Self {
      context,
      memo: FxHashMap::default(),
    }))
  }

  /// What `source` evaluates to, parsing it the first time this engine is handed
  /// the text and re-running the compiled script every time after.
  ///
  /// Named after the engine call it stands in for rather than after the memo,
  /// because the memo is what it does and not what it is for.
  ///
  /// This is the engine's own `eval` with the parse lifted out of it — `eval` is
  /// exactly parse-then-evaluate — so what the memo changes is when a source is
  /// parsed and nothing else about what a fold answers.
  ///
  /// The parse is what the memo saves; the evaluation is not saved and must not
  /// be. Re-running is what keeps a fold answering its own value rather than the
  /// first caller's — an expression that mutates a literal it built reorders a
  /// fresh array on every run, and an arrow form evaluates to a function still
  /// waiting for the arguments this fold is about to pass it.
  ///
  /// A source the parser refused is never memoised, so a later fold of the same
  /// text is refused by the parser again rather than by a cached mistake. It is
  /// a refusal rather than an assertion because this runs inside an evaluation
  /// whose whole contract is that it may fail, where an assertion would abort a
  /// build that a deopt would only leave to the runtime.
  fn eval(&mut self, source: &str, method: &Atom) -> Result<JsValue, Decline> {
    let compiled = match self.memo.get(source) {
      Some(compiled) => compiled.clone(),
      None => {
        let compiled = Script::parse(Source::from_bytes(source), None, &mut self.context)
          .map_err(|error| threw(method, &error))?;

        self.memo.insert(source.to_owned(), compiled.clone());

        compiled
      },
    };

    compiled
      .evaluate(&mut self.context)
      .map_err(|error| threw(method, &error))
  }
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
  ceilings: Ceilings,
}

impl Outward<'_> {
  /// The bridge one level in.
  ///
  /// A value nested deeper than the guard admits on the way in can still be
  /// built on the way out, by a loop the engine ran rather than by syntax the
  /// author wrote. Bounded for the reason the input is bounded, and against the
  /// same ceiling: the conversion recurses, and the stack it recurses on was
  /// claimed for that many levels and no more.
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
    // A name a callback binds — a parameter of its own, or something a block of
    // its body declares — is bound by the engine when it runs the callback. Any
    // other name is asked of the module, and becomes a parameter of the printed
    // arrow carrying the value it resolved to.
    Expr::Ident(ident) => {
      if guard.scope.binds(&ident.sym) {
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

      match reader.resolve(expr) {
        Some(value) if is_a_carryable_receiver(&value) => {
          reader.transport.bind(&ident.sym, &value, guard.depth)
        },
        // A name holding a function, which is what the evaluator answers a
        // callback with. There is no value form to carry, so the declaration it
        // came from crosses instead.
        Some(EvaluateResultValue::Callback(_)) => {
          admit_a_named_function(ident, expr, inner, reader)
        },
        // A name that resolved to nothing is usually not this module's business
        // — the dispatch below owns the call and answers for it. A function is
        // the exception: nothing below the fold carries one into an evaluation.
        _ => match the_module_declares_a_function(ident, expr, reader) {
          true => Err(Decline::rule(unfoldable_function(&ident.sym))),
          false => Err(Decline::NotACandidate),
        },
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
    //
    // A StyleX function is asked about first, because its callee is a name the
    // module bound and every question `admit_call` asks would answer "not mine"
    // for it.
    Expr::Call(call) => match a_stylex_function(call, inner, reader) {
      Some(callable) => admit_a_stylex_function(&callable, call, inner, reader),
      None => admit_call(call, Position::Inside, inner, reader).map(|_| ()),
    },
    // An arrow is a value the language can hold and call: the callback `map` and
    // `filter` take, and the own `toString` an object converts through. It has
    // no *string* form here — the engine is built without function source text,
    // so a conversion that would read one refuses. See [`NO_FUNCTION_SOURCE`].
    Expr::Arrow(arrow) => admit_arrow(arrow, inner, reader),
    _ => Err(Decline::NotACandidate),
  }
}

/// Admits a name the module declared a function under, by crossing the
/// declaration rather than a value.
///
/// The declaration is walked like any other expression, so an arrow reached
/// through a chain of names crosses as the chain — each link a parameter of the
/// printed arrow, defaulted to what the link before it resolved. What the walk
/// admits is therefore the same set of shapes an arrow written in place gets,
/// and nothing about being named is asked separately.
///
/// A name the walk cannot reach a declaration for is refused rather than handed
/// back. The evaluator answered a function, so the fold is the only thing that
/// could have carried it and there is nothing below to hand it to.
fn admit_a_named_function(
  ident: &Ident,
  expr: &Expr,
  guard: Guard,
  reader: &mut Reader,
) -> Result<(), Decline> {
  if reader.transport.holds(&ident.sym) {
    return Ok(());
  }

  // Cloned because the walk below takes the evaluator mutably and the
  // declaration is borrowed out of it. One subtree per name per fold, and the
  // printer would have wanted an owned tree anyway.
  let Some(declaration) = initializer_of(expr, reader).cloned() else {
    return Err(Decline::rule(unfoldable_function(&ident.sym)));
  };

  // Walked in the scope the declaration was written in, which is the module. A
  // name the declaration reads is a module name however deep inside a callback
  // the reading of *this* name was, and the default it prints into stands in the
  // parameter list, where only the module names this fold carries are bound.
  //
  // Depth descends here rather than restarting, unlike the walk over a resolved
  // value: a declaration can name a second function whose declaration names the
  // first, and the descent is what bounds a walk that goes round.
  let outer = Guard {
    scope: Scope::Module,
    ..guard
  };

  admit_value(&declaration, outer, reader)?;

  // Recorded after the walk, so every name the declaration reads is already a
  // parameter ahead of the parameter whose default reads it.
  reader
    .transport
    .carry(&ident.sym, Crossing::Source(Box::new(declaration)));

  Ok(())
}

/// The expression a name was declared with, or `None` where the module declares
/// no such binding.
///
/// One walk from a name to its initializer, because both questions below start
/// with it: what a function crosses as, and whether a name the guard could not
/// resolve was a function at all. Two spellings of the same three links would
/// have been two chances to disagree about which link is optional.
fn initializer_of<'a>(expr: &'a Expr, reader: &'a Reader) -> Option<&'a Expr> {
  get_binding(expr, reader.traversal_state).and_then(|declarator| declarator.init.as_deref())
}

/// Whether the module declares `ident` as a function, which is what makes a name
/// the guard could not resolve a refusal rather than a call to hand back.
///
/// Three declarations answer yes here, and they are the three the reference
/// compiler refuses too: an arrow the transport could not take, because a block
/// body or a destructured parameter is a shape the evaluator answers no callback
/// for; a `function` of either spelling; and a binding written to after it was
/// declared, which is refused for the write rather than for its shape.
///
/// A `function` declaration is asked of the declaration list rather than of a
/// declarator, because it is hoisted and has no initializer to read.
fn the_module_declares_a_function(ident: &Ident, expr: &Expr, reader: &Reader) -> bool {
  matches!(
    reader.traversal_state.declared_as(ident),
    Some(DeclarationType::Function)
  ) || initializer_of(expr, reader).is_some_and(|init| matches!(init, Expr::Arrow(_) | Expr::Fn(_)))
}

/// What the guard admitted, and the name a refusal or a throw is reported
/// under.
///
/// The first two arms are the two ways a *native* function is reached: a method
/// on a receiver, and a global applied as a function. They are told apart
/// because only the second can name something that is not a function at all —
/// `Math` is a valid callee because its methods fold, which says nothing about
/// whether the name itself can be applied. The third is the author's own
/// function, reached through the name the module bound it under, which the
/// engine holds because its declaration crossed as one.
///
/// Only the first two are ever *read*. What reads one is the outermost call, and
/// [`Position`] is why a named callee never reaches that: the arm exists so the
/// admission answers with the binding it admitted rather than with a method it
/// did not, and a chain link's answer is discarded either way.
#[derive(Clone, Copy)]
enum Admitted<'a> {
  Method(&'a Atom),
  Global(&'a Atom),
  Named(&'a Atom),
}

impl<'a> Admitted<'a> {
  /// The method, global or binding the call names.
  fn name(self) -> &'a Atom {
    match self {
      Admitted::Method(name) | Admitted::Global(name) | Admitted::Named(name) => name,
    }
  }
}

/// Where the call under the guard sits, which is the whole of what decides who
/// owns a callee written as a bare name.
///
/// The only thing on this bridge that reads position at all, and it is a
/// parameter rather than something [`Guard`] carries, so the rest of the walk
/// keeps answering the call in front of it and nothing else.
#[derive(Clone, Copy, PartialEq)]
enum Position {
  /// The call the caller asked about, which the dispatch below still owns.
  Outermost,
  /// A call inside an expression the fold has already claimed.
  Inside,
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

/// The expression a parenthesised one wraps, however many layers deep.
///
/// Unwrapped in a loop rather than by recursing, because every caller asks this
/// before the guard descends and so has no nesting budget to spend. A loop needs
/// none.
fn without_parens(expr: &Expr) -> &Expr {
  let mut expr = expr;

  while let Expr::Paren(paren) = expr {
    expr = &paren.expr;
  }

  expr
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
///
/// Read through parentheses, because they change nothing about which name is
/// written and the reference compiler folds `(String)('a')` and `(Math).max(1, 2)`
/// exactly as it folds them bare.
pub(super) fn unshadowed_global<'a>(expr: &'a Expr, state: &StateManager) -> Option<&'a Atom> {
  let expr = without_parens(expr);

  match expr.as_ident() {
    Some(name) if is_valid_callee(expr) && get_binding(expr, state).is_none() => Some(&name.sym),
    _ => None,
  }
}

/// The [StyleX function the engine may call](engine_stylex_functions) that
/// `call` names, or `None` where it names none.
///
/// A name the *callback* binds is not one of them however the module spelled its
/// imports: the engine binds it when it runs the callback, and a value the guard
/// carried under the same name would be shadowed by it anyway.
fn a_stylex_function(call: &CallExpr, guard: Guard, reader: &Reader) -> Option<EngineCallable> {
  let Callee::Expr(callee) = &call.callee else {
    return None;
  };

  let callable = engine_callable(callee, reader.traversal_state)?;

  match guard.scope.binds(&callable.name) {
    true => None,
    false => Some(callable),
  }
}

/// Admits a call to a StyleX function: its arguments are values like any other,
/// and the name it is reached through carries the function itself.
///
/// An argument with no compile-time value is a rule *inside a callback* and a
/// shape handed back outside one, which is the same question the [applied
/// global](../../../../../CONTEXT.md#applied-global) answers by owning every one
/// of its calls. Inside a callback nothing below the fold can answer: the array
/// methods that would have run the body moved into the engine, so handing the
/// call on ends it at a sentence about the *method*, naming neither the argument
/// nor the reason. Outside one the dispatch may still own the call around this
/// one, and a rule raised here would take a fold away from it.
///
/// A call written on its own never reaches here at all: the fold walks a value,
/// and the outermost call stays the dispatch's. That is deliberate — it resolves
/// its arguments this compiler's own way, a theme reference included, and the
/// engine holds no value for one of those.
fn admit_a_stylex_function(
  callable: &EngineCallable,
  call: &CallExpr,
  guard: Guard,
  reader: &mut Reader,
) -> Result<(), Decline> {
  let inside_a_callback = guard.scope.inside_a_callback();

  admit_arguments(&call.args, guard, reader).map_err(|declined| match declined {
    Decline::NotACandidate if inside_a_callback => {
      Decline::rule(uncoercible_value(callable.function_name()))
    },
    declined => declined,
  })?;

  // A name reached directly holds the function; a namespace holds an object of
  // the properties the engine may call. Both come from the callable itself, so
  // the value under a name is a function of the name and the transport's
  // one-value-per-name rule cannot drop a second naming.
  let carried = match callable.reached {
    Reached::Directly => Carried::Function(callable.call()),
    Reached::AsAProperty => Carried::Object(
      EngineCallable::namespace_properties()
        .map(|(property, call)| (property.into(), Carried::Function(call)))
        .collect(),
    ),
  };

  reader
    .transport
    .carry(&callable.name, Crossing::Value(carried));

  Ok(())
}

/// Whether a call is one this module can hand to the engine whole.
///
/// Every boundary is checked here rather than at the outermost call, because a
/// chain hides its middle links: `"AB".toLocaleLowerCase().trim()` is a `trim`
/// whose receiver needs a locale.
///
/// Nearly everything answerable from syntax is answered before the walk, because
/// the walk resolves bindings and resolution is the only expensive thing here. So
/// the shape of the callee, the spelling of the method name and a receiver
/// written as a number are all settled while they are still free, and only an
/// expression this module intends to fold pays to have its names read. Two rules
/// do resolve, and each is named where it is applied: the amplification bound,
/// which is arithmetic on values rather than a shape, and the escaping-property
/// check, which is deliberately behind the walk so a chain is named for its
/// outermost cause.
///
/// `position` is read by one rule only — see [`admit_a_named_call`] — and is a
/// parameter rather than something [`Guard`] carries, so nothing else on the
/// walk can start depending on where it is.
fn admit_call<'a>(
  call: &'a CallExpr,
  position: Position,
  guard: Guard,
  reader: &mut Reader,
) -> Result<Admitted<'a>, Decline> {
  let Callee::Expr(callee) = &call.callee else {
    return Err(Decline::NotACandidate);
  };

  // Whatever the call around this one measured is not this call's business: only
  // the call an arrow is written directly inside says how often it runs. This
  // call sets it again below, for its own arguments.
  let guard = Guard {
    callback: None,
    ..guard
  };

  // `String(x)`, `Number(x)`, `Array(n)` and `Object(x)` are native JavaScript
  // functions, so they are folded by being called rather than by a conversion
  // written out here. A name the module bound is not one of them and is left to
  // the dispatch below, which calls the author's own function.
  if let Some(global) = unshadowed_global(callee, reader.traversal_state) {
    return admit_applied_global(global, call, guard, reader);
  }

  if let Some(name) = without_parens(callee).as_ident() {
    return admit_a_named_call(name, callee, call, position, guard, reader);
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
  let global = unshadowed_global(obj, reader.traversal_state);

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

  admit_amplification(&method.sym, obj, &call.args, guard, reader)?;

  // `Array.from` is the other spelling of a declared length, and the only static
  // that carries one: every remaining `Array` static answers a length its own
  // arguments write out.
  if let Some(global) = global
    && let Some(amplifier) = EntryAmplifier::named(global, Some(&method.sym))
  {
    admit_entry_amplification(amplifier, &call.args, guard, reader)?;
  }

  // Two things on the receiver, and both skipped for a global.
  //
  // A global the engine provides itself carries no value across the bridge: the
  // printed source names it and the language answers. It is admitted as a
  // receiver here and nowhere else — a global's *name* is not a value this fold
  // carries, and admitting it as one would let `['a'].concat(String)` fold a
  // function's own source text into a declaration.
  //
  // Then the count a callback among the arguments would repeat, read off the same
  // receiver — after it has been admitted, so a receiver the guard is about to
  // refuse is never read for a count nothing will use. A global holds no elements
  // to count and no static on one takes a callback this rule owns, so the reading
  // is not attempted there either.
  let counted = match global {
    None => {
      admit_value(obj, guard, reader)?;

      admitted_callback(&method.sym, obj, guard, reader)
    },
    Some(_) => None,
  };

  let inner = Guard {
    callback: counted,
    ..guard
  };

  admit_arguments(&call.args, inner, reader)?;

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
/// An argument the bridge cannot carry hands the call back rather than refusing
/// it. The bridge carries JavaScript values, and this compiler has values of its
/// own — a resolved theme reference, the injected function map, the environment
/// object — which have no JavaScript form to cross as, so the engine cannot be
/// the thing that answers for them. The dispatch below folds a global applied to
/// one, and raises [`uncoercible_value`] where it cannot, so the sentence a
/// refusal carries is still written in one place.
///
/// A rule the walk *did* name is still a refusal: it is about the argument the
/// author wrote rather than about what the bridge holds, and nothing below the
/// fold would name it again.
///
/// The one thing the guard does not answer is whether the global is a function
/// at all: that is the language's answer and is read off the engine in
/// [`Admitted::callable`].
fn admit_applied_global<'a>(
  global: &'a Atom,
  call: &CallExpr,
  guard: Guard,
  reader: &mut Reader,
) -> Result<Admitted<'a>, Decline> {
  if let Some(amplifier) = EntryAmplifier::named(global, None) {
    admit_entry_amplification(amplifier, &call.args, guard, reader)?;
  }

  admit_arguments(&call.args, guard, reader)?;

  Ok(Admitted::Global(global))
}

/// Whether a call whose callee is a bare name is one the engine can answer.
///
/// The callee is walked as a value like any other, so nothing about reaching the
/// function is asked here: a name the surrounding callback binds is the engine's
/// own and carries nothing, and a name the module bound to a function crosses as
/// the declaration it came from — the same carriage a callback *passed* by name
/// takes. What this adds is the dispatch, and the rule it applies is where the
/// call sits.
///
/// **The outermost call stays the dispatch's**, which is the line
/// [`admit_a_stylex_function`] already draws and for the same reason: below the
/// fold a call through a name is resolved this compiler's own way — a dynamic
/// style's own parameters, the injected function map and a resolved theme
/// reference are all answered there, and the engine holds no value for any of
/// them. Measured, that path already folds `inner('a')` to upstream's rule, so
/// taking the call would replace a working answer with a narrower one.
///
/// A call *inside* an expression the fold claimed has no such second answer.
/// Handing one back hands back the whole expression around it, and the method
/// that would have re-run the body moved into the engine — so this is the only
/// place that can answer, and the reason is the one the outermost call gives for
/// not being answered here.
fn admit_a_named_call<'a>(
  name: &'a Ident,
  callee: &Expr,
  call: &CallExpr,
  position: Position,
  guard: Guard,
  reader: &mut Reader,
) -> Result<Admitted<'a>, Decline> {
  if position == Position::Outermost {
    return Err(Decline::NotACandidate);
  }

  // The callee is an arrow this call runs once per evaluation of itself, rather
  // than once per element of a receiver — so what its body repeats is what the
  // expression around it repeats, and a length written into that body is bounded
  // like any other instead of refusing as a callback over something unmeasured.
  // No width travels with the count: a parameter here holds an argument, and an
  // argument's width is not something this reading measured.
  let callee_guard = Guard {
    callback: Some(Callback {
      repeats: guard.repeats,
      characters: None,
    }),
    ..guard
  };

  admit_value(callee, callee_guard, reader)?;

  // The arguments keep the guard's own `callback`, which this call already
  // cleared: an arrow handed to the author's own function is run by a body this
  // fold cannot see, so how often it runs is exactly what nothing here measured.
  admit_arguments(&call.args, guard, reader)?;

  Ok(Admitted::Named(&name.sym))
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
/// The rule is arithmetic rather than syntax: work out how long a string the
/// call would build, and refuse when that is past the ceiling. So a count may be
/// written out, named, or computed — `'x'.repeat(n)` and `'x'.repeat(2 * 2)` are
/// bounded by reading them, exactly as `'x'.repeat(4)` is — and what stays
/// refused is a length that cannot be read at all.
///
/// `repeat` multiplies its receiver, so the receiver's own length is half of the
/// product and a receiver whose length cannot be read leaves it unbounded. A
/// **call** is the receiver deliberately left unread: its answer is bounded per
/// link, and multiplying two allowed lengths is exactly how
/// `"x".repeat(1000000).repeat(1000000)` reaches a length neither of them is.
/// That is the rule that used to be spelled as "the receiver must not be a
/// call", and the product is what it was standing in for — a name holding a
/// bounded string is a receiver it never covered.
///
/// `padStart` and `padEnd` build to their count whatever the receiver holds, so
/// the count alone bounds them and a chain through one cannot multiply.
///
/// A call inside a callback is bounded by the product rather than refused: the
/// body runs once per element of the receiver the call around it was written on,
/// so the bound the source states is one factor and that element count is the
/// other. A receiver nothing counted leaves the product unbounded, and that is
/// the remainder the blanket refusal was standing in for all along.
///
/// Reading a length costs a fold that folded before nothing at all. The
/// name check above answers first, so every call to a method that is not one of
/// the three pays one scan of three names, exactly as it did. For the three, a
/// count and a receiver written out are matched as syntax and nothing is
/// evaluated. The only resolution this rule adds is for a *named* count or
/// receiver — a call that used to refuse outright — where the read is memoised
/// and the walk below would make it a moment later anyway.
fn admit_amplification(
  method: &Atom,
  receiver: &Expr,
  args: &[ExprOrSpread],
  guard: Guard,
  reader: &mut Reader,
) -> Result<(), Decline> {
  if !lists(&LENGTH_AMPLIFYING_METHODS, method) {
    return Ok(());
  }

  let ceiling = guard.ceilings.characters;
  let unreadable = || Decline::rule(unbounded_amplified_length(method, ceiling));

  let count = match args.first() {
    // `"x".padStart()` amplifies nothing, so there is no length to bound.
    None => return Ok(()),
    // A spread is a count that is not one argument, and the guard refuses a
    // spread everywhere else too.
    Some(ExprOrSpread {
      spread: Some(_), ..
    }) => return Err(unreadable()),
    Some(ExprOrSpread { expr, .. }) => resolved_count(expr, reader).ok_or_else(unreadable)?,
  };

  // Saturating because the product exists to be refused on, and a wrapped one
  // would admit.
  let per_evaluation = match method == "repeat" {
    true => receiver_length(receiver, guard, reader)
      .ok_or_else(unreadable)?
      .saturating_mul(count),
    false => count,
  };

  // A read bound bounds one evaluation, and a callback body runs once per element
  // of the receiver the call around it was written on. Where that count was read
  // the product is the bound; where it was not, the same written bound would be
  // multiplied by a number the source never states — `"x".repeat(999999)
  // .split("").map(() => "y".repeat(999999))` is two calls, each inside the
  // bound, building a terabyte between them.
  let repeats = guard.repeats.counted("string", method)?;

  if per_evaluation.saturating_mul(repeats) > ceiling {
    return Err(Decline::rule(amplified_length_too_large(
      method,
      count,
      per_evaluation,
      repeats,
      ceiling,
    )));
  }

  Ok(())
}

/// What a callback passed to this call would repeat, or `None` where the call
/// takes no callback the guard counts.
///
/// The receiver is read rather than the argument, because a callback's body runs
/// once per element of the receiver the call was written on — and by the time
/// this is asked that receiver has already been admitted, so the read is one the
/// fold was going to pay for anyway.
fn admitted_callback(
  method: &Atom,
  receiver: &Expr,
  guard: Guard,
  reader: &mut Reader,
) -> Option<Callback> {
  if !lists(&PER_ELEMENT_METHODS, method) {
    return None;
  }

  // A callback the guard admits but could not measure is still a callback, so it
  // is `Some` holding the unmeasured default rather than `None`: what tells the
  // two apart is whether an arrow in this position is a body that runs at all.
  Some(measured_receiver(receiver, guard, reader).unwrap_or_default())
}

/// What a callback over this receiver would repeat and hold, or `None` where the
/// guard cannot read the receiver at all.
///
/// One reading answers both, so the count and the width come off the same
/// measurement of the same value and cannot come to disagree.
fn measured_receiver(receiver: &Expr, guard: Guard, reader: &mut Reader) -> Option<Callback> {
  let depth = guard.depth;

  // The evaluator answers an array either as a list of its own or as the literal
  // it was written as, and both are one array here — the same two shapes the
  // inward conversion reads, for the same reason.
  let (elements, characters) = match &module_value_of(receiver, reader)? {
    EvaluateResultValue::Vec(items) => (
      items.len(),
      widest_of(items.iter().map(|item| rendered_characters(item, depth))),
    ),
    EvaluateResultValue::Expr(Expr::Array(ArrayLit { elems, .. })) => {
      // A spread stands for however many elements its operand holds, so the
      // written length is not the count — and a count read short is the one
      // reading that would admit a call nothing bounded. The literal arm is
      // answered by written length alone, so a spread has to leave it.
      if elems.iter().flatten().any(|elem| elem.spread.is_some()) {
        return None;
      }

      (
        elems.len(),
        widest_of(elems.iter().map(|elem| rendered_element(elem, depth))),
      )
    },
    _ => return None,
  };

  Some(Callback {
    repeats: guard.repeats.per_element(elements as u64),
    characters,
  })
}

/// The value `expr` holds *in the module*, or `None` where the module has none to
/// answer with.
///
/// The one home for what both speculative reads below share: the paren unwrapping
/// and the **call** they each refuse, whose answer is bounded per link so that
/// reading it is what would let two allowed counts multiply into one that is
/// neither.
///
/// **Why a name a callback binds cannot come back from here**, which is what
/// makes reading a receiver inside a callback safe at all. A module
/// `const parts = ['q']` beside `big.map(parts => parts.map(…))` spells one name
/// two ways, and answering the first where the call is made on the second would
/// count one evaluation against ten thousand. It cannot happen: the evaluator
/// resolves a reference through `StateManager::declaration_of`, which is keyed by
/// the full SWC `Id` — the symbol *and* its `SyntaxContext` — so the parameter
/// and the module binding are different keys and the parameter's has no
/// initializer. The resolver, not a check here, is what holds that; this is the
/// place that depends on it, so it is the place that says so.
fn module_value_of(expr: &Expr, reader: &mut Reader) -> Option<EvaluateResultValue> {
  match without_parens(expr) {
    Expr::Call(_) => None,
    _ => reader.resolve(expr),
  }
}

/// One written array element's rendered width.
///
/// A hole renders to nothing, which is what the language's own join does with it;
/// a spread stands for a count the source does not state, so it has no width.
/// Read from one place because both the receiver's own elements and a nested
/// array's are the same question.
fn rendered_element(elem: &Option<ExprOrSpread>, depth: Depth) -> Option<u64> {
  match elem {
    Some(ExprOrSpread { spread: None, expr }) => rendered_expr(expr, depth),
    Some(_) => None,
    None => Some(0),
  }
}

/// The widest of a receiver's elements, or `None` where one of them renders to a
/// width the guard could not read.
///
/// Any one unreadable element gives up on all of them, because which element a
/// callback's parameter will hold is not something the guard chooses.
fn widest_of(mut widths: impl Iterator<Item = Option<u64>>) -> Option<u64> {
  widths.try_fold(0, |widest, width| Some(widest.max(width?)))
}

/// How many characters one resolved value renders to under the language's own
/// `ToString`, or `None` where the guard cannot read it.
///
/// An object is one of those: it renders to `[object Object]` whatever it holds,
/// and treating that as a width would put a number in front of the engine that
/// says nothing about the value. A refusal is the honest answer.
fn rendered_characters(value: &EvaluateResultValue, depth: Depth) -> Option<u64> {
  match value {
    EvaluateResultValue::Expr(expr) => rendered_expr(expr, depth),
    EvaluateResultValue::Vec(items) => {
      let inner = depth.descend().ok()?;

      joined(
        items.len(),
        items.iter().map(|item| rendered_characters(item, inner)),
      )
    },
    _ => None,
  }
}

/// The same for a value the evaluator answered as the expression it was written
/// as.
fn rendered_expr(expr: &Expr, depth: Depth) -> Option<u64> {
  let inner = depth.descend().ok()?;

  match expr {
    Expr::Lit(Lit::Str(text)) => Some(atom_utf16_length(&text.value) as u64),
    // Read through the conversion every other number-to-string in this compiler
    // uses, so the width is the one the engine will actually build.
    Expr::Lit(Lit::Num(number)) => Some(to_js_string(number.value).len() as u64),
    Expr::Lit(Lit::Bool(truth)) => Some(match truth.value {
      true => "true".len() as u64,
      false => "false".len() as u64,
    }),
    Expr::Lit(Lit::Null(_)) => Some("null".len() as u64),
    Expr::Array(ArrayLit { elems, .. }) => joined(
      elems.len(),
      elems.iter().map(|elem| rendered_element(elem, inner)),
    ),
    _ => None,
  }
}

/// What a list of rendered widths comes to once the language joins them with a
/// comma, or `None` where one of them could not be read.
fn joined(count: usize, mut widths: impl Iterator<Item = Option<u64>>) -> Option<u64> {
  let separators = count.saturating_sub(1) as u64;

  widths.try_fold(separators, |total, width| {
    Some(total.saturating_add(width?))
  })
}

/// The count an amplifying call asks for, or `None` where the argument is not a
/// number this guard can read.
///
/// A literal is answered where it stands, because that is the common spelling
/// and reading it costs nothing. Anything else is resolved through the
/// evaluator, which is a [speculative read](../../../../../CONTEXT.md) like
/// every other the guard makes — and one the fold would pay for anyway, since a
/// call it admits evaluates the same argument a moment later.
///
/// Whatever it resolves to then goes through the compiler's own `ToNumber`,
/// because that is what the language does to it: `'x'.repeat('3')` repeats three
/// times and `'x'.repeat('lots')` repeats none. Reading the count any other way
/// would refuse an input the reference compiler folds, and bound a call by a
/// number the engine is not going to use.
fn resolved_count(expr: &Expr, reader: &mut Reader) -> Option<u64> {
  let resolved = match expr {
    Expr::Lit(_) => return count_of(to_js_number(expr)?),
    _ => as_expr(&reader.resolve(expr)?)?,
  };

  count_of(to_js_number(&resolved)?)
}

/// One resolved number as the bound it puts on what a call will build.
///
/// Truncated toward zero and floored there, which is what the language's own
/// `ToIntegerOrInfinity` does to it — so a fractional or negative count is
/// bounded exactly as the engine will read it, and the `RangeError` a negative
/// one really produces is left to the language to say, in its own words. A count
/// that is infinite is not a bound at all; `NaN` is zero, because `f64::max`
/// answers zero for one and so does the language.
fn count_of(value: f64) -> Option<u64> {
  match value.is_infinite() {
    true => None,
    false => Some(value.trunc().max(0.0) as u64),
  }
}

/// The receiver's own length in UTF-16 code units, or `None` where the guard
/// cannot read it.
///
/// A string written into the source is measured where it stands; anything else
/// is resolved, so a name holding a string is a receiver like the literal it was
/// given the name of. A **call** is refused rather than resolved, which is the
/// half of this rule that keeps per-link bounds from multiplying across a chain.
///
/// A name a callback binds is neither: the module cannot resolve it, and what it
/// holds is an element of a receiver the call around the callback measured — so
/// that element's width is the length, and a name nothing measured has none.
fn receiver_length(receiver: &Expr, guard: Guard, reader: &mut Reader) -> Option<u64> {
  let text = match without_parens(receiver) {
    Expr::Lit(Lit::Str(text)) => text.value.clone(),
    // A name the callback binds is answered from the element it was handed, and
    // this arm is what makes `['a','b'].map(x => x.repeat(3))` fold at all: the
    // module has no value for `x`, so without it there is no length to read.
    // Asked before the resolution rather than left to it — see
    // [`module_value_of`] for why the module could not answer for it anyway.
    Expr::Ident(ident) if guard.scope.binds(&ident.sym) => {
      return guard.scope.characters_of(&ident.sym);
    },
    _ => match module_value_of(receiver, reader)? {
      EvaluateResultValue::Expr(Expr::Lit(Lit::Str(text))) => text.value,
      _ => return None,
    },
  };

  Some(atom_utf16_length(&text) as u64)
}

/// The two calls whose result is one array element per unit of a length an
/// argument declares.
///
/// `Array(n)` is why this exists. The array it makes is *sparse*, so it looks
/// free and is: nothing is allocated until a later call in the chain fills,
/// copies, sorts or joins it, and by then the length is the engine's rather than
/// the guard's. The refusal still arrives — the entry ceiling reads the answer on
/// the way out — but it arrives after half a minute of work rather than before
/// it, which is the failure the ceilings were put in to prevent. Bounding the
/// declaration bounds every call that would go on to materialise one.
///
/// `Array.from(x)` is the same length read one property along, off `x`'s own. It
/// is here for `{ length: n }`, the object that declares a length without
/// holding it; a string or an array handed to `from` was either written out or
/// carried inward, and both of those the ceilings already bounded.
///
/// Every other name was measured against the same question — what is the result
/// length a function of? — and left out for the answer:
///
/// - `fill`, `copyWithin`, `reverse` and `sort` answer their receiver's own
///   length, so they add nothing to it.
/// - `slice`, `splice`, `concat` and `flat` answer a length no larger than the
///   elements their receiver and their arguments already hold.
/// - `map`, `filter` and `join` answer one element, or one element's text, per
///   element of a receiver.
///
/// Each of those is a length something already paid for, so the two below are the
/// whole of what a fold can be asked to build for free.
#[derive(Clone, Copy)]
enum EntryAmplifier {
  /// `Array(n)`, whose length is its single numeric argument.
  Constructor,
  /// `Array.from(x)`, whose length is the one `x` declares.
  From,
}

impl EntryAmplifier {
  /// The amplifier a call on an unshadowed global names, or `None` where the call
  /// declares no length.
  ///
  /// `method` is the static read off the global, and `None` where the global is
  /// applied as a function. One recogniser rather than a test at each of the two
  /// call sites, so the names are written once and the two sites cannot come to
  /// disagree about which spellings this rule owns.
  fn named(global: &Atom, method: Option<&Atom>) -> Option<Self> {
    match (&**global, method.map(|method| &**method)) {
      ("Array", None) => Some(Self::Constructor),
      ("Array", Some("from")) => Some(Self::From),
      _ => None,
    }
  }

  /// How the call is spelled, which is what a refusal names.
  fn name(self) -> &'static str {
    match self {
      Self::Constructor => "Array",
      Self::From => "Array.from",
    }
  }

  /// What the guard could read about the length the call declares.
  fn declared(self, args: &[ExprOrSpread], reader: &mut Reader) -> Declared {
    match self {
      Self::Constructor => constructor_length(args, reader),
      Self::From => length_property(args.first(), reader),
    }
  }
}

/// What the guard could read about the length a call declares.
///
/// Three answers rather than two, because the third is a rule of its own. Outside
/// a callback an unreadable length is the same as no length: the argument that
/// would say is a value the guard carried inward, and both ceilings already
/// bounded that. Inside one it is the dangerous case — `[{ length: 100000000 }]
/// .map(x => Array.from(x).length)` reaches the declaration through a parameter
/// the guard cannot resolve, and folded in sixty-eight seconds when this told the
/// two apart by returning `None` for both.
enum Declared {
  /// The elements the call will build.
  Length(u64),
  /// The call declares no length: its arguments are elements the source wrote
  /// out, or an array-like holding what it says it holds.
  Nothing,
  /// The argument that would say is not one the guard can read.
  Unreadable,
}

/// Whether a call declaring an array length is bounded well enough to evaluate.
///
/// The arithmetic [`admit_amplification`] does, in the other unit and read off a
/// declaration rather than off a receiver: work out how many elements the call
/// would build, and refuse when that is past the entry ceiling. It is that same
/// ceiling which refuses the array on the way *out*, so this changes when the
/// answer arrives rather than what it is — and when is the whole of the
/// difference between a refusal that costs half a minute and one that costs
/// nothing.
///
/// Outside a callback, a length it cannot read is admitted rather than refused —
/// which is where this parts company with an amplifying method's count. A count
/// that cannot be read leaves a product nothing bounds; a length that cannot be
/// read means the argument is a value the guard carried inward, which both
/// ceilings already bounded. Inside a callback that reasoning does not hold, and
/// [`Declared::Unreadable`] is why the read answers three things rather than two.
///
/// The unreadable rule is therefore asked *after* the length rather than in front
/// of it, unlike the string one: a call that declares no length is
/// `Array('a', 'b')`, whose elements the source wrote out, and refusing that
/// inside a callback would take away a fold nothing threatens.
fn admit_entry_amplification(
  amplifier: EntryAmplifier,
  args: &[ExprOrSpread],
  guard: Guard,
  reader: &mut Reader,
) -> Result<(), Decline> {
  let declared = amplifier.declared(args, reader);

  // A length the guard could not read is refused inside a callback, and only
  // there: that is where the declaration arrives through a parameter, so nothing
  // in front of the engine sees it. `[{length: 100000000}].map(x =>
  // Array.from(x).length)` folded in sixty-eight seconds when this told the two
  // apart by reading nothing.
  //
  // Refused whatever the element count came to, because it is the *length* that
  // is unreadable rather than the repeats: a receiver of one element still
  // declares an array of a hundred million.
  if guard.scope.inside_a_callback() && matches!(declared, Declared::Unreadable) {
    return Err(Decline::rule(amplification_inside_a_callback(
      "array",
      amplifier.name(),
    )));
  }

  let Declared::Length(declared) = declared else {
    return Ok(());
  };

  let ceiling = guard.ceilings.entries;

  // A callback body runs once per element of the receiver the call around it was
  // written on, so a length written into one declares that many arrays rather
  // than one. Where the receiver was never counted the product is unbounded, and
  // `['a', 'b'].map(x => Array(9999).fill(x))` is one bounded length multiplied
  // by a number the source never states.
  let repeats = guard.repeats.counted("array", amplifier.name())?;

  match declared.saturating_mul(repeats) > ceiling {
    true => Err(Decline::rule(amplified_entries_too_large(
      amplifier.name(),
      declared,
      repeats,
      ceiling,
    ))),
    false => Ok(()),
  }
}

/// What `Array(n)` declares about the length of its array.
///
/// `Array` declares a length only when it is handed exactly one argument and
/// that argument *is* a number: `Array(3)` is three holes, where `Array('3')` is
/// one element holding a string and `Array('a', 'b')` is two elements. So the
/// number is read as the language reads it rather than through `ToNumber`, which
/// is where this parts company with [`resolved_count`] — `'x'.repeat('3')`
/// repeats three times and `Array('3')` does not.
///
/// A number that is not a valid array length — fractional, negative, `NaN`,
/// infinite, or `2 ** 32` and up — declares nothing either, and is left to the
/// language: `Array` answers each of them with a `RangeError` before allocating,
/// so a ceiling in front of it would replace the accurate sentence with a
/// misleading one.
fn constructor_length(args: &[ExprOrSpread], reader: &mut Reader) -> Declared {
  // More than one argument, or none, is elements the source wrote out. So is a
  // spread, which is refused where it is written in any case.
  let [ExprOrSpread { spread: None, expr }] = args else {
    return Declared::Nothing;
  };

  let number = match expr.as_ref() {
    Expr::Lit(Lit::Num(number)) => number.value,
    _ => match reader.resolve(expr) {
      Some(EvaluateResultValue::Expr(Expr::Lit(Lit::Num(number)))) => number.value,
      // A value that resolved and is not a number is the single element it will
      // become; one that did not resolve is the length nobody can see.
      Some(_) => return Declared::Nothing,
      None => return Declared::Unreadable,
    },
  };

  match valid_array_length(number) {
    Some(length) => Declared::Length(length),
    None => Declared::Nothing,
  }
}

/// What an `Array.from` argument declares about the length it will build.
///
/// `{ length: n }` is a length declared without being held, which is what
/// `Array(n)` is bounded for one call earlier. The argument is *resolved* rather
/// than read as syntax, so a name holding the object and a spread that builds one
/// are the object they come to — the same reading the rest of this guard makes,
/// and the reason `{ ...{ length: n } }` is not a way round the bound.
///
/// The last `length` property wins, because that is the one the object ends up
/// with.
fn length_property(arg: Option<&ExprOrSpread>, reader: &mut Reader) -> Declared {
  // `Array.from()` with nothing to iterate throws, and a spread is refused where
  // it is written.
  let Some(ExprOrSpread { spread: None, expr }) = arg else {
    return Declared::Nothing;
  };

  let resolved = match reader.resolve(expr) {
    Some(resolved) => resolved,
    None => return Declared::Unreadable,
  };

  // Anything that is not an object holds what its length says — a string or an
  // array — and the ceilings bounded it where it was written or carried.
  let EvaluateResultValue::Expr(Expr::Object(object)) = resolved else {
    return Declared::Nothing;
  };

  let length = object.props.iter().rev().find_map(|prop| match prop {
    PropOrSpread::Prop(prop) => match prop.as_ref() {
      Prop::KeyValue(KeyValueProp { key, value }) if is_a_length_key(key) => Some(value),
      _ => None,
    },
    PropOrSpread::Spread(_) => None,
  });

  // An object with no own `length` is the empty array, and one whose length the
  // language will not accept is a throw it raises itself. Both declare nothing
  // this guard has to bound. `trunc` is `ToLength`'s own truncation, which is
  // what makes `{ length: 1.9 }` a length of one rather than none.
  match length
    .and_then(|length| to_js_number(length))
    .and_then(|number| valid_array_length(number.trunc()))
  {
    Some(length) => Declared::Length(length),
    None => Declared::Nothing,
  }
}

/// Whether a property name is the `length` an array-like declares.
///
/// Both spellings of the one key, because `{ 'length': n }` declares what
/// `{ length: n }` does. A computed key is not read: the evaluator answers a
/// resolved object, whose keys are settled by the time this sees them.
fn is_a_length_key(key: &PropName) -> bool {
  match key {
    PropName::Ident(name) => name.sym == "length",
    PropName::Str(name) => name.value.as_str() == Some("length"),
    _ => false,
  }
}

/// One number as the array length the language would make of it, or `None` where
/// the language rejects it instead.
///
/// Shared by both readers, because the range is the language's rather than this
/// guard's: a length outside it raises a `RangeError` before anything is
/// allocated — `Array` from its argument, `Array.from` from `ArrayCreate` — so
/// falling through to that costs nothing and says more than a ceiling could.
/// `Array.from({ length: Infinity })` is the case that makes it worth sharing:
/// bounded here it would name `2 ** 53 - 1`, a number the language never reaches
/// because it refuses the length first.
///
/// The two readers differ only in how they arrive at the number — `Array(n)`
/// takes its argument as written, and an array-like's `length` comes through
/// `ToLength` — which is why the range is checked here and the coercion is not.
fn valid_array_length(number: f64) -> Option<u64> {
  match VALID_ARRAY_LENGTHS.contains(&number) && number.fract() == 0.0 {
    true => Some(number as u64),
    false => None,
  }
}

/// An argument is admitted when it is a value the walk carries — an arrow among
/// them, which is how a callback and an own conversion method reach the engine.
/// Every argument of a call, walked as a value.
///
/// One loop rather than one per calling shape, so a position that maps the
/// refusal — an applied global's, a StyleX function's — differs from the others
/// in the mapping alone and not in what it walked.
fn admit_arguments(
  args: &[ExprOrSpread],
  guard: Guard,
  reader: &mut Reader,
) -> Result<(), Decline> {
  for arg in args {
    admit_argument(arg, guard, reader)?;
  }

  Ok(())
}

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

/// Whether an arrow reads nothing but the names it binds and names the module
/// resolves. Anything else would need a scope the engine does not have.
///
/// The arrow itself is not analysed — the engine parses it, so a destructured
/// parameter and a block body are shapes the language answers rather than
/// shapes this guard has to recognise. What the walk still does is name what
/// the arrow binds, so a read of one is not asked of the module, and apply to
/// the body the rules every other position gets: a callback body is source that
/// really runs.
fn admit_arrow(arrow: &ArrowExpr, guard: Guard, reader: &mut Reader) -> Result<(), Decline> {
  let mut bindings = Bindings::default();
  let mut first = 0;

  for (position, param) in arrow.params.iter().enumerate() {
    bindings.pattern(param, guard.depth)?;

    // The names the first parameter binds are the ones handed an element of the
    // receiver, whether that parameter is a plain name or destructures one.
    if position == 0 {
      first = bindings.names.len();
    }
  }

  // What the call this arrow was written inside measured for it. A parameter with
  // a default may be handed something else entirely, so a defaulted parameter
  // list takes neither the width nor the count.
  let measured = match bindings.evaluates.is_empty() {
    true => guard.callback,
    false => None,
  };

  let (elements, repeats) = match measured {
    Some(callback) => (
      Elements {
        named: first,
        characters: callback.characters,
      },
      callback.repeats,
    ),
    None => (Elements::default(), Repeats::Unmeasured),
  };

  let inner = bindings.enter(&guard, elements, repeats, reader)?;

  match arrow.body.as_ref() {
    BlockStmtOrExpr::Expr(body) => admit_value(body, inner, reader),
    BlockStmtOrExpr::BlockStmt(body) => admit_block(&body.stmts, inner, reader),
  }
}

/// What a set of patterns binds, and the expressions those patterns evaluate.
///
/// Both in one walk, because the names have to be in scope before an expression
/// beside them is walked and the two are written into the same pattern.
#[derive(Default)]
struct Bindings<'a> {
  /// The names the patterns introduce.
  names: Vec<Atom>,
  /// The expressions the patterns evaluate where they stand: a default value,
  /// and a computed key.
  evaluates: Vec<&'a Expr>,
}

impl<'a> Bindings<'a> {
  /// Records what `pat` binds and what it evaluates, or declines a pattern that
  /// binds no name this walk can put in scope.
  ///
  /// Nesting is spent here as it is everywhere else on this bridge: a pattern
  /// is printed into the transport with the arrow it belongs to, so a
  /// destructuring nested deeper than the engine's parser can descend has to be
  /// refused before it reaches that parser.
  fn pattern(&mut self, pat: &'a Pat, depth: Depth) -> Result<(), Decline> {
    let inner = depth.descend()?;

    match pat {
      Pat::Ident(ident) => self.names.push(ident.sym.clone()),
      // A hole binds nothing and skips an element, which the engine does itself.
      Pat::Array(array) => {
        for element in array.elems.iter().flatten() {
          self.pattern(element, inner)?;
        }
      },
      Pat::Object(object) => {
        for prop in &object.props {
          match prop {
            // `{ a }` and `{ a = 1 }`: the key is the name, and the default is
            // an expression beside it.
            ObjectPatProp::Assign(shorthand) => {
              self.names.push(shorthand.key.sym.clone());

              if let Some(default) = &shorthand.value {
                self.evaluates.push(default);
              }
            },
            // `{ a: b }`, and `{ [k]: b }` whose key is a value in its own
            // right.
            ObjectPatProp::KeyValue(entry) => {
              if let PropName::Computed(key) = &entry.key {
                self.evaluates.push(&key.expr);
              }

              self.pattern(&entry.value, inner)?;
            },
            ObjectPatProp::Rest(rest) => self.pattern(&rest.arg, inner)?,
          }
        }
      },
      Pat::Rest(rest) => self.pattern(&rest.arg, inner)?,
      Pat::Assign(assign) => {
        self.pattern(&assign.left, inner)?;
        self.evaluates.push(&assign.right);
      },
      // `[o.a] = …` assigns through a member rather than binding a name, so
      // there is nothing to put in scope; an invalid pattern is not a shape to
      // reason about at all.
      Pat::Expr(_) | Pat::Invalid(_) => return Err(Decline::NotACandidate),
    }

    Ok(())
  }

  /// The guard with these names in scope, once the expressions beside them have
  /// been walked.
  ///
  /// The order is the whole of this: a pattern's own expressions are evaluated
  /// where the pattern is, so they are walked with every name already bound —
  /// reading one declared later throws where the language throws, rather than
  /// quietly resolving to a module name the binding shadows. Written once here
  /// because both callers depend on it and neither states it.
  fn enter<'b>(
    &'b self,
    guard: &'b Guard,
    elements: Elements,
    repeats: Repeats,
    reader: &mut Reader,
  ) -> Result<Guard<'b>, Decline> {
    let inner = guard.binding(&self.names, elements, repeats);

    for expr in &self.evaluates {
      admit_value(expr, inner, reader)?;
    }

    Ok(inner)
  }
}

/// A block, admitted as a scope of its own: what it declares is bound before
/// its statements are walked, so a declaration reads as itself rather than as a
/// module name it shadows.
///
/// A `var` is function-scoped and is bound here as though it were not. That
/// only narrows what is visible — a read of one from an enclosing block finds
/// no binding and the call is handed back — so it costs a fold rather than
/// answering one wrongly.
fn admit_block(stmts: &[Stmt], guard: Guard, reader: &mut Reader) -> Result<(), Decline> {
  let outer = guard.descend()?;
  let mut bindings = Bindings::default();

  for stmt in stmts {
    if let Stmt::Decl(Decl::Var(declaration)) = stmt {
      for declarator in &declaration.decls {
        bindings.pattern(&declarator.name, outer.depth)?;
      }
    }
  }

  // A block declares values its own body built, so nothing here is an element of
  // a measured receiver — and the repeats are the ones the body around it is
  // already counted at.
  let inner = bindings.enter(&outer, Elements::default(), outer.repeats, reader)?;

  for stmt in stmts {
    admit_statement(stmt, inner, reader)?;
  }

  Ok(())
}

/// A statement inside a callback body.
///
/// The set is the statements that compute a value and hand it back, which is
/// all a callback is for. What is left out is left out for a reason, and each
/// one is written here.
///
/// A **loop** is bounded by the engine ([`MAX_LOOP_ITERATIONS`]), but the
/// count that bound is applied to lives on the *call frame* — so a callback
/// invoked once per element starts a fresh count every time, and the bound is
/// multiplied by an element count the source never states. That is the same
/// arithmetic [`admit_amplification`] refuses inside a callback, and every loop
/// this walk can reach is inside one, since a statement is only ever walked in
/// a callback body.
///
/// A **function or class declaration** carries a body this walk does not read,
/// and neither does the value walk read one written as an expression.
///
/// Both are refusals and not "not mine". The exclusion is a boundary this
/// module owns with a reason written down, and a boundary like that has to
/// answer in this module's words — handing the shape back instead ends it at
/// the dispatch's `Unsupported expression: ArrowFunctionExpression`, which
/// names the callback rather than the statement inside it and leaves an author
/// to guess.
///
/// An assignment is not among them, because it is an expression rather than a
/// statement: `v = 1;` is a `Stmt::Expr`, so it is answered by the value walk,
/// which does not model it and hands the call back like every other expression
/// kind it does not read.
fn admit_statement(stmt: &Stmt, guard: Guard, reader: &mut Reader) -> Result<(), Decline> {
  let inner = guard.descend()?;

  match stmt {
    Stmt::Expr(ExprStmt { expr, .. }) => admit_value(expr, inner, reader),
    Stmt::Return(ReturnStmt { arg, .. }) => match arg {
      Some(value) => admit_value(value, inner, reader),
      None => Ok(()),
    },
    // The names were bound by the block around this, so only the initialisers
    // are walked here.
    Stmt::Decl(Decl::Var(declaration)) => {
      for declarator in &declaration.decls {
        if let Some(init) = &declarator.init {
          admit_value(init, inner, reader)?;
        }
      }

      Ok(())
    },
    Stmt::Block(block) => admit_block(&block.stmts, inner, reader),
    Stmt::If(branch) => {
      admit_value(&branch.test, inner, reader)?;
      admit_statement(&branch.cons, inner, reader)?;

      match &branch.alt {
        Some(alt) => admit_statement(alt, inner, reader),
        None => Ok(()),
      }
    },
    // A stray semicolon binds nothing and evaluates nothing.
    Stmt::Empty(_) => Ok(()),
    _ => Err(Decline::rule(unfoldable_statement(get_stmt_node_kind(
      stmt,
    )))),
  }
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
    if string.len() as u64 > outward.ceilings.characters {
      return Err(Decline::rule(folded_string_too_large(
        outward.ceilings.characters,
      )));
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

  if keys.len() as u64 > outward.ceilings.entries {
    return Err(Decline::rule(object_size_too_large(
      outward.ceilings.entries,
    )));
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
  // Every arm of `to_value` answers one of the two shapes `as_expr` reads, so
  // nothing else is reachable by construction — and a refusal is answered rather
  // than a panic if that ever stops holding.
  as_expr(&value)
    .ok_or_else(|| Decline::rule(unfoldable_fold_result("value of an unreadable kind")))
}

/// One evaluated value as the expression it spells, where it spells one.
///
/// An array is the one case that has to be rebuilt rather than cloned, by the
/// evaluator's own conversion rather than by a second copy of it here. Shared
/// between the two positions that ask — a folded property on the way out, and a
/// resolved amplification count on the way in — so the two cannot come to
/// disagree about which values have an expression form.
fn as_expr(value: &EvaluateResultValue) -> Option<Expr> {
  match value {
    EvaluateResultValue::Expr(expr) => Some(expr.clone()),
    EvaluateResultValue::Vec(items) => evaluate_result_vec_to_array_expr(items),
    _ => None,
  }
}

/// An array's `length`, bounded: the count the conversion loop below reads.
///
/// The two ways it can fail say different things, because they are different
/// faults. A length past the entry ceiling is the bound, and names it. A
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

  if length > outward.ceilings.entries as f64 {
    return Err(Decline::rule(array_length_too_large(
      outward.ceilings.entries,
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
/// that names nothing pay nothing, and costs the memo nothing either, because
/// what the memo holds is a compiled script rather than a function.
///
/// The module is assembled here rather than by `create_module`, which takes
/// `&Expr` and clones it — so going through it means cloning the subtree once
/// to build the `Expr` and once more inside. The printer needs an owned tree
/// either way, because it drops the spans in place before emitting.
fn print_fold(call: &CallExpr, params: Vec<Pat>) -> String {
  let folded = Expr::Call(call.clone());

  let printed = match params.is_empty() {
    true => folded,
    false => create_arrow_expression_with_params(params, folded),
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
