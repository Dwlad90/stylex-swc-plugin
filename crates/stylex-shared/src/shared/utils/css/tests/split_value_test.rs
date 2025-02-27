#[cfg(test)]
mod ensure_css_values_are_split_correctly {
  use crate::shared::utils::css::common::split_value;

  #[test]
  fn simple_space_separated_numbers() {
    assert_eq!(
      split_value(Some("0 1 2 3"), None),
      (
        "0".into(),
        Some("1".into()),
        Some("2".into()),
        Some("3".into())
      )
    );
  }

  #[test]
  fn simple_space_separated_lengths() {
    assert_eq!(
      split_value(Some("0px 1rem 2% 3em"), None),
      (
        "0px".into(),
        Some("1rem".into()),
        Some("2%".into()),
        Some("3em".into())
      )
    );
  }

  #[test]
  fn simple_comma_separated_numbers() {
    assert_eq!(
      split_value(Some("0, 1, 2, 3"), None),
      (
        "0".into(),
        Some("1".into()),
        Some("2".into()),
        Some("3".into())
      )
    );
  }

  #[test]
  fn simple_comma_separated_lengths() {
    assert_eq!(
      split_value(Some("0px, 1rem, 2%, 3em"), None),
      (
        "0px".into(),
        Some("1rem".into()),
        Some("2%".into()),
        Some("3em".into())
      )
    );
  }

  #[test]
  fn does_not_lists_within_functions() {
    assert_eq!(
      split_value(Some("rgb(255 200 0)"), None),
      ("rgb(255 200 0)".into(), None, None, None)
    );

    assert_eq!(
      split_value(Some("rgb(255 200 / 0.5)"), None),
      ("rgb(255 200/0.5)".into(), None, None, None)
    );
  }

  #[test]
  fn does_not_lists_within_calc() {
    assert_eq!(
      split_value(Some("calc((100% - 50px) * 0.5)"), None),
      ("calc((100% - 50px) * 0.5)".into(), None, None, None)
    );

    assert_eq!(
      split_value(
        Some("calc((100% - 50px) * 0.5) var(--rightpadding, 20px)"),
        None
      ),
      (
        "calc((100% - 50px) * 0.5)".into(),
        Some("var(--rightpadding,20px)".into()),
        None,
        None
      )
    );
  }

  #[test]
  fn expands_a_string_of_values_with_slash_notation_appropriately() {
    assert_eq!(
      split_value(Some("1px / 2px 3px 4px 5px"), Some("borderRadius")),
      (
        "1px 2px".into(),
        Some("1px 3px".into()),
        Some("1px 4px".into()),
        Some("1px 5px".into())
      )
    );

    assert_eq!(
      split_value(Some("1px 2px / 3px 4px"), Some("borderRadius")),
      (
        "1px 3px".into(),
        Some("2px 4px".into()),
        Some("1px 3px".into()),
        Some("2px 4px".into())
      )
    );

    assert_eq!(
      split_value(Some("1px 2px / 3px 4px 5px"), Some("borderRadius")),
      (
        "1px 3px".into(),
        Some("2px 4px".into()),
        Some("1px 5px".into()),
        Some("2px 4px".into())
      )
    );

    assert_eq!(
      split_value(
        Some("1px 2px 3px 4px / 5px 6px 7px 8px"),
        Some("borderRadius")
      ),
      (
        "1px 5px".into(),
        Some("2px 6px".into()),
        Some("3px 7px".into()),
        Some("4px 8px".into())
      )
    );
  }
}
