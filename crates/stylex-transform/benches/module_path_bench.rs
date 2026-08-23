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
//! **Every group asserts its subject did the work it exists to time.** A
//! transform that stopped compiling, an option gate that stopped gating and a
//! structural key that stopped distinguishing its subjects are all *fast*, and a
//! leg that got quick because the work stopped happening is indistinguishable
//! from a win. [`Fixture::assert_is_what_it_claims`] and the per-group checks
//! beside it are what stand between this file and that reading.
//!
//! ## What it measured when it was written
//!
//! On an Apple-silicon laptop, this same file applied to both trees and run on
//! each -- the branch, and merge-base `c83ac5cbd` -- with the bench file copied
//! into the older tree rather than cherry-picked, so both revisions compiled
//! the same bench. Full runs, not `--quick`:
//!
//! ```text
//!   leg                                     c83ac5cbd      branch    delta
//!   ModuleWalk/calls/1x                      138.63 µs   112.00 µs   -19.2%
//!   ModuleWalk/calls/4x                      549.53 µs   445.18 µs   -19.0%
//!   ModuleWalk/no-calls/1x                    20.68 µs    20.97 µs    +1.4%
//!   ModuleWalk/no-calls/4x                    78.70 µs    79.56 µs    +1.1%
//!   ModuleWalk/imported/1x                   208.39 µs   181.58 µs   -12.9%
//!   ModuleWalk/imported/4x                   776.75 µs   672.27 µs   -13.5%
//!   SeenModuleSource/kept/calls/1x           189.63 µs   162.72 µs   -14.2%
//!   SeenModuleSource/kept/calls/4x           755.04 µs   652.82 µs   -13.5%
//!   SeenModuleSource/kept/no-calls/1x         59.90 µs    58.65 µs    -2.1%
//!   SeenModuleSource/kept/no-calls/4x        234.78 µs   235.03 µs    +0.1%
//!   StructuralKey/call/shallow               100.32 ns    57.96 ns   -42.2%
//!   StructuralKey/call/member                101.26 ns    63.75 ns   -37.0%
//!   StructuralKey/call/nested                266.17 ns   122.77 ns   -53.9%
//!   StateManager/new                          70.35 ns    79.48 ns   +13.0%
//!   FullPipeline/no-calls/1x                 141.34 µs   138.89 µs    -1.7%
//!   FullPipeline/no-calls/4x                 545.39 µs   543.81 µs    -0.3%
//!   FullPipeline/calls/1x                    281.12 µs   255.04 µs    -9.3%
//!   FullPipeline/calls/4x                     1.110 ms    1.006 ms    -9.3%
//!   FullPipeline/imported/1x                 349.26 µs   325.82 µs    -6.7%
//!   FullPipeline/imported/4x                  1.335 ms    1.235 ms    -7.5%
//! ```
//!
//! The `SeenModuleSource/kept` legs are the *same* transform as the matching
//! `ModuleWalk` leg with the clone left in, so what the clone costs is the
//! difference between the two rows: 39.2 µs against 37.7 µs on `no-calls/1x`,
//! 156.1 µs against 155.5 µs on `no-calls/4x`, 205.5 µs against 207.6 µs on
//! `calls/4x`. The same on both revisions to within a couple of points -- the
//! `Rc` and the `OnceCell` this branch wrapped the clone in cost nothing
//! measurable.
//!
//! **What the table can and cannot resolve.** Criterion's interval inside one
//! run of one binary is a couple of tenths of a percent, but two *builds* of the
//! same source do not agree that closely: an earlier pair of full runs put
//! `no-calls/1x` at -0.2% and `no-calls/4x` at +0.6% where this pair puts them
//! at +1.4% and +1.1%, and `FullPipeline/no-calls/1x` at -8.6% where this pair
//! puts it at -1.7%. So a leg reading within about a point and a half either way
//! is *unresolved by this file*, not measured at parity, and the rows worth
//! leaning on are the ones an order of magnitude clear of that -- the walk over a
//! call-heavy module, the structural key, and the pass chain over anything
//! holding calls.
//!
//! Read that way: the branch is much faster wherever call expressions are
//! involved, and indistinguishable on a module with none. Nothing here carries
//! the production-shape cost the investigation was about. `StateManager::new` is
//! the only row that costs more and it costs nine nanoseconds once per module.
//!
//! The cost that is real is measurable only through the built `.node`, where a
//! module holding no StyleX import at all is 1.5-2.0% slower across
//! twenty-eight process medians with a same-binary control of half a point --
//! a consistency none of the legs above show.

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
    ast::{CallExpr, EsVersion, Expr, KeyValueProp, ModuleItem, Program, PropName, Stmt},
    parser::{Parser, Syntax, TsSyntax, lexer::Lexer},
    transforms::{
      base::{fixer::fixer, hygiene::hygiene, resolver},
      typescript::{Config as TypescriptConfig, typescript},
    },
    visit::{Visit, VisitWith, visit_mut_pass},
  },
};

