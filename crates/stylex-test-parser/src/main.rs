use clap::Parser;
use regex::Regex;
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
        }
        _ => match item {
          ModuleItem::Stmt(Stmt::Decl(Decl::Fn(func_decl))) => {
            func_decl.fold_with(self);
          }
          _ => {
            if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var))) = item {
              var.fold_with(self);
            }
          }
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

    let mut program = parse_js(
      cm.clone(),
      fm,
      &handler,
      EsVersion::EsNext,
      Syntax::Typescript(Default::default()),
      IsModule::Bool(true),
      None,
    )
    .unwrap();

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

fn main() {
  pretty_env_logger::formatted_builder().init();
  color_backtrace::install();

  let cli = Cli::parse();

  let stylex_dir = cli
    .stylex_path
    .expect("Please provide a path to the stylex package");

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
      }
    };

    let file_paths = file_paths
      .into_iter()
      .filter(|f| {
        re.is_match(&f.display().to_string())
          || f.display().to_string().contains("__tests__")
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

#[cfg(test)]
mod tests {
  use ctor::ctor;

  #[ctor]
  fn init_color_backtrace() {
    pretty_env_logger::formatted_builder().init();
    color_backtrace::install();
  }
}
