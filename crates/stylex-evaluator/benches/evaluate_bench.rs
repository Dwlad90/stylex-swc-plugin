use std::{
  fs,
  hint::black_box,
  path::{Path, PathBuf},
  sync::Arc,
};

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use stylex_evaluator::evaluate::evaluate;
use stylex_state::{
  common::{fill_state_declarations, fill_top_level_expressions},
  functions::FunctionMap,
  state_manager::StateManager,
};
use stylex_structures::stylex_options::StyleXOptions;
use swc_core::{
  common::{FileName, GLOBALS, Globals, SourceMap, input::StringInput},
  ecma::{
    ast::{
      CallExpr, Callee, Decl, ExportDecl, Expr, ImportDecl, MemberProp, Module, ModuleDecl,
      ModuleItem, VarDeclarator,
    },
    parser::{EsSyntax, Parser, Syntax, lexer::Lexer},
    visit::{Visit, VisitWith},
  },
};

struct EvaluateFixture {
  name: String,
  module: Module,
  expressions: Vec<Expr>,
}

#[derive(Default)]
struct StyleXCallArgCollector {
  expressions: Vec<Expr>,
}

impl Visit for StyleXCallArgCollector {
  fn visit_call_expr(&mut self, call_expr: &CallExpr) {
    if is_stylex_callee(&call_expr.callee) {
      self
        .expressions
        .extend(call_expr.args.iter().map(|arg| arg.expr.as_ref().clone()));
    }

    call_expr.visit_children_with(self);
  }
}

fn is_stylex_callee(callee: &Callee) -> bool {
  let Callee::Expr(expr) = callee else {
    return false;
  };

  let Expr::Member(member) = expr.as_ref() else {
    return false;
  };

  let Expr::Ident(obj) = member.obj.as_ref() else {
    return false;
  };

  if obj.sym.as_ref() != "stylex" {
    return false;
  }

  match &member.prop {
    MemberProp::Ident(prop) => matches!(
      prop.sym.as_ref(),
      "create" | "createTheme" | "defineVars" | "defineConsts" | "keyframes"
    ),
    _ => false,
  }
}

fn perf_fixtures_dir() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../stylex-rs-compiler/benchmark/perf_fixtures")
}

/// The transform's fixture tree, read across the crate boundary.
///
/// A sibling path rather than a copy: the two cases below are transform
/// fixtures that the transform's own tests already pin, and a second copy here
/// would drift from them silently. Nothing is compiled across the boundary --
/// the benchmark only reads the files.
fn transform_fixtures_dir() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../stylex-transform/tests/fixture")
}

fn perf_fixture_paths() -> Vec<PathBuf> {
  let perf = [
    "colors.stylex.js",
    "sizes.stylex.js",
    "create-basic.js",
    "create-complex.js",
    "createTheme-basic.js",
    "createTheme-complex.js",
  ]
  .into_iter()
  .map(|file| perf_fixtures_dir().join(file));

  // Two transform fixtures where a dynamic parameter shadows an imported
  // binding. They are here rather than beside the others because the chain that
  // decides which of the two a name means runs per reference, inside `evaluate`,
  // and nothing else in this benchmark exercises it: the perf fixtures above
  // resolve every name to exactly one binding. The `-edges` case is the
  // expensive half -- shadowed names read through arithmetic, a template
  // literal, `calc()`, eight levels of nested conditions, and a shorthand that
  // expands into longhands.
  let shadowing = [
    "dynamic-param-shadows-import",
    "dynamic-param-shadows-import-edges",
  ]
  .into_iter()
  .map(|case| transform_fixtures_dir().join(case).join("input.stylex.js"));

  perf.chain(shadowing).collect()
}

fn parse_module(path: &PathBuf) -> Module {
  let source = match fs::read_to_string(path) {
    Ok(source) => source,
    Err(error) => panic!("Failed to read {}: {}", path.display(), error),
  };

  let cm: Arc<SourceMap> = Default::default();
  let file_name = Arc::new(FileName::Real(path.clone()));
  let fm = cm.new_source_file(file_name, source);
  let lexer = Lexer::new(
    Syntax::Es(EsSyntax {
      jsx: true,
      ..Default::default()
    }),
    Default::default(),
    StringInput::from(&*fm),
    None,
  );
  let mut parser = Parser::new_from(lexer);

  match parser.parse_module() {
    Ok(module) => module,
    Err(error) => panic!("Failed to parse {}: {:#?}", path.display(), error),
  }
}

fn collect_expressions(module: &Module) -> Vec<Expr> {
  let mut collector = StyleXCallArgCollector::default();
  module.visit_with(&mut collector);
  collector.expressions
}

fn fill_top_level_var_declarations(module: &Module, state: &mut StateManager) {
  for item in &module.body {
    match item {
      ModuleItem::Stmt(stmt) => {
        if let Some(decl) = stmt.as_decl().and_then(|decl| decl.as_var()) {
          fill_var_declarations(&decl.decls, state);
        }
      },
      ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
        decl: Decl::Var(var_decl),
        ..
      })) => fill_var_declarations(&var_decl.decls, state),
      ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl { .. })) => {},
      _ => {},
    }
  }
}

fn fill_var_declarations(declarations: &[VarDeclarator], state: &mut StateManager) {
  for declaration in declarations {
    fill_state_declarations(state, declaration);
  }
}

