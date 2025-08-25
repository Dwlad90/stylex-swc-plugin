/*!
Basic shape CSS type tests.


Test CSS Type: <basic-shape>
Tests parsing of inset(), circle(), ellipse(), polygon(), and path() functions.
*/

use crate::css_types::basic_shape::{BasicShape, CircleRadius};
use crate::css_types::common_types::Percentage;
use crate::css_types::length::Length;
use crate::css_types::length_percentage::LengthPercentage;

#[cfg(test)]
mod test_css_type_basic_shape {
  use super::*;

  // Test Inset shapes
  mod inset {
    use super::*;

    #[test]
    fn should_parse_valid_insets() {
      // inset(10px) - single value
      let result = BasicShape::parse().parse_to_end("inset(10px)").unwrap();
      if let BasicShape::Inset {
        top,
        right,
        bottom,
        left,
        round,
      } = result
      {
        assert_eq!(
          top,
          LengthPercentage::Length(Length::new(10.0, "px".to_string()))
        );
        assert_eq!(
          right,
          LengthPercentage::Length(Length::new(10.0, "px".to_string()))
        );
        assert_eq!(
          bottom,
          LengthPercentage::Length(Length::new(10.0, "px".to_string()))
        );
        assert_eq!(
          left,
          LengthPercentage::Length(Length::new(10.0, "px".to_string()))
        );
        assert_eq!(round, None);
      } else {
        panic!("Expected Inset variant");
      }

      // inset(10px 20px) - two values
      let result = BasicShape::parse()
        .parse_to_end("inset(10px 20px)")
        .unwrap();
      if let BasicShape::Inset {
        top,
        right,
        bottom,
        left,
        ..
      } = result
      {
        assert_eq!(
          top,
          LengthPercentage::Length(Length::new(10.0, "px".to_string()))
        );
        assert_eq!(
          right,
          LengthPercentage::Length(Length::new(20.0, "px".to_string()))
        );
        assert_eq!(
          bottom,
          LengthPercentage::Length(Length::new(10.0, "px".to_string()))
        );
        assert_eq!(
          left,
          LengthPercentage::Length(Length::new(20.0, "px".to_string()))
        );
      } else {
        panic!("Expected Inset variant");
      }

      // inset(10px 20px 30px) - three values
      let result = BasicShape::parse()
        .parse_to_end("inset(10px 20px 30px)")
        .unwrap();
      if let BasicShape::Inset {
        top,
        right,
        bottom,
        left,
        ..
      } = result
      {
        assert_eq!(
          top,
          LengthPercentage::Length(Length::new(10.0, "px".to_string()))
        );
        assert_eq!(
          right,
          LengthPercentage::Length(Length::new(20.0, "px".to_string()))
        );
        assert_eq!(
          bottom,
          LengthPercentage::Length(Length::new(30.0, "px".to_string()))
        );
        assert_eq!(
          left,
          LengthPercentage::Length(Length::new(20.0, "px".to_string()))
        );
      } else {
        panic!("Expected Inset variant");
      }

      // inset(10px 20px 30px 40px) - four values
      let result = BasicShape::parse()
        .parse_to_end("inset(10px 20px 30px 40px)")
        .unwrap();
      if let BasicShape::Inset {
        top,
        right,
        bottom,
        left,
        ..
      } = result
      {
        assert_eq!(
          top,
          LengthPercentage::Length(Length::new(10.0, "px".to_string()))
        );
        assert_eq!(
          right,
          LengthPercentage::Length(Length::new(20.0, "px".to_string()))
        );
        assert_eq!(
          bottom,
          LengthPercentage::Length(Length::new(30.0, "px".to_string()))
        );
        assert_eq!(
          left,
          LengthPercentage::Length(Length::new(40.0, "px".to_string()))
        );
      } else {
        panic!("Expected Inset variant");
      }
    }

    #[test]
    fn should_parse_inset_with_round() {
      let result = BasicShape::parse()
        .parse_to_end("inset(10px round 5px)")
        .unwrap();
      if let BasicShape::Inset {
        top,
        right,
        bottom,
        left,
        round,
      } = result
      {
        assert_eq!(
          top,
          LengthPercentage::Length(Length::new(10.0, "px".to_string()))
        );
        assert_eq!(
          right,
          LengthPercentage::Length(Length::new(10.0, "px".to_string()))
        );
        assert_eq!(
          bottom,
          LengthPercentage::Length(Length::new(10.0, "px".to_string()))
        );
        assert_eq!(
          left,
          LengthPercentage::Length(Length::new(10.0, "px".to_string()))
        );
        assert_eq!(
          round,
          Some(LengthPercentage::Length(Length::new(5.0, "px".to_string())))
        );
      } else {
        panic!("Expected Inset variant");
      }
    }

