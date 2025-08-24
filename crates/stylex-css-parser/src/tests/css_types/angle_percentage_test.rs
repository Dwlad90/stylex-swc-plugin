/*!
CSS Angle Percentage Tests

Test CSS angle-percentage type that accepts both angles and percentages.
*/

#[cfg(test)]
mod test_css_type_angle_percentage {
  use crate::css_types::angle_percentage::{angle_percentage_parser, AnglePercentage};

  #[test]
  fn parses_angle_values() {
    let result = angle_percentage_parser().parse_to_end("45deg").unwrap();
    match result {
      AnglePercentage::Angle(angle) => {
        assert_eq!(angle.value, 45.0);
        assert_eq!(angle.unit, "deg");
      }
      _ => panic!("Expected angle"),
    }

    let result = angle_percentage_parser().parse_to_end("1rad").unwrap();
    match result {
      AnglePercentage::Angle(angle) => {
        assert_eq!(angle.value, 1.0);
        assert_eq!(angle.unit, "rad");
      }
      _ => panic!("Expected angle"),
    }

    let result = angle_percentage_parser().parse_to_end("0.5turn").unwrap();
    match result {
      AnglePercentage::Angle(angle) => {
        assert_eq!(angle.value, 0.5);
        assert_eq!(angle.unit, "turn");
      }
      _ => panic!("Expected angle"),
    }

    let result = angle_percentage_parser().parse_to_end("100grad").unwrap();
    match result {
      AnglePercentage::Angle(angle) => {
        assert_eq!(angle.value, 100.0);
        assert_eq!(angle.unit, "grad");
      }
      _ => panic!("Expected angle"),
    }
  }

  #[test]
  fn parses_percentage_values() {
    let result = angle_percentage_parser().parse_to_end("50%").unwrap();
    match result {
      AnglePercentage::Percentage(percentage) => {
        assert_eq!(percentage.value, 50.0);
      }
      _ => panic!("Expected percentage"),
    }

    let result = angle_percentage_parser().parse_to_end("100%").unwrap();
    match result {
      AnglePercentage::Percentage(percentage) => {
        assert_eq!(percentage.value, 100.0);
      }
      _ => panic!("Expected percentage"),
    }

    let result = angle_percentage_parser().parse_to_end("0%").unwrap();
    match result {
      AnglePercentage::Percentage(percentage) => {
        assert_eq!(percentage.value, 0.0);
      }
      _ => panic!("Expected percentage"),
    }

    let result = angle_percentage_parser().parse_to_end("25%").unwrap();
    match result {
      AnglePercentage::Percentage(percentage) => {
        assert_eq!(percentage.value, 25.0);
      }
      _ => panic!("Expected percentage"),
    }
  }

  #[test]
  fn rejects_invalid_angle_percentage_values() {
    assert!(angle_percentage_parser().parse_to_end("abc").is_err());
    assert!(angle_percentage_parser().parse_to_end("50").is_err());
    assert!(angle_percentage_parser().parse_to_end("10abc").is_err());
  }
}
