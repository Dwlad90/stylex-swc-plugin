#[cfg(test)]
mod transform_value_content_property_tests {
  use crate::shared::{
    structures::{state_manager::StateManager, stylex_options::StyleXOptions},
    utils::css::common::transform_value,
  };

  #[test]
  fn preserves_css_functions_without_quotes() {
    let functions = vec![
      "counters(div, \".\")",
      "counter(chapter)",
      "counter(chapter, upper-roman)",
      "attr(href)",
      "url(image.jpg)",
      "linear-gradient(#e66465, #9198e5)",
      "image-set(\"image1x.png\" 1x, \"image2x.png\" 2x)",
      "\"prefix\"attr(href)",
      "url(foo.jpg)attr(alt)",
      "var(--test)",
      "var(--test, \"default\")",
    ];

    let state_manager = StateManager::new(StyleXOptions::default());

    for input in functions {
      let output = transform_value("content", input, &state_manager);
      assert_eq!(output, input);
    }
  }

  #[test]
  fn preserves_css_keywords_without_quotes() {
    let keywords = vec![
      "normal",
      "none",
      "open-quote",
      "close-quote",
      "no-open-quote",
      "no-close-quote",
      "inherit",
      "initial",
      "revert",
      "revert-layer",
      "unset",
    ];

    let state_manager = StateManager::new(StyleXOptions::default());

    for keyword in keywords {
      let output = transform_value("content", keyword, &state_manager);
      assert_eq!(output, keyword);
    }
  }

  #[test]
  fn handles_mixed_content_values() {
    let mixed_values = vec![
      "open-quote counter(chapter)",
      "\"prefix\"url(image.jpg)",
      "url(\"test.png\")/\"Alt text\"",
      "open-quotecounter(chapter)close-quote",
      "attr(href)normal",
      "\"text\"attr(href)\"more text\"",
      "counter(x)\"text\"counter(y)",
    ];

    let state_manager = StateManager::new(StyleXOptions::default());

    for input in mixed_values {
      let output = transform_value("content", input, &state_manager);
      assert_eq!(output, input);
    }
  }

  #[test]
  fn adds_quotes_to_plain_strings() {
    let strings = vec![
      ("Hello world", "\"Hello world\""),
      ("Simple text", "\"Simple text\""),
      ("123", "\"123px\""),
    ];

    let state_manager = StateManager::new(StyleXOptions::default());

    for (input, expected) in strings {
      let output = transform_value("content", input, &state_manager);
      assert_eq!(output, expected);
    }
  }

  #[test]
  fn preserve_units_in_zero_values_css_variables() {
    let variables = vec![
      // CSS variables should preserve units
      ("--test", "0px", "0px"),
      ("--test", "0vdh", "0vdh"),
      // Regular properties still normalize
      ("transform", "0rad", "0deg"),
      ("animation-duration", "0ms", "0s"),
      // grid-template-rows preserves fr units
      ("grid-template-rows", "0fr", "0fr"),
      // Percentages are always preserved
      ("width", "0%", "0%"),
      // Regular properties normalize px to unitless
      ("margin", "0px", "0"),
    ];

    let state_manager = StateManager::new(StyleXOptions::default());

    for (key, value, expected) in variables {
      let output = transform_value(key, value, &state_manager);
      assert_eq!(
        output, expected,
        "Failed for property '{}' with value '{}': expected '{}', got '{}'",
        key, value, expected, output
      );
    }
  }
}
