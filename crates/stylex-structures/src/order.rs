use crate::order_pair::OrderPair;

pub trait Order {
  fn get_expansion_fn(
    property: &str,
  ) -> Option<fn(Option<String>) -> Result<Vec<OrderPair>, String>>;
}
