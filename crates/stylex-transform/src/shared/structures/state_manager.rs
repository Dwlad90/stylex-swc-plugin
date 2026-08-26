use rustc_hash::{FxHashMap, FxHashSet};
use std::cell::OnceCell;
use std::{option::Option, path::Path, rc::Rc, sync::Arc};
use stylex_macros::{stylex_panic, stylex_unimplemented};

use indexmap::{IndexMap, IndexSet};
use log::debug;
use stylex_path_resolver::{
  package_json::{PackageJsonExtended, find_closest_package_json_folder, get_package_json},
  resolvers::{EXTENSIONS, resolve_file_path},
  utils::relative_path,
};
use swc_core::{
  atoms::Atom,
  common::{DUMMY_SP, EqIgnoreSpan, FileName, SourceFile, Span, SyntaxContext},
  ecma::{
    ast::{
      CallExpr, Callee, Decl, Expr, ExprStmt, Id, Ident, ImportDecl, ImportDefaultSpecifier,
      ImportNamedSpecifier, ImportPhase, ImportSpecifier, JSXAttrOrSpread, Lit, MemberExpr,
      MemberProp, Module, ModuleDecl, ModuleExportName, ModuleItem, NamedExport, Pat, PropName,
      Stmt, Str, VarDecl, VarDeclKind, VarDeclarator,
    },
    visit::{Visit, VisitMut, VisitMutWith, VisitWith},
  },
};

use crate::shared::utils::js::check_declaration::{DeclarationType, declares_ident};
use crate::shared::{
  structures::types::InjectableStylesMap,
  utils::{
    ast::{convertors::create_number_expr, helpers::namespace_name_from_member_prop},
    common::{extract_filename_from_path, extract_filename_with_ext_from_path, extract_path},
    validators::{is_attrs_call, is_props_call},
  },
};
use stylex_ast::ast::factories::{
  create_binding_ident, create_call_expr, create_expr_or_spread, create_jsx_attr_or_spread,
  create_jsx_spread_attr, create_key_value_prop, create_number_expr_or_spread,
  create_object_expression, create_string_expr_or_spread, create_string_key_value_prop,
};
use stylex_constants::constants::{
  api_names::{
    STYLEX_ATTRS, STYLEX_CREATE, STYLEX_CREATE_THEME, STYLEX_DEFAULT_MARKER, STYLEX_DEFINE_CONSTS,
    STYLEX_DEFINE_MARKER, STYLEX_DEFINE_VARS, STYLEX_ENV, STYLEX_FIRST_THAT_WORKS,
    STYLEX_KEYFRAMES, STYLEX_POSITION_TRY, STYLEX_PROPS, STYLEX_TYPES, STYLEX_UNSTABLE_CONDITIONAL,
    STYLEX_UNSTABLE_CREATE_THEME_NESTED, STYLEX_UNSTABLE_DEFINE_CONSTS_NESTED,
    STYLEX_UNSTABLE_DEFINE_VARS_NESTED, STYLEX_VIEW_TRANSITION_CLASS, STYLEX_WHEN,
  },
  common::{CONSTS_FILE_EXTENSION, DEFAULT_INJECT_PATH},
};
use stylex_enums::{
  core::TransformationCycle,
  counter_mode::CounterMode,
  import_path_resolution::ImportPathResolution,
  style_vars_to_keep::{NonNullProp, NonNullProps},
  top_level_expression::TopLevelExpressionKind,
};
use stylex_structures::{
  style_vars_to_keep::StyleVarsToKeep, top_level_expression::TopLevelExpression,
};
use stylex_types::enums::data_structures::injectable_style::InjectableStyleKind;
use stylex_utils::hash::{stable_hash_unspanned, stable_hash_unspanned_call};

use super::{
  key_span_index::{KeySpanIndex, ModuleBase},
  seen_value::SeenValue,
  types::{InjectImportIdents, SeenModuleSource, StylesObjectMap},
};
use stylex_structures::{
  named_import_source::{ImportSources, NamedImportSource, RuntimeInjectionState},
  plugin_pass::PluginPass,
  stylex_options::{CheckModuleResolution, StyleXOptions},
  stylex_state_options::StyleXStateOptions,
  uid_generator::UidGenerator,
};
use stylex_types::structures::meta_data::MetaData;

// LOCK: Rc<T> by design. SWC visitors are sequential per file; cross-file
// parallelism is provided by the host (Node worker pool calls into the
// NAPI binding from multiple threads, each with its own StateManager).
// Arc<T> would add atomic-RMW on every clone with no benefit.
const TRANSFORMED_VARS_FILE_EXTENSION: &str = ".transformed";

type AtomHashSet = FxHashSet<Atom>;

/// Stable identifier for a top-level declarator. Carries the symbol's
/// `Atom` together with its `SyntaxContext` so shadowed bindings remain
/// distinguishable after SWC's `resolver` pass has run.
pub(crate) type DeclId = swc_core::ecma::ast::Id;

/// Position in the final emitted module body where a [`PendingInsertion`]
/// item should land. The enum is deliberately narrow — new variants
/// land only when a real producer needs them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum InsertionSlot {
  /// Value-level namespace imports injected so an `sx` attribute can
  /// reference the `stylex` runtime (`import * as stylex from '...'`).
  /// Emitted after any leading directive prologue, ahead of all other
  /// imports — prepended to the module body. Always emitted, regardless of
  /// `runtime_injection`.
  PrependImport,
  /// Runtime injection helpers (`import _inject` + `var _inject2`).
  /// Emitted after any leading directive prologue, ahead of all
  /// other imports. Mirrors the legacy `prepend_include_module_items`
  /// placement.
  BeforeImports,
  /// Theme side-effect imports added by `treeshake_compensation`.
  /// Emitted between the runtime helpers and the existing import
  /// block. Mirrors the legacy `prepend_import_module_items`
  /// placement; the legacy code kept these separate from the
  /// runtime helpers because they always followed them, regardless
  /// of which producer ran first.
  ThemeImports,
  /// Hoisted dynamic-style constants. Emitted after the existing
  /// import block, before the rest of the body. Mirrors the legacy
  /// `hoisted_module_items` placement.
  AfterImports,
  /// Per-declarator style metadata (the `_inject2(...)` calls) keyed
  /// by the stable hash of the originating var-decl initializer.
  /// Emitted immediately before the matching declarator. Replaces the
  /// legacy `styles_to_inject` map.
  BeforeDecl(u128),
}

#[derive(Debug, Clone)]
pub(crate) struct PendingInsertion {
  pub(crate) slot: InsertionSlot,
  pub(crate) item: ModuleItem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ImportKind {
  Props,
  Attrs,
  Create,
  FirstThatWorks,
  Keyframes,
  DefineVars,
  DefineVarsNested,
  DefineMarker,
  DefineConsts,
  DefineConstsNested,
  CreateTheme,
  CreateThemeNested,
  Conditional,
  PositionTry,
  ViewTransitionClass,
  DefaultMarker,
  When,
  Types,
  Env,
}

impl ImportKind {
  pub(crate) fn from_import_name(name: &str) -> Option<ImportKind> {
    match name {
      STYLEX_CREATE => Some(ImportKind::Create),
      STYLEX_PROPS => Some(ImportKind::Props),
      STYLEX_ATTRS => Some(ImportKind::Attrs),
      STYLEX_KEYFRAMES => Some(ImportKind::Keyframes),
      STYLEX_FIRST_THAT_WORKS => Some(ImportKind::FirstThatWorks),
      STYLEX_DEFINE_VARS => Some(ImportKind::DefineVars),
      STYLEX_UNSTABLE_DEFINE_VARS_NESTED => Some(ImportKind::DefineVarsNested),
      STYLEX_DEFINE_CONSTS => Some(ImportKind::DefineConsts),
      STYLEX_UNSTABLE_DEFINE_CONSTS_NESTED => Some(ImportKind::DefineConstsNested),
      STYLEX_DEFINE_MARKER => Some(ImportKind::DefineMarker),
      STYLEX_CREATE_THEME => Some(ImportKind::CreateTheme),
      STYLEX_UNSTABLE_CREATE_THEME_NESTED => Some(ImportKind::CreateThemeNested),
      STYLEX_UNSTABLE_CONDITIONAL => Some(ImportKind::Conditional),
      STYLEX_POSITION_TRY => Some(ImportKind::PositionTry),
      STYLEX_VIEW_TRANSITION_CLASS => Some(ImportKind::ViewTransitionClass),
      STYLEX_TYPES => Some(ImportKind::Types),
      STYLEX_WHEN => Some(ImportKind::When),
      STYLEX_ENV => Some(ImportKind::Env),
      STYLEX_DEFAULT_MARKER => Some(ImportKind::DefaultMarker),
      _ => None,
    }
  }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ImportState {
  import_paths: FxHashSet<String>,
  /// Local names bound to a value-level `stylex` namespace/default import,
  /// in discovery order. Insertion-ordered (`IndexSet`) so the `sx` runtime
  /// binding reuse (`get_stylex_runtime_binding`) picks the first candidate
  /// deterministically.
  stylex_import: IndexSet<ImportSources>,
  stylex_api_imports: FxHashMap<ImportKind, AtomHashSet>,
}

impl ImportState {
  fn has_import_paths(&self) -> bool {
    !self.import_paths.is_empty()
  }

  fn insert_import_path(&mut self, source_path: String) {
    self.import_paths.insert(source_path);
  }

  fn insert_stylex_import(&mut self, import_source: ImportSources) {
    self.stylex_import.insert(import_source);
  }

  fn stylex_imports(&self) -> &IndexSet<ImportSources> {
    &self.stylex_import
  }

  fn has_stylex_api_import(&self, kind: ImportKind, sym: &Atom) -> bool {
    self
      .stylex_api_imports
      .get(&kind)
      .is_some_and(|set| set.contains(sym))
  }

  fn insert_stylex_api_import(&mut self, kind: ImportKind, sym: Atom) {
    self.stylex_api_imports.entry(kind).or_default().insert(sym);
  }

  fn get_stylex_api_import(&self, kind: ImportKind) -> Option<&AtomHashSet> {
    self.stylex_api_imports.get(&kind)
  }
}

/// The binding sets one `Discover` pre-scan produced, moved to the state
/// together.
///
/// A type rather than four parameters because all four are `FxHashSet<Id>`: see
/// [`StateManager::adopt_binding_writes`].
pub(crate) struct BindingWrites {
  /// See [`StateManager::binding_reassignments`].
  pub(crate) reassignments: FxHashSet<Id>,
  /// See [`StateManager::binding_mutations`].
  pub(crate) mutations: FxHashSet<Id>,
  /// See [`StateManager::binding_deep_mutations`].
  pub(crate) deep_mutations: FxHashSet<Id>,
  /// See [`StateManager::declared_bindings`].
  pub(crate) declared: FxHashSet<Id>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ModuleSourceState {
  seen_module_source_code: Option<Rc<SeenModuleSource>>,
  /// Where the module being transformed starts, in the compiler's own source
  /// map.
  ///
  /// Recorded because a key-span lookup compares a compiled call's position
  /// against candidates indexed from a module re-parsed into a *different* map.
  /// Only offsets into the file compare; this is the base they are taken from.
  ///
  /// `None` until the module walk records it, and a [`ModuleBase`] rather than
  /// a `BytePos` — that type carries why.
  input_module_base: Option<ModuleBase>,
}

impl ModuleSourceState {
  /// The memoized module's [`KeySpanIndex`], built on the first lookup that
  /// needs it.
  ///
  /// Lazy because only a `debug` build asks for it, and built once because the
  /// debug path asks once per style namespace: the walk it replaces was the
  /// largest cost in a `dev` transform.
  fn key_span_index(&self) -> Option<&KeySpanIndex> {
    let seen_module_source = self.seen_module_source_code.as_deref()?;

    Some(
      seen_module_source
        .key_span_index
        .get_or_init(|| KeySpanIndex::build(&seen_module_source.module)),
    )
  }

