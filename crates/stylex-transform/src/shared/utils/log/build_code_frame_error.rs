use anyhow::Error;
use log::{debug, warn};
use std::{
  cell::Cell,
  fs,
  panic::{self, AssertUnwindSafe, UnwindSafe},
  path::Path,
  sync::{Arc, Once, OnceLock},
};
use stylex_macros::{panic_macros::__stylex_panic, stylex_error::StyleXError, stylex_panic};
use swc_compiler_base::{PrintArgs, SourceMapsConfig, TransformOutput, parse_js, print};
use swc_config::is_module::IsModule;
use swc_core::{
  atoms::Atom,
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
    utils::DropSpan,
    visit::*,
  },
};

use crate::shared::{
  structures::{
    key_span_index::{CallKeys, NamespaceKeyQuery},
    state_manager::StateManager,
  },
  utils::ast::convertors::{convert_concat_to_tpl_expr, convert_simple_tpl_to_str_expr},
};
use stylex_regex::regex::URL_REGEX;
use stylex_utils::hash::stable_hash_wide;

pub(crate) struct CodeFrame {
  source_map: Arc<SourceMap>,
  handler: Handler,
}

static SOURCE_MAP: OnceLock<Arc<SourceMap>> = OnceLock::new();
static DIAGNOSTIC_PANIC_HOOK: Once = Once::new();

thread_local! {
  static SUPPRESS_DIAGNOSTIC_PANIC_HOOK: Cell<bool> = const { Cell::new(false) };
}

fn install_diagnostic_panic_hook() {
  DIAGNOSTIC_PANIC_HOOK.call_once(|| {
    let previous_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
      let suppress = SUPPRESS_DIAGNOSTIC_PANIC_HOOK.with(Cell::get);
      if !suppress {
        previous_hook(panic_info);
      }
    }));
  });
}

fn catch_diagnostic_unwind<F, T>(operation: F) -> std::thread::Result<T>
where
  F: FnOnce() -> T + UnwindSafe,
{
  install_diagnostic_panic_hook();

  let previous_suppression = SUPPRESS_DIAGNOSTIC_PANIC_HOOK.with(|suppress| suppress.replace(true));
  let result = panic::catch_unwind(operation);
  SUPPRESS_DIAGNOSTIC_PANIC_HOOK.with(|suppress| suppress.set(previous_suppression));

  result
}

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

  /// Registers `source` for `file_name` unless the shared source map already
  /// holds it, and reports whether the file is available afterwards.
  ///
  /// The closure is only called on a miss, so a caller whose source is expensive
  /// to produce -- a clone of the module's text, or a read of the file -- pays
  /// for it once per module rather than once per lookup.
  ///
  /// The guard is a linear scan of the source map's file list, which grows by one
  /// entry per module a process transforms. That is the trade this makes: a
  /// string compare per registered module, against the copy of the module's text
  /// it replaces. A long-lived process transforming five thousand modules pays
  /// fifty thousand name comparisons where it used to accumulate fifty thousand
  /// source files.
  fn register_source_once(
    &self,
    file_name: &FileName,
    source: impl FnOnce() -> Result<String, Error>,
  ) -> Result<(), Error> {
    if self.source_map.get_source_file(file_name).is_some() {
      return Ok(());
    }

    self
      .source_map
      .new_source_file(file_name.clone().into(), source()?);

    Ok(())
  }

  pub(crate) fn get_span_line_number(&self, span: Span) -> usize {
    self.source_map.lookup_char_pos(span.lo).line
  }

  /// Emits the diagnostic behind a panic boundary: the code frame is a
  /// best-effort aid, so a source-map lookup panic (e.g. a span whose byte
  /// offsets fall inside a multi-byte character) must never replace the error
  /// being reported.
  pub(crate) fn emit_error(&self, span: Span, message: &str) {
    let emitted = catch_diagnostic_unwind(AssertUnwindSafe(|| {
      self.create_error(span, message).emit();
    }));

    if emitted.is_err() {
      warn!("Failed to emit the code frame for error: {}", message);
    }
  }

  /// Like `get_span_line_number`, but behind the same panic boundary as
  /// `emit_error` and `None` for dummy spans ("location unknown").
  pub(crate) fn try_get_span_line_number(&self, span: Span) -> Option<usize> {
    if span.is_dummy() {
      return None;
    }

    catch_diagnostic_unwind(AssertUnwindSafe(|| self.get_span_line_number(span))).ok()
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
      code_frame.emit_error(span, error_message);
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
  // Panic boundary: locating a span re-reads, re-prints, and re-parses the
  // module purely to improve diagnostics; a panic anywhere in there must
  // degrade to "no code frame", never abort the compilation.
  locate_span_with_panic_boundary(|| {
    get_span_from_source_code_impl(wrapped_expression, target_expression, state)
  })
}

