/// Whether a selector segment names a pseudo *element*.
///
/// A `::` prefix marks a pseudo element (`::before`, `::thumb`); a single colon
/// marks a pseudo class (`:hover`, `:nth-child(2)`). Segments carrying neither
/// prefix — attribute selectors, at-rules, custom properties — are not pseudo
/// elements either.
///
/// Legacy single-colon spellings of pseudo elements (`:before`, `:after`,
/// `:first-line`, `:first-letter`) read as pseudo classes here; keys reaching
/// this predicate are expected in the modern double-colon form.
#[inline]
pub fn is_pseudo_element(key: &str) -> bool {
  key.starts_with("::")
}