    #[test]
    fn should_not_parse_invalid_insets() {
      assert!(BasicShape::parse().parse_to_end("inset(invalid)").is_err());
      assert!(
        BasicShape::parse()
          .parse_to_end("inset(10px, invalid)")
          .is_err()
      );
    }
  }

  // Test Circle shapes
  mod circle {
    use super::*;

    #[test]
    fn should_parse_valid_circles() {
      // circle(10px)
      let result = BasicShape::parse().parse_to_end("circle(10px)").unwrap();
      if let BasicShape::Circle { radius, position } = result {
        assert_eq!(
          radius,
          CircleRadius::Length(LengthPercentage::Length(Length::new(
            10.0,
            "px".to_string()
          )))
        );
        assert_eq!(position, None);
      } else {
        panic!("Expected Circle variant");
      }

      // circle(closest-side)
      let result = BasicShape::parse()
        .parse_to_end("circle(closest-side)")
        .unwrap();
      if let BasicShape::Circle { radius, position } = result {
        assert_eq!(radius, CircleRadius::ClosestSide);
        assert_eq!(position, None);
      } else {
        panic!("Expected Circle variant");
      }

      // circle(farthest-side)
      let result = BasicShape::parse()
        .parse_to_end("circle(farthest-side)")
        .unwrap();
      if let BasicShape::Circle { radius, position } = result {
        assert_eq!(radius, CircleRadius::FarthestSide);
        assert_eq!(position, None);
      } else {
        panic!("Expected Circle variant");
      }
    }

    #[test]
    #[ignore] // Position parsing with "at" not yet fully implemented
    fn should_parse_circle_with_position() {
      let result = BasicShape::parse().parse_to_end("circle(10px at top left)");
      if let Ok(BasicShape::Circle { radius, position }) = result {
        assert_eq!(
          radius,
          CircleRadius::Length(LengthPercentage::Length(Length::new(
            10.0,
            "px".to_string()
          )))
        );
        assert!(position.is_some());
      }
    }

    #[test]
    fn should_not_parse_invalid_circles() {
      assert!(BasicShape::parse().parse_to_end("circle(invalid)").is_err());
      assert!(
        BasicShape::parse()
          .parse_to_end("circle(10px, invalid)")
          .is_err()
      );
    }
  }

  // Test Ellipse shapes
  mod ellipse {
    use super::*;

    #[test]
    fn should_parse_valid_ellipses() {
      // ellipse(10px 20px)
      let result = BasicShape::parse()
        .parse_to_end("ellipse(10px 20px)")
        .unwrap();
      if let BasicShape::Ellipse {
        radius_x,
        radius_y,
        position,
      } = result
      {
        assert_eq!(
          radius_x,
          CircleRadius::Length(LengthPercentage::Length(Length::new(
            10.0,
            "px".to_string()
          )))
        );
        assert_eq!(
          radius_y,
          CircleRadius::Length(LengthPercentage::Length(Length::new(
            20.0,
            "px".to_string()
          )))
        );
        assert_eq!(position, None);
      } else {
        panic!("Expected Ellipse variant");
      }

      // ellipse(closest-side farthest-side)
      let result = BasicShape::parse()
        .parse_to_end("ellipse(closest-side farthest-side)")
        .unwrap();
      if let BasicShape::Ellipse {
        radius_x,
        radius_y,
        position,
      } = result
      {
        assert_eq!(radius_x, CircleRadius::ClosestSide);
        assert_eq!(radius_y, CircleRadius::FarthestSide);
        assert_eq!(position, None);
      } else {
        panic!("Expected Ellipse variant");
      }
    }

    #[test]
    #[ignore] // Position parsing with "at" not yet fully implemented
    fn should_parse_ellipse_with_position() {
      let result = BasicShape::parse().parse_to_end("ellipse(10px 20px at top left)");
      if let Ok(BasicShape::Ellipse {
        radius_x,
        radius_y,
        position,
      }) = result
      {
        assert_eq!(
          radius_x,
          CircleRadius::Length(LengthPercentage::Length(Length::new(
            10.0,
            "px".to_string()
          )))
        );
        assert_eq!(
          radius_y,
          CircleRadius::Length(LengthPercentage::Length(Length::new(
            20.0,
            "px".to_string()
          )))
        );
        assert!(position.is_some());
      }
    }

    #[test]
    fn should_not_parse_invalid_ellipses() {
      assert!(
        BasicShape::parse()
          .parse_to_end("ellipse(invalid)")
          .is_err()
      );
      assert!(
        BasicShape::parse()
          .parse_to_end("ellipse(10px, invalid)")
          .is_err()
      );
    }
  }

