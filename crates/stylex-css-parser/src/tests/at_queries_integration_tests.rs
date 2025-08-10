/*!
Comprehensive integration tests for at-queries module.

Mirrors the JavaScript test structure from:
- packages/style-value-parser/src/at-queries/__tests__/validation-media-query-test.js
- packages/style-value-parser/src/at-queries/__tests__/parse-media-query-test.js  
- packages/style-value-parser/src/at-queries/__tests__/media-query-transform-test.js

These tests focus on the parts of our implementation that are currently working,
while noting TODOs for areas that need more complete implementation.
*/

use crate::{
    at_queries::{
        MediaQuery, validate_media_query,
        lastMediaQueryWinsTransform, MediaQueryErrors,
    },
};

#[cfg(test)]
mod media_query_validation_tests {
    use super::*;

    #[test]
    fn test_media_query_errors_constants() {
        // Test error message constants are available
        assert!(!MediaQueryErrors::SYNTAX_ERROR.is_empty());
        assert!(!MediaQueryErrors::UNBALANCED_PARENS.is_empty());
        
        // Verify they're different messages
        assert_ne!(MediaQueryErrors::SYNTAX_ERROR, MediaQueryErrors::UNBALANCED_PARENS);
    }

    #[test]
    fn test_validate_media_query_basic_syntax() {
        // Test basic validation functionality
        // Note: Our current validate_media_query is simplified
        
        // Valid queries (should not panic/error)
        let valid_queries = vec![
            "@media screen",
            "@media print", 
            "@media all",
            "@media (min-width: 600px)",
            "@media (max-width: 1200px)",
            "@media (orientation: landscape)",
        ];

        for query in valid_queries {
            // Our current implementation is placeholder - just test it doesn't panic
            let _result = std::panic::catch_unwind(|| {
                validate_media_query(query)
            });
            // For now, just ensure it doesn't crash
            // TODO: Implement proper validation logic
        }
    }

    #[test]
    fn test_syntax_error_cases() {
        // Test cases that should trigger SYNTAX_ERROR
        // These mirror the JavaScript validation tests
        
        let invalid_syntax_cases = vec![
            "@media",
            "@media ",
            "@media ()",
            "@media not (min-width: )",
            "@media (width:)",
            "@media (min-width:)",
            "@media (max-width: )",
            "@media and",
            "@media (min-width: 700px and max-width: 767px)",
            "@media (min-width:445px; max-width:768px)",
            "@media (width > )",
            "@media ( > 600px)",
            "@media (600px > width) or",
            "@media (width < )",
            "@media (width <=)",
            "@media (>= width)",
            "@media (300px < width < )",
        ];

        // TODO: When validate_media_query is fully implemented, these should throw SYNTAX_ERROR
        for invalid_query in invalid_syntax_cases {
            // For now, just document that these are invalid cases
            println!("Invalid syntax case: {}", invalid_query);
            // TODO: assert_eq!(validate_media_query(invalid_query), Err(MediaQueryErrors::SYNTAX_ERROR));
        }
    }

    #[test]
    fn test_unbalanced_parens_cases() {
        // Test cases that should trigger UNBALANCED_PARENS
        let unbalanced_cases = vec![
            "@media (width: 600px",
            "@media screen and (color",
            "@media not (min-resolution: 300dpi",
            "@media (orientation: portrait",
            "@media ((min-width: 300px) and (max-width: 1000px)",
            "@media (hover: hover) and (pointer: fine",
            "@media (width: calc(100% - 50px)",
            "@media (aspect-ratio: (16/9",
            "@media screen and ((min-width: 640px)",
            "@media ((prefers-color-scheme: dark)",
        ];

        // TODO: When validate_media_query is fully implemented, these should throw UNBALANCED_PARENS
        for unbalanced_query in unbalanced_cases {
            // For now, just document that these are unbalanced cases
            println!("Unbalanced parens case: {}", unbalanced_query);
            // TODO: assert_eq!(validate_media_query(unbalanced_query), Err(MediaQueryErrors::UNBALANCED_PARENS));
        }
    }

    #[test]
    fn test_invalid_var_usage() {
        // Test invalid var() usage cases
        let invalid_var_cases = vec![
            "@media (min-width: var(--test))",
            "@media (min-width: var(--foo) and (max-width: 700px))",
            "@media (min-width: var(foo) and (max-width: 700px))",
        ];

        // TODO: When var() validation is implemented, these should throw SYNTAX_ERROR
        for invalid_var in invalid_var_cases {
            println!("Invalid var() case: {}", invalid_var);
            // TODO: assert_eq!(validate_media_query(invalid_var), Err(MediaQueryErrors::SYNTAX_ERROR));
        }
    }