/// The marker a compiled style namespace carries.
///
/// Counting it is how the `imported` fixture proves it compiled something: a
/// module whose StyleX import stopped resolving still transforms, still returns
/// a program, and emits none of these.
const COMPILED_KEY: &str = "$$css";

/// The two module sizes. Four times the node count, because the question the
/// investigation left open is whether the cost is proportional to the module or
/// fixed per transform: a fixed cost is a quarter of the per-node delta at `4x`,
/// a proportional one is the same.
const SIZE_MULTIPLIERS: [usize; 2] = [1, 4];

/// Statements per multiplier unit. Large enough that the walk dominates the
/// per-transform constants, small enough that the `4x` leg still parses and
/// runs in criterion's default measurement window.
const STATEMENTS_PER_UNIT: usize = 200;

/// Whether the transform memoizes a deep clone of the module into per-transform
/// state.
///
/// A named pair rather than a `bool`, because the option behind it is
/// `use_real_file_for_source` and it reads backwards: *true* means read the
/// source off disk and therefore **skip** the clone. A bare `true` at a call
/// site would say the opposite of what it does.
#[derive(Clone, Copy)]
enum SourceClone {
  /// `use_real_file_for_source: false` -- the module is cloned into state.
  Kept,
  /// `use_real_file_for_source: true` -- in a release build, no clone.
  Skipped,
}

impl SourceClone {
  fn use_real_file_for_source(self) -> bool {
    match self {
      SourceClone::Kept => false,
      SourceClone::Skipped => true,
    }
  }

  fn label(self) -> &'static str {
    match self {
      SourceClone::Kept => "kept",
      SourceClone::Skipped => "skipped",
    }
  }
}

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
/// leg is equally consistent with a slower visitor, and candidate one would stay
/// unresolved.
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

/// The same body as [`calls_source`], preceded by a StyleX import and one
/// `create` call.
///
/// Not a second reading of candidate one: the early return in
/// `visit_mut_module_impl` does not fire for this module, so producers,
/// consumers and finalization all run over it and the leg times the whole
/// transform rather than the walk. It is here because the finding under
/// investigation is that a module with *no* StyleX import regresses, and the
/// natural objection is that such a module is not what anybody compiles. This
/// leg is the same body past the early return, so the two can be read against
/// each other.
fn imported_source(count: usize) -> String {
  format!(
    "import * as stylex from '@stylexjs/stylex';\n\
     export const styles = stylex.create({{ root: {{ color: 'red' }} }});\n{}",
    calls_source(count)
  )
}

/// One parsed module, with the source map and comment store the printer needs.
///
/// The map and the comments are kept beside the program rather than thrown away
/// because [`full_pipeline`] hands both to `print`, as the compiler does. A
/// fresh empty `SourceMap` would give the printer no source file to resolve
/// positions against and no comments to re-emit, which is a different printer
/// run from the one being modelled.
struct Fixture {
  label: String,
  program: Program,
  source_map: Lrc<SourceMap>,
  comments: Rc<SingleThreadedComments>,
  /// Call expressions in the parsed module.
  calls: usize,
  /// Whether the module imports StyleX, and so whether the transform is
  /// expected to compile anything out of it.
  compiles: bool,
}

impl Fixture {
  /// Panics unless the fixture is the leg it claims to be.
  ///
  /// What is checked is **whether the transform compiled anything**, because the
  /// `imported` leg exists to run past the early return. If its import stops
  /// resolving it silently becomes a third copy of the `calls` leg -- faster,
  /// and wrong. The two StyleX-free legs are checked the other way, so a leg
  /// that starts compiling styles cannot quietly stop being the no-import case.
  ///
  /// The call counts are checked where the fixtures are built, because that is
  /// where the two generators can be compared against each other.
  fn assert_is_what_it_claims(&self) {
    let compiled = count_compiled_namespaces(&transform(
      self.program.clone(),
      &self.comments,
      SourceClone::Skipped,
    ));

    if self.compiles {
      assert!(
        self.calls > 0,
        "the `{}` leg holds no call expressions",
        self.label
      );

      assert!(
        compiled > 0,
        "the `{}` leg compiled no style namespaces -- its StyleX import no longer \
         resolves, so it returns at the same place the no-import legs do and is \
         measuring the walk rather than a whole transform",
        self.label
      );
    } else {
      assert_eq!(
        compiled, 0,
        "the `{}` leg compiled {compiled} style namespaces -- it is meant to hold \
         no StyleX import at all, so it is no longer the case the investigation is \
         about",
        self.label
      );
    }
  }
}

