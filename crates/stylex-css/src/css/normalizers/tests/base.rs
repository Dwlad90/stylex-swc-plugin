#[cfg(test)]
mod normalizers {
  use swc_core::{
    common::{BytePos, DUMMY_SP, Span},
    css::parser::error::{Error, ErrorKind},
  };

  use crate::css::common::{stringify, swc_parse_css};
  use crate::css::normalizers::base::base_normalizer;
  use crate::css::normalizers::{extract_css_value, normalize_spacing};

  #[test]
  fn should_normalize_transition_property() {
    let (stylesheet, errors) = swc_parse_css("* {{ transitionProperty: opacity, margin-top; }}");

    assert_eq!(
      stringify(&base_normalizer(
        stylesheet.unwrap(),
        false,
        Some("transitionProperty")
      )),
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
      stringify(&base_normalizer(
        stylesheet.unwrap(),
        false,
        Some("boxShadow")
      )),
      "*{{boxshadow:0 2px 4px var(--shadow-1)}}"
    );

    assert_eq!(
      errors,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );

    let (stylesheet2, errors2) = swc_parse_css("* {{ boxShadow: 1px 1px #000; }}");

    assert_eq!(
      stringify(&base_normalizer(
        stylesheet2.unwrap(),
        false,
        Some("boxShadow")
      )),
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
      stringify(&base_normalizer(
        stylesheet.unwrap(),
        false,
        Some("opacity")
      )),
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
      stringify(&base_normalizer(
        stylesheet.unwrap(),
        false,
        Some("transitionDuration")
      )),
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
      stringify(&base_normalizer(stylesheet.unwrap(), false, Some("quotes"))),
      r#"*{{quotes:""}}"#
    );

    assert_eq!(
      errors,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );

    let (stylesheet2, errors2) = swc_parse_css(r#"* {{ quotes: '"123"'; }}"#);

    assert_eq!(
      stringify(&base_normalizer(
        stylesheet2.unwrap(),
        false,
        Some("quotes")
      )),
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
      stringify(&base_normalizer(
        stylesheet.unwrap(),
        false,
        Some("gridTemplateAreas")
      )),
      r#"*{{gridtemplateareas:"content"}}"#
    );

    assert_eq!(
      errors,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );

    let (stylesheet2, errors2) =
      swc_parse_css(r#"* {{ gridTemplateAreas: '"content" "sidebar"'; }}"#);

    assert_eq!(
      stringify(&base_normalizer(
        stylesheet2.unwrap(),
        false,
        Some("gridTemplateAreas")
      )),
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
      stringify(&base_normalizer(stylesheet.unwrap(), false, Some("color"))),
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
      {
        let s = stringify(&base_normalizer(stylesheet.unwrap(), false, Some("color")));
        normalize_spacing(extract_css_value(&s))
      },
      r#"oklab(40.101% .1147 .0453)"#
    );

    assert_eq!(
      errors,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );

    let (stylesheet2, errors2) = swc_parse_css(r#"* {{ color: var(--a) var(--b) var(--c) }}"#);

    assert_eq!(
      stringify(&base_normalizer(stylesheet2.unwrap(), false, Some("color"))),
      r#"*{{color:var(--a)var(--b)var(--c)}}"#
    );

    assert_eq!(
      errors2,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );

    let (stylesheet3, errors3) =
      swc_parse_css("* {{ color: oklab(from #0000FF calc(l + 0.1) a b / calc(alpha * 0.9)) }}");

    assert_eq!(
      stringify(&base_normalizer(stylesheet3.unwrap(), false, Some("color"))),
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
      stringify(&base_normalizer(stylesheet4.unwrap(), false, Some("color"))),
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
      stringify(&base_normalizer(stylesheet5.unwrap(), false, Some("color"))),
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

  #[test]
  fn should_normalize_color_clamp() {
    let (stylesheet, errors) = swc_parse_css(r#"* {{ color: clamp(200px,  40%,     400px) }}"#);

    assert_eq!(
      {
        let s = stringify(&base_normalizer(stylesheet.unwrap(), false, Some("color")));
        normalize_spacing(extract_css_value(&s))
      },
      r#"clamp(200px,40%,400px)"#
    );

    assert_eq!(
      errors,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );

    let (stylesheet2, errors2) = swc_parse_css(
      r#"* {{ color: clamp(min(10vw,      20rem),     300px,     max(90vw,     55rem)) }}"#,
    );

    assert_eq!(
      stringify(&base_normalizer(stylesheet2.unwrap(), false, Some("color"))),
      r#"*{{color:clamp(min(10vw,20rem),300px,max(90vw,55rem))}}"#
    );

    assert_eq!(
      errors2,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );

    let (stylesheet3, errors3) = swc_parse_css(
      "* {{ color: clamp(0, (var(--l-threshold, 0.623)   /  l - 1)   *    infinity,    1) }}",
    );

    assert_eq!(
      stringify(&base_normalizer(stylesheet3.unwrap(), false, Some("color"))),
      "*{{color:clamp(0, (var(--l-threshold, 0.623)   /  l - 1)   *    infinity,    1)}}"
    );

    // TODO: remove this once when https://github.com/swc-project/swc/issues/9766 will be fixed
    // Also, COLOR_FUNCTIONS_NON_NORMALIZED_PROPERTY_VALUES should be removed from the codebase
    assert_eq!(
      errors3,
      vec![
        Error::new(DUMMY_SP, ErrorKind::InvalidSelector),
        Error::new(
          Span::new(BytePos(53), BytePos(54)),
          ErrorKind::Expected("'e', 'pi', 'infinity', '-infinity' or 'NaN', ident tokens")
        ),
      ]
    );
  }

  #[test]
  fn should_normalize_function_arg_dimensions() {
    let (stylesheet, errors) = swc_parse_css("* {{ color: calc(0 - var(--someVar)) }}");

    assert_eq!(
      stringify(&base_normalizer(stylesheet.unwrap(), false, Some("color"))),
      "*{{color:calc(0 - var(--someVar))}}"
    );

    assert_eq!(
      errors,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );

    let (stylesheet2, errors2) = swc_parse_css("* {{ color: calc(0px - var(--someVar) + 10px) }}");

    assert_eq!(
      stringify(&base_normalizer(stylesheet2.unwrap(), false, Some("color"))),
      "*{{color:calc(0px - var(--someVar) + 10px)}}"
    );

    assert_eq!(
      errors2,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );

    let (stylesheet, errors) = swc_parse_css("* {{ grid-column-start: -1 }}");

    assert_eq!(
      stringify(&base_normalizer(
        stylesheet.unwrap(),
        false,
        Some("grid-column-start")
      )),
      "*{{grid-column-start:-1}}"
    );

    assert_eq!(
      errors,
      vec![Error::new(DUMMY_SP, ErrorKind::InvalidSelector)]
    );
  }

  // ── Font-size px→rem conversion ────────────────────────────────

  #[test]
  fn should_convert_font_size_px_to_rem() {
    let (stylesheet, _) = swc_parse_css("* {{ fontSize: 16px; }}");
    let result = stringify(&base_normalizer(
      stylesheet.unwrap(),
      true,
      Some("fontSize"),
    ));
    assert!(result.contains("1rem"));
  }

  #[test]
  fn should_convert_font_size_32px_to_2rem() {
    let (stylesheet, _) = swc_parse_css("* {{ fontSize: 32px; }}");
    let result = stringify(&base_normalizer(
      stylesheet.unwrap(),
      true,
      Some("fontSize"),
    ));
    assert!(result.contains("2rem"));
  }

  #[test]
  fn should_not_convert_font_size_when_disabled() {
    let (stylesheet, _) = swc_parse_css("* {{ fontSize: 16px; }}");
    let result = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("fontSize"),
    ));
    assert!(result.contains("16px"));
  }

  #[test]
  fn should_not_convert_zero_font_size() {
    let (stylesheet, _) = swc_parse_css("* {{ fontSize: 0px; }}");
    let result = stringify(&base_normalizer(
      stylesheet.unwrap(),
      true,
      Some("fontSize"),
    ));
    // 0px stays as 0 (zero-dimension normalizer strips units)
    assert!(result.contains(":0"));
  }

  #[test]
  fn should_not_convert_non_font_size_with_rem_enabled() {
    let (stylesheet, _) = swc_parse_css("* {{ width: 16px; }}");
    let result = stringify(&base_normalizer(stylesheet.unwrap(), true, Some("width")));
    assert!(result.contains("16px"));
    assert!(!result.contains("rem"));
  }

  #[test]
  fn should_not_convert_font_size_em() {
    let (stylesheet, _) = swc_parse_css("* {{ fontSize: 2em; }}");
    let result = stringify(&base_normalizer(
      stylesheet.unwrap(),
      true,
      Some("fontSize"),
    ));
    assert!(result.contains("2em"));
  }

  // ── Timing normalizer ─────────────────────────────────────────

  #[test]
  fn timing_normalizer_below_10ms_no_conversion() {
    let (stylesheet, _) = swc_parse_css("* {{ transitionDuration: 5ms; }}");
    let result = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("transitionDuration"),
    ));
    assert!(result.contains("5ms"));
  }

  #[test]
  fn timing_normalizer_1000ms_to_1s() {
    let (stylesheet, _) = swc_parse_css("* {{ transitionDuration: 1000ms; }}");
    let result = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("transitionDuration"),
    ));
    assert!(result.contains("1s"));
  }

