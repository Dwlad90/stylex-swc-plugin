use stylex_css_parser_dep::base_types::SubString;
use stylex_css_parser_dep::css_types::position::{
  Horizontal, Position, Vertical,
};

/**
 * Test CSS Type: <position>
 */

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parses_single_keywords() {
    let mut input = SubString::new("left");
    let left_position = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      left_position,
      Position::new(Some(Horizontal::Keyword("left".to_string())), None)
    );

    let mut input = SubString::new("right");
    let right_position = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      right_position,
      Position::new(Some(Horizontal::Keyword("right".to_string())), None)
    );

    let mut input = SubString::new("center");
    let center_position = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      center_position,
      Position::new(Some(Horizontal::Keyword("center".to_string())), None)
    );

    let mut input = SubString::new("top");
    let top_position = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      top_position,
      Position::new(None, Some(Vertical::Keyword("top".to_string())))
    );

    let mut input = SubString::new("bottom");
    let bottom_position = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      bottom_position,
      Position::new(None, Some(Vertical::Keyword("bottom".to_string())))
    );

    let mut input = SubString::new("center");
    let center_position = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      center_position,
      Position::new(Some(Horizontal::Keyword("center".to_string())), None)
    );
  }

  #[test]
  fn test_parses_keyword_combinations() {
    let mut input = SubString::new("left top");
    let left_top = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      left_top,
      Position::new(
        Some(Horizontal::Keyword("left".to_string())),
        Some(Vertical::Keyword("top".to_string()))
      )
    );

    // let mut input = SubString::new("right bottom");
    // let right_bottom = Position::parse().run(&mut input).unwrap();
    // assert_eq!(
    //   right_bottom,
    //   Position::new(
    //     Some(Horizontal::Keyword("right".to_string())),
    //     Some(Vertical::Keyword("bottom".to_string()))
    //   )
    // );

    // let mut input = SubString::new("center center");
    // let center_center = Position::parse().run(&mut input).unwrap();
    // assert_eq!(
    //   center_center,
    //   Position::new(
    //     Some(Horizontal::Keyword("center".to_string())),
    //     Some(Vertical::Keyword("center".to_string()))
    //   )
    // );

    // let mut input = SubString::new("left bottom");
    // let left_bottom = Position::parse().run(&mut input).unwrap();
    // assert_eq!(
    //   left_bottom,
    //   Position::new(
    //     Some(Horizontal::Keyword("left".to_string())),
    //     Some(Vertical::Keyword("bottom".to_string()))
    //   )
    // );

    // let mut input = SubString::new("right top");
    // let right_top = Position::parse().run(&mut input).unwrap();
    // assert_eq!(
    //   right_top,
    //   Position::new(
    //     Some(Horizontal::Keyword("right".to_string())),
    //     Some(Vertical::Keyword("top".to_string()))
    //   )
    // );
  }

  #[test]
  fn test_parses_keyword_with_length_percentage() {
    let mut input = SubString::new("left 50% top 20px");
    let left_top_with_offsets = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      left_top_with_offsets,
      Position::new(
        Some(Horizontal::KeywordWithLength(
          "left".to_string(),
          "50%".to_string()
        )),
        Some(Vertical::KeywordWithLength(
          "top".to_string(),
          "20px".to_string()
        ))
      )
    );

    let mut input = SubString::new("right 20px");
    let right_with_length = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      right_with_length,
      Position::new(
        Some(Horizontal::KeywordWithLength(
          "right".to_string(),
          "20px".to_string()
        )),
        None
      )
    );

    let mut input = SubString::new("50% top");
    let percent_top = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      percent_top,
      Position::new(
        Some(Horizontal::LengthPercentage("50%".to_string())),
        Some(Vertical::Keyword("top".to_string()))
      )
    );

    let mut input = SubString::new("30px bottom");
    let px_bottom = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      px_bottom,
      Position::new(
        Some(Horizontal::LengthPercentage("30px".to_string())),
        Some(Vertical::Keyword("bottom".to_string()))
      )
    );
  }

  #[test]
  fn test_parses_length_percentage_combinations() {
    let mut input = SubString::new("50% 50%");
    let percent_percent = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      percent_percent,
      Position::new(
        Some(Horizontal::LengthPercentage("50%".to_string())),
        Some(Vertical::LengthPercentage("50%".to_string()))
      )
    );

    let mut input = SubString::new("20px 30px");
    let px_px = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      px_px,
      Position::new(
        Some(Horizontal::LengthPercentage("20px".to_string())),
        Some(Vertical::LengthPercentage("30px".to_string()))
      )
    );

    let mut input = SubString::new("25% 40px");
    let percent_px = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      percent_px,
      Position::new(
        Some(Horizontal::LengthPercentage("25%".to_string())),
        Some(Vertical::LengthPercentage("40px".to_string()))
      )
    );
  }

  #[test]
  fn test_parses_keyword_with_offset() {
    let mut input = SubString::new("left 20% top 30%");
    let left_top_with_percentages = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      left_top_with_percentages,
      Position::new(
        Some(Horizontal::KeywordWithLength(
          "left".to_string(),
          "20%".to_string()
        )),
        Some(Vertical::KeywordWithLength(
          "top".to_string(),
          "30%".to_string()
        ))
      )
    );

    let mut input = SubString::new("right 10px bottom 15px");
    let right_bottom_with_lengths = Position::parse().run(&mut input).unwrap();
    assert_eq!(
      right_bottom_with_lengths,
      Position::new(
        Some(Horizontal::KeywordWithLength(
          "right".to_string(),
          "10px".to_string()
        )),
        Some(Vertical::KeywordWithLength(
          "bottom".to_string(),
          "15px".to_string()
        ))
      )
    );
  }

  #[test]
  fn test_rejects_invalid_positions() {
    let mut input = SubString::new("invalid");
    let invalid_position = Position::parse().run(&mut input);
    assert!(invalid_position.is_err());

    let mut input = SubString::new("left left");
    let left_left = Position::parse().run(&mut input);
    assert!(left_left.is_err());

    let mut input = SubString::new("top right bottom");
    let top_right_bottom = Position::parse().run(&mut input);
    assert!(top_right_bottom.is_err());

    let mut input = SubString::new("20");
    let just_number = Position::parse().run(&mut input);
    assert!(just_number.is_err());
  }
}
