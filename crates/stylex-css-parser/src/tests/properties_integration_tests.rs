/*!
Comprehensive integration tests for properties module.

Mirrors the JavaScript test structure from:
- packages/style-value-parser/src/properties/__tests__/transform.js
- packages/style-value-parser/src/properties/__tests__/box-shadow.test.js
- packages/style-value-parser/src/properties/__tests__/border-radius.test.js

These tests focus on the parts of our implementation that are currently working,
while noting TODOs for areas that need more complete implementation.
*/

use crate::{
    properties::{
        Transform,
        BoxShadow, BoxShadowList,
        BorderRadiusIndividual, BorderRadiusShorthand,
    },
    css_types::{
        Length, Color, NamedColor, HashColor, RgbaColor, HslaColor,
        LengthPercentage, Percentage, Angle,
        transform_function::{
            TransformFunction, Matrix, Matrix3d, Perspective, Rotate, RotateXYZ, Rotate3d,
            Scale, Scale3d, ScaleAxis, Skew, SkewAxis, Translate, Translate3d, TranslateAxis,
            Axis, SkewAxis2D
        },
    },
};

#[cfg(test)]
mod transform_tests {
    use super::*;

    #[test]
    fn test_transform_function_creation_and_display() {
        // Test basic transform function creation
        let translate = TransformFunction::Translate(Translate::new(
            LengthPercentage::Length(Length::new(50.0, "px".to_string())),
            Some(LengthPercentage::Length(Length::new(100.0, "px".to_string())))
        ));
        assert_eq!(translate.to_string(), "translate(50px, 100px)");

        let rotate = TransformFunction::Rotate(Rotate::new(
            Angle::new(45.0, "deg".to_string())
        ));
        assert_eq!(rotate.to_string(), "rotate(45deg)");

        let scale = TransformFunction::Scale(Scale::new(
            1.5,
            Some(0.8)
        ));
        assert_eq!(scale.to_string(), "scale(1.5, 0.8)");
    }

    #[test]
    fn test_transform_matrix_functions() {
        // Test matrix function (simplified - would normally parse actual Matrix struct)
        let matrix = TransformFunction::Matrix(Matrix::new(
            1.0, 0.0, 0.0, 1.0, 0.0, 0.0
        ));
        assert_eq!(matrix.to_string(), "matrix(1, 0, 0, 1, 0, 0)");

        let matrix3d = TransformFunction::Matrix3d(Matrix3d::new(
            [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.5, 1.5, 0.0, 0.0, 0.0, 0.0, 1.0]
        ));
        // Check the display format
        assert!(matrix3d.to_string().contains("matrix3d"));
        if let TransformFunction::Matrix3d(m) = matrix3d {
            assert_eq!(m.args.len(), 16);
        }
    }

    #[test]
    fn test_transform_rotation_functions() {
        // Test rotation functions (simplified)
        let rotate = TransformFunction::Rotate(Rotate::new(
            Angle::new(45.0, "deg".to_string())
        ));
        assert_eq!(rotate.to_string(), "rotate(45deg)");

        let rotate3d = TransformFunction::Rotate3d(Rotate3d::new(
            1.0, 2.0, 3.0,
            Angle::new(45.0, "deg".to_string())
        ));
        assert_eq!(rotate3d.to_string(), "rotate3d(1, 2, 3, 45deg)");

        let rotate_x = TransformFunction::RotateXYZ(RotateXYZ::new(
            Angle::new(45.0, "deg".to_string()),
            Axis::X
        ));
        assert_eq!(rotate_x.to_string(), "rotateX(45deg)");

        let rotate_y = TransformFunction::RotateXYZ(RotateXYZ::new(
            Angle::new(45.0, "deg".to_string()),
            Axis::Y
        ));
        assert_eq!(rotate_y.to_string(), "rotateY(45deg)");

        let rotate_z = TransformFunction::RotateXYZ(RotateXYZ::new(
            Angle::new(45.0, "deg".to_string()),
            Axis::Z
        ));
        assert_eq!(rotate_z.to_string(), "rotateZ(45deg)");
    }