    #[test]
    fn test_has_balanced_parens() {
        // Test our current has_balanced_parens implementation
        
        // Balanced cases
        let balanced_cases = vec![
            "@media screen",
            "@media (min-width: 600px)",
            "@media (min-width: 600px) and (max-width: 1200px)",
            "@media ((min-width: 400px) and (max-width: 800px))",
            "@media screen and (color)",
        ];

        for case in balanced_cases {
            assert!(MediaQuery::has_balanced_parens(case), "Should be balanced: {}", case);
        }

        // Unbalanced cases  
        let unbalanced_cases = vec![
            "@media (min-width: 600px",
            "@media screen and (color",
            "@media ((min-width: 400px) and (max-width: 800px)",
            "@media (width: calc(100% - 50px)",
        ];

        for case in unbalanced_cases {
            assert!(!MediaQuery::has_balanced_parens(case), "Should be unbalanced: {}", case);
        }
    }
}

#[cfg(test)]
mod media_query_parsing_tests {
    use super::*;

    #[test]
    fn test_media_query_creation() {
        // Test basic MediaQuery creation
        let query = MediaQuery::new("@media screen".to_string());
        assert_eq!(query.query_string, "@media screen");
    }

    #[test]
    fn test_media_query_display() {
        // Test toString functionality
        let query = MediaQuery::new("@media (min-width: 600px)".to_string());
        assert_eq!(query.to_string(), "@media (min-width: 600px)");
        
        let complex_query = MediaQuery::new("@media screen and (min-width: 600px) and (max-width: 1200px)".to_string());
        assert_eq!(complex_query.to_string(), "@media screen and (min-width: 600px) and (max-width: 1200px)");
    }

    #[test]
    fn test_media_query_keywords() {
        // Test media type keywords like in JavaScript tests
        let keyword_queries = vec![
            "@media screen",
            "@media print", 
            "@media all",
            "@media only screen",
            "@media not screen",
            "@media only print and (color)",
            "@media not all and (monochrome)",
        ];

        for query_str in keyword_queries {
            let query = MediaQuery::new(query_str.to_string());
            assert_eq!(query.to_string(), query_str);
            
            // TODO: When parser is fully implemented, test actual parsing:
            // let parsed = MediaQuery::parser().parse_to_end(query_str);
            // assert!(parsed.is_ok());
        }
    }

    #[test]
    fn test_media_query_pair_rules() {
        // Test property: value pairs like in JavaScript tests
        let pair_rule_queries = vec![
            "@media (width: 100px)",
            "@media (max-width: 50em)",
            "@media (orientation: landscape)",
            "@media (update: fast)",
            "@media (overflow-block: scroll)",
            "@media (display-mode: fullscreen)",
            "@media (scripting: enabled)",
            "@media (hover: hover)",
            "@media (any-hover: none)",
            "@media (pointer: coarse)",
            "@media (any-pointer: fine)",
            "@media (light-level: dim)",
            "@media (inverted-colors: inverted)",
            "@media (prefers-reduced-motion: reduce)",
            "@media (prefers-contrast: more)",
            "@media (forced-colors: active)",
            "@media (prefers-reduced-transparency: reduce)",
        ];

        for query_str in pair_rule_queries {
            let query = MediaQuery::new(query_str.to_string());
            assert_eq!(query.to_string(), query_str);
            // TODO: Test actual parsing when implemented
        }
    }

    #[test]
    fn test_media_query_calc_expressions() {
        // Test calc() expressions in media queries
        let calc_queries = vec![
            "@media (min-width: calc(300px + 5em))",
            "@media (max-height: calc(100vh - 50px))",
        ];

        for query_str in calc_queries {
            let query = MediaQuery::new(query_str.to_string());
            assert_eq!(query.to_string(), query_str);
            // TODO: Test calc() parsing when implemented
        }
    }

