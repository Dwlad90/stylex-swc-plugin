use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::test_transform;

stylex_test_panic!(
  validates_pseudo_selector_format,
  "Pseudo selector must start with \":\" or \"[\"",
  r#"
      import { when, create } from '@stylexjs/stylex';

      const styles = create({
        container: {
          backgroundColor: {
            default: 'blue',
            [when.ancestor('hover')]: 'red',
          },
        },
      });
    "#
);

#[test]
#[should_panic(expected = "Attribute selector must end with \"]\"")]
fn validates_attribute_selector_format() {
  test_transform(
    ts_syntax(),
    Option::None,
    |tr| {
      StyleXTransform::test(tr.comments.clone())
        .with_runtime_injection()
        .into_pass()
    },
    r#"
      import { when, create } from '@stylexjs/stylex';

      const styles = create({
        container: {
          backgroundColor: {
            default: 'blue',
            [when.ancestor('[data-state="open"')]: 'red',
          },
        },
      });
    "#,
    r#""#,
  )
}

stylex_test_panic!(
  rejects_invalid_selector_format,
  "Pseudo selector must start with \":\" or \"[\"",
  r#"
      import { when, create } from '@stylexjs/stylex';

      const styles = create({
        container: {
          backgroundColor: {
            default: 'blue',
            [when.ancestor('invalid')]: 'red',
          },
        },
      });
    "#
);

stylex_test_panic!(
  rejects_pseudo_elements,
  "Pseudo selector cannot start with \"::\"",
  r#"
      import { when, create } from '@stylexjs/stylex';

      const styles = create({
        container: {
          backgroundColor: {
            default: 'blue',
            [when.ancestor('::before')]: 'red',
          },
        },
      });
    "#
);
