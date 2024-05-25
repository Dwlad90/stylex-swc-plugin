use std::collections::HashSet;

use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Callee, Expr, Id, MemberProp, VarDeclarator},
};

use crate::{
  shared::{
    enums::core::ModuleCycle,
    structures::{
      named_import_source::{ImportSources, RuntimeInjection},
      plugin_pass::PluginPass,
      state_manager::StateManager,
      stylex_options::StyleXOptions,
    },
    utils::common::increase_ident_count,
  },
  StyleXOptionsParams,
};

mod fold;
pub(crate) mod styleq;
pub(crate) mod stylex;

pub struct ModuleTransformVisitor<C>
where
  C: Comments,
{
  comments: C,
  cycle: ModuleCycle,
  props_declaration: Option<Id>,
  pub(crate) state: Box<StateManager>,
}

impl<C> ModuleTransformVisitor<C>
where
  C: Comments,
{
  pub(crate) fn new(
    comments: C,
    plugin_pass: Box<PluginPass>,
    config: &mut StyleXOptionsParams,
  ) -> Self {
    let stylex_imports = fill_stylex_imports(&Option::Some(config));

    let mut state = Box::new(StateManager::new(config.clone().into()));

    state.stylex_import.clone_from(&stylex_imports);

    state.options.import_sources = stylex_imports
      .into_iter()
      .map(|stylex_import| *stylex_import)
      .collect();

    state._state = plugin_pass;

    ModuleTransformVisitor {
      comments,
      cycle: ModuleCycle::Initializing,
      props_declaration: Option::None,
      state,
    }
  }

  pub fn new_test_styles(
    comments: C,
    plugin_pass: &PluginPass,
    config: Option<&mut StyleXOptionsParams>,
  ) -> Self {
    let stylex_imports = fill_stylex_imports(&config);

    let mut state = Box::new(match config {
      Some(config) => {
        config.runtime_injection = Option::Some(true);
        config.treeshake_compensation = Option::Some(true);

        StateManager::new(config.clone().clone().into())
      }
      None => {
        let config = StyleXOptions {
          runtime_injection: RuntimeInjection::Boolean(true),
          treeshake_compensation: Option::Some(true),
          ..Default::default()
        };

        StateManager::new(config)
      }
    });

    state.options.import_sources = stylex_imports
      .into_iter()
      .map(|stylex_import| *stylex_import)
      .collect();

    let plugin_pass = Box::new(plugin_pass.clone());

    state._state = plugin_pass;

    ModuleTransformVisitor {
      comments,
      cycle: ModuleCycle::Initializing,
      props_declaration: Option::None,
      state,
    }
  }

  pub fn new_test(
    comments: C,
    plugin_pass: &PluginPass,
    config: Option<&mut StyleXOptionsParams>,
  ) -> Self {
    let stylex_imports = fill_stylex_imports(&config);

    let mut state = Box::new(match &config {
      Some(config) => StateManager::new((*config).clone().into()),
      None => {
        let config = StyleXOptions {
          runtime_injection: RuntimeInjection::Boolean(false),
          treeshake_compensation: Option::Some(true),
          class_name_prefix: "x".to_string(),
          ..Default::default()
        };

        StateManager::new(config)
      }
    });

    state.options.import_sources = stylex_imports.into_iter().map(|s_i| *s_i).collect();

    let plugin_pass = plugin_pass.clone();

    state._state = Box::new(plugin_pass);

    ModuleTransformVisitor {
      comments,
      cycle: ModuleCycle::Initializing,
      props_declaration: Option::None,
      state,
    }
  }

  pub(crate) fn process_declaration(&mut self, call_expr: &CallExpr) -> Option<(Id, String)> {
    let stylex_imports = self.state.stylex_import_stringified();
    if let Callee::Expr(callee) = &mut call_expr.callee.clone() {
      match callee.as_ref() {
        Expr::Ident(ident) => {
          let ident_id = ident.to_id();

          if stylex_imports.contains(&ident.sym.to_string())
            || (self.cycle == ModuleCycle::TransformEnter
              && (self.state.stylex_create_import.contains(&ident.to_id()))
              || self.state.stylex_props_import.contains(&ident.to_id())
              || self.state.stylex_keyframes_import.contains(&ident.to_id())
              || self
                .state
                .stylex_first_that_works_import
                .contains(&ident.to_id())
              || self.state.stylex_include_import.contains(&ident.to_id())
              || self.state.stylex_types_import.contains(&ident.to_id())
              || self
                .state
                .stylex_create_theme_import
                .contains(&ident.to_id())
              || self
                .state
                .stylex_define_vars_import
                .contains(&ident.to_id())
              || self.state.stylex_attrs_import.contains(&ident.to_id()))
          {
            increase_ident_count(&mut self.state, ident);

            return Option::Some((ident_id.clone(), format!("{}", ident.sym)));
          }
        }
        Expr::Member(member) => {
          if let Expr::Ident(ident) = member.obj.as_ref() {
            let ident_id = ident.to_id();

            if stylex_imports.contains(&ident.sym.to_string())
              || (self.cycle == ModuleCycle::TransformEnter
                && (self.state.stylex_create_import.contains(&ident.to_id()))
                || self.state.stylex_props_import.contains(&ident.to_id())
                || self.state.stylex_keyframes_import.contains(&ident.to_id())
                || self
                  .state
                  .stylex_first_that_works_import
                  .contains(&ident.to_id())
                || self.state.stylex_include_import.contains(&ident.to_id())
                || self
                  .state
                  .stylex_create_theme_import
                  .contains(&ident.to_id())
                || self.state.stylex_types_import.contains(&ident.to_id())
                || self
                  .state
                  .stylex_define_vars_import
                  .contains(&ident.to_id())
                || self.state.stylex_attrs_import.contains(&ident.to_id()))
            {
              if let MemberProp::Ident(ident) = member.prop.clone() {
                return Option::Some((ident_id.clone(), format!("{}", ident.sym)));
              }
            }
          }
        }
        _ => {}
      }
    }

    Option::None
  }

  pub(crate) fn transform_call_expression(&mut self, expr: &Expr) -> Option<Expr> {
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
  ) -> (Option<String>, Option<Box<VarDeclarator>>) {
    let mut var_name: Option<String> = Option::None;

    let parent_var_decl = self
      .state
      .declarations
      .clone()
      .into_iter()
      .find(|decl| {
        decl
          .init
          .as_ref()
          .unwrap()
          .eq(&Box::new(Expr::Call(call.clone())))
      })
      .map(Box::new);

    if let Some(parent_var_decl) = &parent_var_decl {
      if let Some(ident) = parent_var_decl.name.as_ident() {
        var_name = Option::Some(ident.sym.clone().to_string());
      }
    }

    (var_name, parent_var_decl)
  }
}

fn fill_stylex_imports(config: &Option<&mut StyleXOptionsParams>) -> HashSet<Box<ImportSources>> {
  let mut stylex_imports = HashSet::new();

  stylex_imports.insert(Box::new(ImportSources::Regular("stylex".to_string())));
  stylex_imports.insert(Box::new(ImportSources::Regular(
    "@stylexjs/stylex".to_string(),
  )));

  if let Some(stylex_imports_extends) = match config {
    Some(ref config) => config.import_sources.clone(),
    None => Option::None,
  } {
    stylex_imports.extend(
      stylex_imports_extends
        .into_iter()
        .map(Box::new)
        .collect::<Vec<Box<ImportSources>>>(),
    )
  }

  stylex_imports
}
