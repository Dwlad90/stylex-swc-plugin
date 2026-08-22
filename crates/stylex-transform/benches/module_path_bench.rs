//! What every module pays, whether or not it imports StyleX.
//!
//! A module with no `@stylexjs/stylex` import still runs the whole module-level
//! path: the memoized-source clone, the `Discover` walk, and the state the walk
//! writes into. Only after that does `visit_mut_module_impl` see there are no
//! import paths and return. So a regression that shows up on a file with no
//! StyleX in it at all is somewhere in those three, and this file times them
//! apart from one another.
//!
//! The corpus harness in `crates/stylex-rs-compiler/benchmark` cannot answer
//! this. It moves a single fixture's delta by several points between runs of
//! the same comparison, which is fine for a group mean and useless for
//! attribution: the thing being chased here is a couple of percent. A criterion
//! bench around one function does not have that problem, so the shape of this
//! file is one group per candidate rather than one number per fixture.
//!
//! **The legs are paired, and the pairing is the measurement.** An absolute
//! nanosecond count from this file says nothing on its own -- it is compared
//! against the same leg built from another revision, and two legs that differ
//! only in the candidate under test are what makes the difference attributable.
//! Where a candidate can be switched off by an option, both settings are timed
//! here so the difference is readable within one run.
//!
//! **Everything runs inside `GLOBALS.set`.** `into_pass` calls `Mark::new()`,
//! which panics outside a `GLOBALS` scope; the transform's diagnostic boundary
//! swallows that panic, so a bench without the scope still prints a number --
//! the cost of a panic and its unwind, not of the work. See
//! `guidelines/PERFORMANCE.md`, and the header of `transform_debug_bench.rs`
//! for the 3.6x misattribution that mistake once produced.
//!
//! ## What it measured when it was written
//!
//! On an Apple-silicon laptop, this same file applied to both trees and run on
//! each -- the branch, and merge-base `c83ac5cbd` -- with the bench file copied
//! into the older tree rather than cherry-picked, so both revisions compiled
//! the same bench:
//!
//! ```text
//!   leg                                     c83ac5cbd      branch    delta
//!   ModuleWalk/calls/1x                      142.24 µs   113.72 µs   -20.1%
//!   ModuleWalk/calls/4x                      561.08 µs   453.80 µs   -19.1%
//!   ModuleWalk/no-calls/1x                    20.72 µs    20.68 µs    -0.2%
//!   ModuleWalk/no-calls/4x                    78.45 µs    78.94 µs    +0.6%
//!   ModuleWalk/imported/1x                   209.06 µs   180.73 µs   -13.6%
//!   ModuleWalk/imported/4x                   789.67 µs   676.93 µs   -14.3%
//!   SeenModuleSource clone, no-calls/1x       39.93 µs    38.57 µs    -3.4%
//!   SeenModuleSource clone, no-calls/4x      154.92 µs   153.68 µs    -0.8%
//!   SeenModuleSource clone, calls/4x         208.33 µs   204.22 µs    -2.0%
//!   StructuralKey/call/shallow                92.77 ns    55.70 ns   -40.0%
//!   StructuralKey/call/member                 99.90 ns    61.79 ns   -38.1%
//!   StructuralKey/call/nested                277.68 ns   121.89 ns   -56.1%
//!   StateManager/new                         121.69 ns   151.18 ns   +24.2%
//!   FullPipeline/no-calls/1x                 154.16 µs   140.94 µs    -8.6%
//!   FullPipeline/calls/1x                    295.87 µs   258.24 µs   -12.7%
//!   FullPipeline/imported/1x                 367.94 µs   329.73 µs   -10.4%
//! ```
//!
//! The `SeenModuleSource` rows are the *difference* between that group's two
//! legs, which is what the clone costs; the legs themselves are the same
//! transform as `ModuleWalk`.
//!
//! Read together: every candidate is at parity or faster on the branch, and the
//! two that are faster are faster by a lot. `StateManager::new` is the only one
//! that costs more, by 29 ns once per transform -- four hundredths of one
//! percent of the smallest fixture in the corpus, and nothing at all of the
//! largest.
//!
//! So none of them carries the production-shape cost the spec is about, and
//! neither does the pass chain around them. That cost is measurable only through
//! the built `.node`: a module holding no StyleX import at all is 1.5-2.0%
//! slower there while being 0.2% faster here. What is left between the two is
//! the napi boundary and the whole-binary code layout a fat-LTO build produces,
//! and neither is something a bench in this crate can hold still. The
//! measurements and the elimination are in
//! `.scratch/production-path-fixed-cost/`.