/// Parses `source`, keeping the source map and the comments the lexer fills.
fn parse(file_name: &FileName, source: &str) -> (Program, Lrc<SourceMap>, SingleThreadedComments) {
  let source_map: Lrc<SourceMap> = Default::default();
  let file = source_map.new_source_file(Arc::new(file_name.clone()), source.to_owned());
  let comments = SingleThreadedComments::default();
  let lexer = Lexer::new(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    EsVersion::EsNext,
    StringInput::from(&*file),
    Some(&comments),
  );
  let mut parser = Parser::new_from(lexer);

  match parser.parse_program() {
    Ok(program) => (program, source_map, comments),
    Err(error) => panic!("Failed to parse {file_name}: {error:#?}"),
  }
}

/// The transform under measurement, configured the way a production build
/// configures it.
///
/// `dev` off and `debug` off, because the development path is not what is being
/// chased -- it is the side the branch made faster. The one knob that varies is
/// [`SourceClone`], which gates the memoized-source clone.
fn transform(
  program: Program,
  comments: &Rc<SingleThreadedComments>,
  clone: SourceClone,
) -> Program {
  let mut options = StyleXOptionsParams {
    use_real_file_for_source: Some(clone.use_real_file_for_source()),
    ..Default::default()
  };

  let pass = StyleXTransform::test(Rc::clone(comments))
    .with_filename(FileName::Anon)
    .with_options(&mut options)
    .with_dev(false)
    .with_debug(false)
    .into_pass();

  program.apply(pass)
}

/// The whole pass chain a compile runs, not just the StyleX pass.
///
/// The three in-crate candidates all sit inside `StyleXTransform`. If none of
/// them carries a cost that a module with no StyleX import still pays, the next
/// place to look is the passes on either side -- the resolver, type stripping,
/// hygiene, the fixer, and the printer -- because those run over the same module
/// and a bench that calls `apply` on the StyleX pass alone reaches none of them.
///
/// The chain is the one `stylex_rs_compiler::transform` builds, in its order,
/// with the fixture's own source map and comments handed to the printer as the
/// compiler hands it the lexer's. What it leaves out is the napi boundary and
/// the metadata extraction, both of which need a JS environment -- and a leg
/// that needed one could not be run from `cargo bench` on two revisions, which
/// is the whole point of this file.
///
/// `verbatim_module_syntax` is left at its default. That is not a re-test of the
/// type-stripping change -- it is the setting under which the two revisions build
/// *identical* configuration, since `strip` is `typescript(Config::default())`,
/// so a difference this leg reads cannot be that change.
fn full_pipeline(fixture: &Fixture, program: Program) -> String {
  let unresolved_mark = Mark::new();
  let top_level_mark = Mark::new();

  let mut stylex = StyleXTransform::test(Rc::clone(&fixture.comments))
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

  let output = print(
    Lrc::clone(&fixture.source_map),
    &program,
    PrintArgs {
      source_map: SourceMapsConfig::Bool(false),
      comments: Some(fixture.comments.as_ref()),
      ..Default::default()
    },
  );

  match output {
    Ok(output) => output.code,
    Err(error) => panic!(
      "Failed to print the `{}` leg's transformed module: {error:?}",
      fixture.label
    ),
  }
}

/// Counts the call expressions in a module.
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

/// Counts the compiled style namespaces in a transformed module.
#[derive(Default)]
struct CompiledCounter {
  compiled: usize,
}

impl Visit for CompiledCounter {
  fn visit_key_value_prop(&mut self, prop: &KeyValueProp) {
    if is_compiled_key(&prop.key) {
      self.compiled += 1;
    }

    prop.visit_children_with(self);
  }
}

/// Whether `key` is the `$$css` marker, quoted or not.
fn is_compiled_key(key: &PropName) -> bool {
  match key {
    PropName::Ident(ident) => ident.sym.as_ref() == COMPILED_KEY,
    PropName::Str(value) => value.value.as_str() == Some(COMPILED_KEY),
    _ => false,
  }
}

