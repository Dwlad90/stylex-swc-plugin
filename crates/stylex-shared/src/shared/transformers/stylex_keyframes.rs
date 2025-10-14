use std::rc::Rc;

use indexmap::IndexMap;
use swc_core::ecma::ast::Expr;

use crate::shared::{
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    flat_compiled_styles_value::FlatCompiledStylesValue, injectable_style::InjectableStyleKind,
    obj_map_type::ObjMapType,
  },
  structures::{
    functions::{FunctionConfig, FunctionMap, FunctionType},
    injectable_style::InjectableStyle,
    order_pair::OrderPair,
    pair::Pair,
    pre_rule::PreRuleValue,
    state_manager::StateManager, types::FlatCompiledStyles,
  },
  utils::{
    ast::convertors::{expr_to_str, key_value_to_str, string_to_expression},
    common::{create_hash, dashify},
    core::flat_map_expanded_shorthands::flat_map_expanded_shorthands,
    css::{common::transform_value_cached, generate_ltr::generate_ltr, generate_rtl::generate_rtl},
    object::{obj_entries, obj_from_entries, obj_map, obj_map_keys_string, Pipe},
  },
};

pub(crate) fn stylex_keyframes(
  frames: &EvaluateResultValue,
  state: &mut StateManager,
) -> (String, InjectableStyleKind) {
  let mut class_name_prefix = state.options.class_name_prefix.clone();

  if class_name_prefix.is_empty() {
    class_name_prefix = "x".to_string();
  }

  let Some(frames) = frames.as_expr().and_then(|expr| expr.as_object()) else {
    panic!("Values must be an object")
  };

  let expanded_object = obj_map(ObjMapType::Object(frames.clone()), state, |frame, state| {
    let Some((_, frame, _)) = frame.as_tuple() else {
      panic!("Values must be an object")
    };

    let pipe_result = Pipe::create(frame)
      .pipe(|frame| expand_frame_shorthands(frame, state))
      .pipe(|entries| obj_map_keys_string(&entries, dashify))
      .pipe(|entries| {
        obj_map(
          ObjMapType::Map(entries),
          state,
          |entry, state| match entry.as_ref() {
            FlatCompiledStylesValue::KeyValue(pair) => {
              Rc::new(FlatCompiledStylesValue::KeyValue(Pair::new(
                pair.key.clone(),
                transform_value_cached(pair.key.as_str(), pair.value.as_str(), state),
              )))
            }
            _ => panic!("Entry must be a tuple of key and value"),
          },
        )
      })
      .done();

    let pairs = pipe_result
      .into_iter()
      .filter_map(|(_, value)| value.as_key_value().cloned())
      .collect::<Vec<Pair>>();

    Rc::new(FlatCompiledStylesValue::KeyValues(pairs))
  });

  let options = state.options.clone();

  let ltr_styles = obj_map(
    ObjMapType::Map(expanded_object.clone()),
    state,
    |frame, _| {
      let Some(pairs) = frame.as_key_values() else {
        panic!("Values must be an object")
      };

      let ltr_values = pairs
        .iter()
        .map(|pair| generate_ltr(pair, &options))
        .collect();

      Rc::new(FlatCompiledStylesValue::KeyValues(ltr_values))
    },
  );

  let stable_styles = obj_map(
    ObjMapType::Map(expanded_object.clone()),
    state,
    |frame, _| {
      let Some(pairs) = frame.as_key_values() else {
        panic!("Values must be an object")
      };

      let ltr_values = pairs
        .iter()
        .map(|pair| generate_ltr(pair, &Default::default()))
        .collect();

      Rc::new(FlatCompiledStylesValue::KeyValues(ltr_values))
    },
  );

  let options = state.options.clone();

  let rtl_styles = obj_map(
    ObjMapType::Map(expanded_object.clone()),
    state,
    |frame, _| {
      let Some(pairs) = frame.as_key_values() else {
        panic!("Values must be an object")
      };

      let rtl_values = pairs
        .iter()
        .map(|pair| generate_rtl(pair, &options).unwrap_or_else(|| pair.clone()))
        .collect();

      Rc::new(FlatCompiledStylesValue::KeyValues(rtl_values))
    },
  );

  let ltr_string = construct_keyframes_obj(&ltr_styles);
  let rtl_string = construct_keyframes_obj(&rtl_styles);
  let stable_string = construct_keyframes_obj(&stable_styles);

  // NOTE: Use a direction-agnostic hash to keep LTR/RTL classnames stable across builds.
  // NOTE: '<>' and '-B' is used to keep existing hashes stable.
  // They should be removed in a future version.
  let animation_name = format!(
    "{}{}-B",
    class_name_prefix,
    create_hash(&format!("<>{}", stable_string))
  );

  let ltr = format!("@keyframes {}{{{}}}", animation_name, ltr_string);
  let rtl = if ltr_string == rtl_string {
    None
  } else {
    Some(format!("@keyframes {}{{{}}}", animation_name, rtl_string))
  };

  (
    animation_name,
    InjectableStyleKind::Regular(InjectableStyle {
      ltr,
      rtl,
      priority: Some(0.0),
    }),
  )
}

fn construct_keyframes_obj(frames: &FlatCompiledStyles) -> String {
  frames
    .into_iter()
    .map(|(key, value)| {
      let value = match value.as_ref() {
        FlatCompiledStylesValue::KeyValues(pairs) => pairs
          .iter()
          .map(|pair| {
            if pair.key.is_empty() || pair.value.is_empty() {
              String::default()
            } else {
              format!("{}:{};", pair.key, pair.value)
            }
          })
          .collect::<Vec<String>>()
          .join(""),
        _ => panic!("Value must be a key value pair array"),
      };

      format!("{}{{{}}}", key, value)
    })
    .collect::<Vec<String>>()
    .join("")
}

fn expand_frame_shorthands(frame: &Expr, state: &mut StateManager) -> IndexMap<String, String> {
  let res: Vec<_> = obj_entries(&frame.clone())
    .iter()
    .flat_map(|pair| {
      let key = key_value_to_str(pair);
      let value = expr_to_str(pair.value.as_ref(), state, &FunctionMap::default())
        .expect("Value is not a string");

      flat_map_expanded_shorthands((key, PreRuleValue::String(value)), &state.options)
        .into_iter()
        .filter_map(|pair| {
          pair.1.as_ref()?;

          Some(pair)
        })
        .collect::<Vec<OrderPair>>()
    })
    .filter(|item| item.1.is_some())
    .collect::<Vec<OrderPair>>();

  obj_from_entries(&res)
}

pub(crate) fn get_keyframes_fn() -> FunctionConfig {
  FunctionConfig {
    fn_ptr: FunctionType::StylexExprFn(|expr: Expr, local_state: &mut StateManager| -> Expr {
      let (animation_name, injected_style) =
        stylex_keyframes(&EvaluateResultValue::Expr(expr), local_state);

      local_state
        .other_injected_css_rules
        .insert(animation_name.clone(), Rc::new(injected_style));

      string_to_expression(animation_name.as_str())
    }),
    takes_path: false,
  }
}