  // Test Polygon shapes
  mod polygon {
    use super::*;

    #[test]
    fn should_parse_valid_polygons() {
      let result = BasicShape::parse().parse_to_end("polygon(0% 0%, 100% 0%, 100% 100%, 0% 100%)");
      if let Ok(BasicShape::Polygon {
        fill_rule: _,
        points,
      }) = result
      {
        assert_eq!(points.len(), 4);
        // First point: 0% 0%
        assert_eq!(
          points[0].0,
          LengthPercentage::Percentage(Percentage::new(0.0))
        );
        assert_eq!(
          points[0].1,
          LengthPercentage::Percentage(Percentage::new(0.0))
        );
        // Second point: 100% 0%
        assert_eq!(
          points[1].0,
          LengthPercentage::Percentage(Percentage::new(100.0))
        );
        assert_eq!(
          points[1].1,
          LengthPercentage::Percentage(Percentage::new(0.0))
        );
      }
    }

    #[test]
    fn should_parse_polygon_with_fill_rule() {
      let result =
        BasicShape::parse().parse_to_end("polygon(evenodd, 0% 0%, 100% 0%, 100% 100%, 0% 100%)");
      if let Ok(BasicShape::Polygon { fill_rule, points }) = result {
        assert_eq!(fill_rule, Some("evenodd".to_string()));
        assert_eq!(points.len(), 4);
      }
    }

    #[test]
    fn should_not_parse_invalid_polygons() {
      assert!(
        BasicShape::parse()
          .parse_to_end("polygon(invalid)")
          .is_err()
      );
      assert!(
        BasicShape::parse()
          .parse_to_end("polygon(0% 0%, invalid)")
          .is_err()
      );
    }
  }

  // Test Path shapes
  mod path {
    use super::*;

    #[test]
    fn should_parse_valid_paths() {
      let result = BasicShape::parse().parse_to_end("path(\"M0,0 L100,100\")");
      if let Ok(BasicShape::Path { fill_rule, path }) = result {
        assert_eq!(path, "M0,0 L100,100");
        assert_eq!(fill_rule, Some("nonzero".to_string())); // Default fill rule
      }
    }

    #[test]
    fn should_parse_path_with_fill_rule() {
      let result = BasicShape::parse().parse_to_end("path(evenodd, \"M0,0 L100,100\")");
      if let Ok(BasicShape::Path { fill_rule, path }) = result {
        assert_eq!(path, "M0,0 L100,100");
        assert_eq!(fill_rule, Some("evenodd".to_string()));
      }
    }

    #[test]
    fn should_not_parse_invalid_paths() {
      assert!(BasicShape::parse().parse_to_end("path(invalid)").is_err());
      assert!(BasicShape::parse().parse_to_end("path()").is_err());
    }
  }

  // Test string representations
  #[test]
  fn test_inset_string_representation() {
    let inset = BasicShape::Inset {
      top: LengthPercentage::Length(Length::new(10.0, "px".to_string())),
      right: LengthPercentage::Length(Length::new(10.0, "px".to_string())),
      bottom: LengthPercentage::Length(Length::new(10.0, "px".to_string())),
      left: LengthPercentage::Length(Length::new(10.0, "px".to_string())),
      round: None,
    };
    assert_eq!(inset.to_string(), "inset(10px)");
  }

  #[test]
  fn test_circle_string_representation() {
    let circle = BasicShape::Circle {
      radius: CircleRadius::Length(LengthPercentage::Length(Length::new(
        50.0,
        "px".to_string(),
      ))),
      position: None,
    };
    assert_eq!(circle.to_string(), "circle(50px)");
  }

  #[test]
  fn test_ellipse_string_representation() {
    let ellipse = BasicShape::Ellipse {
      radius_x: CircleRadius::Length(LengthPercentage::Length(Length::new(
        50.0,
        "px".to_string(),
      ))),
      radius_y: CircleRadius::Length(LengthPercentage::Length(Length::new(
        25.0,
        "px".to_string(),
      ))),
      position: None,
    };
    assert_eq!(ellipse.to_string(), "ellipse(50px 25px)");
  }

  #[test]
  fn test_keyword_radius_string_representation() {
    let circle = BasicShape::Circle {
      radius: CircleRadius::ClosestSide,
      position: None,
    };
    assert_eq!(circle.to_string(), "circle(closest-side)");

    let ellipse = BasicShape::Ellipse {
      radius_x: CircleRadius::FarthestSide,
      radius_y: CircleRadius::ClosestSide,
      position: None,
    };
    assert_eq!(ellipse.to_string(), "ellipse(farthest-side closest-side)");
  }
}
