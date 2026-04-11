use clap::Parser;
use fancy_regex::Regex;
use swc_compiler_base::{PrintArgs, SourceMapsConfig, parse_js, print};
use swc_config::is_module::IsModule;

use std::{
  fmt::Display,
  fs::{self, read_to_string, write},
  io,
  path::{Path, PathBuf},
  sync::Arc,
};
use swc_ecma_parser::Syntax;

use swc_core::{
  common::{
    DUMMY_SP, FileName, SourceMap, SyntaxContext,
    errors::{ColorConfig, Handler},
  },
  ecma::{
    ast::{
      CallExpr, Decl, EsVersion, Expr, ExprStmt, FnDecl, ModuleItem, Stmt, VarDecl, VarDeclKind,
      VarDeclarator,
    },
    visit::{Fold, FoldWith},
  },
};
use walkdir::WalkDir;

use log::error;

#[derive(Parser)]
struct Cli {
  #[clap(
    short = 'p',
    long,
    default_value = "../../../stylex/packages",
    help = "Absolute or relative path to Stylex package repository.",
    value_name = "PATH"
  )]
  stylex_path: Option<String>,
}

fn read_all_files(root_path: &Path) -> Result<Vec<PathBuf>, walkdir::Error> {
  let mut files = Vec::new();
  for entry in WalkDir::new(root_path)
    .follow_links(true)
    .into_iter()
    .filter_map(Result::ok)
  {
    if entry.file_type().is_file() {
      files.push(entry.path().to_owned());
    }
  }
  Ok(files)
}

struct TestsTransformer {
  module_items: Vec<ModuleItem>,
}

impl Fold for TestsTransformer {
  fn fold_module_items(&mut self, module_items: Vec<ModuleItem>) -> Vec<ModuleItem> {
    for item in module_items {
      match item {
        ModuleItem::Stmt(Stmt::Expr(ExprStmt { expr, .. })) => {
          if matches!(expr.as_ref(), Expr::Call(_) | Expr::Ident(_) | Expr::Fn(_)) {
            expr.fold_with(self);
          }
        },
        _ => match item {
          ModuleItem::Stmt(Stmt::Decl(Decl::Fn(func_decl))) => {
            func_decl.fold_with(self);
          },
          _ => {
            if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var))) = item {
              var.fold_with(self);
            }
          },
        },
      }
    }

    self.module_items.clone()
  }
  fn fold_call_expr(&mut self, call_expression: CallExpr) -> CallExpr {
    let black_list = ["require", "jest"];

    let binding = call_expression
      .callee
      .as_expr()
      .and_then(|expr| match *expr.clone() {
        Expr::Ident(ident) => Some(ident.sym.to_string()),
        Expr::Member(member) => Some(member.obj.as_ident().unwrap().sym.to_string()),
        _ => None,
      })
      .unwrap_or_default();

    let fn_name = binding.as_str();

    if !black_list.contains(&fn_name) {
      self
        .module_items
        .push(ModuleItem::Stmt(Stmt::from(ExprStmt {
          span: DUMMY_SP,
          expr: Box::new(Expr::from(call_expression.clone())),
        })));
    }
    call_expression
  }

  fn fold_var_declarator(&mut self, var_declarator: VarDeclarator) -> VarDeclarator {
    if !&var_declarator.init.clone().unwrap().is_call() {
      self
        .module_items
        .push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
          span: DUMMY_SP,
          kind: VarDeclKind::Var,
          declare: false,
          decls: vec![var_declarator.clone()],
          ctxt: SyntaxContext::empty(),
        })))));
    }

    var_declarator
  }

  fn fold_fn_decl(&mut self, func_decl: FnDecl) -> FnDecl {
    self
      .module_items
      .push(ModuleItem::Stmt(Stmt::Decl(Decl::Fn(func_decl.clone()))));

    func_decl
  }
}

struct TypeScriptStripper;

impl Fold for TypeScriptStripper {
  fn fold_binding_ident(
    &mut self,
    mut node: swc_core::ecma::ast::BindingIdent,
  ) -> swc_core::ecma::ast::BindingIdent {
    node.type_ann = None;

    node.optional = false;

    node
  }

