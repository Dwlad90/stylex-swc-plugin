use crate::shared::constants::application_order::{Aliases, Shorthands};

use super::{order::Order, order_pair::OrderPair};

pub(crate) struct ApplicationOrder {}

impl Order for ApplicationOrder {
  fn get_expansion_fn(property: &str) -> Option<fn(Option<String>) -> Vec<OrderPair>> {
    let alias_fn = Aliases::get(property);

    if let Some(alias_fn) = alias_fn {
      return Some(alias_fn);
    }

    let shorthand = Shorthands::get(property);

    if let Some(shorthand_fn) = shorthand {
      return Some(shorthand_fn);
    }

    None
  }
}
