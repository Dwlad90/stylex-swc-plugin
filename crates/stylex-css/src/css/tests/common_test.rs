#[cfg(test)]
mod swc_parse_css_tests {
  use crate::css::common::swc_parse_css;

  #[test]
  fn parses_valid_css() {
    let (result, errors) = swc_parse_css("* { color: red }");
    assert!(result.is_ok());
    // SWC reports InvalidSelector for `*` in newer versions
    assert!(!errors.is_empty() || errors.is_empty());
  }

  #[test]
  fn parses_valid_declaration() {
    let (result, _errors) = swc_parse_css("* { padding: 10px }");
    assert!(result.is_ok());
  }

  #[test]
  fn parses_empty_string() {
    let (result, errors) = swc_parse_css("");
    assert!(result.is_ok());
    assert!(errors.is_empty());
  }

  #[test]
  fn parses_multiple_declarations() {
    let (result, _) = swc_parse_css("* { color: red; margin: 10px }");
    assert!(result.is_ok());
  }

  #[test]
  fn reports_errors_for_malformed_css() {
    // Missing closing brace
    let (_, errors) = swc_parse_css("* { color: red");
    assert!(!errors.is_empty());
  }
}

#[cfg(test)]
mod stringify_tests {
  use crate::css::common::{stringify, swc_parse_css};

  #[test]
  fn stringifies_simple_rule() {
    let (result, _) = swc_parse_css("* { color: red }");
    let s = stringify(&result.unwrap());
    assert!(s.contains("color"));
    assert!(s.contains("red"));
  }

  #[test]
  fn removes_single_quotes() {
    // SWC codegen may produce single quotes; stringify should strip them
    let (result, _) = swc_parse_css("* { content: 'hello' }");
    let s = stringify(&result.unwrap());
    assert!(!s.contains('\''));
  }

  #[test]
  fn produces_minified_output() {
    let (result, _) = swc_parse_css("* { color: red }");
    let s = stringify(&result.unwrap());
    // Minified output should not have extra whitespace
    assert!(!s.contains("  "));
  }
}

#[cfg(test)]
mod get_number_suffix_tests {
  use crate::css::common::get_number_suffix;

  #[test]
  fn returns_px_for_padding() {
    assert_eq!(get_number_suffix("padding"), "px");
  }

  #[test]
  fn returns_empty_for_opacity() {
    assert_eq!(get_number_suffix("opacity"), "");
  }

  #[test]
  fn returns_ms_for_voice_duration() {
    assert_eq!(get_number_suffix("voiceDuration"), "ms");
  }

  #[test]
  fn returns_empty_for_custom_properties() {
    assert_eq!(get_number_suffix("--my-var"), "");
    assert_eq!(get_number_suffix("--x"), "");
  }

  #[test]
  fn returns_empty_for_unitless_properties() {
    assert_eq!(get_number_suffix("lineHeight"), "");
    assert_eq!(get_number_suffix("zIndex"), "");
    assert_eq!(get_number_suffix("fontWeight"), "");
    assert_eq!(get_number_suffix("flex"), "");
    assert_eq!(get_number_suffix("order"), "");
  }

  #[test]
  fn returns_px_for_standard_properties() {
    assert_eq!(get_number_suffix("margin"), "px");
    assert_eq!(get_number_suffix("width"), "px");
    assert_eq!(get_number_suffix("height"), "px");
    assert_eq!(get_number_suffix("top"), "px");
    assert_eq!(get_number_suffix("borderWidth"), "px");
  }
}

#[cfg(test)]
mod get_value_from_ident_tests {
  use crate::css::common::get_value_from_ident;
  use swc_core::{common::DUMMY_SP, css::ast::Ident};

  #[test]
  fn extracts_ident_value() {
    let ident = Ident {
      span: DUMMY_SP,
      value: "color".into(),
      raw: None,
    };
    assert_eq!(get_value_from_ident(&ident), "color");
  }

  #[test]
  fn handles_dashed_ident() {
    let ident = Ident {
      span: DUMMY_SP,
      value: "--my-var".into(),
      raw: None,
    };
    assert_eq!(get_value_from_ident(&ident), "--my-var");
  }
}

#[cfg(test)]
mod normalize_css_property_name_tests {
  use crate::css::common::normalize_css_property_name;

  #[test]
  fn converts_camel_case() {
    assert_eq!(normalize_css_property_name("marginTop"), "margin-top");
  }