fn count_compiled_namespaces(program: &Program) -> usize {
  let mut counter = CompiledCounter::default();
  program.visit_with(&mut counter);
  counter.compiled
}

fn fixture(label: impl Into<String>, source: &str, compiles: bool) -> Fixture {
  let label = label.into();
  let (program, source_map, comments) = parse(&FileName::Anon, source);
  let calls = count_calls(&program);

  Fixture {
    label,
    program,
    source_map,
    comments: Rc::new(comments),
    calls,
    compiles,
  }
}

/// The three legs at every size, each checked to be what it claims.
fn build_fixtures() -> Vec<Fixture> {
  let mut fixtures = Vec::new();

  for multiplier in SIZE_MULTIPLIERS {
    let statements = STATEMENTS_PER_UNIT * multiplier;

    let calls = fixture(
      format!("calls/{multiplier}x"),
      &calls_source(statements),
      /* compiles */ false,
    );

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
      /* compiles */ false,
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
      /* compiles */ true,
    ));
  }

  for fixture in &fixtures {
    fixture.assert_is_what_it_claims();
  }

  fixtures
}

/// Candidate one: the `Discover` walk and the per-node state it writes.
///
/// Read `calls` against `no-calls` at one size for what `add_call_expression`
/// costs, and either of them at `1x` against `4x` for whether the cost is
/// proportional. `imported` is the same body past the early return, not a second
/// reading of the walk.
fn module_walk_group(c: &mut Criterion, fixtures: &[Fixture]) {
  let mut group = c.benchmark_group("ModuleWalk");

  for fixture in fixtures {
    // Batched because `apply` consumes the program: the clone is setup, not
    // work under measurement.
    group.bench_function(&fixture.label, |b| {
      b.iter_batched(
        || fixture.program.clone(),
        |program| {
          black_box(transform(
            program,
            &fixture.comments,
            black_box(SourceClone::Skipped),
          ))
        },
        BatchSize::SmallInput,
      )
    });
  }

  group.finish();
}

