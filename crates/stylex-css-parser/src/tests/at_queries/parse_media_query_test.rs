/*!
Parse media query tests.

Complete test suite for media query parsing functionality.
*/

use crate::at_queries::media_query::{MediaQuery, MediaQueryRule, MediaRuleValue, WordRule};

#[cfg(test)]
mod style_value_parser_at_queries {
  use super::*;

  #[cfg(test)]
  mod parse_media_queries {
    use super::*;

    #[cfg(test)]
    mod keywords {
      use super::*;

      #[test]
      fn at_media_screen() {
        let parsed = MediaQuery::parser().parse_to_end("@media screen").unwrap();

        match &parsed.queries {
          MediaQueryRule::MediaKeyword(keyword) => {
            assert_eq!(keyword.key, "screen");
            assert!(!keyword.not);
            assert_eq!(keyword.only, None);
          }
          _ => panic!("Expected MediaKeyword rule"),
        }

        assert_eq!(parsed.to_string(), "@media screen");
      }

      #[test]
      fn at_media_print() {
        let parsed = MediaQuery::parser().parse_to_end("@media print").unwrap();

        match &parsed.queries {
          MediaQueryRule::MediaKeyword(keyword) => {
            assert_eq!(keyword.key, "print");
            assert!(!keyword.not);
            assert_eq!(keyword.only, None);
          }
          _ => panic!("Expected MediaKeyword rule"),
        }

        assert_eq!(parsed.to_string(), "@media print");
      }

      #[test]
      fn at_media_all() {
        let parsed = MediaQuery::parser().parse_to_end("@media all").unwrap();

        match &parsed.queries {
          MediaQueryRule::MediaKeyword(keyword) => {
            assert_eq!(keyword.key, "all");
            assert!(!keyword.not);
            assert_eq!(keyword.only, None);
          }
          _ => panic!("Expected MediaKeyword rule"),
        }

        assert_eq!(parsed.to_string(), "@media all");
      }

      #[test]
      fn at_media_only_screen() {
        let parsed = MediaQuery::parser()
          .parse_to_end("@media only screen")
          .unwrap();

        match &parsed.queries {
          MediaQueryRule::MediaKeyword(keyword) => {
            assert_eq!(keyword.key, "screen");
            assert!(!keyword.not);
            assert_eq!(keyword.only, Some(true));
          }
          _ => panic!("Expected MediaKeyword rule"),
        }

        assert_eq!(parsed.to_string(), "@media only screen");
      }

      #[test]
      fn at_media_only_print_and_color() {
        let parsed = MediaQuery::parser()
          .parse_to_end("@media only print and (color)")
          .unwrap();

        match &parsed.queries {
          MediaQueryRule::And { rules } => {
            assert_eq!(rules.len(), 2);

            // First rule: only print
            match &rules[0] {
              MediaQueryRule::MediaKeyword(keyword) => {
                assert_eq!(keyword.key, "print");
                assert!(!keyword.not);
                assert_eq!(keyword.only, Some(true));
              }
              _ => panic!("Expected MediaKeyword rule"),
            }

            // Second rule: (color)
            match &rules[1] {
              MediaQueryRule::WordRule(WordRule::Color) => {}
              _ => panic!("Expected WordRule Color"),
            }
          }
          _ => panic!("Expected And rule"),
        }

        assert_eq!(parsed.to_string(), "@media only print and (color)");
      }

      #[test]
      fn at_media_not_screen() {
        let parsed = MediaQuery::parser()
          .parse_to_end("@media not screen")
          .unwrap();

        match &parsed.queries {
          MediaQueryRule::Not { rule } => {
            // NOT rule containing a MediaKeyword
            match rule.as_ref() {
              MediaQueryRule::MediaKeyword(keyword) => {
                assert_eq!(keyword.key, "screen");
                assert!(!keyword.not); // The NOT is at higher level
                assert_eq!(keyword.only, None);
              }
              _ => panic!("Expected MediaKeyword inside NOT rule"),
            }
          }
          _ => panic!("Expected NOT rule, got: {:?}", parsed.queries),
        }

        assert_eq!(parsed.to_string(), "@media not screen");
      }

