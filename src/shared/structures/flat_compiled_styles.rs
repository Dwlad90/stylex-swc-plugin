use indexmap::IndexMap;


use super::included_style::IncludedStyle;


#[derive(Debug, Clone)]
pub(crate) enum FlatCompiledStylesValue {
    String(String),
    Null(Option<String>),
    IncludedStyle(IncludedStyle),
    Bool(bool),
}

pub(crate) type FlatCompiledStyles = IndexMap<String, FlatCompiledStylesValue>;