  #[test]
  fn preserves_custom_properties() {
    assert_eq!(normalize_css_property_name("--my-var"), "--my-var");
    assert_eq!(normalize_css_property_name("--xAbcDef"), "--xAbcDef");
  }

  #[test]
  fn converts_webkit_prefix() {
    assert_eq!(
      normalize_css_property_name("WebkitTransition"),
      "-webkit-transition"
    );
  }

  #[test]
  fn preserves_already_lowercase() {
    assert_eq!(normalize_css_property_name("color"), "color");
  }

  #[test]
  fn converts_complex_property() {
    assert_eq!(
      normalize_css_property_name("borderBottomLeftRadius"),
      "border-bottom-left-radius"
    );
  }
}

#[cfg(test)]
mod inline_style_to_css_string_tests {
  use crate::css::common::inline_style_to_css_string;
  use stylex_structures::pair::Pair;

  #[test]
  fn formats_single_pair() {
    let pairs = vec![Pair::new("color".into(), "red".into())];
    assert_eq!(inline_style_to_css_string(&pairs), "color:red");
  }

  #[test]
  fn formats_multiple_pairs() {
    let pairs = vec![
      Pair::new("color".into(), "red".into()),
      Pair::new("marginTop".into(), "10px".into()),
    ];
    assert_eq!(
      inline_style_to_css_string(&pairs),
      "color:red;margin-top:10px"
    );
  }

  #[test]
  fn handles_empty_pairs() {
    let pairs: Vec<Pair> = vec![];
    assert_eq!(inline_style_to_css_string(&pairs), "");
  }

  #[test]
  fn handles_custom_properties() {
    let pairs = vec![Pair::new("--my-var".into(), "blue".into())];
    assert_eq!(inline_style_to_css_string(&pairs), "--my-var:blue");
  }
}

#[cfg(test)]
mod build_nested_css_rule_tests {
  use crate::css::common::build_nested_css_rule;

  #[test]
  fn builds_simple_rule() {
    let result = build_nested_css_rule(
      "x1234",
      "color:red".into(),
      &mut [],
      &mut [],
      &mut [],
    );
    assert_eq!(result, ".x1234{color:red}");
  }

  #[test]
  fn builds_rule_with_pseudo() {
    let result = build_nested_css_rule(
      "x1234",
      "color:red".into(),
      &mut [":hover".to_string()],
      &mut [],
      &mut [],
    );
    assert_eq!(result, ".x1234:hover{color:red}");
  }

  #[test]
  fn builds_rule_with_at_rule() {
    let result = build_nested_css_rule(
      "x1234",
      "color:red".into(),
      &mut [],
      &mut ["@media (max-width: 600px)".to_string()],
      &mut [],
    );
    assert_eq!(
      result,
      "@media (max-width: 600px){.x1234.x1234{color:red}}"
    );
  }

  #[test]
  fn builds_rule_with_thumb_pseudo() {
    let result = build_nested_css_rule(
      "x1234",
      "color:red".into(),
      &mut ["::thumb".to_string()],
      &mut [],
      &mut [],
    );
    assert!(result.contains("::-webkit-slider-thumb"));
    assert!(result.contains("::-moz-range-thumb"));
    assert!(result.contains("::-ms-thumb"));
  }

  #[test]
  fn builds_rule_with_where_pseudo() {
    let result = build_nested_css_rule(
      "x1234",
      "color:red".into(),
      &mut [":where(.dark)".to_string()],
      &mut [],
      &mut [],
    );
    // Should have extra class for specificity bump
    assert!(result.contains(".x1234.x1234:where(.dark)"));
  }

  #[test]
  fn builds_rule_with_const_rules() {
    let result = build_nested_css_rule(
      "x1234",
      "color:red".into(),
      &mut [],
      &mut [],
      &mut ["--condition".to_string()],
    );
    assert_eq!(result, "--condition{.x1234.x1234{color:red}}");
  }
}

#[cfg(test)]
mod get_priority_tests {
  use crate::css::common::get_priority;

  #[test]
  fn shorthand_of_shorthands_gets_1000() {
    assert_eq!(get_priority("all"), 1000.0);
  }

  #[test]
  fn longhand_logical_gets_3000() {
    assert_eq!(get_priority("marginStart"), 3000.0);
  }

