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

