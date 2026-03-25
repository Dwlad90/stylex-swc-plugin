use stylex_core::stylex_unimplemented;

use super::{order::Order, order_pair::OrderPair};

#[allow(dead_code)]
pub struct ShorthandsOfShorthands {}

impl Order for ShorthandsOfShorthands {
  fn get_expansion_fn(
    _property: &str,
  ) -> Option<fn(Option<String>) -> Result<Vec<OrderPair>, String>> {
    stylex_unimplemented!("ShorthandsOfShorthands")
  }
}
