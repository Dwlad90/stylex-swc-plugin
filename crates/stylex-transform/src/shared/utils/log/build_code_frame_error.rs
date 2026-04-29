use anyhow::Error;
use log::{debug, warn};
use std::{
  collections::hash_map::DefaultHasher,
  fs,
  hash::{Hash, Hasher},
  path::Path,
  sync::{Arc, OnceLock},
};
use stylex_macros::{panic_macros::__stylex_panic, stylex_error::StyleXError, stylex_panic};
use swc_compiler_base::{PrintArgs, SourceMapsConfig, TransformOutput, parse_js, print};
use swc_config::is_module::IsModule;
use swc_core::{
  common::{
    DUMMY_SP, EqIgnoreSpan, FileName, Mark, SourceMap, Span, Spanned, SyntaxContext,
    errors::{Handler, *},
    util::take::Take,
  },
  ecma::{
    ast::*,
    codegen::Config,
    parser::{Syntax, TsSyntax},
    transforms::typescript::strip,
    visit::*,
  },
};

use crate::shared::{
  structures::state_manager::StateManager,
  utils::ast::convertors::{convert_concat_to_tpl_expr, convert_simple_tpl_to_str_expr},
};
use stylex_regex::regex::URL_REGEX;

pub(crate) struct CodeFrame {
  source_map: Arc<SourceMap>,
  handler: Handler,
}

static SOURCE_MAP: OnceLock<Arc<SourceMap>> = OnceLock::new();

impl CodeFrame {
  pub(crate) fn new() -> Self {
    let source_map = SOURCE_MAP
      .get_or_init(|| Arc::new(SourceMap::default()))
      .clone();

    let handler =
      Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(source_map.clone()));

    Self {
      source_map,
      handler,
    }
  }

  pub(crate) fn create_error<'a>(&'a self, span: Span, message: &str) -> DiagnosticBuilder<'a> {
    let prefixed_message = format!("[StyleX] {}", message);
    let mut diagnostic = self.handler.struct_span_err(span, &prefixed_message);

    let urls = URL_REGEX
      .find_iter(message)
      .filter_map(|m| m.ok().map(|m| m.as_str()))
      .collect::<Vec<_>>();

    let note = format!("\n{}", urls.join("\n"));

    diagnostic.warn("Line number isn't real, it's just a placeholder, Please check the actual line number in your editor.");

    diagnostic.note(note.as_str());

    diagnostic
  }

  pub(crate) fn get_span_line_number(&self, span: Span) -> usize {
    self.source_map.lookup_char_pos(span.lo).line
  }
}

fn read_source_file(file_name: &FileName) -> Result<String, std::io::Error> {
  match file_name {
    FileName::Real(path) => fs::read_to_string(path),
    FileName::Custom(path) => fs::read_to_string(path),
    FileName::Url(url) => fs::read_to_string(Path::new(url.path())),
    _ => Err(std::io::Error::other("Unsupported file name type")),
  }
}

pub(crate) fn build_code_frame_error<'a>(
  wrapped_expression: &'a Expr,
  fault_expression: &'a Expr,
  error_message: &'a str,
  state: &mut StateManager,
) -> &'a str {
  match get_span_from_source_code(wrapped_expression, fault_expression, state) {
    Ok((code_frame, span)) => {
      code_frame.create_error(span, error_message).emit();
    },
    Err(error) => {
      if log::log_enabled!(log::Level::Debug) {
        debug!(
          "Failed to generate code frame error: {:?}. File: {}. Expression: {:?}.",
          error,
          state.get_filename(),
          fault_expression,
        );
      } else {
        warn!(
          "Failed to generate code frame error: {:?}. File: {}. For more information enable debug logging.",
          error,
          state.get_filename(),
        )
      };
    },
  }

  error_message
}

/// Finds the span (source location) of a target expression within the source
/// code. Uses caching to avoid redundant AST traversals for the same
/// expression.
///
/// # Arguments
/// * `wrapped_expression` - The parent expression containing the target
/// * `target_expression` - The specific expression to locate
/// * `state` - Mutable reference to the state manager (for caching)
///
/// # Returns
/// A tuple of (CodeFrame, Span) where CodeFrame contains the source map for
/// error display
pub(crate) fn get_span_from_source_code(
  wrapped_expression: &Expr,
  target_expression: &Expr,
  state: &mut StateManager,
) -> Result<(CodeFrame, Span), Error> {
  let cache_key = compute_cache_key(target_expression);
  let file_name = FileName::Custom(state.get_filename().to_owned());

  // Check cache first - avoid expensive AST operations if we've seen this before
  if let Some(cached_span) = state.cached_span(cache_key) {
    let code_frame = load_code_frame_from_cache(&file_name)?;
    return Ok((code_frame, cached_span));
  }

  let code_frame = CodeFrame::new();
  let program = get_memoized_frame_source_code(
    wrapped_expression,
    target_expression,
    state,
    &file_name,
    &code_frame,
  )
  .ok_or_else(|| anyhow::anyhow!("Failed to parse source file: {}", state.get_filename()))?;

  let span = find_expression_span(program, target_expression);

  // Cache the result for future lookups
  state.insert_cached_span(cache_key, span);

  Ok((code_frame, span))
}

