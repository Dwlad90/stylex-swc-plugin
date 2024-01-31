use super::order_pair::OrderPair;

pub(crate) trait Order {
    fn get_expansion_fn(property: &str) -> Option<fn(Option<&str>) -> Vec<OrderPair>>;
}
