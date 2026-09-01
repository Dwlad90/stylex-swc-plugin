#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod enums;
pub mod serialization;
pub mod structures;
pub mod traits;

#[cfg(test)]
mod tests;
