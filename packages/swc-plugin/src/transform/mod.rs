use std::collections::HashSet;

use dashmap::DashMap;
use swc_core::{
  common::comments::Comments,
  ecma::ast::{CallExpr, Callee, Expr, Id, MemberProp, VarDeclarator},
};

use crate::{
  shared::{
    constants::common::DEFAULT_INJECT_PATH,
    enums::ModuleCycle,
    structures::{
      meta_data::MetaData,
      named_import_source::{ImportSources, RuntimeInjection, RuntimeInjectionState},
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
  // declaration: Option<Id>,
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

  // pub fn new_test_classname(comments: C, config: Option<StyleXOptionsParams>) -> Self {
  //     panic!("new_test_classname");
  //     let stylex_imports = fill_stylex_imports(&config);
  //     let mut state = StateManager::new(config.unwrap_or(StyleXOptionsParams::default()).into());

  //     state.stylex_import = stylex_imports;

  //     ModuleTransformVisitor {
  //         comments,
  //         cycle: ModuleCycle::Initializing,
  //         props_declaration: Option::None,
  //         css_output: vec![],
  //         state,
  //     }
  // }
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
        let mut config = StyleXOptions::default();

        config.runtime_injection = RuntimeInjection::Boolean(true);
        config.treeshake_compensation = Option::Some(true);

        StateManager::new(config)
      }
    });

    // state.stylex_import = stylex_imports.clone();
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
      Some(config) => StateManager::new((*config).clone().into()), // Convert &&mut StyleXOptionsParams into StyleXOptionsParams
      None => {
        let mut config = StyleXOptions::default();

        config.runtime_injection = RuntimeInjection::Boolean(false);
        config.treeshake_compensation = Option::Some(true);
        config.class_name_prefix = "x".to_string();

        StateManager::new(config)
      }
    });

    // state.stylex_import = stylex_imports.clone();
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

    match &mut call_expr.callee.clone() {
      Callee::Expr(callee) => match callee.as_ref() {
        Expr::Ident(ident) => {
          let ident_id = ident.to_id();

          // dbg!(&ident_id);

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
            increase_ident_count(&mut self.state, &ident);

            return Option::Some((ident_id.clone(), format!("{}", ident.sym)));
          }
        }
        Expr::Member(member) => match member.obj.as_ref() {
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
              match member.prop.clone() {
                MemberProp::Ident(ident) => {
                  increase_ident_count(&mut self.state, &ident);

                  return Option::Some((ident_id.clone(), format!("{}", ident.sym)));
                }
                _ => {}
              }
            }
          }
          _ => {}
        },
        _ => {}
      },
      _ => {}
    }
    Option::None
  }

  pub(crate) fn transform_call_expression(&mut self, expr: &Expr) -> Option<Expr> {
    match expr {
      Expr::Call(ex) => {
        let declaration = self.process_declaration(&ex);

        // dbg!(&declaration, &ex);

        if let Some(_) = declaration {
          let value = self.transform_call_expression_to_stylex_expr(&ex);
          // let value = if self.state.options.runtime_injection.is_some() {
          // } else {
          //     self.transform_call_expression_to_css_map_expr(&ex)
          // };

          // match value {
          //     Some(value) => {
          //         return Some(value);
          //     }
          //     None => {}
          // }

          return value;
        }
      }
      _ => {}
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

// static COUNTER: AtomicUsize = AtomicUsize::new(0);

// fn generate_unique_identifier(name: &str) -> Ident {
//     let mark = Mark::fresh(Mark::root());
//     let count = COUNTER.fetch_add(1, Ordering::SeqCst);
//     let unique_name = format!("_{}_{}", name, count);
//     Ident::new(unique_name.into(), DUMMY_SP.apply_mark(mark))
// }
