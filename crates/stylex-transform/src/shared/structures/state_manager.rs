use rustc_hash::{FxHashMap, FxHashSet};
use std::hash::Hash;
use std::path::Path;
use std::{option::Option, rc::Rc};
use stylex_macros::{stylex_panic, stylex_unimplemented};

use indexmap::{IndexMap, IndexSet};
use log::debug;
use once_cell::sync::Lazy;
use stylex_path_resolver::{
  package_json::{PackageJsonExtended, find_closest_package_json_folder, get_package_json},
  resolvers::{EXTENSIONS, resolve_file_path},
  utils::relative_path,
};
use swc_core::{
  atoms::Atom,
  common::{DUMMY_SP, EqIgnoreSpan, FileName},
  ecma::{
    ast::{JSXAttrOrSpread, Module, NamedExport, Program},
    utils::drop_span,
  },
};
use swc_core::{
  common::SyntaxContext,
  ecma::ast::{
    CallExpr, Callee, Decl, Expr, ExprStmt, Ident, ImportDecl, ImportDefaultSpecifier,
    ImportNamedSpecifier, ImportPhase, ImportSpecifier, ModuleDecl, ModuleExportName, ModuleItem,
    Pat, Stmt, Str, VarDecl, VarDeclKind, VarDeclarator,
  },
};

use crate::shared::structures::types::InjectableStylesMap;
use crate::shared::utils::ast::convertors::create_number_expr;
use crate::shared::utils::common::{
  extract_filename_from_path, extract_filename_with_ext_from_path, extract_path,
};
use stylex_utils::hash::stable_hash;
use stylex_utils::math::round_f64;
use stylex_ast::ast::factories::create_binding_ident;
use stylex_ast::ast::factories::{
  create_expr_or_spread, create_key_value_prop, create_number_expr_or_spread,
  create_object_expression, create_string_expr_or_spread, create_string_key_value_prop,
};
use stylex_constants::constants::common::{CONSTS_FILE_EXTENSION, DEFAULT_INJECT_PATH};
use stylex_enums::core::TransformationCycle;
use stylex_enums::counter_mode::CounterMode;
use stylex_enums::import_path_resolution::{ImportPathResolution, ImportPathResolutionType};
use stylex_enums::top_level_expression::TopLevelExpressionKind;
use stylex_structures::style_vars_to_keep::StyleVarsToKeep;
use stylex_structures::top_level_expression::TopLevelExpression;
use stylex_types::enums::data_structures::injectable_style::InjectableStyleKind;

use super::seen_value::SeenValue;
use super::types::StylesObjectMap;
use stylex_structures::named_import_source::{
  ImportSources, NamedImportSource, RuntimeInjectionState,
};
use stylex_structures::plugin_pass::PluginPass;
use stylex_structures::stylex_options::ModuleResolution;
use stylex_structures::stylex_options::{CheckModuleResolution, StyleXOptions};
use stylex_structures::stylex_state_options::StyleXStateOptions;
use stylex_structures::uid_generator::UidGenerator;
use stylex_types::structures::meta_data::MetaData;

static TRANSFORMED_VARS_FILE_EXTENSION: Lazy<&'static str> = Lazy::new(|| ".transformed");

type AtomHashMap = FxHashMap<Atom, i16>;
type AtomHashSet = FxHashSet<Atom>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ImportKind {
  Props,
  Attrs,
  Create,
  FirstThatWorks,
  Keyframes,
  DefineVars,
  DefineMarker,
  DefineConsts,
  CreateTheme,
  PositionTry,
  ViewTransitionClass,
  DefaultMarker,
  When,
  Types,
  Env,
}

impl ImportKind {
  pub(crate) const ALL: &[ImportKind] = &[
    ImportKind::Props,
    ImportKind::Attrs,
    ImportKind::Create,
    ImportKind::FirstThatWorks,
    ImportKind::Keyframes,
    ImportKind::DefineVars,
    ImportKind::DefineMarker,
    ImportKind::DefineConsts,
    ImportKind::CreateTheme,
    ImportKind::PositionTry,
    ImportKind::ViewTransitionClass,
    ImportKind::DefaultMarker,
    ImportKind::When,
    ImportKind::Types,
    ImportKind::Env,
  ];

