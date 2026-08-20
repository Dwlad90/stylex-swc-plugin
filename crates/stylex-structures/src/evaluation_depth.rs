//! The ceiling on how deep the evaluator will descend, and where its value
//! comes from.
//!
//! The evaluator folds a nested expression recursively. Without a ceiling its
//! real limit is the thread's stack, and its failure a process abort rather than
//! a diagnostic -- so the ceiling is not a tuning knob so much as the thing that
//! turns a crash into a message. It is configurable because the right number
//! depends on what a project generates, not on anything the compiler can know.

use std::{env, sync::OnceLock};

/// The ceiling when nothing configures one.
///
/// Sized for hand-written styles rather than for the deepest input that could be
/// folded. The number is in the fold's *own* levels, which is not the same as
/// levels of nesting in the source: reading a member descends to the object and
/// then to the value under the key, an array element costs the array as well,
/// resolving a reference descends into what it was bound to, and a parenthesis
/// costs nothing because it is unwrapped before the fold is asked.
pub const DEFAULT_MAX_EVALUATION_DEPTH: usize = 32;

/// Environment variable that overrides [`DEFAULT_MAX_EVALUATION_DEPTH`].
///
/// It overrides the default only -- a project that configures
/// `maxEvaluationDepth` gets what it configured, whatever the environment says.
/// The precedence is that way round on purpose: a stray value in a CI
/// environment must not silently change what a configured project compiles to.
pub const MAX_EVALUATION_DEPTH_ENV: &str = "STYLEX_MAX_EVALUATION_DEPTH";

/// The ceiling to use, given whatever the caller configured.
///
/// `configured` wins; then [`MAX_EVALUATION_DEPTH_ENV`]; then the default. A
/// value of zero from either source would refuse every expression including the
/// ones the compiler folds to do its own work, so it is read as unset rather
/// than honoured -- as is anything that is not a number.
pub fn resolve_max_evaluation_depth(configured: Option<usize>) -> usize {
  match configured {
    Some(depth) if depth > 0 => depth,
    _ => max_evaluation_depth_from_env().unwrap_or(DEFAULT_MAX_EVALUATION_DEPTH),
  }
}

/// The environment's value, parsed once.
///
/// Cached because the environment is process-global and the answer is consulted
/// per compiled file. A malformed value is `None`, which falls through to the
/// default rather than failing the build: the variable is an escape hatch, and
/// one that broke the build when mistyped would be a worse one.
fn max_evaluation_depth_from_env() -> Option<usize> {
  static FROM_ENV: OnceLock<Option<usize>> = OnceLock::new();

  *FROM_ENV.get_or_init(|| {
    env::var(MAX_EVALUATION_DEPTH_ENV)
      .ok()
      .and_then(|raw| raw.trim().parse::<usize>().ok())
      .filter(|depth| *depth > 0)
  })
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
    assert_eq!(
      resolve_max_evaluation_depth(Some(0)),
      resolve_max_evaluation_depth(None)
    );
  }

  // The environment is process-global and this test process sets nothing, so the
  // unconfigured answer is the default. Asserted rather than assumed, because it
  // is the value every fixture in the workspace is measured against.
  #[test]
  fn nothing_configured_and_nothing_in_the_environment_is_the_default() {
    assert_eq!(
      resolve_max_evaluation_depth(None),
      DEFAULT_MAX_EVALUATION_DEPTH
    );
    assert_eq!(DEFAULT_MAX_EVALUATION_DEPTH, 32);
  }

  #[test]
  fn the_environment_variable_is_the_documented_name() {
    assert_eq!(MAX_EVALUATION_DEPTH_ENV, "STYLEX_MAX_EVALUATION_DEPTH");
  }
}
