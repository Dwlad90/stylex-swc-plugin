#[cfg(test)]
mod normalizers {
  use swc_core::{
    common::{BytePos, Span, DUMMY_SP},
    css::parser::error::{Error, ErrorKind},
  };

  use crate::shared::utils::css::normalizers::base::base_normalizer;
  use crate::shared::utils::css::{
    common::{stringify, swc_parse_css},
    normalizers::whitespace_normalizer::whitespace_normalizer,
  };

  #[test]
  fn should_normalize_transition_property() {
    let (stylesheet, errors) = swc_parse_css("* {{ transitionProperty: opacity, margin-top; }}");

    assert_eq!(
      stringify(&base_normalizer(stylesheet.unwrap(), false)),
      "*{{transitionproperty:opacity,margin-top}}"
    );

    assert_eq!(
      errors,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );
  }

  #[test]
  fn should_normalize_box_shadow() {
    let (stylesheet, errors) = swc_parse_css("* {{ boxShadow: 0px 2px 4px var(--shadow-1); }}");

    assert_eq!(
      stringify(&base_normalizer(stylesheet.unwrap(), false)),
      "*{{boxshadow:0 2px 4px var(--shadow-1)}}"
    );

    assert_eq!(
      errors,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );

    let (stylesheet2, errors2) = swc_parse_css("* {{ boxShadow: 1px 1px #000; }}");

    assert_eq!(
      stringify(&base_normalizer(stylesheet2.unwrap(), false)),
      "*{{boxshadow:1px 1px#000}}"
    );

    assert_eq!(
      errors2,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );
  }

  #[test]
  fn should_normalize_opacity() {
    let (stylesheet, errors) = swc_parse_css("* {{ opacity: 0.5; }}");

    assert_eq!(
      stringify(&base_normalizer(stylesheet.unwrap(), false)),
      "*{{opacity:.5}}"
    );

    assert_eq!(
      errors,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );
  }

  #[test]
  fn should_normalize_transition_duration() {
    let (stylesheet, errors) = swc_parse_css("* {{ transitionDuration: 500ms; }}");

    assert_eq!(
      stringify(&base_normalizer(stylesheet.unwrap(), false)),
      "*{{transitionduration:.5s}}"
    );

    assert_eq!(
      errors,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );
  }

