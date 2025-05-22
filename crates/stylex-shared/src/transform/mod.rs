use rustc_hash::FxHashSet;
use swc_core::{
  common::{Mark, comments::Comments},
  ecma::{
    ast::{CallExpr, Callee, Expr, Id, MemberProp, Pass, VarDeclarator},
    transforms::base::resolver,
    utils::drop_span,
    visit::fold_pass,
  },
};

use crate::{
  StyleXOptionsParams,
  shared::{
    enums::core::TransformationCycle,
    structures::{
      named_import_source::{ImportSources, RuntimeInjection},
      plugin_pass::PluginPass,
      state_manager::StateManager,
      stylex_options::{CheckModuleResolution, StyleXOptions},
    },
    utils::common::increase_ident_count,
  },
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

impl<C> StyleXTransform<C>
where
  C: Comments,
{
  pub fn new(comments: C, plugin_pass: PluginPass, config: &mut StyleXOptionsParams) -> Self {
    let stylex_imports = fill_stylex_imports(&Some(config));

    let mut state = StateManager::new(config.clone().into());

    state.stylex_import.clone_from(&stylex_imports);

    state.options.import_sources = stylex_imports.into_iter().collect();

    state._state = plugin_pass;

    StyleXTransform {
      comments,
      props_declaration: None,
      state,
    }
  }

  pub fn new_test_force_runtime_injection(
    comments: C,
    plugin_pass: PluginPass,
    config: Option<&mut StyleXOptionsParams>,
  ) -> Self {
    let stylex_imports = fill_stylex_imports(&config);

    let mut state = match config {
      Some(config) => {
        config.runtime_injection = Some(true);
        config.treeshake_compensation = Some(true);

        StateManager::new(config.clone().into())
      }
      None => {
        let config = StyleXOptions {
          runtime_injection: RuntimeInjection::Boolean(true),
          treeshake_compensation: Some(true),
          unstable_module_resolution: CheckModuleResolution::Haste(
            StyleXOptions::get_haste_module_resolution(None),
          ),
          ..Default::default()
        };

        StateManager::new(config)
      }
    };

    state.options.import_sources = stylex_imports.into_iter().collect();

    state._state = plugin_pass;

    StyleXTransform {
      comments,
      props_declaration: None,
      state,
    }
  }

  pub fn new_test_force_runtime_injection_with_pass(
    comments: C,
    plugin_pass: PluginPass,
    config: Option<&mut StyleXOptionsParams>,
  ) -> impl Pass + use<C> {
    (
      resolve_factory(),
      fold_pass(Self::new_test_force_runtime_injection(
        comments,
        plugin_pass,
        config,
      )),
    )
  }

  pub fn new_test(
    comments: C,
    plugin_pass: PluginPass,
    config: Option<&mut StyleXOptionsParams>,
  ) -> Self {
    let stylex_imports = fill_stylex_imports(&config);

    let mut state = match config {
      Some(config) => StateManager::new(config.clone().into()),
      None => {
        let config = StyleXOptions {
          runtime_injection: RuntimeInjection::Boolean(false),
          treeshake_compensation: Some(true),
          class_name_prefix: "x".to_string(),
          unstable_module_resolution: CheckModuleResolution::Haste(
            StyleXOptions::get_haste_module_resolution(None),
          ),
          ..Default::default()
        };

        StateManager::new(config)
      }
    };

    state.options.import_sources = stylex_imports.into_iter().collect();

    state._state = plugin_pass;

    StyleXTransform {
      comments,
      props_declaration: None,
      state,
    }
  }
  pub fn new_test_with_pass(
    comments: C,
    plugin_pass: PluginPass,
    config: Option<&mut StyleXOptionsParams>,
  ) -> impl Pass + use<C> {
    (
      resolve_factory(),
      fold_pass(Self::new_test(comments, plugin_pass, config)),
    )
  }

  pub(crate) fn process_declaration(&mut self, call_expr: &mut CallExpr) -> Option<(Id, String)> {
    let state = &self.state;

    let stylex_imports = state.stylex_import_stringified();
    if let Callee::Expr(callee) = &mut call_expr.callee {
      match callee.as_ref() {
        Expr::Ident(ident) => {
          let ident_id = ident.to_id();

          if stylex_imports.contains(&ident.sym.to_string())
            || (state.cycle == TransformationCycle::TransformEnter
              && (state.stylex_create_import.contains(&ident.sym))
              || state.stylex_props_import.contains(&ident.sym)
              || state.stylex_keyframes_import.contains(&ident.sym)
              || state.stylex_first_that_works_import.contains(&ident.sym)
              || state.stylex_types_import.contains(&ident.sym)
              || state.stylex_create_theme_import.contains(&ident.sym)
              || state.stylex_define_vars_import.contains(&ident.sym)
              || state.stylex_attrs_import.contains(&ident.sym))
          {
            increase_ident_count(&mut self.state, ident);

            return Some((ident_id.clone(), format!("{}", ident.sym)));
          }
        }
        Expr::Member(member) => {
          if let Expr::Ident(ident) = member.obj.as_ref() {
            let ident_id = ident.to_id();

            if stylex_imports.contains(&ident.sym.to_string())
              || (state.cycle == TransformationCycle::TransformEnter
                && (state.stylex_create_import.contains(&ident.sym))
                || state.stylex_props_import.contains(&ident.sym)
                || state.stylex_keyframes_import.contains(&ident.sym)
                || state.stylex_first_that_works_import.contains(&ident.sym)
                || state.stylex_create_theme_import.contains(&ident.sym)
                || state.stylex_types_import.contains(&ident.sym)
                || state.stylex_define_vars_import.contains(&ident.sym)
                || state.stylex_attrs_import.contains(&ident.sym))
            {
              if let MemberProp::Ident(ident) = &member.prop {
                return Some((ident_id.clone(), format!("{}", ident.sym)));
              }
            }
          }
        }
        _ => {}
      }
    }

    None
  }

  pub(crate) fn transform_call_expression(&mut self, expr: &mut Expr) -> Option<Expr> {
    if let Expr::Call(ex) = expr {
      let declaration = self.process_declaration(ex);

      if declaration.is_some() {
        return self.transform_call_expression_to_stylex_expr(ex);
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
        }
        _ => false,
      })
      .cloned();

    if let Some(ref parent_var_decl) = parent_var_decl {
      if let Some(ident) = parent_var_decl.name.as_ident() {
        var_name = Some(ident.sym.to_string());
      }
    }

    (var_name, parent_var_decl)
  }
}

fn fill_stylex_imports(config: &Option<&mut StyleXOptionsParams>) -> FxHashSet<ImportSources> {
  let mut stylex_imports = FxHashSet::default();

  stylex_imports.insert(ImportSources::Regular("stylex".to_string()));
  stylex_imports.insert(ImportSources::Regular("@stylexjs/stylex".to_string()));

  if let Some(stylex_imports_extends) = match config {
    Some(config) => config.import_sources.clone(),
    None => None,
  } {
    stylex_imports.extend(stylex_imports_extends)
  }

  stylex_imports
}

#[warn(clippy::extra_unused_type_parameters)]
fn resolve_factory() -> impl Pass {
  let unresolved_mark = Mark::new();
  let top_level_mark = Mark::new();

  resolver(unresolved_mark, top_level_mark, true)
}