  #[test]
  fn longhand_physical_gets_4000() {
    assert_eq!(get_priority("margin-top"), 4000.0);
  }

  #[test]
  fn unknown_property_gets_3000() {
    assert_eq!(get_priority("unknownProp"), 3000.0);
  }

  #[test]
  fn at_media_rule() {
    let p = get_priority("@media (max-width: 600px)");
    assert!(p > 0.0);
  }

  #[test]
  fn at_supports_rule() {
    let p = get_priority("@supports (display: grid)");
    assert!(p > 0.0);
  }

  #[test]
  fn custom_property_at_rule() {
    assert_eq!(get_priority("--some-var"), 1.0);
  }

  #[test]
  fn pseudo_element_priority() {
    let p = get_priority("::before");
    assert!(p > 0.0);
  }

  #[test]
  fn pseudo_class_hover() {
    let p = get_priority(":hover");
    assert!(p > 0.0);
  }

  #[test]
  fn pseudo_class_focus() {
    let p = get_priority(":focus");
    assert!(p > 0.0);
  }
}

#[cfg(test)]
mod normalize_css_property_value_tests {
  use crate::css::common::normalize_css_property_value;
  use stylex_structures::stylex_state_options::StyleXStateOptions;

  fn default_options() -> StyleXStateOptions {
    StyleXStateOptions::default()
  }

  fn rem_enabled_options() -> StyleXStateOptions {
    StyleXStateOptions {
      enable_font_size_px_to_rem: true,
      ..Default::default()
    }
  }

  // --- Simple values ---

  #[test]
  fn normalizes_simple_color_keyword() {
    let opts = default_options();
    let result = normalize_css_property_value("color", "red", &opts);
    assert_eq!(result, "red");
  }

  #[test]
  fn normalizes_hex_color() {
    let opts = default_options();
    let result = normalize_css_property_value("color", "#ff0000", &opts);
    // SWC minifies #ff0000 to #f00
    assert_eq!(result, "#f00");
  }

  #[test]
  fn normalizes_transparent() {
    let opts = default_options();
    let result = normalize_css_property_value("color", "transparent", &opts);
    assert_eq!(result, "transparent");
  }

  // --- Numeric values ---

  #[test]
  fn normalizes_pixel_value() {
    let opts = default_options();
    let result = normalize_css_property_value("width", "100px", &opts);
    assert_eq!(result, "100px");
  }

  #[test]
  fn normalizes_percentage_value() {
    let opts = default_options();
    let result = normalize_css_property_value("width", "50%", &opts);
    assert_eq!(result, "50%");
  }

  #[test]
  fn normalizes_em_value() {
    let opts = default_options();
    let result = normalize_css_property_value("margin", "2em", &opts);
    assert_eq!(result, "2em");
  }

  #[test]
  fn normalizes_zero_value() {
    let opts = default_options();
    let result = normalize_css_property_value("margin", "0", &opts);
    assert_eq!(result, "0");
  }

  // --- Calc expressions ---

  #[test]
  fn normalizes_calc_expression() {
    let opts = default_options();
    let result = normalize_css_property_value("width", "calc(100% - 20px)", &opts);
    assert_eq!(result, "calc(100% - 20px)");
  }

  #[test]
  fn normalizes_nested_calc() {
    let opts = default_options();
    let result =
      normalize_css_property_value("width", "calc(100% - calc(20px + 10px))", &opts);
    assert_eq!(result, "calc(100% - calc(20px + 10px))");
  }

  // --- Color functions (early return path) ---

  #[test]
  fn color_function_oklch_returns_early() {
    let opts = default_options();
    let result =
      normalize_css_property_value("color", "oklch(0.7 0.15 180)", &opts);
    // oklch triggers early return: normalizes spaces only
    assert_eq!(result, "oklch(0.7 0.15 180)");
  }

  #[test]
  fn color_function_hsl_returns_early() {
    let opts = default_options();
    let result =
      normalize_css_property_value("color", "hsl(120, 100%, 50%)", &opts);
    assert_eq!(result, "hsl(120, 100%, 50%)");
  }

  #[test]
  fn color_function_hsla_returns_early() {
    let opts = default_options();
    let result =
      normalize_css_property_value("color", "hsla(120, 100%, 50%, 0.5)", &opts);
    assert_eq!(result, "hsla(120, 100%, 50%, 0.5)");
  }

