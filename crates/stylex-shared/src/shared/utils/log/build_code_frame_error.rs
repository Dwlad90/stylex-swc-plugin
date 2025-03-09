use std::{fs, path::Path, sync::Arc};
use swc_compiler_base::{IsModule, PrintArgs, SourceMapsConfig, TransformOutput, parse_js, print};
use swc_core::{
  common::{
    DUMMY_SP, EqIgnoreSpan, FileName, SourceMap, Span, Spanned, SyntaxContext, errors::*, sync::Lrc,
  },
  ecma::{ast::*, visit::*},
};
use swc_ecma_parser::{Syntax, TsSyntax};

use crate::shared::{regex::URL_REGEX, structures::state_manager::StateManager};

pub(crate) struct CodeFrame {
  source_map: Lrc<SourceMap>,
  handler: Handler,
}

impl CodeFrame {
  fn new() -> Self {
    let source_map: Lrc<SourceMap> = Default::default();
    let handler =
      Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(source_map.clone()));

    Self {
      source_map,
      handler,
    }
  }

  pub(crate) fn create_error<'a>(&'a self, span: Span, message: &str) -> DiagnosticBuilder<'a> {
    let mut diagnostic = self.handler.struct_span_err(span, message);

    let urls = URL_REGEX
      .find_iter(message)
      .map(|m| m.as_str())
      .collect::<Vec<_>>();

    let note = format!("\n{}", urls.join("\n"));

    diagnostic.warn("Line number isn't real, it's just a placeholder, Please check the actual line number in your editor.");

    diagnostic.note(note.as_str());

    diagnostic
  }

  pub(crate) fn get_span_line_number(&self, span: Span) -> usize {
    let loc = self.source_map.lookup_char_pos(span.lo);

    loc.line
  }

  // fn format_code_frame(&self, span: Span) -> String {
  //   let loc = self.source_map.lookup_char_pos(span.lo);
  //   let file = loc.file;
  //   let start_line = loc.line.saturating_sub(2);
  //   let end_line = loc.line + 2;

  //   (start_line..=end_line)
  //     .filter_map(|line_idx| {
  //       file.get_line(line_idx).map(|line| {
  //         let mut output = format!("  {}\n", line);
  //         if line_idx == loc.line - 1 {
  //           output.push_str(&format!(
  //             "  {}{}\n",
  //             " ".repeat(loc.col.0),
  //             "^".repeat((span.hi - span.lo).0 as usize)
  //           ));
  //         }
  //         output
  //       })
  //     })
  //     .collect()
}

fn read_source_file(file_name: &FileName) -> Result<String, std::io::Error> {
  match file_name {
    FileName::Real(path) => fs::read_to_string(path),
    FileName::Custom(path) => fs::read_to_string(path),
    FileName::Url(url) => fs::read_to_string(Path::new(url.path())),
    _ => Err(std::io::Error::new(
      std::io::ErrorKind::Other,
      "Unsupported file name type",
    )),
  }
}

pub(crate) fn build_code_frame_error<'a>(
  wrapped_expression: &'a Expr,
  fault_expression: &'a Expr,
  error_message: &'a str,
  state: &StateManager,
) -> &'a str {
  let (code_frame, span) = get_span_from_source_code(wrapped_expression, fault_expression, state);

  code_frame.create_error(span, error_message).emit();

  error_message
}

pub(crate) fn get_span_from_source_code(
  wrapped_expression: &Expr,
  fault_expression: &Expr,
  state: &StateManager,
) -> (CodeFrame, Span) {
  // let file_name = FileName::Custom("/Users/vladislavbuinovski/Projects/Facebook/stylex-swc-plugin.git/stylexjs/crates/stylex-shared/tests/fixture/page/input.js".to_owned());
  let file_name = FileName::Custom(state.get_filename().to_owned());
  let code_frame = CodeFrame::new();

  let source_code = read_source_file(&file_name);

  let frame_source_code = source_code.unwrap_or_else(|_| {
    let module = if cfg!(debug_assertions) && state.get_debug_assertions_module().is_some() {
      state.get_debug_assertions_module().unwrap().clone()
    } else {
      create_module(wrapped_expression)
    };

    print_module(&code_frame, module)
  });

  let file = code_frame
    .source_map
    .new_source_file(Arc::new(file_name), frame_source_code);

  let mut finder = ExpressionFinder::new(fault_expression);

  if let Ok(program) = parse_js(
    code_frame.source_map.clone(),
    file.clone(),
    &code_frame.handler,
    EsVersion::EsNext,
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    IsModule::Bool(true),
    None,
  ) {
    program.fold_with(&mut finder);

    return (
      code_frame,
      finder.get_span().unwrap_or(fault_expression.span()),
    );
  }

  (code_frame, fault_expression.span())
}

fn print_module(code_frame: &CodeFrame, module: Module) -> String {
  let program = Program::Module(module);

  let printed_source_code = print(
    code_frame.source_map.clone(),
    &program,
    PrintArgs {
      source_map: SourceMapsConfig::Bool(false),
      ..Default::default()
    },
  )
  .unwrap_or_else(|_| TransformOutput {
    code: "".to_string(),
    map: None,
    output: None,
    diagnostics: Vec::default(),
  });

  printed_source_code.code
}

fn create_module(wrapped_expression: &Expr) -> Module {
  Module {
    span: DUMMY_SP,
    body: vec![ModuleItem::Stmt(Stmt::Expr(ExprStmt {
      span: DUMMY_SP,
      expr: Box::new(wrapped_expression.clone()),
    }))],
    shebang: None,
  }
}

#[derive(Debug)]
struct ExpressionFinder {
  target: Expr,
  found_expr: Option<Expr>,
}

#[derive(Debug)]
struct Cleaner {}
impl Fold for Cleaner {
  noop_fold_type!();

  fn fold_ident(&mut self, ident: Ident) -> Ident {
    let mut new_ident = ident.clone();

    new_ident.ctxt = SyntaxContext::empty();

    new_ident
  }
}

impl ExpressionFinder {
  fn new(target: &Expr) -> Self {
    Self {
      target: target.clone().fold_children_with(&mut Cleaner {}),
      found_expr: None,
    }
  }

  fn get_span(&self) -> Option<Span> {
    let expr = self.found_expr.as_ref()?;

    Some(Span::new(expr.span_lo(), expr.span_hi()))
  }
}

impl Fold for ExpressionFinder {
  noop_fold_type!();

  fn fold_expr(&mut self, expr: Expr) -> Expr {
    if self.found_expr.is_some() {
      return expr;
    }

    let expr = expr.clone().fold_children_with(self);

    if self.target.eq_ignore_span(&expr) {
      self.found_expr = Some(expr.clone());
      expr
    } else {
      expr.fold_children_with(self)
    }
  }
}

#[track_caller]
pub(crate) fn build_code_frame_error_and_panic(
  wrapped_expression: &Expr,
  fault_expression: &Expr,
  error_message: &str,
  state: &StateManager,
) -> ! {
  let caller_location = std::panic::Location::caller();

  let enhanced_message = format!(
    "{} (called from {}:{})",
    error_message,
    caller_location.file(),
    caller_location.line()
  );

  build_code_frame_error(
    wrapped_expression,
    fault_expression,
    &enhanced_message,
    state,
  );

  panic!("{}", enhanced_message);
}
