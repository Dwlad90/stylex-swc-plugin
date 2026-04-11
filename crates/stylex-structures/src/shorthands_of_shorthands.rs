use stylex_macros::stylex_unimplemented;

use crate::{order::Order, order_pair::OrderPair};

#[allow(dead_code)]
pub struct ShorthandsOfShorthands {}

#[cfg(not(tarpaulin_include))]
impl Order for ShorthandsOfShorthands {
  fn get_expansion_fn(
    _property: &str,
  ) -> Option<fn(Option<String>) -> Result<Vec<OrderPair>, String>> {
    stylex_unimplemented!("ShorthandsOfShorthands")
  }
}