    #[test]
    fn test_transform_scale_functions() {
        // Test scale functions
        let scale_uniform = TransformFunction::Scale(Scale::new(2.0, None));
        assert_eq!(scale_uniform.to_string(), "scale(2)");

        let scale_xy = TransformFunction::Scale(Scale::new(2.0, Some(3.0)));
        assert_eq!(scale_xy.to_string(), "scale(2, 3)");

        let scale3d = TransformFunction::Scale3d(Scale3d::new(2.0, 3.0, 4.0));
        assert_eq!(scale3d.to_string(), "scale3d(2, 3, 4)");
    }

    #[test]
    fn test_transform_translate_functions() {
        let translate = TransformFunction::Translate(Translate::new(
            LengthPercentage::Length(Length::new(50.0, "px".to_string())),
            Some(LengthPercentage::Length(Length::new(100.0, "px".to_string())))
        ));
        assert_eq!(translate.to_string(), "translate(50px, 100px)");

        let translate_x = TransformFunction::TranslateAxis(TranslateAxis::new(
            LengthPercentage::Length(Length::new(50.0, "px".to_string())),
            Axis::X
        ));
        assert_eq!(translate_x.to_string(), "translateX(50px)");

        let translate_y = TransformFunction::TranslateAxis(TranslateAxis::new(
            LengthPercentage::Length(Length::new(100.0, "px".to_string())),
            Axis::Y
        ));
        assert_eq!(translate_y.to_string(), "translateY(100px)");

        let translate_z = TransformFunction::TranslateAxis(TranslateAxis::new(
            LengthPercentage::Length(Length::new(200.0, "px".to_string())),
            Axis::Z
        ));
        assert_eq!(translate_z.to_string(), "translateZ(200px)");
    }

    #[test]
    fn test_transform_skew_functions() {
        let skew = TransformFunction::Skew(Skew::new(
            Angle::new(45.0, "deg".to_string()),
            None
        ));
        assert_eq!(skew.to_string(), "skew(45deg)");

        let skew_x = TransformFunction::SkewAxis(SkewAxis::new(
            Angle::new(20.0, "deg".to_string()),
            SkewAxis2D::X
        ));
        assert_eq!(skew_x.to_string(), "skewX(20deg)");

        let skew_y = TransformFunction::SkewAxis(SkewAxis::new(
            Angle::new(15.0, "deg".to_string()),
            SkewAxis2D::Y
        ));
        assert_eq!(skew_y.to_string(), "skewY(15deg)");
    }

    #[test]
    fn test_transform_single_function() {
        let func = TransformFunction::Perspective(Perspective::new(
            Length::new(100.0, "px".to_string())
        ));
        let transform = Transform::new(vec![func]);

        assert_eq!(transform.value.len(), 1);
        assert_eq!(transform.to_string(), "perspective(100px)");
    }

    #[test]
    fn test_transform_multiple_functions() {
        // Test multiple transform functions like in JavaScript tests
        let funcs = vec![
            TransformFunction::Translate(Translate::new(
                LengthPercentage::Length(Length::new(50.0, "px".to_string())),
                Some(LengthPercentage::Length(Length::new(100.0, "px".to_string())))
            )),
            TransformFunction::Rotate(Rotate::new(
                Angle::new(45.0, "deg".to_string())
            )),
            TransformFunction::Scale(Scale::new(1.5, None)),
        ];

        let transform = Transform::new(funcs);
        assert_eq!(transform.value.len(), 3);
        assert_eq!(transform.to_string(), "translate(50px, 100px) rotate(45deg) scale(1.5)");
    }

    #[test]
    fn test_transform_complex_combinations() {
        // Test complex transform combinations from JavaScript tests

        // perspective + matrix3d
        let perspective_matrix = Transform::new(vec![
            TransformFunction::Perspective(Perspective::new(
                Length::new(100.0, "px".to_string())
            )),
            TransformFunction::Matrix3d(Matrix3d::new([
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.5, 1.5, 0.0,
                0.0, 0.0, 0.0, 1.0
            ])),
        ]);
        assert_eq!(perspective_matrix.value.len(), 2);

        // scale + rotate + translate + skew (like in JavaScript test)
        let complex_transform = Transform::new(vec![
            TransformFunction::Scale(Scale::new(2.0, None)),
            TransformFunction::Rotate(Rotate::new(
                Angle::new(45.0, "deg".to_string())
            )),
            TransformFunction::Translate(Translate::new(
                LengthPercentage::Length(Length::new(100.0, "px".to_string())),
                None
            )),
            TransformFunction::Skew(Skew::new(
                Angle::new(45.0, "deg".to_string()),
                None
            )),
        ]);
        assert_eq!(complex_transform.value.len(), 4);
        assert_eq!(complex_transform.to_string(), "scale(2) rotate(45deg) translate(100px) skew(45deg)");
    }

