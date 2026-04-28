use std::{fmt::Write, rc::Rc};

use stylex_macros::stylex_panic;

use crate::shared::{
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    flat_compiled_styles_value::FlatCompiledStylesValue, obj_map_type::ObjMapType,
  },
  structures::{state_manager::StateManager, types::FlatCompiledStyles},
  utils::{
    css::common::transform_value_cached,
    object::{
      Pipe, obj_map, obj_map_keys_key_value, obj_map_keys_string, preprocess_object_properties,
    },
  },
};
use stylex_constants::constants::messages::{
  ENTRY_MUST_BE_TUPLE, VALUE_MUST_BE_STRING, VALUES_MUST_BE_OBJECT,
};
use stylex_structures::pair::Pair;
use stylex_types::{
  enums::data_structures::injectable_style::InjectableStyleKind,
  structures::injectable_style::InjectableStyle,
};
use stylex_utils::{hash::create_hash, string::dashify};

pub(crate) fn stylex_view_transition_class(
  styles: &EvaluateResultValue,
  state: &mut StateManager,
) -> (String, InjectableStyleKind) {
  let mut class_name_prefix = state.options.class_name_prefix.clone();

  if class_name_prefix.is_empty() {
    class_name_prefix = "x".to_string();
  }

  let Some(styles) = styles.as_expr().and_then(|expr| expr.as_object()) else {
    #[cfg_attr(coverage_nightly, coverage(off))]
    {
      stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
    }
  };
  let preprocessed_object = obj_map(ObjMapType::Object(styles.clone()), state, |style, state| {
    let Some((_, style, _)) = style.as_tuple() else {
      #[cfg_attr(coverage_nightly, coverage(off))]
      {
        stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
      }
    };

    let pipe_result = Pipe::create(style.clone())
      .pipe(|style| preprocess_object_properties(&style, state))
      .pipe(|entries| obj_map_keys_string(&entries, |key| dashify(key).into_owned()))
      .pipe(|entries| {
        obj_map(
          ObjMapType::Map(entries),
          state,
          |entry, state| match entry.as_ref() {
            FlatCompiledStylesValue::KeyValue(pair) => Rc::new(FlatCompiledStylesValue::String(
              transform_value_cached(pair.key.as_str(), pair.value.as_str(), state),
            )),
            #[cfg_attr(coverage_nightly, coverage(off))]
            _ => stylex_panic!("{}", ENTRY_MUST_BE_TUPLE),
          },
        )
      })
      .done();

    let pairs = pipe_result
      .into_iter()
      .filter_map(|(key, value)| value.as_string().cloned().map(|v| Pair::new(key, v)))
      .collect::<Vec<Pair>>();

    Rc::new(FlatCompiledStylesValue::KeyValues(pairs))
  });

  let expanded_object = obj_map_keys_key_value(&preprocessed_object, |k| {
    let dashed_key = dashify(k);
    format!("::view-transition-{}", dashed_key)
  });

  let style_strings = obj_map(
    ObjMapType::Map(expanded_object),
    state,
    construct_view_transition_class_style_str,
  );

  let string_to_hash = &concat_view_transition_class_style_str(&style_strings, state);

  let view_transition_class_name = class_name_prefix + create_hash(string_to_hash).as_str();

  let style = construct_final_view_transition_css_str(style_strings, &view_transition_class_name);

  (
    view_transition_class_name,
    InjectableStyleKind::Regular(InjectableStyle {
      ltr: style,
      rtl: None,
      priority: Some(1.0),
    }),
  )
}

fn construct_view_transition_class_style_str(
  style_strings: Rc<FlatCompiledStylesValue>,
  _state: &mut StateManager,
) -> Rc<FlatCompiledStylesValue> {
  let fmt_pair = |pair: &Pair| {
    let mut result = String::with_capacity(pair.key.len() + pair.value.len() + 2);
    result.push_str(&pair.key);
    result.push(':');
    result.push_str(&pair.value);
    result.push(';');
    result
  };

  match style_strings.as_ref() {
    FlatCompiledStylesValue::KeyValue(pair) => {
      Rc::new(FlatCompiledStylesValue::String(fmt_pair(pair)))
    },
    FlatCompiledStylesValue::KeyValues(pairs) => {
      let capacity = pairs
        .iter()
        .map(|pair| pair.key.len() + pair.value.len() + 2)
        .sum();
      let mut result_string = String::with_capacity(capacity);
      for pair in pairs {
        result_string.push_str(&fmt_pair(pair));
      }
      Rc::new(FlatCompiledStylesValue::String(result_string))
    },
    FlatCompiledStylesValue::String(s) => Rc::new(FlatCompiledStylesValue::String(s.clone())),
    #[cfg_attr(coverage_nightly, coverage(off))]
    _ => stylex_panic!("Expected KeyValues"),
  }
}

fn construct_final_view_transition_css_str(styles: FlatCompiledStyles, class_name: &str) -> String {
  let capacity = styles
    .iter()
    .map(|(key, value)| {
      let style_str = match value.as_ref() {
        FlatCompiledStylesValue::String(s) => s.as_str(),
        #[cfg_attr(coverage_nightly, coverage(off))]
        _ => stylex_panic!("{}", VALUE_MUST_BE_STRING),
      };

      key.len() + class_name.len() + style_str.len() + 6
    })
    .sum();
  let mut result = String::with_capacity(capacity);
  for (key, value) in styles.iter() {
    let style_str = match value.as_ref() {
      FlatCompiledStylesValue::String(s) => s,
      #[cfg_attr(coverage_nightly, coverage(off))]
      _ => stylex_panic!("{}", VALUE_MUST_BE_STRING),
    };
    let _ = write!(result, "{}(*.{}){{{}}}", key, class_name, style_str);
  }
  result
}

fn concat_view_transition_class_style_str(
  style_strings: &FlatCompiledStyles,
  state: &mut StateManager,
) -> String {
  let mut result = String::with_capacity(style_strings.len() * 16);

  style_strings.into_iter().for_each(|(k, v)| {
    let style_str_val = construct_view_transition_class_style_str(Rc::clone(v), state);
    let style_str = match style_str_val.as_ref() {
      FlatCompiledStylesValue::String(s) => s,
      #[cfg_attr(coverage_nightly, coverage(off))]
      _ => stylex_panic!("{}", VALUE_MUST_BE_STRING),
    };
    let _ = write!(result, "{}:{};", k, style_str);
  });

  result
}