  fn get_seen_module_source_code(&self) -> Option<(&Module, &Option<String>)> {
    let seen_module_source = self.seen_module_source_code.as_deref()?;

    Some((&seen_module_source.module, &seen_module_source.source_code))
  }

  fn set_seen_module_source_code(&mut self, module: &Module, source_code: Option<String>) {
    self.seen_module_source_code = Some(Rc::new(SeenModuleSource {
      module: module.clone(),
      source_code,
      // Built from the module above, so it cannot outlive it.
      key_span_index: OnceCell::new(),
    }));
  }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CallExpressionState {
  all_call_expressions: FxHashMap<u128, Callee>,
}

impl CallExpressionState {
  fn add_call_expression(&mut self, call_expr: &CallExpr) {
    self.all_call_expressions.insert(
      stable_hash_unspanned_call(call_expr),
      call_expr.callee.clone(),
    );
  }

  fn is_member_callee(&self, member: &MemberExpr) -> bool {
    self
      .all_call_expressions
      .values()
      .any(|call_expr_callee| match call_expr_callee {
        Callee::Expr(callee) => match callee.as_ref() {
          Expr::Member(call_member) => call_member.eq_ignore_span(member),
          _ => false,
        },
        _ => false,
      })
  }

  fn replace_call_expression(&mut self, call: &CallExpr, ast: &Expr) {
    self
      .all_call_expressions
      .remove(&stable_hash_unspanned_call(call));

    if let Some(call_expr) = ast.as_call() {
      self.add_call_expression(call_expr);
    }
  }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CacheState {
  css_property_seen: FxHashMap<String, String>,
  span_cache: FxHashMap<u128, Span>,
  short_filename_cache: FxHashMap<String, String>,
  /// Per refused expression, the binding whose declaration its code frame
  /// should point at instead of the expression itself.
  ///
  /// Keyed by the expression, not held as one slot, because a refusal is not
  /// always the end of the build: a dynamic style's value falls through to an
  /// inline style, so a later, unrelated diagnostic must not inherit an earlier
  /// refusal's declaration.
  framed_declarations: FxHashMap<u128, Atom>,
}

impl CacheState {
  fn cached_span(&self, cache_key: u128) -> Option<Span> {
    self.span_cache.get(&cache_key).copied()
  }

  fn insert_span(&mut self, cache_key: u128, span: Span) {
    self.span_cache.insert(cache_key, span);
  }

  fn frame_declaration(&mut self, cache_key: u128, name: Atom) {
    self.framed_declarations.insert(cache_key, name);
  }

  fn framed_declaration(&self, cache_key: u128) -> Option<&Atom> {
    self.framed_declarations.get(&cache_key)
  }

  fn has_framed_declarations(&self) -> bool {
    !self.framed_declarations.is_empty()
  }

  fn cached_short_filename(&self, absolute_path: &str) -> Option<&str> {
    self
      .short_filename_cache
      .get(absolute_path)
      .map(String::as_str)
  }

  fn insert_short_filename(&mut self, absolute_path: String, short_filename: String) {
    self
      .short_filename_cache
      .insert(absolute_path, short_filename);
  }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct DeclarationState {
  class_name_declarations: Vec<Ident>,
  function_name_declarations: Vec<Ident>,
}

impl DeclarationState {
  fn add_class_name_declaration(&mut self, ident: Ident) {
    if !self
      .class_name_declarations
      .iter()
      .any(|existing| existing.eq_ignore_span(&ident))
    {
      self.class_name_declarations.push(ident);
    }
  }

  fn add_function_name_declaration(&mut self, ident: Ident) {
    if !self
      .function_name_declarations
      .iter()
      .any(|existing| existing.eq_ignore_span(&ident))
    {
      self.function_name_declarations.push(ident);
    }
  }

  fn class_name_declarations(&self) -> &[Ident] {
    &self.class_name_declarations
  }

  fn function_name_declarations(&self) -> &[Ident] {
    &self.function_name_declarations
  }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct StyleInjectionState {
  inject_import_inserted: Option<InjectImportIdents>,
  metadata: IndexMap<String, IndexSet<MetaData>>,
  /// Transient dedup map for theme side-effect imports queued
  /// under `InsertionSlot::ThemeImports`. Tracks the imports that
  /// have already been queued so duplicates can be skipped. Lookups
  /// use `Vec::contains` via SWC's
  /// `PartialEq` on `ModuleItem`, which short-circuits on the
  /// first differing field — significantly cheaper in the typical
  /// case than a full `stable_hash` walk over the AST. Replaces
  /// the legacy `prepend_import_module_items` field's dual role
  /// (storage + dedup); the storage half is now in
  /// `pending_module_items`.
  queued_theme_imports: Vec<ModuleItem>,
  /// Transient dedup map for per-decl style metadata queued under
  /// `InsertionSlot::BeforeDecl(ast_hash)`. Same rationale as
  /// `queued_theme_imports`: keying by `ast_hash` keeps the
  /// per-bucket `Vec` small (typically 1–2 entries), so
  /// `Vec::contains` short-circuits early on PartialEq mismatches.
  /// Replaces the legacy `styles_to_inject` field's dual role.
  queued_decl_items: IndexMap<u128, Vec<ModuleItem>>,
}

impl StyleInjectionState {
  fn metadata(&self) -> &IndexMap<String, IndexSet<MetaData>> {
    &self.metadata
  }
}

#[derive(Clone, Debug)]
pub struct StateManager {
  pub(crate) plugin_pass: PluginPass,

  /// The source file the compiler parsed, when the host makes it available.
  /// Expression spans in the transformed AST resolve exactly against it.
  pub(crate) input_source_file: Option<Arc<SourceFile>>,
  /// Source map for the compiler's input code, mapping positions back to the
  /// original authored file when earlier tooling (e.g. macro loaders) already
  /// transformed the code.
  pub(crate) input_source_map: Option<Arc<swc_sourcemap::SourceMap>>,

  // Imports
  pub(crate) imports: ImportState,
  pub(crate) export_id: Option<String>,

  /// Sources of every import declaration in the module, in body order and
  /// including type-only imports. Captured by a one-time pre-scan at the
  /// start of the `Discover` cycle and consumed by
  /// `get_stylex_runtime_binding` to find an existing import source when
  /// injecting the `sx` runtime binding (SWC visitors have no parent
  /// pointers, so this pre-scanned list stands in for a walk of the module
  /// body).
  pub(crate) existing_import_sources: Vec<String>,

  /// Names of every identifier bound anywhere in the module (import locals,
  /// var/let/const declarators, function/class names, params). Captured by
  /// the same pre-scan and consumed by `get_stylex_runtime_binding` to test
  /// whether a name is already bound in the module.
  pub(crate) bound_names: FxHashSet<String>,

  /// For each name bound by a non-import declaration (var/let/const,
  /// function/class names, params), the source spans of the scopes in which
  /// it is bound. Consumed by `get_stylex_runtime_binding` to avoid reusing
  /// an imported `stylex` name that a local binding shadows. SWC visitors
  /// expose no scope chain, so the scope span is recorded during the pre-scan
  /// and `is_locally_rebound_at` performs the position-aware shadow check via
  /// span containment against the `sx` site.
  pub(crate) local_rebinding_scopes: FxHashMap<String, Vec<Span>>,

  pub(crate) module_source: ModuleSourceState,

