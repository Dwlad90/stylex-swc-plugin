/*!
Color CSS type tests.
*/

use crate::css_types::color::Color;

#[cfg(test)]
mod test_css_type_color {
  use super::*;

  #[test]
  fn parses_named_colors() {
    let color1 = Color::parse().parse_to_end("red").unwrap();
    match color1 {
      Color::Named(ref named) => {
        assert_eq!(named.value, "red");
      }
      _ => panic!("Expected NamedColor"),
    }
    assert_eq!(color1.to_string(), "red");

    let color2 = Color::parse().parse_to_end("blue").unwrap();
    match color2 {
      Color::Named(ref named) => {
        assert_eq!(named.value, "blue");
      }
      _ => panic!("Expected NamedColor"),
    }
    assert_eq!(color2.to_string(), "blue");

    let color3 = Color::parse().parse_to_end("green").unwrap();
    match color3 {
      Color::Named(ref named) => {
        assert_eq!(named.value, "green");
      }
      _ => panic!("Expected NamedColor"),
    }
    assert_eq!(color3.to_string(), "green");

    let transparent = Color::parse().parse_to_end("transparent").unwrap();
    match transparent {
      Color::Named(ref named) => {
        assert_eq!(named.value, "transparent");
      }
      _ => panic!("Expected NamedColor"),
    }
    assert_eq!(transparent.to_string(), "transparent");
  }

  #[test]
  fn parses_hash_colors() {
    let test_cases = vec![
      ("#ff0000", "ff0000"),
      ("#00ff00", "00ff00"),
      ("#0000ff", "0000ff"),
      ("#ffffff", "ffffff"),
      ("#000000", "000000"),
      ("#abc", "abc"),
      ("#def", "def"),
      ("#123456", "123456"),
    ];

    for (input, expected_hex) in test_cases {
      let color = Color::parse().parse_to_end(input).unwrap();
      match color {
        Color::Hash(ref hash) => {
          assert_eq!(hash.value, expected_hex);
        }
        _ => panic!("Expected HashColor for: {}", input),
      }
      assert_eq!(color.to_string(), input);
    }
  }

  #[test]
  fn parses_rgb_colors() {
    let color1 = Color::parse().parse_to_end("rgb(255, 0, 0)").unwrap();
    match color1 {
      Color::Rgb(ref rgb) => {
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 0);
        assert_eq!(rgb.b, 0);
      }
      _ => panic!("Expected Rgb"),
    }
    assert_eq!(color1.to_string(), "rgb(255, 0, 0)");

    let color2 = Color::parse().parse_to_end("rgb(0, 255, 0)").unwrap();
    match color2 {
      Color::Rgb(ref rgb) => {
        assert_eq!(rgb.r, 0);
        assert_eq!(rgb.g, 255);
        assert_eq!(rgb.b, 0);
      }
      _ => panic!("Expected Rgb"),
    }
    assert_eq!(color2.to_string(), "rgb(0, 255, 0)");

