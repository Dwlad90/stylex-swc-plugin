use core::panic;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use dashmap::DashMap;
use swc_core::{
    common::{comments::Comments, FileName},
    ecma::ast::{CallExpr, Callee, Expr, Id, Ident, MemberProp, VarDeclarator},
};

use crate::{
    shared::{
        enums::ModuleCycle,
        structures::{
            meta_data::MetaData,
            named_import_source::{ImportSources, RuntimeInjection},
            plugin_pass::PluginPass,
            state_manager::StateManager,
            stylex_options::StyleXOptions,
        },
        utils::common::{extract_filename_from_path, increase_ident_count},
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
    css_output: DashMap<String, Vec<MetaData>>,
    pub(crate) state: StateManager,
}

impl<C> ModuleTransformVisitor<C>
where
    C: Comments,
{
    pub(crate) fn new(comments: C, plugin_pass: PluginPass, config: StyleXOptionsParams) -> Self {
        let stylex_imports = fill_stylex_imports(&Option::Some(config.clone()));

        let mut state = StateManager::new(config.into());

        state.stylex_import = stylex_imports;

        state._state = plugin_pass;

        ModuleTransformVisitor {
            comments,
            cycle: ModuleCycle::Initializing,
            props_declaration: Option::None,
            css_output: DashMap::new(),
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
        plugin_pass: PluginPass,
        config: Option<StyleXOptionsParams>,
    ) -> Self {
        let stylex_imports = fill_stylex_imports(&config);

        let mut state = match &config {
            Some(config) => StateManager::new(config.clone().into()),
            None => {
                let mut config = StyleXOptions::default();

                config.runtime_injection = RuntimeInjection::Boolean(true);
                config.treeshake_compensation = Option::Some(true);

                StateManager::new(config.into())
            }
        };

        // state.stylex_import = stylex_imports.clone();
        state.options.import_sources = stylex_imports.into_iter().collect();

        let plugin_pass = plugin_pass.clone();

        state._state = plugin_pass;

        ModuleTransformVisitor {
            comments,
            cycle: ModuleCycle::Initializing,
            props_declaration: Option::None,
            css_output: DashMap::new(),
            state,
        }
    }

    pub(crate) fn process_declaration(&mut self, call_expr: &CallExpr) -> Option<(Id, String)> {
        let stylex_imports = self.state.stylex_import_stringified();

        match &mut call_expr.callee.clone() {
            Callee::Expr(callee) => match callee.as_ref() {
                Expr::Ident(ident) => {
                    let ident_id = ident.to_id();

                    if stylex_imports.contains(&ident.sym.to_string())
                        || self.state.stylex_create_import.contains(&ident.to_id())
                    {
                        increase_ident_count(&mut self.state, &ident);

                        return Option::Some((ident_id.clone(), format!("{}", ident.sym)));
                    }
                }
                Expr::Member(member) => match member.obj.as_ref() {
                    Expr::Ident(ident) => {
                        let ident_id = ident.to_id();

                        if stylex_imports.contains(&ident.sym.to_string())
                            || self.state.stylex_create_import.contains(&ident.to_id())
                        {
                            match member.prop.clone() {
                                MemberProp::Ident(ident) => {
                                    increase_ident_count(&mut self.state, &ident);

                                    return Option::Some((
                                        ident_id.clone(),
                                        format!("{}", ident.sym),
                                    ));
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

    pub(crate) fn transform_call_expression(&mut self, expr: &mut Expr) -> Option<Expr> {
        match expr {
            Expr::Call(ex) => {
                let declaration = self.process_declaration(&ex);

                println!("!!!declaration: {:?}\n\n", declaration);

                if let Some(_) = declaration {
                    let value = self.transform_call_expression_to_stylex_expr(&ex);
                    // let value = if self.state.options.runtime_injection.is_some() {
                    // } else {
                    //     self.transform_call_expression_to_css_map_expr(&ex)
                    // };

                    match value {
                        Some(value) => {
                            return Some(value);
                        }
                        None => {}
                    }
                }
            }
            _ => {}
        }
        None
    }

    pub(crate) fn push_to_css_output(&mut self, var_name: String, metadata: MetaData) {
        let mut value = self.css_output.entry(var_name).or_insert_with(Vec::new);

        if !value
            .iter()
            .any(|item| item.get_class_name() == metadata.get_class_name())
        {
            value.push(metadata);
        }
    }
}

fn fill_stylex_imports(config: &Option<StyleXOptionsParams>) -> HashSet<ImportSources> {
    let mut stylex_imports = HashSet::new();

    stylex_imports.insert(ImportSources::Regular("stylex".to_string()));
    stylex_imports.insert(ImportSources::Regular("@stylexjs/stylex".to_string()));

    if let Some(stylex_imports_extends) = match config {
        Some(ref config) => config.import_sources.clone(),
        None => Option::None,
    } {
        stylex_imports.extend(stylex_imports_extends)
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
