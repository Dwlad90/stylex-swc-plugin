use anyhow::Error;
use log::{debug, warn};
use std::{
  cell::Cell,
  cmp::Reverse,
  collections::hash_map::DefaultHasher,
  fs,
  hash::{Hash, Hasher},
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
    BytePos, DUMMY_SP, EqIgnoreSpan, FileName, Mark, SourceMap, Span, Spanned, SyntaxContext,
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
  structures::state_manager::StateManager,
  utils::ast::{
    convertors::{convert_concat_to_tpl_expr, convert_simple_tpl_to_str_expr},
    helpers::namespace_name_from_prop_key,
  },
};
use rustc_hash::FxHashSet;
use stylex_regex::regex::URL_REGEX;

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
  call_expr: &CallExpr,
  namespace_key: &str,
  state: &mut StateManager,
) -> Result<(CodeFrame, Span), Error> {
  // Same panic boundary as `get_span_from_source_code`: locating a span is
  // best-effort and must never abort the compilation.
  locate_span_with_panic_boundary(|| {
    get_key_span_from_source_code_impl(call_expr, namespace_key, state)
  })
}

fn get_key_span_from_source_code_impl(
  call_expr: &CallExpr,
  namespace_key: &str,
  state: &mut StateManager,
) -> Result<(CodeFrame, Span), Error> {
  let sibling_keys = collect_object_arg_keys(call_expr);
  let namespace_value_keys = collect_namespace_value_keys(call_expr, namespace_key);
  let cache_key = compute_key_span_cache_key(
    call_expr,
    namespace_key,
    &sibling_keys,
    &namespace_value_keys,
  );
  let file_name = FileName::Custom(state.get_filename().to_owned());

  if let Some(cached_span) = state.cached_span(cache_key) {
    let code_frame = load_code_frame_from_cache_for_state(&file_name, state)?;
    return Ok((code_frame, cached_span));
  }

  let code_frame = CodeFrame::new();
  let wrapped_expression = Expr::Call(call_expr.clone());
  let program = get_memoized_frame_source_code(
    &wrapped_expression,
    &wrapped_expression,
    state,
    &file_name,
    &code_frame,
  )
  .ok_or_else(|| anyhow::anyhow!("Failed to parse source file: {}", state.get_filename()))?;

  let mut finder = KeySpanFinder {
    namespace_key,
    sibling_keys: &sibling_keys,
    namespace_value_keys: &namespace_value_keys,
    target_lo: first_object_arg_span(call_expr)
      .or_else(|| (!call_expr.span.is_dummy()).then_some(call_expr.span.lo)),
    best: None,
    ambiguous_best: false,
  };
  program.visit_with(&mut finder);

  let span = finder.resolved_span();

  state.insert_cached_span(cache_key, span);

  Ok((code_frame, span))
}

/// Collects the literal property keys of a call's first object argument.
fn collect_object_arg_keys(call_expr: &CallExpr) -> FxHashSet<Atom> {
  match call_expr.args.first().map(|arg| arg.expr.as_ref()) {
    Some(Expr::Object(object)) => collect_object_lit_keys(object).collect(),
    _ => FxHashSet::default(),
  }
}

fn first_object_arg_span(call_expr: &CallExpr) -> Option<BytePos> {
  call_expr
    .args
    .first()
    .and_then(|arg| match arg.expr.as_ref() {
      Expr::Object(object) if !object.span.is_dummy() => Some(object.span.lo),
      _ => None,
    })
}

fn collect_namespace_value_keys(call_expr: &CallExpr, namespace_key: &str) -> FxHashSet<Atom> {
  let mut keys = FxHashSet::default();

  if let Some(arg) = call_expr.args.first()
    && let Expr::Object(object) = arg.expr.as_ref()
  {
    for prop in &object.props {
      if let PropOrSpread::Prop(prop) = prop
        && let Prop::KeyValue(key_value) = prop.as_ref()
        && namespace_name_from_prop_key(&key_value.key)
          .is_some_and(|name| name.as_ref() == namespace_key)
        && let Expr::Object(namespace_value) = key_value.value.as_ref()
      {
        keys.extend(collect_object_lit_keys(namespace_value));
        break;
      }
    }
  }

  keys
}

fn collect_object_lit_keys(object: &ObjectLit) -> impl Iterator<Item = Atom> + '_ {
  object.props.iter().filter_map(|prop| {
    if let PropOrSpread::Prop(prop) = prop
      && let Prop::KeyValue(key_value) = prop.as_ref()
    {
      namespace_name_from_prop_key(&key_value.key)
    } else {
      None
    }
  })
}

