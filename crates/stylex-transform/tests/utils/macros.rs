/// Unified test macro for StyleX transform tests.
///
/// # Arms
///
/// **Arm 1** — Default with runtime injection (~75% of tests):
/// ```ignore
/// stylex_test!(test_name, r#"input code"#);
/// ```
///
/// **Arm 2** — With custom options + runtime injection:
/// ```ignore
/// stylex_test!(test_name, &mut options, r#"input code"#);
/// ```
///
/// **Arm 3** — Full custom transform via closure (escape hatch):
/// ```ignore
/// stylex_test!(test_name, |tr| StyleXTransform::test(tr.comments.clone()).into_pass(), r#"input code"#);
/// ```
macro_rules! stylex_test {
  ($name:ident, $code:expr) => {
    test!(
      $crate::utils::transform::ts_syntax(),
      |tr| StyleXTransform::test(tr.comments.clone())
        .with_runtime_injection()
        .into_pass(),
      $name,
      $code
    );
  };

  ($name:ident, $options:expr, $code:expr) => {
    test!(
      $crate::utils::transform::ts_syntax(),
      |tr| StyleXTransform::test(tr.comments.clone())
        .with_options($options)
        .with_runtime_injection()
        .into_pass(),
      $name,
      $code
    );
  };

  ($name:ident, |$tr:ident| $transform:expr, $code:expr) => {
    test!(
      $crate::utils::transform::ts_syntax(),
      |$tr| $transform,
      $name,
      $code
    );
  };
}
