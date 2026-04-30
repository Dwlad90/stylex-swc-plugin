use std::borrow::Cow;

// JS-parity: stylex/packages/shared — `OrderPair` is constructed with static
// property name literals (~1000 sites in `application_order.rs`,
// `legacy_expand_shorthands_order.rs`, `property_specificity_order.rs`); the
// only owned cases come from CSS variable unwrapping in
// `flat_map_expanded_shorthands`. `Cow<'static, str>` lets all literal sites
// avoid heap allocation while still permitting owned strings on the rare
// dynamic path.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct OrderPair(pub Cow<'static, str>, pub Option<String>);