  pub(crate) declarations_state: DeclarationState,
  pub(crate) declarations: Vec<VarDeclarator>,
  /// Position in [`Self::declarations`] of the declarator binding each named
  /// `Id`, so a reference resolves with one hash probe instead of a scan of
  /// every declarator in the module.
  ///
  /// Only `Pat::Ident` declarators appear, because that is the only shape
  /// `get_var_decl_from` ever matched. First writer wins, which is the same
  /// answer the `find` it replaces gave. Safe as an index rather than a map
  /// because `declarations` is append-only -- nothing removes, reorders or
  /// truncates it, and the in-place edits reach `init` rather than `name`, so a
  /// recorded position stays the position of the same binding.
  pub(crate) declaration_index: FxHashMap<Id, usize>,
  /// Bindings rebound after their declaration — an assignment, update,
  /// destructuring or loop target. The reference implementation's constant
  /// violations, probed as its own step of the reference-resolution chain.
  /// Keyed by full `Id` (`(Atom, SyntaxContext)`), so a write to a shadowing
  /// binding never deopts the outer one. Populated by the `Discover` pre-scan
  /// ([`ModuleBindingsCollector`]), in the same walk that fills
  /// [`Self::binding_mutations`].
  ///
  /// Behind an `Rc` for the reason the memoized module is. All four sets are
  /// filled once by the `Discover` pre-scan and read-only afterwards, and
  /// `StateManager` derives `Clone` -- which a dynamic style's callback performs
  /// once per invocation. `declared_bindings` alone holds every binding the
  /// module declares, so copying the four of them made a callback's cost scale
  /// with the size of the file it happens to sit in.
  pub(crate) binding_reassignments: Rc<FxHashSet<Id>>,
  /// Bindings whose referenced value is mutated in place — `obj.x = 1`,
  /// `arr.push(…)`, `delete obj.x`, `Object.assign(obj, …)`. The reference
  /// implementation's `isMutated`, probed as the step after
  /// [`Self::binding_reassignments`]. Same keying, same walk. Both kinds make
  /// the declaration initializer an unsound stand-in at the use site and both
  /// refuse with the same text, so the split buys a step-for-step mapping to
  /// the reference implementation rather than a difference in outcome.
  /// Shared rather than copied -- see [`Self::binding_reassignments`].
  pub(crate) binding_mutations: Rc<FxHashSet<Id>>,
  /// Bindings whose referenced value is written to further down a member chain
  /// than the reference implementation's `isMutated` looks: `obj.a.b = 1`
  /// records `obj` here where `obj.a = 1` records it in
  /// [`Self::binding_mutations`].
  ///
  /// Kept apart because the two answer different questions. A write upstream
  /// calls a mutation refuses the reference outright, as upstream refuses it. A
  /// deeper one refuses only where refusing protects something — a declarator
  /// whose initializer would otherwise be inlined at the use site, stale — since
  /// upstream folds these and this compiler deliberately does not.
  /// Shared rather than copied -- see [`Self::binding_reassignments`].
  pub(crate) binding_deep_mutations: Rc<FxHashSet<Id>>,
  /// Every **declared binding** in the module, keyed by full `Id` — the crate
  /// glossary defines the term and why the `Id` is what makes it scope-aware.
  /// Read through [`Self::declares_binding`].
  ///
  /// Populated by the `Discover` pre-scan ([`ModuleBindingsCollector`]) in
  /// either of its modes, in the same walk that fills the two write sets above.
  /// Shared rather than copied -- see [`Self::binding_reassignments`].
  pub(crate) declared_bindings: Rc<FxHashSet<Id>>,
  pub(crate) top_level_expressions: Vec<TopLevelExpression>,
  /// Spans of the calls that initialise a top-level declarator bound to a
  /// pattern rather than a name — `export const { foo } = stylex.create(…);`.
  /// [`Self::top_level_expressions`] is keyed by the exported name and so has
  /// no entry for them, but they are still program level, and a transform that
  /// hoists its result out of a nested position must not hoist here.
  pub(crate) pattern_bound_top_level_calls: FxHashSet<Span>,
  pub(crate) call_expressions: CallExpressionState,
  pub(crate) seen: FxHashMap<u128, Rc<SeenValue>>,
  /// How many expression levels the evaluator is currently inside.
  ///
  /// Lives here rather than on `EvaluationState` because the evaluator's
  /// confidence is forked -- a `&&` operand gets its own `EvaluationState`, and
  /// an object key is folded through a fresh one -- while the native stack is
  /// not. A counter that forked with the confidence would reset while the
  /// frames it is counting are still live, which is precisely the accounting
  /// the budget exists to keep honest.
  pub(crate) evaluation_depth: usize,
  /// Whether the fold in progress refused because it ran out of budget.
  ///
  /// Beside `evaluation_depth` for the same reason, and for one more: it guards
  /// `seen`, which lives here too. A depth refusal is the only refusal that
  /// depends on *where* a subtree sits rather than on what it says, and `seen`
  /// is keyed by a structural hash carrying no depth. So the frames unwinding
  /// out of one must not record their refusal -- a subtree that refused because
  /// it was reached too deep would otherwise answer for the same subtree written
  /// shallowly, and property order would decide the emitted CSS. Cleared when a
  /// new top-level fold begins, so the conservatism lasts exactly as long as the
  /// unwind that earned it.
  ///
  /// It has to last that long, and a narrower rule was tried and reverted. The
  /// obvious improvement is a counter each frame reads before recursing and
  /// compares after, so only the *ancestors* of a refusal stay quiet and a
  /// sibling evaluated later may memoize normally. That breaks, because
  /// `EvaluationState::confident` is sticky for the whole fold: once the depth
  /// refusal clears it, every later frame is unconfident whatever its own
  /// subtree did, and would record `resolved: false` against subtrees that fold
  /// perfectly well on their own. The flag is not being conservative about the
  /// unwind, it is compensating for a flag that never comes back.
  pub(crate) depth_refused: bool,
  /// Whether a guard is reading a name to decide whether it *could* fold an
  /// expression, rather than folding it.
  ///
  /// Set for exactly as long as such a read, and for the same reason
  /// [`Self::depth_refused`] exists: a refusal raised under it is not the
  /// subtree's answer, so recording it would answer for the next reader that
  /// asks the same question in earnest. The engine fold asks whether a receiver
  /// resolves to a value it can carry, and hands a name it cannot read back to
  /// the dispatch below -- which then evaluates the same name itself and has to
  /// find the refusal *that* evaluation earns, with the sentence naming the
  /// binding, rather than a memo entry saying only that something already said
  /// no.
  ///
  /// A value that resolved is still memoized. Only the refusal is withheld,
  /// because only the refusal is the speculation's own.
  pub(crate) speculating: bool,
  pub(crate) cache: CacheState,
  /// Maps a JSX spread expression to the JSX attributes that replace it.
  ///
  /// Bucketed by the [`stable_hash_unspanned`] structural hash of the spread
  /// expression so lookups stay span-insensitive and clone-free on the hot
  /// path, but each bucket stores the source expression alongside its
  /// replacement. Reads confirm a structural match with `eq_ignore_span` before
  /// substituting, so a 64-bit hash collision can never apply one spread's
  /// attributes to a structurally-different spread — the hash only narrows the
  /// candidate set; equality decides. Buckets are size-1 in the absence of a
  /// collision, so the extra check is a single comparison.
  pub(crate) jsx_spread_attr_exprs_map: FxHashMap<u128, Vec<(Expr, Vec<JSXAttrOrSpread>)>>,

  // `stylex.create` calls
  pub(crate) style_map: FxHashMap<String, Rc<StylesObjectMap>>,
  pub(crate) style_vars: FxHashMap<String, VarDeclarator>,

  /// Map of local identifier -> imported name for `@stylexjs/atoms` imports.
  /// The key includes `SyntaxContext`, so shadowed bindings with the same symbol
  /// text remain distinct after SWC's resolver pass. Namespace/default imports
  /// store `"*"`. Populated during `Discover` and consumed by the atoms pass.
  pub(crate) atom_imports: FxHashMap<Id, String>,

  /// Map of `stylex.create` variable name -> set of namespace names that are
  /// dynamic style functions (e.g. `opacity: (o) => ({ opacity: o })`). Used so
  /// an uncalled dynamic-style member access (`styles.opacity`) bails out to
  /// runtime instead of being inlined.
  pub(crate) dynamic_style_namespaces: FxHashMap<String, FxHashSet<String>>,

  // results of `stylex.create` calls that should be kept
  pub(crate) style_vars_to_keep: IndexSet<StyleVarsToKeep>,

  /// Reference graph from each top-level declarator to the set of
  /// declarators it directly references in its initializer / body.
  ///
  /// Built during `finalize_module` after the mark phase
  /// ([`build_decl_use_graph`]) and consumed by [`compute_live_set`] to
  /// run a forward mark-and-sweep that replaces the legacy count-based
  /// cleanup.
  pub(crate) decl_uses: FxHashMap<DeclId, FxHashSet<DeclId>>,

  /// Declarators that must always survive cleanup, regardless of
  /// in-graph references. Populated from non-decl top-level usages
  /// (function bodies, JSX, top-level expressions), exported declarators,
  /// and the mark phase's surviving member-expr accesses on style
  /// namespaces.
  pub(crate) roots: FxHashSet<DeclId>,

  /// Transient live-set computed by [`compute_live_set`] at the start
  /// of `finalize_module`. Each `DeclId` in the set must survive the
  /// sweep; declarators absent from `decl_uses` entirely also survive
  /// via the "not-in-graph ⇒ keep by default" fallback.
  pub(crate) live_set: FxHashSet<DeclId>,

  pub(crate) in_stylex_create: bool,

  pub(crate) options: StyleXStateOptions,
  pub(crate) injection: StyleInjectionState,

  /// Single ordered buffer of slot-tagged items waiting to be merged
  /// into the module body once consumer transforms complete.
  /// Producers append via `queue_insertion`;
  /// `flush_pending_insertions` drains this and splices each item
  /// into the right slot in the final body. Replaces the trio of
  /// accumulator vecs above plus the per-decl `styles_to_inject` map.
  pub(crate) pending_module_items: Vec<PendingInsertion>,

  pub(crate) other_injected_css_rules: InjectableStylesMap,
  pub(crate) top_imports: Vec<ImportDecl>,
  pub(crate) named_exports: FxHashSet<NamedExport>,

  pub cycle: TransformationCycle,
}

impl Default for StateManager {
  fn default() -> Self {
    StateManager::new(StyleXOptions::default())
  }
}

impl StateManager {
  pub fn new(stylex_options: StyleXOptions) -> Self {
    let options = StyleXStateOptions::from(stylex_options);

    Self {
      plugin_pass: PluginPass::default(),
      input_source_file: None,
      input_source_map: None,
      imports: ImportState::default(),
      existing_import_sources: vec![],
      bound_names: FxHashSet::default(),
      local_rebinding_scopes: FxHashMap::default(),
      style_map: FxHashMap::default(),
      style_vars: FxHashMap::default(),
      atom_imports: FxHashMap::default(),
      dynamic_style_namespaces: FxHashMap::default(),
      style_vars_to_keep: IndexSet::default(),
      decl_uses: FxHashMap::default(),
      roots: FxHashSet::default(),
      live_set: FxHashSet::default(),
      export_id: None,

      seen: FxHashMap::default(),
      evaluation_depth: 0,
      depth_refused: false,
      speculating: false,
      cache: CacheState::default(),
      module_source: ModuleSourceState::default(),

      top_imports: vec![],
      named_exports: FxHashSet::default(),

      declarations: vec![],
      declaration_index: FxHashMap::default(),
      declarations_state: DeclarationState::default(),
      binding_reassignments: Rc::default(),
      binding_mutations: Rc::default(),
      binding_deep_mutations: Rc::default(),
      declared_bindings: Rc::default(),
      top_level_expressions: vec![],
      pattern_bound_top_level_calls: FxHashSet::default(),
      call_expressions: CallExpressionState::default(),
      jsx_spread_attr_exprs_map: FxHashMap::default(),

      in_stylex_create: false,
      options,

      injection: StyleInjectionState::default(),
      pending_module_items: vec![],

      other_injected_css_rules: IndexMap::new(),

      cycle: TransformationCycle::Discover,
    }
  }

  pub(crate) fn set_plugin_pass(&mut self, plugin_pass: PluginPass) {
    self.plugin_pass = plugin_pass;
  }

  pub fn metadata(&self) -> &IndexMap<String, IndexSet<MetaData>> {
    self.injection.metadata()
  }

  pub fn add_call_expression(&mut self, call_expr: &CallExpr) {
    self.call_expressions.add_call_expression(call_expr);
  }

  pub(crate) fn is_member_call_callee(&self, member: &MemberExpr) -> bool {
    self.call_expressions.is_member_callee(member)
  }

  pub(crate) fn cached_span(&self, cache_key: u128) -> Option<Span> {
    self.cache.cached_span(cache_key)
  }

  pub(crate) fn insert_cached_span(&mut self, cache_key: u128, span: Span) {
    self.cache.insert_span(cache_key, span);
  }

  /// Records that the refusal raised on the expression behind `cache_key` is
  /// about the binding `name`, so its code frame is written against that
  /// binding's declaration rather than against the read.
  pub(crate) fn frame_declaration(&mut self, cache_key: u128, name: Atom) {
    self.cache.frame_declaration(cache_key, name);
  }

  /// Which kind of declaration binds `ident`, if either does.
  ///
  /// Asked of the state because the state owns both lists, and answered as a
  /// `Copy` verdict because the caller's next move is a refusal that borrows
  /// `self` mutably -- holding a `&[Ident]` across that is what used to be paid
  /// for by cloning both lists on the path every dynamic style's parameter
  /// takes.
  pub(crate) fn declared_as(&self, ident: &Ident) -> Option<DeclarationType> {
    if declares_ident(self.class_name_declarations(), ident) {
      return Some(DeclarationType::Class);
    }

    if declares_ident(self.function_name_declarations(), ident) {
      return Some(DeclarationType::Function);
    }

    None
  }

  /// Takes the binding sets the `Discover` pre-scan collected.
  ///
  /// One argument rather than four of the same type, for the reason
  /// [`ModuleBase`] is its own type: four positional `FxHashSet<Id>` can be
  /// transposed at a call site and still compile, and the answer would be a
  /// reference silently refused for the wrong reason. Named fields make the
  /// mistake visible where it is written, and the sets travel together anyway --
  /// the reference chain reads them as a group.
  pub(crate) fn adopt_binding_writes(&mut self, writes: BindingWrites) {
    self.binding_reassignments = Rc::new(writes.reassignments);
    self.binding_mutations = Rc::new(writes.mutations);
    self.binding_deep_mutations = Rc::new(writes.deep_mutations);
    self.declared_bindings = Rc::new(writes.declared);
  }

