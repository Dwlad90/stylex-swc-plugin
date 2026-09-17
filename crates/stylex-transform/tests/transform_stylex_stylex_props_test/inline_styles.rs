//! A `props` call that is given a plain object beside the compiled styles.
//!
//! The merge keeps such an object as the inline style the runtime applies, so
//! the call answers a `style` property holding the declarations the author
//! wrote, under the names they wrote them with.
//!
//! The last case reads the same object through `attrs`, where the answer is
//! the CSS text a `style` attribute holds. The two spellings of one name are
//! the reason the cases sit together: only the contrast shows that each result
//! names the declaration the way its own reader expects.

use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_runtime_injection()))
}

stylex_test!(
  inline_style_on_its_own,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({ color: 'blue' });
  "#
);

stylex_test!(
  inline_style_beside_a_compiled_style,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    export default stylex.props(styles.red, { color: 'blue' });
  "#
);

stylex_test!(
  inline_style_holding_nothing,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({});
  "#
);

// A value the compiler cannot read leaves the whole call to the runtime.
stylex_test!(
  inline_style_whose_value_is_not_static,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export const withColor = (color) => stylex.props({ color });
  "#
);

// A `style` property is read by the runtime, not by CSS, so each name keeps
// the spelling the author wrote. A custom property already spells its own.
stylex_test!(
  inline_style_keeps_the_name_the_author_wrote,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({
      backgroundColor: 'blue',
      '--myColor': 'red',
    });
  "#
);

// A later inline style wins over an earlier one, declaration by declaration,
// the way the merge orders compiled styles.
//
// The snapshot also pins the order the declarations come out in: a name only
// the earlier style declares comes first, and the name both declare keeps the
// later value. A snapshot cannot say whether an order is right, so this source
// was put through `pnpm run parity:probe` from `crates/stylex-rs-compiler`
// first, and the reference prints the same order. Ticket 74 of
// `.scratch/split-transform-crate` records the reading.
stylex_test!(
  a_later_inline_style_wins_over_an_earlier_one,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props(
      { color: 'blue', margin: '1px' },
      { color: 'green' },
    );
  "#
);

// A declaration set to null declares nothing, so the merge writes no `style`
// property at all.
stylex_test!(
  inline_style_holding_only_null,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({ color: null });
  "#
);

// A condition is answered by one object per outcome, and only the outcome
// that holds the inline style carries the `style` property.
stylex_test!(
  inline_style_under_a_condition,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    export const maybe = (flag) => stylex.props(styles.red, flag && { color: 'blue' });
  "#
);

// The same object read as attributes is text, because an attribute is text.
stylex_test!(
  inline_style_read_as_attributes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    export default stylex.attrs(styles.red, { backgroundColor: 'blue' });
  "#
);

// A number is the value kind written without a unit -- `opacity`, `zIndex`,
// `flexGrow` and `order` are all written this way -- and it stays a number in
// the `style` property, because the runtime applies the object as it stands.
//
// The snapshot also pins how each number is spelled, which is where the two
// compilers can part company: `1e21` is written `1e+21`, `1.50` loses the
// trailing zero, `-0` is `0`, and the six custom properties hold the numbers
// on both sides of the range that is written without an exponent. The source
// was put through `pnpm run parity:probe` from `crates/stylex-rs-compiler`
// first, and the reference prints every one of them the same way.
stylex_test!(
  inline_style_holding_a_number,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({
      opacity: 0.5,
      zIndex: 1e21,
      flexGrow: 1.50,
      order: -0,
      '--a': 1e20,
      '--b': 1e-6,
      '--c': 1e-7,
      '--d': 100,
      '--e': 123456789012345678901234567890,
      '--f': 0.1,
    });
  "#
);

// A boolean is a declaration like any other. Before, the merge kept it and the
// step that writes the properties left it out, so the call answered a `style`
// property holding nothing.
stylex_test!(
  inline_style_holding_a_boolean,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({ color: true, background: false });
  "#
);

// A pseudo-class holds declarations of its own, as deep as the author wrote
// them.
stylex_test!(
  inline_style_holding_an_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({
      ':hover': { color: 'blue', ':focus': { color: 'red' } },
    });
  "#
);

