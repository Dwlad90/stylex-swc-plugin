use indexmap::IndexMap;
use swc_core::ecma::ast::Expr;

use crate::shared::{
  enums::{FlatCompiledStylesValue, ObjMapType},
  structures::{
    evaluate_result::EvaluateResultValue,
    functions::{FunctionConfig, FunctionMap, FunctionType},
    injectable_style::InjectableStyle,
    order_pair::OrderPair,
    pair::Pair,
    pre_rule::PreRuleValue,
    state_manager::StateManager,
  },
  utils::{
    common::{create_hash, dashify, expr_to_str, get_key_str, string_to_expression},
    css::{
      common::{flat_map_expanded_shorthands, generate_ltr, generate_rtl},
      utils::transform_value,
    },
    object::{obj_entries, obj_from_entries, obj_map, obj_map_keys, Pipe},
  },
};

pub(crate) fn stylex_keyframes(
  frames: &EvaluateResultValue,
  state: &StateManager,
) -> (String, InjectableStyle) {
  let mut class_name_prefix = state.options.class_name_prefix.clone();

  if class_name_prefix.is_empty() {
    class_name_prefix = "x".to_string();
  }

  let Some(frames) = frames.as_expr().and_then(|expr| expr.as_object()) else {
    panic!("Values must be an object")
  };

  let extended_object = obj_map(ObjMapType::Object(frames.clone()), |frame| {
    let Some((_, frame, _)) = frame.as_tuple() else {
      panic!("Values must be an object")
    };
    let a = Pipe::create(frame.clone())
      .pipe(|frame| expand_frame_shorthands(&frame, &mut state.clone()))
      .pipe(|entries| obj_map_keys(&entries, dashify))
      .pipe(|entries| {
        obj_map(ObjMapType::Map(entries), |entry| match entry.as_ref() {
          FlatCompiledStylesValue::KeyValue(pair) => {
            Box::new(FlatCompiledStylesValue::KeyValue(Pair {
              key: pair.key.clone(),
              value: transform_value(pair.key.as_str(), pair.value.as_str(), state),
            }))
          }
          _ => panic!("Entry must be a tuple of key and value"),
        })
      })
      .done();

    let (_, result) = a.into_iter().next().unwrap_or((
      "".to_string(),
      Box::new(FlatCompiledStylesValue::KeyValue(Pair::new(
        "".to_string(),
        "".to_string(),
      ))),
    ));

    result
  });

  let ltr_styles = obj_map(ObjMapType::Map(extended_object.clone()), |frame| {
    let Some(pair) = frame.as_key_value() else {
      panic!("Values must be an object")
    };

    let ltr_value = generate_ltr(pair.clone());

    Box::new(FlatCompiledStylesValue::KeyValue(ltr_value))
  });

  let rtl_styles = obj_map(ObjMapType::Map(extended_object.clone()), |frame| {
    let Some(pair) = frame.as_key_value() else {
      panic!("Values must be an object")
    };

    let rtl_value = generate_rtl(pair.clone());

    Box::new(FlatCompiledStylesValue::KeyValue(
      rtl_value.unwrap_or(pair.clone()),
    ))
  });

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
      priority: Option::Some(1.0),
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
            "".to_string()
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
    .clone()
    .iter()
    .flat_map(|pair| {
      let key = get_key_str(pair);
      let value = expr_to_str(pair.value.as_ref(), state, &FunctionMap::default());

      flat_map_expanded_shorthands((key, PreRuleValue::String(value)), &state.options)
        .into_iter()
        .filter_map(|pair| {
          pair.1.as_ref()?;

          Option::Some(pair)
        })
        .collect::<Vec<OrderPair>>()
    })
    .filter(|item| item.1.is_some())
    .collect::<Vec<OrderPair>>();

  obj_from_entries(&res)
}

pub(crate) fn get_keyframes_fn() -> FunctionConfig {
  FunctionConfig {
    fn_ptr: FunctionType::StylexExprFn(
      |expr: Expr, local_state: StateManager| -> (Expr, StateManager) {
        let (animation_name, injected_style) =
          stylex_keyframes(&EvaluateResultValue::Expr(Box::new(expr)), &local_state);

        let mut local_state = local_state.clone();

        local_state
          .injected_keyframes
          .insert(animation_name.clone(), Box::new(injected_style));

        let result = string_to_expression(animation_name.as_str());

        (result.unwrap(), local_state)
      },
    ),
    takes_path: false,
  }
}