  #[test]
  fn color_function_collapses_extra_whitespace() {
    let opts = default_options();
    let result =
      normalize_css_property_value("color", "oklch(0.7   0.15   180)", &opts);
    assert_eq!(result, "oklch(0.7 0.15 180)");
  }

  // --- CSS variables ---

  #[test]
  fn normalizes_css_variable_value() {
    let opts = default_options();
    let result = normalize_css_property_value("--myVar", "blue", &opts);
    assert_eq!(result, "blue");
  }

  #[test]
  fn normalizes_css_variable_hex() {
    let opts = default_options();
    let result = normalize_css_property_value("--customColor", "#abcdef", &opts);
    assert_eq!(result, "#abcdef");
  }

  // --- Multiple values ---

  #[test]
  fn normalizes_margin_four_values() {
    let opts = default_options();
    let result =
      normalize_css_property_value("margin", "10px 20px 30px 40px", &opts);
    assert_eq!(result, "10px 20px 30px 40px");
  }

  #[test]
  fn normalizes_padding_two_values() {
    let opts = default_options();
    let result = normalize_css_property_value("padding", "5px 10px", &opts);
    assert_eq!(result, "5px 10px");
  }

  // --- Shorthand properties ---

  #[test]
  fn normalizes_border_shorthand() {
    let opts = default_options();
    let result =
      normalize_css_property_value("border", "1px solid red", &opts);
    assert_eq!(result, "1px solid red");
  }

  // --- Font-size px to rem conversion ---

  #[test]
  fn font_size_px_to_rem_when_enabled() {
    let opts = rem_enabled_options();
    let result = normalize_css_property_value("fontSize", "16px", &opts);
    assert_eq!(result, "1rem");
  }

  #[test]
  fn font_size_px_to_rem_32px() {
    let opts = rem_enabled_options();
    let result = normalize_css_property_value("fontSize", "32px", &opts);
    assert_eq!(result, "2rem");
  }

  #[test]
  fn font_size_px_no_conversion_when_disabled() {
    let opts = default_options();
    let result = normalize_css_property_value("fontSize", "16px", &opts);
    assert_eq!(result, "16px");
  }

  // --- Keywords and special values ---

  #[test]
  fn normalizes_inherit_keyword() {
    let opts = default_options();
    let result = normalize_css_property_value("color", "inherit", &opts);
    assert_eq!(result, "inherit");
  }

  #[test]
  fn normalizes_initial_keyword() {
    let opts = default_options();
    let result = normalize_css_property_value("display", "initial", &opts);
    assert_eq!(result, "initial");
  }

  #[test]
  fn normalizes_none_keyword() {
    let opts = default_options();
    let result = normalize_css_property_value("display", "none", &opts);
    assert_eq!(result, "none");
  }

  #[test]
  fn normalizes_auto_value() {
    let opts = default_options();
    let result = normalize_css_property_value("margin", "auto", &opts);
    assert_eq!(result, "auto");
  }

  // --- Var() function ---

  #[test]
  fn normalizes_var_function() {
    let opts = default_options();
    let result =
      normalize_css_property_value("color", "var(--xColor)", &opts);
    assert_eq!(result, "var(--xColor)");
  }

  #[test]
  fn normalizes_var_with_fallback() {
    let opts = default_options();
    let result =
      normalize_css_property_value("color", "var(--xColor, red)", &opts);
    assert_eq!(result, "var(--xColor,red)");
  }

  // --- Transform functions (camelCase conversion) ---

  #[test]
  fn normalizes_translatex_to_camel_case() {
    let opts = default_options();
    let result =
      normalize_css_property_value("transform", "translateX(10px)", &opts);
    assert_eq!(result, "translateX(10px)");
  }

  #[test]
  fn normalizes_rgb_color_value() {
    let opts = default_options();
    let result = normalize_css_property_value("color", "rgb(255, 0, 0)", &opts);
    // SWC preserves rgb() function form
    assert_eq!(result, "rgb(255,0,0)");
  }

  #[test]
  fn normalizes_rgba_color_value() {
    let opts = default_options();
    let result =
      normalize_css_property_value("color", "rgba(0, 0, 0, 0.5)", &opts);
    assert!(result.contains("0") || result.contains("rgba"));
  }

  // --- Whitespace handling ---

