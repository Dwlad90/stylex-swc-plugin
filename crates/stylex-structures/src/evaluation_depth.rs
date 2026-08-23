//! The ceiling on how deep the evaluator will descend, and where its value
//! comes from.
//!
//! The evaluator folds a nested expression recursively. Without a ceiling its
//! real limit is the thread's stack, and its failure a process abort rather than
//! a diagnostic -- so the ceiling is not a tuning knob so much as the thing that
//! turns a crash into a message. It is configurable because the right number
//! depends on what a project generates, not on anything the compiler can know.

use std::env;
use std::sync::OnceLock;

/// The ceiling when nothing configures one.
///
/// Sized for hand-written styles rather than for the deepest input that could be
/// folded. The number is in the fold's *own* levels, which is not the same as
/// levels of nesting in the source: reading a member descends to the object and
/// then to the value under the key, an array element costs the array as well,
/// resolving a reference descends into what it was bound to, and a parenthesis
/// costs nothing because it is unwrapped before the fold is asked.
pub const DEFAULT_MAX_EVALUATION_DEPTH: usize = 32;

/// The highest ceiling a caller can ask for.
///
/// The ceiling exists to turn a stack overflow into a diagnostic, so a number
/// large enough that the fold exhausts memory before it reaches the number is
/// not a ceiling. `stacker` grows the fold in 16 MiB segments, so a million
/// levels is already far past any input a diagnostic could still describe.
pub const MAX_EVALUATION_DEPTH_LIMIT: usize = 1 << 20;

/// Environment variable that overrides [`DEFAULT_MAX_EVALUATION_DEPTH`].
///
/// It overrides the default only -- a project that configures
/// `maxEvaluationDepth` gets what it configured, whatever the environment says.
/// The precedence is that way round on purpose: a stray value in a CI
/// environment must not silently change what a configured project compiles to.
pub const MAX_EVALUATION_DEPTH_ENV: &str = "STYLEX_MAX_EVALUATION_DEPTH";

/// The ceiling to use, given whatever the caller configured.
///
/// The environment is read once per process, not once per call. "Once per call"
/// sounded free -- a lookup per options value rather than per folded node -- and
/// it measured at about a microsecond per transform on a `node` process, whose
/// environment `getenv` walks and string-compares entry by entry. That is a
/// fixed cost on every file, so it showed up as roughly 3% on a small module and
/// was invisible on a large one, which is exactly the shape of regression a
/// benchmark corpus of small fixtures reports and a profile does not localize.
///
/// Caching it costs nothing a build can observe: the variable is read from the
/// environment the process was started with, and nothing in a build mutates its
/// own environment between files. What it does cost is that a test cannot set
/// the variable and see the answer change -- which is why the rule below takes
/// the value as an argument and is tested there, rather than through a
/// process-global write that would leak into every other test in the binary.
pub fn resolve_max_evaluation_depth(configured: Option<usize>) -> usize {
  static FROM_ENV: OnceLock<Option<String>> = OnceLock::new();

  let from_env = FROM_ENV.get_or_init(read_env);

  resolve_from(configured, from_env.as_deref())
}

/// One read of the documented variable.
///
/// Split out from the cache so a test can prove *which* variable seeds it
/// without depending on when the cache was first filled: a test that went
/// through the public resolver would answer differently depending on test
/// order, because the cache is filled once per process.
fn read_env() -> Option<String> {
  env::var(MAX_EVALUATION_DEPTH_ENV).ok()
}

/// The precedence, with the environment passed in rather than read.
///
/// Split out so the *rule* is testable without a process-global write: setting
/// an environment variable from a test leaks into every other test in the
/// binary, and precedence does not need a side channel to be verified. Which
/// variable seeds the cache is a separate question, and the one test that does
/// write the environment asks only that -- see `read_env`.
fn resolve_from(configured: Option<usize>, from_env: Option<&str>) -> usize {
  let resolved = match configured {
    Some(depth) if depth > 0 => depth,
    _ => from_env
      .and_then(parse_depth)
      .unwrap_or(DEFAULT_MAX_EVALUATION_DEPTH),
  };

  // Clamped on the way out rather than in the `configured` arm alone, so the
  // environment cannot ask for what an option cannot. `STYLEX_MAX_EVALUATION_DEPTH`
  // parses any `usize`, and a number the fold exhausts memory before reaching is
  // not a ceiling whichever side of the boundary it arrived from.
  resolved.min(MAX_EVALUATION_DEPTH_LIMIT)
}

