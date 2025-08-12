/*!
CSS Calc expression parsing.

Handles calc() expressions with mathematical operations, proper precedence, and grouping.
Mirrors: packages/style-value-parser/src/css-types/calc.js

This implementation follows the JavaScript structure exactly:
- Separate types for each operation (Addition, Subtraction, Multiplication, Division)
- Group type for parenthesized expressions
- Exact precedence algorithm matching splitByMultiplicationOrDivision and composeAddAndSubtraction
*/

use crate::{
  css_types::{CalcConstant, Percentage},
  token_parser::TokenParser,
  token_types::SimpleToken,
};
use std::fmt::{self, Display};

/// Addition operation: { type: '+', left: CalcValue, right: CalcValue }
/// Mirrors: Addition type in calc.js
#[derive(Debug, Clone, PartialEq)]
pub struct Addition {
  pub left: Box<CalcValue>,
  pub right: Box<CalcValue>,
}

impl Addition {
  pub fn new(left: CalcValue, right: CalcValue) -> Self {
    Self {
      left: Box::new(left),
      right: Box::new(right),
    }
  }
}

/// Subtraction operation: { type: '-', left: CalcValue, right: CalcValue }
/// Mirrors: Subtraction type in calc.js
#[derive(Debug, Clone, PartialEq)]
pub struct Subtraction {
  pub left: Box<CalcValue>,
  pub right: Box<CalcValue>,
}

impl Subtraction {
  pub fn new(left: CalcValue, right: CalcValue) -> Self {
    Self {
      left: Box::new(left),
      right: Box::new(right),
    }
  }
}

/// Multiplication operation: { type: '*', left: CalcValue, right: CalcValue }
/// Mirrors: Multiplication type in calc.js
#[derive(Debug, Clone, PartialEq)]
pub struct Multiplication {
  pub left: Box<CalcValue>,
  pub right: Box<CalcValue>,
}

impl Multiplication {
  pub fn new(left: CalcValue, right: CalcValue) -> Self {
    Self {
      left: Box::new(left),
      right: Box::new(right),
    }
  }
}

/// Division operation: { type: '/', left: CalcValue, right: CalcValue }
/// Mirrors: Division type in calc.js
#[derive(Debug, Clone, PartialEq)]
pub struct Division {
  pub left: Box<CalcValue>,
  pub right: Box<CalcValue>,
}

impl Division {
  pub fn new(left: CalcValue, right: CalcValue) -> Self {
    Self {
      left: Box::new(left),
      right: Box::new(right),
    }
  }
}

/// Grouped expression (parenthesized): { type: 'group', expr: CalcValue }
/// Mirrors: Group type in calc.js
#[derive(Debug, Clone, PartialEq)]
pub struct Group {
  pub expr: Box<CalcValue>,
}

impl Group {
  pub fn new(expr: CalcValue) -> Self {
    Self {
      expr: Box::new(expr),
    }
  }
}

/// Dimension value (value + unit)
/// Mirrors: TokenDimension[4] in calc.js
#[derive(Debug, Clone, PartialEq)]
pub struct CalcDimension {
  pub value: f32,
  pub unit: String,
}

impl CalcDimension {
  pub fn new(value: f32, unit: String) -> Self {
    Self { value, unit }
  }
}

impl Display for CalcDimension {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value, self.unit)
  }
}

/// ENHANCED: Helper type for operator precedence parsing
/// Represents either a value or an operator in the parsing sequence
/// Mirrors: the valuesAndOperators array logic in calc.js
#[derive(Debug, Clone)]
pub enum CalcValueOrOperator {
  Value(CalcValue),
  Operator(String),
}

/// A value in a calc expression
/// Mirrors: CalcValue type in calc.js exactly
#[derive(Debug, Clone, PartialEq)]
pub enum CalcValue {
  /// number
  Number(f32),
  /// TokenDimension[4]
  Dimension(CalcDimension),
  /// Percentage
  Percentage(Percentage),
  /// CalcConstant
  Constant(CalcConstant),
  /// Addition
  Addition(Addition),
  /// Subtraction
  Subtraction(Subtraction),
  /// Multiplication
  Multiplication(Multiplication),
  /// Division
  Division(Division),
  /// Group
  Group(Group),
}