  fn fold_function(
    &mut self,
    mut node: swc_core::ecma::ast::Function,
  ) -> swc_core::ecma::ast::Function {
    node.return_type = None;

    node.fold_children_with(self)
  }

  fn fold_key_value_prop(
    &mut self,
    mut node: swc_core::ecma::ast::KeyValueProp,
  ) -> swc_core::ecma::ast::KeyValueProp {
    let new_value = match node.value.as_ref() {
      Expr::TsAs(ts_as) => Box::new((*ts_as.expr).clone()),
      _ => node.value.clone(),
    };

    node.value = new_value;

    node
  }
}

fn transform_file(file_path: &Path, dir: &str) -> Result<(), std::io::Error> {
  let is_snapshot = file_path.to_string_lossy().ends_with(".snap");

  let source_code = read_to_string(file_path)?.replace("{ ... }", "{ }");

  let code = if is_snapshot {
    source_code
  } else {
    let mut transformer = TestsTransformer {
      module_items: Vec::new(),
    };
    // let compiler = Compiler::new(Default::default());

    let cm: Arc<SourceMap> = Default::default();

    let file_path_string = file_path.to_string_lossy().into_owned();
    let file_name = FileName::Custom(file_path_string);

    let fm = cm.new_source_file(file_name.into(), source_code);
    // let compiler = Compiler::new(Default::default());
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let program = parse_js(
      cm.clone(),
      fm,
      &handler,
      EsVersion::EsNext,
      Syntax::Typescript(Default::default()),
      IsModule::Bool(true),
      None,
    )
    .unwrap();

    // Strip TypeScript annotations first
    let mut typescript_stripper = TypeScriptStripper;
    let mut program = program.fold_with(&mut typescript_stripper);

    program = program.fold_with(&mut transformer);

    let transformed_code = print(
      cm,
      &program,
      PrintArgs {
        source_map: SourceMapsConfig::Bool(false),
        ..Default::default()
      },
    );

    transformed_code.unwrap().code
  };

  let test_file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();

  write_file(dir, &test_file_name, code)
}

fn write_file<P: Display, C: AsRef<[u8]>>(
  dir: P,
  test_file_name: P,
  transformed_code: C,
) -> Result<(), io::Error> {
  write(format!("{}/{}", dir, test_file_name), transformed_code)
}

fn create_dir_if_not_exists(dir: &str) -> io::Result<()> {
  fs::remove_dir_all(dir).unwrap_or_default();
  fs::create_dir_all(dir)
}

fn run(stylex_dir: &str) {
  let packages = [
    "babel-plugin",
    "shared",
    "stylex",
    "open-props",
    "benchmarks",
  ];

  let re = Regex::new(r"(tests?\.js|tests?\.js\.snap)$").unwrap();

  for package in packages.iter() {
    let path = format!("{}/{}", stylex_dir, package);
    let root_path = Path::new(path.as_str());

    let file_paths = match read_all_files(root_path) {
      Ok(paths) => paths,
      Err(err) => {
        error!("Error reading files: {}", err);
        return;
      },
    };

    let file_paths = file_paths
      .into_iter()
      .filter(|f| {
        re.is_match(&f.display().to_string()).unwrap_or_else(|err| {
          error!(
            "Error matching regex for '{}': {}. Skipping pattern match.",
            f.display(),
            err
          );

          false
        }) || f.display().to_string().contains("__tests__")
          || f.display().to_string().contains("/test/")
          || f.display().to_string().contains("/tests/")
      })
      .collect::<Vec<PathBuf>>();

    let dir = format!("./output/__tests__/{}", package);

    create_dir_if_not_exists(dir.as_str()).unwrap();

    for path in &file_paths {
      if let Err(err) = transform_file(path, dir.as_str()) {
        error!("Error transforming file {}: {}", path.display(), err);
      }
    }
  }
}

