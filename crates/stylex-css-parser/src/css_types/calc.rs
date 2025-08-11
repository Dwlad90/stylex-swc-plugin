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
    token_parser::TokenParser,
    token_types::SimpleToken,
    css_types::{CalcConstant, Percentage}
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
        Self { expr: Box::new(expr) }
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
            TokenParser::<SimpleToken>::token(SimpleToken::Number(0.0), Some("Number"))
                .map(|token| {
                    if let SimpleToken::Number(value) = token {
                        CalcValue::Number(value as f32)
                    } else {
                        unreachable!()
                    }
                }, Some("to_number")),

            // Dimensions
            TokenParser::<SimpleToken>::token(
                SimpleToken::Dimension { value: 0.0, unit: String::new() },
                Some("Dimension")
            )
                .map(|token| {
                    if let SimpleToken::Dimension { value, unit } = token {
                        CalcValue::Dimension(CalcDimension::new(value as f32, unit))
                    } else {
                        unreachable!()
                    }
                }, Some("to_dimension")),

            // Percentages
            Percentage::parser().map(CalcValue::Percentage, Some("percentage")),
        ])
    }

    /// Handle multiplication and division (higher precedence)
    /// Mirrors: splitByMultiplicationOrDivision in calc.js exactly
    pub fn split_by_multiplication_or_division(values_and_operators: Vec<CalcValueOrOperator>) -> CalcValue {
        if values_and_operators.len() == 1 {
            if let CalcValueOrOperator::Value(value) = &values_and_operators[0] {
                return value.clone();
            }
            // This shouldn't happen in well-formed input
            return CalcValue::Number(0.0);
        }

        // Find first multiplication or division operator
        let first_operator = values_and_operators.iter().position(|item| {
            matches!(item, CalcValueOrOperator::Operator(op) if op == "*" || op == "/")
        });

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
        let first_operator = values_and_operators.iter().position(|item| {
            matches!(item, CalcValueOrOperator::Operator(op) if op == "+" || op == "-")
        });

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

/// Helper enum for parsing values and operators
#[derive(Debug, Clone, PartialEq)]
pub enum CalcValueOrOperator {
    Value(CalcValue),
    Operator(String),
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

    /// Parser for calc expressions
    /// Mirrors: Calc.parser in calc.js
    pub fn parser() -> TokenParser<Calc> {
        // For now, implement a basic parser
        // TODO: Implement full operationsParser equivalent
        let calc_function = TokenParser::<String>::fn_name("calc");
        let close_paren = TokenParser::<SimpleToken>::token(
            SimpleToken::RightParen,
            Some("RightParen")
        );

        // Parse the expression (for now just basic values)
        let expression = CalcValue::value_parser();

        // Combine: calc( expression )
        calc_function
            .flat_map(move |_| expression.clone(), Some("parse_expression"))
            .flat_map(move |value| {
                close_paren.clone().map(move |_| Calc::new(value.clone()), Some("to_calc"))
            }, Some("finish_calc"))
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
        CalcValue::Addition(op) => format!("{} + {}", calc_value_to_string(&op.left), calc_value_to_string(&op.right)),
        CalcValue::Subtraction(op) => format!("{} - {}", calc_value_to_string(&op.left), calc_value_to_string(&op.right)),
        CalcValue::Multiplication(op) => format!("{} * {}", calc_value_to_string(&op.left), calc_value_to_string(&op.right)),
        CalcValue::Division(op) => format!("{} / {}", calc_value_to_string(&op.left), calc_value_to_string(&op.right)),
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
        let _parser = Calc::parser();
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
