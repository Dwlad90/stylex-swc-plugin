use std::collections::HashSet;

use super::number_properties::NUMBER_PROPERTY_SUFFIXIES;

pub(crate) fn get_time_units() -> HashSet<String> {
  NUMBER_PROPERTY_SUFFIXIES
    .keys()
    .map(|key| key.to_string())
    .collect()
}
