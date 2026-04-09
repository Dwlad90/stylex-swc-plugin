/// Unified test macro for StyleX transform tests.
///
/// # Arms
///
/// **Arm 1** — Default with runtime injection (~75% of tests):
/// ```ignore
/// stylex_test!(test_name, r#"input code"#);
/// ```
///
/// **Arm 2** — Full custom transform via closure (escape hatch):
/// ```ignore
/// stylex_test!(test_name, |tr| StyleXTransform::test(tr.comments.clone()).into_pass(), r#"input code"#);
/// ```
///
/// **Arm 3** — With custom options + runtime injection:
/// ```ignore
/// stylex_test!(test_name, &mut options, r#"input code"#);
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

  // Closure arm MUST come before options arm — closures are valid expressions
  // and `$options:expr` would eagerly match them otherwise.
  ($name:ident, |$tr:ident| $transform:expr, $code:expr) => {
    test!(
      $crate::utils::transform::ts_syntax(),
      |$tr| $transform,
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
}

/// Unified test macro for StyleX validation tests (`#[should_panic]`).
///
/// Uses `test_transform` instead of SWC's `test!` since `test!` doesn't
/// support `#[should_panic]`.
macro_rules! stylex_test_panic {
  // Arm 1: default runtime injection
  ($name:ident, $panic_msg:expr, $code:expr) => {
    #[test]
    #[should_panic(expected = $panic_msg)]
    fn $name() {
      test_transform(
        $crate::utils::transform::ts_syntax(),
        Option::None,
        |tr| {
          StyleXTransform::test(tr.comments.clone())
            .with_runtime_injection()
            .into_pass()
        },
        $code,
        $code,
      );
    }
  };

  // Arm 2: #[ignore] + default runtime injection
  (#[ignore $($reason:tt)*], $name:ident, $panic_msg:expr, $code:expr) => {
    #[test]
    #[ignore $($reason)*]
    #[should_panic(expected = $panic_msg)]
    fn $name() {
      test_transform(
        $crate::utils::transform::ts_syntax(),
        Option::None,
        |tr| {
          StyleXTransform::test(tr.comments.clone())
            .with_runtime_injection()
            .into_pass()
        },
        $code,
        $code,
      );
    }
  };

  // Arm 3 (closure): custom closure — must come before Arm 5 (options)
  ($name:ident, $panic_msg:expr, |$tr:ident| $transform:expr, $code:expr) => {
    #[test]
    #[should_panic(expected = $panic_msg)]
    fn $name() {
      test_transform(
        $crate::utils::transform::ts_syntax(),
        Option::None,
        |$tr| $transform,
        $code,
        $code,
      );
    }
  };

  // Arm 4 (closure): #[ignore] + custom closure
  (#[ignore $($reason:tt)*], $name:ident, $panic_msg:expr, |$tr:ident| $transform:expr, $code:expr) => {
    #[test]
    #[ignore $($reason)*]
    #[should_panic(expected = $panic_msg)]
    fn $name() {
      test_transform(
        $crate::utils::transform::ts_syntax(),
        Option::None,
        |$tr| $transform,
        $code,
        $code,
      );
    }
  };

  // Arm 5: custom options + runtime injection
  ($name:ident, $panic_msg:expr, $options:expr, $code:expr) => {
    #[test]
    #[should_panic(expected = $panic_msg)]
    fn $name() {
      test_transform(
        $crate::utils::transform::ts_syntax(),
        Option::None,
        |tr| {
          StyleXTransform::test(tr.comments.clone())
            .with_options($options)
            .with_runtime_injection()
            .into_pass()
        },
        $code,
        $code,
      );
    }
  };

  // Arm 6: #[ignore] + custom options + runtime injection
  (#[ignore $($reason:tt)*], $name:ident, $panic_msg:expr, $options:expr, $code:expr) => {
    #[test]
    #[ignore $($reason)*]
    #[should_panic(expected = $panic_msg)]
    fn $name() {
      test_transform(
        $crate::utils::transform::ts_syntax(),
        Option::None,
        |tr| {
          StyleXTransform::test(tr.comments.clone())
            .with_options($options)
            .with_runtime_injection()
            .into_pass()
        },
        $code,
        $code,
      );
    }
  };
}
