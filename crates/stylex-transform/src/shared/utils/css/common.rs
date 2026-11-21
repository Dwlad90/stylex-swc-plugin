use std::fmt::Write;

use stylex_css::css::common::{get_number_suffix, normalize_css_property_value};
use stylex_css::values::content::transform_content_value;
use stylex_state::state_manager::StateManager;
use stylex_structures::raw_value::TRawValue;
use stylex_utils::{math::round_f64, number::to_js_string};

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
    return transform_content_value(value.trim());
  }

  // A blank value has nothing to normalize, so normalization fails the build,
  // as the reference does. The `content` path above quotes its value first, so
  // `content: ' '` is not blank when it gets there.
  normalize_css_property_value(key, value.as_ref(), &state.options)
}

pub(crate) fn transform_value_cached(
  key: &str,
  value: &TRawValue,
  state: &mut StateManager,
) -> String {
  // Keyed by JS identity, not CSS text: `width: 1` and `width: '1'` read the
  // same but compile to different declarations.
  //
  // The capacity holds the key, its length (at most 20 digits) and two colons.
  // A short length leaves spare room, so a short value fits with no second
  // allocation. `write_identity_key` grows the buffer only for a long value.
  // This runs once per declaration.
  //
  // The result is returned owned rather than borrowed from the cache because
  // every caller needs an owned value immediately — a `Pair`, or an entry in
  // the `Vec<String>` the class name is hashed from. Handing back an `Rc<str>`
  // would move that allocation to the call sites rather than remove it; making
  // it disappear means carrying `Rc<str>` through `Pair` and the hash pipeline,
  // which is a change to the class-name path and not a caching concern.
  //
  // The key length goes first. A key and a value can both contain a colon, so
  // without the length `a` + `x:sy` and `a:sx` + `y` give the same cache key.
  let mut cache_key = String::with_capacity(key.len() + 22);
  let _ = write!(cache_key, "{}:", key.len());
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
