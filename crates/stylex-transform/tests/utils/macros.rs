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

/// Unified test macro for transform tests with expected output (`test_transform`).
///
/// Uses `test_transform` (input + expected output) instead of snapshot-based `test!`.
///
/// # Arms
///
/// **Arm 1** — Default runtime injection:
/// ```ignore
/// stylex_test_transform!(test_name, r#"input"#, r#"expected"#);
/// ```
///
/// **Arm 2** — Custom closure:
/// ```ignore
/// stylex_test_transform!(test_name, |tr| custom_pass(), r#"input"#, r#"expected"#);
/// ```
///
/// **Arm 3** — Custom options + runtime injection:
/// ```ignore
/// stylex_test_transform!(test_name, &mut opts, r#"input"#, r#"expected"#);
/// ```
///
/// **Arm 4** — `#[ignore]` + default runtime injection:
/// ```ignore
/// stylex_test_transform!(#[ignore], test_name, r#"input"#, r#"expected"#);
/// ```
///
/// **Arm 5** — `#[ignore]` + custom closure:
/// ```ignore
/// stylex_test_transform!(#[ignore], test_name, |tr| custom_pass(), r#"input"#, r#"expected"#);
/// ```
///
/// **Arm 6** — `#[ignore]` + custom options + runtime injection:
/// ```ignore
/// stylex_test_transform!(#[ignore], test_name, &mut opts, r#"input"#, r#"expected"#);
/// ```
macro_rules! stylex_test_transform {
  // Arm 1: default runtime injection
  ($name:ident, $input:expr, $expected:expr) => {
    #[test]
    fn $name() {
      test_transform(
        $crate::utils::transform::ts_syntax(),
        Option::None,
        |tr| {
          StyleXTransform::test(tr.comments.clone())
            .with_runtime_injection()
            .into_pass()
        },
        $input,
        $expected,
      );
    }
  };

  // Arm 2 (closure): custom closure — must come before Arm 3 (options)
  ($name:ident, |$tr:ident| $transform:expr, $input:expr, $expected:expr) => {
    #[test]
    fn $name() {
      test_transform(
        $crate::utils::transform::ts_syntax(),
        Option::None,
        |$tr| $transform,
        $input,
        $expected,
      );
    }
  };

  // Arm 3: custom options + runtime injection
  ($name:ident, $options:expr, $input:expr, $expected:expr) => {
    #[test]
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
        $input,
        $expected,
      );
    }
  };

  // Arm 4: #[ignore] + default runtime injection
  (#[ignore $($reason:tt)*], $name:ident, $input:expr, $expected:expr) => {
    #[test]
    #[ignore $($reason)*]
    fn $name() {
      test_transform(
        $crate::utils::transform::ts_syntax(),
        Option::None,
        |tr| {
          StyleXTransform::test(tr.comments.clone())
            .with_runtime_injection()
            .into_pass()
        },
        $input,
        $expected,
      );
    }
  };

  // Arm 5 (closure): #[ignore] + custom closure — must come before Arm 6 (options)
  (#[ignore $($reason:tt)*], $name:ident, |$tr:ident| $transform:expr, $input:expr, $expected:expr) => {
    #[test]
    #[ignore $($reason)*]
    fn $name() {
      test_transform(
        $crate::utils::transform::ts_syntax(),
        Option::None,
        |$tr| $transform,
        $input,
        $expected,
      );
    }
  };

  // Arm 6: #[ignore] + custom options + runtime injection
  (#[ignore $($reason:tt)*], $name:ident, $options:expr, $input:expr, $expected:expr) => {
    #[test]
    #[ignore $($reason)*]
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
        $input,
        $expected,
      );
    }
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
