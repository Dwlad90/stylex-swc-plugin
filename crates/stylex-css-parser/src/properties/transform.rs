/*!
CSS Transform property parsing.

Handles transform property syntax with multiple transform functions.
Currently simplified - will be enhanced when TransformFunction is fully implemented.
Mirrors: packages/style-value-parser/src/properties/transform.js
*/

use crate::token_parser::TokenParser;
use std::fmt::{self, Display};

/// Placeholder for TransformFunction - will be replaced with actual implementation
/// TODO: Replace with proper TransformFunction when css_types/transform_function.rs is implemented
#[derive(Debug, Clone, PartialEq)]
pub struct TransformFunction {
    pub function_name: String,
    pub args: Vec<String>,
}

impl TransformFunction {
    /// Create a placeholder transform function
    pub fn new(function_name: String, args: Vec<String>) -> Self {
        Self { function_name, args }
    }

    /// Placeholder parser - will be replaced with proper implementation
    pub fn parser() -> TokenParser<TransformFunction> {
        // TODO: Implement proper TransformFunction parser
        // For now, return a parser that always fails
        TokenParser::never()
    }
}

impl Display for TransformFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.args.is_empty() {
            write!(f, "{}()", self.function_name)
        } else {
            write!(f, "{}({})", self.function_name, self.args.join(", "))
        }
    }
}

/// CSS transform property value
/// Mirrors: Transform class in transform.js
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub value: Vec<TransformFunction>,
}

impl Transform {
    /// Create a new Transform
    pub fn new(value: Vec<TransformFunction>) -> Self {
        Self { value }
    }

    /// Parser for transform values
    /// Mirrors: Transform.parse in transform.js
    pub fn parser() -> TokenParser<Transform> {
        // TODO: Implement proper parser once TransformFunction is ready
        // For now, simplified implementation

        // This would be: TokenParser.oneOrMore(TransformFunction.parser)
        //   .separatedBy(TokenParser.tokens.Whitespace)
        //   .map((value) => new Transform(value));

        // Placeholder implementation
        TokenParser::never()
    }
}

impl Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function_strings: Vec<String> = self.value
            .iter()
            .map(|func| func.to_string())
            .collect();
        write!(f, "{}", function_strings.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_function_creation() {
        let func = TransformFunction::new(
            "translateX".to_string(),
            vec!["10px".to_string()]
        );

        assert_eq!(func.function_name, "translateX");
        assert_eq!(func.args, vec!["10px"]);
    }

    #[test]
    fn test_transform_function_display() {
        let func = TransformFunction::new(
            "translateX".to_string(),
            vec!["10px".to_string()]
        );
        assert_eq!(func.to_string(), "translateX(10px)");

        let func_multiple_args = TransformFunction::new(
            "translate".to_string(),
            vec!["10px".to_string(), "20px".to_string()]
        );
        assert_eq!(func_multiple_args.to_string(), "translate(10px, 20px)");

        let func_no_args = TransformFunction::new(
            "none".to_string(),
            vec![]
        );
        assert_eq!(func_no_args.to_string(), "none()");
    }

    #[test]
    fn test_transform_creation() {
        let func1 = TransformFunction::new(
            "translateX".to_string(),
            vec!["10px".to_string()]
        );
        let func2 = TransformFunction::new(
            "rotate".to_string(),
            vec!["45deg".to_string()]
        );

        let transform = Transform::new(vec![func1, func2]);
        assert_eq!(transform.value.len(), 2);
    }

    #[test]
    fn test_transform_display() {
        let func1 = TransformFunction::new(
            "translateX".to_string(),
            vec!["10px".to_string()]
        );
        let func2 = TransformFunction::new(
            "rotate".to_string(),
            vec!["45deg".to_string()]
        );

        let transform = Transform::new(vec![func1, func2]);
        assert_eq!(transform.to_string(), "translateX(10px) rotate(45deg)");
    }

    #[test]
    fn test_transform_single_function() {
        let func = TransformFunction::new(
            "scale".to_string(),
            vec!["1.5".to_string()]
        );

        let transform = Transform::new(vec![func]);
        assert_eq!(transform.to_string(), "scale(1.5)");
    }

    #[test]
    fn test_transform_empty() {
        let transform = Transform::new(vec![]);
        assert_eq!(transform.to_string(), "");
    }

    #[test]
    fn test_transform_parser_creation() {
        // Basic test that parsers can be created (even if they're placeholders)
        let _func_parser = TransformFunction::parser();
        let _transform_parser = Transform::parser();
    }

    #[test]
    fn test_transform_equality() {
        let func1 = TransformFunction::new(
            "translateX".to_string(),
            vec!["10px".to_string()]
        );
        let func2 = TransformFunction::new(
            "translateX".to_string(),
            vec!["10px".to_string()]
        );
        let func3 = TransformFunction::new(
            "translateY".to_string(),
            vec!["10px".to_string()]
        );

        let transform1 = Transform::new(vec![func1]);
        let transform2 = Transform::new(vec![func2]);
        let transform3 = Transform::new(vec![func3]);

        assert_eq!(transform1, transform2);
        assert_ne!(transform1, transform3);
    }

    #[test]
    fn test_transform_common_functions() {
        // Test common transform function patterns
        let translate = TransformFunction::new(
            "translate".to_string(),
            vec!["50px".to_string(), "100px".to_string()]
        );
        assert_eq!(translate.to_string(), "translate(50px, 100px)");

        let rotate = TransformFunction::new(
            "rotate".to_string(),
            vec!["90deg".to_string()]
        );
        assert_eq!(rotate.to_string(), "rotate(90deg)");

        let scale = TransformFunction::new(
            "scale".to_string(),
            vec!["1.5".to_string(), "0.8".to_string()]
        );
        assert_eq!(scale.to_string(), "scale(1.5, 0.8)");

        let skew = TransformFunction::new(
            "skewX".to_string(),
            vec!["20deg".to_string()]
        );
        assert_eq!(skew.to_string(), "skewX(20deg)");
    }

    #[test]
    fn test_transform_complex_combination() {
        // Test a complex transform with multiple functions
        let funcs = vec![
            TransformFunction::new("translate".to_string(), vec!["50%".to_string(), "-50%".to_string()]),
            TransformFunction::new("rotate".to_string(), vec!["45deg".to_string()]),
            TransformFunction::new("scale".to_string(), vec!["1.2".to_string()]),
        ];

        let transform = Transform::new(funcs);
        assert_eq!(transform.to_string(), "translate(50%, -50%) rotate(45deg) scale(1.2)");
    }
}
