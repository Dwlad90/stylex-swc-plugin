// Test JavaScript compatibility based on the attached color-test.js

#[cfg(test)]
mod tests {
    use crate::css_types::color::Color;

    #[test]
    fn test_javascript_compatibility() {
        let test_cases = vec![
            // From the JavaScript color-test.js file:

            // Named colors
            "red",
            "blue",
            "green",
            "transparent",

            // Hash colors
            "#ff0000",
            "#00ff00",
            "#0000ff",
            "#ffffff",

            // RGB values
            "rgb(255, 0, 0)",
            "rgb(0, 255, 0)",
            "rgb(0, 0, 255)",

            // Space-separated RGB
            "rgb(255 0 0)",
            "rgb(0 255 0)",
            "rgb(0 0 255)",

            // RGBA values
            "rgba(255, 0, 0, 0.5)",
            "rgba(0, 255, 0, 0.5)",
            "rgba(0, 0, 255, 0.5)",

            // Space-separated RGBA values (CSS4)
            "rgb(255 0 0 / 0.5)",
            "rgb(0 255 0 / 0.5)",
            "rgb(0 0 255 / 0.5)",
            "rgb(255 0 0 / 50%)",
            "rgb(0 255 0 / 50%)",
            "rgb(0 0 255 / 50%)",

            // LCH
            "lch(50% 100 270deg)",
        ];

        println!("Testing JavaScript compatibility...\n");

        let mut passed = 0;
        let mut total = 0;

        for input in test_cases {
            total += 1;
            match Color::parse().parse_to_end(input) {
                Ok(color) => {
                    println!("✅ {} -> {}", input, color);
                    passed += 1;
                }
                Err(e) => {
                    println!("❌ {} -> Error: {:?}", input, e);
                }
            }
        }

        println!("\n=== Results ===");
        println!("Passed: {}/{}", passed, total);

        assert_eq!(passed, total, "All JavaScript test cases should pass");
        println!("🎉 All JavaScript test cases are working!");
    }
}