      #[test]
      fn at_media_not_all_and_monochrome() {
        let parsed = MediaQuery::parser()
          .parse_to_end("@media not all and (monochrome)")
          .unwrap();

        match &parsed.queries {
          MediaQueryRule::And { rules } => {
            assert_eq!(rules.len(), 2);

            // First rule: NOT rule containing "all"
            match &rules[0] {
              MediaQueryRule::Not { rule } => {
                match rule.as_ref() {
                  MediaQueryRule::MediaKeyword(keyword) => {
                    assert_eq!(keyword.key, "all");
                    assert!(!keyword.not); // NOT is at higher level
                    assert_eq!(keyword.only, None);
                  }
                  _ => panic!("Expected MediaKeyword inside NOT"),
                }
              }
              _ => panic!("Expected NOT rule"),
            }

            // Second rule: (monochrome)
            match &rules[1] {
              MediaQueryRule::WordRule(WordRule::Monochrome) => {}
              _ => panic!("Expected WordRule Monochrome"),
            }
          }
          _ => panic!("Expected And rule"),
        }

        assert_eq!(parsed.to_string(), "@media not all and (monochrome)");
      }
    }

    #[cfg(test)]
    mod pair_rule {
      use super::*;

      #[test]
      fn at_media_width_100px() {
        let input = "@media (width: 100px)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::Pair { key, value } => {
            assert_eq!(key, "width");
            match value {
              MediaRuleValue::Length(length) => {
                assert_eq!(length.value, 100.0);
                assert_eq!(length.unit, "px");
              }
              _ => panic!("Expected Length value"),
            }
          }
          _ => panic!("Expected Pair rule"),
        }

        assert_eq!(parsed.to_string(), "@media (width: 100px)");
      }