/// Candidate two: `set_seen_module_source_code`, the deep clone of the module
/// into per-transform state.
///
/// One leg per fixture, with the clone kept. What the clone costs is this row
/// minus the matching `ModuleWalk` row, which is the same transform with the
/// clone skipped -- the two are timed in one run of one binary, so subtracting
/// them is sound. The clone is on both revisions, so only a difference in that
/// *difference* is attributable to a branch.
///
/// The `imported` fixtures are left out: the clone is proportional to module
/// size and nothing about a StyleX import changes it, so a third pair of sizes
/// would cost runtime to restate what the other two say.
fn seen_module_source_group(c: &mut Criterion, fixtures: &[Fixture]) {
  // `visit_mut_module_impl` clones unconditionally under `debug_assertions`, so
  // in a debug build both settings clone and this group reads zero against
  // `ModuleWalk` -- a flat, fast, meaningless difference, which is exactly the
  // failure mode a bench must not report as a result. `cargo test` does not run
  // benches, so this fires only for someone benchmarking in the wrong profile.
  //
  // Bound to a local first because the condition is a compile-time constant and
  // `assert!` on one is a lint. Checked at run time anyway rather than as a
  // `const` assertion: a `const` one would refuse to compile the bench in a
  // debug tree, where every other group in this file is still worth running.
  let debug_assertions_on = cfg!(debug_assertions);

  if debug_assertions_on {
    // Skipped rather than asserted. A panic here unwinds out of
    // `module_path_benchmarks` and takes the three groups below with it, which
    // is the outcome the note above says a `const` assertion was avoided to
    // prevent -- the runtime one had exactly the same effect. The group's
    // absence from the report is the signal.
    eprintln!(
      "skipping `SeenModuleSource`: the memoized-source clone it exists to price is \
       forced on under `cfg!(debug_assertions)`, so both settings would clone and \
       the difference against `ModuleWalk` would read as zero"
    );

    return;
  }

  let mut group = c.benchmark_group("SeenModuleSource");

  for fixture in fixtures.iter().filter(|fixture| !fixture.compiles) {
    let clone = SourceClone::Kept;

    group.bench_function(format!("{}/{}", clone.label(), fixture.label), |b| {
      b.iter_batched(
        || fixture.program.clone(),
        |program| black_box(transform(program, &fixture.comments, black_box(clone))),
        BatchSize::SmallInput,
      )
    });
  }

  group.finish();
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
  let (program, _, _) = parse(&FileName::Anon, &format!("{source};"));

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
/// which is the shape the investigation reports. Timed on its own because the
/// walk leg above cannot separate the hash from the map insert and the callee
/// clone beside it.
fn structural_key_group(c: &mut Criterion) {
  let subjects: Vec<(&str, CallExpr)> = KEY_SUBJECTS
    .into_iter()
    .map(|(label, source)| (label, parse_call(source)))
    .collect();

  // A key that answered the same for every shape would be timing a degenerate
  // hash -- fast, and no longer the thing every call expression pays.
  // Width-inferred rather than named: the key is 128 bits wide here and was 64
  // on the revision this is compared against, and the check is about the values
  // being distinct rather than about how wide they are.
  let keys: Vec<_> = subjects
    .iter()
    .map(|(_, call)| stable_hash_unspanned_call(call))
    .collect();

  for (index, key) in keys.iter().enumerate() {
    assert!(
      !keys[index + 1..].contains(key),
      "two of the key subjects hash alike, so this group is no longer timing a \
       key that distinguishes the shapes it is given"
    );
  }

  let mut group = c.benchmark_group("StructuralKey");

  for (label, call) in &subjects {
    group.bench_function(format!("call/{label}"), |b| {
      b.iter(|| black_box(stable_hash_unspanned_call(black_box(call))))
    });
  }

  group.finish();
}

/// Candidate three: `StateManager` construction.
///
/// Fixed per transform rather than proportional to the module, so it cannot
/// explain a cost that holds its percentage from a 72 µs module to a 1.4 ms one.
/// Measured anyway because it is one more line in the same file, and because
/// "not this either" is worth having written down.
///
/// The options are built **once** and cloned in the batched setup, not built
/// inside the timed closure. `StyleXOptions::default()` allocates strings, an
/// index set and a shared map, and on some revisions reads the evaluation-depth
/// environment behind a `OnceLock` -- all of it work that is not
/// `StateManager::new`, and some of it work that differs between the revisions
/// being compared. Timing it here cost this file a published number that had to
/// be withdrawn.
fn state_manager_group(c: &mut Criterion) {
  let options = StyleXOptions::default();

  let mut group = c.benchmark_group("StateManager");

  group.bench_function("new", |b| {
    b.iter_batched(
      || options.clone(),
      |options| black_box(StateManager::new(black_box(options))),
      BatchSize::SmallInput,
    )
  });

  group.finish();
}

/// Candidate four, and the one the other three point at when they come up
/// empty: everything a compile does around the StyleX pass.
///
/// Read against the same group from another revision. A difference here with no
/// difference in `ModuleWalk` puts the cost outside the StyleX transform
/// entirely, which is a different investigation from the one that opened this
/// file.
fn full_pipeline_group(c: &mut Criterion, fixtures: &[Fixture]) {
  // The printer is the last pass and the one most easily reduced to nothing by a
  // mistake upstream; an empty answer here would be the fastest leg in the file.
  for fixture in fixtures {
    let code = full_pipeline(fixture, fixture.program.clone());

    assert!(
      !code.trim().is_empty(),
      "the `{}` leg printed nothing, so this group is timing a pass chain that \
       stopped producing a module",
      fixture.label
    );

    if fixture.compiles {
      assert!(
        code.contains(COMPILED_KEY),
        "the `{}` leg printed a module holding no compiled style namespace, so \
         the chain is no longer running the StyleX pass over a module it can \
         compile",
        fixture.label
      );
    }
  }

  let mut group = c.benchmark_group("FullPipeline");

  for fixture in fixtures {
    group.bench_function(&fixture.label, |b| {
      b.iter_batched(
        || fixture.program.clone(),
        |program| black_box(full_pipeline(black_box(fixture), program)),
        BatchSize::SmallInput,
      )
    });
  }

  group.finish();
}

/// Every group, in one `GLOBALS` scope over one set of fixtures.
///
/// One entry point rather than one per group because the fixtures are the
/// expensive part of setup -- six modules parsed, and each transformed to check
/// it is the leg it claims -- and because a `GLOBALS` scope opened five times is
/// five chances to forget it.
fn module_path_benchmarks(c: &mut Criterion) {
  let globals = Globals::default();

  GLOBALS.set(&globals, || {
    let fixtures = build_fixtures();

    module_walk_group(c, &fixtures);
    seen_module_source_group(c, &fixtures);
    structural_key_group(c);
    state_manager_group(c);
    full_pipeline_group(c, &fixtures);
  });
}

criterion_group!(module_path_benches, module_path_benchmarks);
criterion_main!(module_path_benches);
