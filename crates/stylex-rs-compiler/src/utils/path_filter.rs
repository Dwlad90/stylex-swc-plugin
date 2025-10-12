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
        }
      },
      Err(e) => {
        warn!(
          "Invalid regex pattern '{}': {}. Skipping pattern match.",
          regex_str, e
        );
        false
      }
    },
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_match_pattern_glob() {
    let pattern = PathFilterUnion::Glob("src/**/*.rs".to_string());

    assert!(match_pattern("src/main.rs", &pattern));
    assert!(match_pattern("src/lib/utils.rs", &pattern));
    assert!(match_pattern("src/deep/nested/file.rs", &pattern));
    assert!(!match_pattern("lib/main.rs", &pattern));
    assert!(!match_pattern("src/main.ts", &pattern));
  }

  #[test]
  fn test_match_pattern_regex() {
    let pattern = PathFilterUnion::Regex(r"\.test\.rs$".to_string());

    assert!(match_pattern("src/button.test.rs", &pattern));
    assert!(match_pattern("lib/component.test.rs", &pattern));
    assert!(!match_pattern("src/button.rs", &pattern));
    assert!(!match_pattern("src/button.test.ts", &pattern));
  }

  #[test]
  fn test_match_pattern_regex_negative_lookbehind() {
    // Test negative lookbehind: match .tsx files NOT preceded by src/
    // The lookbehind checks the position before the entire match
    let pattern = PathFilterUnion::Regex(r"(?<!src/).*\.tsx$".to_string());

    // Both should match because the lookbehind checks position before .*,
    // which is start-of-string for both, and start-of-string is not "src/"
    assert!(match_pattern("lib/components/Button.tsx", &pattern));
    assert!(match_pattern("src/components/Button.tsx", &pattern));

    // Test a pattern that actually checks if string starts with src/
    // Use negative lookahead instead: don't start with src/
    let pattern2 = PathFilterUnion::Regex(r"^(?!src/).*\.tsx$".to_string());
    assert!(match_pattern("lib/components/Button.tsx", &pattern2));
    assert!(!match_pattern("src/components/Button.tsx", &pattern2));
  }

  #[test]
  fn test_match_pattern_regex_negative_lookahead() {
    // Test negative lookahead: match node_modules but not @stylexjs
    let pattern = PathFilterUnion::Regex(r"node_modules(?!/@stylexjs)".to_string());

    // Should match - not followed by /@stylexjs
    assert!(match_pattern(
      "node_modules/some-package/index.js",
      &pattern
    ));

    // Should NOT match - followed by /@stylexjs
    assert!(!match_pattern(
      "node_modules/@stylexjs/stylex/index.js",
      &pattern
    ));
  }

  #[test]
  fn test_match_pattern_complex_glob() {
    // Note: glob crate doesn't support brace expansion like {rs,toml}
    // Users should specify separate patterns instead
    let pattern_rs = PathFilterUnion::Glob("**/*.rs".to_string());
    let pattern_toml = PathFilterUnion::Glob("**/*.toml".to_string());

    assert!(match_pattern("src/main.rs", &pattern_rs));
    assert!(match_pattern("Cargo.toml", &pattern_toml));

    // Brace expansion doesn't work - this is expected behavior
    let pattern_braces = PathFilterUnion::Glob("**/*.{rs,toml}".to_string());
    assert!(!match_pattern("Cargo.toml", &pattern_braces));
  }

  #[test]
  fn test_should_transform_file_no_patterns() {
    let result = should_transform_file("src/main.rs", &None, &None);
    assert!(result);
  }

  #[test]
  fn test_should_transform_file_empty_patterns() {
    let result = should_transform_file("src/main.rs", &Some(vec![]), &Some(vec![]));
    assert!(result);
  }

  #[test]
  fn test_should_transform_file_include_glob() {
    let include = Some(vec![PathFilterUnion::Glob("src/**/*.rs".to_string())]);

    // These paths should be relative to cwd in actual use
    assert!(should_transform_file("src/main.rs", &include, &None));
    assert!(should_transform_file("src/lib/utils.rs", &include, &None));
    assert!(!should_transform_file("lib/main.rs", &include, &None));
  }

  #[test]
  fn test_should_transform_file_include_regex() {
    let include = Some(vec![PathFilterUnion::Regex(r"^src/.*\.rs$".to_string())]);

    assert!(should_transform_file("src/main.rs", &include, &None));
    assert!(should_transform_file("src/utils.rs", &include, &None));
    assert!(!should_transform_file("lib/main.rs", &include, &None));
  }

  #[test]
  fn test_should_transform_file_exclude_glob() {
    let exclude = Some(vec![PathFilterUnion::Glob("**/*.test.rs".to_string())]);

    assert!(should_transform_file("src/main.rs", &None, &exclude));
    assert!(!should_transform_file("src/main.test.rs", &None, &exclude));
    assert!(!should_transform_file("lib/utils.test.rs", &None, &exclude));
  }

  #[test]
  fn test_should_transform_file_exclude_regex() {
    let exclude = Some(vec![PathFilterUnion::Regex(r"\.test\.rs$".to_string())]);

    assert!(should_transform_file("src/main.rs", &None, &exclude));
    assert!(!should_transform_file("src/main.test.rs", &None, &exclude));
  }

  #[test]
  fn test_should_transform_file_multiple_include_patterns() {
    let include = Some(vec![
      PathFilterUnion::Glob("src/**/*.rs".to_string()),
      PathFilterUnion::Glob("app/**/*.rs".to_string()),
    ]);

    assert!(should_transform_file("src/main.rs", &include, &None));
    assert!(should_transform_file("app/main.rs", &include, &None));
    assert!(!should_transform_file("lib/main.rs", &include, &None));
  }

  #[test]
  fn test_should_transform_file_multiple_exclude_patterns() {
    let exclude = Some(vec![
      PathFilterUnion::Glob("**/*.test.rs".to_string()),
      PathFilterUnion::Glob("**/*.spec.rs".to_string()),
    ]);

    assert!(should_transform_file("src/main.rs", &None, &exclude));
    assert!(!should_transform_file("src/main.test.rs", &None, &exclude));
    assert!(!should_transform_file("src/main.spec.rs", &None, &exclude));
  }

  #[test]
  fn test_should_transform_file_combined_include_exclude() {
    let include = Some(vec![PathFilterUnion::Glob("src/**/*.rs".to_string())]);
    let exclude = Some(vec![PathFilterUnion::Glob("**/*.test.rs".to_string())]);

    assert!(should_transform_file("src/main.rs", &include, &exclude));
    assert!(!should_transform_file(
      "src/main.test.rs",
      &include,
      &exclude
    ));
    assert!(!should_transform_file("lib/main.rs", &include, &exclude));
  }

  #[test]
  fn test_should_transform_file_mixed_patterns() {
    let include = Some(vec![
      PathFilterUnion::Glob("src/**/*.rs".to_string()),
      PathFilterUnion::Regex(r"^app/.*\.rs$".to_string()),
    ]);
    let exclude = Some(vec![
      PathFilterUnion::Glob("**/__tests__/**".to_string()),
      PathFilterUnion::Regex(r"\.test\.rs$".to_string()),
    ]);

    assert!(should_transform_file("src/main.rs", &include, &exclude));
    assert!(should_transform_file("app/main.rs", &include, &exclude));
    assert!(!should_transform_file(
      "src/__tests__/main.rs",
      &include,
      &exclude
    ));
    assert!(!should_transform_file(
      "src/main.test.rs",
      &include,
      &exclude
    ));
    assert!(!should_transform_file("lib/main.rs", &include, &exclude));
  }

  #[test]
  fn test_should_transform_file_exclude_takes_precedence() {
    // Even if file matches include, exclude should filter it out
    let include = Some(vec![PathFilterUnion::Glob("src/**/*.rs".to_string())]);
    let exclude = Some(vec![PathFilterUnion::Glob("src/__tests__/**".to_string())]);

    assert!(should_transform_file("src/main.rs", &include, &exclude));
    assert!(!should_transform_file(
      "src/__tests__/main.rs",
      &include,
      &exclude
    ));
  }

  #[test]
  fn test_match_pattern_edge_cases() {
    // Test with dotfiles
    let pattern = PathFilterUnion::Glob("**/.*.rs".to_string());
    assert!(match_pattern(".hidden.rs", &pattern));

    // Test with multiple dots
    let pattern2 = PathFilterUnion::Glob("**/*.test.rs".to_string());
    assert!(match_pattern("component.test.rs", &pattern2));
  }

  #[test]
  fn test_invalid_regex_pattern() {
    // Invalid regex should not panic, should return false
    let pattern = PathFilterUnion::Regex("[invalid(".to_string());
    assert!(!match_pattern("src/main.rs", &pattern));
  }

  #[test]
  fn test_invalid_glob_pattern() {
    // Invalid glob should not panic, should return false
    let pattern = PathFilterUnion::Glob("[invalid".to_string());
    assert!(!match_pattern("src/main.rs", &pattern));
  }
}