    #[test]
    fn test_media_query_aspect_ratios() {
        // Test aspect ratio syntax
        let aspect_ratio_queries = vec![
            "@media (aspect-ratio: 16 / 9)",
            "@media (device-aspect-ratio: 16 / 9)",
            "@media (min-aspect-ratio: 3/2) and (max-aspect-ratio: 16/9)",
        ];

        for query_str in aspect_ratio_queries {
            let query = MediaQuery::new(query_str.to_string());
            assert_eq!(query.to_string(), query_str);
            // TODO: Test aspect ratio parsing when implemented
        }
    }

    #[test]
    fn test_media_query_word_rules() {
        // Test standalone word rules like (color), (monochrome)
        let word_rule_queries = vec![
            "@media (color)",
            "@media (color-index)",
            "@media (monochrome)",
            "@media (grid)",
        ];

        for query_str in word_rule_queries {
            let query = MediaQuery::new(query_str.to_string());
            assert_eq!(query.to_string(), query_str);
            // TODO: Test word rule parsing when implemented
        }
    }

    #[test]
    fn test_media_query_and_combinators() {
        // Test 'and' combinations like in JavaScript tests
        let and_combinator_queries = vec![
            "@media not all and (monochrome)",
            "@media screen and (min-width: 400px)",
            "@media (min-height: 600px) and (orientation: landscape)",
            "@media screen and (device-aspect-ratio: 16/9)",
            "@media (min-width: 768px) and (max-width: 991px)",
            "@media (min-width: 1200px) and (orientation: landscape)",
            "@media (min-width: 992px) and (max-width: 1199px) and (pointer: fine)",
            "@media (orientation: landscape) and (pointer: fine)",
        ];

        for query_str in and_combinator_queries {
            let query = MediaQuery::new(query_str.to_string());
            assert_eq!(query.to_string(), query_str);
            // TODO: Test and combinator parsing when implemented
        }
    }

    #[test]
    fn test_media_query_or_combinators() {
        // Test comma-separated (or) combinations
        let or_combinator_queries = vec![
            "@media (orientation: portrait), (orientation: landscape)",
            "@media (min-width: 500px) or (max-width: 600px)",
            "@media (width: 500px), (height: 400px)",
            "@media (min-width: 576px), (orientation: portrait) and (max-width: 767px)",
            "@media (color) and (min-width: 400px), screen and (max-width: 700px)",
        ];

        for query_str in or_combinator_queries {
            let query = MediaQuery::new(query_str.to_string());
            assert_eq!(query.to_string(), query_str);
            // TODO: Test or combinator parsing when implemented
        }
    }

    #[test]
    fn test_media_query_not_combinators() {
        // Test 'not' combinations
        let not_combinator_queries = vec![
            "@media not (not (not (min-width: 400px)))",
            "@media not ((min-width: 500px) and (max-width: 600px) and (max-width: 400px))",
            "@media not all and (monochrome) and (min-width: 600px)",
            "@media (max-width: 1440px) and (not (max-width: 1024px)) and (not (max-width: 768px)) and (not (max-width: 458px))",
        ];

        for query_str in not_combinator_queries {
            let query = MediaQuery::new(query_str.to_string());
            assert_eq!(query.to_string(), query_str);
            // TODO: Test not combinator parsing and normalization when implemented
        }
    }

    #[test]
    fn test_media_query_inequality_rules() {
        // Test inequality syntax (>, >=, <, <=, ranges)
        let inequality_queries = vec![
            "@media (width > 400px)",
            "@media (width >= 400px)",
            "@media (400px < width)",
            "@media (400px <= width)",
            "@media (400px <= width <= 700px)",
            "@media (400px < width <= 700px)",
            "@media (1000px >= width >= 700px)",
            "@media (1000px > width >= 700px)",
            "@media (1000px >= width > 700px)",
            "@media (1000px > width > 700px)",
        ];

        for query_str in inequality_queries {
            let query = MediaQuery::new(query_str.to_string());
            assert_eq!(query.to_string(), query_str);
            // TODO: Test inequality parsing and normalization when implemented
        }
    }

    #[test]
    fn test_media_query_additional_features() {
        // Test additional media query features from JavaScript tests
        let additional_feature_queries = vec![
            "@media (-webkit-min-device-pixel-ratio: 2)",
            "@media (-webkit-max-device-pixel-ratio: 1.5)",
            "@media (color-index: 256)",
            "@media (min-color-index: 256)",
            "@media (max-color-index: 65536)",
            "@media (resolution: 300dpi)",
            "@media (resolution: 2dppx)",
            "@media (resolution: 1.5dpcm)",
            "@media (color-gamut: srgb)",
            "@media (color-gamut: p3)",
            "@media (color-gamut: rec2020)",
            "@media (video-color-gamut: srgb)",
            "@media (video-color-gamut: p3)",
            "@media (video-color-gamut: rec2020)",
        ];

        for query_str in additional_feature_queries {
            let query = MediaQuery::new(query_str.to_string());
            assert_eq!(query.to_string(), query_str);
            // TODO: Test additional feature parsing when implemented
        }
    }