fn main() {
  pretty_env_logger::formatted_builder().init();
  color_backtrace::install();

  let cli = Cli::parse();

  let stylex_dir = cli
    .stylex_path
    .expect("Please provide a path to the stylex package");
  run(stylex_dir.as_str());
}

#[cfg(test)]
mod tests {
  use std::{
    fs,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
  };

  use super::*;
  use ctor::ctor;

  static CWD_LOCK: Mutex<()> = Mutex::new(());

  #[ctor]
  fn init_color_backtrace() {
    let _ = pretty_env_logger::formatted_builder().try_init();
    color_backtrace::install();
  }

  fn unique_temp_dir(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{unique}"))
  }

  #[test]
  fn read_all_files_collects_nested_files() {
    let temp_dir = unique_temp_dir("stylex-test-parser-read");
    fs::create_dir_all(temp_dir.join("nested")).unwrap();
    fs::write(temp_dir.join("root.js"), "const a = 1;").unwrap();
    fs::write(temp_dir.join("nested").join("child.js"), "const b = 2;").unwrap();

    let files = read_all_files(&temp_dir).unwrap();
    assert_eq!(files.len(), 2);

    fs::remove_dir_all(temp_dir).unwrap();
  }

  #[test]
  fn write_and_recreate_output_directory() {
    let temp_dir = unique_temp_dir("stylex-test-parser-write");
    create_dir_if_not_exists(temp_dir.to_str().unwrap()).unwrap();
    write_file(
      temp_dir.display().to_string(),
      "sample.js".to_string(),
      "const x = 1;",
    )
    .unwrap();
    assert!(temp_dir.join("sample.js").exists());

    // Recreating should clear previous contents.
    create_dir_if_not_exists(temp_dir.to_str().unwrap()).unwrap();
    assert!(!temp_dir.join("sample.js").exists());

    fs::remove_dir_all(temp_dir).unwrap();
  }

  #[test]
  fn transform_file_handles_snapshot_and_regular_js() {
    let temp_dir = unique_temp_dir("stylex-test-parser-transform");
    let output_dir = temp_dir.join("out");
    fs::create_dir_all(&output_dir).unwrap();

    let regular_file = temp_dir.join("sample.test.js");
    fs::write(&regular_file, "const value = 1; value;").unwrap();
    transform_file(&regular_file, output_dir.to_str().unwrap()).unwrap();
    assert!(output_dir.join("sample.test.js").exists());

    let snap_file = temp_dir.join("sample.tests.js.snap");
    fs::write(&snap_file, "SNAPSHOT_CONTENT").unwrap();
    transform_file(&snap_file, output_dir.to_str().unwrap()).unwrap();
    assert_eq!(
      fs::read_to_string(output_dir.join("sample.tests.js.snap")).unwrap(),
      "SNAPSHOT_CONTENT"
    );

    fs::remove_dir_all(temp_dir).unwrap();
  }

  #[test]
  fn run_processes_all_expected_packages() {
    let _guard = CWD_LOCK.lock().unwrap();
    let temp_workspace = unique_temp_dir("stylex-test-parser-run");
    let stylex_root = temp_workspace.join("stylex");

    for package in [
      "babel-plugin",
      "shared",
      "stylex",
      "open-props",
      "benchmarks",
    ] {
      let package_dir = stylex_root.join(package);
      fs::create_dir_all(package_dir.join("__tests__")).unwrap();
      fs::write(
        package_dir.join("__tests__").join("fixture.test.js"),
        "const fixture = 1; fixture;",
      )
      .unwrap();
    }

    let original_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_workspace).unwrap();
    run(stylex_root.to_str().unwrap());
    std::env::set_current_dir(&original_cwd).unwrap();

    for package in [
      "babel-plugin",
      "shared",
      "stylex",
      "open-props",
      "benchmarks",
    ] {
      let generated = temp_workspace
        .join("output")
        .join("__tests__")
        .join(package)
        .join("fixture.test.js");
      assert!(
        generated.exists(),
        "missing generated file for package: {package}"
      );
    }

    fs::remove_dir_all(temp_workspace).unwrap();
  }
}
