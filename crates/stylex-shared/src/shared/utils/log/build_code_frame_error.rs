use std::sync::Arc;
use swc_compiler_base::{parse_js, print, IsModule, PrintArgs, SourceMapsConfig};
use swc_core::{
  common::{errors::*, EqIgnoreSpan, FileName, SourceMap, Span, Spanned, DUMMY_SP},
  ecma::{ast::*, visit::*},
};
use swc_ecma_parser::Syntax;

struct CodeFrame {
  source_map: Arc<SourceMap>,
  handler: Handler,
}

impl CodeFrame {
  fn new() -> Self {
    let source_map: Arc<SourceMap> = Default::default();
    let handler =
      Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(source_map.clone()));
    Self {
      source_map,
      handler,
    }
  }

  fn create_error<'a>(&'a self, span: Span, message: &str) -> DiagnosticBuilder<'a> {
    let mut diagnostic = self.handler.struct_err(message);
    diagnostic.set_span(span);

    let loc = self.source_map.lookup_char_pos(span.lo);
    let error_message = format!(
      "{}: {}\n{}",
      loc.file.name,
      message,
      self.format_code_frame(span)
    );
    diagnostic.note(&error_message);
    diagnostic
  }

  fn format_code_frame(&self, span: Span) -> String {
    let loc = self.source_map.lookup_char_pos(span.lo);
    let file = loc.file;
    let start_line = loc.line.saturating_sub(2);
    let end_line = loc.line + 2;

    (start_line..=end_line)
      .filter_map(|line_idx| {
        file.get_line(line_idx).map(|line| {
          let mut output = format!("  {}\n", line);
          if line_idx == loc.line {
            output.push_str(&format!(
              "  {}{}\n",
              " ".repeat(loc.col.0),
              "^".repeat((span.hi - span.lo).0 as usize)
            ));
          }
          output
        })
      })
      .collect()
  }
}

pub(crate) fn build_code_frame_error<'a>(
  wrapped_expression: &'a Expr,
  fault_expression: &'a Expr,
  error_message: &'a str,
  file_name: &'a str,
) -> &'a str {
  let code_frame = CodeFrame::new();
  let file_name = FileName::Custom(file_name.to_owned());

  let program = Program::Module(Module {
    span: DUMMY_SP,
    body: vec![ModuleItem::Stmt(Stmt::Expr(ExprStmt {
      span: DUMMY_SP,
      expr: Box::new(wrapped_expression.clone()),
    }))],
    shebang: None,
  });

  if let Ok(output_code) = print(
    code_frame.source_map.clone(),
    &program,
    PrintArgs {
      source_map: SourceMapsConfig::Bool(false),
      ..Default::default()
    },
  ) {
    let file = code_frame
      .source_map
      .new_source_file(Arc::new(file_name), output_code.code);

    let mut finder = ExpressionFinder::new(fault_expression);

    if let Ok(program) = parse_js(
      code_frame.source_map.clone(),
      file.clone(),
      &code_frame.handler,
      EsVersion::EsNext,
      Syntax::Typescript(Default::default()),
      IsModule::Bool(true),
      None,
    ) {
      program.fold_with(&mut finder);
      if let Some(span) = finder.get_span() {
        code_frame.create_error(span, error_message).emit();
      }
    }
  }

  error_message
}

#[derive(Debug)]
struct ExpressionFinder<'a> {
  target: &'a Expr,
  found_expr: Option<Expr>,
}

impl<'a> ExpressionFinder<'a> {
  fn new(target: &'a Expr) -> Self {
    Self {
      target,
      found_expr: None,
    }
  }

  fn get_span(&self) -> Option<Span> {
    let expr = self.found_expr.as_ref()?;

    Some(Span::new(expr.span_lo(), expr.span_hi()))
  }
}

impl<'a> Fold for ExpressionFinder<'a> {
  fn fold_expr(&mut self, expr: Expr) -> Expr {
    if self.target.eq_ignore_span(&expr) {
      self.found_expr = Some(expr.clone());
      expr
    } else {
      expr.fold_children_with(self)
    }
  }
}

pub(crate) fn build_code_frame_error_and_panic(
  wrapped_expression: &Expr,
  fault_expression: &Expr,
  error_message: &str,
  file_name: &str,
) -> ! {
  panic!(
    "{}",
    build_code_frame_error(
      wrapped_expression,
      fault_expression,
      error_message,
      file_name,
    )
  );
}
