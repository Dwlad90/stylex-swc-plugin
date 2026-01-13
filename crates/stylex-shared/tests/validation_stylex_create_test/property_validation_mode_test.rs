use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{PropertyValidationMode, StyleXOptionsParams},
  },
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

// Test default behavior (silent mode)
#[test]
fn does_not_throw_by_default_for_disallowed_properties_silent_mode() {
  // This test just ensures no panic occurs - the transformation will remove the
  // disallowed properties, resulting in an empty styles object
  let result = std::panic::catch_unwind(|| {
    test_transform(
      Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
      }),
      Option::None,
      |tr| {
        StyleXTransform::new_test_force_runtime_injection_with_pass(
          tr.comments.clone(),
          PluginPass::default(),
          None,
        )
      },
      r#"
        import * as stylex from '@stylexjs/stylex';
        const styles = stylex.create({
          root: {
            border: '1px solid red',
          },
        });
      "#,
      r#"
        import * as stylex from '@stylexjs/stylex';
      "#,
    )
  });

  assert!(result.is_ok(), "Test should not panic");
}

// Test throw mode
#[test]
#[should_panic(expected = "is not supported")]
fn throws_error_when_property_validation_mode_is_throw() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      let mut config = StyleXOptionsParams {
        property_validation_mode: Some(PropertyValidationMode::Throw),
        ..StyleXOptionsParams::default()
      };

      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        Some(&mut config),
      )
    },
    r#"
      import * as stylex from '@stylexjs/stylex';
      const styles = stylex.create({
        root: {
          border: '1px solid red',
        },
      });
    "#,
    r#""#,
  )
}

// Test warn mode
#[test]
fn does_not_throw_when_property_validation_mode_is_warn() {
  // This test ensures no panic occurs and warnings are printed
  let result = std::panic::catch_unwind(|| {
    test_transform(
      Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
      }),
      Option::None,
      |tr| {
        let mut config = StyleXOptionsParams {
          property_validation_mode: Some(PropertyValidationMode::Warn),
          ..StyleXOptionsParams::default()
        };

        StyleXTransform::new_test_force_runtime_injection_with_pass(
          tr.comments.clone(),
          PluginPass::default(),
          Some(&mut config),
        )
      },
      r#"
        import * as stylex from '@stylexjs/stylex';
        const styles = stylex.create({
          root: {
            border: '1px solid red',
          },
        });
      "#,
      r#"
        import * as stylex from '@stylexjs/stylex';
      "#,
    )
  });

  assert!(result.is_ok(), "Test should not panic in warn mode");
}

// Test silent mode explicitly
#[test]
fn does_not_throw_when_property_validation_mode_is_silent() {
  let result = std::panic::catch_unwind(|| {
    test_transform(
      Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
      }),
      Option::None,
      |tr| {
        let mut config = StyleXOptionsParams {
          property_validation_mode: Some(PropertyValidationMode::Silent),
          ..StyleXOptionsParams::default()
        };

        StyleXTransform::new_test_force_runtime_injection_with_pass(
          tr.comments.clone(),
          PluginPass::default(),
          Some(&mut config),
        )
      },
      r#"
        import * as stylex from '@stylexjs/stylex';
        const styles = stylex.create({
          root: {
            border: '1px solid red',
          },
        });
      "#,
      r#"
        import * as stylex from '@stylexjs/stylex';
      "#,
    )
  });

  assert!(result.is_ok(), "Test should not panic in silent mode");
}

// Test with background property
#[test]
fn works_with_background_property() {
  let result = std::panic::catch_unwind(|| {
    test_transform(
      Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
      }),
      Option::None,
      |tr| {
        let mut config = StyleXOptionsParams {
          property_validation_mode: Some(PropertyValidationMode::Silent),
          ..StyleXOptionsParams::default()
        };

        StyleXTransform::new_test_force_runtime_injection_with_pass(
          tr.comments.clone(),
          PluginPass::default(),
          Some(&mut config),
        )
      },
      r#"
        import * as stylex from '@stylexjs/stylex';
        const styles = stylex.create({
          root: {
            background: 'red',
          },
        });
      "#,
      r#"
        import * as stylex from '@stylexjs/stylex';
      "#,
    )
  });

  assert!(
    result.is_ok(),
    "Test should not panic with background property"
  );
}

// Test with animation property
#[test]
fn works_with_animation_property() {
  let result = std::panic::catch_unwind(|| {
    test_transform(
      Syntax::Typescript(TsSyntax {
        tsx: true,
        ..Default::default()
      }),
      Option::None,
      |tr| {
        let mut config = StyleXOptionsParams {
          property_validation_mode: Some(PropertyValidationMode::Silent),
          ..StyleXOptionsParams::default()
        };

        StyleXTransform::new_test_force_runtime_injection_with_pass(
          tr.comments.clone(),
          PluginPass::default(),
          Some(&mut config),
        )
      },
      r#"
        import * as stylex from '@stylexjs/stylex';
        const styles = stylex.create({
          root: {
            animation: 'spin 1s',
          },
        });
      "#,
      r#"
        import * as stylex from '@stylexjs/stylex';
      "#,
    )
  });

  assert!(
    result.is_ok(),
    "Test should not panic with animation property"
  );
}

// Test throw mode with background
#[test]
#[should_panic(expected = "is not supported")]
fn throws_for_background_in_throw_mode() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      let mut config = StyleXOptionsParams {
        property_validation_mode: Some(PropertyValidationMode::Throw),
        ..StyleXOptionsParams::default()
      };

      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        Some(&mut config),
      )
    },
    r#"
      import * as stylex from '@stylexjs/stylex';
      const styles = stylex.create({
        root: {
          background: 'red',
        },
      });
    "#,
    r#""#,
  )
}

// Test throw mode with animation
#[test]
#[should_panic(expected = "is not supported")]
fn throws_for_animation_in_throw_mode() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
    |tr| {
      let mut config = StyleXOptionsParams {
        property_validation_mode: Some(PropertyValidationMode::Throw),
        ..StyleXOptionsParams::default()
      };

      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass::default(),
        Some(&mut config),
      )
    },
    r#"
      import * as stylex from '@stylexjs/stylex';
      const styles = stylex.create({
        root: {
          animation: 'spin 1s',
        },
      });
    "#,
    r#""#,
  )
}