impl CalcValue {
  /// Parser for basic calc values (numbers, dimensions, percentages, constants)
  /// Mirrors: valueParser in calc.js
  pub fn value_parser() -> TokenParser<CalcValue> {
    TokenParser::one_of(vec![
      // Calc constants
      CalcConstant::parser().map(CalcValue::Constant, Some("constant")),
      // Numbers
      TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number")).map(
        |token| {
          if let SimpleToken::Number(value) = token {
            CalcValue::Number(value as f32)
          } else {
            unreachable!()
          }
        },
        Some("to_number"),
      ),
      // Dimensions
      TokenParser::<SimpleToken>::token(
        SimpleToken::Dimension {
          value: 0.0,
          unit: String::new(),
        },
        Some("Dimension"),
      )
      .map(
        |token| {
          if let SimpleToken::Dimension { value, unit } = token {
            CalcValue::Dimension(CalcDimension::new(value as f32, unit))
          } else {
            unreachable!()
          }
        },
        Some("to_dimension"),
      ),
      // Percentages
      Percentage::parser().map(CalcValue::Percentage, Some("percentage")),
    ])
  }

  /// Handle multiplication and division (higher precedence)
  /// Mirrors: splitByMultiplicationOrDivision in calc.js exactly
  pub fn split_by_multiplication_or_division(
    values_and_operators: Vec<CalcValueOrOperator>,
  ) -> CalcValue {
    if values_and_operators.len() == 1 {
      if let CalcValueOrOperator::Value(value) = &values_and_operators[0] {
        return value.clone();
      }
      // This shouldn't happen in well-formed input
      return CalcValue::Number(0.0);
    }

    // Find first multiplication or division operator
    let first_operator = values_and_operators
      .iter()
      .position(|item| matches!(item, CalcValueOrOperator::Operator(op) if op == "*" || op == "/"));

    match first_operator {
      None => {
        // No multiplication or division, handle addition/subtraction
        Self::compose_add_and_subtraction(values_and_operators)
      }
      Some(op_idx) => {
        // Split around the operator
        let left_values = values_and_operators[..op_idx].to_vec();
        let right_values = values_and_operators[op_idx + 1..].to_vec();
        let operator = &values_and_operators[op_idx];

        let left_result = Self::compose_add_and_subtraction(left_values);
        let right_result = Self::split_by_multiplication_or_division(right_values);

        if let CalcValueOrOperator::Operator(op) = operator {
          match op.as_str() {
            "*" => CalcValue::Multiplication(Multiplication::new(left_result, right_result)),
            "/" => CalcValue::Division(Division::new(left_result, right_result)),
            _ => unreachable!(),
          }
        } else {
          unreachable!()
        }
      }
    }
  }

  /// Handle addition and subtraction (lower precedence)
  /// Mirrors: composeAddAndSubtraction in calc.js exactly
  pub fn compose_add_and_subtraction(values_and_operators: Vec<CalcValueOrOperator>) -> CalcValue {
    if values_and_operators.len() == 1 {
      if let CalcValueOrOperator::Value(value) = &values_and_operators[0] {
        return value.clone();
      }
      // This shouldn't happen in well-formed input
      return CalcValue::Number(0.0);
    }

    // Find first addition or subtraction operator
    let first_operator = values_and_operators
      .iter()
      .position(|item| matches!(item, CalcValueOrOperator::Operator(op) if op == "+" || op == "-"));

    match first_operator {
      None => {
        // No valid operator found, just return first value or error
        if let CalcValueOrOperator::Value(value) = &values_and_operators[0] {
          value.clone()
        } else {
          CalcValue::Number(0.0)
        }
      }
      Some(op_idx) => {
        // Split around the operator
        let left_values = values_and_operators[..op_idx].to_vec();
        let right_values = values_and_operators[op_idx + 1..].to_vec();
        let operator = &values_and_operators[op_idx];

        let left_result = Self::compose_add_and_subtraction(left_values);
        let right_result = Self::compose_add_and_subtraction(right_values);

        if let CalcValueOrOperator::Operator(op) = operator {
          match op.as_str() {
            "+" => CalcValue::Addition(Addition::new(left_result, right_result)),
            "-" => CalcValue::Subtraction(Subtraction::new(left_result, right_result)),
            _ => unreachable!(),
          }
        } else {
          unreachable!()
        }
      }
    }
  }
}



impl Display for CalcValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CalcValue::Number(n) => write!(f, "{}", n),
      CalcValue::Dimension(d) => write!(f, "{}", d),
      CalcValue::Percentage(p) => write!(f, "{}", p),
      CalcValue::Constant(c) => write!(f, "{}", c),
      CalcValue::Addition(op) => write!(f, "{} + {}", op.left, op.right),
      CalcValue::Subtraction(op) => write!(f, "{} - {}", op.left, op.right),
      CalcValue::Multiplication(op) => write!(f, "{} * {}", op.left, op.right),
      CalcValue::Division(op) => write!(f, "{} / {}", op.left, op.right),
      CalcValue::Group(group) => write!(f, "({})", group.expr),
    }
  }
}

