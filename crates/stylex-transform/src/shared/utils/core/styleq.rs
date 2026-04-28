use std::rc::Rc;

use stylex_macros::{stylex_panic, stylex_unreachable};
use stylex_styleq::{StyleMap, StyleqArgument, StyleqValue};

use crate::shared::{
  enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue,
  structures::types::FlatCompiledStyles,
  utils::core::parse_nullable_style::{ResolvedArg, StyleObject},
};

pub(crate) struct StyleQResult {
  pub(crate) class_name: String,
  pub(crate) inline_style: Option<FlatCompiledStyles>,
  pub(crate) data_style_src: Option<String>,
}

impl StyleqValue for FlatCompiledStylesValue {
  fn as_class_name(&self) -> Option<&str> {
    match self {
      FlatCompiledStylesValue::String(value) => Some(value.as_str()),
      _ => None,
    }
  }

  fn is_null(&self) -> bool {
    matches!(self, FlatCompiledStylesValue::Null)
  }

  fn is_true_bool(&self) -> bool {
    matches!(self, FlatCompiledStylesValue::Bool(true))
  }
}

impl StyleqArgument<Rc<FlatCompiledStylesValue>> for ResolvedArg {
  fn as_style(&self) -> Option<&StyleMap<Rc<FlatCompiledStylesValue>>> {
    match self {
      ResolvedArg::StyleObject(style_object, _, _) => style_object_as_style(style_object),
      ResolvedArg::ConditionalStyle(_, Some(style_object), _, _, _) => {
        style_object_as_style(style_object)
      },
      ResolvedArg::ConditionalStyle(_, None, _, _, _) => None,
    }
  }

  fn should_skip(&self) -> bool {
    matches!(
      self,
      ResolvedArg::StyleObject(StyleObject::Nullable, _, _)
        | ResolvedArg::ConditionalStyle(_, None, _, _, _)
    )
  }
}

pub(crate) fn styleq(arguments: &[ResolvedArg]) -> StyleQResult {
  if arguments.is_empty() {
    return StyleQResult {
      class_name: String::new(),
      inline_style: None,
      data_style_src: None,
    };
  }

  let styleq = stylex_styleq::create_styleq(stylex_styleq::StyleqOptions {
    dedupe_class_name_chunks: true,
    ..Default::default()
  });
  let result = styleq.styleq(arguments);

  StyleQResult {
    class_name: result.class_name,
    inline_style: result.inline_style,
    data_style_src: Some(result.data_style_src),
  }
}

fn style_object_as_style(style_object: &StyleObject) -> Option<&FlatCompiledStyles> {
  match style_object {
    StyleObject::Style(style) => Some(style),
    StyleObject::Nullable => None,
    #[cfg_attr(coverage_nightly, coverage(off))]
    StyleObject::Other => {
      stylex_panic!("Only compiled StyleX style objects are allowed in styleq().")
    },
    #[cfg_attr(coverage_nightly, coverage(off))]
    StyleObject::Unreachable => {
      stylex_unreachable!("Encountered an unexpected style object variant in styleq processing.")
    },
  }
}
