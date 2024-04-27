use crate::shared::constants::legacy_expand_shorthands_order::{Aliases, Shorthands};

use super::{order::Order, order_pair::OrderPair};

pub(crate) struct LegacyExpandShorthandsOrder {}

impl Order for LegacyExpandShorthandsOrder {
  fn get_expansion_fn(property: String) -> Option<fn(Option<String>) -> Vec<OrderPair>> {
    let alias_fn = Aliases::get(property.clone());

    if let Some(alias_fn) = alias_fn {
      return Option::Some(alias_fn);
    }

    let shorthand = Shorthands::get(property);

    if let Some(shorthand_fn) = shorthand {
      return Option::Some(shorthand_fn);
    }

    Option::None
  }
}
