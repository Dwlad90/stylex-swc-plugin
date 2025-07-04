use std::fmt::Write;
use std::rc::Rc;

use indexmap::IndexMap;
use swc_core::ecma::ast::{Expr, PropOrSpread};

use crate::shared::{
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    flat_compiled_styles_value::FlatCompiledStylesValue, injectable_style::InjectableStyleKind,
    obj_map_type::ObjMapType,
  },
  structures::{
    functions::{FunctionConfig, FunctionType},
    injectable_style::InjectableStyle,
    pair::Pair,
    state_manager::StateManager,
  },
  utils::{
    ast::{
      convertors::{lit_to_string, string_to_expression},
      factories::{object_lit_factory, prop_or_spread_string_factory},
    },
    common::{create_hash, dashify},
    css::common::{generate_ltr, generate_rtl, transform_value_cached},
    object::{Pipe, obj_map, obj_map_keys_string, preprocess_object_properties},
  },
};

pub(crate) fn stylex_position_try(
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

  let extended_object = {
    let pipe_result = Pipe::create(styles.clone())
      .pipe(|styles| preprocess_object_properties(&Expr::Object(styles), state))
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

    object_lit_factory(
      pipe_result
        .into_iter()
        .map(|(key, value)| {
          let value = value.as_string().expect("Value must be a string").clone();

          prop_or_spread_string_factory(&key, &value)
        })
        .collect::<Vec<PropOrSpread>>(),
    )
  };

  let ltr_styles = obj_map(
    ObjMapType::Object(extended_object.clone()),
    state,
    |style, _| {
      let Some(tuple) = style.as_tuple() else {
        panic!("Values must be a tuple of key, value, and css type")
      };

      let ltr_values = generate_ltr(&Pair {
        key: tuple.0.clone(),
        value: lit_to_string(
          tuple
            .1
            .clone()
            .as_lit()
            .expect("Value must be a string literal"),
        )
        .unwrap_or_default(),
      });

      Rc::new(FlatCompiledStylesValue::KeyValue(ltr_values))
    },
  );

  let rtl_styles = obj_map(
    ObjMapType::Object(extended_object.clone()),
    state,
    |style, _| {
      let Some(tuple) = style.as_tuple() else {
        panic!("Values must be a tuple of key, value, and css type")
      };

      let value = lit_to_string(
        tuple
          .1
          .clone()
          .as_lit()
          .expect("Value must be a string literal"),
      )
      .unwrap_or_default();

      let key = tuple.0.clone();

      let rtl_values = generate_rtl(&Pair {
        key: key.clone(),
        value: value.clone(),
      });

      match rtl_values {
        Some(rtl_value) => Rc::new(FlatCompiledStylesValue::KeyValue(rtl_value)),
        None => Rc::new(FlatCompiledStylesValue::KeyValue(Pair { key, value })),
      }
    },
  );

  let ltr_string = construct_position_try_obj(ltr_styles);
  let rtl_string = construct_position_try_obj(rtl_styles);

  let position_try_name = format!("--{}{}", class_name_prefix, create_hash(&ltr_string));

  let ltr = format!("@position-try {}{{{}}}", position_try_name, ltr_string);
  let rtl = if ltr_string == rtl_string {
    None
  } else {
    Some(format!(
      "@position-try {}{{{}}}",
      position_try_name, rtl_string
    ))
  };

  (
    position_try_name,
    InjectableStyleKind::Regular(InjectableStyle {
      ltr,
      rtl,
      priority: Some(1.0),
    }),
  )
}

pub(crate) fn get_position_try_fn() -> FunctionConfig {
  FunctionConfig {
    fn_ptr: FunctionType::StylexExprFn(|expr: Expr, local_state: &mut StateManager| -> Expr {
      let (position_try_name, injected_style) =
        stylex_position_try(&EvaluateResultValue::Expr(expr), local_state);

      local_state
        .other_injected_css_rules
        .insert(position_try_name.clone(), Rc::new(injected_style));

      let result = string_to_expression(position_try_name.as_str());

      result
    }),
    takes_path: false,
  }
}

fn construct_position_try_obj(styles: IndexMap<String, Rc<FlatCompiledStylesValue>>) -> String {
  // Collect and sort keys for deterministic output
  let mut sorted_keys = styles.keys().cloned().collect::<Vec<String>>();
  sorted_keys.sort();

  sorted_keys
    .into_iter()
    .map(|k| {
      let v = styles.get(&k).expect("Key must exist");
      match v.as_ref() {
        FlatCompiledStylesValue::String(val) => format!("{}:{};", k, val),
        FlatCompiledStylesValue::KeyValue(pair) => {
          format!("{}:{};{}:{};", k, k, pair.key, pair.value)
        }
        FlatCompiledStylesValue::KeyValues(pairs) => {
          let mut strng = String::new();
          for pair in pairs {
            let _ = write!(strng, "{}:{};{}:{};", k, k, pair.key, pair.value);
          }
          strng
        }
        _ => String::default(),
      }
    })
    .collect::<Vec<String>>()
    .join("")
}