use std::{hint::black_box, rc::Rc, sync::Arc};

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use stylex_structures::stylex_options::{StyleXOptions, StyleXOptionsParams};
use stylex_transform::{StyleXTransform, shared::structures::state_manager::StateManager};
use stylex_utils::hash::stable_hash_unspanned_call;
use swc_compiler_base::{PrintArgs, SourceMapsConfig, print};
use swc_core::{
  common::{
    FileName, GLOBALS, Globals, Mark, SourceMap, comments::SingleThreadedComments,
    input::StringInput, sync::Lrc,
  },
  ecma::{
    ast::{CallExpr, EsVersion, Expr, ModuleItem, Program, Stmt},
    parser::{Parser, Syntax, TsSyntax, lexer::Lexer},
    transforms::{
      base::{fixer::fixer, hygiene::hygiene, resolver},
      typescript::{Config as TypescriptConfig, typescript},
    },
    visit::{Visit, VisitWith, visit_mut_pass},
  },
};

/// The two module sizes. Four times the node count, because the question the
/// spec left open is whether the cost is proportional to the module or fixed
/// per transform: a fixed cost is a quarter of the per-node delta at `4x`, a
/// proportional one is the same.
const SIZE_MULTIPLIERS: [usize; 2] = [1, 4];

/// Statements per multiplier unit. Large enough that the walk dominates the
/// per-transform constants, small enough that the `4x` leg still parses and
/// runs in criterion's default measurement window.
const STATEMENTS_PER_UNIT: usize = 200;

/// A module of `count` statements, every one of which contains a call
/// expression, and none of which mention StyleX.
///
/// Call expressions are what `add_call_expression` fires on: during `Discover`
/// every call in the module is structurally hashed and its callee cloned into a
/// map, whether or not the module imports StyleX. This is the leg that carries
/// that cost.
///
/// The callees vary in shape -- bare identifier, member chain, computed member
/// -- because the structural hasher walks the callee and a single shape would
/// measure one arm of it.
fn calls_source(count: usize) -> String {
  let mut source = String::from("export const results = [];\n");

  for index in 0..count {
    match index % 3 {
      0 => source.push_str(&format!("results.push(compute({index}, 'a{index}'));\n")),
      1 => source.push_str(&format!("results.push(helpers.format({index}).trim());\n")),
      _ => source.push_str(&format!(
        "results.push(registry['handler{index}']({index}));\n"
      )),
    }
  }

  source
}

/// A module of `count` statements with the same rough node count as
/// [`calls_source`] and **no call expressions at all**.
///
/// The control. Whatever both legs share is the walk itself; what only the
/// `calls` leg pays is `add_call_expression`. Without this leg a slower `calls`
/// leg is equally consistent with a slower visitor, and the spec's first
/// candidate would stay unresolved.
fn no_calls_source(count: usize) -> String {
  let mut source = String::from("export const results = [];\n");

  for index in 0..count {
    match index % 3 {
      0 => source.push_str(&format!("results[{index}] = {index} + 'a{index}';\n")),
      1 => source.push_str(&format!(
        "results[{index}] = helpers.format[{index}].trimmed;\n"
      )),
      _ => source.push_str(&format!(
        "results[{index}] = registry['handler{index}'] ?? {index};\n"
      )),
    }
  }

  source
}