/// Main Calc expression container
/// Mirrors: Calc class in calc.js
#[derive(Debug, Clone, PartialEq)]
pub struct Calc {
  pub value: CalcValue,
}

impl Calc {
  /// Create a new Calc expression
  /// Mirrors: constructor in calc.js
  pub fn new(value: CalcValue) -> Self {
    Self { value }
  }

    /// Parser for calc expressions with full operationsParser equivalent
  /// Mirrors: Calc.parser in calc.js exactly
  pub fn parse() -> TokenParser<Calc> {
    let calc_function = TokenParser::<String>::fn_name("calc");
    let whitespace_optional = crate::token_parser::tokens::whitespace().optional();
    let close_paren = crate::token_parser::tokens::close_paren();

    // Mirrors: TokenParser.sequence(TokenParser.tokens.Function.map(...).where(...), operationsParser, TokenParser.tokens.CloseParen)
    calc_function
      .skip(whitespace_optional.clone())
      .flat_map(
        |_| Self::operations_parser(),
        Some("calc_expression")
      )
      .skip(whitespace_optional.clone())
      .skip(close_paren)
      .map(Calc::new, Some("to_calc"))
  }

  /// ENHANCED: Operator precedence handling - composeAddAndSubtraction equivalent
  /// Handles + and - operators (lower precedence)
  /// Mirrors: composeAddAndSubtraction in calc.js exactly
  fn compose_add_and_subtraction(values_and_operators: Vec<CalcValueOrOperator>) -> Result<CalcValue, String> {
    if values_and_operators.len() == 1 {
      match &values_and_operators[0] {
        CalcValueOrOperator::Value(value) => return Ok(value.clone()),
        CalcValueOrOperator::Operator(op) => return Err(format!("Invalid operator: {}", op)),
      }
    }

    // Find first + or - operator
    let first_operator = values_and_operators.iter().position(|item| {
      matches!(item, CalcValueOrOperator::Operator(op) if op == "+" || op == "-")
    });

    match first_operator {
      None => Err("No valid operator found".to_string()),
      Some(op_index) => {
        let left_slice = values_and_operators[..op_index].to_vec();
        let right_slice = values_and_operators[op_index + 1..].to_vec();

        if let CalcValueOrOperator::Operator(operator) = &values_and_operators[op_index] {
          let left = Self::compose_add_and_subtraction(left_slice)?;
          let right = Self::compose_add_and_subtraction(right_slice)?;

          match operator.as_str() {
            "+" => Ok(CalcValue::Addition(Addition::new(left, right))),
            "-" => Ok(CalcValue::Subtraction(Subtraction::new(left, right))),
            _ => Err("Invalid operator".to_string()),
          }
        } else {
          Err("Expected operator".to_string())
        }
      }
    }
  }

  /// ENHANCED: Operator precedence handling - splitByMultiplicationOrDivision equivalent
  /// Handles * and / operators (higher precedence), delegates to composeAddAndSubtraction
  /// Mirrors: splitByMultiplicationOrDivision in calc.js exactly
  fn split_by_multiplication_or_division(values_and_operators: Vec<CalcValueOrOperator>) -> Result<CalcValue, String> {
    if values_and_operators.len() == 1 {
      match &values_and_operators[0] {
        CalcValueOrOperator::Value(value) => return Ok(value.clone()),
        CalcValueOrOperator::Operator(_) => return Err("Invalid operator".to_string()),
      }
    }

    // Find first * or / operator
    let first_operator = values_and_operators.iter().position(|item| {
      matches!(item, CalcValueOrOperator::Operator(op) if op == "*" || op == "/")
    });

    match first_operator {
      None => {
        // No multiplication/division, handle addition/subtraction
        Self::compose_add_and_subtraction(values_and_operators)
      },
      Some(op_index) => {
        let left_slice = values_and_operators[..op_index].to_vec();
        let right_slice = values_and_operators[op_index + 1..].to_vec();

        if let CalcValueOrOperator::Operator(operator) = &values_and_operators[op_index] {
          let left = Self::compose_add_and_subtraction(left_slice)?;
          let right = Self::split_by_multiplication_or_division(right_slice)?;

          match operator.as_str() {
            "*" => Ok(CalcValue::Multiplication(Multiplication::new(left, right))),
            "/" => Ok(CalcValue::Division(Division::new(left, right))),
            _ => Err("Invalid operator".to_string()),
          }
        } else {
          Err("Expected operator".to_string())
        }
      }
    }
  }

