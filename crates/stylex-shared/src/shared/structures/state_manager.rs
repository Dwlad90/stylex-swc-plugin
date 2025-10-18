use core::panic;
use rustc_hash::{FxHashMap, FxHashSet};
use std::hash::Hash;
use std::path::Path;
use std::{option::Option, rc::Rc};

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
    ast::{JSXAttrOrSpread, Module},
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

use crate::shared::{
  constants::common::DEFAULT_INJECT_PATH,
  structures::{types::InjectableStylesMap, uid_generator::CounterMode},
  utils::ast::factories::{
    expr_or_spread_number_expression_factory, expr_or_spread_string_expression_factory,
  },
};
use crate::shared::{
  enums::data_structures::injectable_style::InjectableStyleKind,
  utils::{
    ast::factories::binding_ident_factory,
    common::{
      extract_filename_from_path, extract_filename_with_ext_from_path, extract_path, round_f64,
    },
  },
};
use crate::shared::{
  enums::{
    core::TransformationCycle,
    data_structures::{
      import_path_resolution::{ImportPathResolution, ImportPathResolutionType},
      style_vars_to_keep::StyleVarsToKeep,
      top_level_expression::{TopLevelExpression, TopLevelExpressionKind},
    },
  },
  utils::common::stable_hash,
};

use super::plugin_pass::PluginPass;
use super::stylex_options::ModuleResolution;
use super::stylex_options::{CheckModuleResolution, StyleXOptions};
use super::stylex_state_options::StyleXStateOptions;
use super::uid_generator::UidGenerator;
use super::{meta_data::MetaData, types::StylesObjectMap};
use super::{
  named_import_source::{ImportSources, NamedImportSource, RuntimeInjectionState},
  seen_value::SeenValue,
};

static TRANSFORMED_VARS_FILE_EXTENSION: Lazy<&'static str> = Lazy::new(|| ".transformed");

