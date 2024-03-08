use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use swc_core::ecma::ast::{BinaryOp, Expr, Id, KeyValueProp, VarDeclarator};

use crate::shared::{
    constants::{
        self, messages::ILLEGAL_PROP_ARRAY_VALUE, long_hand_logical::LONG_HAND_LOGICAL,
        long_hand_physical::LONG_HAND_PHYSICAL, priorities::PRIORITIES,
        shorthands_of_longhands::SHORTHANDS_OF_LONGHANDS,
        shorthands_of_shorthands::SHORTHANDS_OF_SHORTHANDS,
    },
    structures::{
        included_style::IncludedStyle,
        injectable_style::{self, InjectableStyle},
        pre_rule::{CompiledResult, ComputedStyle, PreRule, PreRules, Styles},
    },
    utils::{
        common::get_key_str,
        css::{convert_style_to_class_name, get_priority},
        validators::validate_and_return_property,
    },
};

use super::{injectable_style::InjectableStyleBase, pair::Pair};

#[derive(Debug, Serialize, Deserialize, Clone)]

pub(crate) struct MetaData {
    class_name: String,
    style: InjectableStyleBase,
    priority: u16,
}

impl MetaData {
    pub(crate) fn new(class_name: String, injectable_style: InjectableStyle) -> Self {
        Self {
            class_name,
            style: InjectableStyleBase::from(injectable_style.clone()),
            priority: injectable_style.priority.unwrap(),
        }
    }
    pub(crate) fn _get_style(&self) -> &InjectableStyleBase {
        &self.style
    }

    pub(crate) fn get_css(&self) -> String {
        format!("{}", self.style.ltr)
    }

    pub(crate) fn get_class_name(&self) -> &str {
        &self.class_name
    }

    pub(crate) fn get_priority(&self) -> &u16 {
        &self.priority
    }

    fn set_priority(key: &str) -> u16 {
        get_priority(key)
    }

    pub(crate) fn convert_from_injected_styles_map(
        injected_styles_map: IndexMap<String, InjectableStyle>,
    ) -> Vec<MetaData> {
        injected_styles_map
            .into_iter()
            .map(|(class_name, injectable_style)| {
                MetaData::new(class_name.clone(), injectable_style)
            })
            .collect::<Vec<MetaData>>()
    }
}