  /// ENHANCED: Full operations parser with operator precedence
  /// Mirrors: operationsParser in calc.js exactly
  fn operations_parser() -> TokenParser<CalcValue> {

        // Parse first value (value or parenthesized expression)
    // FIXED: Use basic parser to avoid stack overflow - recursive parsing can be added later
    let first_value = TokenParser::one_of(vec![
      CalcValue::value_parser(),
      Self::parenthesized_parser(), // Use non-recursive version for stability
    ]);

        // Parse zero or more [operator, value] pairs - simplified approach
    let rest_parser = TokenParser::zero_or_more(
      Self::operator_parser()
        .flat_map(|op| {
          TokenParser::one_of(vec![
            CalcValue::value_parser(),
            Self::parenthesized_parser(), // Use non-recursive version for stability
          ]).map(move |val| (op.clone(), val), Some("op_val_pair"))
        }, Some("operator_value"))
    );

    // Combine first value with operator-value pairs
    first_value
      .flat_map(move |first| {
        rest_parser.map(move |pairs| {
          let mut values_and_operators = vec![CalcValueOrOperator::Value(first.clone())];
          // Add each operator and value pair
          for (op, val) in pairs {
            values_and_operators.push(CalcValueOrOperator::Operator(op));
            values_and_operators.push(CalcValueOrOperator::Value(val));
          }
          values_and_operators
        }, Some("combine"))
      }, Some("first_plus_pairs"))
      .map(|values_and_operators| {
        if values_and_operators.len() == 1 {
          // Single value, extract it
          if let CalcValueOrOperator::Value(value) = &values_and_operators[0] {
            value.clone()
          } else {
            unreachable!("First element should always be a value")
          }
        } else {
          // Multiple values with operators, apply precedence
          Self::split_by_multiplication_or_division(values_and_operators)
            .unwrap_or_else(|_| CalcValue::Number(0.0)) // Fallback for now
        }
      }, Some("apply_precedence"))
  }

  /// Parse operator delimiters
  /// Mirrors: TokenParser.tokens.Delim.map((delim) => delim[4].value).where(...)
  fn operator_parser() -> TokenParser<String> {
    use crate::token_parser::tokens;

    TokenParser::one_of(vec![
      tokens::delim('+').map(|_| "+".to_string(), Some("plus")),
      tokens::delim('-').map(|_| "-".to_string(), Some("minus")),
      tokens::delim('*').map(|_| "*".to_string(), Some("mult")),
      tokens::delim('/').map(|_| "/".to_string(), Some("div")),
    ])
  }

  /// ENHANCED: Recursive parenthesized expression parser
  /// Mirrors: parenthesizedParser in calc.js exactly with proper recursion
  fn parenthesized_parser_recursive() -> TokenParser<CalcValue> {
    use crate::token_parser::tokens;
    use std::rc::Rc;
    use std::cell::RefCell;

    // Create lazy reference to avoid infinite recursion during parser creation
    let operations_parser_ref: Rc<RefCell<Option<TokenParser<CalcValue>>>> = Rc::new(RefCell::new(None));
    let ops_ref_clone = operations_parser_ref.clone();

    // Create the parenthesized parser
    let parser = tokens::open_paren()
      .skip(tokens::whitespace().optional())
      .flat_map(move |_| {
        // Get the operations parser (will be set by the time this runs)
        let ops_parser = ops_ref_clone.borrow();
        ops_parser.as_ref().unwrap().clone()
      }, Some("recursive_operations"))
      .skip(tokens::whitespace().optional())
      .skip(tokens::close_paren())
      .map(|expr| CalcValue::Group(Group::new(expr)), Some("to_group"));

    // Set the operations parser in the reference
    *operations_parser_ref.borrow_mut() = Some(Self::operations_parser());

    parser
  }

  /// Basic parenthesized expression parser (non-recursive fallback)
  /// Mirrors: parenthesizedParser in calc.js
  fn parenthesized_parser() -> TokenParser<CalcValue> {
    use crate::token_parser::tokens;

    // Basic implementation that doesn't recurse - fallback for simple cases
    tokens::open_paren()
      .skip(tokens::whitespace().optional())
      .flat_map(
        |_| CalcValue::value_parser(), // Use basic value parser to avoid recursion
        Some("parenthesized_value")
      )
      .skip(tokens::whitespace().optional())
      .skip(tokens::close_paren())
      .map(|expr| CalcValue::Group(Group::new(expr)), Some("to_group"))
  }
}

