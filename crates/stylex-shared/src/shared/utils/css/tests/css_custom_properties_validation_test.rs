#[cfg(test)]
mod css_tests {
  use crate::shared::{
    structures::state_manager::StateManager, utils::css::common::transform_value_cached,
  };

  #[test]
  #[should_panic(expected = "Rule contains an unclosed function")]
  fn disallow_unclosed_style_value_functions() {
    assert_eq!(
      transform_value_cached("color", "var(--foo", &mut StateManager::default()),
      "1px",
    );
  }

  #[test]
  #[should_panic(expected = "Unprefixed custom properties")]
  fn disallow_unprefixed_custom_properties() {
    assert_eq!(
      transform_value_cached("color", "var(foo)", &mut StateManager::default()),
      "1px",
    );
  }

  #[test]
  fn allow_custom_properties() {
    assert_eq!(
      transform_value_cached("color", "var(--foo)", &mut StateManager::default()),
      "var(--foo)",
    );
    assert_eq!(
      transform_value_cached(
        "backgroundColor",
        "var(--bar)",
        &mut StateManager::default()
      ),
      "var(--bar)"
    );
    assert_eq!(
      transform_value_cached(
        "transitionProperty",
        "opacity, margin-top",
        &mut StateManager::default()
      ),
      "opacity,margin-top"
    );

    assert_eq!(
      transform_value_cached(
        "transitionProperty",
        "opacity, marginTop",
        &mut StateManager::default()
      ),
      "opacity,margin-top"
    );

    assert_eq!(
      transform_value_cached(
        "boxShadow",
        "0px 2px 4px var(--shadow-1)",
        &mut StateManager::default()
      ),
      "0 2px 4px var(--shadow-1)"
    );

    assert_eq!(
      transform_value_cached(
        "padding",
        "var(--rightpadding, 20px)",
        &mut StateManager::default()
      ),
      "var(--rightpadding,20px)"
    );
    assert_eq!(
      transform_value_cached(
        "padding",
        "calc((100% - 50px) * 0.5) var(--rightpadding, 20px)",
        &mut StateManager::default()
      ),
      "calc((100% - 50px) * .5) var(--rightpadding,20px)"
    );

    assert_eq!(
      transform_value_cached(
        "margin",
        "max(0px, (48px - var(--x16dnrjz)) / 2)",
        &mut StateManager::default()
      ),
      "max(0px,(48px - var(--x16dnrjz)) / 2)"
    );

    assert_eq!(
      transform_value_cached(
        "backgroundColor",
        "var(----__hashed_var__1jqb1tb, revert)",
        &mut StateManager::default()
      ),
      "var(----__hashed_var__1jqb1tb,revert)"
    );

    assert_eq!(
      transform_value_cached(
        "--__hashed_var__1jqb1tb",
        "var(----__hashed_var__1jqb1tb, revert)",
        &mut StateManager::default()
      ),
      "var(----__hashed_var__1jqb1tb,revert)"
    );

    assert_eq!(
      transform_value_cached("boxShadow", "1px 1px #000", &mut StateManager::default()),
      "1px 1px #000",
    );

    assert_eq!(
      transform_value_cached("quotes", r#""''""#, &mut StateManager::default()),
      r#""""#
    );

    assert_eq!(
      transform_value_cached("quotes", r#""'123'""#, &mut StateManager::default()),
      r#""123""#
    );

    assert_eq!(
      transform_value_cached(
        "gridTemplateAreas",
        r#"'"content"'"#,
        &mut StateManager::default()
      ),
      r#""content""#
    );

    assert_eq!(
      transform_value_cached(
        "gridTemplateAreas",
        r#"'"content" "sidebar"'"#,
        &mut StateManager::default()
      ),
      r#""content" "sidebar""#
    );

    assert_eq!(
      transform_value_cached(
        "gridTemplateAreas",
        r#"'"content""sidebar"'"#,
        &mut StateManager::default()
      ),
      r#""content" "sidebar""#
    );

    assert_eq!(
      transform_value_cached(
        "--span-t",
        r#"translateX(4px)"#,
        &mut StateManager::default()
      ),
      r#"translateX(4px)"#
    );

    assert_eq!(
      transform_value_cached(
        "gridTemplateColumns",
        r#"auto 0fr 0fr"#,
        &mut StateManager::default()
      ),
      r#"auto 0fr 0fr"#
    );

    assert_eq!(
      transform_value_cached(
        "color",
        r#"oklch(42.1% 0.192 328.6 / 1)"#,
        &mut StateManager::default()
      ),
      r#"oklch(42.1% 0.192 328.6 / 1)"#
    );

    assert_eq!(
      transform_value_cached(
        "color",
        r#"oklch(from var(--xs74gcj) l c h / 0.5)"#,
        &mut StateManager::default()
      ),
      r#"oklch(from var(--xs74gcj) l c h / 0.5)"#
    );

    assert_eq!(
      transform_value_cached(
        "color",
        r#"radial-gradient(circle at 0% 0%, oklch(from   var(--colors-tile-background) calc(l + 0.1) calc(c + 0.2) h) 0, transparent 15%), radial-gradient(circle at 80% 100%,oklch(from var(--colors-tile-background) calc(l - 0.25) calc(c + 0.01) h) 0, transparent 30%), linear-gradient(45deg,var(--colors-tile-background) 0%, oklch(from var(--colors-tile-background) calc(l - 0.1) calc(c + 0.3) h) 100%)"#,
        &mut StateManager::default()
      ),
      r#"radial-gradient(circle at 0% 0%, oklch(from var(--colors-tile-background) calc(l + 0.1) calc(c + 0.2) h) 0, transparent 15%), radial-gradient(circle at 80% 100%,oklch(from var(--colors-tile-background) calc(l - 0.25) calc(c + 0.01) h) 0, transparent 30%), linear-gradient(45deg,var(--colors-tile-background) 0%, oklch(from var(--colors-tile-background) calc(l - 0.1) calc(c + 0.3) h) 100%)"#
    );
  }

  #[test]
  fn allow_url_properties() {
    assert_eq!(
      transform_value_cached(
        "backgroundImage",
        r#"url("https://images.unsplash.com/photo-1634170380004-4b3b3b3b3b3b")"#,
        &mut StateManager::default()
      ),
      r#"url("https://images.unsplash.com/photo-1634170380004-4b3b3b3b3b3b")"#,
    );

    assert_eq!(
      transform_value_cached(
        "backgroundImage",
        r#"url("http://images.unsplash.com/photo-1634170380004-4b3b3b3b3b3b")"#,
        &mut StateManager::default()
      ),
      r#"url("http://images.unsplash.com/photo-1634170380004-4b3b3b3b3b3b")"#,
    );

    assert_eq!(
      transform_value_cached(
        "backgroundImage",
        r#"url("https://1.2.3.4/photo-1634170380004-4b3b3b3b3b3b")"#,
        &mut StateManager::default()
      ),
      r#"url("https://1.2.3.4/photo-1634170380004-4b3b3b3b3b3b")"#,
    );

    assert_eq!(
      transform_value_cached(
        "backgroundImage",
        r#"url("http://1.2.3.4/photo-1634170380004-4b3b3b3b3b3b")"#,
        &mut StateManager::default()
      ),
      r#"url("http://1.2.3.4/photo-1634170380004-4b3b3b3b3b3b")"#,
    );

    assert_eq!(
      transform_value_cached(
        "backgroundImage",
        r#"url("/photo-1634170380004-4b3b3b3b3b3b")"#,
        &mut StateManager::default()
      ),
      r#"url("/photo-1634170380004-4b3b3b3b3b3b")"#,
    );

    assert_eq!(
      transform_value_cached(
        "backgroundImage",
        r#"url("./photo-1634170380004-4b3b3b3b3b3b")"#,
        &mut StateManager::default()
      ),
      r#"url("./photo-1634170380004-4b3b3b3b3b3b")"#,
    );

    assert_eq!(
      transform_value_cached(
        "backgroundImage",
        r#"url(asset:communityEmpowermentRoles/Communities-Empowerment-Roles-Platform-Spark-New-Convo-QP-WWW_light)"#,
        &mut StateManager::default()
      ),
      r#"url(asset:communityEmpowermentRoles/Communities-Empowerment-Roles-Platform-Spark-New-Convo-QP-WWW_light)"#,
    );

    assert_eq!(
      transform_value_cached(
        "backgroundImage",
        r#"url("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAMUlEQVQ4T2NkYGAQYcAP3uCTZhw1gGGYhAGBZIA/nYDCgBDAm9BGDWAAJyRCgLaBCAAgXwixzAS0pgAAAABJRU5ErkJggg==")"#,
        &mut StateManager::default()
      ),
      r#"url("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAMUlEQVQ4T2NkYGAQYcAP3uCTZhw1gGGYhAGBZIA/nYDCgBDAm9BGDWAAJyRCgLaBCAAgXwixzAS0pgAAAABJRU5ErkJggg==")"#,
    );
  }
}
