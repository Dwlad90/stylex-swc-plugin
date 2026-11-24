use phf::phf_map;

pub static PSEUDO_CLASS_PRIORITIES: phf::Map<&'static str, &'static f64> = phf_map! {
  ":is" => &40.0,
  ":where" => &40.0,
  ":not" => &40.0,
  ":has" => &45.0,
  ":dir" => &50.0,
  ":lang" => &51.0,
  ":first-child" => &52.0,
  ":first-of-type" => &53.0,
  ":last-child" => &54.0,
  ":last-of-type" => &55.0,
  ":only-child" => &56.0,
  ":only-of-type" => &57.0,

  ":nth-child" => &60.0,
  ":nth-last-child" => &61.0,
  ":nth-of-type" => &62.0,
  ":nth-last-of-type" => &63.0, // "nth-last-of-type" is the same priority as "nth-of-typ.0e
  ":empty" => &70.0,

  ":link" => &80.0,
  ":any-link" => &81.0,
  ":local-link" => &82.0,
  ":target-within" => &83.0,
  ":target" => &84.0,
  ":visited" => &85.0,

  ":enabled" => &91.0,
  ":disabled" => &92.0,
  ":required" => &93.0,
  ":optional" => &94.0,
  ":read-only" => &95.0,
  ":read-write" => &96.0,
  ":placeholder-shown" => &97.0,
  ":in-range" => &98.0,
  ":out-of-range" => &99.0,

  ":default" => &100.0,
  ":checked" => &101.0,
  ":indeterminate" => &101.0,
  ":blank" => &102.0,
  ":valid" => &103.0,
  ":invalid" => &104.0,
  ":user-invalid" => &105.0,

  ":autofill" => &110.0,

  ":picture-in-picture" => &120.0,
  ":modal" => &121.0,
  ":fullscreen" => &122.0,
  ":paused" => &123.0,
  ":playing" => &124.0,
  ":current" => &125.0,
  ":past" => &126.0,
  ":future" => &127.0,

  ":hover" => &130.0,
  ":focusWithin" => &140.0,
  ":focus" => &150.0,
  ":focusVisible" => &160.0,
  ":active" => &170.0,
};

pub static AT_RULE_PRIORITIES: phf::Map<&'static str, &'static f64> = phf_map! {
  "@supports" => &30.0,
  "@media" => &200.0,
  "@container" => &300.0,
};

pub static PSEUDO_ELEMENT_PRIORITY: f64 = 5000.0;

/// The priority of a key that names no at-rule, pseudo or property that the
/// tables know: an attribute selector, a `defineConsts` placeholder
/// (`var(--x)`) or an unknown at-rule.
pub const DEFAULT_PRIORITY: f64 = 3000.0;

/// The priority of a pseudo-class that is not in [`PSEUDO_CLASS_PRIORITIES`].
pub const UNKNOWN_PSEUDO_CLASS_PRIORITY: f64 = 40.0;

/// The priority of a custom property (`--x`), which is 1, or of a key that
/// starts with an at-rule of [`AT_RULE_PRIORITIES`]. `None` for any other key.
/// No at-rule of the table starts another, so the order of the search does not
/// change the answer.
pub fn get_at_rule_priority(key: &str) -> Option<f64> {
  if key.starts_with("--") {
    return Some(1.0);
  }

  AT_RULE_PRIORITIES
    .entries()
    .find(|(at_rule, _)| key.starts_with(**at_rule))
    .map(|(_, priority)| **priority)
}

/// The priority of a chain of pseudo parts (`:hover::after`): the sum of the
/// parts. `None` for one part or none, and for a chain with a functional
/// pseudo-class, such as `:not(.x)`.
pub fn compound_pseudo_priority(parts: &[&str]) -> Option<f64> {
  if parts.len() <= 1 || parts.iter().any(|part| part.contains('(')) {
    return None;
  }

  let total = parts
    .iter()
    .map(|part| {
      if part.starts_with("::") {
        PSEUDO_ELEMENT_PRIORITY
      } else {
        PSEUDO_CLASS_PRIORITIES
          .get(*part)
          .map_or(UNKNOWN_PSEUDO_CLASS_PRIORITY, |priority| **priority)
      }
    })
    .sum();

  Some(total)
}
