pub mod base;
pub mod convert_camel_case_values;
pub mod detect_unclosed_fns;
pub mod detect_unclosed_strings;
pub mod font_size_px_to_rem;
pub mod leading_zero;
pub mod quotes;
pub mod timings;
pub mod whitespace;
pub mod whitespace_normalizer;
pub mod zero_dimensions;

#[cfg(test)]
mod tests;

pub use whitespace_normalizer::{extract_css_value, normalize_spacing};