  /// Appends a declarator and records where it went.
  ///
  /// Every caller that grows [`Self::declarations`] goes through here, so the
  /// index beside it cannot fall behind: pushing to the field directly leaves
  /// the binding it added invisible to [`Self::declaration_of`], which is what
  /// the three test builders that did so discovered.
  pub(crate) fn push_declaration(&mut self, declarator: VarDeclarator) {
    if let Pat::Ident(binding) = &declarator.name {
      self
        .declaration_index
        .entry(binding.id.to_id())
        .or_insert(self.declarations.len());
    }

    self.declarations.push(declarator);
  }

  /// The declarator binding `ident`, by hash probe rather than by scan.
  pub(crate) fn declaration_of(&self, ident: &Ident) -> Option<&VarDeclarator> {
    let found = self
      .declaration_index
      .get(&ident.to_id())
      .and_then(|position| self.declarations.get(*position));

    // The index and the list have to agree, and only [`Self::push_declaration`]
    // keeps them agreeing. The field is `pub(crate)`, so nothing in the type
    // system stops a future caller pushing straight to it and leaving the
    // binding it added invisible here -- which is what three test builders did
    // the day the index was added. Checking the answer against the scan it
    // replaced turns that back into a loud test failure rather than a reference
    // that quietly stops resolving. Compiled out of release, where the scan is
    // the cost the index exists to avoid.
    debug_assert!(
      found.is_some()
        == self.declarations.iter().any(|declarator| matches!(
          &declarator.name,
          Pat::Ident(binding) if binding.id.eq_ignore_span(ident)
        )),
      "`declaration_index` disagrees with `declarations` about `{}`; something \
       grew the list without going through `push_declaration`",
      ident.sym
    );

    found
  }

  /// The binding a refusal on the expression behind `cache_key` is about, if one
  /// was recorded.
  pub(crate) fn framed_declaration(&self, cache_key: u128) -> Option<&Atom> {
    self.cache.framed_declaration(cache_key)
  }

  /// Whether any refusal recorded a declaration to frame.
  ///
  /// False for every build that refuses nothing, which is every successful one,
  /// so the annotation path answers the question without hashing an expression.
  pub(crate) fn has_framed_declarations(&self) -> bool {
    self.cache.has_framed_declarations()
  }

  /// The `file:line` annotation's short filename for `absolute_path`, if a
  /// previous style in this file already asked for it.
  ///
  /// Shortening a path means reading the package boundaries around it and around
  /// the working directory, and the debug path asks once per style namespace
  /// with the same path every time -- it was half of a `dev` transform.
  pub(crate) fn cached_short_filename(&self, absolute_path: &str) -> Option<&str> {
    self.cache.cached_short_filename(absolute_path)
  }

  pub(crate) fn insert_cached_short_filename(
    &mut self,
    absolute_path: String,
    short_filename: String,
  ) {
    self
      .cache
      .insert_short_filename(absolute_path, short_filename);
  }

  pub(crate) fn add_class_name_declaration(&mut self, ident: Ident) {
    self.declarations_state.add_class_name_declaration(ident);
  }

  pub(crate) fn add_function_name_declaration(&mut self, ident: Ident) {
    self.declarations_state.add_function_name_declaration(ident);
  }

  pub(crate) fn class_name_declarations(&self) -> &[Ident] {
    self.declarations_state.class_name_declarations()
  }

  pub(crate) fn function_name_declarations(&self) -> &[Ident] {
    self.declarations_state.function_name_declarations()
  }

  pub(crate) fn has_import_paths(&self) -> bool {
    self.imports.has_import_paths()
  }

  /// Whether any identifier named `name` is bound anywhere in the module.
  /// Backed by the [`StateManager::bound_names`] pre-scan.
  pub(crate) fn has_binding(&self, name: &str) -> bool {
    self.bound_names.contains(name)
  }

  /// Whether `name` is bound by a non-import declaration whose scope encloses
  /// `site` — i.e. a local binding that shadows an imported `stylex` name at
  /// that `sx` site. The check is position-aware: reuse is blocked only when
  /// the re-binding actually covers the `sx` site, not merely when it exists
  /// somewhere in the module. Backed by the
  /// [`StateManager::local_rebinding_scopes`] pre-scan.
  pub(crate) fn is_locally_rebound_at(&self, name: &str, site: Span) -> bool {
    self.local_rebinding_scopes.get(name).is_some_and(|scopes| {
      scopes
        .iter()
        .any(|scope| scope.lo <= site.lo && scope.hi >= site.hi)
    })
  }

  /// Whether `ident`'s binding is rebound anywhere in the module. See
  /// [`StateManager::binding_reassignments`].
  pub(crate) fn has_binding_reassignment(&self, ident: &Ident) -> bool {
    self.binding_reassignments.contains(&ident.to_id())
  }

  /// Whether the value `ident`'s binding references is mutated in place
  /// anywhere in the module. See [`StateManager::binding_mutations`].
  pub(crate) fn has_binding_mutation(&self, ident: &Ident) -> bool {
    self.binding_mutations.contains(&ident.to_id())
  }

  /// Whether the value `ident`'s binding references is written to further down a
  /// member chain than upstream's `isMutated` looks. See
  /// [`StateManager::binding_deep_mutations`].
  pub(crate) fn has_deep_binding_mutation(&self, ident: &Ident) -> bool {
    self.binding_deep_mutations.contains(&ident.to_id())
  }

  /// Whether this module declares a binding `ident` refers to. Asked of a
  /// reference, not of a name: the `Id` carries the syntax context, so this
  /// answers `false` for a global that some unrelated scope happens to bind the
  /// name of. See [`StateManager::declared_bindings`].
  pub(crate) fn declares_binding(&self, ident: &Ident) -> bool {
    self.declared_bindings.contains(&ident.to_id())
  }

  /// Seeds an empty replacement entry for a JSX spread expression seen during
  /// discovery. The expression is stored alongside its (initially empty)
  /// replacement so later lookups can confirm structural equality and never act
  /// on a bare hash collision. A structurally-identical spread already present
  /// in the bucket is not duplicated.
  pub(crate) fn seed_jsx_spread_expr(&mut self, expr: &Expr) {
    let key = stable_hash_unspanned(expr);
    let bucket = self.jsx_spread_attr_exprs_map.entry(key).or_default();

    if !bucket.iter().any(|(seen, _)| seen.eq_ignore_span(expr)) {
      bucket.push((expr.clone(), Vec::new()));
    }
  }

  /// Whether `call` was recorded as a JSX spread expression during discovery,
  /// confirmed by structural (`eq_ignore_span`) match, not just a hash hit.
  pub(crate) fn has_jsx_spread_call(&self, call: &CallExpr) -> bool {
    let key = stable_hash_unspanned_call(call);
    self
      .jsx_spread_attr_exprs_map
      .get(&key)
      .is_some_and(|bucket| {
        bucket
          .iter()
          .any(|(seen, _)| matches!(seen, Expr::Call(seen_call) if seen_call.eq_ignore_span(call)))
      })
  }

  /// Records the replacement JSX attributes for a `stylex.props(...)` call
  /// previously seeded as a JSX spread. Returns `true` when a matching entry
  /// (hash + structural match) was found and updated.
  pub(crate) fn set_jsx_spread_replacement(
    &mut self,
    call: &CallExpr,
    attrs: Vec<JSXAttrOrSpread>,
  ) -> bool {
    let key = stable_hash_unspanned_call(call);
    let Some(bucket) = self.jsx_spread_attr_exprs_map.get_mut(&key) else {
      return false;
    };

    for (seen, replacement) in bucket.iter_mut() {
      if matches!(seen, Expr::Call(seen_call) if seen_call.eq_ignore_span(call)) {
        *replacement = attrs;
        return true;
      }
    }

    false
  }

  /// Looks up the replacement JSX attributes recorded for a spread expression,
  /// confirming structural equality so a hash collision can never return a
  /// different expression's attributes. An entry that exists but has no
  /// recorded replacement yields `Some(&[])`.
  pub(crate) fn jsx_spread_replacement(&self, expr: &Expr) -> Option<&[JSXAttrOrSpread]> {
    let key = stable_hash_unspanned(expr);
    self
      .jsx_spread_attr_exprs_map
      .get(&key)?
      .iter()
      .find(|(seen, _)| seen.eq_ignore_span(expr))
      .map(|(_, replacement)| replacement.as_slice())
  }

  pub(crate) fn insert_import_path(&mut self, source_path: String) {
    self.imports.insert_import_path(source_path);
  }

  pub(crate) fn insert_stylex_import(&mut self, import_source: ImportSources) {
    self.imports.insert_stylex_import(import_source);
  }

  pub(crate) fn stylex_imports(&self) -> &IndexSet<ImportSources> {
    self.imports.stylex_imports()
  }

  pub(crate) fn is_regular_stylex_import(&self, ident_sym: &str) -> bool {
    self.stylex_imports().iter().any(|import_source| {
      matches!(import_source, ImportSources::Regular(regular) if regular.as_str() == ident_sym)
    })
  }

  pub(crate) fn is_style_var_ident(&self, ident: &Ident) -> bool {
    self.style_map.contains_key(ident.sym.as_ref())
      && self
        .style_vars
        .get(ident.sym.as_ref())
        .and_then(|decl| decl.name.as_ident())
        .is_some_and(|bind_ident| bind_ident.id.to_id() == ident.to_id())
  }

  /// Check if an import of the given kind contains the given symbol.
  pub(crate) fn has_stylex_api_import(&self, kind: ImportKind, sym: &Atom) -> bool {
    self.imports.has_stylex_api_import(kind, sym)
  }

  /// Insert a symbol into the import set for the given kind.
  pub(crate) fn insert_stylex_api_import(&mut self, kind: ImportKind, sym: Atom) {
    self.imports.insert_stylex_api_import(kind, sym);
  }

  /// Get the import set for the given kind, if any entries exist.
  pub(crate) fn get_stylex_api_import(&self, kind: ImportKind) -> Option<&AtomHashSet> {
    self.imports.get_stylex_api_import(kind)
  }

  /// Check if any import of the given kinds contains the given symbol.
  pub(crate) fn any_stylex_api_import_contains(&self, kinds: &[ImportKind], sym: &Atom) -> bool {
    kinds
      .iter()
      .any(|kind| self.has_stylex_api_import(*kind, sym))
  }

  pub(crate) fn is_stylex_namespace_import(&self, ident_sym: &str) -> bool {
    self
      .stylex_imports()
      .iter()
      .any(|import_source| match import_source {
        ImportSources::Regular(regular) => regular.as_str() == ident_sym,
        ImportSources::Named(named) => named.r#as.as_str() == ident_sym,
      })
  }

  pub(crate) fn is_stylex_import_for_kinds(&self, ident_sym: &str, kinds: &[ImportKind]) -> bool {
    if self.is_stylex_namespace_import(ident_sym) {
      return true;
    }

    self.any_stylex_api_import_contains(kinds, &Atom::from(ident_sym))
  }

