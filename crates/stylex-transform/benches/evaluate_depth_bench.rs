//! How the fold's cost scales with the depth of the expression it folds.
//!
//! The evaluator memoizes by a structural, span-insensitive hash of the
//! expression it is about to fold, and it asks for one at every level of a
//! nested expression. The key is a hash of the whole remaining subtree, so a
//! fold that descends `n` levels hashes `n + (n-1) + (n-2)` … nodes: the memo
//! that exists to avoid repeated work pays for the subtree to decide whether it
//! can avoid it. The result is a fold that costs about four times as much for
//! twice the depth.
//!
//! That curve is what these benchmarks pin. The default ceiling bounds depth at
//! 32, so nothing a project writes reaches the interesting part of it -- these
//! raise the ceiling well past the default on purpose, because the subject is
//! the shape of the curve rather than the time any real input spends. A key
//! composed incrementally from its children's hashes reports here as a
//! flattened curve; one that keeps the whole-subtree walk reports as the same
//! one. That comparison has been run: the composed key flattens this curve and
//! costs 14-42% on every fixture `evaluate_bench` measures, which is
//! `docs/adr/0006-an-incremental-memo-key-was-built-and-measured-slower.md` and
//! is why this group still reports the quadratic.
//!
//! The counted, machine-independent half of the same measurement lives in
//! `stylex_utils`' `key_cost_scaling_tests`, which pins the node walk in bytes
//! rather than in milliseconds. This file is what says whether the bytes matter.
//!
//! Every fold here runs inside `GLOBALS.set`, because the fold can reach the
//! code-frame path and that path calls `Mark::new()`. Why that is not optional
//! is in `guidelines/PERFORMANCE.md` under "Writing a bench".

use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use stylex_state::{
  common::fill_state_declarations, evaluate_result_value::EvaluateResultValue,
  functions::FunctionMap, state_manager::StateManager,
};
use stylex_structures::{core_stylex_options::CoreStyleXOptions, stylex_options::StyleXOptions};
use stylex_transform::shared::utils::js::evaluate::evaluate;
use stylex_utils::hash::{stable_hash_unspanned, stable_hash_wide};
use swc_core::{
  common::{FileName, GLOBALS, Globals, SourceMap, input::StringInput, sync::Lrc},
  ecma::{
    ast::{Decl, EsVersion, Expr, Lit, Module, ModuleItem, Stmt},
    parser::{EsSyntax, Parser, Syntax, lexer::Lexer},
    utils::drop_span,
  },
};

/// The depths measured. Each doubling is one point on the curve; four points is
/// enough to read a quadratic apart from a linear one and cheap enough to run
/// under the repo's default two-second measurement window.
const DEPTHS: [usize; 4] = [30, 60, 120, 240];

/// The ceiling the fold runs under here, raised past every depth above so the
/// benchmark measures a fold rather than a refusal. Where the shipped default
/// (32) comes from is `stylex_structures::evaluation_depth`.
const CEILING: usize = 512;

/// A module declaring `MY_CONST` and then stating `MY_CONST` under `depth`
/// levels of `+ 1`.
///
/// The tower is the same shape the evaluation-depth cases are measured against:
/// every level adds one to the folded value, so a fold that stopped early would
/// produce a different number rather than the same one.
fn tower_module(depth: usize) -> String {
  let mut source = String::from("const MY_CONST = 5;\n");

  source.push_str(&"(".repeat(depth));
  source.push_str("MY_CONST");
  source.push_str(&" + 1)".repeat(depth));
  source.push(';');

  source
}

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
    Err(error) => panic!("Failed to parse a tower module: {:#?}", error),
  }
}

/// The declaration `MY_CONST` resolves through, and the tower to fold.
struct Tower {
  module: Module,
  expr: Expr,
}

fn tower(depth: usize) -> Tower {
  let module = parse(&tower_module(depth));
  let expr = tower_expr(&module);

  Tower { module, expr }
}

/// The last expression statement of `module` -- the expression the benchmark is
/// about, as opposed to the declaration it reads.
fn tower_expr(module: &Module) -> Expr {
  let expr = module
    .body
    .iter()
    .filter_map(|item| match item {
      ModuleItem::Stmt(Stmt::Expr(expr_stmt)) => Some(expr_stmt.expr.as_ref().clone()),
      _ => None,
    })
    .next_back();

  match expr {
    Some(expr) => expr,
    None => panic!("every source built here ends in an expression statement"),
  }
}

/// A state manager that resolves `MY_CONST` and will descend past [`CEILING`]
/// levels before refusing.
fn state(module: &Module) -> StateManager {
  let mut state = StateManager::new(StyleXOptions {
    core: CoreStyleXOptions::default().maybe_max_evaluation_depth(Some(CEILING)),
    ..Default::default()
  });

  for item in &module.body {
    if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) = item {
      for declarator in &var_decl.decls {
        fill_state_declarations(&mut state, declarator);
      }
    }
  }

  state
}

