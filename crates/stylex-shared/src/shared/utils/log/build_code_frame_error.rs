use anyhow::Error;
use log::{debug, warn};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{fs, path::Path, sync::Arc};
use swc_compiler_base::{PrintArgs, SourceMapsConfig, TransformOutput, parse_js, print};
use swc_config::is_module::IsModule;
use swc_core::{
  common::{
    DUMMY_SP, EqIgnoreSpan, FileName, SourceMap, Span, Spanned, SyntaxContext,
    errors::{Handler, *},
  },
  ecma::{
    ast::*,
    codegen::Config,
    parser::{Syntax, TsSyntax},
    visit::*,
  },
};

use crate::shared::{
  regex::URL_REGEX,
  structures::state_manager::StateManager,
  utils::ast::convertors::{convert_concat_to_tpl_expr, convert_simple_tpl_to_str_expr},
};

pub(crate) struct CodeFrame {
  source_map: Arc<SourceMap>,
  handler: Handler,
}

impl CodeFrame {
  pub(crate) fn new() -> Self {
    let source_map = Arc::new(SourceMap::default());
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
    }
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
    }
  }

  error_message
}

/// Finds the span (source location) of a target expression within the source code.
/// Uses caching to avoid redundant AST traversals for the same expression.
///
/// # Arguments
/// * `wrapped_expression` - The parent expression containing the target
/// * `target_expression` - The specific expression to locate
/// * `state` - Mutable reference to the state manager (for caching)
///
/// # Returns
/// A tuple of (CodeFrame, Span) where CodeFrame contains the source map for error display
pub(crate) fn get_span_from_source_code(
  wrapped_expression: &Expr,
  target_expression: &Expr,
  state: &mut StateManager,
) -> Result<(CodeFrame, Span), Error> {
  let cache_key = compute_cache_key(target_expression);
  let file_name = FileName::Custom(state.get_filename().to_owned());

  // Check cache first - avoid expensive AST operations if we've seen this before
  if let Some(&cached_span) = state.span_cache.get(&cache_key) {
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

  let span = find_expression_span(&program, target_expression);

  // Cache the result for future lookups
  state.span_cache.insert(cache_key, span);

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
fn find_expression_span(program: &Program, target_expression: &Expr) -> Span {
  let mut finder = ExpressionFinder::new(target_expression);
  let _program = program.clone().fold_with(&mut finder);

  if let Some(span) = finder.get_span() {
    return span;
  }

  // Fallback: try finding after template literal conversion
  let converted_target = target_expression.clone().fold_with(&mut TplConverter {});
  let mut fallback_finder = ExpressionFinder::new(&converted_target);
  let _program = program.clone().fold_with(&mut fallback_finder);

  fallback_finder
    .get_span()
    .unwrap_or_else(|| target_expression.span())
}

/// Gets or parses the source code as a Program AST, with memoization.
/// Returns a cleaned and normalized Program that can be used for expression finding.
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
    program.as_module().expect("Program must be a module"),
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
      // Clean and normalize: remove syntax contexts, convert template literals
      let normalized = program
        .fold_with(&mut Cleaner {})
        .fold_with(&mut TplConverter {});
      Some(normalized)
    }
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
    }
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
/// Uses discriminant matching for fast filtering before expensive eq_ignore_span checks.
#[derive(Debug)]
struct ExpressionFinder {
  target: Expr,
  target_discriminant: std::mem::Discriminant<Expr>,
  found_expr: Option<Expr>,
}

/// Visitor that normalizes AST by removing syntax contexts and type annotations.
/// This allows for more reliable expression matching across different parsing contexts.
#[derive(Debug)]
struct Cleaner {}
impl Fold for Cleaner {
  noop_fold_type!();

  fn fold_binding_ident(&mut self, mut node: BindingIdent) -> BindingIdent {
    node.id.ctxt = SyntaxContext::empty();
    node.type_ann = None;
    node.fold_children_with(self)
  }

  fn fold_ident(&mut self, mut ident: Ident) -> Ident {
    ident.ctxt = SyntaxContext::empty();
    ident.fold_children_with(self)
  }
}

impl ExpressionFinder {
  fn new(target: &Expr) -> Self {
    let cleaned_target = target.clone().fold_children_with(&mut Cleaner {});
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

impl Fold for TplConverter {
  noop_fold_type!();

  fn fold_expr(&mut self, expr: Expr) -> Expr {
    let expr = convert_concat_to_tpl_expr(expr);
    let expr = convert_simple_tpl_to_str_expr(expr);
    expr.fold_children_with(self)
  }
}

impl Fold for ExpressionFinder {
  noop_fold_type!();

  fn fold_expr(&mut self, expr: Expr) -> Expr {
    if self.found_expr.is_some() {
      return expr;
    }

    // Fast discriminant check filters expressions by variant type
    if std::mem::discriminant(&expr) != self.target_discriminant {
      return expr.fold_children_with(self);
    }

    // Expensive structural comparison only for matching variants
    if self.target.eq_ignore_span(&expr) {
      self.found_expr = Some(expr.clone());
      return expr;
    }

    expr.fold_children_with(self)
  }
}

#[track_caller]
pub(crate) fn build_code_frame_error_and_panic(
  wrapped_expression: &Expr,
  fault_expression: &Expr,
  error_message: &str,
  state: &mut StateManager,
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
