use std::{fmt::Write, rc::Rc};

use stylex_macros::stylex_panic;
use swc_core::ecma::ast::{Expr, PropOrSpread};

use crate::shared::{
  enums::data_structures::{
    evaluate_result_value::EvaluateResultValue,
    flat_compiled_styles_value::FlatCompiledStylesValue, obj_map_type::ObjMapType,
  },
  structures::{
    functions::{FunctionConfig, FunctionType},
    state_manager::StateManager,
    types::FlatCompiledStyles,
  },
  utils::{
    ast::convertors::{convert_lit_to_string, create_string_expr},
    common::downcast_style_options_to_state_manager,
    css::common::transform_value_cached,
    object::{Pipe, obj_map, obj_map_keys_string, preprocess_object_properties},
  },
};
use stylex_ast::ast::factories::{create_object_lit, create_string_key_value_prop};
use stylex_constants::constants::messages::{
  ENTRY_MUST_BE_TUPLE, THEME_VAR_TUPLE, VALUE_MUST_BE_STRING, VALUES_MUST_BE_OBJECT,
};
use stylex_css::css::{generate_ltr::generate_ltr, generate_rtl::generate_rtl};
use stylex_structures::pair::Pair;
use stylex_types::{
  enums::data_structures::injectable_style::InjectableStyleKind,
  structures::injectable_style::InjectableStyle,
};
use stylex_utils::{hash::create_hash, string::dashify};

pub(crate) fn stylex_position_try(
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

  let extended_object = {
    let pipe_result = Pipe::create(styles.clone())
      .pipe(|styles| preprocess_object_properties(&Expr::Object(styles), state))
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

    create_object_lit(
      pipe_result
        .into_iter()
        .map(|(key, value)| {
          let value = match value.as_string() {
            Some(s) => s.clone(),
            #[cfg_attr(coverage_nightly, coverage(off))]
            None => stylex_panic!("{}", VALUE_MUST_BE_STRING),
          };

          create_string_key_value_prop(&key, &value)
        })
        .collect::<Vec<PropOrSpread>>(),
    )
  };

  let options = state.options.clone();

  let ltr_styles = obj_map(
    ObjMapType::Object(extended_object.clone()),
    state,
    |style, _| {
      let Some(tuple) = style.as_tuple() else {
        #[cfg_attr(coverage_nightly, coverage(off))]
        {
          stylex_panic!("{}", THEME_VAR_TUPLE)
        }
      };

      let pair = Pair {
        key: tuple.0.clone(),
        value: convert_lit_to_string(match tuple.1.clone().as_lit() {
          Some(lit) => lit,
          #[cfg_attr(coverage_nightly, coverage(off))]
          None => stylex_panic!("{}", VALUE_MUST_BE_STRING),
        })
        .unwrap_or_default(),
      };

      let ltr_values = generate_ltr(&pair, &options).into_owned();

      Rc::new(FlatCompiledStylesValue::KeyValue(ltr_values))
    },
  );

  let rtl_styles = obj_map(
    ObjMapType::Object(extended_object.clone()),
    state,
    |style, _| {
      let Some(tuple) = style.as_tuple() else {
        #[cfg_attr(coverage_nightly, coverage(off))]
        {
          stylex_panic!("{}", THEME_VAR_TUPLE)
        }
      };

      let value = convert_lit_to_string(match tuple.1.clone().as_lit() {
        Some(lit) => lit,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("{}", VALUE_MUST_BE_STRING),
      })
      .unwrap_or_default();

      let key = tuple.0.clone();

      let pair = Pair {
        key: key.clone(),
        value: value.clone(),
      };

      let rtl_values = generate_rtl(&pair, &options);

      match rtl_values {
        Some(rtl_value) => Rc::new(FlatCompiledStylesValue::KeyValue(rtl_value.into_owned())),
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
      priority: Some(0.0),
    }),
  )
}

pub(crate) fn get_position_try_fn() -> FunctionConfig {
  FunctionConfig {
    fn_ptr: FunctionType::StylexExprFn(
      |expr: Expr, local_state: &mut dyn stylex_types::traits::StyleOptions| -> Expr {
        let state = downcast_style_options_to_state_manager(local_state);

        let (position_try_name, injected_style) =
          stylex_position_try(&EvaluateResultValue::Expr(expr), state);

        state
          .other_injected_css_rules
          .insert(position_try_name.clone().into(), Rc::new(injected_style));

        create_string_expr(position_try_name.as_str())
      },
    ),
    takes_path: false,
  }
}

fn construct_position_try_obj(styles: FlatCompiledStyles) -> String {
  // Collect and sort keys for deterministic output
  let mut sorted_keys = styles.keys().cloned().collect::<Vec<String>>();
  sorted_keys.sort();

  sorted_keys
    .into_iter()
    .map(|k| {
      let v = match styles.get(&k) {
        Some(v) => v,
        #[cfg_attr(coverage_nightly, coverage(off))]
        None => stylex_panic!("Expected property key to exist in compiled styles."),
      };
      match v.as_ref() {
        FlatCompiledStylesValue::String(val) => format!("{}:{};", k, val),
        FlatCompiledStylesValue::KeyValue(pair) => {
          format!("{}:{};{}:{};", k, k, pair.key, pair.value)
        },
        FlatCompiledStylesValue::KeyValues(pairs) => {
          let mut strng = String::new();
          for pair in pairs {
            let _ = write!(strng, "{}:{};{}:{};", k, k, pair.key, pair.value);
          }
          strng
        },
        _ => String::default(),
      }
    })
    .collect::<Vec<String>>()
    .join("")
}
