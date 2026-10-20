//! The native half of `processStylexRules`.
//!
//! `src/process-stylex-rules/index.ts` owns the public function. It runs the
//! vendored reference, `src/process-stylex-rules/reference.ts`, and gives it
//! this module's media query parser in place of the one the plugin bundles.

use napi_derive::napi;
use stylex_stylesheet::width_sort_key::media_query_width_sort_key;

/// One `min-width` or `max-width` bound in px, as upstream's
/// `mediaQueryWidthSortKey` answers it.
#[napi(object)]
pub struct MediaQueryWidthBound {
  #[napi(ts_type = "'min' | 'max'")]
  pub kind: String,
  pub value: f64,
}

/// `mediaQueryWidthSortKey(prelude)`: the single width bound an `@media`
/// prelude sorts by, or `null` when it has none.
#[napi]
pub fn media_query_width_bound(prelude: String) -> Option<MediaQueryWidthBound> {
  media_query_width_sort_key(&prelude).map(|bound| MediaQueryWidthBound {
    kind: bound.kind.as_str().to_string(),
    value: bound.value,
  })
}
