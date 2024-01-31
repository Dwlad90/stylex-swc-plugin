use crate::shared::constants::application_order::{Aliases, Shorthands};

use super::{order::Order, order_pair::OrderPair};

pub(crate) struct ShorthandsOfShorthands {}

impl Order for ShorthandsOfShorthands {
    fn get_expansion_fn(property: &str) -> Option<fn(Option<&str>) -> Vec<OrderPair>> {
        panic!("PropertySpecificity not implemented")
    }
}