  pub(crate) fn is_stylex_import_for_current_cycle(&self, ident_sym: &str) -> bool {
    match self.cycle {
      TransformationCycle::TransformProducers => {
        use ImportKind::*;
        self.is_stylex_import_for_kinds(
          ident_sym,
          &[
            Create,
            DefineVars,
            DefineVarsNested,
            DefineConsts,
            DefineConstsNested,
            DefineMarker,
            CreateTheme,
            CreateThemeNested,
            PositionTry,
            Keyframes,
            FirstThatWorks,
            Types,
            DefaultMarker,
            When,
            Conditional,
          ],
        )
      },
      TransformationCycle::TransformConsumers => {
        self.is_stylex_import_for_kinds(ident_sym, &[ImportKind::Attrs, ImportKind::Props])
      },
      _ => self.is_stylex_namespace_import(ident_sym),
    }
  }

  /// Applies the `env` configuration to the given identifiers and
  /// member_expressions maps. This is the Rust equivalent of the JavaScript
  /// `applyStylexEnv` method.
  pub(crate) fn apply_stylex_env(
    &self,
    identifiers: &mut super::types::FunctionMapIdentifiers,
    member_expressions: &mut super::types::FunctionMapMemberExpression,
  ) {
    if self.options.env.is_empty() {
      return;
    }

    let env = Rc::clone(&self.options.env);

    // For namespace imports (e.g., `import stylex from '@stylexjs/stylex'`),
    // add `env` to member_expressions so `stylex.env.x` resolves.
    for name in self.stylex_imports() {
      let member_expression = member_expressions.entry(name.clone()).or_default();
      member_expression.insert(
        STYLEX_ENV.into(),
        Box::new(super::functions::FunctionConfigType::EnvObject(env.clone())),
      );
    }

    // For direct env imports (e.g., `import { env } from '@stylexjs/stylex'`),
    // add the env object directly to identifiers.
    if let Some(env_imports) = self.get_stylex_api_import(ImportKind::Env) {
      for name in env_imports {
        identifiers.insert(
          name.clone(),
          Box::new(super::functions::FunctionConfigType::EnvObject(env.clone())),
        );
      }
    }
  }

  /// Gets the source code program if it exists and is not yet normalized
  pub(crate) fn get_seen_module_source_code(&self) -> Option<(&Module, &Option<String>)> {
    self.module_source.get_seen_module_source_code()
  }

  /// The memoized module's key span index, built on first use
  pub(crate) fn key_span_index(&self) -> Option<&KeySpanIndex> {
    self.module_source.key_span_index()
  }

  /// Where the module being transformed starts, for turning a compiled call's
  /// position into a file offset. See [`ModuleSourceState::input_module_base`].
  pub(crate) fn input_module_base(&self) -> Option<ModuleBase> {
    self.module_source.input_module_base
  }

  /// Records that base, once, as the module walk begins.
  pub(crate) fn set_input_module_base(&mut self, base: ModuleBase) {
    self.module_source.input_module_base = Some(base);
  }

  /// Sets the source code module (marks as not yet normalized)
  pub(crate) fn set_seen_module_source_code(
    &mut self,
    module: &Module,
    source_code: Option<String>,
  ) {
    self
      .module_source
      .set_seen_module_source_code(module, source_code);
  }

