use crate::utils::prelude::*;

stylex_test!(
  keyframes_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.keyframes({
      from: {
        color: 'red',
      },
      to: {
        color: 'blue',
      }
    });
  "#
);

stylex_test!(
  local_variables_used_in_keyframes_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const COLOR = 'red';
    export const name = stylex.keyframes({
      from: {
        color: COLOR,
      },
      to: {
        color: 'blue',
      }
    });
  "#
);

stylex_test!(
  template_literals_used_in_keyframes_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const COLOR = 'red';
    const name = stylex.keyframes({
      from: {
        color: COLOR,
      },
      to: {
        color: 'blue',
      }
    });
    export const styles = stylex.create({
      root: {
        animationName: `${name}`,
      }
    });
  "#
);

stylex_test!(
  keyframes_object_used_inline,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      root: {
        animationName: stylex.keyframes({
          from: {
            color: 'red',
          },
          to: {
            color: 'blue',
          },
        }),
      },
    });
  "#
);

stylex_test!(
  keyframes_object_rtl_polyfills_legacy,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.keyframes({
      from: {
        insetBlockStart: 0,
      },
      to: {
        insetBlockStart: 100,
      }
    });
  "#
);

// An explicitly empty `classNamePrefix` is honoured rather than replaced by
// the default, so the animation name carries no prefix.
stylex_test!(
  keyframes_with_empty_class_name_prefix,
  |tr| build_test_transform(tr.comments.clone(), |b| b
    .with_class_name_prefix("")
    .with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.keyframes({
      from: {
        color: 'red',
      },
      to: {
        color: 'blue',
      }
    });
  "#
);

// A step value that is not a string or a number declares nothing. There is no
// condition to apply inside an animation step and no fallback to choose from, so
// a nested value object and a fallback array mean nothing there; `null` means
// nothing anywhere, and `undefined` is a global identifier with no binding to
// read. All of them leave the step with whatever else it declares —
// here, nothing.
//
// The animation name is a hash of the steps, so `x1mv4754-B` recurring is the
// claim that the declaration is really gone rather than emitted empty: it is the
// name a `from` step that declared nothing produces. Measured output of
// `@stylexjs/babel-plugin` 0.19.0 for each input.
stylex_test!(
  a_step_value_that_is_not_a_string_declares_nothing,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const nested = stylex.keyframes({
      from: { color: { default: 'red' } },
      to: { color: 'blue' },
    });
    export const array = stylex.keyframes({
      from: { color: ['red', 'blue'] },
      to: { color: 'blue' },
    });
    export const nullish = stylex.keyframes({
      from: { color: null },
      to: { color: 'blue' },
    });
    export const boolean = stylex.keyframes({
      from: { color: true },
      to: { color: 'blue' },
    });
    export const undef = stylex.keyframes({
      from: { color: undefined },
      to: { color: 'blue' },
    });
  "#
);

// Only the declaration that cannot be read drops; the step keeps its siblings,
// and reaches the name a step declaring only that sibling produces.
stylex_test!(
  a_dropped_step_declaration_keeps_its_siblings,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const dropped = stylex.keyframes({
      from: { color: null, opacity: 0.5 },
      to: { color: 'blue' },
    });
    export const sibling = stylex.keyframes({
      from: { opacity: 0.5 },
      to: { color: 'blue' },
    });
  "#
);

// A shorthand expands to longhands that each drop, so the step declares
// nothing rather than a partial expansion.
stylex_test!(
  a_shorthand_step_value_that_is_not_a_string_declares_nothing,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const name = stylex.keyframes({
      from: { margin: null },
      to: { color: 'blue' },
    });
  "#
);

// A blank step value declares nothing for the same reason a blank style value
// does: `color:` is not valid CSS. The reference implementation reaches a null
// dereference inside its value normaliser here, so this converges on what it
// does deliberately for `null` -- the animation name is the one a step
// declaring nothing produces.
stylex_test!(
  a_blank_step_value_declares_nothing,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const empty = stylex.keyframes({
      from: { color: '' },
      to: { color: 'blue' },
    });
    export const blank = stylex.keyframes({
      from: { color: ' ' },
      to: { color: 'blue' },
    });
    export const blankWithSibling = stylex.keyframes({
      from: { color: ' ', opacity: 0.5 },
      to: { color: 'blue' },
    });
  "#
);
