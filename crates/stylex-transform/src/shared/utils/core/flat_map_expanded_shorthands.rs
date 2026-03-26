use log::warn;
use stylex_macros::stylex_panic;
use swc_core::ecma::ast::Expr;

use stylex_css_order::structures::application_order::ApplicationOrder;
use stylex_css_order::structures::legacy_expand_shorthands_order::LegacyExpandShorthandsOrder;
use stylex_css_order::structures::property_specificity_order::PropertySpecificityOrder;
use stylex_structures::order::Order;
use stylex_structures::order_pair::OrderPair;
use stylex_enums::property_validation_mode::PropertyValidationMode;
use stylex_enums::style_resolution::StyleResolution;
use stylex_structures::stylex_state_options::StyleXStateOptions;
use crate::shared::structures::pre_rule::PreRuleValue;
use crate::shared::utils::ast::convertors::convert_lit_to_string;

pub(crate) fn flat_map_expanded_shorthands(
  obj_entry: (String, PreRuleValue),
  options: &StyleXStateOptions,
) -> Vec<OrderPair> {
  let (key, raw_value) = obj_entry;

  let value = match raw_value {
    PreRuleValue::String(value) => Some(value),
    PreRuleValue::Vec(_) => {
      let msg = "Cannot use fallbacks for shorthands. Use the expansion instead.";
      match options.property_validation_mode {
        PropertyValidationMode::Throw => {
          stylex_panic!("{}", msg);
        }
        PropertyValidationMode::Warn => {
          warn!("{}", msg);
          return vec![];
        }
        PropertyValidationMode::Silent => {
          return vec![];
        }
      }
    }
    PreRuleValue::Expr(expr) => match expr {
      Expr::Lit(lit) => Some(match convert_lit_to_string(&lit) {
        Some(s) => s,
        None => stylex_panic!("Failed to convert literal value to string in shorthand expansion."),
      }),
      _ => {
        let msg = "Cannot use expressions for shorthands. Use the expansion instead.";
        match options.property_validation_mode {
          PropertyValidationMode::Throw => {
            stylex_panic!("{}", msg);
          }
          PropertyValidationMode::Warn => {
            warn!("{}", msg);
            return vec![];
          }
          PropertyValidationMode::Silent => {
            return vec![];
          }
        }
      }
    },
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
    return match (expansion_fn)(value) {
      Ok(expanded) => expanded,
      Err(error_message) => match options.property_validation_mode {
        PropertyValidationMode::Throw => {
          stylex_panic!("{}", error_message);
        }
        PropertyValidationMode::Warn => {
          warn!("{}", error_message);
          vec![]
        }
        PropertyValidationMode::Silent => {
          vec![]
        }
      },
    };
  }

  let order_pair = OrderPair(key, value);

  let vec_order_pair: Vec<OrderPair> = vec![order_pair];

  vec_order_pair
}
