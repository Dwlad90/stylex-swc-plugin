use clap::Parser;

use std::{
    fs::{self, read_to_string, write},
    io,
    path::{Path, PathBuf},
    sync::Arc,
};
use swc_core::{
    base::{
        config::{IsModule, SourceMapsConfig},
        Compiler, PrintArgs,
    },
    common::{
        errors::{ColorConfig, Handler},
        FileName, SourceMap, DUMMY_SP,
    },
    ecma::{
        ast::{
            CallExpr, Decl, EsVersion, Expr, ExprStmt, FnDecl, ModuleItem, Stmt, VarDecl,
            VarDeclKind, VarDeclarator,
        },
        parser::Syntax,
        visit::{Fold, FoldWith},
    },
};
use walkdir::WalkDir;

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
            if let ModuleItem::Stmt(Stmt::Expr(ExprStmt { expr, .. })) = item {
                if matches!(expr.as_ref(), Expr::Call(_) | Expr::Ident(_) | Expr::Fn(_)) {
                    expr.fold_with(self);
                }
            } else if let ModuleItem::Stmt(Stmt::Decl(Decl::Fn(func_decl))) = item {
                func_decl.fold_with(self);
            } else if let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var))) = item {
                var.fold_with(self);
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
            self.module_items
                .push(ModuleItem::Stmt(Stmt::from(ExprStmt {
                    span: DUMMY_SP,
                    expr: Box::new(Expr::from(call_expression.clone())),
                })));
        }
        call_expression
    }

    fn fold_var_declarator(&mut self, var_declarator: VarDeclarator) -> VarDeclarator {
        if !&var_declarator.init.clone().unwrap().is_call() {
            self.module_items
                .push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
                    span: DUMMY_SP,
                    kind: VarDeclKind::Var,
                    declare: false,
                    decls: vec![var_declarator.clone()],
                })))));
        }

        var_declarator
    }

    fn fold_fn_decl(&mut self, func_decl: FnDecl) -> FnDecl {
        self.module_items
            .push(ModuleItem::Stmt(Stmt::Decl(Decl::Fn(func_decl.clone()))));

        func_decl
    }
}

fn transform_file(file_path: &Path, dir: &str) -> Result<(), std::io::Error> {
    let mut transformer = TestsTransformer {
        module_items: Vec::new(),
    };
    let compiler = Compiler::new(Default::default());

    let source_code = read_to_string(file_path)?;

    let cm: Arc<SourceMap> = Default::default();

    let file_path_string = file_path.to_string_lossy().into_owned();
    let file_name = FileName::Custom(file_path_string);

    let fm = cm.new_source_file(file_name, source_code);
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let mut program = compiler
        .parse_js(
            fm,
            &handler,
            EsVersion::EsNext,
            Syntax::Typescript(Default::default()),
            IsModule::Bool(true),
            None,
        )
        .unwrap();

    program = program.fold_with(&mut transformer);

    let transformed_code = compiler.print(
        &program,
        PrintArgs {
            source_map: SourceMapsConfig::Bool(false),
            ..Default::default()
        },
    );

    let test_file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();

    write(
        format!("{}/{}", dir, test_file_name),
        transformed_code.unwrap().code,
    )
}

fn create_dir_if_not_exists(dir: &str) -> io::Result<()> {
    fs::remove_dir_all(dir).unwrap_or_default();
    fs::create_dir_all(dir)
}

fn main() {
    let cli = Cli::parse();

    let stylex_dir = cli
        .stylex_path
        .expect("Please provide a path to the stylex package");

    let packages = ["babel-plugin", "shared", "stylex", "open-props"];

    for package in packages.iter() {
        let path = format!("{}/{}", stylex_dir, package);
        let root_path = Path::new(path.as_str());

        let file_paths = match read_all_files(root_path) {
            Ok(paths) => paths,
            Err(err) => {
                eprintln!("Error reading files: {}", err);
                return;
            }
        };

        let file_paths = file_paths
            .into_iter()
            .filter(|f| f.display().to_string().ends_with("-test.js"))
            .collect::<Vec<PathBuf>>();

        let dir = format!("./output/__tests__/{}", package);

        create_dir_if_not_exists(dir.as_str()).unwrap();

        for path in &file_paths {
            if let Err(err) = transform_file(path, dir.as_str()) {
                eprintln!("Error transforming file {}: {}", path.display(), err);
            }
        }
    }
}
