use std::{fs, hint::black_box, path::PathBuf, sync::Arc};

use criterion::{Criterion, criterion_group, criterion_main};
use stylex_structures::stylex_options::StyleXOptions;
use stylex_transform::shared::{
  structures::{functions::FunctionMap, state_manager::StateManager},
  utils::{
    common::{fill_state_declarations, fill_top_level_expressions},
    js::evaluate::evaluate,
  },
};
use swc_core::{
  common::{FileName, SourceMap, input::StringInput},
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

fn perf_fixture_paths() -> Vec<PathBuf> {
  [
    "colors.stylex.js",
    "sizes.stylex.js",
    "create-basic.js",
    "create-complex.js",
    "createTheme-basic.js",
    "createTheme-complex.js",
  ]
  .into_iter()
  .map(|file| perf_fixtures_dir().join(file))
  .collect()
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

fn load_fixtures() -> Vec<EvaluateFixture> {
  perf_fixture_paths()
    .into_iter()
    .map(|path| {
      let module = parse_module(&path);
      let expressions = collect_expressions(&module);
      let name = match path.file_name().and_then(|file_name| file_name.to_str()) {
        Some(file_name) => file_name.to_string(),
        None => path.display().to_string(),
      };

      EvaluateFixture {
        name,
        module,
        expressions,
      }
    })
    .filter(|fixture| !fixture.expressions.is_empty())
    .collect()
}

fn evaluate_benchmarks(c: &mut Criterion) {
  let mut group = c.benchmark_group("EvaluatePerfFixtures");
  let fixtures = load_fixtures();
  let functions = FunctionMap::default();

  for fixture in fixtures {
    group.bench_function(fixture.name, |b| {
      b.iter(|| {
        let mut state = StateManager::new(StyleXOptions::default());
        fill_top_level_expressions(black_box(&fixture.module), black_box(&mut state));
        fill_top_level_var_declarations(black_box(&fixture.module), black_box(&mut state));

        for expression in &fixture.expressions {
          black_box(evaluate(
            black_box(expression),
            black_box(&mut state),
            black_box(&functions),
          ));
        }
      })
    });
  }

  group.finish();
}

criterion_group!(evaluate_benches, evaluate_benchmarks);
criterion_main!(evaluate_benches);