/// Panics unless folding `tower` produced `5 + depth`.
///
/// Every level of the tower adds one, so the folded number is the only value the
/// full descent produces -- which makes this the check that the benchmark is
/// timing a fold rather than a refusal. Without it a ceiling left at the default
/// would time 32 levels of descent plus a deopt, at every depth, and report it
/// as a flat curve.
fn assert_folds_to_depth(tower: &Tower, depth: usize) {
  let mut state = state(&tower.module);
  let result = evaluate(&tower.expr, &mut state, &FunctionMap::default());

  let folded = match (result.confident, result.value.as_ref()) {
    (true, Some(EvaluateResultValue::Expr(Expr::Lit(Lit::Num(number))))) => number.value,
    (confident, value) => panic!(
      "a {depth}-level tower folded to confident={confident}, value={value:?}, \
       which is not a number -- the benchmark below would be timing that instead \
       of a fold"
    ),
  };

  assert_eq!(
    folded,
    (5 + depth) as f64,
    "a {depth}-level tower folded to {folded} rather than to 5 + {depth}"
  );
}

/// The fold, end to end: what a nested expression actually costs.
fn fold_benchmarks(c: &mut Criterion) {
  let globals = Globals::default();

  GLOBALS.set(&globals, || {
    let mut group = c.benchmark_group("EvaluateDepth");
    let functions = FunctionMap::default();

    for depth in DEPTHS {
      let tower = tower(depth);

      assert_folds_to_depth(&tower, depth);

      // Batched rather than plain `iter`, so building the state manager is not
      // measured. A fold cannot reuse one: `seen` memoizes what it folded, so a
      // second iteration against the same state would hit the memo and time
      // nothing.
      group.bench_function(format!("arithmetic/{depth}"), |b| {
        b.iter_batched(
          || state(&tower.module),
          |mut state| {
            black_box(evaluate(
              black_box(&tower.expr),
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

/// One memo key over the whole tower -- the per-level cost the fold above pays
/// once for every level it descends. Linear here, quadratic there.
fn key_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("StructuralKeyDepth");

  for depth in DEPTHS {
    let tower = tower(depth);

    group.bench_function(format!("arithmetic/{depth}"), |b| {
      b.iter(|| black_box(stable_hash_unspanned(black_box(&tower.expr))))
    });
  }

  group.finish();
}

/// What the key's fallback arm costs, either side of the boundary that selects
/// it.
///
/// `stable_hash_unspanned` hashes the shapes it covers in place, and hands
/// everything else to `stable_hash(&drop_span(path.clone()))` -- a deep clone of
/// the subtree, span-stripped, then walked again. A collection longer than 128
/// is one of the shapes it does not cover, so an object of 128 properties and an
/// object of 129 differ by one property and by which arm they take. The gap
/// between these two is the arm's price per call -- 8.5 us against 2.7 -- and how
/// often a real project pays it is recorded in
/// `docs/adr/0005-the-memo-key-is-a-whole-subtree-hash.md`.
fn key_fallback_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("StructuralKeyFallback");

  for props in [128usize, 129] {
    let source = {
      let entries = (0..props)
        .map(|index| format!("key{index}: {index}"))
        .collect::<Vec<_>>()
        .join(", ");

      format!("({{ {entries} }});")
    };

    let object = tower_expr(&parse(&source));

    // The gap between the two legs is the arm's price, and only while they take
    // different arms. The boundary is a private constant this bench cannot read,
    // so raising it past 129 -- a plausible tuning change, since the arm is
    // taken on a 130-colour palette -- would leave both legs on the fast arm and
    // report the collapse as a flat pair, which reads exactly like a win.
    //
    // The fallback is `stable_hash_wide(&drop_span(clone))`, so a leg whose key
    // equals that value took it. That is the same identity `stylex_utils`' own
    // tests pin the two arms with.
    let took_fallback =
      stable_hash_unspanned(&object) == stable_hash_wide(&drop_span(object.clone()));

    assert_eq!(
      took_fallback,
      props > 128,
      "an object of {props} properties {} the fallback arm, so this group is no \
       longer pricing the boundary between the two",
      if took_fallback {
        "took"
      } else {
        "did not take"
      }
    );

    group.bench_function(format!("object/{props}"), |b| {
      b.iter(|| black_box(stable_hash_unspanned(black_box(&object))))
    });
  }

  group.finish();
}

criterion_group!(
  evaluate_depth_benches,
  fold_benchmarks,
  key_benchmarks,
  key_fallback_benchmarks
);
criterion_main!(evaluate_depth_benches);