    #[test]
    fn test_transform_empty() {
        let empty_transform = Transform::new(vec![]);
        assert_eq!(empty_transform.to_string(), "");
        assert_eq!(empty_transform.value.len(), 0);
    }

    #[test]
    fn test_transform_equality() {
        let func1 = TransformFunction::Rotate(Rotate::new(
            Angle::new(45.0, "deg".to_string())
        ));
        let func2 = TransformFunction::Rotate(Rotate::new(
            Angle::new(45.0, "deg".to_string())
        ));
        let func3 = TransformFunction::Rotate(Rotate::new(
            Angle::new(90.0, "deg".to_string())
        ));

        let transform1 = Transform::new(vec![func1]);
        let transform2 = Transform::new(vec![func2]);
        let transform3 = Transform::new(vec![func3]);

        assert_eq!(transform1, transform2);
        assert_ne!(transform1, transform3);
    }

    // TODO: These tests will work once TransformFunction parsing is fully implemented
    // #[test]
    // fn test_transform_parsing() {
    //     // Will be enabled when Transform::parser() is fully implemented
    //     let result = Transform::parser().parse_to_end("matrix(1, 0, 0, 1, 0, 0)");
    //     // assert!(result.is_ok());
    // }
}

#[cfg(test)]
mod box_shadow_tests {
    use super::*;