// A declaration set to null inside such an object stays there. Only a null the
// style itself declares is dropped, because that is the one the merge reads as
// clearing the property.
stylex_test!(
  inline_style_holding_an_object_with_a_null,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({ ':hover': { color: null }, margin: null });
  "#
);

// Every kind at once, beside a compiled style, which is the shape a component
// really writes.
stylex_test!(
  inline_style_of_each_kind_beside_a_compiled_style,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      red: {
        color: 'red',
      }
    });
    export default stylex.props(styles.red, {
      opacity: 0.5,
      display: 'block',
      color: true,
      ':hover': { color: 'blue' },
    });
  "#
);

// A later declaration wins whatever kind either of them is, and the name keeps
// the place of its first writing.
stylex_test!(
  a_later_inline_style_of_another_kind_wins,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props(
      { opacity: 0.5, margin: '1px' },
      { opacity: 'inherit' },
      { margin: true },
    );
  "#
);

// The same kinds read as attributes are text, because an attribute is text.
// Each one is spelled the way JavaScript spells it, which for an object is the
// text every plain object spells.
stylex_test!(
  inline_style_of_each_kind_read_as_attributes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.attrs({
      marginTop: 10,
      opacity: 0.5,
      color: true,
      background: false,
      ':hover': { color: 'blue' },
    });
  "#
);

// A number JavaScript writes by name rather than with digits. The emitter has
// no numeral for these three, so without a spelling of their own it invents
// the arithmetic that answers them and the printed module stops matching the
// one the reference prints.
stylex_test!(
  inline_style_holding_a_number_written_by_name,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({
      opacity: Infinity,
      zIndex: -Infinity,
      order: NaN,
    });
  "#
);

// The same three read as attributes, where they are text already.
stylex_test!(
  a_number_written_by_name_read_as_an_attribute,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.attrs({
      opacity: Infinity,
      zIndex: -Infinity,
      order: NaN,
    });
  "#
);

// A declaration the author gave no value declares nothing. The merge skips it
// whole, so it writes no property and does not clear what a style before it
// declared -- which is what separates it from a null.
stylex_test!(
  inline_style_holding_a_declaration_with_no_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({ color: 'red' }, { color: undefined, margin: undefined });
  "#
);

// An author writes a list where one property is given fallbacks. A style
// object has no list form, so each element is named by the place it holds.
stylex_test!(
  inline_style_holding_a_list,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({
      color: ['red', 'blue'],
      zIndex: [1, null],
      margin: [],
      padding: [['a'], { b: 1 }],
    });
  "#
);

// The same lists read as attributes are one run of text with commas between.
// A list of lists reads as one run, and a slot holding nothing writes nothing
// between two commas.
stylex_test!(
  inline_style_holding_a_list_read_as_attributes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.attrs({
      color: ['red', 'blue'],
      zIndex: [1, null],
      margin: [],
      padding: [['a'], { b: 1 }],
    });
  "#
);

// The name that sets what an object inherits from declares no property of its
// own, so it is left out rather than written into the style the runtime
// applies.
stylex_test!(
  inline_style_holding_the_prototype_name,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({ __proto__: 'red', color: 'blue' });
  "#
);

// A prototype that is an object declares its own names all the same, because
// the merge walks what a style inherits from. They come after the names the
// style wrote, are shadowed by them, and a whole chain reads the same way.
//
// The last declaration holds the other half of the rule: nothing merges a
// nested object, so nothing walks what that one inherits from.
stylex_test!(
  inline_style_inheriting_from_another_object,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.props({
      __proto__: { __proto__: { a: '1' }, color: 'red', margin: '9px' },
      margin: '1px',
      ':hover': { __proto__: { b: '2' }, padding: '3px' },
    });
  "#
);

// A custom property keeps the author's spelling where a style object is
// written and takes the attribute spelling where a style attribute is. The
// runtime dashes every capital of an attribute name and leaves nothing alone.
stylex_test!(
  a_custom_property_read_as_an_attribute,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export default stylex.attrs({ '--myColor': 'red', ABCDef: '1' });
  "#
);