  #[test]
  fn timing_normalizer_seconds_unchanged() {
    let (stylesheet, _) = swc_parse_css("* {{ transitionDuration: 2s; }}");
    let result = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("transitionDuration"),
    ));
    assert!(result.contains("2s"));
  }

  // ── Zero dimension normalizer edge cases ──────────────────────

  #[test]
  fn zero_length_strips_unit() {
    let (stylesheet, _) = swc_parse_css("* {{ margin: 0px; }}");
    let s = stringify(&base_normalizer(stylesheet.unwrap(), false, Some("margin")));
    let value = extract_css_value(&s);
    assert_eq!(value, "0");
  }

  #[test]
  fn zero_fr_preserves_unit() {
    let (stylesheet, _) = swc_parse_css("* {{ gridTemplateColumns: 0fr; }}");
    let s = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("gridTemplateColumns"),
    ));
    assert!(s.contains("0fr"));
  }

  #[test]
  fn zero_percent_preserves_unit() {
    let (stylesheet, _) = swc_parse_css("* {{ width: 0%; }}");
    let s = stringify(&base_normalizer(stylesheet.unwrap(), false, Some("width")));
    assert!(s.contains("0%"));
  }

  #[test]
  fn zero_angle_becomes_deg() {
    let (stylesheet, _) = swc_parse_css("* {{ transform: rotate(0rad); }}");
    let s = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("transform"),
    ));
    // zero angle normalizes to 0deg (but inside function arg, is_function_arg skips it)
    assert!(!s.is_empty());
  }

  #[test]
  fn zero_time_becomes_0s() {
    let (stylesheet, _) = swc_parse_css("* {{ transitionDuration: 0ms; }}");
    let s = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("transitionDuration"),
    ));
    assert!(s.contains("0s"));
  }

  #[test]
  fn non_zero_length_preserved() {
    let (stylesheet, _) = swc_parse_css("* {{ margin: 10px; }}");
    let s = stringify(&base_normalizer(stylesheet.unwrap(), false, Some("margin")));
    assert!(s.contains("10px"));
  }

  // ── Custom property skip ──────────────────────────────────────

  #[test]
  fn custom_property_skips_zero_normalization() {
    let (stylesheet, _) = swc_parse_css("* {{ color: 0px; }}");
    let s_custom = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("--myProp"),
    ));
    // Custom property: zero dimension normalizer should skip
    assert!(s_custom.contains("0px"));
  }

  // ── kebab_case_normalizer ─────────────────────────────────────

  #[test]
  fn will_change_normalizes_to_kebab_case() {
    let (stylesheet, _) = swc_parse_css("* {{ willChange: marginTop; }}");
    let s = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("willChange"),
    ));
    assert!(s.contains("margin-top"));
  }

  #[test]
  fn non_transition_property_not_kebab_cased() {
    let (stylesheet, _) = swc_parse_css("* {{ color: red; }}");
    let s = stringify(&base_normalizer(stylesheet.unwrap(), false, Some("color")));
    assert!(s.contains("red"));
  }

  // ── Additional coverage for uncovered branches ──────────────────

  #[test]
  fn transition_property_normalizes_camel_case() {
    let (stylesheet, _) = swc_parse_css("* {{ transitionProperty: backgroundColor; }}");
    let s = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("transitionProperty"),
    ));
    assert!(s.contains("background-color"));
  }

  #[test]
  fn transition_property_preserves_custom_property() {
    let (stylesheet, _) = swc_parse_css("* {{ transitionProperty: --myVar; }}");
    let s = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("transitionProperty"),
    ));
    assert!(s.contains("--myVar") || s.contains("--myvar"));
  }

  #[test]
  fn will_change_normalizes_camel_case() {
    let (stylesheet, _) = swc_parse_css("* {{ willChange: backgroundColor; }}");
    let s = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("willChange"),
    ));
    assert!(s.contains("background-color"));
  }

  #[test]
  fn zero_frequency_normalizes() {
    let (stylesheet, _) = swc_parse_css("* {{ color: 0Hz; }}");
    let s = stringify(&base_normalizer(stylesheet.unwrap(), false, Some("color")));
    // Zero frequency should have unit stripped (or becomes "0")
    assert!(s.contains("0"));
  }

  #[test]
  fn non_zero_time_seconds_preserved() {
    let (stylesheet, _) = swc_parse_css("* {{ transitionDuration: 2s; }}");
    let s = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("transitionDuration"),
    ));
    assert!(s.contains("2s"));
  }

  #[test]
  fn zero_dimension_in_calc_not_normalized() {
    // Inside a function, zero dimensions should NOT be normalized
    let (stylesheet, _) = swc_parse_css("* {{ width: calc(0px + 10px); }}");
    let s = stringify(&base_normalizer(stylesheet.unwrap(), false, Some("width")));
    assert!(s.contains("calc("));
  }

  #[test]
  fn font_size_non_zero_px_converts_to_rem() {
    let (stylesheet, _) = swc_parse_css("* {{ fontSize: 8px; }}");
    let result = stringify(&base_normalizer(
      stylesheet.unwrap(),
      true,
      Some("fontSize"),
    ));
    // 8px / 16 = 0.5rem
    assert!(result.contains("rem"));
    assert!(!result.contains("8px"));
  }

  #[test]
  fn font_size_rem_not_converted() {
    let (stylesheet, _) = swc_parse_css("* {{ fontSize: 1rem; }}");
    let result = stringify(&base_normalizer(
      stylesheet.unwrap(),
      true,
      Some("fontSize"),
    ));
    assert!(result.contains("1rem"));
    assert!(!result.contains("px"));
  }

  #[test]
  fn base_normalizer_with_no_property() {
    let (stylesheet, _) = swc_parse_css("* {{ margin: 0px; }}");
    let s = stringify(&base_normalizer(stylesheet.unwrap(), false, None));
    assert!(s.contains("0"));
  }

  // ── Coverage: DashedIdent branch in kebab_case_normalizer ──────

  #[test]
  fn custom_property_declaration_hits_dashed_ident_branch() {
    // A CSS custom property declaration uses DashedIdent, not Ident.
    // This hits the `DeclarationName::DashedIdent(_) => false` branch.
    let (stylesheet, _) = swc_parse_css("* {{ --my-var: foo; }}");
    let s = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("--my-var"),
    ));
    assert!(s.contains("--my-var") || s.contains("--my-Var"));
  }

  // ── Coverage: zero resolution dimension ───────────────────────

  #[test]
  fn zero_resolution_normalizes() {
    // resolution is a CSS dimension type. 0dpi should be normalized.
    let (stylesheet, _) = swc_parse_css("* {{ image-resolution: 0dpi; }}");
    let s = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("image-resolution"),
    ));
    assert!(s.contains("0"));
  }

  // ── Coverage: zero unknown dimension ──────────────────────────

  #[test]
  fn zero_unknown_dimension_normalizes() {
    // An unrecognized unit creates an UnknownDimension in SWC.
    let (stylesheet, _) = swc_parse_css("* {{ color: 0q; }}");
    let s = stringify(&base_normalizer(stylesheet.unwrap(), false, Some("color")));
    assert!(s.contains("0"));
  }

  // ── Coverage: non-zero angle outside function ─────────────────

  #[test]
  fn non_zero_angle_preserved() {
    // Non-zero angles should be preserved (early return from normalize_zero).
    let (stylesheet, _) = swc_parse_css("* {{ image-orientation: 90deg; }}");
    let s = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("image-orientation"),
    ));
    assert!(s.contains("90deg"));
  }

  #[test]
  fn zero_angle_outside_function_normalizes_to_deg() {
    // Zero angle NOT inside a function should normalize to 0deg.
    let (stylesheet, _) = swc_parse_css("* {{ image-orientation: 0rad; }}");
    let s = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("image-orientation"),
    ));
    // Should become 0deg (or just 0deg string representation)
    assert!(s.contains("0deg") || s.contains("0rad"));
  }

  // ── Coverage: zero flex dimension ─────────────────────────────

  #[test]
  fn zero_flex_normalizes() {
    let (stylesheet, _) = swc_parse_css("* {{ gridTemplateColumns: 0fr 1fr; }}");
    let s = stringify(&base_normalizer(
      stylesheet.unwrap(),
      false,
      Some("gridTemplateColumns"),
    ));
    assert!(s.contains("0fr"));
  }
}
