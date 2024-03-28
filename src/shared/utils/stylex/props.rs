use indexmap::IndexMap;

use crate::{
    shared::{
        enums::FnResult, structures::flat_compiled_styles::FlatCompiledStylesValue,
        utils::stylex::js_to_expr::NestedStringObject,
    },
    transform::styleq::styleq::{styleq, StyleQResult},
};

use super::parse_nallable_style::ResolvedArg;

pub(crate) fn props(styles: &Vec<ResolvedArg>) -> Option<FnResult> {
    dbg!(&styles);
    let StyleQResult {
        class_name,
        inline_style,
    } = styleq(styles);

    let mut props_map: IndexMap<String, FlatCompiledStylesValue> = IndexMap::new();

    if !class_name.is_empty() {
        props_map.insert(
            "className".to_string(),
            FlatCompiledStylesValue::String(class_name),
        );
    }

    if let Some(inline_style) = inline_style {
        todo!("Implement inline style");
        props_map.extend(inline_style);
    }

    return Some(FnResult::Props(
        NestedStringObject::FlatCompiledStylesValues(props_map),
    ));
}
