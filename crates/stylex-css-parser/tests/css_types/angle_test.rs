#[cfg(test)]
mod angle_tests {

  use stylex_css_parser::{
    base_types::SubString,
    css_types::angle::{Angle, AngleUnit},
  };

  #[test]
  fn parses_css_angle_types_strings_correctly() {
    // Test deg angles
    assert_eq!(
      Angle::parse().parse("0deg").unwrap(),
      Angle {
        value: 0.0,
        unit: AngleUnit::Deg
      }
    );

    assert_eq!(
      Angle::parse().parse("45deg").unwrap(),
      Angle {
        value: 45.0,
        unit: AngleUnit::Deg
      }
    );

    assert_eq!(
      Angle::parse().parse("90deg").unwrap(),
      Angle {
        value: 90.0,
        unit: AngleUnit::Deg
      }
    );

    assert_eq!(
      Angle::parse().parse("180deg").unwrap(),
      Angle {
        value: 180.0,
        unit: AngleUnit::Deg
      }
    );

    assert_eq!(
      Angle::parse().parse("270deg").unwrap(),
      Angle {
        value: 270.0,
        unit: AngleUnit::Deg
      }
    );

    assert_eq!(
      Angle::parse().parse("-90deg").unwrap(),
      Angle {
        value: -90.0,
        unit: AngleUnit::Deg
      }
    );

    // Test turn angles
    assert_eq!(
      Angle::parse().parse("0.5turn").unwrap(),
      Angle {
        value: 0.5,
        unit: AngleUnit::Turn
      }
    );

    // Test rad angles
    assert_eq!(
      Angle::parse().parse("2rad").unwrap(),
      Angle {
        value: 2.0,
        unit: AngleUnit::Rad
      }
    );

    // Test grad angles
    assert_eq!(
      Angle::parse().parse("100grad").unwrap(),
      Angle {
        value: 100.0,
        unit: AngleUnit::Grad
      }
    );

    // Test decimal angles
    assert_eq!(
      Angle::parse().parse("1.5deg").unwrap(),
      Angle {
        value: 1.5,
        unit: AngleUnit::Deg
      }
    );
  }

  #[test]
  fn parses_css_angle_types_substrings_correctly() {
    // Create a SubString and test parsing from it
    let mut val = SubString::new("0deg");
    let result = Angle::parse().run(&mut val).unwrap();

    assert_eq!(
      result,
      Angle {
        value: 0.0,
        unit: AngleUnit::Deg
      }
    );

    // Check that the substring was consumed
    assert_eq!(val.to_string(), "");

    // Test parsing with trailing content
    let mut val = SubString::new("45deg foo");
    let result = Angle::parse().run(&mut val).unwrap();

    assert_eq!(
      result,
      Angle {
        value: 45.0,
        unit: AngleUnit::Deg
      }
    );

    // Check that only the angle part was consumed
    assert_eq!(val.to_string(), " foo");
  }
}

// Add a separate module for specific angle unit tests
#[cfg(test)]
mod angle_unit_tests {
  use std::f32::consts;
  use stylex_css_parser::css_types::angle::{AngleUnit, Deg, Grad, Rad, Turn};

  #[test]
  fn test_deg_unit() {
    let deg = Deg::new(90.0);
    assert_eq!(deg.value, 90.0);
    assert_eq!(deg.unit, AngleUnit::Deg);
    assert_eq!(format!("{}", deg), "90deg");
  }

  #[test]
  fn test_grad_unit() {
    let grad = Grad::new(100.0);
    assert_eq!(grad.value, 100.0);
    assert_eq!(grad.unit, AngleUnit::Grad);
    assert_eq!(format!("{}", grad), "100grad");
  }

  #[test]
  fn test_rad_unit() {
    let rad = Rad::new(consts::PI);
    assert_eq!(rad.value, consts::PI);
    assert_eq!(rad.unit, AngleUnit::Rad);
    assert_eq!(format!("{}", rad), format!("{}rad", consts::PI));
  }

  #[test]
  fn test_turn_unit() {
    let turn = Turn::new(0.5);
    assert_eq!(turn.value, 0.5);
    assert_eq!(turn.unit, AngleUnit::Turn);
    assert_eq!(format!("{}", turn), "0.5turn");
  }
}

#[cfg(test)]
mod angle_parser_tests {
  use stylex_css_parser::css_types::angle::{Angle, AngleUnit, Deg, Grad, Rad, Turn};

  #[test]
  fn test_specific_angle_parsers() {
    // Test Deg parser
    assert_eq!(
      Deg::parse().parse("45deg").unwrap(),
      Angle {
        value: 45.0,
        unit: AngleUnit::Deg
      }
    );

    // Test Grad parser
    assert_eq!(
      Grad::parse().parse("50grad").unwrap(),
      Angle {
        value: 50.0,
        unit: AngleUnit::Grad
      }
    );

    // Test Rad parser
    assert_eq!(
      Rad::parse().parse("1.57rad").unwrap(),
      Angle {
        value: 1.57,
        unit: AngleUnit::Rad
      }
    );

    // Test Turn parser
    assert_eq!(
      Turn::parse().parse("0.25turn").unwrap(),
      Angle {
        value: 0.25,
        unit: AngleUnit::Turn
      }
    );
  }

  #[test]
  fn test_zero_angle() {
    // Test that "0" is parsed as a dimensionless angle
    assert_eq!(
      Angle::parse().parse("0").unwrap(),
      Angle {
        value: 0.0,
        unit: AngleUnit::Default
      }
    );
  }

  #[test]
  fn test_invalid_angles() {
    // Test invalid angle values
    assert!(Angle::parse().parse("deg").is_err());
    assert!(Angle::parse().parse("10").is_err()); // No unit but not zero
    assert!(Angle::parse().parse("10px").is_err()); // Wrong unit
    assert!(Angle::parse().parse("").is_err()); // Empty string
  }
}
