#[cfg(test)]
mod css_tests {
  use crate::shared::utils::css::transform_value;

  #[test]
  #[should_panic(expected = "Unexpected end of file, but expected ')'")]
  fn disallow_unclosed_style_value_functions() {
    assert_eq!(transform_value("color", "var(--foo"), "1px");
  }

  #[test]
  #[should_panic(expected = "Unprefixed custom properties")]
  fn disallow_unprefixed_custom_properties() {
    assert_eq!(transform_value("color", "var(foo)"), "1px");
  }

  #[test]
  fn allow_custom_properties() {
    assert_eq!(transform_value("color", "var(--foo)"), "var(--foo)");
    assert_eq!(
        transform_value("backgroundColor", "var(--bar)"),
        "var(--bar)"
    );
    assert_eq!(
        transform_value("transitionProperty", "opacity, margin-top"),
        "opacity,margin-top"
    );

    assert_eq!(
        transform_value("transitionProperty", "opacity, marginTop"),
        "opacity,margin-top"
    );

    assert_eq!(
        transform_value("boxShadow", "0px 2px 4px var(--shadow-1)"),
        "0 2px 4px var(--shadow-1)"
    );

    assert_eq!(
        transform_value("padding", "var(--rightpadding, 20px)"),
        "var(--rightpadding,20px)"
    );
    assert_eq!(
        transform_value(
            "padding",
            "calc((100% - 50px) * 0.5) var(--rightpadding, 20px)"
        ),
        "calc((100% - 50px) * .5) var(--rightpadding,20px)"
    );

    assert_eq!(
        transform_value("margin", "max(0px, (48px - var(--x16dnrjz)) / 2)"),
        "max(0px,(48px - var(--x16dnrjz)) / 2)"
    );

    assert_eq!(
      transform_value("backgroundColor", "var(----__hashed_var__1jqb1tb, revert)"),
      "var(----__hashed_var__1jqb1tb,revert)"
    );

    assert_eq!(
      transform_value(
        "--__hashed_var__1jqb1tb",
        "var(----__hashed_var__1jqb1tb, revert)"
      ),
      "var(----__hashed_var__1jqb1tb,revert)"
    );

    assert_eq!(
      transform_value(
        "boxShadow",
        "1px 1px #000"
      ),
      "1px 1px #000"
    );
  }
}
