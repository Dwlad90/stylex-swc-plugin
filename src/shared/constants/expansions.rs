use phf::phf_map;

use crate::shared::structures::{
  application_order::ApplicationOrder, order::Order, property_specificity::PropertySpecificity,
  shorthands_of_shorthands::ShorthandsOfShorthands,
};

pub(crate) enum EXPANSIONS {
  ApplicationOrder(ApplicationOrder),
  PropertySpecificity(PropertySpecificity),
  ShorthandsOfShorthands(ShorthandsOfShorthands),
  // "property-specificity" => ApplicationOrder,
  // "shorthands-of-shorthands" => ApplicationOrder,
}
