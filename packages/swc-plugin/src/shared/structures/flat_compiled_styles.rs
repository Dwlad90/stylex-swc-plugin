use indexmap::IndexMap;

use crate::shared::enums::FlatCompiledStylesValue;

pub(crate) type FlatCompiledStyles = IndexMap<String, FlatCompiledStylesValue>;