  #[test]
  fn should_normalize_quotes() {
    let (stylesheet, errors) = swc_parse_css(r#"* {{ quotes: '""'; }}"#);

    assert_eq!(
      stringify(&base_normalizer(stylesheet.unwrap(), false)),
      r#"*{{quotes:""}}"#
    );

    assert_eq!(
      errors,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );

    let (stylesheet2, errors2) = swc_parse_css(r#"* {{ quotes: '"123"'; }}"#);

    assert_eq!(
      stringify(&base_normalizer(stylesheet2.unwrap(), false)),
      r#"*{{quotes:"123"}}"#
    );

    assert_eq!(
      errors2,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );
  }

  #[test]
  fn should_normalize_grid_template_areas() {
    let (stylesheet, errors) = swc_parse_css(r#"* {{ gridTemplateAreas: '"content"'; }}"#);

    assert_eq!(
      stringify(&base_normalizer(stylesheet.unwrap(), false)),
      r#"*{{gridtemplateareas:"content"}}"#
    );

    assert_eq!(
      errors,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );

    let (stylesheet2, errors2) =
      swc_parse_css(r#"* {{ gridTemplateAreas: '"content" "sidebar"'; }}"#);

    assert_eq!(
      stringify(&base_normalizer(stylesheet2.unwrap(), false)),
      r#"*{{gridtemplateareas:"content" "sidebar"}}"#
    );

    assert_eq!(
      errors2,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );
  }

  #[test]
  fn should_normalize_color_oklch() {
    let (stylesheet, errors) =
      swc_parse_css("* {{ color: oklch(from var(--xs74gcj) l c h / 0.5) }}");

    assert_eq!(
      stringify(&base_normalizer(stylesheet.unwrap(), false)),
      "*{{color:oklch(from var(--xs74gcj) l c h / 0.5)}}"
    );

    // TODO: remove this once when https://github.com/swc-project/swc/issues/9766 will be fixed
    // Also, COLOR_FUNCTIONS_NON_NORMALIZED_PROPERTY_VALUES should be removed from the codebase
    assert_eq!(
      errors,
      vec![
        Error::new(DUMMY_SP, ErrorKind::InvalidSelector),
        Error::new(
          Span::new(BytePos(38), BytePos(39)),
          ErrorKind::Expected("'none' value of an ident token")
        )
      ]
    );
  }

  #[test]
  fn should_normalize_color_oklab() {
    let (stylesheet, errors) = swc_parse_css(r#"* {{ color: oklab(40.101% 0.1147 0.0453) }}"#);

    assert_eq!(
      whitespace_normalizer(stringify(&base_normalizer(stylesheet.unwrap(), false))),
      r#"oklab(40.101% .1147 .0453)"#
    );

    assert_eq!(
      errors,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );

    let (stylesheet2, errors2) = swc_parse_css(r#"* {{ color: var(--a) var(--b) var(--c) }}"#);

    assert_eq!(
      stringify(&base_normalizer(stylesheet2.unwrap(), false)),
      r#"*{{color:var(--a)var(--b)var(--c)}}"#
    );

    assert_eq!(
      errors2,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );

    let (stylesheet3, errors3) =
      swc_parse_css("* {{ color: oklab(from #0000FF calc(l + 0.1) a b / calc(alpha * 0.9)) }}");

    assert_eq!(
      stringify(&base_normalizer(stylesheet3.unwrap(), false)),
      "*{{color:oklab(from #0000FF calc(l + 0.1) a b / calc(alpha * 0.9))}}"
    );

    // TODO: remove this once when https://github.com/swc-project/swc/issues/9766 will be fixed
    // Also, COLOR_FUNCTIONS_NON_NORMALIZED_PROPERTY_VALUES should be removed from the codebase
    assert_eq!(
      errors3,
      vec![
        Error::new(DUMMY_SP, ErrorKind::InvalidSelector),
        Error::new(
          Span::new(BytePos(36), BytePos(37)),
          ErrorKind::Expected("'e', 'pi', 'infinity', '-infinity' or 'NaN', ident tokens")
        ),
        Error::new(
          Span::new(BytePos(45), BytePos(46)),
          ErrorKind::Expected("'none' value of an ident token")
        )
      ]
    );

    let (stylesheet4, errors4) =
      swc_parse_css("* {{ color: oklab(from hsl(180 100% 50%) calc(l - 0.1) a b) }}");

    assert_eq!(
      stringify(&base_normalizer(stylesheet4.unwrap(), false)),
      "*{{color:oklab(from hsl(180 100% 50%) calc(l - 0.1) a b)}}"
    );

    // TODO: remove this once when https://github.com/swc-project/swc/issues/9766 will be fixed
    // Also, COLOR_FUNCTIONS_NON_NORMALIZED_PROPERTY_VALUES should be removed from the codebase
    assert_eq!(
      errors4,
      vec![
        Error::new(DUMMY_SP, ErrorKind::InvalidSelector),
        Error::new(
          Span::new(BytePos(46), BytePos(47)),
          ErrorKind::Expected("'e', 'pi', 'infinity', '-infinity' or 'NaN', ident tokens")
        ),
        Error::new(
          Span::new(BytePos(55), BytePos(56)),
          ErrorKind::Expected("'none' value of an ident token")
        )
      ]
    );

    let (stylesheet5, errors5) = swc_parse_css("* {{ color: oklab(from green l a b / 0.5) }}");

    assert_eq!(
      stringify(&base_normalizer(stylesheet5.unwrap(), false)),
      "*{{color:oklab(from green l a b / 0.5)}}"
    );

    // TODO: remove this once when https://github.com/swc-project/swc/issues/9766 will be fixed
    // Also, COLOR_FUNCTIONS_NON_NORMALIZED_PROPERTY_VALUES should be removed from the codebase
    assert_eq!(
      errors5,
      vec![
        Error::new(DUMMY_SP, ErrorKind::InvalidSelector),
        Error::new(
          Span::new(BytePos(29), BytePos(30)),
          ErrorKind::Expected("'none' value of an ident token")
        )
      ]
    );
  }
}
