use super::{order::Order, order_pair::OrderPair};

pub(crate) struct ShorthandsOfShorthands {}

impl Order for ShorthandsOfShorthands {
  fn get_expansion_fn(_property: String) -> Option<fn(Option<String>) -> Vec<OrderPair>> {
    panic!("PropertySpecificity not implemented")
  }
}
