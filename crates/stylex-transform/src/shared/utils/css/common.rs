use crate::shared::structures::state_manager::StateManager;
use stylex_constants::constants::common::{CSS_CONTENT_FUNCTIONS, CSS_CONTENT_KEYWORDS};
use stylex_utils::math::round_to_decimal_places;

// Re-export moved functions from stylex_css so existing callers keep compiling.
#[allow(unused_imports)]
pub(crate) use stylex_css::css::common::{
  build_nested_css_rule, generate_css_rule, get_number_suffix, get_priority, get_value_from_ident,
  inline_style_to_css_string, normalize_css_property_name, normalize_css_property_value, stringify,
  swc_parse_css,
};

#[allow(unused_imports)]
pub(crate) use stylex_css::values::common::split_value_required;

#[allow(unused_imports)]
pub(crate) use stylex_css::values::common::split_value;

pub(crate) fn transform_value(key: &str, value: &str, state: &StateManager) -> String {
  let css_property_value = value.trim();

  let value = match &css_property_value.parse::<f64>() {
    Ok(value) => format!(
      "{0}{1}",
      round_to_decimal_places(*value, 4),
      get_number_suffix(key)
    ),
    Err(_) => css_property_value.to_string(),
  };

  if key == "content" || key == "hyphenateCharacter" || key == "hyphenate-character" {
    let is_css_function = CSS_CONTENT_FUNCTIONS
      .iter()
      .any(|func| value.contains(func));

    let is_keyword = CSS_CONTENT_KEYWORDS.contains(&value.as_str());

    let double_quote_count = value.matches('"').count();
    let single_quote_count = value.matches('\'').count();

    let has_matching_quotes = double_quote_count >= 2 || single_quote_count >= 2;

    if is_css_function || is_keyword || has_matching_quotes {
      return value.to_string();
    }

    return format!("\"{}\"", value);
  }

  normalize_css_property_value(key, value.as_ref(), &state.options)
}

pub(crate) fn transform_value_cached(key: &str, value: &str, state: &mut StateManager) -> String {
  let cache_key = format!("{}:{}", key, value);

  let cache = state.css_property_seen.get(&cache_key);

  if let Some(result) = cache {
    return result.to_string();
  }

  let result = transform_value(key, value, state);

  state.css_property_seen.insert(cache_key, result.clone());

  result
}
