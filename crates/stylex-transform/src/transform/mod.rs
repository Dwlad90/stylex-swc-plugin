use indexmap::IndexMap;
use rustc_hash::{FxHashMap, FxHashSet};
use swc_core::{
  common::{EqIgnoreSpan, Mark, comments::Comments},
  ecma::{
    ast::{CallExpr, Callee, Expr, Id, MemberProp, Pass, VarDeclarator},
    transforms::{base::resolver, typescript::strip},
    visit::fold_pass,
  },
};

use crate::shared::{structures::state_manager::StateManager, utils::common::increase_ident_count};
use stylex_enums::{
  property_validation_mode::PropertyValidationMode, style_resolution::StyleResolution,
  sx_prop_name_param::SxPropNameParam,
};
use stylex_structures::{
  named_import_source::{ImportSources, RuntimeInjection},
  plugin_pass::PluginPass,
  stylex_env::EnvEntry,
  stylex_options::{ModuleResolution, StyleXOptionsParams},
};

mod fold;
pub(crate) mod styleq;
pub(crate) mod stylex;

pub struct StyleXTransform<C>
where
  C: Comments,
{
  pub comments: C,
  props_declaration: Option<Id>,
  pub state: StateManager,
}

/// Builder for constructing test transforms with the `With` pattern.
///
/// # Examples
/// ```ignore
/// // Default test with runtime injection:
/// StyleXTransform::test(tr.comments.clone()).with_runtime_injection().into_pass()
///
/// // With custom options:
/// StyleXTransform::test(tr.comments.clone()).with_options(&mut cfg).into_pass()
///
/// // With custom PluginPass and options:
/// StyleXTransform::test(tr.comments.clone())
///     .with_pass(pass)
///     .with_options(&mut cfg)
///     .with_runtime_injection()
///     .into_pass()
/// ```
#[must_use = "builder; call .build() or .into_pass() to finalize the transform"]
pub struct StyleXTransformBuilder<C>
where
  C: Comments,
{
  comments: C,
  plugin_pass: PluginPass,
  config: Option<StyleXOptionsParams>,
  runtime_injection: bool,
}

