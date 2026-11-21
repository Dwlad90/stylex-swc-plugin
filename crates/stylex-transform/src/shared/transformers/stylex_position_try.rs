use crate::shared::transformers::named_rule::fold_to_rule_name;
use crate::shared::utils::object::{
  obj_map_entry, obj_map_keys_and_transform_values, preprocess_object_properties,
};
use stylex_constants::constants::messages::VALUES_MUST_BE_OBJECT;
use stylex_css::css::{generate_ltr::generate_ltr, generate_rtl::generate_rtl};
use stylex_js::string::js_string_cmp;
use stylex_macros::stylex_panic;
use stylex_state::{
  evaluate_result_value::EvaluateResultValue,
  functions::{FunctionConfig, FunctionType},
  state_manager::StateManager,
};
use stylex_structures::pair::PairCow;
use stylex_types::{
  enums::data_structures::injectable_style::InjectableStyleKind,
  structures::injectable_style::InjectableStyle,
};
use stylex_utils::{hash::create_hash, string::dashify};

pub(crate) fn stylex_position_try(
  styles: &EvaluateResultValue,
  state: &mut StateManager,
) -> (String, InjectableStyleKind) {
  // NOTE: an unset `classNamePrefix` arrives here already defaulted to `x`,
  // so an empty one was asked for explicitly and is honoured as empty.
  let class_name_prefix = state.options.class_name_prefix.clone();

  let Some(styles) = styles.as_expr().filter(|expr| expr.is_object()) else {
    stylex_panic!("{}", VALUES_MUST_BE_OBJECT)
  };

  let entries = preprocess_object_properties(styles, state);
  let declarations =
    obj_map_keys_and_transform_values(&entries, state, |key| dashify(key).into_owned());

  // The generators map a whole declaration, because a direction flip can
  // rewrite the property name as well as the value.
  let ltr_styles = obj_map_entry(&declarations, |pair| generate_ltr(pair, &state.options));
  let rtl_styles = obj_map_entry(&declarations, |pair| {
    generate_rtl(pair, &state.options).unwrap_or_else(|| PairCow::borrowed(pair))
  });

  let ltr_string = construct_position_try_obj(ltr_styles);
  let rtl_string = construct_position_try_obj(rtl_styles);

  let position_try_name = format!("--{}{}", class_name_prefix, create_hash(&ltr_string));

  let ltr = format!("@position-try {} {{{}}}", position_try_name, ltr_string);
  let rtl = (ltr_string != rtl_string)
    .then(|| format!("@position-try {} {{{}}}", position_try_name, rtl_string));

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
    fn_ptr: FunctionType::StylexExprFn(|expr, state| {
      fold_to_rule_name(expr, state, stylex_position_try)
    }),
    takes_path: false,
  }
}

/// The body of one `@position-try` rule, its declarations written in the order
/// their property names sort in, which is the UTF-16 order JavaScript sorts by.
fn construct_position_try_obj(mut styles: Vec<PairCow>) -> String {
  // Each name is held once, so the order of the sort is total.
  styles.sort_unstable_by(|left, right| js_string_cmp(&left.key, &right.key));

  let mut output = String::with_capacity(styles.len().saturating_mul(32));

  for style in &styles {
    style.push_css_text(&mut output);
  }

  output
}
