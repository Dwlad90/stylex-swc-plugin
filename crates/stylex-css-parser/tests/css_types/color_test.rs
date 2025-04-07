use stylex_css_parser::css_types::color::{Color, HashColor, NamedColor, Rgb, Rgba};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_named_colors() {
    let red = "red";
    let blue = "blue";
    let green = "green";
    let transparent = "transparent";

    assert_eq!(
      Color::parse().parse(red),
      Ok(Color::Named(NamedColor::new("red".to_string())))
    );
    assert_eq!(
      Color::parse().parse(blue),
      Ok(Color::Named(NamedColor::new("blue".to_string())))
    );
    assert_eq!(
      Color::parse().parse(green),
      Ok(Color::Named(NamedColor::new("green".to_string())))
    );
    assert_eq!(
      Color::parse().parse(transparent),
      Ok(Color::Named(NamedColor::new("transparent".to_string())))
    );
  }

  #[test]
  fn test_hash_colors() {
    let red_hex = "#ff0000";
    let green_hex = "#00ff00";
    let blue_hex = "#0000ff";
    let white_hex = "#ffffff";

    assert_eq!(
      Color::parse().parse(red_hex),
      Ok(Color::Hash(HashColor::new("ff0000".to_string())))
    );
    assert_eq!(
      Color::parse().parse(green_hex),
      Ok(Color::Hash(HashColor::new("00ff00".to_string())))
    );
    assert_eq!(
      Color::parse().parse(blue_hex),
      Ok(Color::Hash(HashColor::new("0000ff".to_string())))
    );
    assert_eq!(
      Color::parse().parse(white_hex),
      Ok(Color::Hash(HashColor::new("ffffff".to_string())))
    );
  }

  #[test]
  fn test_rgb_values() {
    let red_rgb = "rgb(255, 0, 0)";
    let green_rgb = "rgb(0, 255, 0)";
    let blue_rgb = "rgb(0, 0, 255)";

    assert_eq!(
      Color::parse().parse(red_rgb),
      Ok(Color::Rgb(Rgb::new(255.0, 0.0, 0.0)))
    );
    assert_eq!(
      Color::parse().parse(green_rgb),
      Ok(Color::Rgb(Rgb::new(0.0, 255.0, 0.0)))
    );
    assert_eq!(
      Color::parse().parse(blue_rgb),
      Ok(Color::Rgb(Rgb::new(0.0, 0.0, 255.0)))
    );
  }

  #[test]
  fn test_space_separated_rgb_values() {
    let red_rgb = "rgb(255 0 0)";
    let green_rgb = "rgb(0 255 0)";
    let blue_rgb = "rgb(0 0 255)";

    assert_eq!(
      Color::parse().parse(red_rgb),
      Ok(Color::Rgb(Rgb::new(255.0, 0.0, 0.0)))
    );
    assert_eq!(
      Color::parse().parse(green_rgb),
      Ok(Color::Rgb(Rgb::new(0.0, 255.0, 0.0)))
    );
    assert_eq!(
      Color::parse().parse(blue_rgb),
      Ok(Color::Rgb(Rgb::new(0.0, 0.0, 255.0)))
    );
  }

  #[test]
  fn test_rgba_values() {
    let red_rgba = "rgba(255, 0, 0, 0.5)";
    let green_rgba = "rgba(0, 255, 0, 0.5)";
    let blue_rgba = "rgba(0, 0, 255, 0.5)";

    assert_eq!(
      Color::parse().parse(red_rgba),
      Ok(Color::Rgba(Rgba::new(255.0, 0.0, 0.0, 0.5)))
    );
    assert_eq!(
      Color::parse().parse(green_rgba),
      Ok(Color::Rgba(Rgba::new(0.0, 255.0, 0.0, 0.5)))
    );
    assert_eq!(
      Color::parse().parse(blue_rgba),
      Ok(Color::Rgba(Rgba::new(0.0, 0.0, 255.0, 0.5)))
    );
  }

  #[test]
  fn test_space_separated_rgba_values() {
    let red_rgba = "rgb(255 0 0 / 0.5)";
    let green_rgba = "rgb(0 255 0 / 0.5)";
    let blue_rgba = "rgb(0 0 255 / 0.5)";
    let red_rgba_percent = "rgb(255 0 0 / 50%)";
    let green_rgba_percent = "rgb(0 255 0 / 50%)";
    let blue_rgba_percent = "rgb(0 0 255 / 50%)";

    assert_eq!(
      Color::parse().parse(red_rgba),
      Ok(Color::Rgba(Rgba::new(255.0, 0.0, 0.0, 0.5)))
    );
    assert_eq!(
      Color::parse().parse(green_rgba),
      Ok(Color::Rgba(Rgba::new(0.0, 255.0, 0.0, 0.5)))
    );
    assert_eq!(
      Color::parse().parse(blue_rgba),
      Ok(Color::Rgba(Rgba::new(0.0, 0.0, 255.0, 0.5)))
    );

    assert_eq!(
      Color::parse().parse(red_rgba_percent),
      Ok(Color::Rgba(Rgba::new(255.0, 0.0, 0.0, 0.5)))
    );
    assert_eq!(
      Color::parse().parse(green_rgba_percent),
      Ok(Color::Rgba(Rgba::new(0.0, 255.0, 0.0, 0.5)))
    );
    assert_eq!(
      Color::parse().parse(blue_rgba_percent),
      Ok(Color::Rgba(Rgba::new(0.0, 0.0, 255.0, 0.5)))
    );
  }

  // #[test]
  // fn test_hsl_values() {
  //   let hsl_color = "hsl(270, 60%, 70%)";
  //   let expected = Color::Hsl(Hsl::new(
  //     Angle {
  //       value: 270.0,
  //       unit: "deg".to_string(),
  //     },
  //     Percentage { value: 60.0 },
  //     Percentage { value: 70.0 },
  //   ));

  //   assert_eq!(Color::parse().parse(hsl_color), Ok(expected));
  // }

  // #[test]
  // fn test_hsla_values() {
  //   let hsla_color = "hsla(270, 60%, 70%, 0.5)";
  //   let expected = Color::Hsla(Hsla::new(
  //     Angle {
  //       value: 270.0,
  //       unit: "deg".to_string(),
  //     },
  //     Percentage { value: 60.0 },
  //     Percentage { value: 70.0 },
  //     0.5,
  //   ));

  //   assert_eq!(Color::parse().parse(hsla_color), Ok(expected));
  // }

  // #[test]
  // fn test_invalid_colors() {
  //   assert_eq!(Color::parse().parse_to_end("invalid"), None);
  //   assert_eq!(Color::parse().parse_to_end("#gggggg"), None);
  //   assert_eq!(Color::parse().parse_to_end("rgb(256, 0, 0)"), None);
  // }
}