impl<C> StyleXTransformBuilder<C>
where
  C: Comments,
{
  pub fn with_pass(mut self, pass: PluginPass) -> Self {
    self.plugin_pass = pass;
    self
  }

  pub fn with_filename(mut self, filename: swc_core::common::FileName) -> Self {
    self.plugin_pass.filename = filename;
    self
  }

  pub fn with_cwd(mut self, cwd: impl Into<std::path::PathBuf>) -> Self {
    self.plugin_pass.cwd = Some(cwd.into());
    self
  }

  pub fn with_options(mut self, config: &mut StyleXOptionsParams) -> Self {
    self.config = Some(config.clone());
    self
  }

  pub fn with_runtime_injection(mut self) -> Self {
    self.runtime_injection = true;
    self
  }

  fn ensure_config(&mut self) -> &mut StyleXOptionsParams {
    self.config.get_or_insert_with(StyleXOptionsParams::default)
  }

  pub fn with_style_resolution(mut self, val: StyleResolution) -> Self {
    self.ensure_config().style_resolution = Some(val);
    self
  }

  pub fn with_dev(mut self, val: bool) -> Self {
    self.ensure_config().dev = Some(val);
    self
  }

  pub fn with_debug(mut self, val: bool) -> Self {
    self.ensure_config().debug = Some(val);
    self
  }

  pub fn with_enable_debug_class_names(mut self, val: bool) -> Self {
    self.ensure_config().enable_debug_class_names = Some(val);
    self
  }

  pub fn with_enable_dev_class_names(mut self, val: bool) -> Self {
    self.ensure_config().enable_dev_class_names = Some(val);
    self
  }

  pub fn with_enable_debug_data_prop(mut self, val: bool) -> Self {
    self.ensure_config().enable_debug_data_prop = Some(val);
    self
  }

  pub fn with_class_name_prefix(mut self, val: impl Into<String>) -> Self {
    self.ensure_config().class_name_prefix = Some(val.into());
    self
  }

  pub fn with_unstable_module_resolution(mut self, val: ModuleResolution) -> Self {
    self.ensure_config().unstable_module_resolution = Some(val);
    self
  }

  pub fn with_treeshake_compensation(mut self, val: bool) -> Self {
    self.ensure_config().treeshake_compensation = Some(val);
    self
  }

  pub fn with_runtime_injection_option(mut self, val: RuntimeInjection) -> Self {
    self.ensure_config().runtime_injection = Some(val);
    self
  }

  pub fn with_enable_logical_styles_polyfill(mut self, val: bool) -> Self {
    self.ensure_config().enable_logical_styles_polyfill = Some(val);
    self
  }

  pub fn with_enable_legacy_value_flipping(mut self, val: bool) -> Self {
    self.ensure_config().enable_legacy_value_flipping = Some(val);
    self
  }

  pub fn with_enable_font_size_px_to_rem(mut self, val: bool) -> Self {
    self.ensure_config().enable_font_size_px_to_rem = Some(val);
    self
  }

  pub fn with_enable_inlined_conditional_merge(mut self, val: bool) -> Self {
    self.ensure_config().enable_inlined_conditional_merge = Some(val);
    self
  }

  pub fn with_enable_media_query_order(mut self, val: bool) -> Self {
    self.ensure_config().enable_media_query_order = Some(val);
    self
  }

  pub fn with_enable_minified_keys(mut self, val: bool) -> Self {
    self.ensure_config().enable_minified_keys = Some(val);
    self
  }

  pub fn with_inject_stylex_side_effects(mut self, val: bool) -> Self {
    self.ensure_config().inject_stylex_side_effects = Some(val);
    self
  }

  pub fn with_property_validation_mode(mut self, val: PropertyValidationMode) -> Self {
    self.ensure_config().property_validation_mode = Some(val);
    self
  }

  pub fn with_import_sources(mut self, val: Vec<ImportSources>) -> Self {
    self.ensure_config().import_sources = Some(val);
    self
  }

  pub fn with_aliases(mut self, val: FxHashMap<String, Vec<String>>) -> Self {
    self.ensure_config().aliases = Some(val);
    self
  }

  pub fn with_defined_stylex_css_variables(mut self, val: FxHashMap<String, String>) -> Self {
    self.ensure_config().defined_stylex_css_variables = Some(val);
    self
  }

  pub fn with_env(mut self, val: IndexMap<String, EnvEntry>) -> Self {
    self.ensure_config().env = Some(val);
    self
  }

  pub fn with_sx_prop_name(mut self, val: SxPropNameParam) -> Self {
    self.ensure_config().sx_prop_name = Some(val);
    self
  }

  pub fn build(self) -> StyleXTransform<C> {
    let had_config = self.config.is_some();
    let mut config = self.config.unwrap_or_default();

    if self.runtime_injection {
      config.runtime_injection = Some(RuntimeInjection::Boolean(true));
      config.treeshake_compensation = Some(true);
    }

    // Apply Haste as the test default only when no explicit config was provided.
    // If the caller set any option (e.g. with_filename, with_dev), treat it as
    // an intentional configuration and skip the default.
    if !had_config && config.unstable_module_resolution.is_none() {
      config.unstable_module_resolution = Some(ModuleResolution::haste(None));
    }

    let stylex_imports = fill_stylex_imports_from_params(&config);

    let mut state = StateManager::new(config.into());

    state.options.import_sources = stylex_imports.into_iter().collect();
    state.set_plugin_pass(self.plugin_pass);

    StyleXTransform {
      comments: self.comments,
      props_declaration: None,
      state,
    }
  }

  pub fn into_pass(self) -> impl Pass + use<C> {
    let unresolved_mark = Mark::new();
    let top_level_mark = Mark::new();

    (
      resolve_factory(unresolved_mark, top_level_mark),
      fold_pass(self.build()),
    )
  }
}

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub fn new(comments: C, plugin_pass: PluginPass, config: &mut StyleXOptionsParams) -> Self {
    let stylex_imports = fill_stylex_imports(&Some(config));

    let mut state = StateManager::new(config.clone().into());

    state.options.import_sources = stylex_imports.into_iter().collect();

    state.set_plugin_pass(plugin_pass);

    StyleXTransform {
      comments,
      props_declaration: None,
      state,
    }
  }

  /// Start building a test transform using the builder / `With` pattern.
  pub fn test(comments: C) -> StyleXTransformBuilder<C> {
    StyleXTransformBuilder {
      comments,
      plugin_pass: PluginPass::default(),
      config: None,
      runtime_injection: false,
    }
  }

  pub(crate) fn process_declaration(&mut self, call_expr: &mut CallExpr) -> Option<(Id, String)> {
    if let Callee::Expr(callee) = &mut call_expr.callee {
      match callee.as_ref() {
        Expr::Ident(ident) => {
          if self
            .state
            .is_stylex_import_for_current_cycle(ident.sym.as_ref())
          {
            increase_ident_count(&mut self.state, ident);
            return Some((ident.to_id(), ident.sym.to_string()));
          }
        },
        Expr::Member(member) => {
          if let (Expr::Ident(obj_ident), MemberProp::Ident(prop_ident)) =
            (member.obj.as_ref(), &member.prop)
            && self
              .state
              .is_stylex_import_for_current_cycle(obj_ident.sym.as_ref())
          {
            return Some((obj_ident.to_id(), prop_ident.sym.to_string()));
          }
        },
        _ => {},
      }
    }

    None
  }

  pub(crate) fn transform_call_expression(&mut self, expr: &mut Expr) -> Option<Expr> {
    if let Expr::Call(ex) = expr {
      let declaration = self.process_declaration(ex);

      if declaration.is_some() {
        return self.transform_stylex_fns(ex);
      }
    }

    None
  }

  pub(crate) fn get_call_var_name(
    &mut self,
    call: &CallExpr,
  ) -> (Option<String>, Option<VarDeclarator>) {
    let mut var_name: Option<String> = None;

    let parent_var_decl = self
      .state
      .declarations
      .iter()
      .find(|decl| match &decl.init {
        Some(init) => {
          if let Expr::Call(init_call) = init.as_ref() {
            init_call.eq_ignore_span(call)
          } else {
            false
          }
        },
        _ => false,
      })
      .cloned();

    if let Some(ref parent_var_decl) = parent_var_decl
      && let Some(ident) = parent_var_decl.name.as_ident()
    {
      var_name = Some(ident.sym.to_string());
    }

    (var_name, parent_var_decl)
  }
}