/// Computes a cache key for an expression based on its type and structure
fn compute_cache_key(expr: &Expr) -> u64 {
  let mut hasher = DefaultHasher::new();
  std::mem::discriminant(expr).hash(&mut hasher);
  expr.hash(&mut hasher);
  hasher.finish()
}

/// Loads a CodeFrame with the source file for error display
fn load_code_frame_from_cache(file_name: &FileName) -> Result<CodeFrame, Error> {
  let code_frame = CodeFrame::new();
  let source = read_source_file(file_name)
    .map_err(|e| anyhow::anyhow!("Failed to read source file: {}", e))?;
  code_frame
    .source_map
    .new_source_file(file_name.clone().into(), source);
  Ok(code_frame)
}

/// Finds the span of a target expression within a program AST
fn find_expression_span(program: Program, target_expression: &Expr) -> Span {
  let mut finder = ExpressionFinder::new(target_expression);
  program.visit_with(&mut finder);

  if let Some(span) = finder.get_span() {
    return span;
  }

  // Fallback: try finding after template literal conversion
  let mut converted_target = target_expression.clone();
  converted_target.visit_mut_with(&mut TplConverter {});
  let mut fallback_finder = ExpressionFinder::new(&converted_target);
  program.visit_with(&mut fallback_finder);

  fallback_finder
    .get_span()
    .unwrap_or_else(|| target_expression.span())
}

/// Gets or parses the source code as a Program AST, with memoization.
/// Returns a cleaned and normalized Program that can be used for expression
/// finding.
fn get_memoized_frame_source_code(
  wrapped_expression: &Expr,
  target_expression: &Expr,
  state: &mut StateManager,
  file_name: &FileName,
  code_frame: &CodeFrame,
) -> Option<Program> {
  if let Some((cached_program, source_code)) = state.get_seen_module_source_code()
    && let Some(source_code) = source_code
  {
    code_frame
      .source_map
      .new_source_file(Arc::new(file_name.clone()), source_code.to_owned());
    return Some(Program::Module(cached_program.clone()));
  }

  let source_code = get_source_code(wrapped_expression, state, file_name, code_frame)?;

  let source_file = code_frame
    .source_map
    .new_source_file(Arc::new(file_name.clone()), source_code.clone());

  let program = parse_and_normalize_program(
    &source_file,
    code_frame,
    state.get_filename(),
    target_expression,
  )?;

  state.set_seen_module_source_code(
    match program.as_module() {
      Some(module) => module,
      #[cfg_attr(coverage_nightly, coverage(off))]
      None => stylex_panic!("Expected a module program for source code caching."),
    },
    Some(source_code),
  );

  Some(program)
}

/// Gets the source code with the following priority:
/// 1. seen_source_code from state (if not yet normalized)
/// 2. Read from file (original source)
/// 3. Create synthetic module (fallback)
fn get_source_code(
  wrapped_expression: &Expr,
  state: &StateManager,
  file_name: &FileName,
  code_frame: &CodeFrame,
) -> Option<String> {
  if let Some((module, source_code)) = state.get_seen_module_source_code() {
    if let Some(source_code) = source_code {
      return Some(source_code.clone());
    } else {
      return Some(print_module(
        code_frame,
        module.clone(),
        Some(
          Config::default()
            .with_minify(false)
            .with_omit_last_semi(false)
            .with_reduce_escaped_newline(false)
            .with_inline_script(false),
        ),
      ));
    }
  }
  if let Ok(source) = read_source_file(file_name) {
    return Some(source);
  }

  let synthetic_module = create_module(wrapped_expression);
  Some(print_module(code_frame, synthetic_module, None))
}

/// Parses source code into a Program AST and normalizes it
fn parse_and_normalize_program(
  source_file: &Arc<swc_core::common::SourceFile>,
  code_frame: &CodeFrame,
  filename: &str,
  target_expression: &Expr,
) -> Option<Program> {
  let parse_result = parse_js(
    code_frame.source_map.clone(),
    source_file.clone(),
    &code_frame.handler,
    EsVersion::EsNext,
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    IsModule::Bool(true),
    None,
  );

  match parse_result {
    Ok(program) => {
      let unresolved_mark = Mark::new();
      let top_level_mark = Mark::new();

      // Clean and normalize: remove syntax contexts, convert template literals
      let mut normalized = program.apply(strip(unresolved_mark, top_level_mark));
      normalized.visit_mut_with(&mut TplConverter {});
      Some(normalized)
    },
    Err(error) => {
      if log::log_enabled!(log::Level::Debug) {
        debug!(
          "Failed to parse program: {:?}. File: {}. Expression: {:?}",
          error, filename, target_expression
        );
      } else {
        warn!("Failed to parse program: {:?}. File: {}", error, filename);
      }
      None
    },
  }
}

pub(crate) fn print_module(
  code_frame: &CodeFrame,
  module: Module,
  codegen_config: Option<Config>,
) -> String {
  print_program(code_frame, Program::Module(module), codegen_config)
}