    let color3 = Color::parse().parse_to_end("rgb(128, 128, 128)").unwrap();
    match color3 {
      Color::Rgb(ref rgb) => {
        assert_eq!(rgb.r, 128);
        assert_eq!(rgb.g, 128);
        assert_eq!(rgb.b, 128);
      }
      _ => panic!("Expected Rgb"),
    }
    assert_eq!(color3.to_string(), "rgb(128, 128, 128)");
  }

  #[test]
  fn parses_rgba_colors() {
    let color1 = Color::parse().parse_to_end("rgba(255, 0, 0, 0.5)").unwrap();
    match color1 {
      Color::Rgba(ref rgba) => {
        assert_eq!(rgba.r, 255);
        assert_eq!(rgba.g, 0);
        assert_eq!(rgba.b, 0);
        assert_eq!(rgba.a, 0.5);
      }
      _ => panic!("Expected Rgba"),
    }
    assert_eq!(color1.to_string(), "rgba(255, 0, 0, 0.5)");

    let color2 = Color::parse()
      .parse_to_end("rgba(0, 128, 255, 1.0)")
      .unwrap();
    match color2 {
      Color::Rgba(ref rgba) => {
        assert_eq!(rgba.r, 0);
        assert_eq!(rgba.g, 128);
        assert_eq!(rgba.b, 255);
        assert_eq!(rgba.a, 1.0);
      }
      _ => panic!("Expected Rgba"),
    }
    assert_eq!(color2.to_string(), "rgba(0, 128, 255, 1)");

    let color3 = Color::parse()
      .parse_to_end("rgba(255, 255, 255, 0)")
      .unwrap();
    match color3 {
      Color::Rgba(ref rgba) => {
        assert_eq!(rgba.r, 255);
        assert_eq!(rgba.g, 255);
        assert_eq!(rgba.b, 255);
        assert_eq!(rgba.a, 0.0);
      }
      _ => panic!("Expected Rgba"),
    }
    assert_eq!(color3.to_string(), "rgba(255, 255, 255, 0)");
  }

  #[test]
  fn parses_space_separated_rgb_values() {
    let color1 = Color::parse().parse_to_end("rgb(255 0 0)").unwrap();
    match color1 {
      Color::Rgb(ref rgb) => {
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 0);
        assert_eq!(rgb.b, 0);
      }
      _ => panic!("Expected Rgb"),
    }

    let color2 = Color::parse().parse_to_end("rgb(0 255 0)").unwrap();
    match color2 {
      Color::Rgb(ref rgb) => {
        assert_eq!(rgb.r, 0);
        assert_eq!(rgb.g, 255);
        assert_eq!(rgb.b, 0);
      }
      _ => panic!("Expected Rgb"),
    }

    let color3 = Color::parse().parse_to_end("rgb(0 0 255)").unwrap();
    match color3 {
      Color::Rgb(ref rgb) => {
        assert_eq!(rgb.r, 0);
        assert_eq!(rgb.g, 0);
        assert_eq!(rgb.b, 255);
      }
      _ => panic!("Expected Rgb"),
    }
  }

  #[test]
  fn parses_space_separated_rgba_values() {
    let color1 = Color::parse().parse_to_end("rgba(255 0 0 / 0.5)").unwrap();
    match color1 {
      Color::Rgba(ref rgba) => {
        assert_eq!(rgba.r, 255);
        assert_eq!(rgba.g, 0);
        assert_eq!(rgba.b, 0);
        assert_eq!(rgba.a, 0.5);
      }
      _ => panic!("Expected Rgba"),
    }

    let color2 = Color::parse().parse_to_end("rgba(0 255 0 / 0.8)").unwrap();
    match color2 {
      Color::Rgba(ref rgba) => {
        assert_eq!(rgba.r, 0);
        assert_eq!(rgba.g, 255);
        assert_eq!(rgba.b, 0);
        assert_eq!(rgba.a, 0.8);
      }
      _ => panic!("Expected Rgba"),
    }

    let color3 = Color::parse().parse_to_end("rgba(0 0 255 / 50%)").unwrap();
    match color3 {
      Color::Rgba(ref rgba) => {
        assert_eq!(rgba.r, 0);
        assert_eq!(rgba.g, 0);
        assert_eq!(rgba.b, 255);
        assert_eq!(rgba.a, 0.5);
      }
      _ => panic!("Expected Rgba"),
    }
  }

  #[test]
  fn parses_hsl_colors() {
    let color1 = Color::parse().parse_to_end("hsl(120, 100%, 50%)").unwrap();
    match color1 {
      Color::Hsl(ref hsl) => {
        assert_eq!(hsl.h.value, 120.0);
        assert_eq!(hsl.s.value, 100.0);
        assert_eq!(hsl.l.value, 50.0);
      }
      _ => panic!("Expected Hsl"),
    }
    assert_eq!(color1.to_string(), "hsl(120deg, 100%, 50%)");

    let color2 = Color::parse().parse_to_end("hsl(0, 100%, 50%)").unwrap();
    match color2 {
      Color::Hsl(ref hsl) => {
        assert_eq!(hsl.h.value, 0.0);
        assert_eq!(hsl.s.value, 100.0);
        assert_eq!(hsl.l.value, 50.0);
      }
      _ => panic!("Expected Hsl"),
    }
    assert_eq!(color2.to_string(), "hsl(0deg, 100%, 50%)");

    let color3 = Color::parse().parse_to_end("hsl(240, 100%, 50%)").unwrap();
    match color3 {
      Color::Hsl(ref hsl) => {
        assert_eq!(hsl.h.value, 240.0);
        assert_eq!(hsl.s.value, 100.0);
        assert_eq!(hsl.l.value, 50.0);
      }
      _ => panic!("Expected Hsl"),
    }
    assert_eq!(color3.to_string(), "hsl(240deg, 100%, 50%)");
  }

  #[test]
  fn parses_hsla_colors() {
    let color1 = Color::parse()
      .parse_to_end("hsla(240, 100%, 50%, 0.8)")
      .unwrap();
    match color1 {
      Color::Hsla(ref hsla) => {
        assert_eq!(hsla.h.value, 240.0);
        assert_eq!(hsla.s.value, 100.0);
        assert_eq!(hsla.l.value, 50.0);
        assert_eq!(hsla.a, 0.8);
      }
      _ => panic!("Expected Hsla"),
    }
    assert_eq!(color1.to_string(), "hsla(240deg, 100%, 50%, 0.8)");

    let color2 = Color::parse()
      .parse_to_end("hsla(120, 50%, 75%, 0.3)")
      .unwrap();
    match color2 {
      Color::Hsla(ref hsla) => {
        assert_eq!(hsla.h.value, 120.0);
        assert_eq!(hsla.s.value, 50.0);
        assert_eq!(hsla.l.value, 75.0);
        assert_eq!(hsla.a, 0.3);
      }
      _ => panic!("Expected Hsla"),
    }
    assert_eq!(color2.to_string(), "hsla(120deg, 50%, 75%, 0.3)");
  }

  #[test]
  fn parses_lch_colors() {
    let color = Color::parse().parse_to_end("lch(50% 30 180)").unwrap();
    match color {
      Color::Lch(ref lch) => {
        assert_eq!(lch.l, 50.0);
        assert_eq!(lch.c, 30.0);
        match &lch.h {
          crate::css_types::color::LchHue::Number(h) => assert_eq!(*h, 180.0),
          _ => panic!("Expected number hue for lch(50% 30 180)"),
        }
        assert_eq!(lch.alpha, None);
      }
      _ => panic!("Expected Lch"),
    }
    assert_eq!(color.to_string(), "lch(50 30 180)");
  }

  #[test]
  fn parses_oklch_colors() {
    let color = Color::parse().parse_to_end("oklch(0.7 0.15 180)").unwrap();
    match color {
      Color::Oklch(ref oklch) => {
        assert_eq!(oklch.l, 0.7);
        assert_eq!(oklch.c, 0.15);
        // Number 180 is interpreted as 180deg directly (CSS spec compliant)
        assert_eq!(oklch.h.value, 180.0);
        assert_eq!(oklch.h.unit, "deg");
        assert_eq!(oklch.alpha, None);
      }
      _ => panic!("Expected Oklch"),
    }
    assert_eq!(color.to_string(), "oklch(0.7 0.15 180deg)");
  }

  #[test]
  fn parses_oklab_colors() {
    let color = Color::parse().parse_to_end("oklab(0.7 -0.15 0.1)").unwrap();
    match color {
      Color::Oklab(ref oklab) => {
        assert_eq!(oklab.l, 0.7);
        assert_eq!(oklab.a, -0.15);
        assert_eq!(oklab.b, 0.1);
        assert_eq!(oklab.alpha, None);
      }
      _ => panic!("Expected Oklab"),
    }
    assert_eq!(color.to_string(), "oklab(0.7 -0.15 0.1)");
  }

  #[test]
  fn parses_modern_color_spaces_with_alpha() {
    // LCH with alpha
    let lch_alpha = Color::parse()
      .parse_to_end("lch(50% 30 180deg / 0.8)")
      .unwrap();
    match lch_alpha {
      Color::Lch(ref lch) => {
        assert_eq!(lch.l, 50.0);
        assert_eq!(lch.c, 30.0);
        match &lch.h {
          crate::css_types::color::LchHue::Angle(angle) => {
            assert_eq!(angle.value, 180.0);
            assert_eq!(angle.unit, "deg");
          }
          _ => panic!("Expected angle hue"),
        }
        assert_eq!(lch.alpha, Some(0.8));
      }
      _ => panic!("Expected Lch"),
    }

    // OKLCH with alpha
    let oklch_alpha = Color::parse()
      .parse_to_end("oklch(0.7 0.15 180deg / 0.5)")
      .unwrap();
    match oklch_alpha {
      Color::Oklch(ref oklch) => {
        assert_eq!(oklch.l, 0.7);
        assert_eq!(oklch.c, 0.15);
        assert_eq!(oklch.h.value, 180.0);
        assert_eq!(oklch.alpha, Some(0.5));
      }
      _ => panic!("Expected Oklch"),
    }

    // OKLAB with alpha
    let oklab_alpha = Color::parse()
      .parse_to_end("oklab(0.7 -0.15 0.1 / 0.9)")
      .unwrap();
    match oklab_alpha {
      Color::Oklab(ref oklab) => {
        assert_eq!(oklab.l, 0.7);
        assert_eq!(oklab.a, -0.15);
        assert_eq!(oklab.b, 0.1);
        assert_eq!(oklab.alpha, Some(0.9));
      }
      _ => panic!("Expected Oklab"),
    }
  }

  #[test]
  fn parses_modern_color_spaces_with_none_values() {
    // OKLCH with 'none' values
    let oklch_none = Color::parse()
      .parse_to_end("oklch(none 0.15 none)")
      .unwrap();
    match oklch_none {
      Color::Oklch(ref oklch) => {
        assert_eq!(oklch.l, 0.0); // 'none' maps to 0
        assert_eq!(oklch.c, 0.15);
        assert_eq!(oklch.h.value, 0.0); // 'none' * 360 = 0
      }
      _ => panic!("Expected Oklch"),
    }

    // OKLAB with 'none' values
    let oklab_none = Color::parse().parse_to_end("oklab(0.5 none none)").unwrap();
    match oklab_none {
      Color::Oklab(ref oklab) => {
        assert_eq!(oklab.l, 0.5);
        assert_eq!(oklab.a, 0.0); // 'none' maps to 0
        assert_eq!(oklab.b, 0.0); // 'none' maps to 0
      }
      _ => panic!("Expected Oklab"),
    }
  }

  #[test]
  fn comprehensive_color_parsing() {
    let test_cases = vec![
      // Named colors
      "red",
      "blue",
      "green",
      "black",
      "white",
      "transparent",
      "currentColor",
      // Hex colors
      "#ff0000",
      "#00ff00",
      "#0000ff",
      "#abc",
      "#def",
      "#123456",
      "#abcdef",
      // RGB colors
      "rgb(255, 0, 0)",
      "rgb(0, 255, 0)",
      "rgb(0, 0, 255)",
      "rgb(128, 128, 128)",
      // RGBA colors
      "rgba(255, 0, 0, 0.5)",
      "rgba(0, 255, 0, 1.0)",
      "rgba(0, 0, 255, 0.25)",
      // HSL colors
      "hsl(0, 100%, 50%)",
      "hsl(120, 100%, 50%)",
      "hsl(240, 100%, 50%)",
      // HSLA colors
      "hsla(0, 100%, 50%, 0.5)",
      "hsla(120, 100%, 50%, 1.0)",
      // Modern color spaces
      "lch(50% 30 180)",
      "oklch(0.7 0.15 180)",
      "oklab(0.7 -0.15 0.1)",
    ];

    for input in test_cases {
      let color = Color::parse().parse_to_end(input).unwrap();

      // Verify it produces valid output
      assert!(
        !color.to_string().is_empty(),
        "Should produce output for: {}",
        input
      );

      // Verify round-trip parsing for most cases
      // (some cases may have normalized output like hsl(120deg, 100%, 50%))
      let reparsed = Color::parse().parse_to_end(&color.to_string());
      assert!(
        reparsed.is_ok(),
        "Should reparse successfully: {} -> {}",
        input,
        color
      );
    }
  }

  #[test]
  fn case_insensitive_parsing() {
    let test_cases = vec![
      ("RED", "red"),
      ("Blue", "blue"),
      ("TRANSPARENT", "transparent"),
      ("RGB(255, 0, 0)", "rgb(255, 0, 0)"),
      ("HSL(120, 100%, 50%)", "hsl(120deg, 100%, 50%)"),
    ];

    for (input, expected_normalized) in test_cases {
      let color = Color::parse().parse_to_end(input).unwrap();
      assert_eq!(
        color.to_string().to_lowercase(),
        expected_normalized.to_lowercase()
      );
    }
  }

  #[test]
  fn whitespace_handling() {
    let test_cases = vec![
      "rgb(255,0,0)",            // No spaces
      "rgb( 255 , 0 , 0 )",      // Extra spaces
      "hsl( 120 , 100% , 50% )", // HSL with spaces
      "rgba(255, 0, 0, 0.5)",    // RGBA normal
    ];

    for input in test_cases {
      let color = Color::parse().parse_to_end(input).unwrap();
      assert!(!color.to_string().is_empty(), "Should parse: {}", input);
    }
  }

  #[test]
  fn edge_cases() {
    // Test edge values
    let edge_cases = vec![
      "rgb(0, 0, 0)",           // Black
      "rgb(255, 255, 255)",     // White
      "rgba(0, 0, 0, 0)",       // Transparent black
      "rgba(255, 255, 255, 1)", // Opaque white
      "hsl(0, 0%, 0%)",         // Black in HSL
      "hsl(0, 0%, 100%)",       // White in HSL
      "hsla(0, 0%, 0%, 0)",     // Transparent black in HSLA
    ];

    for input in edge_cases {
      let color = Color::parse().parse_to_end(input).unwrap();
      assert!(
        !color.to_string().is_empty(),
        "Should parse edge case: {}",
        input
      );
    }
  }

  #[test]
  fn rejects_invalid_colors() {
    assert!(Color::parse().parse_to_end("invalid").is_err());
    assert!(Color::parse().parse_to_end("#gggggg").is_err());
    assert!(Color::parse().parse_to_end("rgb(256, 0, 0)").is_err());
  }

  #[test]
  fn oklch_hue_number_interpretation() {
    // Test that numbers in OKLCH hue are interpreted as degrees directly
    let test_cases = vec![
      ("oklch(0.5 0.1 0)", 0.0),
      ("oklch(0.5 0.1 90)", 90.0),
      ("oklch(0.5 0.1 180)", 180.0),
      ("oklch(0.5 0.1 270)", 270.0),
      ("oklch(0.5 0.1 360)", 360.0),
      ("oklch(0.5 0.1 0.5)", 0.5), // Fractional degrees
    ];

    for (input, expected_degrees) in test_cases {
      let color = Color::parse().parse_to_end(input).unwrap();
      match color {
        Color::Oklch(ref oklch) => {
          assert_eq!(
            oklch.h.value, expected_degrees,
            "Failed for input: {}, expected {} degrees, got {} degrees",
            input, expected_degrees, oklch.h.value
          );
          assert_eq!(oklch.h.unit, "deg");
        }
        _ => panic!("Expected Oklch for input: {}", input),
      }
    }
  }
}
