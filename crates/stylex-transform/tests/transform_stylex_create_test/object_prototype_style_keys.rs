//! A style key spelled like a name every object inherits.
//!
//! `valueOf`, `hasOwnProperty`, `toString` and the rest are not CSS properties,
//! and an author who writes one has made a mistake either way. What matters is
//! that this compiler reads the key as the property name it was written as,
//! rather than reaching the method the name inherits -- which is what decides
//! the class name, and so what a stylesheet and a page have to agree on.
//!
//! The reference compiler reaches its own inherited method, and answers three
//! different ways for the seven names. All seven were measured against
//! `@stylexjs/babel-plugin@0.19.0`, and each case below records which:
//!
//! - `valueOf`, `hasOwnProperty`, `propertyIsEnumerable` and `__proto__` --
//!   the reference drops the declaration and emits `{ $$css: true }` alone.
//! - `constructor` and `isPrototypeOf` -- the reference stops the build, with
//!   `A StyleX namespace must be an object.` and `pairs is not iterable`.
//! - `toString` -- the reference emits one declaration per character of the
//!   text its own method returned: fourteen rules, `.x…{o:}`, `.x…{b:}` and so
//!   on.
//!
//! The parity corpus carries the value-level rows for these names, under the
//! `style key off Object.prototype` family. Nothing carried the key-level ones,
//! so a change that started dropping such a key would have been silent here.

use crate::utils::prelude::*;

// The four the reference drops.
stylex_test!(
  value_of_is_read_as_the_property_it_was_written_as,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const s = stylex.create({ x: { valueOf: 'red' } });
  "#
);

stylex_test!(
  has_own_property_is_read_as_the_property_it_was_written_as,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const s = stylex.create({ x: { hasOwnProperty: 'red' } });
  "#
);

stylex_test!(
  property_is_enumerable_is_read_as_the_property_it_was_written_as,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const s = stylex.create({ x: { propertyIsEnumerable: 'red' } });
  "#
);

// A key the language itself treats apart: in an object literal `__proto__`
// sets the prototype rather than declaring a property. It is read here as the
// name it was written as, like the others.
stylex_test!(
  dunder_proto_is_read_as_the_property_it_was_written_as,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const s = stylex.create({ x: { __proto__: 'red' } });
  "#
);

// The two the reference stops the build on.
stylex_test!(
  constructor_is_read_as_the_property_it_was_written_as,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const s = stylex.create({ x: { constructor: 'red' } });
  "#
);

stylex_test!(
  is_prototype_of_is_read_as_the_property_it_was_written_as,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const s = stylex.create({ x: { isPrototypeOf: 'red' } });
  "#
);

// The one the reference expands into a declaration per character.
stylex_test!(
  to_string_is_read_as_the_property_it_was_written_as,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const s = stylex.create({ x: { toString: 'red' } });
  "#
);
