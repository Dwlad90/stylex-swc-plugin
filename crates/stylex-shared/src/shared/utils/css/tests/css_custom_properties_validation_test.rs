#[cfg(test)]
mod css_tests {
  use crate::shared::{
    structures::state_manager::StateManager, utils::css::common::transform_value,
  };

  #[test]
  #[should_panic(expected = "Rule contains an unclosed function")]
  fn disallow_unclosed_style_value_functions() {
    assert_eq!(
      transform_value("color", "var(--foo", &StateManager::default()),
      "1px",
    );
  }

  #[test]
  #[should_panic(expected = "Unprefixed custom properties")]
  fn disallow_unprefixed_custom_properties() {
    assert_eq!(
      transform_value("color", "var(foo)", &StateManager::default()),
      "1px",
    );
  }

  #[test]
  fn allow_custom_properties() {
    assert_eq!(
      transform_value("color", "var(--foo)", &StateManager::default()),
      "var(--foo)",
    );
    assert_eq!(
      transform_value("backgroundColor", "var(--bar)", &StateManager::default()),
      "var(--bar)"
    );
    assert_eq!(
      transform_value(
        "transitionProperty",
        "opacity, margin-top",
        &StateManager::default()
      ),
      "opacity,margin-top"
    );

    assert_eq!(
      transform_value(
        "transitionProperty",
        "opacity, marginTop",
        &StateManager::default()
      ),
      "opacity,margin-top"
    );

    assert_eq!(
      transform_value(
        "boxShadow",
        "0px 2px 4px var(--shadow-1)",
        &StateManager::default()
      ),
      "0 2px 4px var(--shadow-1)"
    );

    assert_eq!(
      transform_value(
        "padding",
        "var(--rightpadding, 20px)",
        &StateManager::default()
      ),
      "var(--rightpadding,20px)"
    );
    assert_eq!(
      transform_value(
        "padding",
        "calc((100% - 50px) * 0.5) var(--rightpadding, 20px)",
        &StateManager::default()
      ),
      "calc((100% - 50px) * .5) var(--rightpadding,20px)"
    );

    assert_eq!(
      transform_value(
        "margin",
        "max(0px, (48px - var(--x16dnrjz)) / 2)",
        &StateManager::default()
      ),
      "max(0px,(48px - var(--x16dnrjz)) / 2)"
    );

    assert_eq!(
      transform_value(
        "backgroundColor",
        "var(----__hashed_var__1jqb1tb, revert)",
        &StateManager::default()
      ),
      "var(----__hashed_var__1jqb1tb,revert)"
    );

    assert_eq!(
      transform_value(
        "--__hashed_var__1jqb1tb",
        "var(----__hashed_var__1jqb1tb, revert)",
        &StateManager::default()
      ),
      "var(----__hashed_var__1jqb1tb,revert)"
    );

    assert_eq!(
      transform_value("boxShadow", "1px 1px #000", &StateManager::default()),
      "1px 1px #000",
    );

    assert_eq!(
      transform_value("quotes", r#""''""#, &StateManager::default()),
      r#""""#
    );

    assert_eq!(
      transform_value("quotes", r#""'123'""#, &StateManager::default()),
      r#""123""#
    );

    assert_eq!(
      transform_value(
        "gridTemplateAreas",
        r#"'"content"'"#,
        &StateManager::default()
      ),
      r#""content""#
    );

    assert_eq!(
      transform_value(
        "gridTemplateAreas",
        r#"'"content" "sidebar"'"#,
        &StateManager::default()
      ),
      r#""content" "sidebar""#
    );

    assert_eq!(
      transform_value(
        "gridTemplateAreas",
        r#"'"content""sidebar"'"#,
        &StateManager::default()
      ),
      r#""content" "sidebar""#
    );

    assert_eq!(
      transform_value("--span-t", r#"translateX(4px)"#, &StateManager::default()),
      r#"translateX(4px)"#
    );
  }

  #[test]
  fn allow_url_properties() {
    assert_eq!(
      transform_value(
        "backgroundImage",
        r#"url("https://images.unsplash.com/photo-1634170380004-4b3b3b3b3b3b")"#,
        &StateManager::default()
      ),
      r#"url("https://images.unsplash.com/photo-1634170380004-4b3b3b3b3b3b")"#,
    );

    assert_eq!(
      transform_value(
        "backgroundImage",
        r#"url("http://images.unsplash.com/photo-1634170380004-4b3b3b3b3b3b")"#,
        &StateManager::default()
      ),
      r#"url("http://images.unsplash.com/photo-1634170380004-4b3b3b3b3b3b")"#,
    );

    assert_eq!(
      transform_value(
        "backgroundImage",
        r#"url("https://1.2.3.4/photo-1634170380004-4b3b3b3b3b3b")"#,
        &StateManager::default()
      ),
      r#"url("https://1.2.3.4/photo-1634170380004-4b3b3b3b3b3b")"#,
    );

    assert_eq!(
      transform_value(
        "backgroundImage",
        r#"url("http://1.2.3.4/photo-1634170380004-4b3b3b3b3b3b")"#,
        &StateManager::default()
      ),
      r#"url("http://1.2.3.4/photo-1634170380004-4b3b3b3b3b3b")"#,
    );

    assert_eq!(
      transform_value(
        "backgroundImage",
        r#"url("/photo-1634170380004-4b3b3b3b3b3b")"#,
        &StateManager::default()
      ),
      r#"url("/photo-1634170380004-4b3b3b3b3b3b")"#,
    );

    assert_eq!(
      transform_value(
        "backgroundImage",
        r#"url("./photo-1634170380004-4b3b3b3b3b3b")"#,
        &StateManager::default()
      ),
      r#"url("./photo-1634170380004-4b3b3b3b3b3b")"#,
    );
  }
}
