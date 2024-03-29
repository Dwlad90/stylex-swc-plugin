use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::shared::structures::injectable_style::InjectableStyle;

use super::injectable_style::InjectableStyleBase;

#[derive(Debug, Serialize, Deserialize, Clone)]

pub(crate) struct MetaData {
    class_name: String,
    style: InjectableStyleBase,
    priority: f32,
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

    pub(crate) fn get_priority(&self) -> &f32 {
        &self.priority
    }

    // fn set_priority(key: &str) -> u16 {
    //     get_priority(key)
    // }

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