/// One environment value, read as a ceiling or not at all.
///
/// Zero is refused along with everything unparseable: a ceiling of zero would
/// refuse every expression, including the folds the compiler runs to do its own
/// work. Both fall through to the caller's default rather than failing the
/// build -- the variable is an escape hatch, and one that broke the build when
/// mistyped would be a worse one.
fn parse_depth(raw: &str) -> Option<usize> {
  raw.trim().parse::<usize>().ok().filter(|depth| *depth > 0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_configured_depth_is_used_as_given() {
    assert_eq!(resolve_max_evaluation_depth(Some(7)), 7);
    assert_eq!(resolve_max_evaluation_depth(Some(1)), 1);
    assert_eq!(resolve_max_evaluation_depth(Some(100_000)), 100_000);
  }

  // Zero would refuse every expression, including the folds the compiler runs to
  // do its own work, so it is read as unset rather than honoured.
  #[test]
  fn a_configured_zero_falls_back_rather_than_refusing_everything() {
    assert_eq!(resolve_from(Some(0), None), DEFAULT_MAX_EVALUATION_DEPTH);
  }

  #[test]
  fn nothing_configured_and_nothing_in_the_environment_is_the_default() {
    assert_eq!(resolve_from(None, None), DEFAULT_MAX_EVALUATION_DEPTH);
    assert_eq!(DEFAULT_MAX_EVALUATION_DEPTH, 32);
  }

  // The reading the environment variable exists for.
  #[test]
  fn the_environment_supplies_the_ceiling_when_nothing_is_configured() {
    assert_eq!(resolve_from(None, Some("256")), 256);
    assert_eq!(resolve_from(None, Some("1")), 1);
  }

  // Surrounding whitespace is what a shell export or a CI variable pane leaves
  // behind, and it is not a reason to ignore an otherwise good number.
  #[test]
  fn the_environment_value_is_trimmed_before_it_is_read() {
    assert_eq!(resolve_from(None, Some("  64  ")), 64);
    assert_eq!(resolve_from(None, Some("\t8\n")), 8);
  }

  // Config wins, which is the whole point of the precedence: a stray value in a
  // CI environment cannot change what a configured project compiles to.
  #[test]
  fn a_configured_depth_beats_the_environment() {
    assert_eq!(resolve_from(Some(16), Some("256")), 16);
  }

  // And a configured zero does not beat it, because zero is not a ceiling. The
  // environment is consulted next, exactly as if nothing were configured.
  #[test]
  fn a_configured_zero_falls_through_to_the_environment() {
    assert_eq!(resolve_from(Some(0), Some("256")), 256);
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
        resolve_from(None, Some(raw)),
        DEFAULT_MAX_EVALUATION_DEPTH,
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
    assert_eq!(resolve_from(None, Some("+8")), 8);
  }

  #[test]
  fn parse_depth_answers_for_itself() {
    assert_eq!(parse_depth("32"), Some(32));
    assert_eq!(parse_depth(" 32 "), Some(32));
    assert_eq!(parse_depth("0"), None);
    assert_eq!(parse_depth("nope"), None);
  }

  #[test]
  fn the_environment_variable_is_the_documented_name() {
    assert_eq!(MAX_EVALUATION_DEPTH_ENV, "STYLEX_MAX_EVALUATION_DEPTH");
  }

  // A ceiling the fold cannot reach before exhausting memory is not a ceiling.
  // The clamp is what keeps a number that crossed the boundary as `ToUint32`
  // garbage -- or one a caller simply asked too much of -- from removing the
  // guard it was configuring.
  #[test]
  fn a_ceiling_past_the_limit_is_clamped_rather_than_honoured() {
    assert_eq!(
      resolve_from(Some(usize::MAX), None),
      MAX_EVALUATION_DEPTH_LIMIT
    );
    // The environment reaches the same clamp. It parses any `usize`, so without
    // this the escape hatch could ask for what the option cannot.
    assert_eq!(
      resolve_from(None, Some("99999999999")),
      MAX_EVALUATION_DEPTH_LIMIT
    );
    assert_eq!(
      resolve_from(Some(MAX_EVALUATION_DEPTH_LIMIT + 1), None),
      MAX_EVALUATION_DEPTH_LIMIT
    );
    assert_eq!(
      resolve_from(Some(MAX_EVALUATION_DEPTH_LIMIT), None),
      MAX_EVALUATION_DEPTH_LIMIT
    );
  }

  // A configured ceiling below the limit is still taken as given: the clamp is a
  // ceiling on the ceiling, not a floor on it.
  #[test]
  fn a_ceiling_below_the_limit_is_untouched_by_the_clamp() {
    assert_eq!(resolve_from(Some(1), None), 1);
    assert_eq!(
      resolve_from(Some(MAX_EVALUATION_DEPTH_LIMIT - 1), None),
      MAX_EVALUATION_DEPTH_LIMIT - 1
    );
  }

  // `cargo nextest run` gives each test its own process, which is what makes the
  // write below safe; it is the only environment write in the crate, and it is
  // undone before the test returns. Written against `read_env` rather than the
  // public resolver because the cache is filled once per process, so a test that
  // went through it would answer differently depending on test order.
  //
  // Without this the variable's *name* was never exercised: the assertion it
  // replaces read `env::var(MAX_EVALUATION_DEPTH_ENV)` on both sides, so it held
  // for any spelling, including one nothing sets.
  #[test]
  fn the_cached_read_takes_the_documented_variable() {
    // SAFETY: this test owns its process under nextest, and nothing else in this
    // binary reads the environment.
    unsafe { env::set_var(MAX_EVALUATION_DEPTH_ENV, "  256 ") };

    assert_eq!(read_env().as_deref(), Some("  256 "));
    assert_eq!(resolve_from(None, read_env().as_deref()), 256);

    // SAFETY: as above.
    unsafe { env::remove_var(MAX_EVALUATION_DEPTH_ENV) };

    assert_eq!(read_env(), None);
  }

  // The public entry point agrees with the rule asked directly, so the cached
  // read is wired to the same precedence every other case here pins.
  #[test]
  fn the_public_resolver_answers_the_default_with_nothing_configured() {
    assert_eq!(
      resolve_max_evaluation_depth(None),
      resolve_from(None, read_env().as_deref())
    );
  }
}
