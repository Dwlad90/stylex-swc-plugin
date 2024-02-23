use std::collections::{HashMap, HashSet};
use std::option::Option;

use serde::Deserialize;
use swc_core::ecma::ast::Id;

use super::named_import_source::ImportSources;
use super::plugin_pass::PluginPass;
use super::stylex_options::StyleXOptions;
use super::stylex_state_options::StyleXStateOptions;

#[derive(Deserialize, Clone, Debug)]
pub struct StateManager {
    // _state: PluginPass, // Assuming PluginPass is a struct in your code

    // Imports
    pub(crate) import_paths: HashSet<String>,
    pub(crate) stylex_import: HashSet<ImportSources>,
    pub(crate) stylex_props_import: HashSet<String>,
    pub(crate) stylex_attrs_import: HashSet<String>,
    pub(crate) stylex_create_import: HashSet<Id>,
    pub(crate) stylex_include_import: HashSet<String>,
    pub(crate) stylex_first_that_works_import: HashSet<String>,
    pub(crate) stylex_keyframes_import: HashSet<String>,
    pub(crate) stylex_define_vars_import: HashSet<String>,
    pub(crate) stylex_create_theme_import: HashSet<String>,
    pub(crate) stylex_types_import: HashSet<String>,
    pub(crate) inject_import_inserted: Option<String>, // Assuming this is a string identifier

    // `stylex.create` calls
    // style_map: HashMap<String, CompiledNamespaces>, // Assuming CompiledNamespaces is a struct in your code
    // style_vars: HashMap<String, NodePath>,          // Assuming NodePath is a struct in your code

    // results of `stylex.create` calls that should be kept
    pub(crate) style_vars_to_keep: HashSet<(String, bool, String, bool, Vec<String>)>,

    pub(crate) in_style_x_create: bool,

    pub(crate) options: StyleXStateOptions, // Assuming StyleXStateOptions is a struct in your code
}
impl StateManager {
    pub fn new(stylex_options: StyleXOptions) -> Self {
        let options: StyleXStateOptions = StyleXStateOptions::from(stylex_options);
        Self {
            // _state: state,
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
            // style_map: HashMap::new(),
            // style_vars: HashMap::new(),
            style_vars_to_keep: HashSet::new(),
            in_style_x_create: false,
            options, // Assuming StyleXStateOptions has a new function
        }
    }
}
