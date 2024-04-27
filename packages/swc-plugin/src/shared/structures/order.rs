use super::order_pair::OrderPair;

pub(crate) trait Order {
  fn get_expansion_fn(property: String) -> Option<fn(Option<String>) -> Vec<OrderPair>>;
}
