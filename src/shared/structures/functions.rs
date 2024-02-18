use std::collections::HashMap;

use serde::Deserialize;

use super::named_import_source::ImportSources;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub(crate) enum Function {
    StylexInclude(fn() -> ()),        // replace `()` with the actual types
    StylexFirstThatWorks(fn() -> ()), // replace `()` with the actual types
    Keyframes(fn() -> ()),            // replace `()` with the actual types
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub(crate) struct FunctionConfig {
    pub(crate) fn_ptr: Function,
    pub(crate) takes_path: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub(crate) struct Functions {
    pub(crate) include: FunctionConfig,
    pub(crate) first_that_works: FunctionConfig,
    pub(crate) keyframes: FunctionConfig,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct FunctionMap {
    pub(crate) identifiers: HashMap<String, FunctionConfig>,
    pub(crate) member_expressions: HashMap<ImportSources, Functions>,
}
