use crate::shared::structures::state_manager::StateManager;
use stylex_constants::constants::common::{CSS_CONTENT_FUNCTIONS, CSS_CONTENT_KEYWORDS};
use stylex_structures::raw_value::TRawValue;
use stylex_types::traits::StyleOptions;
use stylex_utils::{math::round_f64, number::to_js_string};

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

/// Converts a CSS value in JS to the final CSS string value.
///
/// A unit suffix is appended only for a value the author wrote as a number.
/// A string is passed through untouched however numeric it looks, so
/// `gridTemplateColumns: '1'` stays `1` rather than becoming a `1px` track.
pub(crate) fn transform_value(key: &str, raw_value: &TRawValue, state: &StateManager) -> String {
  let value = match raw_value.as_number() {
    Some(number) => format!(
      "{0}{1}",
      to_js_string(round_f64(number, 4)),
      get_number_suffix(key)
    ),
    None => raw_value.as_css_text().into_owned(),
  };

  if key == "content" || key == "hyphenateCharacter" || key == "hyphenate-character" {
    let val = value.trim();

    let is_css_function = CSS_CONTENT_FUNCTIONS.iter().any(|func| val.contains(func));

    let is_keyword = CSS_CONTENT_KEYWORDS.contains(&val);

    let double_quote_count = val.matches('"').count();
    let single_quote_count = val.matches('\'').count();

    let has_matching_quotes = double_quote_count >= 2 || single_quote_count >= 2;

    if is_css_function || is_keyword || has_matching_quotes {
      return val.to_string();
    }

    return format!("\"{}\"", val);
  }

  normalize_css_property_value(key, value.as_ref(), &state.options)
}

pub(crate) fn transform_value_cached(
  key: &str,
  value: &TRawValue,
  state: &mut StateManager,
) -> String {
  // Keyed by JS identity, not CSS text: `width: 1` and `width: '1'` read the
  // same but compile to different declarations. Built in one allocation — this
  // runs once per declaration.
  let mut cache_key = String::with_capacity(key.len() + 1);
  cache_key.push_str(key);
  cache_key.push(':');
  value.write_identity_key(&mut cache_key);

  let cache = state.css_property_seen().get(&cache_key);

  if let Some(result) = cache {
    return result.to_string();
  }

  let result = transform_value(key, value, state);

  state
    .css_property_seen_mut()
    .insert(cache_key, result.clone());

  result
}
