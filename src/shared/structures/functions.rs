use std::collections::HashMap;

use swc_core::ecma::ast::Expr;

use super::named_import_source::ImportSources;

type FunctionType = fn(Vec<Expr>) -> Expr;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Function {
    StylexInclude(fn() -> ()),        // replace `()` with the actual types
    StylexFirstThatWorks(fn() -> ()), // replace `()` with the actual types
    Keyframes(fn() -> ()),            // replace `()` with the actual types
    Misc(fn() -> ()),                 // replace `()` with the actual types
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct FunctionConfig {
    pub fn_ptr: FunctionType,
    pub takes_path: bool,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Functions {
    pub(crate) include: FunctionConfig,
    pub(crate) first_that_works: FunctionConfig,
    pub(crate) keyframes: FunctionConfig,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionMap {
    pub identifiers: HashMap<String, FunctionConfig>,
    pub member_expressions: HashMap<ImportSources, HashMap<String, FunctionConfig>>,
}
