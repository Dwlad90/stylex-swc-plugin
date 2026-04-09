/// Snapshot-based test macro (delegates to SWC's `test!`).
///
/// ```ignore
/// stylex_test!(name, r#"code"#);                    // default
/// stylex_test!(name, |tr| custom_pass(), r#"code"#); // closure
/// stylex_test!(name, &mut opts, r#"code"#);          // options
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

  // Closure arm MUST come before options — closures are valid expressions
  // and `$options:expr` would eagerly match them.
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

/// Transform test macro with expected output (input + expected).
///
/// Supports optional `#[ignore]` as the first argument:
/// ```ignore
/// stylex_test_transform!(name, r#"in"#, r#"out"#);
/// stylex_test_transform!(name, |tr| custom(), r#"in"#, r#"out"#);
/// stylex_test_transform!(name, &mut opts, r#"in"#, r#"out"#);
/// stylex_test_transform!(#[ignore], name, r#"in"#, r#"out"#);
/// ```
#[allow(unused_macros)]
macro_rules! stylex_test_transform {
  // --- Internal dispatch (3 modes) — must precede catch-all entry ---

  (@impl [$($attrs:tt)*] $name:ident, $input:expr, $expected:expr) => {
    #[test]
    $($attrs)*
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

  // Closure MUST come before options — closures match `$options:expr`.
  (@impl [$($attrs:tt)*] $name:ident, |$tr:ident| $transform:expr, $input:expr, $expected:expr) => {
    #[test]
    $($attrs)*
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

  (@impl [$($attrs:tt)*] $name:ident, $options:expr, $input:expr, $expected:expr) => {
    #[test]
    $($attrs)*
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

  // --- Entry points ---

  (#[ignore $($reason:tt)*], $($rest:tt)*) => {
    stylex_test_transform!(@impl [#[ignore $($reason)*]] $($rest)*);
  };
  ($($rest:tt)*) => {
    stylex_test_transform!(@impl [] $($rest)*);
  };
}

/// Validation test macro (`#[should_panic]`).
///
/// Supports optional `#[ignore]` as the first argument:
/// ```ignore
/// stylex_test_panic!(name, "msg", r#"code"#);
/// stylex_test_panic!(name, "msg", |tr| custom(), r#"code"#);
/// stylex_test_panic!(name, "msg", &mut opts, r#"code"#);
/// stylex_test_panic!(#[ignore], name, "msg", r#"code"#);
/// ```
#[allow(unused_macros)]
macro_rules! stylex_test_panic {
  // --- Internal dispatch (3 modes) — must precede catch-all entry ---

  (@impl [$($attrs:tt)*] $name:ident, $panic_msg:expr, $code:expr) => {
    #[test]
    $($attrs)*
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

  // Closure MUST come before options — closures match `$options:expr`.
  (@impl [$($attrs:tt)*] $name:ident, $panic_msg:expr, |$tr:ident| $transform:expr, $code:expr) => {
    #[test]
    $($attrs)*
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

  (@impl [$($attrs:tt)*] $name:ident, $panic_msg:expr, $options:expr, $code:expr) => {
    #[test]
    $($attrs)*
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

  // --- Entry points ---

  (#[ignore $($reason:tt)*], $($rest:tt)*) => {
    stylex_test_panic!(@impl [#[ignore $($reason)*]] $($rest)*);
  };
  ($($rest:tt)*) => {
    stylex_test_panic!(@impl [] $($rest)*);
  };
}
