use std::rc::Rc;

use indexmap::IndexMap;
use stylex_macros::stylex_panic;
use stylex_structures::pre_rule_value::PreRuleValue;
use swc_core::ecma::ast::{Expr, Lit};

use crate::shared::{
  enums::data_structures::obj_map_type::ObjMapType,
  utils::{
    ast::convertors::convert_expr_to_str,
    core::flat_map_expanded_shorthands::flat_map_expanded_shorthands,
    object::{Pipe, obj_entries, obj_from_entries, obj_map, obj_map_keys_and_transform_values},
  },
};
use stylex_ast::ast::convertors::{convert_key_value_to_str, create_string_expr, normalize_expr};
use stylex_constants::constants::messages::VALUES_MUST_BE_OBJECT;
use stylex_css::css::{generate_ltr::generate_ltr, generate_rtl::generate_rtl};
use stylex_state::{
  common::downcast_style_options_to_state_manager,
  evaluate_result_value::EvaluateResultValue,
  flat_compiled_styles_value::FlatCompiledStylesValue,
  functions::{FunctionConfig, FunctionMap, FunctionType},
  state_manager::StateManager,
  types::FlatCompiledStyles,
};
use stylex_structures::{order_pair::OrderPair, pair::Pair, raw_value::TRawValue};
use stylex_types::{
  enums::data_structures::injectable_style::InjectableStyleKind,
  structures::injectable_style::InjectableStyle,
};
use stylex_utils::{hash::create_hash, string::dashify};

pub(crate) fn stylex_keyframes(
  frames: &EvaluateResultValue,
  state: &mut StateManager,
) -> (String, InjectableStyleKind) {
  // NOTE: an unset `classNamePrefix` arrives here already defaulted to `x`,
  // so an empty one was asked for explicitly and is honoured as empty.
  let class_name_prefix = state.options.class_name_prefix.clone();

  let Some(frames) = frames.as_expr().and_then(|expr| expr.as_object()) else {
    stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
  };

  let expanded_object = obj_map(ObjMapType::Object(frames.clone()), state, |frame, state| {
    let Some((_, frame, _)) = frame.as_tuple() else {
      stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
    };

    let pipe_result = Pipe::create(frame)
      .pipe(|frame| expand_frame_shorthands(frame, state))
      .pipe(|entries| {
        obj_map_keys_and_transform_values(
          &entries,
          state,
          |key| dashify(key).into_owned(),
          FlatCompiledStylesValue::KeyValue,
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
        stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
      };

      let ltr_values = pairs
        .iter()
        .map(|pair| generate_ltr(pair, &options).into_owned())
        .collect();

      Rc::new(FlatCompiledStylesValue::KeyValues(ltr_values))
    },
  );

  let stable_styles = obj_map(
    ObjMapType::Map(expanded_object.clone()),
    state,
    |frame, _| {
      let Some(pairs) = frame.as_key_values() else {
        stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
      };

      let ltr_values = pairs
        .iter()
        .map(|pair| generate_ltr(pair, &Default::default()).into_owned())
        .collect();

      Rc::new(FlatCompiledStylesValue::KeyValues(ltr_values))
    },
  );

  let options = state.options.clone();

  let rtl_styles = obj_map(ObjMapType::Map(expanded_object), state, |frame, _| {
    let Some(pairs) = frame.as_key_values() else {
      stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
    };

    let rtl_values = pairs
      .iter()
      .map(|pair| {
        generate_rtl(pair, &options)
          .map(|pair| pair.into_owned())
          .unwrap_or_else(|| pair.clone())
      })
      .collect();

    Rc::new(FlatCompiledStylesValue::KeyValues(rtl_values))
  });

  let ltr_string = construct_keyframes_obj(&ltr_styles);
  let rtl_string = construct_keyframes_obj(&rtl_styles);
  let stable_string = construct_keyframes_obj(&stable_styles);

  // NOTE: Use a direction-agnostic hash to keep LTR/RTL classnames stable across
  // builds. NOTE: '<>' and '-B' is used to keep existing hashes stable.
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
          .filter_map(Pair::as_css_text)
          .collect::<Vec<String>>()
          .join(""),
        _ => stylex_panic!("Value must be a key value pair array"),
      };

      format!("{}{{{}}}", key, value)
    })
    .collect::<Vec<String>>()
    .join("")
}

fn expand_frame_shorthands(frame: &Expr, state: &mut StateManager) -> IndexMap<String, TRawValue> {
  let res: Vec<_> = obj_entries(&frame.clone())
    .iter()
    .flat_map(|pair| {
      let key = convert_key_value_to_str(pair);
      // A numeric frame value keeps its JS type all the way to
      // `transform_value`, which is what appends the unit suffix; only a
      // non-numeric value is coerced to a string here.
      let value = match normalize_expr(pair.value.as_ref()) {
        Expr::Lit(Lit::Num(num)) => Some(PreRuleValue::number(num.value)),
        _ => convert_expr_to_str(pair.value.as_ref(), state, &FunctionMap::default())
          .map(PreRuleValue::string),
      };

      // A step value that is not a string or a number declares nothing. An
      // animation step has no condition to apply and no fallback to choose
      // from, so a nested value object and a fallback array mean nothing here,
      // and neither does `null` -- the step keeps whatever else it declares.
      let Some(value) = value else {
        return vec![];
      };

      flat_map_expanded_shorthands((key, value), &state.options)
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
          .insert(animation_name.clone().into(), Rc::new(injected_style));

        create_string_expr(animation_name.as_str())
      },
    ),
    takes_path: false,
  }
}