fn compute_key_span_cache_key(
  call_expr: &CallExpr,
  namespace_key: &str,
  sibling_keys: &FxHashSet<Atom>,
  namespace_value_keys: &FxHashSet<Atom>,
) -> u64 {
  let mut hasher = DefaultHasher::new();
  "stylex-key-span:v2".hash(&mut hasher);
  call_expr.callee.hash(&mut hasher);
  call_expr.span.lo.0.hash(&mut hasher);
  call_expr.span.hi.0.hash(&mut hasher);
  if let Some(arg) = call_expr.args.first()
    && let Expr::Object(object) = arg.expr.as_ref()
  {
    object.span.lo.0.hash(&mut hasher);
    object.span.hi.0.hash(&mut hasher);
  }
  namespace_key.hash(&mut hasher);

  let mut sorted_keys: Vec<&Atom> = sibling_keys.iter().collect();
  sorted_keys.sort();
  sorted_keys.hash(&mut hasher);

  let mut sorted_value_keys: Vec<&Atom> = namespace_value_keys.iter().collect();
  sorted_value_keys.sort();
  sorted_value_keys.hash(&mut hasher);

  hasher.finish()
}

struct KeySpanCandidate {
  namespace_value_overlap: usize,
  overlap: usize,
  distance_from_target: Option<u32>,
  span: Span,
}

impl KeySpanCandidate {
  /// Ranking key: higher is better. A smaller distance to the target wins,
  /// hence the `Reverse`.
  fn rank(&self) -> (usize, usize, Reverse<Option<u32>>) {
    (
      self.namespace_value_overlap,
      self.overlap,
      Reverse(self.distance_from_target),
    )
  }
}

/// Visitor that finds call-expression object arguments and returns the property
/// span for `namespace_key`. The sibling-key overlap is a tie-breaker for
/// compiled calls with dummy spans.
struct KeySpanFinder<'a> {
  namespace_key: &'a str,
  sibling_keys: &'a FxHashSet<Atom>,
  namespace_value_keys: &'a FxHashSet<Atom>,
  target_lo: Option<BytePos>,
  best: Option<KeySpanCandidate>,
  ambiguous_best: bool,
}

impl Visit for KeySpanFinder<'_> {
  noop_visit_type!();

  fn visit_call_expr(&mut self, call: &CallExpr) {
    if let Some(arg) = call.args.first()
      && let Expr::Object(object) = arg.expr.as_ref()
      && let Some(candidate) = self.candidate_from_object(call, object)
    {
      self.record_candidate(candidate);
    }

    call.visit_children_with(self);
  }
}

impl KeySpanFinder<'_> {
  fn candidate_from_object(&self, call: &CallExpr, object: &ObjectLit) -> Option<KeySpanCandidate> {
    let mut key_span = None;
    let mut namespace_value_overlap = 0;
    let mut overlap = 0;

    for prop in &object.props {
      if let PropOrSpread::Prop(prop) = prop
        && let Prop::KeyValue(key_value) = prop.as_ref()
        && let Some(name) = namespace_name_from_prop_key(&key_value.key)
      {
        if self.sibling_keys.contains(&name) {
          overlap += 1;
        }

        if name.as_ref() == self.namespace_key {
          key_span = Some(key_value.key.span());

          if let Expr::Object(namespace_value) = key_value.value.as_ref() {
            namespace_value_overlap = collect_object_lit_keys(namespace_value)
              .filter(|name| self.namespace_value_keys.contains(name))
              .count();
          }
        }
      }
    }

    key_span.map(|span| KeySpanCandidate {
      namespace_value_overlap,
      overlap,
      distance_from_target: self.target_lo.map(|target_lo| {
        let candidate_lo = if !object.span.is_dummy() {
          object.span.lo
        } else {
          call.span.lo
        };

        candidate_lo.0.abs_diff(target_lo.0)
      }),
      span,
    })
  }

  fn record_candidate(&mut self, candidate: KeySpanCandidate) {
    match self.best.as_ref() {
      None => {
        self.best = Some(candidate);
        self.ambiguous_best = false;
      },
      Some(best) if candidate.rank() > best.rank() => {
        self.best = Some(candidate);
        self.ambiguous_best = false;
      },
      Some(best) if candidate.rank() == best.rank() => {
        self.ambiguous_best = true;
      },
      Some(_) => {},
    }
  }

  fn resolved_span(self) -> Span {
    if self.ambiguous_best {
      DUMMY_SP
    } else {
      self.best.map_or(DUMMY_SP, |candidate| candidate.span)
    }
  }
}

/// Loads a CodeFrame with the source file for error display.
fn load_code_frame_from_cache_for_state(
  file_name: &FileName,
  state: &StateManager,
) -> Result<CodeFrame, Error> {
  let code_frame = CodeFrame::new();
  let source = state
    .get_seen_module_source_code()
    .and_then(|(_, source_code)| source_code.as_ref().cloned())
    .map(Ok)
    .unwrap_or_else(|| {
      read_source_file(file_name)
        .map_err(|error| anyhow::anyhow!("Failed to read source file: {}", error))
    })?;

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

  // The target expression's own span belongs to the caller's source map, not
  // the code-frame one, so its byte offsets are meaningless here and can even
  // land inside a multi-byte character, panicking on source-map lookups. A
  // dummy span signals "location unknown" instead.
  fallback_finder.get_span().unwrap_or(DUMMY_SP)
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
