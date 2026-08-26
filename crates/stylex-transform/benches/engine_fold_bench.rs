//! What the engine-backed fold costs.
//!
//! A method call that carries its own value is folded by printing it as source
//! and handing it to a JavaScript engine, rather than by matching its name
//! against a table. That buys the whole prototype surface and costs a round
//! trip, and this file is the three prices that round trip is made of.
//!
//! **Cold start** is what the first fold in a process pays and no later one
//! does: the engine is one thread-local built on first use, so a file with no
//! foldable call never pays it and a file with a thousand pays it once.
//!
//! **A warm fold** is what every call after that costs, end to end through the
//! evaluator — the guard walk, the print, the engine's parse and evaluation,
//! and the conversion of the answer back into the evaluator's own value. That
//! is the `fold` leg of the second group.
//!
//! **The round trip** is what that fold adds over the JavaScript it exists to
//! run, read as the gap between the `fold` leg and the `engine` leg beside it —
//! the same source, handed straight to a warm engine. One pair rather than two
//! groups, because a warm fold measured on its own and a warm fold measured
//! next to the engine are the same measurement.
//!
//! That gap is an upper bound on the round trip, not the round trip itself. The
//! `fold` leg enters through `evaluate`, so it carries the evaluator's own entry
//! cost — the memo lookup, the per-call state — alongside the guard walk, the
//! print and the conversion of the answer back. Nothing here separates the two,
//! and the bound is what a later ticket has to move.
//!
//! Every fold through the evaluator runs inside `GLOBALS.set`, because the fold
//! can reach the code-frame path and that path calls `Mark::new()`. Why that is
//! not optional is in `guidelines/PERFORMANCE.md` under "Writing a bench". The
//! cold-start group is the engine alone and reaches none of it.

use std::hint::black_box;

use boa_engine::{Context, Source};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use stylex_ast::ast::convertors::convert_atom_to_string;
use stylex_structures::stylex_options::StyleXOptions;
use stylex_transform::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  structures::{functions::FunctionMap, state_manager::StateManager},
  utils::js::evaluate::evaluate,
};
use swc_core::{
  common::{FileName, GLOBALS, Globals, SourceMap, input::StringInput, sync::Lrc},
  ecma::{
    ast::{EsVersion, Expr, Lit, Module, ModuleItem, Stmt},
    parser::{EsSyntax, Parser, Syntax, lexer::Lexer},
  },
};

/// The loop-iteration limit `engine_fold`'s own context is built with.
///
/// Duplicated rather than shared because the constant is private to the module
/// under measurement, and a bench that reached into it would be measuring a
/// different engine than the one that ships. Its only effect here is that the
/// cold-start leg builds the same context the fold builds.
const MAX_LOOP_ITERATIONS: u64 = 10_000_000;

/// One measured shape: what to call it, what to fold, and what folding it must
/// answer.
struct Leg {
  name: &'static str,
  /// Written the way the fold prints it — minified, no trailing semicolon — so
  /// the engine leg can be handed the same text the fold would hand it, rather
  /// than a prettier spelling that parses differently.
  source: &'static str,
  /// The answer, spelled the way the language spells it. One field serves both
  /// sides of the bridge: the engine's own `String(value)` and the evaluator's
  /// folded value render the same, so neither side needs an expectation of its
  /// own that could drift from the other's.
  answer: &'static str,
}

/// The shapes that cost differently: a string method with no arguments, a
/// callback the engine invokes once per element, a chain that folds at every
/// link, and an answer that comes back as an array and so runs the outward half
/// of the bridge.
///
/// They are the same shapes `perf_fixtures/engine-fold.js` is built from, and
/// they avoid a mutating method for the same reason it does: the baseline is
/// only a baseline if the revision before the change can be measured on it too.
const LEGS: &[Leg] = &[
  Leg {
    name: "string",
    source: r#""  read  ".trim()"#,
    answer: "read",
  },
  Leg {
    name: "callback",
    source: r#"[4,8,12,16].map(step=>step+"px").join(" ")"#,
    answer: "4px 8px 12px 16px",
  },
  Leg {
    name: "chain",
    source: r#""  rgba(0,0,0,.2)|rgba(0,0,0,.4)  ".trim().split("|").join(" ")"#,
    answer: "rgba(0,0,0,.2) rgba(0,0,0,.4)",
  },
  Leg {
    name: "array-answer",
    source: r#"["-webkit-sticky"].concat(["sticky"])"#,
    answer: "-webkit-sticky,sticky",
  },
];

