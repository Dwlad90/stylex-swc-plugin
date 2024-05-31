use super::{order::Order, order_pair::OrderPair};

pub(crate) struct ShorthandsOfShorthands {}

impl Order for ShorthandsOfShorthands {
  fn get_expansion_fn(_property: &str) -> Option<fn(Option<String>) -> Vec<OrderPair>> {
    unimplemented!("PropertySpecificity")
  }
}