/// Runs a span-locating closure behind the diagnostic panic boundary,
/// degrading a panic to a regular "no code frame" error.
fn locate_span_with_panic_boundary(
  locate: impl FnOnce() -> Result<(CodeFrame, Span), Error>,
) -> Result<(CodeFrame, Span), Error> {
  catch_diagnostic_unwind(AssertUnwindSafe(locate)).unwrap_or_else(|_| {
    Err(anyhow::anyhow!(
      "Panicked while locating the source span for a diagnostic"
    ))
  })
}

fn get_span_from_source_code_impl(
  wrapped_expression: &Expr,
  target_expression: &Expr,
  state: &mut StateManager,
) -> Result<(CodeFrame, Span), Error> {
  let cache_key = compute_cache_key(target_expression);
  let file_name = FileName::Custom(state.get_filename().to_owned());

  // Check cache first - avoid expensive AST operations if we've seen this before
  if let Some(cached_span) = state.cached_span(cache_key) {
    let code_frame = load_code_frame_from_cache_for_state(&file_name, state)?;
    return Ok((code_frame, cached_span));
  }

  let code_frame = CodeFrame::new();

  let span = with_memoized_module(
    wrapped_expression,
    target_expression,
    state,
    &file_name,
    &code_frame,
    |module| find_expression_span(module, target_expression),
  )?;

  // Cache the result for future lookups
  state.insert_cached_span(cache_key, span);

  Ok((code_frame, span))
}

/// Computes a cache key for an expression based on its type and structure.
///
/// 128 bits, because the read side acts on a hit alone: `cached_span` returns a
/// span and the caller turns it straight into a `file:line`. Unlike the
/// evaluator's memo, a collision here is *directly observable* -- a style
/// annotated with another style's line number -- so the width is the only thing
/// standing behind it.
fn compute_cache_key(expr: &Expr) -> u128 {
  stable_hash_wide(&(std::mem::discriminant(expr), expr))
}

/// Finds the span of a style namespace by its **key** inside the parsed
/// source, instead of matching the namespace's value expression.
///
/// Object keys are static strings that survive value-level code transforms
/// (e.g. compile-time macro expansion done by an earlier loader), so this
/// locates the original source position even when the compiled AST's values
/// no longer textually match the file on disk — the case where
/// `find_expression_span` has nothing to match against.
///
/// The `call_expr`'s own argument keys are used to disambiguate between
/// multiple objects containing an identically named property: the candidate
/// object sharing the most sibling keys with the compiled call wins.
///
/// Returns a dummy span when the key cannot be located.
pub(crate) fn get_key_span_from_source_code(
  wrapped_call: &Expr,
  call_expr: &CallExpr,
  call_keys: &CallKeys,
  siblings_digest: u128,
  namespace_key: &str,
  state: &mut StateManager,
) -> Result<(CodeFrame, Span), Error> {
  // Same panic boundary as `get_span_from_source_code`: locating a span is
  // best-effort and must never abort the compilation.
  locate_span_with_panic_boundary(|| {
    get_key_span_from_source_code_impl(
      wrapped_call,
      call_expr,
      call_keys,
      siblings_digest,
      namespace_key,
      state,
    )
  })
}

fn get_key_span_from_source_code_impl(
  wrapped_call: &Expr,
  call_expr: &CallExpr,
  call_keys: &CallKeys,
  siblings_digest: u128,
  namespace_key: &str,
  state: &mut StateManager,
) -> Result<(CodeFrame, Span), Error> {
  let query = NamespaceKeyQuery::for_namespace(call_keys, call_expr, namespace_key);
  let cache_key = compute_key_span_cache_key(siblings_digest, &query);
  let file_name = FileName::Custom(state.get_filename().to_owned());

  if let Some(cached_span) = state.cached_span(cache_key) {
    let code_frame = load_code_frame_from_cache_for_state(&file_name, state)?;
    return Ok((code_frame, cached_span));
  }

  let code_frame = CodeFrame::new();

  // The caller's already-wrapped call, not a second deep clone of it: this ran
  // once per namespace and the wrapper is the same expression for all of them.
  memoize_module(wrapped_call, wrapped_call, state, &file_name, &code_frame)?;

  // One index over the whole module, not one walk per namespace key: the debug
  // path asks this question once per style, and the walk it replaces made a
  // `dev` build quadratic in the size of a file that is one long list of them.
  let span = match state.key_span_index() {
    Some(index) => index.resolve(&query),
    None => return Err(missing_memoized_module(state)),
  };

  state.insert_cached_span(cache_key, span);

  Ok((code_frame, span))
}