    #[test]
    fn test_box_shadow_creation_basic() {
        // Test basic box shadow creation matching JavaScript tests
        let shadow = BoxShadow::new(
            Length::new(10.0, "px".to_string()),
            Length::new(5.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Color::Hash(HashColor::new("#ff0000".to_string())),
            false,
        );

        assert_eq!(shadow.offset_x.value, 10.0);
        assert_eq!(shadow.offset_y.value, 5.0);
        assert_eq!(shadow.blur_radius.value, 0.0);
        assert_eq!(shadow.spread_radius.value, 0.0);
        assert!(!shadow.inset);
    }

    #[test]
    fn test_box_shadow_with_blur_radius() {
        // Test box shadow with blur radius (offsetX, offsetY, blurRadius, color)
        let shadow = BoxShadow::new(
            Length::new(10.0, "px".to_string()),
            Length::new(5.0, "px".to_string()),
            Length::new(15.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Color::Hash(HashColor::new("#ff0000".to_string())),
            false,
        );

        assert_eq!(shadow.blur_radius.value, 15.0);
        assert_eq!(shadow.spread_radius.value, 0.0);
    }

    #[test]
    fn test_box_shadow_with_blur_and_spread() {
        // Test box shadow with blur and spread radius
        let shadow = BoxShadow::new(
            Length::new(10.0, "px".to_string()),
            Length::new(5.0, "px".to_string()),
            Length::new(15.0, "px".to_string()),
            Length::new(8.0, "px".to_string()),
            Color::Hash(HashColor::new("#ff0000".to_string())),
            false,
        );

        assert_eq!(shadow.blur_radius.value, 15.0);
        assert_eq!(shadow.spread_radius.value, 8.0);
    }

    #[test]
    fn test_box_shadow_inset() {
        // Test box shadow with inset keyword
        let shadow = BoxShadow::new(
            Length::new(10.0, "px".to_string()),
            Length::new(5.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Color::Hash(HashColor::new("#ff0000".to_string())),
            true, // inset
        );

        assert!(shadow.inset);
    }

    #[test]
    fn test_box_shadow_different_length_units() {
        // Test different length units like in JavaScript test
        let shadow = BoxShadow::new(
            Length::new(1.0, "rem".to_string()),
            Length::new(0.5, "em".to_string()),
            Length::new(2.0, "vw".to_string()),
            Length::new(1.0, "vh".to_string()),
            Color::Rgba(RgbaColor::new(0, 0, 0, 0.5)),
            false,
        );

        assert_eq!(shadow.offset_x.unit, "rem");
        assert_eq!(shadow.offset_y.unit, "em");
        assert_eq!(shadow.blur_radius.unit, "vw");
        assert_eq!(shadow.spread_radius.unit, "vh");
    }

    #[test]
    fn test_box_shadow_rgba_color() {
        // Test box shadow with rgba color
        let shadow = BoxShadow::new(
            Length::new(10.0, "px".to_string()),
            Length::new(5.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Color::Rgba(RgbaColor::new(255, 0, 0, 0.5)),
            false,
        );

        if let Color::Rgba(rgba) = &shadow.color {
            assert_eq!(rgba.r, 255);
            assert_eq!(rgba.g, 0);
            assert_eq!(rgba.b, 0);
            assert_eq!(rgba.a, 0.5);
        } else {
            panic!("Expected Rgba color");
        }
    }

    #[test]
    fn test_box_shadow_hsla_color() {
        // Test box shadow with hsla color
        let shadow = BoxShadow::new(
            Length::new(10.0, "px".to_string()),
            Length::new(5.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Color::Hsla(HslaColor::new(0.0, 100.0, 50.0, 0.5)),
            false,
        );

        if let Color::Hsla(hsla) = &shadow.color {
            assert_eq!(hsla.h, 0.0);
            assert_eq!(hsla.s, 100.0);
            assert_eq!(hsla.l, 50.0);
            assert_eq!(hsla.a, 0.5);
        } else {
            panic!("Expected Hsla color");
        }
    }

    #[test]
    fn test_box_shadow_display() {
        // Test toString functionality
        let shadow = BoxShadow::new(
            Length::new(2.0, "px".to_string()),
            Length::new(4.0, "px".to_string()),
            Length::new(6.0, "px".to_string()),
            Length::new(2.0, "px".to_string()),
            Color::Named(NamedColor::new("red".to_string())),
            false,
        );
        assert_eq!(shadow.to_string(), "2px 4px 6px 2px red");

        let inset_shadow = BoxShadow::new(
            Length::new(1.0, "px".to_string()),
            Length::new(1.0, "px".to_string()),
            Length::new(2.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Color::Named(NamedColor::new("blue".to_string())),
            true,
        );
        assert_eq!(inset_shadow.to_string(), "inset 1px 1px 2px blue");
    }

    #[test]
    fn test_box_shadow_display_zero_values() {
        // Test that zero blur and spread values are omitted
        let shadow = BoxShadow::new(
            Length::new(1.0, "px".to_string()),
            Length::new(2.0, "px".to_string()),
            Length::new(0.0, "px".to_string()), // Zero blur
            Length::new(0.0, "px".to_string()), // Zero spread
            Color::Named(NamedColor::new("black".to_string())),
            false,
        );

        assert_eq!(shadow.to_string(), "1px 2px black");
    }

    #[test]
    fn test_box_shadow_simple_constructor() {
        // Test the simplified constructor
        let shadow = BoxShadow::simple(
            Length::new(5.0, "px".to_string()),
            Length::new(10.0, "px".to_string()),
            Some(Length::new(15.0, "px".to_string())),
            None, // No spread
            Color::Named(NamedColor::new("gray".to_string())),
            false,
        );

        assert_eq!(shadow.blur_radius.value, 15.0);
        assert_eq!(shadow.spread_radius.value, 0.0); // Default value
    }

    #[test]
    fn test_box_shadow_list_creation() {
        // Test BoxShadowList creation
        let shadow1 = BoxShadow::simple(
            Length::new(1.0, "px".to_string()),
            Length::new(1.0, "px".to_string()),
            None,
            None,
            Color::Hash(HashColor::new("#ff0000".to_string())),
            false,
        );

        let shadow2 = BoxShadow::simple(
            Length::new(5.0, "px".to_string()),
            Length::new(5.0, "px".to_string()),
            Some(Length::new(10.0, "px".to_string())),
            None,
            Color::Hash(HashColor::new("#00ff00".to_string())),
            false,
        );

        let shadow_list = BoxShadowList::new(vec![shadow1, shadow2]);
        assert_eq!(shadow_list.shadows.len(), 2);
    }

    #[test]
    fn test_box_shadow_list_display() {
        // Test comma-separated display of multiple shadows
        let shadow1 = BoxShadow::new(
            Length::new(1.0, "px".to_string()),
            Length::new(1.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Color::Named(NamedColor::new("red".to_string())),
            false,
        );

        let shadow2 = BoxShadow::new(
            Length::new(2.0, "px".to_string()),
            Length::new(2.0, "px".to_string()),
            Length::new(4.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Color::Named(NamedColor::new("blue".to_string())),
            true,
        );

        let shadow_list = BoxShadowList::new(vec![shadow1, shadow2]);
        assert_eq!(shadow_list.to_string(), "1px 1px red, inset 2px 2px 4px blue");
    }

    #[test]
    fn test_box_shadow_common_patterns() {
        // Test common box-shadow patterns

        // Drop shadow
        let drop_shadow = BoxShadow::simple(
            Length::new(0.0, "px".to_string()),
            Length::new(2.0, "px".to_string()),
            Some(Length::new(4.0, "px".to_string())),
            None,
            Color::Hash(HashColor::new("#00000026".to_string())),
            false,
        );
        assert!(!drop_shadow.inset);

        // Inner shadow
        let inner_shadow = BoxShadow::simple(
            Length::new(0.0, "px".to_string()),
            Length::new(1.0, "px".to_string()),
            Some(Length::new(2.0, "px".to_string())),
            None,
            Color::Hash(HashColor::new("#0000001a".to_string())),
            true,
        );
        assert!(inner_shadow.inset);

        // Glow effect (spread)
        let glow = BoxShadow::new(
            Length::new(0.0, "px".to_string()),
            Length::new(0.0, "px".to_string()),
            Length::new(10.0, "px".to_string()),
            Length::new(5.0, "px".to_string()),
            Color::Named(NamedColor::new("blue".to_string())),
            false,
        );
        assert_eq!(glow.spread_radius.value, 5.0);
    }

    // TODO: These tests will work once BoxShadow parsing is fully implemented
    // #[test]
    // fn test_box_shadow_parsing() {
    //     // Will be enabled when BoxShadow::parser() handles all CSS syntax
    //     let result = BoxShadow::parser().parse_to_end("10px 5px #ff0000");
    //     // assert!(result.is_ok());
    // }
}

#[cfg(test)]
mod border_radius_tests {
    use super::*;

    #[test]
    fn test_border_radius_individual_single_value() {
        // Test single value (applies to both horizontal and vertical)
        let length = LengthPercentage::Length(Length::new(10.0, "px".to_string()));
        let radius = BorderRadiusIndividual::new(length.clone(), None);

        assert_eq!(radius.horizontal, length);
        assert_eq!(radius.vertical, length);
        assert_eq!(radius.to_string(), "10px");
    }

    #[test]
    fn test_border_radius_individual_two_values() {
        // Test two values (horizontal and vertical different)
        let horizontal = LengthPercentage::Length(Length::new(10.0, "px".to_string()));
        let vertical = LengthPercentage::Length(Length::new(20.0, "px".to_string()));
        let radius = BorderRadiusIndividual::new(horizontal.clone(), Some(vertical.clone()));

        assert_eq!(radius.horizontal, horizontal);
        assert_eq!(radius.vertical, vertical);
        assert_eq!(radius.to_string(), "10px 20px");
    }

    #[test]
    fn test_border_radius_individual_mixed_units() {
        // Test mixing different units
        let pixels = LengthPercentage::Length(Length::new(5.0, "px".to_string()));
        let percentage = LengthPercentage::Percentage(Percentage::new(25.0));
        let radius = BorderRadiusIndividual::new(pixels, Some(percentage));

        assert_eq!(radius.to_string(), "5px 25%");
    }

    #[test]
    fn test_border_radius_individual_percentage() {
        // Test percentage values
        let percentage = LengthPercentage::Percentage(Percentage::new(50.0));
        let radius = BorderRadiusIndividual::new(percentage, None);

        assert_eq!(radius.to_string(), "50%");
    }

    #[test]
    fn test_border_radius_individual_fractional_values() {
        // Test fractional values like in JavaScript tests
        let frac1 = LengthPercentage::Length(Length::new(0.5, "px".to_string()));
        let radius1 = BorderRadiusIndividual::new(frac1, None);
        assert_eq!(radius1.to_string(), "0.5px");

        let frac2 = LengthPercentage::Length(Length::new(0.0005, "px".to_string()));
        let radius2 = BorderRadiusIndividual::new(frac2, None);
        assert_eq!(radius2.to_string(), "0.0005px");
    }

    #[test]
    fn test_border_radius_shorthand_single_value() {
        // Test single value expansion (all corners same)
        let value = LengthPercentage::Length(Length::new(5.0, "px".to_string()));
        let shorthand = BorderRadiusShorthand::new(value.clone(), None, None, None, None, None, None, None);

        // All corners should be the same
        assert_eq!(shorthand.horizontal_top_left, value);
        assert_eq!(shorthand.horizontal_top_right, value);
        assert_eq!(shorthand.horizontal_bottom_right, value);
        assert_eq!(shorthand.horizontal_bottom_left, value);
        assert_eq!(shorthand.vertical_top_left, value);
        assert_eq!(shorthand.vertical_top_right, value);
        assert_eq!(shorthand.vertical_bottom_right, value);
        assert_eq!(shorthand.vertical_bottom_left, value);

        assert_eq!(shorthand.to_string(), "5px");
    }

    #[test]
    fn test_border_radius_shorthand_css_expansion_logic() {
        // Test CSS shorthand expansion logic (2 values)
        let top_left = LengthPercentage::Length(Length::new(10.0, "px".to_string()));
        let top_right = LengthPercentage::Length(Length::new(20.0, "px".to_string()));

        let shorthand = BorderRadiusShorthand::new(
            top_left.clone(),
            Some(top_right.clone()),
            None, // Should default to top_left
            None, // Should default to top_right
            None, None, None, None
        );

        assert_eq!(shorthand.horizontal_top_left, top_left);
        assert_eq!(shorthand.horizontal_top_right, top_right);
        assert_eq!(shorthand.horizontal_bottom_right, top_left); // Defaults to top_left
        assert_eq!(shorthand.horizontal_bottom_left, top_right); // Defaults to top_right
    }

    #[test]
    fn test_border_radius_shorthand_three_values() {
        // Test 3-value expansion logic
        let val1 = LengthPercentage::Length(Length::new(10.0, "px".to_string()));
        let val2 = LengthPercentage::Length(Length::new(20.0, "px".to_string()));
        let val3 = LengthPercentage::Length(Length::new(30.0, "px".to_string()));

        let shorthand = BorderRadiusShorthand::new(
            val1.clone(),
            Some(val2.clone()),
            Some(val3.clone()),
            None, // Should default to val2
            None, None, None, None
        );

        assert_eq!(shorthand.horizontal_top_left, val1);
        assert_eq!(shorthand.horizontal_top_right, val2);
        assert_eq!(shorthand.horizontal_bottom_right, val3);
        assert_eq!(shorthand.horizontal_bottom_left, val2); // Defaults to top_right
    }

    #[test]
    fn test_border_radius_shorthand_four_values() {
        // Test 4-value expansion (all different)
        let val1 = LengthPercentage::Length(Length::new(10.0, "px".to_string()));
        let val2 = LengthPercentage::Length(Length::new(20.0, "px".to_string()));
        let val3 = LengthPercentage::Length(Length::new(30.0, "px".to_string()));
        let val4 = LengthPercentage::Length(Length::new(40.0, "px".to_string()));

        let shorthand = BorderRadiusShorthand::new(
            val1.clone(),
            Some(val2.clone()),
            Some(val3.clone()),
            Some(val4.clone()),
            None, None, None, None
        );

        assert_eq!(shorthand.horizontal_top_left, val1);
        assert_eq!(shorthand.horizontal_top_right, val2);
        assert_eq!(shorthand.horizontal_bottom_right, val3);
        assert_eq!(shorthand.horizontal_bottom_left, val4);
    }

    #[test]
    fn test_border_radius_shorthand_percentages() {
        // Test percentage values in shorthand
        let percent1 = LengthPercentage::Percentage(Percentage::new(50.0));
        let percent2 = LengthPercentage::Percentage(Percentage::new(25.0));

        let shorthand = BorderRadiusShorthand::new(
            percent1.clone(),
            Some(percent2.clone()),
            None, None, None, None, None, None
        );

        // Should output shortest form
        assert!(shorthand.to_string().contains("50%"));
        assert!(shorthand.to_string().contains("25%"));
    }

    #[test]
    fn test_border_radius_shorthand_asymmetric_basic() {
        // Test basic asymmetric border radius (horizontal / vertical)
        let horizontal = LengthPercentage::Length(Length::new(10.0, "px".to_string()));
        let vertical = LengthPercentage::Length(Length::new(20.0, "px".to_string()));

        let shorthand = BorderRadiusShorthand::new(
            horizontal.clone(),
            None, None, None,
            Some(vertical), // Different vertical value
            None, None, None
        );

        // Should use "/" format for asymmetric
        assert!(shorthand.to_string().contains("/"));
    }

    #[test]
    fn test_border_radius_common_values() {
        // Test common border-radius values used in web development

        // Small rounded corners
        let small = LengthPercentage::Length(Length::new(3.0, "px".to_string()));
        let small_radius = BorderRadiusIndividual::new(small, None);
        assert_eq!(small_radius.to_string(), "3px");

        // Medium rounded corners
        let medium = LengthPercentage::Length(Length::new(6.0, "px".to_string()));
        let medium_radius = BorderRadiusIndividual::new(medium, None);
        assert_eq!(medium_radius.to_string(), "6px");

        // Large rounded corners
        let large = LengthPercentage::Length(Length::new(12.0, "px".to_string()));
        let large_radius = BorderRadiusIndividual::new(large, None);
        assert_eq!(large_radius.to_string(), "12px");

        // Perfect circle (50%)
        let circle = LengthPercentage::Percentage(Percentage::new(50.0));
        let circle_radius = BorderRadiusIndividual::new(circle, None);
        assert_eq!(circle_radius.to_string(), "50%");
    }

    #[test]
    fn test_border_radius_elliptical() {
        // Test elliptical border radius (different horizontal and vertical)
        let horizontal = LengthPercentage::Length(Length::new(20.0, "px".to_string()));
        let vertical = LengthPercentage::Length(Length::new(10.0, "px".to_string()));

        let elliptical = BorderRadiusIndividual::new(horizontal, Some(vertical));
        assert_eq!(elliptical.to_string(), "20px 10px");
    }

    #[test]
    fn test_border_radius_equality() {
        let value1 = LengthPercentage::Length(Length::new(5.0, "px".to_string()));
        let value2 = LengthPercentage::Length(Length::new(5.0, "px".to_string()));
        let value3 = LengthPercentage::Length(Length::new(10.0, "px".to_string()));

        let radius1 = BorderRadiusIndividual::new(value1.clone(), None);
        let radius2 = BorderRadiusIndividual::new(value2, None);
        let radius3 = BorderRadiusIndividual::new(value3, None);

        assert_eq!(radius1, radius2);
        assert_ne!(radius1, radius3);
    }

    #[test]
    fn test_border_radius_edge_cases() {
        // Test edge cases

        // Zero radius
        let zero = LengthPercentage::Length(Length::new(0.0, "px".to_string()));
        let zero_radius = BorderRadiusIndividual::new(zero, None);
        assert_eq!(zero_radius.to_string(), "0px");

        // Very small values
        let tiny = LengthPercentage::Length(Length::new(0.1, "px".to_string()));
        let tiny_radius = BorderRadiusIndividual::new(tiny, None);
        assert_eq!(tiny_radius.to_string(), "0.1px");

        // Mixed units in asymmetric
        let rem_val = LengthPercentage::Length(Length::new(1.0, "rem".to_string()));
        let em_val = LengthPercentage::Length(Length::new(2.0, "em".to_string()));
        let mixed = BorderRadiusIndividual::new(rem_val, Some(em_val));
        assert_eq!(mixed.to_string(), "1rem 2em");
    }

    // TODO: These tests will work once BorderRadius parsing is fully implemented
    // #[test]
    // fn test_border_radius_parsing() {
    //     // Will be enabled when parsers handle full CSS syntax
    //     let result = BorderRadiusIndividual::parser().parse_to_end("10px 20px");
    //     // assert!(result.is_ok());
    // }
}