/// A module that does import StyleX but produces and consumes nothing, so the
/// early return in `visit_mut_module_impl` does not fire and the phases after
/// `Discover` run over the same body.
///
/// Present because the spec's finding is that a module with *no* StyleX import
/// regresses, and the natural objection is that such a module is not what
/// anybody compiles. This leg answers it: the same body, past the early return.
fn imported_source(count: usize) -> String {
  format!(
    "import * as stylex from '@stylexjs/stylex';\n\
     export const styles = stylex.create({{ root: {{ color: 'red' }} }});\n{}",
    calls_source(count)
  )
}

fn parse(file_name: &FileName, source: &str) -> Program {
  let cm: Lrc<SourceMap> = Default::default();
  let fm = cm.new_source_file(Arc::new(file_name.clone()), source.to_owned());
  let lexer = Lexer::new(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    EsVersion::EsNext,
    StringInput::from(&*fm),
    None,
  );
  let mut parser = Parser::new_from(lexer);

  match parser.parse_program() {
    Ok(program) => program,
    Err(error) => panic!("Failed to parse {file_name}: {error:#?}"),
  }
}

/// The transform under measurement, configured the way a production build
/// configures it.
///
/// `dev` off and `debug` off, because the development path is not what is being
/// chased -- it is the side the branch made faster. `use_real_file_for_source`
/// is the one knob that varies: it gates the memoized-source clone, which is
/// candidate two.
fn transform(program: Program, real_file_for_source: bool) -> Program {
  let comments = Rc::new(SingleThreadedComments::default());

  let mut options = StyleXOptionsParams {
    use_real_file_for_source: Some(real_file_for_source),
    ..Default::default()
  };

  let pass = StyleXTransform::test(comments)
    .with_filename(FileName::Anon)
    .with_options(&mut options)
    .with_dev(false)
    .with_debug(false)
    .into_pass();

  program.apply(pass)
}

/// Counts the call expressions in a module.
///
/// Standing behind the two walk legs: `no_calls_source` is only a control if it
/// really holds no calls, and `calls_source` is only measuring the per-call cost
/// if it holds as many as it looks like it does. A reflow of either generator
/// that quietly introduced or removed calls would otherwise turn the comparison
/// into noise that still prints.
#[derive(Default)]
struct CallCounter {
  calls: usize,
}

impl Visit for CallCounter {
  fn visit_call_expr(&mut self, call: &CallExpr) {
    self.calls += 1;
    call.visit_children_with(self);
  }
}

fn count_calls(program: &Program) -> usize {
  let mut counter = CallCounter::default();
  program.visit_with(&mut counter);
  counter.calls
}

/// One parsed module, with the call count that makes it the leg it claims to be.
struct Fixture {
  label: String,
  program: Program,
  calls: usize,
}

fn fixture(label: impl Into<String>, source: &str) -> Fixture {
  let label = label.into();
  let program = parse(&FileName::Anon, source);
  let calls = count_calls(&program);

  Fixture {
    label,
    program,
    calls,
  }
}

/// The three walk legs at every size, checked to be what they claim.
fn walk_fixtures() -> Vec<Fixture> {
  let mut fixtures = Vec::new();

  for multiplier in SIZE_MULTIPLIERS {
    let statements = STATEMENTS_PER_UNIT * multiplier;

    let calls = fixture(format!("calls/{multiplier}x"), &calls_source(statements));

    assert!(
      calls.calls >= statements,
      "the `{}` leg holds {} call expressions for {statements} statements -- \
       it is meant to hold at least one per statement, so it is no longer \
       measuring the per-call path",
      calls.label,
      calls.calls
    );

    let no_calls = fixture(
      format!("no-calls/{multiplier}x"),
      &no_calls_source(statements),
    );

    assert_eq!(
      no_calls.calls, 0,
      "the `{}` control leg holds {} call expressions -- it is the leg that is \
       meant to hold none, so the difference between the two legs is no longer \
       the per-call cost",
      no_calls.label, no_calls.calls
    );

    fixtures.push(calls);
    fixtures.push(no_calls);
    fixtures.push(fixture(
      format!("imported/{multiplier}x"),
      &imported_source(statements),
    ));
  }

  fixtures
}