/// The same, for a namespace-key lookup. 128 bits for the same reason as
/// [`compute_cache_key`].
///
/// Hashed as one tuple rather than field by field, so the wide hasher is built
/// once and the pieces cannot drift out of the key by being added to the
/// function and forgotten in the digest.
/// The half of a key-span cache key that belongs to the *call*.
///
/// Built once per `stylex.create` and mixed into each namespace's key, because
/// the callee, the spans and the sorted sibling keys are the same for every
/// namespace of one call -- and sorting the siblings per namespace is what made
/// the debug path quadratic in a call's own namespace count.
pub(crate) fn compute_call_siblings_digest(call_expr: &CallExpr, call_keys: &CallKeys) -> u128 {
  let object_span = call_expr
    .args
    .first()
    .and_then(|arg| match arg.expr.as_ref() {
      Expr::Object(object) => Some((object.span.lo.0, object.span.hi.0)),
      _ => None,
    });

  stable_hash_wide(&(
    "stylex-call-siblings:v1",
    &call_expr.callee,
    call_expr.span.lo.0,
    call_expr.span.hi.0,
    object_span,
    // Sorted, because a `FxHashSet`'s iteration order is not part of the
    // identity being keyed -- two calls with the same keys in a different order
    // are the same call.
    call_keys.sorted_sibling_keys(),
  ))
}

/// The per-namespace half, mixed with the digest above.
///
fn compute_key_span_cache_key(siblings_digest: u128, query: &NamespaceKeyQuery) -> u128 {
  let mut sorted_value_keys: Vec<&Atom> = query.namespace_value_keys.iter().collect();
  sorted_value_keys.sort();

  stable_hash_wide(&(
    "stylex-key-span:v4",
    siblings_digest,
    query.namespace_key,
    sorted_value_keys,
    query.target_lo.map(|lo| lo.0),
  ))
}

/// Loads a CodeFrame with the source file for error display.
fn load_code_frame_from_cache_for_state(
  file_name: &FileName,
  state: &StateManager,
) -> Result<CodeFrame, Error> {
  let code_frame = CodeFrame::new();

  // Registered at most once. The source map behind every `CodeFrame` is a
  // process-global `OnceLock`, so re-registering here would append another copy
  // of the module to it on every call -- and the debug-data path calls this once
  // per style. On a 200 KB module with 1 257 styles that was a quarter of a
  // gigabyte of duplicated source and the largest single cost in a `dev` build.
  code_frame.register_source_once(file_name, || {
    state
      .get_seen_module_source_code()
      .and_then(|(_, source_code)| source_code.as_ref().cloned())
      .map(Ok)
      .unwrap_or_else(|| {
        read_source_file(file_name)
          .map_err(|error| anyhow::anyhow!("Failed to read source file: {}", error))
      })
  })?;

  Ok(code_frame)
}

/// Finds the span of a target expression within a program AST
fn find_expression_span(module: &Module, target_expression: &Expr) -> Span {
  let mut finder = ExpressionFinder::new(target_expression);
  module.visit_with(&mut finder);

  if let Some(span) = finder.get_span() {
    return span;
  }

  // Fallback: try finding after template literal conversion
  let mut converted_target = target_expression.clone();
  converted_target.visit_mut_with(&mut TplConverter {});
  let mut fallback_finder = ExpressionFinder::new(&converted_target);
  module.visit_with(&mut fallback_finder);

  // The target expression's own span belongs to the caller's source map, not
  // the code-frame one, so its byte offsets are meaningless here and can even
  // land inside a multi-byte character, panicking on source-map lookups. A
  // dummy span signals "location unknown" instead.
  fallback_finder.get_span().unwrap_or(DUMMY_SP)
}