pub(crate) fn print_program(
  code_frame: &CodeFrame,
  program: Program,
  codegen_config: Option<Config>,
) -> String {
  let printed_source_code = print(
    code_frame.source_map.clone(),
    &program,
    PrintArgs {
      source_map: SourceMapsConfig::Bool(false),
      codegen_config: codegen_config.unwrap_or_default(),
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

pub(crate) fn create_module(wrapped_expression: &Expr) -> Module {
  Module {
    span: DUMMY_SP,
    body: vec![ModuleItem::Stmt(Stmt::Expr(ExprStmt {
      span: DUMMY_SP,
      expr: Box::new(wrapped_expression.clone()),
    }))],
    shebang: None,
  }
}

/// Visitor that searches for a specific expression in an AST.
/// Uses discriminant matching for fast filtering before expensive
/// eq_ignore_span checks.
#[derive(Debug)]
struct ExpressionFinder {
  target: Expr,
  target_discriminant: std::mem::Discriminant<Expr>,
  found_expr: Option<Expr>,
}

/// Visitor that normalizes AST by removing syntax contexts and type
/// annotations. This allows for more reliable expression matching across
/// different parsing contexts.
#[derive(Debug)]
struct Cleaner {}
impl VisitMut for Cleaner {
  noop_visit_mut_type!();

  fn visit_mut_binding_ident(&mut self, node: &mut BindingIdent) {
    node.id.ctxt = SyntaxContext::empty();
    node.type_ann = None;
    node.visit_mut_children_with(self);
  }

  fn visit_mut_ident(&mut self, ident: &mut Ident) {
    ident.ctxt = SyntaxContext::empty();
    ident.visit_mut_children_with(self);
  }
}

impl ExpressionFinder {
  fn new(target: &Expr) -> Self {
    let mut cleaned_target = target.clone();
    cleaned_target.visit_mut_children_with(&mut Cleaner {});
    let target_discriminant = std::mem::discriminant(&cleaned_target);

    Self {
      target: cleaned_target,
      target_discriminant,
      found_expr: None,
    }
  }

  fn get_span(&self) -> Option<Span> {
    let expr = self.found_expr.as_ref()?;

    Some(Span::new(expr.span_lo(), expr.span_hi()))
  }
}

/// Visitor that normalizes template literals and string concatenations.
/// Helps match expressions that may be written differently in source vs AST.
#[derive(Debug)]
struct TplConverter {}

impl VisitMut for TplConverter {
  noop_visit_mut_type!();

  fn visit_mut_expr(&mut self, expr: &mut Expr) {
    let converted = convert_simple_tpl_to_str_expr(convert_concat_to_tpl_expr(expr.take()));
    *expr = converted;
    expr.visit_mut_children_with(self);
  }
}

impl Visit for ExpressionFinder {
  noop_visit_type!();

  fn visit_expr(&mut self, expr: &Expr) {
    if self.found_expr.is_some() {
      return;
    }

    // Fast discriminant check filters expressions by variant type
    if std::mem::discriminant(expr) != self.target_discriminant {
      expr.visit_children_with(self);
      return;
    }

    // Expensive structural comparison only for matching variants
    if self.target.eq_ignore_span(expr) {
      self.found_expr = Some(expr.clone());
      return;
    }

    expr.visit_children_with(self);
  }
}

#[track_caller]
#[cold]
pub(crate) fn build_code_frame_error_and_panic(
  wrapped_expression: &Expr,
  fault_expression: &Expr,
  error_message: &str,
  state: &mut StateManager,
) -> ! {
  let caller_location = std::panic::Location::caller();

  // Emit the code frame diagnostic to stderr (already [StyleX]-prefixed)
  let (file, line) = match get_span_from_source_code(wrapped_expression, fault_expression, state) {
    Ok((code_frame, span)) => {
      code_frame.create_error(span, error_message).emit();
      let line_num = code_frame.get_span_line_number(span);
      (Some(state.get_filename().to_owned()), Some(line_num))
    },
    Err(error) => {
      if log::log_enabled!(log::Level::Debug) {
        debug!(
          "Failed to generate code frame error: {:?}. File: {}. Expression: {:?}.",
          error,
          state.get_filename(),
          fault_expression,
        );
      } else {
        warn!(
          "Failed to generate code frame error: {:?}. File: {}. For more information enable debug logging.",
          error,
          state.get_filename(),
        );
      }
      (Some(state.get_filename().to_owned()), None)
    },
  };

  let err = StyleXError {
    message: error_message.to_string().into(),
    file: file.map(Into::into),
    key_path: None,
    line,
    col: None,
    source_location: Some(format!("{}:{}", caller_location.file(), caller_location.line()).into()),
  };

  __stylex_panic(err)
}

#[track_caller]
#[cold]
pub(crate) fn build_code_frame_error_and_panic_at(
  expr: &Expr,
  error_message: &str,
  state: &mut StateManager,
) -> ! {
  build_code_frame_error_and_panic(expr, expr, error_message, state)
}