  pub(crate) fn from_import_name(name: &str) -> Option<ImportKind> {
    match name {
      "create" => Some(ImportKind::Create),
      "props" => Some(ImportKind::Props),
      "attrs" => Some(ImportKind::Attrs),
      "keyframes" => Some(ImportKind::Keyframes),
      "firstThatWorks" => Some(ImportKind::FirstThatWorks),
      "defineVars" => Some(ImportKind::DefineVars),
      "defineConsts" => Some(ImportKind::DefineConsts),
      "defineMarker" => Some(ImportKind::DefineMarker),
      "createTheme" => Some(ImportKind::CreateTheme),
      "positionTry" => Some(ImportKind::PositionTry),
      "viewTransitionClass" => Some(ImportKind::ViewTransitionClass),
      "types" => Some(ImportKind::Types),
      "when" => Some(ImportKind::When),
      "env" => Some(ImportKind::Env),
      "defaultMarker" => Some(ImportKind::DefaultMarker),
      _ => None,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SeenValueWithVarDeclCount {
  pub(crate) seen_value: SeenValue,
  pub(crate) var_decl_count: Option<AtomHashMap>,
}

#[derive(Clone, Debug)]
pub struct StateManager {
  pub(crate) _state: PluginPass,

  // Imports
  pub(crate) import_paths: FxHashSet<String>,
  pub(crate) stylex_import: FxHashSet<ImportSources>,
  pub(crate) import_specifiers: Vec<String>,
  pub(crate) stylex_api_imports: FxHashMap<ImportKind, AtomHashSet>,
  pub(crate) inject_import_inserted: Option<(Ident, Ident)>,
  pub(crate) export_id: Option<String>,

  pub(crate) seen_module_source_code: Option<Box<(Program, Option<String>)>>,

  pub(crate) class_name_declarations: Vec<Ident>,
  pub(crate) function_name_declarations: Vec<Ident>,
  pub(crate) declarations: Vec<VarDeclarator>,
  pub(crate) top_level_expressions: Vec<TopLevelExpression>,
  pub all_call_expressions: FxHashMap<u64, CallExpr>,
  pub(crate) var_decl_count_map: AtomHashMap,
  pub(crate) seen: FxHashMap<u64, Rc<SeenValueWithVarDeclCount>>,
  pub(crate) css_property_seen: FxHashMap<String, String>,
  pub(crate) span_cache: FxHashMap<u64, swc_core::common::Span>,
  pub(crate) jsx_spread_attr_exprs_map: FxHashMap<Expr, Vec<JSXAttrOrSpread>>,

  // `stylex.create` calls
  pub(crate) style_map: FxHashMap<String, Rc<StylesObjectMap>>,
  pub(crate) style_vars: FxHashMap<String, VarDeclarator>,

  // results of `stylex.create` calls that should be kept
  pub(crate) style_vars_to_keep: IndexSet<StyleVarsToKeep>,
  pub(crate) member_object_ident_count_map: AtomHashMap,

  pub(crate) in_stylex_create: bool,

  pub(crate) options: StyleXStateOptions,
  pub metadata: IndexMap<String, IndexSet<MetaData>>,
  pub(crate) styles_to_inject: IndexMap<u64, Vec<ModuleItem>>,
  pub(crate) prepend_include_module_items: Vec<ModuleItem>,
  pub(crate) hoisted_module_items: Vec<ModuleItem>,
  pub(crate) prepend_import_module_items: Vec<ModuleItem>,

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
      _state: PluginPass::default(),
      import_paths: FxHashSet::default(),
      stylex_import: FxHashSet::default(),
      import_specifiers: vec![],
      stylex_api_imports: FxHashMap::default(),
      inject_import_inserted: None,
      style_map: FxHashMap::default(),
      style_vars: FxHashMap::default(),
      style_vars_to_keep: IndexSet::default(),
      member_object_ident_count_map: FxHashMap::default(),
      export_id: None,

      seen: FxHashMap::default(),
      css_property_seen: FxHashMap::default(),
      seen_module_source_code: None,
      span_cache: FxHashMap::default(),

      top_imports: vec![],
      named_exports: FxHashSet::default(),

      declarations: vec![],
      class_name_declarations: vec![],
      function_name_declarations: vec![],
      top_level_expressions: vec![],
      all_call_expressions: FxHashMap::default(),
      var_decl_count_map: FxHashMap::default(),
      jsx_spread_attr_exprs_map: FxHashMap::default(),

      in_stylex_create: false,
      options,

      metadata: IndexMap::new(),
      styles_to_inject: IndexMap::new(),
      prepend_include_module_items: vec![],
      prepend_import_module_items: vec![],
      hoisted_module_items: vec![],

      other_injected_css_rules: IndexMap::new(),

      cycle: TransformationCycle::Initializing,
    }
  }

  /// Check if an import of the given kind contains the given symbol.
  pub(crate) fn has_stylex_api_import(&self, kind: ImportKind, sym: &Atom) -> bool {
    self
      .stylex_api_imports
      .get(&kind)
      .is_some_and(|set| set.contains(sym))
  }

  /// Insert a symbol into the import set for the given kind.
  pub(crate) fn insert_stylex_api_import(&mut self, kind: ImportKind, sym: Atom) {
    self.stylex_api_imports.entry(kind).or_default().insert(sym);
  }

  /// Get the import set for the given kind, if any entries exist.
  pub(crate) fn get_stylex_api_import(&self, kind: ImportKind) -> Option<&AtomHashSet> {
    self.stylex_api_imports.get(&kind)
  }

  /// Check if any import of the given kinds contains the given symbol.
  pub(crate) fn any_stylex_api_import_contains(&self, kinds: &[ImportKind], sym: &Atom) -> bool {
    kinds
      .iter()
      .any(|kind| self.has_stylex_api_import(*kind, sym))
  }

  /// Applies the `env` configuration to the given identifiers and member_expressions maps.
  /// This is the Rust equivalent of the JavaScript `applyStylexEnv` method.
  pub(crate) fn apply_stylex_env(
    &self,
    identifiers: &mut super::types::FunctionMapIdentifiers,
    member_expressions: &mut super::types::FunctionMapMemberExpression,
  ) {
    if self.options.env.is_empty() {
      return;
    }

    let env = self.options.env.clone();

    // For namespace imports (e.g., `import stylex from '@stylexjs/stylex'`),
    // add `env` to member_expressions so `stylex.env.x` resolves.
    for name in &self.stylex_import {
      let member_expression = member_expressions.entry(name.clone()).or_default();
      member_expression.insert(
        "env".into(),
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
    if let Some((program, source_code)) = self.seen_module_source_code.as_ref().map(|b| b.as_ref())
      && let Program::Module(module) = program
    {
      return Some((module, source_code));
    }

    None
  }

  /// Sets the source code module (marks as not yet normalized)
  pub(crate) fn set_seen_module_source_code(
    &mut self,
    module: &Module,
    source_code: Option<String>,
  ) {
    self.seen_module_source_code = Some(Box::new((Program::Module(module.clone()), source_code)));
  }

  pub fn import_as(&self, import: &str) -> Option<String> {
    for import_source in &self.options.import_sources {
      match import_source {
        ImportSources::Regular(_) => {},
        ImportSources::Named(named) => {
          if named.from.eq(import) {
            return Some(named.r#as.to_string());
          }
        },
      }
    }

    None
  }

  pub fn import_sources(&self) -> Vec<ImportSources> {
    self.options.import_sources.to_vec()
  }

  pub fn import_sources_stringified(&self) -> Vec<String> {
    self
      .options
      .import_sources
      .iter()
      .map(|import_source| match import_source {
        ImportSources::Regular(regular) => regular.as_str(),
        ImportSources::Named(named) => named.from.as_str(),
      })
      .map(ToString::to_string)
      .collect()
  }

  pub fn stylex_import_stringified(&self) -> Vec<String> {
    self
      .stylex_import
      .iter()
      .map(|import_source| match &import_source {
        ImportSources::Regular(regular) => regular.as_str(),
        ImportSources::Named(named) => named.r#as.as_str(),
      })
      .map(ToString::to_string)
      .collect()
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

  pub(crate) fn get_short_filename(&self) -> String {
    extract_filename_from_path(&self._state.filename)
  }
  pub(crate) fn get_filename(&self) -> &str {
    extract_path(&self._state.filename)
  }
  pub(crate) fn get_filename_for_hashing(
    &self,
    package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
  ) -> Option<String> {
    let filename = self.get_filename();

    let unstable_module_resolution = &self.options.unstable_module_resolution;

    let theme_file_extension = match unstable_module_resolution {
      CheckModuleResolution::CommonJS(ModuleResolution {
        theme_file_extension,
        ..
      })
      | CheckModuleResolution::Haste(ModuleResolution {
        theme_file_extension,
        ..
      })
      | CheckModuleResolution::CrossFileParsing(ModuleResolution {
        theme_file_extension,
        ..
      }) => theme_file_extension.as_deref().unwrap_or(".stylex"),
    };

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
      CheckModuleResolution::Haste(_) => {
        let filename = FileName::Real(filename.into());
        extract_filename_with_ext_from_path(&filename).map(|s| s.to_string())
      },
      CheckModuleResolution::CommonJS(_) | CheckModuleResolution::CrossFileParsing(_) => {
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

    if let Some(root_dir) = match &self.options.unstable_module_resolution {
      CheckModuleResolution::CommonJS(module_resolution) => module_resolution.root_dir.as_deref(),
      CheckModuleResolution::Haste(module_resolution) => module_resolution.root_dir.as_deref(),
      CheckModuleResolution::CrossFileParsing(module_resolution) => {
        module_resolution.root_dir.as_deref()
      },
    } {
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
      return ImportPathResolution::False;
    }

    let theme_file_extension = (match &self.options.unstable_module_resolution {
      CheckModuleResolution::CommonJS(module_resolution) => module_resolution,
      CheckModuleResolution::Haste(module_resolution) => module_resolution,
      CheckModuleResolution::CrossFileParsing(module_resolution) => module_resolution,
    })
    .theme_file_extension
    .as_deref()
    .unwrap_or(".stylex");

    let consts_file_extension = format!("{}{}", theme_file_extension, CONSTS_FILE_EXTENSION);

    let is_theme_file = matches_file_suffix(theme_file_extension, import_path);
    let is_consts_only_file = matches_file_suffix(&consts_file_extension, import_path);

    let is_valid_transformed_vars_file =
      matches_file_suffix(*TRANSFORMED_VARS_FILE_EXTENSION, import_path);

    if !is_theme_file && !is_valid_transformed_vars_file && !is_consts_only_file {
      return ImportPathResolution::False;
    }

    match &self.options.unstable_module_resolution {
      CheckModuleResolution::CommonJS(_) => {
        let filename = self.get_filename();

        let (_, root_dir) = StateManager::get_package_name_and_path(filename, package_json_seen)
          .unwrap_or_else(|| stylex_panic!("Cannot get package name and path for: {}", filename));

        let aliases = self.options.aliases.as_ref().cloned().unwrap_or_default();

        let resolved_file_path = file_path_resolver(
          import_path,
          source_file_path,
          &root_dir,
          &aliases,
          package_json_seen,
        );

        debug!("Resolved import path: {}", resolved_file_path);

        let resolved_file_path =
          self.get_canonical_file_path(&resolved_file_path, package_json_seen);

        ImportPathResolution::Tuple(ImportPathResolutionType::ThemeNameRef, resolved_file_path)
      },
      CheckModuleResolution::Haste(_) => ImportPathResolution::Tuple(
        ImportPathResolutionType::ThemeNameRef,
        add_file_extension(import_path, source_file_path),
      ),
      _ => stylex_unimplemented!("This module resolution strategy is not yet supported."),
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

  fn setup_injection_imports(&mut self) -> Ident {
    if !self.prepend_include_module_items.is_empty() {
      return match self.inject_import_inserted.as_ref() {
        Some(idents) => idents.1.clone(),
        None => stylex_panic!(
          "inject_import_inserted is None when prepend_include_module_items is non-empty"
        ),
      };
    }
    let mut uid_generator = UidGenerator::new("inject", CounterMode::Local);

    let runtime_injection = self
      .options
      .runtime_injection
      .as_ref()
      .cloned()
      .unwrap_or(RuntimeInjectionState::Boolean(true));

    let (inject_module_ident, inject_var_ident) = match self.inject_import_inserted.take() {
      Some(idents) => idents,
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

        let idents = (module_ident, var_ident);
        self.inject_import_inserted = Some(idents.clone());

        idents
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

    self.prepend_include_module_items.extend(module_items);
    inject_var_ident
  }

  fn update_references(&mut self, call: &CallExpr, ast: &Expr, _fallback_ast: Option<&Expr>) {
    if let Some(item) = self.declarations.iter_mut().find(|decl| {
      decl.init.as_ref().is_some_and(
        |expr| matches!(**expr, Expr::Call(ref existing_call) if existing_call == call),
      )
    }) {
      item.init = Some(Box::new(ast.clone()));
    }

    if let Some((_, item)) = self.style_vars.iter_mut().find(|(_, decl)| {
      decl.init.as_ref().is_some_and(
        |expr| matches!(**expr, Expr::Call(ref existing_call) if existing_call == call),
      )
    }) {
      item.init = Some(Box::new(ast.clone()));
    }

    if let Some(top_level_expr) = self
      .top_level_expressions
      .iter_mut()
      .find(|TopLevelExpression(_, expr, _)| matches!(expr, Expr::Call(c) if c == call))
    {
      top_level_expr.1 = ast.clone();
    }

    let call_hash = stable_hash(call);

    self.all_call_expressions.remove(&call_hash);

    if let Some(call_expr) = ast.as_call() {
      let new_call_hash = stable_hash(call_expr);

      self
        .all_call_expressions
        .insert(new_call_hash, call_expr.clone());
    }
  }

  fn add_style(&mut self, metadata: &MetaData) {
    let var_name = "stylex";
    let value = self.metadata.entry(var_name.to_string()).or_default();

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
      create_key_value_prop("priority", create_number_expr(round_f64(*priority, 1))),
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

    let stylex_call_expr = CallExpr {
      span: DUMMY_SP,
      type_args: None,
      callee: Callee::Expr(Box::new(Expr::Ident(inject_var_ident.clone()))),
      args: vec![create_expr_or_spread(stylex_inject_obj)],
      ctxt: SyntaxContext::empty(),
    };

    let stylex_call = Expr::Call(stylex_call_expr);

    let module = ModuleItem::Stmt(Stmt::Expr(ExprStmt {
      span: DUMMY_SP,
      expr: Box::new(stylex_call),
    }));

    let ast_hash = stable_hash(ast);

    let styles_to_inject = self.styles_to_inject.entry(ast_hash).or_default();

    if !styles_to_inject.contains(&drop_span(module.clone())) {
      styles_to_inject.push(drop_span(module.clone()));
    }

    if let Some(fallback_ast) = fallback_ast {
      let fallback_ast_hash = stable_hash(fallback_ast);

      let fallback_styles_to_inject = self.styles_to_inject.entry(fallback_ast_hash).or_default();

      if !fallback_styles_to_inject.contains(&drop_span(module.clone())) {
        fallback_styles_to_inject.push(drop_span(module.clone()));
      }
    }
  }

  // pub(crate) fn _get_css_vars(&self) -> FxHashMap<String, String> {
  //   self.options.defined_stylex_css_variables.clone()
  // }

  pub(crate) fn get_treeshake_compensation(&self) -> bool {
    self.options.treeshake_compensation
  }

  // Now you can use these helper functions to simplify your function
  pub fn combine(&mut self, other: &Self) {
    // Hash sets: extend in-place
    self.import_paths.extend(other.import_paths.iter().cloned());
    self
      .stylex_import
      .extend(other.stylex_import.iter().cloned());

    // Combine all API import sets (fixes bug where 7 kinds were previously missing)
    for kind in ImportKind::ALL {
      if let Some(other_set) = other.stylex_api_imports.get(kind) {
        let self_set = self.stylex_api_imports.entry(*kind).or_default();
        for item in other_set {
          self_set.insert(item.clone());
        }
      }
    }

    // Option fields: only clone from other if self is None
    if self.inject_import_inserted.is_none() {
      self.inject_import_inserted = other.inject_import_inserted.clone();
    }
    if self.export_id.is_none() {
      self.export_id = other.export_id.clone();
    }

    // Vecs: merge in-place
    chain_collect_in_place(&mut self.declarations, &other.declarations);
    chain_collect_in_place(
      &mut self.top_level_expressions,
      &other.top_level_expressions,
    );

    // Hash maps: extend in-place (other values take precedence)
    extend_hash_map(&mut self.all_call_expressions, &other.all_call_expressions);
    extend_hash_map(&mut self.var_decl_count_map, &other.var_decl_count_map);
    extend_hash_map(&mut self.style_map, &other.style_map);
    extend_hash_map(&mut self.style_vars, &other.style_vars);

    // Index set: extend in-place
    self
      .style_vars_to_keep
      .extend(other.style_vars_to_keep.iter().cloned());

    extend_hash_map(
      &mut self.member_object_ident_count_map,
      &other.member_object_ident_count_map,
    );
    self.in_stylex_create = self.in_stylex_create || other.in_stylex_create;

    // Index maps: extend in-place
    extend_index_map(&mut self.metadata, &other.metadata);
    extend_hash_map(&mut self.seen, &other.seen);
    extend_index_map(&mut self.styles_to_inject, &other.styles_to_inject);

    chain_collect_in_place(
      &mut self.prepend_include_module_items,
      &other.prepend_include_module_items,
    );
    chain_collect_in_place(
      &mut self.prepend_import_module_items,
      &other.prepend_import_module_items,
    );
    extend_index_map(
      &mut self.other_injected_css_rules,
      &other.other_injected_css_rules,
    );
    chain_collect_in_place(&mut self.top_imports, &other.top_imports);
  }
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

/// Merge `source` items into `target` Vec, deduplicating.
/// Preserves original combine() semantics: when target is shorter than source
/// and target is a prefix of source, takes source directly. Otherwise merges.
fn chain_collect_in_place<T: Clone + Eq>(target: &mut Vec<T>, source: &[T]) {
  if target.as_slice() == source {
    return;
  }

  if target.len() < source.len() {
    if source.iter().take(target.len()).eq(target.iter()) {
      *target = source.to_vec();
      return;
    }

    target.retain(|item| source.contains(item));
    let new_items: Vec<_> = source
      .iter()
      .filter(|item| !target.contains(item))
      .cloned()
      .collect();
    target.extend(new_items);
    return;
  }

  target.retain(|item| !source.contains(item));
  let new_items: Vec<_> = source
    .iter()
    .filter(|item| !target.contains(item))
    .cloned()
    .collect();
  target.extend(new_items);
}

/// Extend a FxHashMap in-place. Source values take precedence on key conflicts.
fn extend_hash_map<K: Clone + Eq + Hash, V: Clone + PartialEq>(
  target: &mut FxHashMap<K, V>,
  source: &FxHashMap<K, V>,
) {
  if target != source {
    target.extend(source.iter().map(|(k, v)| (k.clone(), v.clone())));
  }
}

/// Extend an IndexMap in-place. Source values take precedence on key conflicts.
fn extend_index_map<K: Clone + Eq + Hash, V: Clone + PartialEq>(
  target: &mut IndexMap<K, V>,
  source: &IndexMap<K, V>,
) {
  if target != source {
    target.extend(source.iter().map(|(k, v)| (k.clone(), v.clone())));
  }
}

fn file_path_resolver(
  relative_file_path: &str,
  source_file_path: &str,
  root_path: &str,
  aliases: &FxHashMap<String, Vec<String>>,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> String {
  let resolved_file_path = resolve_file_path(
    relative_file_path,
    source_file_path,
    root_path,
    aliases,
    package_json_seen,
  );

  if let Ok(resolved_path) = resolved_file_path {
    let resolved_path_str = resolved_path.display().to_string();

    return resolved_path_str;
  }

  stylex_panic!("Cannot resolve file path: {}", relative_file_path)
}

impl stylex_types::traits::StyleOptions for StateManager {
  fn options(&self) -> &StyleXStateOptions {
    &self.options
  }

  fn css_property_seen(&self) -> &FxHashMap<String, String> {
    &self.css_property_seen
  }

  fn css_property_seen_mut(&mut self) -> &mut FxHashMap<String, String> {
    &mut self.css_property_seen
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