type AtomHashMap = FxHashMap<Atom, i16>;
type AtomHashSet = FxHashSet<Atom>;

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
  pub(crate) stylex_props_import: AtomHashSet,
  pub(crate) stylex_create_import: AtomHashSet,
  pub(crate) stylex_first_that_works_import: AtomHashSet,
  pub(crate) stylex_keyframes_import: AtomHashSet,
  pub(crate) stylex_define_vars_import: AtomHashSet,
  pub(crate) stylex_define_consts_import: AtomHashSet,
  pub(crate) stylex_create_theme_import: AtomHashSet,
  pub(crate) stylex_position_try_import: AtomHashSet,
  pub(crate) stylex_view_transition_class_import: AtomHashSet,
  pub(crate) stylex_default_marker_import: AtomHashSet,
  pub(crate) stylex_when_import: AtomHashSet,
  pub(crate) stylex_types_import: AtomHashSet,
  pub(crate) inject_import_inserted: Option<(Ident, Ident)>,
  pub(crate) export_id: Option<String>,

  pub(crate) debug_assertions_module: Option<Module>,

  pub(crate) class_name_declarations: Vec<Ident>,
  pub(crate) function_name_declarations: Vec<Ident>,
  pub(crate) declarations: Vec<VarDeclarator>,
  pub(crate) top_level_expressions: Vec<TopLevelExpression>,
  pub(crate) all_call_expressions: FxHashMap<u64, CallExpr>,
  pub(crate) var_decl_count_map: AtomHashMap,
  pub(crate) seen: FxHashMap<u64, Rc<SeenValueWithVarDeclCount>>,
  pub(crate) css_property_seen: FxHashMap<String, String>,
  pub(crate) seen_source_code_by_path: FxHashMap<FileName, String>,
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

  pub(crate) cycle: TransformationCycle,
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
      stylex_props_import: FxHashSet::default(),
      stylex_create_import: FxHashSet::default(),
      stylex_first_that_works_import: FxHashSet::default(),
      stylex_keyframes_import: FxHashSet::default(),
      stylex_define_vars_import: FxHashSet::default(),
      stylex_define_consts_import: FxHashSet::default(),
      stylex_create_theme_import: FxHashSet::default(),
      stylex_types_import: FxHashSet::default(),
      stylex_position_try_import: FxHashSet::default(),
      stylex_view_transition_class_import: FxHashSet::default(),
      stylex_default_marker_import: FxHashSet::default(),
      stylex_when_import: FxHashSet::default(),
      inject_import_inserted: None,
      style_map: FxHashMap::default(),
      style_vars: FxHashMap::default(),
      style_vars_to_keep: IndexSet::default(),
      member_object_ident_count_map: FxHashMap::default(),
      export_id: None,

      debug_assertions_module: None,

      seen: FxHashMap::default(),
      css_property_seen: FxHashMap::default(),
      seen_source_code_by_path: FxHashMap::default(),

      top_imports: vec![],

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

  pub(crate) fn get_debug_assertions_module(&self) -> Option<&Module> {
    if cfg!(debug_assertions) {
      self.debug_assertions_module.as_ref()
    } else {
      panic!("Cannot get debug assertions module in release mode");
    }
  }
  pub(crate) fn set_debug_assertions_module(&mut self, module: &Module) {
    if cfg!(debug_assertions) {
      self.debug_assertions_module = Some(drop_span(module.clone()));
    } else {
      panic!("Cannot set debug assertions module in release mode");
    }
  }

  pub fn import_as(&self, import: &str) -> Option<String> {
    for import_source in &self.options.import_sources {
      match import_source {
        ImportSources::Regular(_) => {}
        ImportSources::Named(named) => {
          if named.from.eq(import) {
            return Some(named.r#as.to_string());
          }
        }
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

    if filename.is_empty() || !matches_file_suffix(theme_file_extension, filename) {
      return None;
    }

    match unstable_module_resolution {
      CheckModuleResolution::Haste(_) => {
        let filename = FileName::Real(filename.into());
        extract_filename_with_ext_from_path(&filename).map(|s| s.to_string())
      }
      CheckModuleResolution::CommonJS(_) | CheckModuleResolution::CrossFileParsing(_) => {
        Some(self.get_canonical_file_path(filename, package_json_seen))
      }
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
        return format!(
          "{}:{}",
          package_name.unwrap_or_else(|| "_unknown_name_".to_string()),
          package_dir
        );
      }
    }

    if let Some(root_dir) = match &self.options.unstable_module_resolution {
      CheckModuleResolution::CommonJS(module_resolution) => module_resolution.root_dir.as_deref(),
      CheckModuleResolution::Haste(module_resolution) => module_resolution.root_dir.as_deref(),
      CheckModuleResolution::CrossFileParsing(module_resolution) => {
        module_resolution.root_dir.as_deref()
      }
    } {
      let file_path = Path::new(file_path);
      let root_dir = Path::new(root_dir);

      if let Some(root_dir) = relative_path(file_path, root_dir).to_str() {
        return root_dir.to_string();
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

    let is_valid_stylex_file = matches_file_suffix(theme_file_extension, import_path);
    let is_valid_transformed_vars_file =
      matches_file_suffix(&TRANSFORMED_VARS_FILE_EXTENSION, import_path);

    if !is_valid_stylex_file && !is_valid_transformed_vars_file {
      return ImportPathResolution::False;
    }

    match &self.options.unstable_module_resolution {
      CheckModuleResolution::CommonJS(_) => {
        let filename = self.get_filename();

        let (_, root_dir) = StateManager::get_package_name_and_path(filename, package_json_seen)
          .unwrap_or_else(|| panic!("Cannot get package name and path for: {}", filename));

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
      }
      CheckModuleResolution::Haste(_) => ImportPathResolution::Tuple(
        ImportPathResolutionType::ThemeNameRef,
        add_file_extension(import_path, source_file_path),
      ),
      _ => unimplemented!("Module resolution is not supported"),
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

    let needs_runtime_injection = style
      .values()
      .any(|value| matches!(value.as_ref(), InjectableStyleKind::Regular(_)));

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
      return self.inject_import_inserted.as_ref().unwrap().1.clone();
    }
    let mut uid_generator = UidGenerator::new("inject", CounterMode::Local);

    let runtime_injection = self
      .options
      .runtime_injection
      .as_ref()
      .cloned()
      .unwrap_or_else(|| RuntimeInjectionState::Regular(String::default()));

    let (inject_module_ident, inject_var_ident) = match self.inject_import_inserted.take() {
      Some(idents) => idents,
      None => {
        let module_ident = uid_generator.generate_ident();

        let var_ident = match &runtime_injection {
          RuntimeInjectionState::Regular(_) => uid_generator.generate_ident(),
          RuntimeInjectionState::Named(NamedImportSource { r#as, .. }) => {
            uid_generator = UidGenerator::new(r#as, CounterMode::Local);
            uid_generator.generate_ident()
          }
        };

        let idents = (module_ident, var_ident);
        self.inject_import_inserted = Some(idents.clone());

        idents
      }
    };

    let module_items = match &runtime_injection {
      RuntimeInjectionState::Regular(_) => vec![
        add_inject_default_import_expression(&inject_module_ident),
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
    let css = metadata.get_css();
    let css_rtl = metadata.get_css_rtl();

    let mut stylex_inject_args = vec![
      expr_or_spread_string_expression_factory(css),
      expr_or_spread_number_expression_factory(round_f64(*priority, 1)),
    ];

    if let Some(rtl) = css_rtl {
      stylex_inject_args.push(expr_or_spread_string_expression_factory(rtl));
    }

    let stylex_call_expr = CallExpr {
      span: DUMMY_SP,
      type_args: None,
      callee: Callee::Expr(Box::new(Expr::Ident(inject_var_ident.clone()))),
      args: stylex_inject_args,
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
    self.import_paths = union_hash_set(&self.import_paths, &other.import_paths);
    self.stylex_import = union_hash_set(&self.stylex_import, &other.stylex_import);
    self.stylex_props_import =
      union_hash_set(&self.stylex_props_import, &other.stylex_props_import);
    self.stylex_create_import =
      union_hash_set(&self.stylex_create_import, &other.stylex_create_import);
    self.stylex_first_that_works_import = union_hash_set(
      &self.stylex_first_that_works_import,
      &other.stylex_first_that_works_import,
    );
    self.stylex_keyframes_import = union_hash_set(
      &self.stylex_keyframes_import,
      &other.stylex_keyframes_import,
    );
    self.stylex_define_vars_import = union_hash_set(
      &self.stylex_define_vars_import,
      &other.stylex_define_vars_import,
    );
    self.stylex_create_theme_import = union_hash_set(
      &self.stylex_create_theme_import,
      &other.stylex_create_theme_import,
    );
    self.stylex_types_import =
      union_hash_set(&self.stylex_types_import, &other.stylex_types_import);
    self.inject_import_inserted = self
      .inject_import_inserted
      .clone()
      .or(other.inject_import_inserted.clone());
    self.export_id = self.export_id.clone().or(other.export_id.clone());
    self.declarations = chain_collect(self.declarations.clone(), other.declarations.clone());
    self.top_level_expressions = chain_collect(
      self.top_level_expressions.clone(),
      other.top_level_expressions.clone(),
    );

    self.all_call_expressions = chain_collect_hash_map(
      self.all_call_expressions.clone(),
      other.all_call_expressions.clone(),
    );
    self.var_decl_count_map = chain_collect_hash_map(
      self.var_decl_count_map.clone(),
      other.var_decl_count_map.clone(),
    );
    self.style_map = chain_collect_hash_map(self.style_map.clone(), other.style_map.clone());
    self.style_vars = chain_collect_hash_map(self.style_vars.clone(), other.style_vars.clone());
    self.style_vars_to_keep =
      union_index_set(&self.style_vars_to_keep.clone(), &other.style_vars_to_keep);
    self.member_object_ident_count_map = chain_collect_hash_map(
      self.member_object_ident_count_map.clone(),
      other.member_object_ident_count_map.clone(),
    );
    self.in_stylex_create = self.in_stylex_create || other.in_stylex_create;

    self.metadata = chain_collect_index_map(self.metadata.clone(), other.metadata.clone());
    self.seen = chain_collect_hash_map(self.seen.clone(), other.seen.clone());
    self.styles_to_inject = chain_collect_index_map(
      self.styles_to_inject.clone(),
      other.styles_to_inject.clone(),
    );
    self.prepend_include_module_items = chain_collect(
      self.prepend_include_module_items.clone(),
      other.prepend_include_module_items.clone(),
    );
    self.prepend_import_module_items = chain_collect(
      self.prepend_import_module_items.clone(),
      other.prepend_import_module_items.clone(),
    );
    self.other_injected_css_rules = chain_collect_index_map(
      self.other_injected_css_rules.clone(),
      other.other_injected_css_rules.clone(),
    );
    self.top_imports = chain_collect(self.top_imports.clone(), other.top_imports.clone());
  }
}

fn add_inject_default_import_expression(ident: &Ident) -> ModuleItem {
  ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
    span: DUMMY_SP,
    specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
      span: DUMMY_SP,
      local: ident.clone(),
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
      name: Pat::from(binding_ident_factory(decl_ident.clone())),
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

fn chain_collect<T: Clone + Eq + PartialEq>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
  if vec1 == vec2 {
    return vec1;
  }

  if vec1.len() < vec2.len() {
    let commom_part = vec2.iter().take(vec1.len()).cloned().collect::<Vec<T>>();

    if commom_part == vec1 {
      return vec2;
    }

    let mut vec = vec1.clone();

    vec.retain(|item| vec2.contains(item));

    for item in vec2.iter() {
      if !vec.contains(item) {
        vec.push(item.clone());
      }
    }

    return vec;
  }

  let mut vec = vec1.clone();

  vec.retain(|item| !vec2.contains(item));

  for item in vec2.iter() {
    if !vec.contains(item) {
      vec.push(item.clone());
    }
  }

  vec
}

fn union_hash_set<T: Clone + Eq + Hash>(set1: &FxHashSet<T>, set2: &FxHashSet<T>) -> FxHashSet<T> {
  set1.union(set2).cloned().collect()
}

fn chain_collect_hash_map<K: Eq + Hash, V: Clone + PartialEq>(
  map1: FxHashMap<K, V>,
  map2: FxHashMap<K, V>,
) -> FxHashMap<K, V> {
  if map1 == map2 {
    return map1;
  }
  map1.into_iter().chain(map2).collect()
}

fn union_index_set<T: Clone + Eq + Hash>(set1: &IndexSet<T>, set2: &IndexSet<T>) -> IndexSet<T> {
  if set1 == set2 {
    return set1.clone();
  }

  set1.union(set2).cloned().collect()
}

fn chain_collect_index_map<K: Eq + Hash, V: Clone + PartialEq>(
  map1: IndexMap<K, V>,
  map2: IndexMap<K, V>,
) -> IndexMap<K, V> {
  if map1 == map2 {
    return map1;
  }

  map1.into_iter().chain(map2).collect()
}

fn file_path_resolver(
  relative_file_path: &str,
  source_file_path: &str,
  root_path: &str,
  aliases: &FxHashMap<String, Vec<String>>,
  package_json_seen: &mut FxHashMap<String, PackageJsonExtended>,
) -> String {
  if EXTENSIONS
    .iter()
    .any(|ext| relative_file_path.ends_with(ext))
  {
    unimplemented!("Extension match found, but handling is unimplemented");
  }

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

  panic!("Cannot resolve file path: {}", relative_file_path)
}
