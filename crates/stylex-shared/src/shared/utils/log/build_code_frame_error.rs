use std::{fs, path::Path, sync::Arc};
use swc_compiler_base::{parse_js, print, IsModule, PrintArgs, SourceMapsConfig, TransformOutput};
use swc_core::{
  common::{errors::*, EqIgnoreSpan, FileName, SourceMap, Span, Spanned, DUMMY_SP},
  ecma::{ast::*, visit::*},
};
use swc_ecma_parser::{Syntax, TsSyntax};

use crate::shared::{regex::URL_REGEX, structures::state_manager::StateManager};

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
// }

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
  let file_name = FileName::Custom(state.get_filename().to_owned());
  let code_frame = CodeFrame::new();

  let source_code = read_source_file(&file_name);

  let frame_source_code = source_code.unwrap_or_else(|_| {
    let program = Program::Module(Module {
      span: DUMMY_SP,
      body: vec![ModuleItem::Stmt(Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(wrapped_expression.clone()),
      }))],
      shebang: None,
    });

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
    });

    printed_source_code.code
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

    if let Some(span) = finder.get_span() {
      code_frame.create_error(span, error_message).emit();
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
  state: &StateManager,
) -> ! {
  panic!(
    "{}",
    build_code_frame_error(wrapped_expression, fault_expression, error_message, state)
  );
}
