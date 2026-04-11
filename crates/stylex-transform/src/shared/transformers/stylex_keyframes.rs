use std::rc::Rc;

use indexmap::IndexMap;
use stylex_macros::stylex_panic;
use swc_core::ecma::ast::Expr;

use crate::shared::enums::data_structures::flat_compiled_styles_value::FlatCompiledStylesValue;
use crate::shared::enums::data_structures::obj_map_type::ObjMapType;
use crate::shared::structures::functions::{FunctionConfig, FunctionMap, FunctionType};
use crate::shared::structures::pre_rule::PreRuleValue;
use crate::shared::structures::state_manager::StateManager;
use crate::shared::structures::types::FlatCompiledStyles;
use crate::shared::utils::ast::convertors::{
  convert_expr_to_str, convert_key_value_to_str, create_string_expr,
};
use crate::shared::utils::core::flat_map_expanded_shorthands::flat_map_expanded_shorthands;
use stylex_utils::hash::create_hash;
use stylex_utils::string::dashify;
use crate::shared::utils::css::common::transform_value_cached;
use crate::shared::utils::object::{
  Pipe, obj_entries, obj_from_entries, obj_map, obj_map_keys_string,
};
use crate::shared::{
  enums::data_structures::evaluate_result_value::EvaluateResultValue,
  utils::common::downcast_style_options_to_state_manager,
};
use stylex_constants::constants::messages::{
  ENTRY_MUST_BE_TUPLE, VALUE_MUST_BE_STRING, VALUES_MUST_BE_OBJECT,
};
use stylex_css::css::generate_ltr::generate_ltr;
use stylex_css::css::generate_rtl::generate_rtl;
use stylex_structures::order_pair::OrderPair;
use stylex_structures::pair::Pair;
use stylex_types::enums::data_structures::injectable_style::InjectableStyleKind;
use stylex_types::structures::injectable_style::InjectableStyle;

pub(crate) fn stylex_keyframes(
  frames: &EvaluateResultValue,
  state: &mut StateManager,
) -> (String, InjectableStyleKind) {
  let mut class_name_prefix = state.options.class_name_prefix.clone();

  if class_name_prefix.is_empty() {
    class_name_prefix = "x".to_string();
  }

  let Some(frames) = frames.as_expr().and_then(|expr| expr.as_object()) else {
    #[cfg(not(tarpaulin_include))]
    {
      stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
    }
  };

  let expanded_object = obj_map(ObjMapType::Object(frames.clone()), state, |frame, state| {
    let Some((_, frame, _)) = frame.as_tuple() else {
      #[cfg(not(tarpaulin_include))]
      {
        stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
      }
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
            },
            #[cfg(not(tarpaulin_include))]
            _ => stylex_panic!("{}", ENTRY_MUST_BE_TUPLE),
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
        #[cfg(not(tarpaulin_include))]
        {
          stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
        }
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
        #[cfg(not(tarpaulin_include))]
        {
          stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
        }
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
        #[cfg(not(tarpaulin_include))]
        {
          stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
        }
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
  // TODO: They should be removed in a future version.
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
        #[cfg(not(tarpaulin_include))]
        _ => stylex_panic!("Value must be a key value pair array"),
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
      let key = convert_key_value_to_str(pair);
      let value = match convert_expr_to_str(pair.value.as_ref(), state, &FunctionMap::default()) {
        Some(v) => v,
        #[cfg(not(tarpaulin_include))]
        None => stylex_panic!("{}", VALUE_MUST_BE_STRING),
      };

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
    fn_ptr: FunctionType::StylexExprFn(
      |expr: Expr, local_state: &mut dyn stylex_types::traits::StyleOptions| -> Expr {
        let state = downcast_style_options_to_state_manager(local_state);

        let (animation_name, injected_style) =
          stylex_keyframes(&EvaluateResultValue::Expr(expr), state);

        state
          .other_injected_css_rules
          .insert(animation_name.clone(), Rc::new(injected_style));

        create_string_expr(animation_name.as_str())
      },
    ),
    takes_path: false,
  }
}