fn parse(source: &str) -> Module {
  let cm: Lrc<SourceMap> = Default::default();
  let fm = cm.new_source_file(FileName::Anon.into(), source.to_owned());
  let lexer = Lexer::new(
    Syntax::Es(EsSyntax::default()),
    EsVersion::EsNext,
    StringInput::from(&*fm),
    None,
  );
  let mut parser = Parser::new_from(lexer);

  match parser.parse_module() {
    Ok(module) => module,
    Err(error) => panic!("Failed to parse `{source}`: {:#?}", error),
  }
}

/// The single expression statement each leg's source is.
fn expression(source: &str) -> Expr {
  let module = parse(source);

  let expr = module.body.iter().find_map(|item| match item {
    ModuleItem::Stmt(Stmt::Expr(stmt)) => Some(stmt.expr.as_ref().clone()),
    _ => None,
  });

  match expr {
    Some(expr) => expr,
    None => panic!("`{source}` is not an expression statement"),
  }
}

/// A folded value as text, rendered the way the language renders it — an array
/// as its elements joined by commas, which is what `String([a, b])` answers.
///
/// That is what lets one recorded answer per leg serve both the evaluator's
/// value and the engine's, so the two sides of the bridge cannot come to be
/// checked against expectations that disagree.
///
/// `None` for anything else, which is what a leg that stopped folding produces
/// and what the assertion below reports.
fn fold_text(value: &EvaluateResultValue) -> Option<String> {
  match value {
    EvaluateResultValue::Expr(Expr::Lit(Lit::Str(string))) => {
      Some(convert_atom_to_string(&string.value))
    },
    EvaluateResultValue::Expr(Expr::Lit(Lit::Num(number))) => Some(number.value.to_string()),
    EvaluateResultValue::Vec(items) => {
      let rendered = items
        .iter()
        .map(|item| match fold_text(item) {
          Some(text) => text,
          None => String::from("?"),
        })
        .collect::<Vec<_>>()
        .join(",");

      Some(rendered)
    },
    _ => None,
  }
}

/// The state one iteration folds against.
///
/// Rebuilt per iteration rather than reused: the evaluator memoizes what it
/// folded, so a second fold against one state would time the memo rather than
/// the round trip. Building it is setup and stays outside the measurement.
fn state() -> StateManager {
  StateManager::new(StyleXOptions::default())
}

/// Panics unless the leg folded to the value it is recorded as folding.
///
/// A refusal, a deopt and a memo hit are all fast, so a leg that got quick
/// because the work stopped happening is indistinguishable from a win. This is
/// also what warms the thread-local engine, so the group below times a warm
/// fold rather than one cold fold and many warm ones.
fn assert_folds_to_its_answer(leg: &Leg, expr: &Expr) {
  let name = leg.name;
  let result = evaluate(expr, &mut state(), &FunctionMap::default());

  let folded = match (result.confident, result.value.as_ref()) {
    (true, Some(value)) => fold_text(value),
    (confident, value) => panic!(
      "the `{name}` leg answered confident={confident}, value={value:?} — the \
       group below would be timing that rather than a fold"
    ),
  };

  match folded {
    Some(folded) => assert_eq!(
      folded, leg.answer,
      "the `{name}` leg folded to `{folded}` where it folded `{}`; a leg that \
       folds something else is no longer comparable with the numbers already \
       published",
      leg.answer
    ),
    None => panic!("the `{name}` leg folded a value this bench cannot read"),
  }
}

