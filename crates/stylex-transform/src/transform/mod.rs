use rustc_hash::FxHashSet;
use swc_core::{
  atoms::Atom,
  common::{Mark, comments::Comments},
  ecma::{
    ast::{CallExpr, Callee, Expr, Id, MemberProp, Pass, VarDeclarator},
    transforms::{base::resolver, typescript::strip},
    utils::drop_span,
    visit::fold_pass,
  },
};

use crate::shared::{structures::state_manager::StateManager, utils::common::increase_ident_count};
use stylex_enums::core::TransformationCycle;
use stylex_structures::{
  named_import_source::{ImportSources, RuntimeInjection},
  plugin_pass::PluginPass,
  stylex_options::{CheckModuleResolution, StyleXOptions, StyleXOptionsParams},
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

  pub fn with_options(mut self, config: &mut StyleXOptionsParams) -> Self {
    self.config = Some(config.clone());
    self
  }

  pub fn with_runtime_injection(mut self) -> Self {
    self.runtime_injection = true;
    self
  }

  pub fn build(self) -> StyleXTransform<C> {
    let stylex_imports = match &self.config {
      Some(config) => fill_stylex_imports_from_params(config),
      None => fill_stylex_imports_default(),
    };

    let mut state = match self.config {
      Some(mut config) => {
        if self.runtime_injection {
          config.runtime_injection = Some(RuntimeInjection::Boolean(true));
          config.treeshake_compensation = Some(true);
        }
        StateManager::new(config.into())
      },
      None => {
        let config = StyleXOptions {
          runtime_injection: if self.runtime_injection {
            RuntimeInjection::Boolean(true)
          } else {
            RuntimeInjection::Boolean(false)
          },
          treeshake_compensation: true,
          class_name_prefix: "x".to_string(),
          unstable_module_resolution: CheckModuleResolution::Haste(
            StyleXOptions::get_haste_module_resolution(None),
          ),
          ..Default::default()
        };
        StateManager::new(config)
      },
    };

    state.options.import_sources = stylex_imports.into_iter().collect();
    state._state = self.plugin_pass;

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

    state._state = plugin_pass;

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

  #[deprecated(note = "Use StyleXTransform::test(comments).with_runtime_injection().build()")]
  pub fn new_test_force_runtime_injection(
    comments: C,
    plugin_pass: PluginPass,
    config: Option<&mut StyleXOptionsParams>,
  ) -> Self {
    let mut builder = Self::test(comments).with_pass(plugin_pass).with_runtime_injection();

    if let Some(config) = config {
      builder = builder.with_options(config);
    }

    builder.build()
  }

  #[deprecated(
    note = "Use StyleXTransform::test(comments).with_runtime_injection().into_pass()"
  )]
  pub fn new_test_force_runtime_injection_with_pass(
    comments: C,
    plugin_pass: PluginPass,
    config: Option<&mut StyleXOptionsParams>,
  ) -> impl Pass + use<C> {
    let mut builder = Self::test(comments).with_pass(plugin_pass).with_runtime_injection();

    if let Some(config) = config {
      builder = builder.with_options(config);
    }

    builder.into_pass()
  }

  #[deprecated(note = "Use StyleXTransform::test(comments).build()")]
  pub fn new_test(
    comments: C,
    plugin_pass: PluginPass,
    config: Option<&mut StyleXOptionsParams>,
  ) -> Self {
    let mut builder = Self::test(comments).with_pass(plugin_pass);

    if let Some(config) = config {
      builder = builder.with_options(config);
    }

    builder.build()
  }

  #[deprecated(note = "Use StyleXTransform::test(comments).into_pass()")]
  pub fn new_test_with_pass(
    comments: C,
    plugin_pass: PluginPass,
    config: Option<&mut StyleXOptionsParams>,
  ) -> impl Pass + use<C> {
    let mut builder = Self::test(comments).with_pass(plugin_pass);

    if let Some(config) = config {
      builder = builder.with_options(config);
    }

    builder.into_pass()
  }

  fn is_stylex_import(&self, ident_sym: &str) -> bool {
    let state = &self.state;
    let stylex_imports = state.stylex_import_stringified();

    let ident_sym = Atom::from(ident_sym);

    let ident_str = ident_sym.as_str();

    if stylex_imports.iter().any(|s| s == ident_str) {
      return true;
    }

    match state.cycle {
      TransformationCycle::TransformEnter => {
        state.stylex_create_import.contains(&ident_sym)
          || state.stylex_define_vars_import.contains(&ident_sym)
          || state.stylex_define_consts_import.contains(&ident_sym)
          || state.stylex_define_marker_import.contains(&ident_sym)
          || state.stylex_create_theme_import.contains(&ident_sym)
          || state.stylex_position_try_import.contains(&ident_sym)
          || state.stylex_keyframes_import.contains(&ident_sym)
          || state.stylex_first_that_works_import.contains(&ident_sym)
          || state.stylex_types_import.contains(&ident_sym)
          || state.stylex_default_marker_import.contains(&ident_sym)
          || state.stylex_when_import.contains(&ident_sym)
      },
      TransformationCycle::TransformExit => {
        state.stylex_attrs_import.contains(&ident_sym)
          || state.stylex_props_import.contains(&ident_sym)
      },
      _ => false,
    }
  }

  pub(crate) fn process_declaration(&mut self, call_expr: &mut CallExpr) -> Option<(Id, String)> {
    if let Callee::Expr(callee) = &mut call_expr.callee {
      match callee.as_ref() {
        Expr::Ident(ident) => {
          if self.is_stylex_import(ident.sym.as_ref()) {
            increase_ident_count(&mut self.state, ident);
            return Some((ident.to_id(), ident.sym.to_string()));
          }
        },
        Expr::Member(member) => {
          if let (Expr::Ident(obj_ident), MemberProp::Ident(prop_ident)) =
            (member.obj.as_ref(), &member.prop)
            && self.is_stylex_import(obj_ident.sym.as_ref())
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
            init_call == &drop_span(call.clone())
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