  pub fn import_as(&self, import: &str) -> Option<&str> {
    for import_source in &self.options.import_sources {
      match import_source {
        ImportSources::Regular(_) => {},
        ImportSources::Named(named) => {
          if named.from.eq(import) {
            return Some(named.r#as.as_str());
          }
        },
      }
    }

    None
  }

  pub fn is_import_source(&self, import: &str) -> bool {
    self
      .options
      .import_sources
      .iter()
      .any(|import_source| match import_source {
        ImportSources::Regular(regular) => regular.as_str() == import,
        ImportSources::Named(named) => named.from.as_str() == import,
      })
  }

  /// Borrowing iterator over configured import-source module paths, in order.
  /// Avoids allocating a `Vec<String>` for callers that only scan for a match.
  pub(crate) fn import_source_names(&self) -> impl Iterator<Item = &str> {
    self
      .options
      .import_sources
      .iter()
      .map(|import_source| match import_source {
        ImportSources::Regular(regular) => regular.as_str(),
        ImportSources::Named(named) => named.from.as_str(),
      })
  }

  /// Borrowing iterator over the local names of discovered value-level stylex
  /// namespace/default imports, in discovery order. Avoids allocating a
  /// `Vec<String>` for callers that only scan for a match.
  pub(crate) fn stylex_import_names(&self) -> impl Iterator<Item = &str> {
    self
      .stylex_imports()
      .iter()
      .map(|import_source| match import_source {
        ImportSources::Regular(regular) => regular.as_str(),
        ImportSources::Named(named) => named.r#as.as_str(),
      })
  }

  pub(crate) fn is_test(&self) -> bool {
    self.options.test
  }

  pub(crate) fn is_dev(&self) -> bool {
    self.options.dev
  }
  pub(crate) fn is_debug(&self) -> bool {
    self.options.debug
  }

  pub(crate) fn enable_inlined_conditional_merge(&self) -> bool {
    self.options.enable_inlined_conditional_merge
  }

  /// Provides the parsed input source file so span positions can be resolved
  /// without re-parsing. Set by hosts that own the parse (e.g. the NAPI
  /// compiler).
  pub fn set_input_source_file(&mut self, source_file: Arc<SourceFile>) {
    self.input_source_file = Some(source_file);
  }

  /// Provides the source map of the compiler's input code, enabling positions
  /// to be mapped back to the original authored file.
  pub fn set_input_source_map(&mut self, source_map: Arc<swc_sourcemap::SourceMap>) {
    self.input_source_map = Some(source_map);
  }

  pub(crate) fn get_short_filename(&self) -> String {
    extract_filename_from_path(&self.plugin_pass.filename)
  }
  pub(crate) fn get_filename(&self) -> &str {
    extract_path(&self.plugin_pass.filename)
  }
  pub(crate) fn get_filename_for_hashing(
    &self,
    package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
  ) -> Option<String> {
    let filename = self.get_filename();

    let unstable_module_resolution = &self.options.unstable_module_resolution;
    let theme_file_extension = unstable_module_resolution
      .theme_file_extension()
      .unwrap_or(".stylex");

    if filename.is_empty() {
      return None;
    }

    let consts_file_extension = format!("{}{}", theme_file_extension, CONSTS_FILE_EXTENSION);

    let is_theme_file = matches_file_suffix(theme_file_extension, filename);
    let is_consts_only_file = matches_file_suffix(&consts_file_extension, filename);

    if !is_theme_file && !is_consts_only_file {
      return None;
    }

    match unstable_module_resolution {
      CheckModuleResolution::Haste { .. } => {
        let filename = FileName::Real(filename.into());
        extract_filename_with_ext_from_path(&filename).map(|s| s.to_string())
      },
      CheckModuleResolution::CommonJs { .. } | CheckModuleResolution::CrossFileParsing { .. } => {
        Some(self.get_canonical_file_path(filename, package_json_seen))
      },
    }
  }
  pub(crate) fn get_package_name_and_path(
    filepath: &str,
    package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
  ) -> Option<(Option<String>, String)> {
    let folder = Path::new(filepath).parent()?;
    let package_json_path = find_closest_package_json_folder(Path::new(filepath));

    if let Some(package_json_path) = package_json_path {
      let (package_json, _) = get_package_json(&package_json_path, package_json_seen);
      // Try to read and parse package.json
      Some((
        package_json.name,
        package_json_path.to_string_lossy().into_owned(),
      ))
    } else {
      // Recursively check parent directory if not at root
      if folder.parent().is_some() && !folder.as_os_str().is_empty() {
        StateManager::get_package_name_and_path(
          folder.to_string_lossy().as_ref(),
          package_json_seen,
        )
      } else {
        None
      }
    }
  }
  pub(crate) fn get_canonical_file_path(
    &self,
    file_path: &str,
    package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
  ) -> String {
    if let Some(pkg_info) = StateManager::get_package_name_and_path(file_path, package_json_seen) {
      let (package_name, package_dir) = pkg_info;

      let package_dir_path = Path::new(&package_dir);
      let file_path = Path::new(file_path);
      let relative_package_path = relative_path(file_path, package_dir_path);

      if let Some(package_dir) = relative_package_path.to_str() {
        // Normalize path separators to forward slashes for consistency across platforms
        let normalized_path = package_dir.replace('\\', "/");
        return format!(
          "{}:{}",
          package_name.unwrap_or_else(|| "_unknown_name_".to_string()),
          normalized_path
        );
      }
    }

    if let Some(root_dir) = self.options.unstable_module_resolution.root_dir() {
      let file_path = Path::new(file_path);
      let root_dir = Path::new(root_dir);

      if let Some(rel_path) = relative_path(file_path, root_dir).to_str() {
        // Normalize path separators to forward slashes for consistency across platforms
        let normalized_path = rel_path.replace('\\', "/");
        return normalized_path;
      }
    };

    let file_name = Path::new(file_path)
      .file_name()
      .unwrap_or_default()
      .to_string_lossy();

    format!("_unknown_path_:{}", file_name)
  }

  pub(crate) fn import_path_resolver(
    &self,
    import_path: &str,
    package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
  ) -> ImportPathResolution {
    let source_file_path = self.get_filename();

    if source_file_path.is_empty() {
      return ImportPathResolution::Unresolved;
    }

    let theme_file_extension = self
      .options
      .unstable_module_resolution
      .theme_file_extension()
      .unwrap_or(".stylex");

    let consts_file_extension = format!("{}{}", theme_file_extension, CONSTS_FILE_EXTENSION);

    let is_theme_file = matches_file_suffix(theme_file_extension, import_path);
    let is_consts_only_file = matches_file_suffix(&consts_file_extension, import_path);

    let is_valid_transformed_vars_file =
      matches_file_suffix(TRANSFORMED_VARS_FILE_EXTENSION, import_path);

    if !is_theme_file && !is_valid_transformed_vars_file && !is_consts_only_file {
      return ImportPathResolution::Unresolved;
    }

    match &self.options.unstable_module_resolution {
      CheckModuleResolution::CommonJs { .. } => {
        let filename = self.get_filename();

        let (_, root_dir) = StateManager::get_package_name_and_path(filename, package_json_seen)
          .unwrap_or_else(|| stylex_panic!("Cannot get package name and path for: {}", filename));

        let aliases = self.options.aliases.as_ref().cloned().unwrap_or_default();

        let resolved_file_path = match file_path_resolver(
          import_path,
          source_file_path,
          &root_dir,
          &aliases,
          self.options.unstable_module_resolution.root_dir(),
          package_json_seen,
        ) {
          Ok(resolved_file_path) => resolved_file_path,
          Err(err) => {
            debug!("Could not resolve import path {}: {}", import_path, err);
            return ImportPathResolution::Unresolved;
          },
        };

        debug!("Resolved import path: {}", resolved_file_path);

        let resolved_file_path =
          self.get_canonical_file_path(&resolved_file_path, package_json_seen);

        ImportPathResolution::Resolved {
          path: resolved_file_path,
        }
      },
      CheckModuleResolution::Haste { .. } => ImportPathResolution::Resolved {
        path: add_file_extension(import_path, source_file_path),
      },
      CheckModuleResolution::CrossFileParsing { .. } => {
        stylex_unimplemented!("This module resolution strategy is not yet supported.")
      },
    }
  }

  pub(crate) fn find_top_level_expr(
    &self,
    call: &CallExpr,
    extended_predicate_fn: impl Fn(&TopLevelExpression) -> bool,
    kind: Option<TopLevelExpressionKind>,
  ) -> Option<&TopLevelExpression> {
    self.top_level_expressions.iter().find(|tpe| {
      kind.is_none_or(|kind| tpe.0 == kind)
        && (matches!(tpe.1, Expr::Call(ref c) if c.eq_ignore_span(call))
          || extended_predicate_fn(tpe))
    })
  }

  /// Find the top level expression recorded from *this* call node, matched by
  /// span.
  ///
  /// [`Self::find_top_level_expr`] compares span-insensitively, so calls that
  /// differ only in source position all resolve to the first of them — every
  /// `defineMarker()` in a module is the same expression. Spans are unique per
  /// position and the discovery pass records them untouched, so they are what
  /// pins an entry to the call it was recorded from.
  ///
  /// A dummy span identifies nothing, so a span-less call never matches — a
  /// synthesized call is not something the discovery pass can have recorded,
  /// and matching one against a recorded entry would resurrect the very
  /// conflation this lookup exists to avoid.
  pub(crate) fn find_top_level_expr_by_span(&self, call: &CallExpr) -> Option<&TopLevelExpression> {
    if call.span.is_dummy() {
      return None;
    }

    self
      .top_level_expressions
      .iter()
      .find(|TopLevelExpression(_, expr, _)| {
        matches!(expr, Expr::Call(recorded_call) if recorded_call.span == call.span)
      })
  }

  /// Position of the declarator initialised by *this* call node, matched by
  /// span for the reason given on [`Self::find_top_level_expr_by_span`].
  ///
  /// Returns the position rather than a reference so callers can both read the
  /// declarator and later rewrite its initializer without searching twice. The
  /// position stays valid only while `declarations` is not pushed to or
  /// reordered — `apply_dynamic_style_functions` does push, so a caller that
  /// holds one across a transform has to check that its own path cannot reach
  /// there.
  ///
  /// Span-less calls never match, for the reason given on
  /// [`Self::find_top_level_expr_by_span`].
  pub(crate) fn find_call_declaration_index_by_span(&self, call: &CallExpr) -> Option<usize> {
    if call.span.is_dummy() {
      return None;
    }

    self.declarations.iter().position(|decl| {
      decl.init.as_ref().is_some_and(
        |init| matches!(**init, Expr::Call(ref recorded_call) if recorded_call.span == call.span),
      )
    })
  }

  /// The declarator initialised by *this* call node, for callers that only read
  /// it. See [`Self::find_call_declaration_index_by_span`], which this defers
  /// to, for how the match is made.
  pub(crate) fn find_call_declaration_by_span(&self, call: &CallExpr) -> Option<&VarDeclarator> {
    self
      .declarations
      .get(self.find_call_declaration_index_by_span(call)?)
  }

  pub(crate) fn find_call_declaration(&self, call: &CallExpr) -> Option<&VarDeclarator> {
    self.declarations.iter().find(|decl| {
      decl
        .init
        .as_ref()
        .is_some_and(|expr| matches!(**expr, Expr::Call(ref c) if c.eq_ignore_span(call)))
    })
  }

  pub(crate) fn register_styles(
    &mut self,
    call: &CallExpr,
    style: &InjectableStylesMap,
    ast: &Expr,
    fallback_ast: Option<&Expr>,
  ) {
    // Early return if there are no styles to process
    if style.is_empty() {
      return;
    }

    let metadatas = MetaData::convert_from_injected_styles_map(style);
    if metadatas.is_empty() {
      return;
    }

    let needs_runtime_injection = style.values().any(|value| {
      matches!(
        value.as_ref(),
        InjectableStyleKind::Regular(_) | InjectableStyleKind::Const(_)
      )
    });

    let inject_var_ident = if needs_runtime_injection {
      Some(self.setup_injection_imports())
    } else {
      None
    };

    for metadata in metadatas {
      self.add_style(&metadata);

      if let Some(ref inject_var_ident) = inject_var_ident {
        self.add_style_to_inject(&metadata, inject_var_ident, ast, fallback_ast);
      }
    }

    // Update all references to this call expression with the new AST
    self.update_references(call, ast, fallback_ast);
  }

  /// Registers injected styles produced by the atoms transform.
  ///
  /// Atom styles are compiled inline into `stylex.props(...)` arguments that
  /// are consumed before pending insertions are flushed, so there is no
  /// surviving declarator to anchor `_inject2(...)` calls to. Runtime injection
  /// calls are queued to [`InsertionSlot::AfterImports`] — right after the
  /// import block and before the module body that uses them.
  ///
  /// [`register_styles`]: StateManager::register_styles
  pub(crate) fn register_atom_styles(&mut self, style: &InjectableStylesMap) {
    if style.is_empty() {
      return;
    }

    let metadatas = MetaData::convert_from_injected_styles_map(style);
    if metadatas.is_empty() {
      return;
    }

    let inject_var_ident = if self.options.runtime_injection.is_some() {
      Some(self.setup_injection_imports())
    } else {
      None
    };

    for metadata in metadatas {
      self.add_style(&metadata);

      if let Some(ref inject_var_ident) = inject_var_ident {
        let item = build_atom_inject_item(&metadata, inject_var_ident);
        self.queue_insertion(InsertionSlot::AfterImports, item);
      }
    }
  }

  fn setup_injection_imports(&mut self) -> Ident {
    // Once the runtime helpers have been queued, the var-ident is
    // cached on `inject_import_inserted` — return it on subsequent
    // calls without re-queueing.
    if let Some(idents) = self.injection.inject_import_inserted.as_ref() {
      return idents.var.clone();
    }
    let mut uid_generator = UidGenerator::new("inject", CounterMode::Local);

    let runtime_injection = self
      .options
      .runtime_injection
      .as_ref()
      .cloned()
      .unwrap_or(RuntimeInjectionState::Boolean(true));

    let (inject_module_ident, inject_var_ident) = match self.injection.inject_import_inserted.take()
    {
      Some(idents) => (idents.module, idents.var),
      None => {
        let module_ident = uid_generator.generate_ident();

        let var_ident = match &runtime_injection {
          RuntimeInjectionState::Regular(_) | RuntimeInjectionState::Boolean(_) => {
            uid_generator.generate_ident()
          },
          RuntimeInjectionState::Named(NamedImportSource { r#as, .. }) => {
            uid_generator = UidGenerator::new(r#as, CounterMode::Local);
            uid_generator.generate_ident()
          },
        };

        let idents = InjectImportIdents {
          module: module_ident,
          var: var_ident,
        };
        self.injection.inject_import_inserted = Some(idents.clone());

        (idents.module, idents.var)
      },
    };

    let module_items = match &runtime_injection {
      RuntimeInjectionState::Boolean(_) => vec![
        add_inject_default_import_expression(&inject_module_ident, None),
        add_inject_var_decl_expression(&inject_var_ident, &inject_module_ident),
      ],
      RuntimeInjectionState::Regular(name) => vec![
        add_inject_default_import_expression(&inject_module_ident, Some(name)),
        add_inject_var_decl_expression(&inject_var_ident, &inject_module_ident),
      ],
      RuntimeInjectionState::Named(_) => vec![
        add_inject_named_import_expression(&inject_module_ident, &inject_var_ident),
        add_inject_var_decl_expression(&inject_var_ident, &inject_module_ident),
      ],
    };

    // Each call queues into the BeforeImports slot. The early
    // return above guards against re-queueing on the second call.
    for item in module_items {
      self.queue_insertion(InsertionSlot::BeforeImports, item);
    }
    inject_var_ident
  }

  fn update_references(&mut self, call: &CallExpr, ast: &Expr, _fallback_ast: Option<&Expr>) {
    if let Some(item) = self.declarations.iter_mut().find(|decl| {
      decl.init.as_ref().is_some_and(|expr| {
        matches!(**expr, Expr::Call(ref existing_call) if existing_call.eq_ignore_span(call))
      })
    }) {
      item.init = Some(Box::new(ast.clone()));
    }

    if let Some((_, item)) = self.style_vars.iter_mut().find(|(_, decl)| {
      decl.init.as_ref().is_some_and(|expr| {
        matches!(**expr, Expr::Call(ref existing_call) if existing_call.eq_ignore_span(call))
      })
    }) {
      item.init = Some(Box::new(ast.clone()));
    }

    if let Some(top_level_expr) = self.top_level_expressions.iter_mut().find(
      |TopLevelExpression(_, expr, _)| matches!(expr, Expr::Call(c) if c.eq_ignore_span(call)),
    ) {
      top_level_expr.1 = ast.clone();
    }

    self.call_expressions.replace_call_expression(call, ast);
  }

  fn add_style(&mut self, metadata: &MetaData) {
    let var_name = "stylex";
    let value = self
      .injection
      .metadata
      .entry(var_name.to_string())
      .or_default();

    if !value.contains(metadata) {
      value.insert(metadata.clone());
    }
  }

  fn add_style_to_inject(
    &mut self,
    metadata: &MetaData,
    inject_var_ident: &Ident,
    ast: &Expr,
    fallback_ast: Option<&Expr>,
  ) {
    let priority = metadata.get_priority();
    let css_ltr = metadata.get_css();
    let css_rtl = metadata.get_css_rtl();
    let const_key = metadata.get_const_key();
    let const_value = metadata.get_const_value();

    let mut stylex_inject_args = vec![
      create_string_key_value_prop("ltr", css_ltr),
      create_key_value_prop("priority", create_number_expr(*priority)),
    ];

    if let Some(const_key) = const_key
      && let Some(const_value) = const_value
    {
      let const_value_expr = match const_value.parse::<f64>() {
        Ok(value) => create_number_expr_or_spread(value),
        Err(_) => create_string_expr_or_spread(const_value),
      };

      stylex_inject_args.push(create_string_key_value_prop("constKey", const_key));
      stylex_inject_args.push(create_key_value_prop("constVal", *const_value_expr.expr));
    }

    if let Some(rtl) = css_rtl {
      stylex_inject_args.push(create_string_key_value_prop("rtl", rtl));
    }

    let stylex_inject_obj = create_object_expression(stylex_inject_args);

    let stylex_call_expr = create_call_expr(
      Expr::Ident(inject_var_ident.clone()),
      vec![create_expr_or_spread(stylex_inject_obj)],
    );

    let stylex_call = Expr::Call(stylex_call_expr);

    let module = ModuleItem::Stmt(Stmt::Expr(ExprStmt {
      span: DUMMY_SP,
      expr: Box::new(stylex_call),
    }));

    let ast_hash = stable_hash_unspanned(ast);
    let normalized_module = module;

    // Per-decl dedup: keying by `ast_hash` keeps the per-bucket
    // `Vec` small (typically 1–2 entries), so the span-insensitive
    // `eq_ignore_span` scan short-circuits cheaply rather than walking
    // a full `stable_hash` over the AST. Items go in the bucket
    // (owned) while a clone goes into the pending buffer; the
    // bucket clone is then reused by the fallback path.
    let bucket = self
      .injection
      .queued_decl_items
      .entry(ast_hash)
      .or_default();
    let needs_primary_queue = !bucket
      .iter()
      .any(|item| item.eq_ignore_span(&normalized_module));
    if needs_primary_queue {
      bucket.push(normalized_module.clone());
    }

    if let Some(fallback_ast) = fallback_ast {
      let fallback_ast_hash = stable_hash_unspanned(fallback_ast);
      let fallback_bucket = self
        .injection
        .queued_decl_items
        .entry(fallback_ast_hash)
        .or_default();
      let needs_fallback_queue = !fallback_bucket
        .iter()
        .any(|item| item.eq_ignore_span(&normalized_module));
      if needs_fallback_queue {
        fallback_bucket.push(normalized_module.clone());
      }

      if needs_primary_queue {
        self.queue_insertion(
          InsertionSlot::BeforeDecl(ast_hash),
          normalized_module.clone(),
        );
      }
      if needs_fallback_queue {
        self.queue_insertion(
          InsertionSlot::BeforeDecl(fallback_ast_hash),
          normalized_module,
        );
      }
    } else if needs_primary_queue {
      self.queue_insertion(InsertionSlot::BeforeDecl(ast_hash), normalized_module);
    }
  }

  pub(crate) fn get_treeshake_compensation(&self) -> bool {
    self.options.treeshake_compensation
  }

  /// Queue a `ModuleItem` for placement in the final module body.
  /// `slot` decides where the linear merge in
  /// [`flush_pending_insertions`] will splice the item.
  pub(crate) fn queue_insertion(&mut self, slot: InsertionSlot, item: ModuleItem) {
    self
      .pending_module_items
      .push(PendingInsertion { slot, item });
  }

  /// Queue a `ThemeImports` item, deduped against earlier queues
  /// of the same import. Replaces the legacy
  /// `prepend_import_module_items.contains` gate that the theme
  /// side-effect import path used. Each evaluation gets its own
  /// `EvaluationState.added_imports` set; this dedup lives on the
  /// StateManager so it works across evaluations.
  ///
  /// `Vec::contains` short-circuits on PartialEq mismatch, which
  /// is significantly cheaper than a full `stable_hash` walk over
  /// the AST in the typical case.
  /// Whether a refusal raised right now is the evaluated subtree's own answer,
  /// and so worth recording against it in the memo.
  ///
  /// Two things can make it somebody else's, and they are asked together because
  /// every caller wants the one answer rather than the two reasons:
  ///
  /// - [`Self::depth_refused`] -- the refusal is about *where* the subtree sat,
  ///   which the memo key cannot say, so it would answer for the same subtree
  ///   written shallowly.
  /// - [`Self::speculating`] -- the refusal is about nothing yet, because a guard
  ///   was reading a name to decide whether it *could* fold rather than folding.
  ///
  /// Named once here so a third reason is a third arm of one question rather than
  /// a third condition every reader has to remember to add.
  pub(crate) fn owns_its_refusals(&self) -> bool {
    !self.depth_refused && !self.speculating
  }

  pub(crate) fn queue_theme_import_if_absent(&mut self, item: ModuleItem) {
    if !self.injection.queued_theme_imports.contains(&item) {
      self.injection.queued_theme_imports.push(item.clone());
      self.queue_insertion(InsertionSlot::ThemeImports, item);
    }
  }
}

/// Read-only visitor used by [`build_decl_use_graph`] to collect every
/// `Ident` referenced inside a top-level declarator's initializer (or
/// the body of a non-`VarDecl` top-level item).
///
/// Skips identifier-shaped property keys (`{foo: …}`) and member props
/// (`obj.foo`) so that property names do not pollute the reference set —
/// only true variable references are recorded. Each captured ident is
/// stored as its full `Id` (`(Atom, SyntaxContext)`) so resolver-aware
/// shadowing is preserved.
#[derive(Default)]
struct CollectIdentsVisitor {
  idents: FxHashSet<DeclId>,
}

impl Visit for CollectIdentsVisitor {
  fn visit_ident(&mut self, ident: &Ident) {
    self.idents.insert(ident.to_id());
  }

  fn visit_member_prop(&mut self, member_prop: &MemberProp) {
    if !member_prop.is_ident() {
      member_prop.visit_children_with(self);
    }
  }

  fn visit_prop_name(&mut self, prop_name: &PropName) {
    if !prop_name.is_ident() {
      prop_name.visit_children_with(self);
    }
  }
}

/// Build the reference graph used by the new cleanup pass.
///
/// Walks `module.body` once. Top-level `VarDeclarator`s with a simple
/// `Pat::Ident` binding contribute an edge from their `DeclId` to every
/// `DeclId` referenced in their initializer. Top-level items that are
/// not declarators (function decls, class decls, expression statements,
/// non-`VarDecl` exports) are treated as observation points: every
/// `DeclId` they reference is added to `state.roots` directly.
///
/// The graph is consumed by [`compute_live_set`] to compute reachability
/// from `roots` and decide which declarators survive the sweep.
pub(crate) fn build_decl_use_graph(module: &Module, state: &mut StateManager) {
  for item in &module.body {
    match item {
      ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) => {
        for decl in &var_decl.decls {
          collect_decl_uses(state, decl);
        }
      },
      ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => match &export_decl.decl {
        Decl::Var(var_decl) => {
          for decl in &var_decl.decls {
            collect_decl_uses(state, decl);
          }
        },
        other_decl => {
          let mut visitor = CollectIdentsVisitor::default();
          other_decl.visit_with(&mut visitor);
          state.roots.extend(visitor.idents);
        },
      },
      _ => {
        let mut visitor = CollectIdentsVisitor::default();
        item.visit_with(&mut visitor);
        state.roots.extend(visitor.idents);
      },
    }
  }
}

/// Compute the transitive closure of `state.roots` over `state.decl_uses`.
///
/// Returns the set of `DeclId`s that are reachable from any root via the
/// reference graph. The sweep keeps every declarator whose `DeclId` is
/// either in the returned set or absent from `state.decl_uses` entirely
/// (the "not-in-graph ⇒ keep by default" fallback).
///
/// Iterative breadth-first traversal with a worklist; cycles and
/// self-references terminate naturally because already-marked nodes are
/// not revisited.
pub(crate) fn compute_live_set(state: &StateManager) -> FxHashSet<DeclId> {
  let mut live: FxHashSet<DeclId> = FxHashSet::default();
  let mut worklist: Vec<DeclId> = state.roots.iter().cloned().collect();

  while let Some(node) = worklist.pop() {
    if !live.insert(node.clone()) {
      continue;
    }
    if let Some(targets) = state.decl_uses.get(&node) {
      for target in targets {
        worklist.push(target.clone());
      }
    }
  }

  live
}

fn collect_decl_uses(state: &mut StateManager, decl: &VarDeclarator) {
  let mut visitor = CollectIdentsVisitor::default();
  if let Some(init) = &decl.init {
    init.visit_with(&mut visitor);
  }

  if let Pat::Ident(bind_ident) = &decl.name {
    let decl_id: DeclId = bind_ident.id.to_id();
    state
      .decl_uses
      .entry(decl_id)
      .or_default()
      .extend(visitor.idents);
  } else {
    // Non-`Pat::Ident` declarators (destructuring, etc.) are not tracked
    // by the graph; they fall through to the sweep's "absent ⇒ keep"
    // fallback. Treat their referenced idents as roots so anything they
    // depend on is preserved.
    state.roots.extend(visitor.idents);
  }
}

/// Visitor used by [`mark_style_vars_to_keep`] to populate
/// `state.style_vars_to_keep` from the surviving member-expression accesses
/// on style namespaces and to materialize any JSX-spread replacements
/// recorded during the discovery phase.
///
/// Replaces what the legacy `TransformationCycle::PreCleaning` arms in
/// `visit_mut_member_expr.rs` and `visit_mut_jsx_attr_or_spread.rs` did,
/// so the finalize phase can collapse to a single sweep cycle.
struct MarkStyleVarsVisitor<'a> {
  state: &'a mut StateManager,
}

impl VisitMut for MarkStyleVarsVisitor<'_> {
  fn visit_mut_call_expr(&mut self, call: &mut CallExpr) {
    if is_stylex_consumer_call(call, self.state) {
      return;
    }

    call.visit_mut_children_with(self);
  }