/// Candidate one: the `Discover` walk and the per-node state it writes.
///
/// Read `calls` against `no-calls` at one size for what `add_call_expression`
/// costs, and either of them at `1x` against `4x` for whether the cost is
/// proportional. Read the whole group against the same group built from
/// `c83ac5cbd` for whether this branch made it worse.
fn module_walk_benchmarks(c: &mut Criterion) {
  let globals = Globals::default();

  GLOBALS.set(&globals, || {
    let fixtures = walk_fixtures();

    let mut group = c.benchmark_group("ModuleWalk");

    for fixture in &fixtures {
      // Batched because `apply` consumes the program: the clone is setup, not
      // work under measurement.
      group.bench_function(&fixture.label, |b| {
        b.iter_batched(
          || fixture.program.clone(),
          |program| black_box(transform(program, black_box(true))),
          BatchSize::SmallInput,
        )
      });
    }

    group.finish();
  });
}

/// Candidate two: `set_seen_module_source_code`, the deep clone of the module
/// into per-transform state.
///
/// `use_real_file_for_source` is what gates it in a release build, so the two
/// legs here are the same module transformed with the clone off and on. The
/// difference is the clone plus the `Rc` and `OnceCell` this branch wrapped it
/// in; the clone itself is on both revisions, so only a difference in the
/// *difference* is attributable to the branch.
fn seen_module_source_benchmarks(c: &mut Criterion) {
  let globals = Globals::default();

  GLOBALS.set(&globals, || {
    let fixtures = walk_fixtures();

    let mut group = c.benchmark_group("SeenModuleSource");

    for fixture in &fixtures {
      for (label, real_file) in [("without-clone", true), ("with-clone", false)] {
        group.bench_function(format!("{label}/{}", fixture.label), |b| {
          b.iter_batched(
            || fixture.program.clone(),
            |program| black_box(transform(program, black_box(real_file))),
            BatchSize::SmallInput,
          )
        });
      }
    }

    group.finish();
  });
}

/// The call expressions the structural key is timed over.
///
/// Three shapes, because the hasher walks the callee and the arguments and a
/// single shape would time one arm of it. The nesting in the third is what a
/// real module's calls look like once arguments are themselves calls.
const KEY_SUBJECTS: [(&str, &str); 3] = [
  ("shallow", "compute(1, 'a')"),
  ("member", "helpers.format(value).trim()"),
  (
    "nested",
    "outer(inner(a, b), registry['handler'](c, d), { key: value })",
  ),
];

/// Parses `source` as a single expression statement and returns the call it
/// holds.
fn parse_call(source: &str) -> CallExpr {
  let program = parse(&FileName::Anon, &format!("{source};"));

  // `parse_program` answers `Script` for a body with no import or export and
  // `Module` for one with either, and the subjects here are written without
  // both -- matched rather than assumed, so a subject that later grows an
  // import fails with a sentence instead of skipping the arm.
  let statement = match &program {
    Program::Script(script) => script.body.first(),
    Program::Module(module) => module.body.first().and_then(ModuleItem::as_stmt),
  };

  let call = statement
    .and_then(Stmt::as_expr)
    .and_then(|stmt| match stmt.expr.as_ref() {
      Expr::Call(call) => Some(call.clone()),
      _ => None,
    });

  match call {
    Some(call) => call,
    None => panic!("`{source}` did not parse to a single call expression"),
  }
}

/// Candidate one, at per-node resolution: the structural key every call
/// expression in every module is hashed into.
///
/// `add_call_expression` runs this once per call during `Discover`, so a few
/// nanoseconds here is a cost proportional to how many calls a module holds --
/// which is the shape the spec reports. Timed on its own because the walk leg
/// above cannot separate the hash from the map insert and the callee clone
/// beside it.
fn structural_key_benchmarks(c: &mut Criterion) {
  let globals = Globals::default();

  GLOBALS.set(&globals, || {
    let subjects: Vec<(&str, CallExpr)> = KEY_SUBJECTS
      .into_iter()
      .map(|(label, source)| (label, parse_call(source)))
      .collect();

    let mut group = c.benchmark_group("StructuralKey");

    for (label, call) in &subjects {
      group.bench_function(format!("call/{label}"), |b| {
        b.iter(|| black_box(stable_hash_unspanned_call(black_box(call))))
      });
    }

    group.finish();
  });
}

