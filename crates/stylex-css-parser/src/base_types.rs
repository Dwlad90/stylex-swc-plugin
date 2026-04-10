/*!
Base types and utilities for CSS parsing.

This module provides fundamental types and utilities used throughout the CSS parser,
including string manipulation utilities.
*/

use std::fmt::{self, Display};

/// A string slice utility that provides efficient substring operations.
#[derive(Debug, Clone)]
pub struct SubString {
  pub string: String,
  pub start_index: usize,
  pub end_index: usize,
}

impl SubString {
  /// Create a new SubString from a string
  pub fn new(s: &str) -> Self {
    Self {
      string: s.to_string(),
      start_index: 0,
      end_index: if s.is_empty() { 0 } else { s.len() - 1 },
    }
  }

  /// Check if the substring starts with the given string
  /// Uses a loop to avoid creating a new string
  pub fn starts_with(&self, s: &str) -> bool {
    let chars: Vec<char> = self.string.chars().collect();
    let search_chars: Vec<char> = s.chars().collect();

    for i in 0..search_chars.len() {
      if self.start_index + i > self.end_index
        || self.start_index + i >= chars.len()
        || chars[self.start_index + i] != search_chars[i]
      {
        return false;
      }
    }
    true
  }

  /// Get the first character of the substring
  pub fn first(&self) -> Option<char> {
    if self.start_index > self.end_index {
      return None;
    }
    self.string.chars().nth(self.start_index)
  }

  /// Get a character at a relative index from the start
  pub fn get(&self, relative_index: usize) -> Option<char> {
    let absolute_index = self.start_index + relative_index;
    if absolute_index > self.end_index {
      return None;
    }
    self.string.chars().nth(absolute_index)
  }

  /// Convert the substring to a String
  pub fn into_string(&self) -> String {
    if self.start_index > self.end_index {
      return String::new();
    }

    let chars: Vec<char> = self.string.chars().collect();
    let start = self.start_index.min(chars.len());
    let end = (self.end_index + 1).min(chars.len());

    if start >= end {
      String::new()
    } else {
      chars[start..end].iter().collect()
    }
  }

  /// Check if the substring is empty
  pub fn is_empty(&self) -> bool {
    self.start_index > self.end_index
  }
}

impl Display for SubString {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.into_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_with_string() {
    let substr = SubString::new("hello");
    assert_eq!(substr.string, "hello");
    assert_eq!(substr.start_index, 0);
    assert_eq!(substr.end_index, 4); // length - 1
  }

  #[test]
  fn test_new_with_empty_string() {
    let substr = SubString::new("");
    assert_eq!(substr.string, "");
    assert_eq!(substr.start_index, 0);
    assert_eq!(substr.end_index, 0);
  }

  #[test]
  fn test_starts_with() {
    let substr = SubString::new("hello world");
    assert!(substr.starts_with("hello"));
    assert!(substr.starts_with("h"));
    assert!(substr.starts_with(""));
    assert!(!substr.starts_with("world"));
    assert!(!substr.starts_with("hello world!")); // longer than original
  }

  #[test]
  fn test_first() {
    let substr = SubString::new("hello");
    assert_eq!(substr.first(), Some('h'));

    let empty_substr = SubString::new("");
    assert_eq!(empty_substr.first(), None);
  }

  #[test]
  fn test_get() {
    let substr = SubString::new("hello");
    assert_eq!(substr.get(0), Some('h'));
    assert_eq!(substr.get(1), Some('e'));
    assert_eq!(substr.get(4), Some('o'));
    assert_eq!(substr.get(5), None); // out of bounds
  }

  #[test]
  fn test_to_string() {
    let substr = SubString::new("hello");
    assert_eq!(substr.to_string(), "hello");

    let empty_substr = SubString::new("");
    assert_eq!(empty_substr.to_string(), "");
  }

  #[test]
  fn test_is_empty() {
    let substr = SubString::new("hello");
    assert!(!substr.is_empty());

    let empty_substr = SubString::new("");
    assert!(!empty_substr.is_empty()); // Even empty string has start_index=0, end_index=0

    // Test with manually modified indices
    let mut modified_substr = SubString::new("hello");
    modified_substr.start_index = 3;
    modified_substr.end_index = 2; // start_index > end_index
    assert!(modified_substr.is_empty());
  }

  #[test]
  fn test_unicode_support() {
    let substr = SubString::new("héllo 🌍");
    assert_eq!(substr.first(), Some('h'));
    assert_eq!(substr.get(1), Some('é'));
    assert!(substr.starts_with("hé"));
    assert_eq!(substr.to_string(), "héllo 🌍");
  }
}