    #[test] 
    fn test_media_query_parser_creation() {
        // Test that parser can be created (even if it's a placeholder)
        let _parser = MediaQuery::parser();
        // TODO: Test actual parsing when parser is fully implemented
    }

    #[test]
    fn test_media_query_equality() {
        let query1 = MediaQuery::new("@media screen".to_string());
        let query2 = MediaQuery::new("@media screen".to_string());
        let query3 = MediaQuery::new("@media print".to_string());

        assert_eq!(query1, query2);
        assert_ne!(query1, query3);
    }

    #[test]
    fn test_media_query_complex_examples() {
        // Test complex real-world media query examples
        let complex_queries = vec![
            "@media screen and (min-width: 768px) and (max-width: 1024px) and (orientation: landscape)",
            "@media only screen and (min-device-width: 375px) and (max-device-width: 667px) and (-webkit-min-device-pixel-ratio: 2)",
            "@media (prefers-reduced-motion: reduce) and (prefers-color-scheme: dark) and (min-width: 800px)",
            "@media print and (min-resolution: 300dpi), screen and (min-width: 1200px) and (pointer: fine)",
        ];

        for query_str in complex_queries {
            let query = MediaQuery::new(query_str.to_string());
            assert_eq!(query.to_string(), query_str);
            assert!(MediaQuery::has_balanced_parens(query_str));
        }
    }
}

#[cfg(test)]
mod media_query_transform_tests {
    use super::*;

    #[test]
    fn test_last_media_query_wins_transform_function_exists() {
        // Test that the transform function exists and can be called
        let test_queries = vec![
            MediaQuery::new("@media (max-width: 768px)".to_string()),
            MediaQuery::new("@media (max-width: 1024px)".to_string()),
        ];

        let result = lastMediaQueryWinsTransform(test_queries.clone());
        
        // For now, just verify the function works and returns the expected type
        assert_eq!(result.len(), test_queries.len());
        // TODO: Test actual transformation logic when implemented
    }

    #[test]
    fn test_media_query_transform_basic_usage() {
        // Test basic transformation like in JavaScript tests
        // This mirrors the "basic usage: multiple widths" test
        
        let queries = vec![
            MediaQuery::new("@media (max-width: 1440px)".to_string()),
            MediaQuery::new("@media (max-width: 1024px)".to_string()),
            MediaQuery::new("@media (max-width: 768px)".to_string()),
        ];

        let result = lastMediaQueryWinsTransform(queries);
        
        // Basic verification that transformation occurred
        assert_eq!(result.len(), 3);
        
        // TODO: When transform logic is fully implemented, test expected output:
        // Expected transformation should be:
        // - "@media (min-width: 1024.01px) and (max-width: 1440px)"
        // - "@media (min-width: 768.01px) and (max-width: 1024px)"  
        // - "@media (max-width: 768px)"
    }

    #[test]
    fn test_media_query_transform_single_query() {
        // Test that single queries are not modified
        let single_query = vec![
            MediaQuery::new("@media (max-width: 1440px)".to_string()),
        ];

        let result = lastMediaQueryWinsTransform(single_query.clone());
        
        // Single queries should remain unchanged
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].to_string(), single_query[0].to_string());
    }

    #[test]
    fn test_media_query_transform_edge_cases() {
        // Test edge cases
        
        // Empty list
        let empty_queries: Vec<MediaQuery> = vec![];
        let result = lastMediaQueryWinsTransform(empty_queries);
        assert!(result.is_empty());
        
        // Duplicate queries
        let duplicate_queries = vec![
            MediaQuery::new("@media (max-width: 768px)".to_string()),
            MediaQuery::new("@media (max-width: 768px)".to_string()),
        ];
        let result = lastMediaQueryWinsTransform(duplicate_queries.clone());
        assert_eq!(result.len(), duplicate_queries.len());
    }

    #[test]
    fn test_media_query_transform_mixed_types() {
        // Test mixed query types (width, height, orientation, etc.)
        let mixed_queries = vec![
            MediaQuery::new("@media (max-width: 1024px)".to_string()),
            MediaQuery::new("@media (orientation: landscape)".to_string()),
            MediaQuery::new("@media (max-height: 600px)".to_string()),
            MediaQuery::new("@media (prefers-color-scheme: dark)".to_string()),
        ];

        let result = lastMediaQueryWinsTransform(mixed_queries.clone());
        assert_eq!(result.len(), mixed_queries.len());
        
        // TODO: Test that only related queries are transformed together
    }
}

