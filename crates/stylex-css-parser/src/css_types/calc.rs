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

    /// Simplified parser for calc expressions
    /// This is a basic implementation - the full parser would need to handle:
    /// - Complex operator precedence
    /// - Parenthesized expressions
    /// - Recursive parsing
    /// Mirrors: Calc.parser in calc.js
    pub fn parser() -> TokenParser<Calc> {
        // For now, implement a simplified version that just parses calc(value)
        // TODO: Implement full expression parsing with precedence and grouping

        // Parse calc function start
        let calc_function = TokenParser::<SimpleToken>::token(
            SimpleToken::Function("calc".to_string()),
            Some("Function")
        )
        .where_fn(|token| {
            if let SimpleToken::Function(name) = token {
                name == "calc"
            } else {
                false
            }
        }, Some("calc_function"));

        // Parse closing parenthesis
        let close_paren = TokenParser::<SimpleToken>::token(
            SimpleToken::RightParen,
            Some("RightParen")
        );

        // Use flat_map to chain the parsers properly
        calc_function
            .flat_map(|_| CalcValue::value_parser(), Some("parse_value"))
            .flat_map(move |value| {
                close_paren.map(move |_| Calc::new(value.clone()), Some("to_calc"))
            }, Some("finish_calc"))
    }
}

impl Display for Calc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "calc({})", self.value)
    }
}

/// Helper functions for parsing with proper operator precedence
/// These would be used in the full implementation

/// Compose addition and subtraction operations (lower precedence)
/// Mirrors: composeAddAndSubtraction function in calc.js
pub fn compose_add_and_subtraction(values_and_ops: Vec<CalcValue>) -> CalcValue {
    if values_and_ops.len() == 1 {
        return values_and_ops[0].clone();
    }

    // TODO: Implement proper precedence parsing
    // For now, return the first value as a placeholder
    values_and_ops[0].clone()
}

/// Handle multiplication and division operations (higher precedence)
/// Mirrors: splitByMultiplicationOrDivision function in calc.js
pub fn split_by_multiplication_or_division(values_and_ops: Vec<CalcValue>) -> CalcValue {
    if values_and_ops.len() == 1 {
        return values_and_ops[0].clone();
    }

    // TODO: Implement proper precedence parsing
    // For now, return the first value as a placeholder
    values_and_ops[0].clone()
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
