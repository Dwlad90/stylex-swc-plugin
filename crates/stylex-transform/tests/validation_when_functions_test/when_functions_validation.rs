use crate::utils::prelude::*;

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

stylex_test_panic!(
  validates_attribute_selector_format,
  "Attribute selector must end with \"]\"",
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
  "#
);

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