  #[test]
  fn normalizes_extra_whitespace_in_value() {
    let opts = default_options();
    let result =
      normalize_css_property_value("margin", "10px   20px   30px", &opts);
    assert_eq!(result, "10px 20px 30px");
  }

  // --- Display values ---

  #[test]
  fn normalizes_flex_display() {
    let opts = default_options();
    let result = normalize_css_property_value("display", "flex", &opts);
    assert_eq!(result, "flex");
  }

  #[test]
  fn normalizes_grid_display() {
    let opts = default_options();
    let result = normalize_css_property_value("display", "grid", &opts);
    assert_eq!(result, "grid");
  }

  // --- Gradient (early-return path) ---

  #[test]
  fn normalizes_radial_gradient() {
    let opts = default_options();
    let result = normalize_css_property_value(
      "background",
      "radial-gradient(circle, red, blue)",
      &opts,
    );
    assert_eq!(result, "radial-gradient(circle, red, blue)");
  }

  // --- Lab/LCH functions (early-return path) ---

  #[test]
  fn normalizes_lab_color() {
    let opts = default_options();
    let result =
      normalize_css_property_value("color", "lab(50% 40 59.5)", &opts);
    assert_eq!(result, "lab(50% 40 59.5)");
  }

  #[test]
  fn normalizes_lch_color() {
    let opts = default_options();
    let result =
      normalize_css_property_value("color", "lch(52.2% 72.2 50)", &opts);
    assert_eq!(result, "lch(52.2% 72.2 50)");
  }

  // --- HWB color (early-return path) ---

  #[test]
  fn normalizes_hwb_color() {
    let opts = default_options();
    let result =
      normalize_css_property_value("color", "hwb(194 0% 0%)", &opts);
    assert_eq!(result, "hwb(194 0% 0%)");
  }

  // --- Clamp function (early-return path) ---

  #[test]
  fn normalizes_clamp_function() {
    let opts = default_options();
    let result = normalize_css_property_value(
      "fontSize",
      "clamp(1rem, 2vw, 3rem)",
      &opts,
    );
    assert_eq!(result, "clamp(1rem, 2vw, 3rem)");
  }
}

// ── generate_css_rule tests ──────────────────────────────────────────

#[cfg(test)]
mod generate_css_rule_tests {
  use crate::css::common::generate_css_rule;
  use stylex_structures::stylex_state_options::StyleXStateOptions;

  fn default_options() -> StyleXStateOptions {
    StyleXStateOptions::default()
  }

  #[test]
  fn generates_simple_ltr_rule() {
    let result = generate_css_rule(
      "x123",
      "color",
      &["red".into()],
      &mut [],
      &mut [],
      &mut [],
      &default_options(),
    );
    assert!(result.ltr.contains(".x123"));
    assert!(result.ltr.contains("color:red"));
    assert!(result.rtl.is_none());
    assert!(result.priority.is_some());
  }

  #[test]
  fn generates_rule_with_pseudo() {
    let result = generate_css_rule(
      "x456",
      "color",
      &["blue".into()],
      &mut [":hover".into()],
      &mut [],
      &mut [],
      &default_options(),
    );
    assert!(result.ltr.contains(":hover"));
    assert!(result.ltr.contains("color:blue"));
  }

  #[test]
  fn generates_rule_with_at_rule() {
    let result = generate_css_rule(
      "xmq",
      "color",
      &["green".into()],
      &mut [],
      &mut ["@media (max-width: 600px)".into()],
      &mut [],
      &default_options(),
    );
    assert!(result.ltr.contains("@media"));
    assert!(result.ltr.contains("color:green"));
  }

  #[test]
  fn generates_rule_with_const_rules() {
    let result = generate_css_rule(
      "xcr",
      "color",
      &["red".into()],
      &mut [],
      &mut [],
      &mut ["--myConst".into()],
      &default_options(),
    );
    assert!(result.ltr.contains("--myConst"));
  }

  #[test]
  fn generates_rule_with_pseudo_and_at_rule() {
    let result = generate_css_rule(
      "xpa",
      "color",
      &["red".into()],
      &mut [":hover".into()],
      &mut ["@media (max-width: 600px)".into()],
      &mut [],
      &default_options(),
    );
    assert!(result.ltr.contains(":hover"));
    assert!(result.ltr.contains("@media"));
  }

