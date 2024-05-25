#[cfg(test)]
mod ensure_css_values_are_split_correctly {
  use crate::shared::utils::css::common::split_value;

  #[test]
  fn simple_space_separated_numbers() {
    assert_eq!(
      split_value(Option::Some("0 1 2 3")),
      (
        "0".into(),
        Option::Some("1".into()),
        Option::Some("2".into()),
        Option::Some("3".into())
      )
    );
  }

  #[test]
  fn simple_space_separated_lengths() {
    assert_eq!(
      split_value(Option::Some("0px 1rem 2% 3em")),
      (
        "0px".into(),
        Option::Some("1rem".into()),
        Option::Some("2%".into()),
        Option::Some("3em".into())
      )
    );
  }

  #[test]
  fn simple_comma_separated_numbers() {
    assert_eq!(
      split_value(Option::Some("0, 1, 2, 3")),
      (
        "0".into(),
        Option::Some("1".into()),
        Option::Some("2".into()),
        Option::Some("3".into())
      )
    );
  }

  #[test]
  fn simple_comma_separated_lengths() {
    assert_eq!(
      split_value(Option::Some("0px, 1rem, 2%, 3em")),
      (
        "0px".into(),
        Option::Some("1rem".into()),
        Option::Some("2%".into()),
        Option::Some("3em".into())
      )
    );
  }

  #[test]
  fn does_not_lists_within_functions() {
    assert_eq!(
      split_value(Option::Some("rgb(255 200 0)")),
      (
        "rgb(255 200 0)".into(),
        Option::None,
        Option::None,
        Option::None
      )
    );

    assert_eq!(
      split_value(Option::Some("rgb(255 200 / 0.5)")),
      (
        "rgb(255 200/0.5)".into(),
        Option::None,
        Option::None,
        Option::None
      )
    );
  }

  #[test]
  fn does_not_lists_within_calc() {
    assert_eq!(
      split_value(Option::Some("calc((100% - 50px) * 0.5)")),
      (
        "calc((100% - 50px) * 0.5)".into(),
        Option::None,
        Option::None,
        Option::None
      )
    );

    assert_eq!(
      split_value(Option::Some(
        "calc((100% - 50px) * 0.5) var(--rightpadding, 20px)"
      )),
      (
        "calc((100% - 50px) * 0.5)".into(),
        Option::Some("var(--rightpadding,20px)".into()),
        Option::None,
        Option::None
      )
    );
  }
}