fn fill_stylex_imports(config: &Option<&mut StyleXOptionsParams>) -> FxHashSet<ImportSources> {
  let mut stylex_imports = fill_stylex_imports_default();

  if let Some(stylex_imports_extends) = match config {
    Some(config) => config.import_sources.clone(),
    None => None,
  } {
    stylex_imports.extend(stylex_imports_extends)
  }

  stylex_imports
}

fn fill_stylex_imports_default() -> FxHashSet<ImportSources> {
  let mut stylex_imports = FxHashSet::default();
  stylex_imports.insert(ImportSources::Regular("stylex".to_string()));
  stylex_imports.insert(ImportSources::Regular("@stylexjs/stylex".to_string()));
  stylex_imports
}

fn fill_stylex_imports_from_params(config: &StyleXOptionsParams) -> FxHashSet<ImportSources> {
  let mut stylex_imports = fill_stylex_imports_default();

  if let Some(ref extends) = config.import_sources {
    stylex_imports.extend(extends.clone());
  }

  stylex_imports
}

#[warn(clippy::extra_unused_type_parameters)]
fn resolve_factory(unresolved_mark: Mark, top_level_mark: Mark) -> impl Pass {
  resolver(unresolved_mark, top_level_mark, true)
}

#[warn(clippy::extra_unused_type_parameters)]
fn _typescript_factory(unresolved_mark: Mark, top_level_mark: Mark) -> impl Pass {
  strip(unresolved_mark, top_level_mark)
}