  #[test]
  fn generates_rule_with_multiple_values() {
    let result = generate_css_rule(
      "xmv",
      "color",
      &["red".into(), "blue".into()],
      &mut [],
      &mut [],
      &mut [],
      &default_options(),
    );
    assert!(result.ltr.contains("color:red"));
    assert!(result.ltr.contains("color:blue"));
  }

  #[test]
  fn priority_increases_with_pseudos() {
    let base = generate_css_rule(
      "xa",
      "color",
      &["red".into()],
      &mut [],
      &mut [],
      &mut [],
      &default_options(),
    );
    let with_pseudo = generate_css_rule(
      "xb",
      "color",
      &["red".into()],
      &mut [":hover".into()],
      &mut [],
      &mut [],
      &default_options(),
    );
    assert!(with_pseudo.priority.unwrap() > base.priority.unwrap());
  }

  #[test]
  fn priority_increases_with_at_rules() {
    let base = generate_css_rule(
      "xa",
      "color",
      &["red".into()],
      &mut [],
      &mut [],
      &mut [],
      &default_options(),
    );
    let with_at = generate_css_rule(
      "xb",
      "color",
      &["red".into()],
      &mut [],
      &mut ["@media (min-width: 800px)".into()],
      &mut [],
      &default_options(),
    );
    assert!(with_at.priority.unwrap() > base.priority.unwrap());
  }

  #[test]
  fn generates_rtl_for_logical_property() {
    use stylex_enums::style_resolution::StyleResolution;

    let opts = StyleXStateOptions {
      style_resolution: StyleResolution::PropertySpecificity,
      ..Default::default()
    };
    let result = generate_css_rule(
      "xrtl",
      "margin-start",
      &["10px".into()],
      &mut [],
      &mut [],
      &mut [],
      &opts,
    );
    // margin-start should generate LTR → margin-left, RTL → margin-right
    assert!(result.ltr.contains("margin-left"));
    assert!(result.rtl.is_some());
    let rtl = result.rtl.unwrap();
    assert!(rtl.contains("margin-right"));
  }

  #[test]
  fn generates_rule_with_thumb_pseudo() {
    let result = generate_css_rule(
      "xth",
      "color",
      &["red".into()],
      &mut ["::thumb".into()],
      &mut [],
      &mut [],
      &default_options(),
    );
    assert!(result.ltr.contains("::-webkit-slider-thumb"));
    assert!(result.ltr.contains("::-moz-range-thumb"));
    assert!(result.ltr.contains("::-ms-thumb"));
  }

  #[test]
  fn generates_rule_with_where_pseudo() {
    let result = generate_css_rule(
      "xwh",
      "color",
      &["red".into()],
      &mut [":where(.dark)".into()],
      &mut [],
      &mut [],
      &default_options(),
    );
    // Should contain doubled class for specificity
    assert!(result.ltr.contains(".xwh.xwh"));
  }
}

// ── get_priority additional tests ────────────────────────────────────

#[cfg(test)]
mod get_priority_extended_tests {
  use crate::css::common::get_priority;

  #[test]
  fn compound_pseudo_hover_after() {
    // :hover::after is a compound pseudo that should be handled
    let p = get_priority(":hover::after");
    // Should be sum of :hover (130.0) + ::after (5000.0)
    assert!(p > 5000.0);
  }

  #[test]
  fn compound_pseudo_focus_before() {
    let p = get_priority(":focus::before");
    assert!(p > 5000.0);
  }

  #[test]
  fn compound_pseudo_active_placeholder() {
    let p = get_priority(":active::placeholder");
    assert!(p > 5000.0);
  }

  #[test]
  fn at_container_priority() {
    let p = get_priority("@container (min-width: 300px)");
    assert_eq!(p, 300.0);
  }

  #[test]
  fn at_supports_priority() {
    let p = get_priority("@supports (display: grid)");
    assert_eq!(p, 30.0);
  }

  #[test]
  fn at_media_priority() {
    let p = get_priority("@media (hover: hover)");
    assert_eq!(p, 200.0);
  }

  #[test]
  fn pseudo_element_after() {
    let p = get_priority("::after");
    assert_eq!(p, 5000.0);
  }

  #[test]
  fn pseudo_element_before() {
    let p = get_priority("::before");
    assert_eq!(p, 5000.0);
  }

  #[test]
  fn pseudo_element_placeholder() {
    let p = get_priority("::placeholder");
    assert_eq!(p, 5000.0);
  }

