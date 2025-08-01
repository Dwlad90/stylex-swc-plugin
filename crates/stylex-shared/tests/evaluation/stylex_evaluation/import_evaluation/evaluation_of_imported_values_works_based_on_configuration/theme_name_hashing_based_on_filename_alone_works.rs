use std::env;

use insta::assert_snapshot;
use stylex_shared::shared::structures::stylex_options::{StyleXOptions, StyleXOptionsParams};
use stylex_shared::shared::utils::common::create_hash;
use stylex_shared::{StyleXTransform, shared::structures::plugin_pass::PluginPass};
use swc_core::common::FileName;
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

use crate::utils::transform::stringify_js;

struct Options {
  class_name_prefix: &'static str,
}

static OPTIONS: Options = Options {
  class_name_prefix: "__hashed_var__",
};

fn tranform(input: &str) -> String {
  stringify_js(
    input,
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      let mut config = StyleXOptionsParams {
        class_name_prefix: Some("__hashed_var__".to_string()),
        runtime_injection: Some(true),
        treeshake_compensation: Some(true),
        unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
        ..Default::default()
      };

      StyleXTransform::new_test_force_runtime_injection_with_pass(
        tr.comments.clone(),
        PluginPass {
          filename: FileName::Real(
            format!("{}/test.skip.js", env::current_dir().unwrap().display()).into(),
          ),
          ..Default::default()
        },
        Some(&mut config),
      )
    },
  )
}

#[test]
fn importing_file_with_stylex_suffix_works() {
  let input = r#"import stylex from 'stylex';
    import { MyTheme } from 'otherFile.stylex';
    const styles = stylex.create({
        red: {
            color: MyTheme.foreground,
        }
    });
    stylex(styles.red);"#;

  let transformation = tranform(input);

  let expected_var_name = format!(
    "var(--{}{})",
    OPTIONS.class_name_prefix,
    create_hash("otherFile.stylex.js//MyTheme.foreground")
  );

  assert_eq!(expected_var_name, "var(--__hashed_var__1jqb1tb)");

  assert!(transformation.contains(&expected_var_name));

  assert_snapshot!(transformation);
}

#[test]
fn importing_file_with_dot_stylex_and_reading_var_group_hash_returns_a_class_name() {
  let input = r#"import stylex from 'stylex';
        import { MyTheme } from 'otherFile.stylex';
        const styles = stylex.create({
          red: {
            color: MyTheme.__varGroupHash__,
          }
        });
        stylex(styles.red);"#;

  let transformation = tranform(input);

  let expected_var_name = format!(
    "{}{}",
    OPTIONS.class_name_prefix,
    create_hash("otherFile.stylex.js//MyTheme")
  );

  assert_eq!(expected_var_name, "__hashed_var__jvfbhb");

  assert!(transformation.contains(&expected_var_name));

  assert_snapshot!(transformation);
}

#[test]
fn maintains_variable_names_that_start_with_double_dash_from_stylex_files() {
  let input = r#"import stylex from 'stylex';
        import { MyTheme } from 'otherFile.stylex';
        const styles = stylex.create({
          red: {
            color: MyTheme['--foreground'],
          }
        });
        stylex(styles.red);"#;

  let transformation = tranform(input);

  let expected_var_name = "var(--foreground)";

  assert_eq!(expected_var_name, "var(--foreground)");

  assert!(transformation.contains(expected_var_name));

  assert_snapshot!(transformation);
}

#[test]
fn importing_file_with_stylex_suffix_works_with_keyframes() {
  let input = r#"import stylex from 'stylex';
    import { MyTheme } from 'otherFile.stylex';
    export const fade = stylex.keyframes({
        from: {
            color: MyTheme.foreground,
        }
    });
    const styles = stylex.create({
        red: {
            animationName: fade,
        }
    });
    stylex(styles.red);"#;

  let transformation = tranform(input);

  let expected_var_name = format!(
    "var(--{}{})",
    OPTIONS.class_name_prefix,
    create_hash("otherFile.stylex.js//MyTheme.foreground")
  );

  assert_eq!(expected_var_name, "var(--__hashed_var__1jqb1tb)");

  assert!(transformation.contains(&expected_var_name));

  assert_snapshot!(transformation);
}

#[test]
fn importing_file_with_stylex_js_suffix_works() {
  let input = r#"import stylex from 'stylex';
    import { MyTheme } from 'otherFile.stylex.js';
    const styles = stylex.create({
        red: {
            color: MyTheme.foreground,
        }
    });
    stylex(styles.red);"#;

  let transformation = tranform(input);

  let expected_var_name = format!(
    "var(--{}{})",
    OPTIONS.class_name_prefix,
    create_hash("otherFile.stylex.js//MyTheme.foreground")
  );

  assert_eq!(expected_var_name, "var(--__hashed_var__1jqb1tb)");

  assert!(transformation.contains(&expected_var_name));

  assert_snapshot!(transformation);
}

#[test]
fn importing_file_with_stylex_js_with_an_alias_suffix_works() {
  let input = r#"import stylex from 'stylex';
    import { MyTheme as mt } from 'otherFile.stylex.js';
    const styles = stylex.create({
        red: {
            color: mt.foreground,
        }
    });
    stylex(styles.red);"#;

  let transformation = tranform(input);

  let expected_var_name = format!(
    "var(--{}{})",
    OPTIONS.class_name_prefix,
    create_hash("otherFile.stylex.js//MyTheme.foreground")
  );

  assert_eq!(expected_var_name, "var(--__hashed_var__1jqb1tb)");

  assert!(transformation.contains(&expected_var_name));

  assert_snapshot!(transformation);
}

#[test]
#[should_panic(
  expected = "Could not resolve the path to the imported file.\nPlease ensure that the theme file has a .stylex.js or .stylex.ts extension and follows the\nrules for defining variables:\n\nhttps://stylexjs.com/docs/learn/theming/defining-variables/#rules-when-defining-variables"
)]
fn importing_file_without_a_stylex_suffix_fails() {
  let input = r#"import stylex from 'stylex';
    import { MyTheme } from 'otherFile';
    const styles = stylex.create({
        red: {
            color: MyTheme.foreground,
        }
    });
    stylex(styles.red);"#;

  tranform(input);
}

#[test]
fn imported_vars_with_stylex_suffix_can_be_used_as_style_keys() {
  let input = r#"import stylex from 'stylex';
    import { MyTheme } from 'otherFile.stylex';
    const styles = stylex.create({
        red: {
            [MyTheme.foreground]: 'red',
        }
    });
    stylex(styles.red);"#;

  let transformation = tranform(input);

  assert_snapshot!(transformation);
}

#[test]
fn imported_vars_with_stylex_suffix_can_be_used_as_style_keys_dynamically() {
  let input = r#"import stylex from 'stylex';
    import { MyTheme } from 'otherFile.stylex';
    export const styles = stylex.create({
        color: (color) => ({
            [MyTheme.foreground]: color,
        })
    });
    stylex.props(styles.color('red'));"#;

  let transformation = tranform(input);

  assert_snapshot!(transformation);
}

#[test]
fn imported_vars_with_stylex_suffix_used_as_renamed_style_keys_dynamically() {
  let input = r#"import stylex from 'stylex';
    import { theme as MyTheme } from 'otherFile.stylex';
    export const styles = stylex.create({
        color: (color) => ({
            [MyTheme.foreground]: color,
        })
    });
    stylex.props(styles.color('red'));"#;

  let transformation = tranform(input);

  assert_snapshot!(transformation);
}
