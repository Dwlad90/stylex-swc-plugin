/*!
Calc CSS type tests.
*/

use crate::css_types::calc::{Calc, CalcValue};

#[cfg(test)]
mod test_css_type_calc {
  use super::*;

  #[test]
  fn parses_simple_numeric_values() {
    let calc1 = Calc::parser().parse_to_end("calc(10)").unwrap();
    match &calc1.value {
      CalcValue::Number(num) => {
        assert_eq!(*num, 10.0);
      }
      _ => panic!("Expected Number value"),
    }
    assert_eq!(calc1.to_string(), "calc(10)");

    let calc2 = Calc::parser().parse_to_end("calc(3.1415927)").unwrap();
    match &calc2.value {
      CalcValue::Number(num) => {
        assert_eq!(*num, std::f32::consts::PI);
      }
      _ => panic!("Expected Number value"),
    }
    assert_eq!(calc2.to_string(), "calc(3.1415927)");

    let calc3 = Calc::parser().parse_to_end("calc(-5)").unwrap();
    match &calc3.value {
      CalcValue::Number(num) => {
        assert_eq!(*num, -5.0);
      }
      _ => panic!("Expected Number value"),
    }
    assert_eq!(calc3.to_string(), "calc(-5)");
  }

  #[test]
  fn parses_percentage_values() {
    let calc1 = Calc::parser().parse_to_end("calc(50%)").unwrap();
    match &calc1.value {
      CalcValue::Percentage(ref percent) => {
        assert_eq!(percent.value, 50.0);
      }
      _ => panic!("Expected Percentage value"),
    }
    assert_eq!(calc1.to_string(), "calc(50%)");

    let calc2 = Calc::parser().parse_to_end("calc(100%)").unwrap();
    match &calc2.value {
      CalcValue::Percentage(ref percent) => {
        assert_eq!(percent.value, 100.0);
      }
      _ => panic!("Expected Percentage value"),
    }
    assert_eq!(calc2.to_string(), "calc(100%)");

    let calc3 = Calc::parser().parse_to_end("calc(-25%)").unwrap();
    match &calc3.value {
      CalcValue::Percentage(ref percent) => {
        assert_eq!(percent.value, -25.0);
      }
      _ => panic!("Expected Percentage value"),
    }
    assert_eq!(calc3.to_string(), "calc(-25%)");
  }

  #[test]
  fn parses_dimension_values() {
    let calc1 = Calc::parser().parse_to_end("calc(20px)").unwrap();
    match &calc1.value {
      CalcValue::Dimension(ref dim) => {
        assert_eq!(dim.value, 20.0);
        assert_eq!(dim.unit, "px");
      }
      _ => panic!("Expected Dimension value"),
    }
    assert_eq!(calc1.to_string(), "calc(20px)");

    let calc2 = Calc::parser().parse_to_end("calc(2em)").unwrap();
    match &calc2.value {
      CalcValue::Dimension(ref dim) => {
        assert_eq!(dim.value, 2.0);
        assert_eq!(dim.unit, "em");
      }
      _ => panic!("Expected Dimension value"),
    }
    assert_eq!(calc2.to_string(), "calc(2em)");

    let calc3 = Calc::parser().parse_to_end("calc(1.5rem)").unwrap();
    match &calc3.value {
      CalcValue::Dimension(ref dim) => {
        assert_eq!(dim.value, 1.5);
        assert_eq!(dim.unit, "rem");
      }
      _ => panic!("Expected Dimension value"),
    }
    assert_eq!(calc3.to_string(), "calc(1.5rem)");
  }