/// Names a fixture after its file, except where the file name carries nothing --
/// a transform fixture is always `input.stylex.js`, so two of them would collide
/// into one benchmark. Those are named after the directory that identifies them.
fn fixture_name(path: &Path) -> String {
  let file_name = path.file_name().and_then(|name| name.to_str());

  match file_name {
    Some("input.stylex.js") => match path.parent().and_then(|dir| dir.file_name()) {
      Some(dir) => dir.to_string_lossy().into_owned(),
      None => path.display().to_string(),
    },
    Some(name) => name.to_string(),
    None => path.display().to_string(),
  }
}

fn load_fixtures() -> Vec<EvaluateFixture> {
  perf_fixture_paths()
    .into_iter()
    .map(|path| {
      let module = parse_module(&path);
      let expressions = collect_expressions(&module);
      let name = fixture_name(&path);

      EvaluateFixture {
        name,
        module,
        expressions,
      }
    })
    .filter(|fixture| !fixture.expressions.is_empty())
    .collect()
}

/// The state one iteration folds against.
fn fixture_state(fixture: &EvaluateFixture) -> StateManager {
  let mut state = StateManager::new(StyleXOptions::default());

  fill_top_level_expressions(&fixture.module, &mut state);
  fill_top_level_var_declarations(&fixture.module, &mut state);

  state
}

/// How many of each fixture's expressions fold confidently, as measured.
///
/// Pinned rather than merely required to be non-zero, because three of these
/// legs fold *nothing* and there is no way for this harness to change that: the
/// fixtures import a theme, resolving one needs a real filename, and
/// `set_plugin_pass` is `pub(crate)` so a bench cannot supply one. Those legs
/// price the refusal path rather than a fold, which is worth knowing when
/// reading their numbers -- see `guidelines/PERFORMANCE.md`.
///
/// What the pin does buy is the guarantee the guard exists for. A leg that stops
/// folding what it used to, because a refusal moved earlier, is now a failed
/// bench rather than a faster one.
const EXPECTED_CONFIDENT_FOLDS: &[(&str, usize)] = &[
  ("colors.stylex.js", 1),
  ("sizes.stylex.js", 1),
  ("create-basic.js", 1),
  ("create-complex.js", 0),
  ("createTheme-basic.js", 1),
  ("createTheme-complex.js", 3),
  ("dynamic-param-shadows-import", 0),
  ("dynamic-param-shadows-import-edges", 0),
];

/// Panics unless the fixture folds exactly what it is recorded as folding.
///
/// A refusal is fast, and a leg that got quick because the work stopped
/// happening is indistinguishable from a win. Every other bench in this
/// directory carries a guard like this one; this file had none, which is how two
/// fixtures that fold nothing came to be added and their numbers published.
fn assert_folds_as_expected(fixture: &EvaluateFixture, functions: &FunctionMap) {
  let mut state = fixture_state(fixture);

  let confident = fixture
    .expressions
    .iter()
    .filter(|expression| evaluate(expression, &mut state, functions).confident)
    .count();

  let expected = EXPECTED_CONFIDENT_FOLDS
    .iter()
    .find(|(name, _)| *name == fixture.name)
    .map(|(_, expected)| *expected);

  match expected {
    Some(expected) => assert_eq!(
      confident,
      expected,
      "the `{}` leg folds {confident} of its {} expressions where it folded \
       {expected}; a leg that stopped folding is timing a refusal, and one that \
       started is no longer comparable with the numbers already published",
      fixture.name,
      fixture.expressions.len()
    ),
    None => panic!(
      "the `{}` leg is not recorded in `EXPECTED_CONFIDENT_FOLDS`, so nothing \
       says what it measures",
      fixture.name
    ),
  }
}

/// Runs inside `GLOBALS.set` because `evaluate` can reach the code-frame path,
/// which calls `Mark::new()`. Why that is not optional, and why a bench without
/// it still reports a number, is in `guidelines/PERFORMANCE.md` under "Writing
/// a bench".
fn evaluate_benchmarks(c: &mut Criterion) {
  let globals = Globals::default();

  GLOBALS.set(&globals, || {
    let mut group = c.benchmark_group("EvaluatePerfFixtures");
    let fixtures = load_fixtures();
    let functions = FunctionMap::default();

    for fixture in fixtures {
      assert_folds_as_expected(&fixture, &functions);

      // Batched, because the state cannot be reused: `seen` memoizes what it
      // folded, so a second iteration against one state would time the memo.
      // Building it is setup rather than work under measurement -- it allocates
      // the options, walks the module twice, and reads the evaluation-depth
      // environment behind a `OnceLock`, none of which is `evaluate`. Timing it
      // was the defect `module_path_bench` was already fixed for, and it dilutes
      // every ratio this group reports.
      group.bench_function(fixture.name.clone(), |b| {
        b.iter_batched(
          || fixture_state(&fixture),
          |mut state| {
            for expression in &fixture.expressions {
              black_box(evaluate(
                black_box(expression),
                black_box(&mut state),
                black_box(&functions),
              ));
            }
          },
          BatchSize::SmallInput,
        )
      });
    }

    group.finish();
  });
}

criterion_group!(evaluate_benches, evaluate_benchmarks);
criterion_main!(evaluate_benches);
