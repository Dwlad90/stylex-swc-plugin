use indexmap::IndexMap;
use swc_core::ecma::ast::Expr;

use crate::shared::{
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    flat_compiled_styles_value::FlatCompiledStylesValue, obj_map_type::ObjMapType,
  },
  structures::{
    functions::{FunctionConfig, FunctionMap, FunctionType},
    injectable_style::InjectableStyle,
    order_pair::OrderPair,
    pair::Pair,
    pre_rule::PreRuleValue,
    state_manager::StateManager,
  },
  utils::{
    ast::convertors::{expr_to_str, string_to_expression},
    common::{create_hash, dashify, get_key_str},
    core::flat_map_expanded_shorthands::flat_map_expanded_shorthands,
    css::common::{generate_ltr, generate_rtl, transform_value},
    object::{obj_entries, obj_from_entries, obj_map, obj_map_keys, Pipe},
  },
};

pub(crate) fn stylex_keyframes(
  frames: &EvaluateResultValue,
  state: &mut StateManager,
) -> (String, InjectableStyle) {
  let mut class_name_prefix = state.options.class_name_prefix.clone();

  if class_name_prefix.is_empty() {
    class_name_prefix = "x".to_string();
  }

  let Some(frames) = frames.as_expr().and_then(|expr| expr.as_object()) else {
    panic!("Values must be an object")
  };

  let extended_object = obj_map(ObjMapType::Object(frames.clone()), state, |frame, state| {
    let Some((_, frame, _)) = frame.as_tuple() else {
      panic!("Values must be an object")
    };

    let pipe_result = Pipe::create(frame)
      .pipe(|frame| expand_frame_shorthands(frame, state))
      .pipe(|entries| obj_map_keys(&entries, dashify))
      .pipe(|entries| {
        obj_map(
          ObjMapType::Map(entries),
          state,
          |entry, state| match entry.as_ref() {
            FlatCompiledStylesValue::KeyValue(pair) => {
              Box::new(FlatCompiledStylesValue::KeyValue(Pair {
                key: pair.key.clone(),
                value: transform_value(pair.key.as_str(), pair.value.as_str(), state),
              }))
            }
            _ => panic!("Entry must be a tuple of key and value"),
          },
        )
      })
      .done();

    let (_, result) = pipe_result.into_iter().next().unwrap_or((
      String::default(),
      Box::new(FlatCompiledStylesValue::KeyValue(Pair::new(
        String::default(),
        String::default(),
      ))),
    ));

    result
  });

  let ltr_styles = obj_map(
    ObjMapType::Map(extended_object.clone()),
    state,
    |frame, _| {
      let Some(pair) = frame.as_key_value() else {
        panic!("Values must be an object")
      };

      let ltr_value = generate_ltr(pair);

      Box::new(FlatCompiledStylesValue::KeyValue(ltr_value))
    },
  );

  let rtl_styles = obj_map(
    ObjMapType::Map(extended_object.clone()),
    state,
    |frame, _| {
      let Some(pair) = frame.as_key_value() else {
        panic!("Values must be an object")
      };

      let rtl_value = generate_rtl(pair);

      Box::new(FlatCompiledStylesValue::KeyValue(
        rtl_value.unwrap_or(pair.clone()),
      ))
    },
  );

  let ltr_string = construct_keyframes_obj(&ltr_styles);
  let rtl_string = construct_keyframes_obj(&rtl_styles);

  let animation_name = format!(
    "{}{}-B",
    class_name_prefix,
    create_hash(&format!("<>{}", ltr_string))
  );

  let ltr = format!("@keyframes {}{{{}}}", animation_name, ltr_string);
  let rtl = if ltr_string == rtl_string {
    None
  } else {
    Some(format!("@keyframes {}{{{}}}", animation_name, rtl_string))
  };

  (
    animation_name,
    InjectableStyle {
      ltr,
      rtl,
      priority: Some(1.0),
    },
  )
}

fn construct_keyframes_obj(frames: &IndexMap<String, Box<FlatCompiledStylesValue>>) -> String {
  frames
    .into_iter()
    .map(|(key, value)| {
      let value = match value.as_ref() {
        FlatCompiledStylesValue::KeyValue(pair) => {
          let Pair { key, value } = pair;

          if key.is_empty() || value.is_empty() {
            String::default()
          } else {
            format!("{}:{};", key, value)
          }
        }
        _ => panic!("Value must be a key value pair"),
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
      let key = get_key_str(pair);
      let value = expr_to_str(pair.value.as_ref(), state, &FunctionMap::default());

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
        stylex_keyframes(&EvaluateResultValue::Expr(Box::new(expr)), local_state);

      local_state
        .injected_keyframes
        .insert(animation_name.clone(), Box::new(injected_style));

      let result = string_to_expression(animation_name.as_str());

      result
    }),
    takes_path: false,
  }
}