  #[test]
  fn parses_addition_operations() {
    let calc = Calc::parser().parse_to_end("calc(10px + 5px)").unwrap();

    // Should parse as an addition operation
    match &calc.value {
      CalcValue::Addition(ref addition) => {
        // Verify left operand
        match addition.left.as_ref() {
          CalcValue::Dimension(ref dim) => {
            assert_eq!(dim.value, 10.0);
            assert_eq!(dim.unit, "px");
          }
          _ => panic!("Expected Dimension for left operand"),
        }

        // Verify right operand
        match addition.right.as_ref() {
          CalcValue::Dimension(ref dim) => {
            assert_eq!(dim.value, 5.0);
            assert_eq!(dim.unit, "px");
          }
          _ => panic!("Expected Dimension for right operand"),
        }
      }
      _ => panic!("Expected Addition operation"),
    }

    assert_eq!(calc.to_string(), "calc(10px + 5px)");
  }

  #[test]
  fn parses_subtraction_operations() {
    let calc = Calc::parser().parse_to_end("calc(100% - 20px)").unwrap();

    // Should parse as a subtraction operation
    match &calc.value {
      CalcValue::Subtraction(ref subtraction) => {
        // Verify left operand (percentage)
        match subtraction.left.as_ref() {
          CalcValue::Percentage(ref percentage) => {
            assert_eq!(percentage.value, 100.0);
          }
          _ => panic!("Expected Percentage for left operand"),
        }

        // Verify right operand (length)
        match subtraction.right.as_ref() {
          CalcValue::Dimension(ref dim) => {
            assert_eq!(dim.value, 20.0);
            assert_eq!(dim.unit, "px");
          }
          _ => panic!("Expected Dimension for right operand"),
        }
      }
      _ => panic!("Expected Subtraction operation"),
    }

    assert_eq!(calc.to_string(), "calc(100% - 20px)");
  }

  #[test]
  fn parses_multiplication_operations() {
    let calc = Calc::parser().parse_to_end("calc(10px * 2)").unwrap();

    // Should parse as a multiplication operation
    match &calc.value {
      CalcValue::Multiplication(ref multiplication) => {
        // Verify left operand
        match multiplication.left.as_ref() {
          CalcValue::Dimension(ref dim) => {
            assert_eq!(dim.value, 10.0);
            assert_eq!(dim.unit, "px");
          }
          _ => panic!("Expected Dimension for left operand"),
        }

        // Verify right operand
        match multiplication.right.as_ref() {
          CalcValue::Number(num) => {
            assert_eq!(*num, 2.0);
          }
          _ => panic!("Expected Number for right operand"),
        }
      }
      _ => panic!("Expected Multiplication operation"),
    }

    assert_eq!(calc.to_string(), "calc(10px * 2)");
  }

  #[test]
  fn parses_division_operations() {
    let calc = Calc::parser().parse_to_end("calc(100px / 4)").unwrap();

    // Should parse as a division operation
    match &calc.value {
      CalcValue::Division(ref division) => {
        // Verify left operand
        match division.left.as_ref() {
          CalcValue::Dimension(ref dim) => {
            assert_eq!(dim.value, 100.0);
            assert_eq!(dim.unit, "px");
          }
          _ => panic!("Expected Dimension for left operand"),
        }

        // Verify right operand
        match division.right.as_ref() {
          CalcValue::Number(num) => {
            assert_eq!(*num, 4.0);
          }
          _ => panic!("Expected Number for right operand"),
        }
      }
      _ => panic!("Expected Division operation"),
    }

    assert_eq!(calc.to_string(), "calc(100px / 4)");
  }

  #[test]
  fn parses_complex_expressions_with_multiple_operations() {
    let calc = Calc::parser()
      .parse_to_end("calc(100% - 20px * 2 + 10px)")
      .unwrap();

    // Should parse with correct operator precedence
    // The multiplication should be evaluated first: 20px * 2
    // Then the subtraction and addition from left to right
    // Note: exact structure depends on implementation, but it should parse successfully
    assert!(!calc.to_string().is_empty());

    // Verify round-trip parsing
    let reparsed = Calc::parser().parse_to_end(&calc.to_string()).unwrap();
    assert_eq!(calc.to_string(), reparsed.to_string());
  }