/// Candidate three: `StateManager` construction and drop.
///
/// Fixed per transform rather than proportional to the module, so it cannot
/// explain a cost that holds its percentage from a 72 µs module to a 1.4 ms one.
/// Measured anyway because it is one more line in the same file, and because
/// "not this either" is worth having written down.
fn state_manager_benchmarks(c: &mut Criterion) {
  let globals = Globals::default();

  GLOBALS.set(&globals, || {
    let mut group = c.benchmark_group("StateManager");

    group.bench_function("new", |b| {
      b.iter(|| black_box(StateManager::new(black_box(StyleXOptions::default()))))
    });

    group.finish();
  });
}

/// The whole pass chain a compile runs, not just the StyleX pass.
///
/// The three candidates above are all inside `StyleXTransform`. If none of them
/// carries a cost that a module with no StyleX import still pays, the next place
/// to look is the passes on either side of it -- the resolver, type stripping,
/// hygiene, the fixer, and the printer -- because those run over the same module
/// and none of them is reached by a bench that calls `apply` on the StyleX pass
/// alone.
///
/// The chain is the one `stylex_rs_compiler::transform` builds, in its order, so
/// a difference here and a difference through the built `.node` are the same
/// difference. What it deliberately leaves out is the napi boundary and the
/// metadata extraction, which need a JS environment: a bench that needed one
/// could not be run from `cargo bench` on both revisions, which is the whole
/// point of this file.
///
/// `verbatim_module_syntax` is off, matching what the compiler passes for a
/// TypeScript input. That is the setting under which the two revisions build
/// *identical* configuration -- `strip` is `typescript(Config::default())` -- so
/// any difference this leg reads is not the type-stripping configuration.
fn full_pipeline(
  program: Program,
  source_map: &Lrc<SourceMap>,
  comments: &SingleThreadedComments,
) -> String {
  let unresolved_mark = Mark::new();
  let top_level_mark = Mark::new();

  let mut stylex = StyleXTransform::test(comments)
    .with_filename(FileName::Anon)
    .with_dev(false)
    .with_debug(false)
    .build();

  let program = program
    .apply(resolver(unresolved_mark, top_level_mark, true))
    .apply(typescript(
      TypescriptConfig::default(),
      unresolved_mark,
      top_level_mark,
    ))
    .apply(&mut visit_mut_pass(&mut stylex))
    .apply(hygiene())
    .apply(&mut fixer(None));

  print(
    source_map.clone(),
    &program,
    PrintArgs {
      source_map: SourceMapsConfig::Bool(false),
      comments: Some(comments),
      ..Default::default()
    },
  )
  .map(|output| output.code)
  .unwrap_or_else(|error| panic!("Failed to print the transformed module: {error:?}"))
}

/// Candidate four, and the one the other three point at when they come up empty:
/// everything a compile does around the StyleX pass.
///
/// Read against the same group from another revision. A difference here with no
/// difference in `ModuleWalk` puts the cost outside the StyleX transform
/// entirely, which is a different investigation from the one the spec opened.
fn full_pipeline_benchmarks(c: &mut Criterion) {
  let globals = Globals::default();

  GLOBALS.set(&globals, || {
    let fixtures = walk_fixtures();

    let mut group = c.benchmark_group("FullPipeline");

    for fixture in &fixtures {
      group.bench_function(&fixture.label, |b| {
        b.iter_batched(
          || {
            (
              fixture.program.clone(),
              Lrc::<SourceMap>::default(),
              SingleThreadedComments::default(),
            )
          },
          |(program, source_map, comments)| {
            black_box(full_pipeline(program, &source_map, &comments))
          },
          BatchSize::SmallInput,
        )
      });
    }

    group.finish();
  });
}

criterion_group!(
  module_path_benches,
  module_walk_benchmarks,
  seen_module_source_benchmarks,
  structural_key_benchmarks,
  state_manager_benchmarks,
  full_pipeline_benchmarks
);
criterion_main!(module_path_benches);
