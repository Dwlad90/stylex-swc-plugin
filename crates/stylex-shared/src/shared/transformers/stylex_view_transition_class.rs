use std::fmt::Write;
use std::rc::Rc;

use crate::shared::{
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    flat_compiled_styles_value::FlatCompiledStylesValue, injectable_style::InjectableStyleKind,
    obj_map_type::ObjMapType,
  },
  structures::{
    injectable_style::InjectableStyle, pair::Pair, state_manager::StateManager,
    types::FlatCompiledStyles,
  },
  utils::{
    common::{create_hash, dashify},
    css::common::transform_value_cached,
    object::{
      Pipe, obj_map, obj_map_keys_key_value, obj_map_keys_string, preprocess_object_properties,
    },
  },
};

pub(crate) fn stylex_view_transition_class(
  styles: &EvaluateResultValue,
  state: &mut StateManager,
) -> (String, InjectableStyleKind) {
  let mut class_name_prefix = state.options.class_name_prefix.clone();

  if class_name_prefix.is_empty() {
    class_name_prefix = "x".to_string();
  }

  let Some(styles) = styles.as_expr().and_then(|expr| expr.as_object()) else {
    panic!("Values must be an object")
  };
  let preprocessed_object = obj_map(ObjMapType::Object(styles.clone()), state, |style, state| {
    let Some((_, style, _)) = style.as_tuple() else {
      panic!("Values must be an object")
    };

    let pipe_result = Pipe::create(style.clone())
      .pipe(|style| preprocess_object_properties(&style, state))
      .pipe(|entries| obj_map_keys_string(&entries, dashify))
      .pipe(|entries| {
        obj_map(
          ObjMapType::Map(entries),
          state,
          |entry, state| match entry.as_ref() {
            FlatCompiledStylesValue::KeyValue(pair) => Rc::new(FlatCompiledStylesValue::String(
              transform_value_cached(pair.key.as_str(), pair.value.as_str(), state),
            )),
            _ => panic!("Entry must be a tuple of key and value"),
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
    format!("::view-transition-{}", dashify(k))
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
  let fmt_pair = |pair: &Pair| format!("{}:{};", pair.key, pair.value);

  match style_strings.as_ref() {
    FlatCompiledStylesValue::KeyValue(pair) => {
      Rc::new(FlatCompiledStylesValue::String(fmt_pair(pair)))
    }
    FlatCompiledStylesValue::KeyValues(pairs) => {
      let mut result_string = String::new();
      for pair in pairs {
        result_string.push_str(&fmt_pair(pair));
      }
      Rc::new(FlatCompiledStylesValue::String(result_string))
    }
    FlatCompiledStylesValue::String(s) => Rc::new(FlatCompiledStylesValue::String(s.clone())),
    _ => panic!("Expected KeyValues"),
  }
}

fn construct_final_view_transition_css_str(styles: FlatCompiledStyles, class_name: &str) -> String {
  let mut result = String::new();
  for (key, value) in styles.iter() {
    let style_str = match value.as_ref() {
      FlatCompiledStylesValue::String(s) => s,
      _ => panic!("Expected FlatCompiledStylesValue::String"),
    };
    let _ = write!(result, "{}(*.{}){{{}}}", key, class_name, style_str);
  }
  result
}

fn concat_view_transition_class_style_str(
  style_strings: &FlatCompiledStyles,
  state: &mut StateManager,
) -> String {
  let mut result = String::new();

  style_strings.into_iter().for_each(|(k, v)| {
    let style_str_val = construct_view_transition_class_style_str(Rc::clone(v), state);
    let style_str = match style_str_val.as_ref() {
      FlatCompiledStylesValue::String(s) => s,
      _ => panic!("Expected FlatCompiledStylesValue::String"),
    };
    let _ = write!(result, "{}:{};", k, style_str);
  });

  result
}