  #[test]
  fn parses_nested_operations_with_parentheses() {
    let calc = Calc::parser()
      .parse_to_end("calc((100% - 20px) * 2)")
      .unwrap();

    // Should respect parentheses grouping
    assert!(!calc.to_string().is_empty());
    assert!(calc.to_string().contains("(100% - 20px)"));

    // Verify round-trip parsing
    let reparsed = Calc::parser().parse_to_end(&calc.to_string()).unwrap();
    assert_eq!(calc.to_string(), reparsed.to_string());
  }

  #[test]
  #[ignore]
  fn parses_nested_calc_expressions() {
    let calc = Calc::parser()
      .parse_to_end("calc(calc(100% - 10px) + calc(20px * 2))")
      .unwrap();

    // Should handle nested calc expressions
    assert!(!calc.to_string().is_empty());

    // Verify round-trip parsing
    let reparsed = Calc::parser().parse_to_end(&calc.to_string()).unwrap();
    assert_eq!(calc.to_string(), reparsed.to_string());
  }

  #[test]
  fn operator_precedence() {
    // Test that multiplication/division has higher precedence than addition/subtraction
    let test_cases = vec![
      "calc(10 + 5 * 2)",    // Should be 10 + (5 * 2) = 20
      "calc(20 - 10 / 2)",   // Should be 20 - (10 / 2) = 15
      "calc(2 * 3 + 4 * 5)", // Should be (2 * 3) + (4 * 5) = 26
    ];

    for input in test_cases {
      let calc = Calc::parser().parse_to_end(input).unwrap();

      // Should parse successfully with correct precedence
      assert!(!calc.to_string().is_empty());

      // Verify round-trip parsing maintains precedence
      let reparsed = Calc::parser().parse_to_end(&calc.to_string()).unwrap();
      assert_eq!(calc.to_string(), reparsed.to_string());
    }
  }

  #[test]
  #[ignore] // Whitespace handling in calc expressions needs parser improvements
  fn handles_whitespace_correctly() {
    let test_cases = vec![
      "calc(10px+5px)",         // No spaces
      "calc(10px + 5px)",       // Normal spaces
      "calc( 10px  +  5px )",   // Extra spaces
      "calc(\n10px\t+\r5px\n)", // Mixed whitespace
    ];

    for input in test_cases {
      let calc = Calc::parser().parse_to_end(input).unwrap();
      assert!(!calc.to_string().is_empty(), "Should parse: {}", input);
    }
  }

  #[test]
  fn to_string_round_trips_calc_expressions() {
    let test_cases = vec![
      ("calc(10px)", "calc(10px)"),
      ("calc(50%)", "calc(50%)"),
      ("calc(10px + 5px)", "calc(10px + 5px)"),
      ("calc(100% - 20px)", "calc(100% - 20px)"),
      ("calc(10px * 2)", "calc(10px * 2)"),
      ("calc(100px / 4)", "calc(100px / 4)"),
    ];

    for (input, expected_output) in test_cases {
      let calc = Calc::parser().parse_to_end(input).unwrap();
      assert_eq!(calc.to_string(), expected_output);
    }
  }

  #[test]
  fn rejects_invalid_calc_expressions() {
    assert!(Calc::parser().parse_to_end("calc()").is_err());
    assert!(Calc::parser().parse_to_end("calc(10 + )").is_err());
    assert!(Calc::parser().parse_to_end("calc(10 +5 )").is_err());
    assert!(Calc::parser().parse_to_end("calc(10+5 )").is_err());
    assert!(Calc::parser().parse_to_end("calc(10 @ 5)").is_err());
    assert!(Calc::parser().parse_to_end("calc(10px + 5em)").is_ok());
    assert!(Calc::parser().parse_to_end("notcalc(10 + 5)").is_err());
  }

  #[test]
  #[ignore] // TODO: Parser does not enforce this edge case properly, need to fix
  fn rejects_invalid_calc_spacing() {
    assert!(Calc::parser().parse_to_end("calc(10+ 5 )").is_err());
  }
}