  fn visit_mut_member_expr(&mut self, member_expression: &mut MemberExpr) {
    if let Expr::Ident(ident) = member_expression.obj.as_ref()
      && self.state.is_style_var_ident(ident)
      && let Some(namespace_name) = member_namespace_name(&member_expression.prop)
    {
      let decl_id = ident.to_id();
      self.state.roots.insert(decl_id.clone());
      self.state.style_vars_to_keep.insert(StyleVarsToKeep(
        decl_id,
        namespace_name,
        NonNullProps::True,
      ));
    }

    member_expression.visit_mut_children_with(self);
  }

  fn visit_mut_jsx_attr_or_spreads(&mut self, jsx_attrs: &mut Vec<JSXAttrOrSpread>) {
    let mut result: Vec<JSXAttrOrSpread> = jsx_attrs
      .iter()
      .flat_map(|jsx_attr| match jsx_attr {
        JSXAttrOrSpread::SpreadElement(spread) => {
          match self.state.jsx_spread_replacement(spread.expr.as_ref()) {
            // A recorded spread with replacement attrs — substitute them.
            Some(updated_exprs) if !updated_exprs.is_empty() => updated_exprs.to_vec(),
            // A recorded spread with no replacement attrs — keep the original.
            Some(_) => vec![jsx_attr.clone()],
            // Not a recorded spread — keep the original spread element.
            None => vec![create_jsx_spread_attr(*spread.expr.clone())],
          }
        },
        JSXAttrOrSpread::JSXAttr(attr) => vec![create_jsx_attr_or_spread(attr.clone())],
      })
      .collect();

    result.visit_mut_children_with(self);
    *jsx_attrs = result;
  }
}

fn is_stylex_consumer_call(call: &CallExpr, state: &StateManager) -> bool {
  is_props_call(call, state)
    || is_attrs_call(call, state)
    || call
      .callee
      .as_expr()
      .and_then(|callee| callee.as_ident())
      .is_some_and(|ident| state.is_regular_stylex_import(&ident.sym))
}

fn member_namespace_name(member_prop: &MemberProp) -> Option<NonNullProp> {
  if let Some(namespace_name) = namespace_name_from_member_prop(member_prop) {
    Some(NonNullProp::Atom(namespace_name))
  } else if member_prop.is_computed() {
    Some(NonNullProp::True)
  } else {
    None
  }
}

/// Walk the module to populate `state.style_vars_to_keep` from surviving
/// member-expr accesses on style namespaces, and to apply any deferred
/// JSX-spread replacements collected during discovery.
///
/// This is the "mark" step of the finalize phase. The "sweep" step that
/// actually deletes unused declarations runs afterwards under
/// `TransformationCycle::Finalize`.
pub(crate) fn mark_style_vars_to_keep(module: &mut Module, state: &mut StateManager) {
  let mut visitor = MarkStyleVarsVisitor { state };
  module.visit_mut_with(&mut visitor);
}

/// Drain `state.pending_module_items` and splice every queued
/// [`PendingInsertion`] into `module_body` according to its slot.
///
/// Output ordering mirrors the legacy split between
/// `inject_runtime_styles` and the in-walk hoisted-items splice in
/// `visit_mut_module_items::TransformConsumers`:
///
/// 1. The leading directive prologue (a string-literal `ExprStmt` at position
///    0), if present, stays at position 0.
/// 2. `BeforeImports` items follow the directive — runtime helpers matching the
///    legacy `prepend_include_module_items` placement.
/// 3. `ThemeImports` items follow the runtime helpers, still ahead of the
///    existing import block — matching the legacy `prepend_import_module_items`
///    placement.
/// 4. The existing import block follows.
/// 5. `AfterImports` items follow the import block — matching the legacy
///    in-walk splice that placed `hoisted_module_items` after imports during
///    the consumer walk.
/// 6. The remainder of the body follows. For each item, every relevant
///    initializer is hashed and any matching `BeforeDecl` metadata is spliced
///    before it.
///
/// `runtime_injection` matches the legacy gate on
/// `options.runtime_injection.is_some()`: when `false`, the runtime
/// helpers (`BeforeImports`), theme side-effect imports
/// (`ThemeImports`), and per-decl metadata (`BeforeDecl`) are
/// dropped on the floor — exactly as the legacy
/// `inject_runtime_styles` was simply not invoked.
/// `AfterImports` always emits — the legacy in-walk hoisted splice
/// ran regardless of the option.
pub(crate) fn flush_pending_insertions(
  state: &mut StateManager,
  module_body: &mut Vec<ModuleItem>,
  runtime_injection: bool,
) {
  if state.pending_module_items.is_empty() {
    return;
  }

  let pending = std::mem::take(&mut state.pending_module_items);

  let mut prepend_imports: Vec<ModuleItem> = Vec::new();
  let mut before_imports: Vec<ModuleItem> = Vec::new();
  let mut theme_imports: Vec<ModuleItem> = Vec::new();
  let mut after_imports: Vec<ModuleItem> = Vec::new();
  let mut before_decl: FxHashMap<u128, Vec<ModuleItem>> = FxHashMap::default();

  for PendingInsertion { slot, item } in pending {
    match slot {
      InsertionSlot::PrependImport => prepend_imports.push(item),
      InsertionSlot::BeforeImports => {
        if runtime_injection {
          before_imports.push(item);
        }
      },
      InsertionSlot::ThemeImports => {
        if runtime_injection {
          theme_imports.push(item);
        }
      },
      InsertionSlot::AfterImports => after_imports.push(item),
      InsertionSlot::BeforeDecl(hash) => {
        if runtime_injection {
          before_decl.entry(hash).or_default().push(item);
        }
      },
    }
  }

  // Step 1: replicate the legacy in-walk splice that placed
  // hoisted items between the import block and the rest of the
  // body. Doing it here keeps the BeforeDecl iteration in step 4
  // walking the same shape the legacy `inject_runtime_styles` saw.
  let original = std::mem::take(module_body);
  let body_with_after_imports = if after_imports.is_empty() {
    original
  } else {
    let directive_end = original
      .iter()
      .take_while(|item| {
        item
          .as_stmt()
          .and_then(|stmt| stmt.as_expr())
          .is_some_and(|expr_stmt| matches!(expr_stmt.expr.as_lit(), Some(Lit::Str(_))))
      })
      .count();
    let import_end = original
      .iter()
      .enumerate()
      .skip(directive_end)
      .find(|(_, item)| !matches!(item, ModuleItem::ModuleDecl(ModuleDecl::Import(_))))
      .map(|(idx, _)| idx)
      .unwrap_or(original.len());
    let mut merged = Vec::with_capacity(original.len() + after_imports.len());
    let mut iter = original.into_iter();
    for _ in 0..import_end {
      if let Some(item) = iter.next() {
        merged.push(item);
      }
    }
    merged.extend(after_imports);
    merged.extend(iter);
    merged
  };

  // Step 2: peel a leading directive prologue so BeforeImports
  // items splice in *after* it (matching legacy `inject_runtime_styles`).
  let mut iter = body_with_after_imports.into_iter().peekable();
  let mut result: Vec<ModuleItem> = Vec::new();

  let leading_directive = iter
    .peek()
    .and_then(|item| item.as_stmt())
    .and_then(|stmt| stmt.as_expr())
    .is_some_and(|expr_stmt| matches!(expr_stmt.expr.as_lit(), Some(Lit::Str(_))));

  if leading_directive && let Some(directive) = iter.next() {
    result.push(directive);
  }

  // Step 3: injected `sx` runtime imports go first (ahead of the runtime
  // helpers and existing imports), then theme side-effect imports go
  // directly after the runtime helpers — preserving the legacy
  // `prepend_include` -> `prepend_import` -> existing-imports order
  // regardless of which producer queued first.
  result.extend(prepend_imports);
  result.extend(before_imports);
  result.extend(theme_imports);

  // Step 4: walk the rest, splicing BeforeDecl metadata before
  // the first matching var-decl initializer. Consuming the bucket
  // preserves deterministic first-match-wins behavior for duplicate
  // initializer hashes.
  for item in iter {
    for hash in decl_init_hashes(&item) {
      if let Some(metas) = before_decl.remove(&hash) {
        result.extend(metas);
      }
    }
    result.push(item);
  }

  *module_body = result;
}

/// Stable hashes of every relevant var-decl initializer reachable from
/// `item`, matching the keys [`StateManager::queue_insertion`] uses
/// under [`InsertionSlot::BeforeDecl`].
fn decl_init_hashes(item: &ModuleItem) -> Vec<u128> {
  let mut hashes: Vec<u128> = Vec::new();

  let var_decls: Option<Vec<&VarDeclarator>> = match item {
    ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => export_decl
      .decl
      .as_var()
      .map(|var_decl| var_decl.decls.iter().collect()),
    ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(export_default_expr)) => {
      // `export default { ... }` is treated by the legacy code as a
      // synthetic `default = <obj>` declarator whose init is the
      // object expression — so its style metadata can splice in
      // front of the export.
      if export_default_expr.expr.is_object() {
        hashes.push(stable_hash_unspanned(export_default_expr.expr.as_ref()));
      }
      None
    },
    ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) => Some(var_decl.decls.iter().collect()),
    _ => None,
  };