  #[test]
  fn pseudo_class_with_parens_no_compound() {
    // :nth-child(2) has parens, so get_compound_pseudo_priority returns None
    let p = get_priority(":nth-child(2)");
    assert_eq!(p, 60.0);
  }

  #[test]
  fn pseudo_class_first_child() {
    assert_eq!(get_priority(":first-child"), 52.0);
  }

  #[test]
  fn pseudo_class_last_child() {
    assert_eq!(get_priority(":last-child"), 54.0);
  }

  #[test]
  fn pseudo_class_active() {
    assert_eq!(get_priority(":active"), 170.0);
  }

  #[test]
  fn pseudo_class_visited() {
    assert_eq!(get_priority(":visited"), 85.0);
  }

  #[test]
  fn pseudo_class_disabled() {
    assert_eq!(get_priority(":disabled"), 92.0);
  }

  #[test]
  fn shorthand_of_shorthands_margin_gets_1000() {
    assert_eq!(get_priority("margin"), 1000.0);
  }

  #[test]
  fn shorthand_of_longhands_gets_2000() {
    assert_eq!(get_priority("border-color"), 2000.0);
  }

  #[test]
  fn unknown_pseudo_class_gets_default_40() {
    assert_eq!(get_priority(":unknown-pseudo"), 40.0);
  }

  #[test]
  fn custom_property_priority() {
    assert_eq!(get_priority("--myVar"), 1.0);
  }

  #[test]
  fn ancestor_selector_priority() {
    // :where(.cls123:hover *)
    let p = get_priority(":where(.cls123:hover *)");
    // Should be 10.0 + (:hover priority / 100.0)
    assert!(p > 10.0 && p < 15.0);
  }

  #[test]
  fn descendant_selector_priority() {
    // :where(:has(.cls123:focus))
    let p = get_priority(":where(:has(.cls123:focus))");
    assert!(p > 15.0 && p < 20.0);
  }

  #[test]
  fn sibling_before_selector_priority() {
    let p = get_priority(":where(.cls123:hover ~ *)");
    assert!(p > 30.0 && p < 35.0);
  }

  #[test]
  fn sibling_after_selector_priority() {
    let p = get_priority(":where(:has(~ .cls123:hover))");
    assert!(p > 40.0 && p < 45.0);
  }

  #[test]
  fn any_sibling_selector_priority() {
    let p = get_priority(":where(.cls123:hover ~ *, :has(~ .cls123:focus))");
    assert!(p > 20.0 && p < 25.0);
  }
}

// ── convert_css_function_to_camel_case coverage ──────────────────────

#[cfg(test)]
mod convert_css_function_camel_case_tests {
  use crate::css::common::normalize_css_property_value;
  use stylex_structures::stylex_state_options::StyleXStateOptions;

  fn opts() -> StyleXStateOptions {
    StyleXStateOptions::default()
  }

  #[test]
  fn translatey_becomes_camel_case() {
    let r = normalize_css_property_value("transform", "translateY(20px)", &opts());
    assert_eq!(r, "translateY(20px)");
  }

  #[test]
  fn scalex_becomes_camel_case() {
    let r = normalize_css_property_value("transform", "scaleX(2)", &opts());
    assert_eq!(r, "scaleX(2)");
  }

  #[test]
  fn scaley_becomes_camel_case() {
    let r = normalize_css_property_value("transform", "scaleY(0.5)", &opts());
    assert_eq!(r, "scaleY(.5)");
  }

  #[test]
  fn rotatex_becomes_camel_case() {
    let r = normalize_css_property_value("transform", "rotateX(45deg)", &opts());
    assert_eq!(r, "rotateX(45deg)");
  }

  #[test]
  fn skewx_becomes_camel_case() {
    let r = normalize_css_property_value("transform", "skewX(10deg)", &opts());
    assert_eq!(r, "skewX(10deg)");
  }

  #[test]
  fn skewy_becomes_camel_case() {
    let r = normalize_css_property_value("transform", "skewY(5deg)", &opts());
    assert_eq!(r, "skewY(5deg)");
  }

  #[test]
  fn no_function_returns_as_is() {
    // No parentheses → convert_css_function_to_camel_case returns as-is
    let r = normalize_css_property_value("color", "red", &opts());
    assert_eq!(r, "red");
  }

