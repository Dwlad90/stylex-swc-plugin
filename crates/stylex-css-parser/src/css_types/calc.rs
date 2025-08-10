/*!
CSS Calc expression parsing.

Handles calc() expressions with mathematical operations, proper precedence, and grouping.
Mirrors: packages/style-value-parser/src/css-types/calc.js
*/

use crate::{
    token_parser::TokenParser,
    token_types::SimpleToken,
    css_types::{CalcConstant, Percentage}
};
use std::fmt::{self, Display};

/// Binary operation types
/// Mirrors: Addition, Subtraction, Multiplication, Division types in calc.js
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinaryOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
        }
    }

    pub fn from_str(s: &str) -> Option<BinaryOp> {
        match s {
            "+" => Some(BinaryOp::Add),
            "-" => Some(BinaryOp::Subtract),
            "*" => Some(BinaryOp::Multiply),
            "/" => Some(BinaryOp::Divide),
            _ => None,
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

/// Binary operation with left and right operands
/// Mirrors: Addition, Subtraction, Multiplication, Division in calc.js
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperation {
    pub op: BinaryOp,
    pub left: Box<CalcValue>,
    pub right: Box<CalcValue>,
}

impl BinaryOperation {
    pub fn new(op: BinaryOp, left: CalcValue, right: CalcValue) -> Self {
        Self {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

/// Grouped expression (parenthesized)
/// Mirrors: Group type in calc.js
#[derive(Debug, Clone, PartialEq)]
pub struct CalcGroup {
    pub expr: Box<CalcValue>,
}

impl CalcGroup {
    pub fn new(expr: CalcValue) -> Self {
        Self { expr: Box::new(expr) }
    }
}

/// A value in a calc expression
/// Mirrors: CalcValue type in calc.js
#[derive(Debug, Clone, PartialEq)]
pub enum CalcValue {
    Number(f32),
    Dimension(CalcDimension),
    Percentage(Percentage),
    Constant(CalcConstant),
    BinaryOp(BinaryOperation),
    Group(CalcGroup),
}

/// Helper enum for operator parsing
#[derive(Debug, Clone, PartialEq)]
pub enum CalcToken {
    Value(CalcValue),
    Operator(BinaryOp),
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

    /// Parser for operators (+, -, *, /)
    pub fn operator_parser() -> TokenParser<BinaryOp> {
        TokenParser::<SimpleToken>::token(SimpleToken::Unknown(String::new()), Some("Delim"))
            .map(|token| {
                if let SimpleToken::Unknown(delim) = token {
                    delim
                } else {
                    String::new()
                }
            }, Some("extract_delim"))
            .where_fn(|delim| {
                delim == "+" || delim == "-" || delim == "*" || delim == "/"
            }, Some("valid_operator"))
            .map(|delim| {
                BinaryOp::from_str(&delim).unwrap()
            }, Some("to_binary_op"))
    }

    /// Expressions parser with proper precedence
    /// Mirrors: operationsParser in calc.js
    pub fn expressions_parser() -> TokenParser<CalcValue> {
        // Forward declaration - we'll implement this recursively
        Self::expressions_parser_impl()
    }

    /// Implementation of expressions parser
    fn expressions_parser_impl() -> TokenParser<CalcValue> {
        // Parse parenthesized expressions
        let parenthesized = TokenParser::<SimpleToken>::token(SimpleToken::LeftParen, Some("LeftParen"))
            .flat_map(|_| {
                // Skip optional whitespace, parse expression, skip optional whitespace, then close paren
                Self::expressions_parser()
                    .flat_map(|expr| {
                        TokenParser::<SimpleToken>::token(SimpleToken::RightParen, Some("RightParen"))
                            .map(move |_| CalcValue::Group(CalcGroup::new(expr.clone())), Some("to_group"))
                    }, Some("close_paren"))
            }, Some("parenthesized"));

        // Primary values (either basic value or parenthesized expression)
        let primary = TokenParser::one_of(vec![
            Self::value_parser(),
            parenthesized,
        ]);

        // Parse a sequence of values and operators
        let first_value = primary.clone();
        let rest_values = TokenParser::zero_or_more(
            TokenParser::<CalcToken>::sequence(vec![
                Self::operator_parser().map(CalcToken::Operator, Some("op")),
                primary.clone().map(CalcToken::Value, Some("val")),
            ])
        );

        // Combine first value with remaining values and operators
        first_value
            .flat_map(move |first| {
                let first_clone = first.clone();
                rest_values.clone().map(move |rest| {
                    let mut tokens = vec![CalcToken::Value(first_clone.clone())];
                    for pair in rest {
                        tokens.extend(pair);
                    }
                    Self::parse_with_precedence(tokens)
                }, Some("combine"))
            }, Some("parse_sequence"))
    }

    /// Parse tokens with proper operator precedence
    /// Implements the precedence algorithm from JavaScript
    fn parse_with_precedence(tokens: Vec<CalcToken>) -> CalcValue {
        if tokens.len() == 1 {
            if let CalcToken::Value(val) = &tokens[0] {
                return val.clone();
            }
        }

        // Convert tokens to values and operators for precedence parsing
        let mut values_and_ops = Vec::new();
        for token in tokens {
            match token {
                CalcToken::Value(val) => values_and_ops.push(val),
                CalcToken::Operator(op) => {
                    // Create a placeholder for operator - we'll handle this in precedence functions
                    values_and_ops.push(CalcValue::Number(match op {
                        BinaryOp::Add => -1.0,
                        BinaryOp::Subtract => -2.0,
                        BinaryOp::Multiply => -3.0,
                        BinaryOp::Divide => -4.0,
                    }));
                }
            }
        }

        Self::split_by_multiplication_or_division(values_and_ops)
    }

    /// Handle multiplication and division (higher precedence)
    /// Mirrors: splitByMultiplicationOrDivision in calc.js
    fn split_by_multiplication_or_division(values_and_ops: Vec<CalcValue>) -> CalcValue {
        if values_and_ops.len() == 1 {
            return values_and_ops[0].clone();
        }

        // Find first multiplication or division operator
        let mut first_mul_div = None;
        for (i, value) in values_and_ops.iter().enumerate() {
            if let CalcValue::Number(n) = value {
                if *n == -3.0 || *n == -4.0 { // * or /
                    first_mul_div = Some(i);
                    break;
                }
            }
        }

        match first_mul_div {
            None => {
                // No multiplication or division, handle addition/subtraction
                Self::compose_add_and_subtraction(values_and_ops)
            }
            Some(op_idx) => {
                // Split around the operator
                let left_values = values_and_ops[..op_idx].to_vec();
                let right_values = values_and_ops[op_idx + 1..].to_vec();
                let op = &values_and_ops[op_idx];

                let left_result = Self::compose_add_and_subtraction(left_values);
                let right_result = Self::split_by_multiplication_or_division(right_values);

                let binary_op = if let CalcValue::Number(n) = op {
                    if *n == -3.0 { BinaryOp::Multiply } else { BinaryOp::Divide }
                } else {
                    BinaryOp::Multiply // fallback
                };

                CalcValue::BinaryOp(BinaryOperation::new(binary_op, left_result, right_result))
            }
        }
    }

    /// Handle addition and subtraction (lower precedence)
    /// Mirrors: composeAddAndSubtraction in calc.js
    fn compose_add_and_subtraction(values_and_ops: Vec<CalcValue>) -> CalcValue {
        if values_and_ops.len() == 1 {
            return values_and_ops[0].clone();
        }

        // Find first addition or subtraction operator
        let mut first_add_sub = None;
        for (i, value) in values_and_ops.iter().enumerate() {
            if let CalcValue::Number(n) = value {
                if *n == -1.0 || *n == -2.0 { // + or -
                    first_add_sub = Some(i);
                    break;
                }
            }
        }

        match first_add_sub {
            None => {
                // No operators, return first value
                values_and_ops[0].clone()
            }
            Some(op_idx) => {
                // Split around the operator
                let left_values = values_and_ops[..op_idx].to_vec();
                let right_values = values_and_ops[op_idx + 1..].to_vec();
                let op = &values_and_ops[op_idx];

                let left_result = Self::compose_add_and_subtraction(left_values);
                let right_result = Self::compose_add_and_subtraction(right_values);

                let binary_op = if let CalcValue::Number(n) = op {
                    if *n == -1.0 { BinaryOp::Add } else { BinaryOp::Subtract }
                } else {
                    BinaryOp::Add // fallback
                };

                CalcValue::BinaryOp(BinaryOperation::new(binary_op, left_result, right_result))
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
            CalcValue::BinaryOp(op) => write!(f, "{} {} {}", op.left, op.op.as_str(), op.right),
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
    pub fn new(value: CalcValue) -> Self {
        Self { value }
    }

    /// Parser for calc expressions with full precedence support
    /// Mirrors: Calc.parser in calc.js
    pub fn parser() -> TokenParser<Calc> {
        // Parse calc function start
        let calc_function = TokenParser::<String>::fn_name("calc");

        // Parse closing parenthesis
        let close_paren = TokenParser::<SimpleToken>::token(
            SimpleToken::RightParen,
            Some("RightParen")
        );

        // Parse the full expression or simple value
        let expression_or_value = TokenParser::one_of(vec![
            CalcValue::expressions_parser(),
            CalcValue::value_parser(),
        ]);

        // Combine: calc( expression )
        calc_function
            .flat_map(move |_| expression_or_value.clone(), Some("parse_expression"))
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_op_from_str() {
        assert_eq!(BinaryOp::from_str("+"), Some(BinaryOp::Add));
        assert_eq!(BinaryOp::from_str("-"), Some(BinaryOp::Subtract));
        assert_eq!(BinaryOp::from_str("*"), Some(BinaryOp::Multiply));
        assert_eq!(BinaryOp::from_str("/"), Some(BinaryOp::Divide));
        assert_eq!(BinaryOp::from_str("invalid"), None);
    }

    #[test]
    fn test_binary_op_as_str() {
        assert_eq!(BinaryOp::Add.as_str(), "+");
        assert_eq!(BinaryOp::Subtract.as_str(), "-");
        assert_eq!(BinaryOp::Multiply.as_str(), "*");
        assert_eq!(BinaryOp::Divide.as_str(), "/");
    }

    #[test]
    fn test_calc_dimension_creation() {
        let dim = CalcDimension::new(16.0, "px".to_string());
        assert_eq!(dim.value, 16.0);
        assert_eq!(dim.unit, "px");
        assert_eq!(dim.to_string(), "16px");
    }

    #[test]
    fn test_calc_value_display() {
        let number = CalcValue::Number(42.0);
        assert_eq!(number.to_string(), "42");

        let dimension = CalcValue::Dimension(CalcDimension::new(16.0, "px".to_string()));
        assert_eq!(dimension.to_string(), "16px");

        let percentage = CalcValue::Percentage(Percentage::new(50.0));
        assert_eq!(percentage.to_string(), "50%");

        let constant = CalcValue::Constant(CalcConstant::Pi);
        assert_eq!(constant.to_string(), "pi");
    }

    #[test]
    fn test_binary_operation() {
        let left = CalcValue::Number(10.0);
        let right = CalcValue::Number(5.0);
        let add_op = BinaryOperation::new(BinaryOp::Add, left, right);

        let calc_value = CalcValue::BinaryOp(add_op);
        assert_eq!(calc_value.to_string(), "10 + 5");
    }

    #[test]
    fn test_calc_group() {
        let inner = CalcValue::Number(42.0);
        let group = CalcGroup::new(inner);
        let calc_value = CalcValue::Group(group);

        assert_eq!(calc_value.to_string(), "(42)");
    }

    #[test]
    fn test_calc_creation() {
        let value = CalcValue::Number(42.0);
        let calc = Calc::new(value);
        assert_eq!(calc.to_string(), "calc(42)");
    }

    #[test]
    fn test_calc_value_parser_creation() {
        // Basic test that parser can be created
        let _parser = CalcValue::value_parser();
    }

    #[test]
    fn test_calc_parser_creation() {
        // Basic test that parser can be created
        let _parser = Calc::parser();
    }

    #[test]
    fn test_complex_calc_expression() {
        // Test a more complex nested expression
        let left = CalcValue::Number(10.0);
        let right = CalcValue::Dimension(CalcDimension::new(20.0, "px".to_string()));
        let add_op = BinaryOperation::new(BinaryOp::Add, left, right);

        let inner_group = CalcGroup::new(CalcValue::BinaryOp(add_op));
        let multiply_right = CalcValue::Number(2.0);
        let multiply_op = BinaryOperation::new(
            BinaryOp::Multiply,
            CalcValue::Group(inner_group),
            multiply_right
        );

        let calc = Calc::new(CalcValue::BinaryOp(multiply_op));
        assert_eq!(calc.to_string(), "calc((10 + 20px) * 2)");
    }

    #[test]
    fn test_calc_value_equality() {
        let val1 = CalcValue::Number(42.0);
        let val2 = CalcValue::Number(42.0);
        let val3 = CalcValue::Number(24.0);

        assert_eq!(val1, val2);
        assert_ne!(val1, val3);
    }

    #[test]
    fn test_calc_constants_in_expressions() {
        // Test calc constants in expressions
        let pi_value = CalcValue::Constant(CalcConstant::Pi);
        let two = CalcValue::Number(2.0);
        let multiply_op = BinaryOperation::new(BinaryOp::Multiply, pi_value, two);

        let calc = Calc::new(CalcValue::BinaryOp(multiply_op));
        assert_eq!(calc.to_string(), "calc(pi * 2)");
    }
}