/// Gets or parses the source code as a Program AST, with memoization.
/// Returns a cleaned and normalized Program that can be used for expression
/// finding.
/// Runs `visit` against the module's parsed, normalized source, parsing and
/// memoizing it first if that has not happened yet.
///
/// Shaped as a closure rather than as "return me the program" for two reasons.
/// The program lives in the state, which is borrowed mutably here, so it cannot
/// be handed back and still leave the caller free to write the resolved span
/// back afterwards -- the closure's borrow ends when this returns. And the
/// previous shape *did* hand it back, as `Program::Module(module.clone())`: a
/// deep clone of the whole module, once per style in a `dev` build, which made
/// the annotation cost grow about quadratically with file size. A bigger module
/// was both more clones and a bigger clone.
fn with_memoized_module<T>(
  wrapped_expression: &Expr,
  target_expression: &Expr,
  state: &mut StateManager,
  file_name: &FileName,
  code_frame: &CodeFrame,
  visit: impl FnOnce(&Module) -> T,
) -> Result<T, Error> {
  memoize_module(
    wrapped_expression,
    target_expression,
    state,
    file_name,
    code_frame,
  )?;

  match state.get_seen_module_source_code() {
    Some((module, _)) => Ok(visit(module)),
    None => Err(missing_memoized_module(state)),
  }
}

/// The error for "the module should have been memoized by now".
///
/// Both callers reach it only after [`memoize_module`] returned `Ok`, which
/// either found a memoized module or stored one, so neither is reachable in
/// practice. It stays an error rather than a panic because the getters' types
/// cannot say so, and a diagnostic aid must never be the reason a build stops.
fn missing_memoized_module(state: &StateManager) -> Error {
  anyhow::anyhow!("Failed to parse source file: {}", state.get_filename())
}

/// Parses and memoizes the module's own source on the state, and registers that
/// source with `code_frame`, unless a previous lookup already did.
///
/// Separate from [`with_memoized_module`] because the namespace-key lookup does
/// not want the module itself: it wants the key span index built from it, which
/// the state hands out and which cannot be borrowed out of a closure holding the
/// module.
fn memoize_module(
  wrapped_expression: &Expr,
  target_expression: &Expr,
  state: &mut StateManager,
  file_name: &FileName,
  code_frame: &CodeFrame,
) -> Result<(), Error> {
  if let Some((_, source_code)) = state.get_seen_module_source_code()
    && let Some(source_code) = source_code
  {
    // Registered once, not once per lookup -- see `register_source_once`.
    code_frame.register_source_once(file_name, || Ok(source_code.to_owned()))?;
  } else {
    let source_code = get_source_code(wrapped_expression, state, file_name, code_frame)
      .ok_or_else(|| anyhow::anyhow!("Failed to read source file: {}", state.get_filename()))?;

    let source_file = code_frame
      .source_map
      .new_source_file(Arc::new(file_name.clone()), source_code.clone());

    let program = parse_and_normalize_program(
      &source_file,
      code_frame,
      state.get_filename(),
      target_expression,
    )
    .ok_or_else(|| anyhow::anyhow!("Failed to parse source file: {}", state.get_filename()))?;

    state.set_seen_module_source_code(
      match program.as_module() {
        Some(module) => module,
        // Unreachable: `parse_and_normalize_program` parses with
        // `IsModule::Bool(true)`, so a successful parse is always a module.
        // Kept as a panic rather than dropped because it guards an invariant of
        // that call rather than an input, and the day the call learns a second
        // mode this is where it should stop.
        None => stylex_panic!("Expected a module program for source code caching."),
      },
      Some(source_code),
    );
  }

  Ok(())
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
  mut program: Program,
  codegen_config: Option<Config>,
) -> String {
  // The printed AST carries spans from the compiler's own source map, which
  // are meaningless in the shared code-frame map. The codegen resolves
  // non-dummy spans against its source map (e.g. `span_to_snippet` for
  // trailing-comma detection), so foreign offsets would read unrelated files
  // and can panic mid-character on multi-byte sources.
  program.visit_mut_with(&mut DropSpan {});

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
    code: String::new(),
    map: None,
    output: None,
    diagnostics: Vec::default(),
    extracted_comments: None,
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
  found_span: Option<Span>,
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
      found_span: None,
    }
  }

  fn get_span(&self) -> Option<Span> {
    self.found_span
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
    if self.found_span.is_some() {
      return;
    }

    // Fast discriminant check filters expressions by variant type
    if std::mem::discriminant(expr) != self.target_discriminant {
      expr.visit_children_with(self);
      return;
    }

    // Expensive structural comparison only for matching variants
    if self.target.eq_ignore_span(expr) {
      self.found_span = Some(Span::new(expr.span_lo(), expr.span_hi()));
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
      code_frame.emit_error(span, error_message);
      let line_num = code_frame.try_get_span_line_number(span);
      (Some(state.get_filename().to_owned()), line_num)
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

#[cfg(test)]
#[path = "tests/build_code_frame_error_tests.rs"]
mod tests;
