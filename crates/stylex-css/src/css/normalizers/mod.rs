pub mod whitespace_normalizer;

#[cfg(test)]
mod tests;

pub use whitespace_normalizer::{extract_css_value, normalize_spacing};