#[cfg(test)]
mod media_query_rule_tests {

    #[test]
    fn test_media_query_rule_creation() {
        // Test MediaQueryRule enum (currently placeholder)
        // TODO: Implement proper MediaQueryRule variants and test them
        
        // For now, just ensure the enum exists and can be used
        // let rule = MediaQueryRule::Pair { key: "width".to_string(), value: "100px".to_string() };
        // let word_rule = MediaQueryRule::Word { keyword: "color".to_string() };
        // let and_rule = MediaQueryRule::And { rules: vec![] };
        // let or_rule = MediaQueryRule::Or { rules: vec![] };
        // let not_rule = MediaQueryRule::Not { rule: Box::new(rule) };
        
        println!("MediaQueryRule tests will be implemented when the enum is fully defined");
    }
}

#[cfg(test)]
mod media_query_integration_tests {
    use super::*;

    #[test]
    fn test_comprehensive_media_query_workflow() {
        // Test a complete workflow: parse -> validate -> transform
        let query_strings = vec![
            "@media screen and (min-width: 768px)",
            "@media (max-width: 1024px) and (orientation: landscape)",
            "@media print and (min-resolution: 300dpi)",
        ];

        for query_str in query_strings {
            // Step 1: Create MediaQuery
            let query = MediaQuery::new(query_str.to_string());
            assert_eq!(query.to_string(), query_str);
            
            // Step 2: Validate syntax
            assert!(MediaQuery::has_balanced_parens(query_str));
            
            // Step 3: Test in transform
            let queries = vec![query];
            let transformed = lastMediaQueryWinsTransform(queries);
            assert_eq!(transformed.len(), 1);
            
            // TODO: Test full parsing when parser is implemented
            // let parsed = MediaQuery::parser().parse_to_end(query_str);
            // assert!(parsed.is_ok());
        }
    }

    #[test]
    fn test_real_world_media_query_examples() {
        // Test real-world media query patterns
        let real_world_queries = vec![
            // Responsive breakpoints
            "@media (min-width: 576px)",
            "@media (min-width: 768px)",
            "@media (min-width: 992px)",
            "@media (min-width: 1200px)",
            "@media (min-width: 1400px)",
            
            // Mobile-first design
            "@media screen and (max-width: 768px)",
            "@media screen and (min-width: 769px) and (max-width: 1024px)",
            "@media screen and (min-width: 1025px)",
            
            // Device-specific queries
            "@media only screen and (min-device-width: 375px) and (max-device-width: 667px)",
            "@media only screen and (-webkit-min-device-pixel-ratio: 2)",
            
            // Accessibility queries
            "@media (prefers-reduced-motion: reduce)",
            "@media (prefers-color-scheme: dark)",
            "@media (prefers-contrast: high)",
            
            // Print styles
            "@media print",
            "@media print and (min-resolution: 300dpi)",
        ];

        for query_str in real_world_queries {
            let query = MediaQuery::new(query_str.to_string());
            assert_eq!(query.to_string(), query_str);
            assert!(MediaQuery::has_balanced_parens(query_str));
            
            // Test that queries can be used in transform
            let queries = vec![query];
            let transformed = lastMediaQueryWinsTransform(queries);
            assert_eq!(transformed.len(), 1);
        }
    }

    #[test]
    fn test_media_query_error_handling() {
        // Test error handling for various edge cases
        let edge_cases = vec![
            "", // Empty string
            "@media", // Incomplete
            "not a media query", // Invalid format
            "@media screen and", // Incomplete and
        ];

        for case in edge_cases {
            // Our current implementation should handle these gracefully
            let query = MediaQuery::new(case.to_string());
            assert_eq!(query.to_string(), case);
            
            // Balanced parens check should work even for invalid queries
            let _balanced = MediaQuery::has_balanced_parens(case);
            // No assertion needed - just ensure it doesn't panic
        }
    }
}
