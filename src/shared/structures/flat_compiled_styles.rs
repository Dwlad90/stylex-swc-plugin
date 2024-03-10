use indexmap::IndexMap;


use super::included_style::IncludedStyle;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FlatCompiledStylesValue {
    String(String),
    Null,
    IncludedStyle(IncludedStyle),
    Bool(bool),
}

pub(crate) type FlatCompiledStyles = IndexMap<String, FlatCompiledStylesValue>;
