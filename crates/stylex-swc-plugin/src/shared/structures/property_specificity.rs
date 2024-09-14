use super::{order::Order, order_pair::OrderPair};

#[allow(dead_code)]
pub(crate) struct PropertySpecificity {}

impl Order for PropertySpecificity {
  fn get_expansion_fn(_property: &str) -> Option<fn(Option<String>) -> Vec<OrderPair>> {
    unimplemented!("PropertySpecificity")
  }
}