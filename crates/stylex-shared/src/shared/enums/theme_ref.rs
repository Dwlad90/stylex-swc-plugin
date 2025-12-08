pub(crate) enum ThemeRefResult {
  CssVar(String),
  Proxy,
  #[allow(dead_code)]
  ToString(String),
}

impl ThemeRefResult {
  pub fn as_css_var(&self) -> Option<&String> {
    match self {
      ThemeRefResult::CssVar(s) => Some(s),
      _ => None,
    }
  }

  pub fn _as_is_proxy(&self) -> Option<()> {
    match self {
      ThemeRefResult::Proxy => Some(()),
      _ => None,
    }
  }
}