  if let Some(decls) = var_decls {
    for decl in decls {
      if let Some(init) = decl.init.as_ref()
        && (init.is_object() || init.is_lit())
      {
        hashes.push(stable_hash_unspanned(init.as_ref()));
      }
    }
  }

  hashes
}

/// Builds an `_inject2({ ltr, priority, [rtl] })` statement for an atom style.
/// Mirrors the object construction in [`StateManager::add_style_to_inject`] but
/// produces a free-standing `ModuleItem` (atom injections do not carry
/// `constKey` / `constVal`).
fn build_atom_inject_item(metadata: &MetaData, inject_var_ident: &Ident) -> ModuleItem {
  let priority = metadata.get_priority();
  let css_ltr = metadata.get_css();
  let css_rtl = metadata.get_css_rtl();

  let mut stylex_inject_args = vec![
    create_string_key_value_prop("ltr", css_ltr),
    create_key_value_prop("priority", create_number_expr(*priority)),
  ];

  if let Some(rtl) = css_rtl {
    stylex_inject_args.push(create_string_key_value_prop("rtl", rtl));
  }

  let stylex_inject_obj = create_object_expression(stylex_inject_args);

  let stylex_call_expr = create_call_expr(
    Expr::Ident(inject_var_ident.clone()),
    vec![create_expr_or_spread(stylex_inject_obj)],
  );

  ModuleItem::Stmt(Stmt::Expr(ExprStmt {
    span: DUMMY_SP,
    expr: Box::new(Expr::Call(stylex_call_expr)),
  }))
}

fn add_inject_default_import_expression(ident: &Ident, inject_path: Option<&str>) -> ModuleItem {
  ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
    span: DUMMY_SP,
    specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
      span: DUMMY_SP,
      local: ident.clone(),
    })],
    src: Box::new(Str {
      span: DUMMY_SP,
      raw: None,
      value: inject_path.unwrap_or(DEFAULT_INJECT_PATH).into(),
    }),
    type_only: false,
    with: None,
    phase: ImportPhase::Evaluation,
  }))
}

pub(crate) fn add_import_expression(path: &str) -> ModuleItem {
  ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
    span: DUMMY_SP,
    specifiers: vec![],
    src: Box::new(Str {
      span: DUMMY_SP,
      raw: None,
      value: path.into(),
    }),
    type_only: false,
    with: None,
    phase: ImportPhase::Evaluation,
  }))
}

fn add_inject_named_import_expression(ident: &Ident, imported_ident: &Ident) -> ModuleItem {
  ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
    span: DUMMY_SP,
    specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
      span: DUMMY_SP,
      local: ident.clone(),
      imported: Some(ModuleExportName::Ident(imported_ident.clone())),
      is_type_only: false,
    })],
    src: Box::new(Str {
      span: DUMMY_SP,
      raw: None,
      value: DEFAULT_INJECT_PATH.into(),
    }),
    type_only: false,
    with: None,
    phase: ImportPhase::Evaluation,
  }))
}

fn add_inject_var_decl_expression(decl_ident: &Ident, value_ident: &Ident) -> ModuleItem {
  ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
    declare: false,
    decls: vec![VarDeclarator {
      definite: true,
      span: DUMMY_SP,
      name: Pat::from(create_binding_ident(decl_ident.clone())),
      init: Some(Box::new(Expr::from(value_ident.clone()))),
    }],
    kind: VarDeclKind::Var,
    span: DUMMY_SP,
    ctxt: SyntaxContext::empty(),
  }))))
}

pub(crate) fn matches_file_suffix(allowed_suffix: &str, filename: &str) -> bool {
  if filename.ends_with(allowed_suffix) {
    return true;
  }

  EXTENSIONS.iter().any(|&suffix| {
    let suffix = if allowed_suffix.is_empty() {
      suffix
    } else {
      &format!("{}{}", allowed_suffix, suffix)[..]
    };
    filename.ends_with(suffix)
  })
}

fn add_file_extension(imported_file_path: &str, source_file: &str) -> String {
  if EXTENSIONS
    .iter()
    .any(|ext| imported_file_path.ends_with(ext))
  {
    return imported_file_path.to_string();
  }

  let file_extension = Path::new(source_file)
    .extension()
    .and_then(std::ffi::OsStr::to_str)
    .unwrap_or_default();

  if file_extension.is_empty() {
    return imported_file_path.to_string();
  }

  format!("{}.{}", imported_file_path, file_extension)
}

fn file_path_resolver(
  relative_file_path: &str,
  source_file_path: &str,
  root_path: &str,
  aliases: &FxHashMap<String, Vec<String>>,
  root_dir: Option<&str>,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> anyhow::Result<String> {
  let resolved_path = resolve_file_path(
    relative_file_path,
    source_file_path,
    root_path,
    aliases,
    root_dir,
    package_json_seen,
  )?;

  Ok(resolved_path.display().to_string())
}

impl stylex_types::traits::StyleOptions for StateManager {
  fn options(&self) -> &StyleXStateOptions {
    &self.options
  }

  fn css_property_seen(&self) -> &FxHashMap<String, String> {
    &self.cache.css_property_seen
  }

  fn css_property_seen_mut(&mut self) -> &mut FxHashMap<String, String> {
    &mut self.cache.css_property_seen
  }

  fn other_injected_css_rules(&self) -> &stylex_types::traits::InjectableStylesMap {
    &self.other_injected_css_rules
  }

  fn other_injected_css_rules_mut(&mut self) -> &mut stylex_types::traits::InjectableStylesMap {
    &mut self.other_injected_css_rules
  }

  fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
    self
  }
}
