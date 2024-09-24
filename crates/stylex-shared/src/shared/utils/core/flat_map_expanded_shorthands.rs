use crate::shared::structures::{
  application_order::ApplicationOrder, legacy_expand_shorthands_order::LegacyExpandShorthandsOrder,
  order::Order, order_pair::OrderPair, pre_rule::PreRuleValue,
  property_specificity_order::PropertySpecificityOrder, stylex_options::StyleResolution,
  stylex_state_options::StyleXStateOptions,
};

pub(crate) fn flat_map_expanded_shorthands(
  obj_entry: (String, PreRuleValue),
  options: &StyleXStateOptions,
) -> Vec<OrderPair> {
  let (key, raw_value) = obj_entry;

  let value = match raw_value {
    PreRuleValue::String(value) => Some(value),
    PreRuleValue::Vec(_) => {
      panic!("Cannot use fallbacks for shorthands. Use the expansion instead.")
    }
    PreRuleValue::Expr(_) => {
      panic!("Cannot use expressions for shorthands. Use the expansion instead.")
    }
    PreRuleValue::Null => None,
  };

  let key = if key.starts_with("var(") && key.ends_with(')') {
    key[4..key.len() - 1].to_string()
  } else {
    key
  };

  let expansion_fn = match &options.style_resolution {
    StyleResolution::ApplicationOrder => ApplicationOrder::get_expansion_fn(key.as_str()),
    StyleResolution::LegacyExpandShorthands => {
      LegacyExpandShorthandsOrder::get_expansion_fn(key.as_str())
    }
    StyleResolution::PropertySpecificity => {
      PropertySpecificityOrder::get_expansion_fn(key.as_str())
    }
  };

  if let Some(expansion_fn) = expansion_fn {
    return (expansion_fn)(value);
  }

  let order_pair = OrderPair(key, value);

  let vec_order_pair: Vec<OrderPair> = vec![order_pair];

  vec_order_pair
}
