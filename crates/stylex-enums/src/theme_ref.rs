use std::sync::Arc;

pub enum ThemeRefResult {
  CssVar(Arc<str>),
  Proxy,
  #[allow(dead_code)]
  ToString(String),
}

impl ThemeRefResult {
  pub fn as_css_var(&self) -> Option<&str> {
    match self {
      ThemeRefResult::CssVar(s) => Some(s),
      _ => None,
    }
  }

  pub fn as_is_proxy(&self) -> Option<()> {
    match self {
      ThemeRefResult::Proxy => Some(()),
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_as_css_var_with_css_var() {
    let result = ThemeRefResult::CssVar("--color-primary".to_string());
    assert_eq!(result.as_css_var(), Some(&"--color-primary".to_string()));
  }

  #[test]
  fn test_as_css_var_with_proxy() {
    let result = ThemeRefResult::Proxy;
    assert_eq!(result.as_css_var(), None);
  }

  #[test]
  fn test_as_css_var_with_to_string() {
    let result = ThemeRefResult::ToString("val".to_string());
    assert_eq!(result.as_css_var(), None);
  }

  #[test]
  fn test_as_is_proxy_with_proxy() {
    let result = ThemeRefResult::Proxy;
    assert_eq!(result.as_is_proxy(), Some(()));
  }

  #[test]
  fn test_as_is_proxy_with_css_var() {
    let result = ThemeRefResult::CssVar("x".to_string());
    assert_eq!(result.as_is_proxy(), None);
  }

  #[test]
  fn test_as_is_proxy_with_to_string() {
    let result = ThemeRefResult::ToString("x".to_string());
    assert_eq!(result.as_is_proxy(), None);
  }
}