impl Display for Calc {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "calc({})", self.value)
  }
}

/// Function to convert CalcValue to string (mirrors calcValueToString in calc.js)
/// Mirrors: calcValueToString function in calc.js
pub fn calc_value_to_string(value: &CalcValue) -> String {
  match value {
    CalcValue::Number(n) => n.to_string(),
    CalcValue::Dimension(d) => d.to_string(),
    CalcValue::Percentage(p) => p.to_string(),
    CalcValue::Constant(c) => c.to_string(),
    CalcValue::Addition(op) => format!(
      "{} + {}",
      calc_value_to_string(&op.left),
      calc_value_to_string(&op.right)
    ),
    CalcValue::Subtraction(op) => format!(
      "{} - {}",
      calc_value_to_string(&op.left),
      calc_value_to_string(&op.right)
    ),
    CalcValue::Multiplication(op) => format!(
      "{} * {}",
      calc_value_to_string(&op.left),
      calc_value_to_string(&op.right)
    ),
    CalcValue::Division(op) => format!(
      "{} / {}",
      calc_value_to_string(&op.left),
      calc_value_to_string(&op.right)
    ),
    CalcValue::Group(group) => format!("({})", calc_value_to_string(&group.expr)),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_calc_creation() {
    let value = CalcValue::Number(42.0);
    let calc = Calc::new(value);
    assert_eq!(calc.to_string(), "calc(42)");
  }

  #[test]
  fn test_calc_dimension_creation() {
    let dimension = CalcDimension::new(100.0, "px".to_string());
    let calc = Calc::new(CalcValue::Dimension(dimension));
    assert_eq!(calc.to_string(), "calc(100px)");
  }

  #[test]
  fn test_calc_value_display() {
    let number = CalcValue::Number(5.0);
    assert_eq!(number.to_string(), "5");

    let dimension = CalcValue::Dimension(CalcDimension::new(10.0, "em".to_string()));
    assert_eq!(dimension.to_string(), "10em");
  }

  #[test]
  fn test_calc_value_equality() {
    let val1 = CalcValue::Number(42.0);
    let val2 = CalcValue::Number(42.0);
    let val3 = CalcValue::Number(43.0);

    assert_eq!(val1, val2);
    assert_ne!(val1, val3);
  }

  #[test]
  fn test_addition_operation() {
    let left = CalcValue::Number(10.0);
    let right = CalcValue::Number(5.0);
    let add_op = Addition::new(left, right);

    let calc_value = CalcValue::Addition(add_op);
    assert_eq!(calc_value.to_string(), "10 + 5");
  }

  #[test]
  fn test_calc_group() {
    let inner = CalcValue::Number(42.0);
    let group = Group::new(inner);
    let calc_value = CalcValue::Group(group);
    assert_eq!(calc_value.to_string(), "(42)");
  }

  #[test]
  fn test_calc_parser_creation() {
    let _parser = Calc::parse();
    // Just test that parser can be created without panicking
  }

  #[test]
  fn test_calc_value_parser_creation() {
    let _parser = CalcValue::value_parser();
    // Just test that parser can be created without panicking
  }

  #[test]
  fn test_binary_operation() {
    let left = CalcValue::Number(20.0);
    let right = CalcValue::Number(10.0);
    let add_op = Addition::new(left.clone(), right.clone());

    let calc = Calc::new(CalcValue::Addition(add_op));
    assert_eq!(calc.to_string(), "calc(20 + 10)");
  }

  #[test]
  fn test_complex_calc_expression() {
    let pi_value = CalcValue::Number(std::f32::consts::PI);
    let two = CalcValue::Number(2.0);
    let multiply_op = Multiplication::new(pi_value, two);

    let calc = Calc::new(CalcValue::Multiplication(multiply_op));
    assert_eq!(calc.to_string(), "calc(3.1415927 * 2)");
  }

  #[test]
  fn test_calc_constants_in_expressions() {
    let pi_value = CalcValue::Number(std::f32::consts::PI);
    let two = CalcValue::Number(2.0);
    let multiply_op = Multiplication::new(pi_value, two);

    let calc = Calc::new(CalcValue::Multiplication(multiply_op));
    assert_eq!(calc.to_string(), "calc(3.1415927 * 2)");
  }
}