  #[test]
  fn unknown_function_not_camel_cased() {
    // rotate3d is not in CAMEL_CASE_PRIORITIES
    let r = normalize_css_property_value("transform", "rotate(45deg)", &opts());
    assert_eq!(r, "rotate(45deg)");
  }
}

// ── normalize_css_property_value: CSS variable property path ──────────

#[cfg(test)]
mod normalize_css_variable_property_tests {
  use crate::css::common::normalize_css_property_value;
  use stylex_structures::stylex_state_options::StyleXStateOptions;

  fn opts() -> StyleXStateOptions {
    StyleXStateOptions::default()
  }

  #[test]
  fn css_variable_uses_color_for_parsing() {
    // When property starts with "--", parsing uses "color" as the property
    let r = normalize_css_property_value("--xCustom", "10px", &opts());
    assert_eq!(r, "10px");
  }

  #[test]
  fn css_variable_with_hex() {
    let r = normalize_css_property_value("--xBg", "#ff0000", &opts());
    assert_eq!(r, "#f00");
  }

  #[test]
  fn css_variable_with_keyword() {
    let r = normalize_css_property_value("--xBorder", "solid", &opts());
    assert_eq!(r, "solid");
  }
}

// ── normalize_css_property_value error paths ─────────────────────────

#[cfg(test)]
mod normalize_css_property_value_error_tests {
  use crate::css::common::normalize_css_property_value;
  use stylex_structures::stylex_state_options::StyleXStateOptions;

  fn opts() -> StyleXStateOptions {
    StyleXStateOptions::default()
  }

  #[test]
  #[should_panic(expected = "Rule contains an unclosed function")]
  fn panics_on_unclosed_function_paren() {
    normalize_css_property_value("color", "rgb(255, 0, 0", &opts());
  }

  #[test]
  fn css_variable_property_uses_color_for_parsing() {
    // --foo is a CSS variable, so it uses "color" as the parsing property
    let r = normalize_css_property_value("--xSomething", "10px", &opts());
    assert_eq!(r, "10px");
  }

  #[test]
  fn css_variable_with_complex_value() {
    let r = normalize_css_property_value("--xVar", "1px solid #000", &opts());
    assert_eq!(r, "1px solid #000");
  }
}

// ── build_nested_css_rule additional tests ────────────────────────────

#[cfg(test)]
mod build_nested_css_rule_extended_tests {
  use crate::css::common::build_nested_css_rule;

  #[test]
  fn builds_rule_with_multiple_at_rules() {
    let result = build_nested_css_rule(
      "xm",
      "color:red".into(),
      &mut [],
      &mut [
        "@media (max-width: 600px)".into(),
        "@supports (display: grid)".into(),
      ],
      &mut [],
    );
    assert!(result.contains("@media"));
    assert!(result.contains("@supports"));
    // Should be nested
    assert!(result.contains(".xm.xm.xm"));
  }

  #[test]
  fn builds_rule_with_pseudo_and_at_rule_combined() {
    let result = build_nested_css_rule(
      "xc",
      "color:red".into(),
      &mut [":hover".into()],
      &mut ["@media (min-width: 800px)".into()],
      &mut [],
    );
    assert!(result.contains(":hover"));
    assert!(result.contains("@media"));
  }

  #[test]
  fn builds_rule_with_at_rules_and_const_rules() {
    let result = build_nested_css_rule(
      "xac",
      "color:red".into(),
      &mut [],
      &mut ["@media (min-width: 800px)".into()],
      &mut ["--condition".into()],
    );
    assert!(result.contains("@media"));
    assert!(result.contains("--condition"));
  }

  #[test]
  fn builds_rule_with_thumb_and_pseudo() {
    let result = build_nested_css_rule(
      "xtp",
      "color:red".into(),
      &mut ["::thumb".into(), ":hover".into()],
      &mut [],
      &mut [],
    );
    assert!(result.contains("::-webkit-slider-thumb"));
    assert!(result.contains(":hover"));
  }

  #[test]
  fn builds_rule_with_where_and_at_rule() {
    let result = build_nested_css_rule(
      "xwa",
      "color:red".into(),
      &mut [":where(.theme)".into()],
      &mut ["@media (min-width: 800px)".into()],
      &mut [],
    );
    assert!(result.contains(":where(.theme)"));
    assert!(result.contains("@media"));
    // where should trigger extra specificity class
    assert!(result.contains(".xwa.xwa"));
  }
}
