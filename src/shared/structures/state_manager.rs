use std::collections::{HashMap, HashSet};
use std::option::Option;

use indexmap::IndexMap;
use swc_core::ecma::ast::{Id, VarDeclarator};

use crate::shared::enums::{StyleVarsToKeep, TopLevelExpression};
use crate::shared::utils::common::extract_filename_from_path;

use super::flat_compiled_styles::FlatCompiledStylesValue;
use super::named_import_source::ImportSources;
use super::plugin_pass::PluginPass;
use super::stylex_options::StyleXOptions;
use super::stylex_state_options::StyleXStateOptions;

#[derive(Clone, Debug)]
pub struct StateManager {
    pub(crate) _state: PluginPass, // Assuming PluginPass is a struct in your code

    // Imports
    pub(crate) import_paths: HashSet<String>,
    pub(crate) stylex_import: HashSet<ImportSources>,
    pub(crate) stylex_props_import: HashSet<Id>,
    pub(crate) stylex_attrs_import: HashSet<Id>,
    pub(crate) stylex_create_import: HashSet<Id>,
    pub(crate) stylex_include_import: HashSet<Id>,
    pub(crate) stylex_first_that_works_import: HashSet<Id>,
    pub(crate) stylex_keyframes_import: HashSet<Id>,
    pub(crate) stylex_define_vars_import: HashSet<Id>,
    pub(crate) stylex_create_theme_import: HashSet<Id>,
    pub(crate) stylex_types_import: HashSet<Id>,
    pub(crate) inject_import_inserted: Option<String>, // Assuming this is a string identifier

    pub(crate) declarations: Vec<VarDeclarator>,
    pub(crate) top_level_expressions: Vec<TopLevelExpression>,
    pub(crate) var_decl_count_map: HashMap<Id, i8>,

    // `stylex.create` calls
    pub(crate) style_map:
        HashMap<String, IndexMap<String, IndexMap<String, FlatCompiledStylesValue>>>, // Assuming CompiledNamespaces is a struct in your code
    pub(crate) style_vars: HashMap<String, VarDeclarator>, // Assuming NodePath is a struct in your code

    // results of `stylex.create` calls that should be kept
    pub(crate) style_vars_to_keep: HashSet<StyleVarsToKeep>,

    pub(crate) in_style_x_create: bool,

    pub(crate) options: StyleXStateOptions, // Assuming StyleXStateOptions is a struct in your code
}
impl StateManager {
    pub fn new(stylex_options: StyleXOptions) -> Self {
        let options: StyleXStateOptions = StyleXStateOptions::from(stylex_options);
        Self {
            _state: PluginPass::default(),
            import_paths: HashSet::new(),
            stylex_import: HashSet::new(),
            stylex_props_import: HashSet::new(),
            stylex_attrs_import: HashSet::new(),
            stylex_create_import: HashSet::new(),
            stylex_include_import: HashSet::new(),
            stylex_first_that_works_import: HashSet::new(),
            stylex_keyframes_import: HashSet::new(),
            stylex_define_vars_import: HashSet::new(),
            stylex_create_theme_import: HashSet::new(),
            stylex_types_import: HashSet::new(),
            inject_import_inserted: None,
            style_map: HashMap::new(),
            style_vars: HashMap::new(),
            style_vars_to_keep: HashSet::new(),

            declarations: vec![],
            top_level_expressions: vec![],
            var_decl_count_map: HashMap::new(),

            in_style_x_create: false,
            options, // Assuming StyleXStateOptions has a new function
        }
    }

    pub fn import_as(&self, import: &str) -> Option<String> {
        for import_source in &self.options.import_sources {
            match import_source {
                ImportSources::Regular(_) => {}
                ImportSources::Named(named) => {
                    if named.from.eq(import) {
                        return Option::Some(named.r#as.clone());
                    }
                }
            }
        }

        Option::None
    }

    pub fn import_sources(&self) -> Vec<ImportSources> {
        self.options.import_sources.clone()
    }

    pub fn import_sources_stringified(&self) -> Vec<String> {
        self.options
            .import_sources
            .clone()
            .into_iter()
            .map(|import_source| match import_source {
                ImportSources::Regular(regular) => regular,
                ImportSources::Named(named) => named.from,
            })
            .collect()
    }

    pub fn stylex_import_stringified(&self) -> Vec<String> {
        self.stylex_import
            .clone()
            .into_iter()
            .map(|import_source| match import_source {
                ImportSources::Regular(regular) => regular,
                ImportSources::Named(named) => named.r#as,
            })
            .collect()
    }

    pub(crate) fn is_test(&self) -> bool {
        self.options.test
    }

    pub(crate) fn is_dev(&self) -> bool {
        self.options.dev
    }

    pub(crate) fn gen_conditional_classes(&self) -> bool {
        self.options.gen_conditional_classes
    }

    pub(crate) fn get_filename(&self) -> String {
        extract_filename_from_path(self._state.filename.clone())
    }
    // pub(crate) fn css_vars(&self) -> HashMap<String, String> {
    //     self.options.defined_stylex_css_variables.clone()
    // }
}