      #[test]
      fn at_media_max_width_50em() {
        let input = "@media (max-width: 50em)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::Pair { key, value } => {
            assert_eq!(key, "max-width");
            match value {
              MediaRuleValue::Length(length) => {
                assert_eq!(length.value, 50.0);
                assert_eq!(length.unit, "em");
              }
              _ => panic!("Expected Length value"),
            }
          }
          _ => panic!("Expected Pair rule"),
        }

        assert_eq!(parsed.to_string(), "@media (max-width: 50em)");
      }

      #[test]
      fn at_media_orientation_landscape() {
        let input = "@media (orientation: landscape)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::Pair { key, value } => {
            assert_eq!(key, "orientation");
            match value {
              MediaRuleValue::String(s) => {
                assert_eq!(s, "landscape");
              }
              _ => panic!("Expected String value"),
            }
          }
          _ => panic!("Expected Pair rule"),
        }

        assert_eq!(parsed.to_string(), "@media (orientation: landscape)");
      }

      #[test]
      fn at_media_prefers_color_scheme_dark() {
        let input = "@media (prefers-color-scheme: dark)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::Pair { key, value } => {
            assert_eq!(key, "prefers-color-scheme");
            match value {
              MediaRuleValue::String(s) => {
                assert_eq!(s, "dark");
              }
              _ => panic!("Expected String value"),
            }
          }
          _ => panic!("Expected Pair rule"),
        }

        assert_eq!(parsed.to_string(), "@media (prefers-color-scheme: dark)");
      }

      #[test]
      fn at_media_min_width_calc() {
        let input = "@media (min-width: calc(300px + 5em))";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::Pair { key, value } => {
            assert_eq!(key, "min-width");
            match value {
              MediaRuleValue::String(calc_str) => {
                assert_eq!(calc_str, "calc(300px + 5em)");
              }
              _ => panic!("Expected String value for calc"),
            }
          }
          _ => panic!("Expected Pair rule"),
        }

        assert_eq!(parsed.to_string(), "@media (min-width: calc(300px + 5em))");
      }

      #[test]
      fn at_media_max_height_calc() {
        let input = "@media (max-height: calc(100vh - 50px))";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::Pair { key, value } => {
            assert_eq!(key, "max-height");
            match value {
              MediaRuleValue::String(calc_str) => {
                assert_eq!(calc_str, "calc(100vh - 50px)");
              }
              _ => panic!("Expected String value for calc"),
            }
          }
          _ => panic!("Expected Pair rule"),
        }

        assert_eq!(
          parsed.to_string(),
          "@media (max-height: calc(100vh - 50px))"
        );
      }

      #[test]
      fn at_media_aspect_ratio() {
        let input = "@media (aspect-ratio: 16 / 9)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::Pair { key, value } => {
            assert_eq!(key, "aspect-ratio");
            match value {
              MediaRuleValue::Fraction(fraction) => {
                assert_eq!(fraction.numerator, 16.0);
                assert_eq!(fraction.denominator, 9.0);
              }
              _ => panic!("Expected Fraction value"),
            }
          }
          _ => panic!("Expected Pair rule"),
        }

        assert_eq!(parsed.to_string(), "@media (aspect-ratio: 16/9)");
      }

      #[test]
      fn at_media_device_aspect_ratio() {
        let input = "@media (device-aspect-ratio: 16 / 9)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::Pair { key, value } => {
            assert_eq!(key, "device-aspect-ratio");
            match value {
              MediaRuleValue::Fraction(fraction) => {
                assert_eq!(fraction.numerator, 16.0);
                assert_eq!(fraction.denominator, 9.0);
              }
              _ => panic!("Expected Fraction value"),
            }
          }
          _ => panic!("Expected Pair rule"),
        }

        assert_eq!(parsed.to_string(), "@media (device-aspect-ratio: 16/9)");
      }

      #[test]
      fn at_media_min_resolution_150dpi() {
        let input = "@media (min-resolution: 150dpi)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::Pair { key, value } => {
            assert_eq!(key, "min-resolution");
            match value {
              MediaRuleValue::Length(length) => {
                assert_eq!(length.value, 150.0);
                assert_eq!(length.unit, "dpi");
              }
              _ => panic!("Expected Length value"),
            }
          }
          _ => panic!("Expected Pair rule"),
        }

        assert_eq!(parsed.to_string(), "@media (min-resolution: 150dpi)");
      }
    }

    #[cfg(test)]
    mod word_rule {
      use super::*;

      #[test]
      fn at_media_color() {
        let input = "@media (color)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::WordRule(WordRule::Color) => {}
          _ => panic!("Expected Color WordRule"),
        }

        assert_eq!(parsed.to_string(), "@media (color)");
      }

      #[test]
      fn at_media_color_index() {
        let input = "@media (color-index)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::WordRule(WordRule::ColorIndex) => {}
          _ => panic!("Expected ColorIndex WordRule"),
        }

        assert_eq!(parsed.to_string(), "@media (color-index)");
      }

      #[test]
      fn at_media_monochrome() {
        let input = "@media (monochrome)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::WordRule(WordRule::Monochrome) => {}
          _ => panic!("Expected Monochrome WordRule"),
        }

        assert_eq!(parsed.to_string(), "@media (monochrome)");
      }

      #[test]
      fn at_media_grid() {
        let input = "@media (grid)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::WordRule(WordRule::Grid) => {}
          _ => panic!("Expected Grid WordRule"),
        }

        assert_eq!(parsed.to_string(), "@media (grid)");
      }
    }

    #[cfg(test)]
    mod and_combinator {
      use super::*;

      #[test]
      fn at_media_not_all_and_monochrome() {
        let input = "@media not all and (monochrome)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::And { rules } => {
            assert_eq!(rules.len(), 2);

            // First rule: NOT rule containing "all"
            match &rules[0] {
              MediaQueryRule::Not { rule } => {
                match rule.as_ref() {
                  MediaQueryRule::MediaKeyword(keyword) => {
                    assert_eq!(keyword.key, "all");
                    assert!(!keyword.not); // NOT is at higher level
                    assert_eq!(keyword.only, None);
                  }
                  _ => panic!("Expected MediaKeyword inside NOT"),
                }
              }
              _ => panic!("Expected NOT rule"),
            }

            // Second rule: (monochrome)
            match &rules[1] {
              MediaQueryRule::WordRule(WordRule::Monochrome) => {}
              _ => panic!("Expected WordRule Monochrome"),
            }
          }
          _ => panic!("Expected And rule"),
        }

        assert_eq!(parsed.to_string(), "@media not all and (monochrome)");
      }

      #[test]
      fn at_media_screen_and_min_width() {
        let input = "@media screen and (min-width: 400px)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::And { rules } => {
            assert_eq!(rules.len(), 2);

            // First rule: screen
            match &rules[0] {
              MediaQueryRule::MediaKeyword(keyword) => {
                assert_eq!(keyword.key, "screen");
                assert!(!keyword.not);
                assert_eq!(keyword.only, None);
              }
              _ => panic!("Expected MediaKeyword rule"),
            }

            // Second rule: min-width pair
            match &rules[1] {
              MediaQueryRule::Pair { key, value } => {
                assert_eq!(key, "min-width");
                match value {
                  MediaRuleValue::Length(length) => {
                    assert_eq!(length.value, 400.0);
                    assert_eq!(length.unit, "px");
                  }
                  _ => panic!("Expected Length value"),
                }
              }
              _ => panic!("Expected Pair rule"),
            }
          }
          _ => panic!("Expected And rule"),
        }

        assert_eq!(parsed.to_string(), "@media screen and (min-width: 400px)");
      }

      #[test]
      fn at_media_min_height_and_orientation() {
        let input = "@media (min-height: 600px) and (orientation: landscape)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::And { rules } => {
            assert_eq!(rules.len(), 2);

            // Check that we have both rules regardless of order
            let has_min_height = rules.iter().any(|r| {
              matches!(r, MediaQueryRule::Pair { key, value } if key == "min-height" && matches!(value, MediaRuleValue::Length(l) if l.value == 600.0 && l.unit == "px"))
            });
            let has_orientation = rules.iter().any(|r| {
              matches!(r, MediaQueryRule::Pair { key, value } if key == "orientation" && matches!(value, MediaRuleValue::String(s) if s == "landscape"))
            });

            assert!(has_min_height, "Should have min-height: 600px rule");
            assert!(has_orientation, "Should have orientation: landscape rule");
          }
          _ => panic!("Expected And rule"),
        }

        assert_eq!(
          parsed.to_string(),
          "@media (orientation: landscape) and (min-height: 600px)"
        );
      }
    }

    #[cfg(test)]
    mod or_combinator {
      use super::*;

      #[test]
      fn at_media_orientation_portrait_or_landscape() {
        let input = "@media (orientation: portrait), (orientation: landscape)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::Or { rules } => {
            assert_eq!(rules.len(), 2);

            match &rules[0] {
              MediaQueryRule::Pair { key, value } => {
                assert_eq!(key, "orientation");
                match value {
                  MediaRuleValue::String(s) => {
                    assert_eq!(s, "portrait");
                  }
                  _ => panic!("Expected String value"),
                }
              }
              _ => panic!("Expected Pair rule"),
            }

            // Second rule: landscape
            match &rules[1] {
              MediaQueryRule::Pair { key, value } => {
                assert_eq!(key, "orientation");
                match value {
                  MediaRuleValue::String(s) => {
                    assert_eq!(s, "landscape");
                  }
                  _ => panic!("Expected String value"),
                }
              }
              _ => panic!("Expected Pair rule"),
            }
          }
          _ => panic!("Expected Or rule"),
        }

        assert_eq!(
          parsed.to_string(),
          "@media (orientation: portrait), (orientation: landscape)"
        );
      }

      #[test]
      fn at_media_min_width_or_max_width() {
        let input = "@media (min-width: 500px) or (max-width: 600px)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::Or { rules } => {
            assert_eq!(rules.len(), 2);

            // Should contain both width rules
            let has_min = rules
              .iter()
              .any(|r| matches!(r, MediaQueryRule::Pair { key, .. } if key == "min-width"));
            let has_max = rules
              .iter()
              .any(|r| matches!(r, MediaQueryRule::Pair { key, .. } if key == "max-width"));
            assert!(has_min && has_max);
          }
          _ => panic!("Expected Or rule"),
        }

        assert_eq!(
          parsed.to_string(),
          "@media (min-width: 500px), (max-width: 600px)"
        );
      }
    }

    #[cfg(test)]
    mod not_combinator {
      use super::*;

      #[test]
      fn at_media_not_triple_negation() {
        let input = "@media not (not (not (min-width: 400px)))";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        // Triple NOT should normalize to single NOT
        match &parsed.queries {
          MediaQueryRule::Not { rule } => match rule.as_ref() {
            MediaQueryRule::Pair { key, value } => {
              assert_eq!(key, "min-width");
              match value {
                MediaRuleValue::Length(length) => {
                  assert_eq!(length.value, 400.0);
                  assert_eq!(length.unit, "px");
                }
                _ => panic!("Expected Length value"),
              }
            }
            _ => panic!("Expected Pair rule inside NOT"),
          },
          _ => panic!("Expected NOT rule for triple NOT normalization"),
        }

        assert_eq!(parsed.to_string(), "@media not (min-width: 400px)");
      }

      #[test]
      fn at_media_not_complex_and_normalization() {
        let input = "@media not ((min-width: 500px) and (max-width: 600px) and (max-width: 400px))";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        // This should create a NOT rule containing the AND rule
        match &parsed.queries {
          MediaQueryRule::Not { rule } => match rule.as_ref() {
            MediaQueryRule::And { rules } => {
              // Should contain all the original constraints
              assert_eq!(rules.len(), 3);

              // Check for min-width: 500px
              let has_min_width = rules.iter().any(|r| {
                matches!(r, MediaQueryRule::Pair { key, value } if key == "min-width" && matches!(value, MediaRuleValue::Length(l) if l.value == 500.0))
              });

              // Check for max-width: 600px
              let has_max_width_600 = rules.iter().any(|r| {
                matches!(r, MediaQueryRule::Pair { key, value } if key == "max-width" && matches!(value, MediaRuleValue::Length(l) if l.value == 600.0))
              });

              // Check for max-width: 400px
              let has_max_width_400 = rules.iter().any(|r| {
                matches!(r, MediaQueryRule::Pair { key, value } if key == "max-width" && matches!(value, MediaRuleValue::Length(l) if l.value == 400.0))
              });

              assert!(has_min_width, "Should have min-width: 500px");
              assert!(has_max_width_600, "Should have max-width: 600px");
              assert!(has_max_width_400, "Should have max-width: 400px");
            }
            _ => panic!("Expected And rule inside NOT"),
          },
          _ => panic!("Expected NOT rule"),
        }

        // The exact output will depend on our implementation's normalization
        println!("Parsed result: {}", parsed);
      }

      #[test]
      fn at_media_not_all_and_monochrome_and_min_width() {
        let input = "@media not all and (monochrome) and (min-width: 600px)";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        match &parsed.queries {
          MediaQueryRule::And { rules } => {
            assert_eq!(rules.len(), 3);

            // First rule: NOT rule containing "all"
            match &rules[0] {
              MediaQueryRule::Not { rule } => {
                match rule.as_ref() {
                  MediaQueryRule::MediaKeyword(keyword) => {
                    assert_eq!(keyword.key, "all");
                    assert!(!keyword.not); // NOT is at higher level
                    assert_eq!(keyword.only, None);
                  }
                  _ => panic!("Expected MediaKeyword inside NOT"),
                }
              }
              _ => panic!("Expected NOT rule"),
            }

            // Second rule: (monochrome)
            match &rules[1] {
              MediaQueryRule::WordRule(WordRule::Monochrome) => {}
              _ => panic!("Expected WordRule Monochrome"),
            }

            // Third rule: min-width
            match &rules[2] {
              MediaQueryRule::Pair { key, value } => {
                assert_eq!(key, "min-width");
                match value {
                  MediaRuleValue::Length(length) => {
                    assert_eq!(length.value, 600.0);
                    assert_eq!(length.unit, "px");
                  }
                  _ => panic!("Expected Length value"),
                }
              }
              _ => panic!("Expected Pair rule"),
            }
          }
          _ => panic!("Expected And rule"),
        }

        assert_eq!(
          parsed.to_string(),
          "@media not all and (monochrome) and (min-width: 600px)"
        );
      }

      #[test]
      fn at_media_complex_range_normalization() {
        let input = "@media (max-width: 1440px) and (not (max-width: 1024px)) and (not (max-width: 768px)) and (not (max-width: 458px))";
        let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

        // Should normalize to optimal range
        match &parsed.queries {
          MediaQueryRule::And { rules } => {
            assert_eq!(rules.len(), 2);

            // Should have both min-width and max-width after normalization
            let has_min = rules.iter().any(|r| {
              matches!(r, MediaQueryRule::Pair { key, value } if key == "min-width" && matches!(value, MediaRuleValue::Length(l) if l.value == 1024.01))
            });
            let has_max = rules.iter().any(|r| {
              matches!(r, MediaQueryRule::Pair { key, value } if key == "max-width" && matches!(value, MediaRuleValue::Length(l) if l.value == 1440.0))
            });
            assert!(has_min && has_max);
          }
          _ => panic!("Expected And rule after normalization"),
        }

        assert_eq!(
          parsed.to_string(),
          "@media (min-width: 1024.01px) and (max-width: 1440px)"
        );
      }
    }
  }

  #[cfg(test)]
  mod normalize_media_queries {
    use super::*;

    #[cfg(test)]
    mod flatten_and_combinator_logic {
      use super::*;

      #[test]
      fn flattens_nested_and_rules() {
        let parsed = MediaQuery::parser()
          .parse_to_end(
            "@media (min-width: 400px) and ((max-width: 700px) and (orientation: landscape))",
          )
          .unwrap();

        match &parsed.queries {
          MediaQueryRule::And { rules } => {
            assert_eq!(rules.len(), 3);

            // Should be flattened to 3 separate rules
            let keys: Vec<_> = rules
              .iter()
              .map(|r| match r {
                MediaQueryRule::Pair { key, .. } => key.clone(),
                _ => "other".to_string(),
              })
              .collect();

            assert!(keys.contains(&"min-width".to_string()));
            assert!(keys.contains(&"max-width".to_string()));
            assert!(keys.contains(&"orientation".to_string()));
          }
          _ => panic!("Expected And rule"),
        }

        assert_eq!(
          parsed.to_string(),
          "@media (orientation: landscape) and (min-width: 400px) and (max-width: 700px)"
        );
      }

      #[test]
      fn flattens_complex_nested_and_rules() {
        let parsed = MediaQuery::parser()
          .parse_to_end(
            "@media ((min-width: 400px) and ((max-width: 700px) and (orientation: landscape)))",
          )
          .unwrap();

        match &parsed.queries {
          MediaQueryRule::And { rules } => {
            assert_eq!(rules.len(), 3);

            // Should be flattened to 3 separate rules
            let keys: Vec<_> = rules
              .iter()
              .map(|r| match r {
                MediaQueryRule::Pair { key, .. } => key.clone(),
                _ => "other".to_string(),
              })
              .collect();

            assert!(keys.contains(&"min-width".to_string()));
            assert!(keys.contains(&"max-width".to_string()));
            assert!(keys.contains(&"orientation".to_string()));
          }
          _ => panic!("Expected And rule"),
        }

        assert_eq!(
          parsed.to_string(),
          "@media (orientation: landscape) and (min-width: 400px) and (max-width: 700px)"
        );
      }
    }

    #[cfg(test)]
    mod simplify_not_combinator_logic {
      use super::*;

      #[test]
      fn removes_duplicate_nots() {
        let parsed = MediaQuery::parser()
          .parse_to_end("@media not (not (min-width: 400px))")
          .unwrap();

        // Double NOT should cancel out
        match &parsed.queries {
          MediaQueryRule::Pair { key, value } => {
            assert_eq!(key, "min-width");
            match value {
              MediaRuleValue::Length(length) => {
                assert_eq!(length.value, 400.0);
                assert_eq!(length.unit, "px");
              }
              _ => panic!("Expected Length value"),
            }
          }
          _ => panic!("Expected Pair rule after double NOT cancellation"),
        }

        assert_eq!(parsed.to_string(), "@media (min-width: 400px)");
      }

      #[test]
      fn removes_triple_negation() {
        let parsed = MediaQuery::parser()
          .parse_to_end("@media not (not (not (hover: hover)))")
          .unwrap();

        // Triple NOT should become single NOT
        match &parsed.queries {
          MediaQueryRule::Not { rule } => match rule.as_ref() {
            MediaQueryRule::Pair { key, value } => {
              assert_eq!(key, "hover");
              match value {
                MediaRuleValue::String(s) => {
                  assert_eq!(s, "hover");
                }
                _ => panic!("Expected String value"),
              }
            }
            _ => panic!("Expected Pair rule inside NOT"),
          },
          _ => panic!("Expected NOT rule for triple NOT"),
        }

        assert_eq!(parsed.to_string(), "@media not (hover: hover)");
      }

      #[test]
      fn removes_even_number_of_nots() {
        let parsed = MediaQuery::parser()
          .parse_to_end("@media not (not (not (not (update: fast))))")
          .unwrap();

        // Quadruple NOT should cancel out completely
        match &parsed.queries {
          MediaQueryRule::Pair { key, value } => {
            assert_eq!(key, "update");
            match value {
              MediaRuleValue::String(s) => {
                assert_eq!(s, "fast");
              }
              _ => panic!("Expected String value"),
            }
          }
          _ => panic!("Expected Pair rule after quadruple NOT cancellation"),
        }

        assert_eq!(parsed.to_string(), "@media (update: fast)");
      }
    }

    #[cfg(test)]
    mod inequality_rule_tests {
      use super::*;

      #[test]
      fn at_media_width_greater_than_400px() {
        let parsed = MediaQuery::parser()
          .parse_to_end("@media (width > 400px)")
          .unwrap();

        match &parsed.queries {
          MediaQueryRule::Pair { key, value } => {
            assert_eq!(key, "min-width");
            match value {
              MediaRuleValue::Length(length) => {
                assert_eq!(length.value, 400.01);
                assert_eq!(length.unit, "px");
              }
              _ => panic!("Expected Length value"),
            }
          }
          _ => panic!("Expected Pair rule"),
        }

        assert_eq!(parsed.to_string(), "@media (min-width: 400.01px)");
      }

      #[test]
      fn at_media_width_greater_equal_400px() {
        let parsed = MediaQuery::parser()
          .parse_to_end("@media (width >= 400px)")
          .unwrap();

        match &parsed.queries {
          MediaQueryRule::Pair { key, value } => {
            assert_eq!(key, "min-width");
            match value {
              MediaRuleValue::Length(length) => {
                assert_eq!(length.value, 400.0); // No epsilon for >=
                assert_eq!(length.unit, "px");
              }
              _ => panic!("Expected Length value"),
            }
          }
          _ => panic!("Expected Pair rule"),
        }

        assert_eq!(parsed.to_string(), "@media (min-width: 400px)");
      }

      #[test]
      fn at_media_400px_less_than_width() {
        let parsed = MediaQuery::parser()
          .parse_to_end("@media (400px < width)")
          .unwrap();

        match &parsed.queries {
          MediaQueryRule::Pair { key, value } => {
            assert_eq!(key, "min-width");
            match value {
              MediaRuleValue::Length(length) => {
                assert_eq!(length.value, 400.01);
                assert_eq!(length.unit, "px");
              }
              _ => panic!("Expected Length value"),
            }
          }
          _ => panic!("Expected Pair rule"),
        }

        assert_eq!(parsed.to_string(), "@media (min-width: 400.01px)");
      }

      #[test]
      fn at_media_double_inequality_range() {
        let parsed = MediaQuery::parser()
          .parse_to_end("@media (400px <= width <= 700px)")
          .unwrap();

        match &parsed.queries {
          MediaQueryRule::And { rules } => {
            assert_eq!(rules.len(), 2);

            // Should have both min-width and max-width
            let has_min = rules.iter().any(|r| {
              matches!(r, MediaQueryRule::Pair { key, value } if key == "min-width" && matches!(value, MediaRuleValue::Length(l) if l.value == 400.0))
            });
            let has_max = rules.iter().any(|r| {
              matches!(r, MediaQueryRule::Pair { key, value } if key == "max-width" && matches!(value, MediaRuleValue::Length(l) if l.value == 700.0))
            });
            assert!(has_min && has_max);
          }
          _ => panic!("Expected And rule"),
        }

        assert_eq!(
          parsed.to_string(),
          "@media (min-width: 400px) and (max-width: 700px)"
        );
      }
    }
  }

  #[cfg(test)]
  mod simplify_range_intervals {
    use super::*;

    #[test]
    fn min_width_and_not_max_width() {
      let parsed = MediaQuery::parser()
        .parse_to_end("@media (min-width: 100px) and (not (max-width: 200px))")
        .unwrap();

      // Should simplify to min-width only with adjusted value
      assert_eq!(parsed.to_string(), "@media (min-width: 200.01px)");
    }

    #[test]
    fn min_width_and_min_width_takes_larger() {
      let parsed = MediaQuery::parser()
        .parse_to_end("@media (min-width: 100px) and (min-width: 200px)")
        .unwrap();

      // Should simplify to the larger min-width
      assert_eq!(parsed.to_string(), "@media (min-width: 200px)");
    }

    #[test]
    fn complex_max_width_negations() {
      let parsed = MediaQuery::parser()
        .parse_to_end("@media (max-width: 1440px) and (not (max-width: 1024px)) and (not (max-width: 768px)) and (not (max-width: 458px))")
        .unwrap();

      // Should simplify to optimal range
      assert_eq!(
        parsed.to_string(),
        "@media (min-width: 1024.01px) and (max-width: 1440px)"
      );
    }
  }

  #[cfg(test)]
  mod additional_media_query_features {
    use super::*;

    #[test]
    fn webkit_min_device_pixel_ratio() {
      let input = "@media (-webkit-min-device-pixel-ratio: 2)";
      let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

      match &parsed.queries {
        MediaQueryRule::Pair { key, value } => {
          assert_eq!(key, "-webkit-min-device-pixel-ratio");
          match value {
            MediaRuleValue::Number(n) => {
              assert_eq!(*n, 2.0);
            }
            _ => panic!("Expected Number value"),
          }
        }
        _ => panic!("Expected Pair rule"),
      }

      assert_eq!(
        parsed.to_string(),
        "@media (-webkit-min-device-pixel-ratio: 2)"
      );
    }

    #[test]
    fn color_gamut_p3() {
      let input = "@media (color-gamut: p3)";
      let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

      match &parsed.queries {
        MediaQueryRule::Pair { key, value } => {
          assert_eq!(key, "color-gamut");
          match value {
            MediaRuleValue::String(s) => {
              assert_eq!(s, "p3");
            }
            _ => panic!("Expected String value"),
          }
        }
        _ => panic!("Expected Pair rule"),
      }

      assert_eq!(parsed.to_string(), "@media (color-gamut: p3)");
    }

    #[test]
    fn resolution_2dppx() {
      let input = "@media (resolution: 2dppx)";
      let parsed = MediaQuery::parser().parse_to_end(input).unwrap();

      match &parsed.queries {
        MediaQueryRule::Pair { key, value } => {
          assert_eq!(key, "resolution");
          match value {
            MediaRuleValue::Length(length) => {
              assert_eq!(length.value, 2.0);
              assert_eq!(length.unit, "dppx");
            }
            _ => panic!("Expected Length value"),
          }
        }
        _ => panic!("Expected Pair rule"),
      }

      assert_eq!(parsed.to_string(), "@media (resolution: 2dppx)");
    }
  }
}
