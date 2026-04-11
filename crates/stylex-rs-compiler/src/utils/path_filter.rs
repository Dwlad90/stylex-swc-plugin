use crate::enums::PathFilterUnion;
use fancy_regex::Regex;
use glob::Pattern as GlobPattern;
use log::warn;
use std::env;
use std::path::Path;

/// Determines whether a file should be transformed based on include/exclude patterns
pub(crate) fn should_transform_file(
  file_path: &str,
  include: &Option<Vec<PathFilterUnion>>,
  exclude: &Option<Vec<PathFilterUnion>>,
) -> bool {
  let cwd = env::current_dir().unwrap_or_default();
  let file_path_buf = Path::new(file_path);

  // Get relative path
  let relative_path = file_path_buf
    .strip_prefix(&cwd)
    .unwrap_or(file_path_buf)
    .to_string_lossy();

  // Normalize path separators to forward slashes for consistent matching
  let relative_path = relative_path.replace('\\', "/");

  // Check include patterns
  if let Some(include_patterns) = include
    && !include_patterns.is_empty()
  {
    let included = include_patterns
      .iter()
      .any(|pattern| match_pattern(&relative_path, pattern));
    if !included {
      return false;
    }
  }

  // Check exclude patterns
  if let Some(exclude_patterns) = exclude {
    let excluded = exclude_patterns
      .iter()
      .any(|pattern| match_pattern(&relative_path, pattern));
    if excluded {
      return false;
    }
  }

  true
}

/// Matches a file path against a pattern (glob or regex)
fn match_pattern(file_path: &str, pattern: &PathFilterUnion) -> bool {
  match pattern {
    PathFilterUnion::Glob(glob) => GlobPattern::new(glob)
      .map(|p| p.matches(file_path))
      .unwrap_or_else(|e| {
        warn!(
          "Invalid glob pattern '{}': {}. Skipping pattern match.",
          glob, e
        );
        false
      }),
    PathFilterUnion::Regex(regex_str) => match Regex::new(regex_str) {
      Ok(r) => match r.is_match(file_path) {
        Ok(matched) => matched,
        Err(e) => {
          warn!(
            "Error matching regex pattern '{}' against '{}': {}. Skipping pattern match.",
            regex_str, file_path, e
          );
          false
        },
      },
      Err(e) => {
        warn!(
          "Invalid regex pattern '{}': {}. Skipping pattern match.",
          regex_str, e
        );
        false
      },
    },
  }
}

#[cfg(test)]
#[path = "../tests/path_filter_tests.rs"]
mod tests;