/// Panics unless `engine` answers the leg with the value it is recorded as
/// answering.
///
/// The engine's own `String(value)`, which is why one recorded answer covers
/// this side too. A context that came back answering `undefined` for everything
/// would otherwise report as a very fast cold start.
fn assert_engine_answers(leg: &Leg, engine: &mut Context) {
  let name = leg.name;

  let answered = match engine.eval(Source::from_bytes(leg.source)) {
    Ok(value) => value,
    Err(error) => panic!("the engine refused the `{name}` leg: {error}"),
  };

  let answered = match answered.to_string(engine) {
    Ok(text) => text.to_std_string_lossy(),
    Err(error) => panic!("the `{name}` leg answered a value with no text: {error}"),
  };

  assert_eq!(
    answered, leg.answer,
    "the engine answered the `{name}` leg with `{answered}` where it answered \
     `{}`",
    leg.answer
  );
}

/// The engine the fold builds on first use, with the one runtime limit its
/// default leaves open.
///
/// Dropped at the end of each iteration, where the fold's own engine is
/// `ManuallyDrop`. That difference is deliberate and not a divergence: the leak
/// there is about the order two thread-locals are dropped in *at thread exit*,
/// which nothing here reaches. Building and dropping a context mid-run is
/// ordinary.
fn cold_engine() -> Context {
  let mut engine = Context::default();

  engine
    .runtime_limits_mut()
    .set_loop_iteration_limit(MAX_LOOP_ITERATIONS);

  engine
}

/// What the first fold in a process pays and no later one does.
///
/// One leg, not one per shape. Measured per shape the four came back within 8%
/// of each other — 114 to 126 microseconds — against warm engine costs spanning
/// 2.3 to 10.7, which says the number is the context construction and not the
/// JavaScript. Reporting it four times would imply a leg-dependence that is not
/// there, so the cheapest shape stands for all of them.
fn cold_start_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("EngineFoldColdStart");

  let leg = match LEGS.first() {
    Some(leg) => leg,
    None => panic!("there is nothing to price"),
  };

  // Asserted through the engine rather than through the evaluator, because this
  // group is the engine alone and reaches no fold.
  assert_engine_answers(leg, &mut cold_engine());

  group.bench_function("build-and-answer", |b| {
    b.iter(|| black_box(cold_engine().eval(Source::from_bytes(black_box(leg.source)))))
  });

  group.finish();
}

/// A warm fold, beside the JavaScript it exists to run.
///
/// `engine` is a warm context handed the source the fold would print; `fold` is
/// the same expression through the evaluator, which is the warm fold itself.
/// The gap between them bounds what the fold adds over the language — the guard
/// walk, the print and the conversion of the answer back, plus the evaluator's
/// own cost of being entered, which nothing here tells apart from them.
fn round_trip_benchmarks(c: &mut Criterion) {
  let globals = Globals::default();

  GLOBALS.set(&globals, || {
    let mut group = c.benchmark_group("EngineFoldRoundTrip");
    let functions = FunctionMap::default();
    let mut engine = cold_engine();

    for leg in LEGS {
      let name = leg.name;
      let expr = expression(leg.source);

      assert_folds_to_its_answer(leg, &expr);
      assert_engine_answers(leg, &mut engine);

      group.bench_function(format!("engine/{name}"), |b| {
        b.iter(|| black_box(engine.eval(Source::from_bytes(black_box(leg.source)))))
      });

      group.bench_function(format!("fold/{name}"), |b| {
        b.iter_batched(
          state,
          |mut state| {
            black_box(evaluate(
              black_box(&expr),
              black_box(&mut state),
              black_box(&functions),
            ))
          },
          BatchSize::SmallInput,
        )
      });
    }

    group.finish();
  });
}

criterion_group!(
  engine_fold_benches,
  cold_start_benchmarks,
  round_trip_benchmarks
);
criterion_main!(engine_fold_benches);
