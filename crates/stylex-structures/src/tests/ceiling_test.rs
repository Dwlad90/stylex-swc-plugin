use super::*;

/// A ceiling of this test's own, so the rule is pinned without depending on any
/// real one's numbers -- those move when what they bound is re-measured, and
/// the precedence does not.
static CEILING: Ceiling = Ceiling::new("STYLEX_TEST_CEILING", 32, 8 * 1024);

#[test]
fn a_configured_value_is_used_as_given() {
  assert_eq!(CEILING.resolve_from(Some(7), None), 7);
  assert_eq!(CEILING.resolve_from(Some(1), None), 1);
  assert_eq!(CEILING.resolve_from(Some(5_000), None), 5_000);
}

// Zero would refuse every expression, including the folds the compiler runs to
// do its own work, so it is read as unset rather than honoured.
#[test]
fn a_configured_zero_falls_back_rather_than_refusing_everything() {
  assert_eq!(CEILING.resolve_from(Some(0), None), CEILING.default);
}

#[test]
fn nothing_configured_and_nothing_in_the_environment_is_the_default() {
  assert_eq!(CEILING.resolve_from(None, None), 32);
}

// The reading the environment variable exists for.
#[test]
fn the_environment_supplies_the_value_when_nothing_is_configured() {
  assert_eq!(CEILING.resolve_from(None, Some("256")), 256);
  assert_eq!(CEILING.resolve_from(None, Some("1")), 1);
}

// Surrounding whitespace is what a shell export or a CI variable pane leaves
// behind, and it is not a reason to ignore an otherwise good number.
#[test]
fn the_environment_value_is_trimmed_before_it_is_read() {
  assert_eq!(CEILING.resolve_from(None, Some("  64  ")), 64);
  assert_eq!(CEILING.resolve_from(None, Some("\t8\n")), 8);
}

// Config wins, which is the whole point of the precedence: a stray value in a
// CI environment cannot change what a configured project compiles to.
#[test]
fn a_configured_value_beats_the_environment() {
  assert_eq!(CEILING.resolve_from(Some(16), Some("256")), 16);
}

// And a configured zero does not beat it, because zero is not a ceiling. The
// environment is consulted next, exactly as if nothing were configured.
#[test]
fn a_configured_zero_falls_through_to_the_environment() {
  assert_eq!(CEILING.resolve_from(Some(0), Some("256")), 256);
}

// Every way an environment value can fail to be a ceiling, each falling back
// rather than failing the build.
#[test]
fn an_unusable_environment_value_is_ignored() {
  for raw in [
    "",
    "   ",
    "0",
    "  0  ",
    "-1",
    "1.5",
    "32px",
    "abc",
    "1e3",
    "0x20",
    "99999999999999999999999999999999999999",
  ] {
    assert_eq!(
      CEILING.resolve_from(None, Some(raw)),
      CEILING.default,
      "`{}` should not be read as a ceiling",
      raw
    );
  }
}

// An explicit sign is accepted, because Rust's integer parser accepts it and
// `+8` is unambiguously eight. Pinned rather than left to be discovered: it is
// the one spelling in the neighbourhood of the rejected ones that works.
#[test]
fn a_leading_plus_is_still_a_number() {
  assert_eq!(CEILING.resolve_from(None, Some("+8")), 8);
}

#[test]
fn parse_answers_for_itself() {
  assert_eq!(parse("32"), Some(32));
  assert_eq!(parse(" 32 "), Some(32));
  assert_eq!(parse("0"), None);
  assert_eq!(parse("nope"), None);
}

// A value past the limit is not a ceiling. The clamp is what keeps a number
// that crossed a boundary as `ToUint32` garbage -- or one a caller simply asked
// too much of -- from removing the guard it was configuring.
#[test]
fn a_value_past_the_limit_is_clamped_rather_than_honoured() {
  assert_eq!(CEILING.resolve_from(Some(usize::MAX), None), CEILING.limit);
  // The environment reaches the same clamp. It parses any `usize`, so without
  // this the escape hatch could ask for what the option cannot.
  assert_eq!(
    CEILING.resolve_from(None, Some("99999999999")),
    CEILING.limit
  );
  assert_eq!(
    CEILING.resolve_from(Some(CEILING.limit + 1), None),
    CEILING.limit
  );
  assert_eq!(
    CEILING.resolve_from(Some(CEILING.limit), None),
    CEILING.limit
  );
}

// A configured value below the limit is still taken as given: the clamp is a
// ceiling on the ceiling, not a floor on it.
#[test]
fn a_value_below_the_limit_is_untouched_by_the_clamp() {
  assert_eq!(CEILING.resolve_from(Some(1), None), 1);
  assert_eq!(
    CEILING.resolve_from(Some(CEILING.limit - 1), None),
    CEILING.limit - 1
  );
}

// Which variable seeds the cache is *not* asserted through `resolve`, and
// deliberately not. Proving it needs a process whose environment differs from
// this one's, and writing the environment from a test is `unsafe` in this
// edition -- against `guidelines/stack/RUST.md` -- for a read that is cached
// once per process and so unobservable from the test that wrote it anyway. The
// name is a field, so it is read rather than inferred, and the end-to-end
// reading is in `crates/stylex-rs-compiler/__test__/index.spec.ts`, which
// compiles in a child process with the variable set.
#[test]
fn the_public_resolver_agrees_with_the_rule_it_wraps() {
  assert_eq!(
    CEILING.resolve(Some(9)),
    CEILING.resolve_from(Some(9), None)
  );
}
