use crate::utils::{prelude::*, transform::stringify_js};
use swc_core::common::FileName;

stylex_test!(
  basic_stylex_call,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_dev(true).with_runtime_injection()
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      sidebar: {
        boxSizing: 'border-box',
        gridArea: 'sidebar',
      },
      content: {
        gridArea: 'content',
      },
      root: {
        display: 'grid',
        gridTemplateRows: '100%',
        gridTemplateAreas: '"content"',
      },
      withSidebar: {
        gridTemplateColumns: 'auto minmax(0, 1fr)',
        gridTemplateRows: '100%',
        gridTemplateAreas: '"sidebar content"',
        '@media (max-width: 640px)': {
          gridTemplateRows: 'minmax(0, 1fr) auto',
          gridTemplateAreas: '"content" "sidebar"',
          gridTemplateColumns: '100%',
        },
      },
      noSidebar: {
        gridTemplateColumns: 'minmax(0, 1fr)',
      },
    });
    stylex.props([
      styles.root,
      sidebar == null ? styles.noSidebar : styles.withSidebar,
    ]);
  "#
);

stylex_test!(
  basic_stylex_call_exported,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real("/html/js/FooBar.react.js".into()))
      .with_dev(true)
      .with_runtime_injection()
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      sidebar: {
        boxSizing: 'border-box',
        gridArea: 'sidebar',
      },
      content: {
        gridArea: 'content',
      },
      root: {
        display: 'grid',
        gridTemplateRows: '100%',
        gridTemplateAreas: '"content"',
      },
      withSidebar: {
        gridTemplateColumns: 'auto minmax(0, 1fr)',
        gridTemplateRows: '100%',
        gridTemplateAreas: '"sidebar content"',
        '@media (max-width: 640px)': {
          gridTemplateRows: 'minmax(0, 1fr) auto',
          gridTemplateAreas: '"content" "sidebar"',
          gridTemplateColumns: '100%',
        },
      },
      noSidebar: {
        gridTemplateColumns: 'minmax(0, 1fr)',
      },
    });
    export const complex = stylex.props([
      styles.root,
      sidebar == null && !isSidebar ? styles.noSidebar : styles.withSidebar,
      isSidebar && styles.sidebar,
      isContent && styles.content,
    ]);
  "#
);

stylex_test!(
  stylex_call_with_debug_on,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real("/html/js/FooBar.react.js".into()))
      .with_dev(true)
      .with_debug(true)
      .with_runtime_injection()
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      sidebar: {
        boxSizing: 'border-box',
        gridArea: 'sidebar',
      },
      content: {
        gridArea: 'content',
      },
      root: {
        display: 'grid',
        gridTemplateRows: '100%',
        gridTemplateAreas: '"content"',
      },
      withSidebar: {
        gridTemplateColumns: 'auto minmax(0, 1fr)',
        gridTemplateRows: '100%',
        gridTemplateAreas: '"sidebar content"',
        '@media (max-width: 640px)': {
          gridTemplateRows: 'minmax(0, 1fr) auto',
          gridTemplateAreas: '"content" "sidebar"',
          gridTemplateColumns: '100%',
        },
      },
      noSidebar: {
        gridTemplateColumns: 'minmax(0, 1fr)',
      },
    });
    export const complex = stylex.props([
      styles.root,
      sidebar == null && !isSidebar ? styles.noSidebar : styles.withSidebar,
      isSidebar && styles.sidebar,
      isContent && styles.content,
    ]);
  "#
);

stylex_test!(
  stylex_call_with_debug_on_uses_hashed_classnames,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_filename(FileName::Real("/html/js/FooBar.react.js".into()))
      .with_dev(true)
      .with_debug(true)
      .with_runtime_injection()
  }),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const styles = stylex.create({
      sidebar: {
        boxSizing: 'border-box',
        gridArea: 'sidebar',
      },
      content: {
        gridArea: 'content',
      },
      root: {
        display: 'grid',
        gridTemplateRows: '100%',
        gridTemplateAreas: '"content"',
      },
      withSidebar: {
        gridTemplateColumns: 'auto minmax(0, 1fr)',
        gridTemplateRows: '100%',
        gridTemplateAreas: '"sidebar content"',
        '@media (max-width: 640px)': {
          gridTemplateRows: 'minmax(0, 1fr) auto',
          gridTemplateAreas: '"content" "sidebar"',
          gridTemplateColumns: '100%',
        },
      },
      noSidebar: {
        gridTemplateColumns: 'minmax(0, 1fr)',
      },
    });
    export const complex = stylex.props([
      styles.root,
      sidebar == null && !isSidebar ? styles.noSidebar : styles.withSidebar,
      isSidebar && styles.sidebar,
      isContent && styles.content,
    ]);
  "#
);

stylex_test!(
  hoisting_correctly_with_duplicate_names,
  r#"
    import * as stylex from "@stylexjs/stylex";
    import * as React from "react";

    function Foo() {
      const styles = stylex.create({
        div: { color: "red" },
      });
      return <div {...stylex.props(styles.div)}>Hello, foo!</div>;
    }

    function Bar() {
      const styles = stylex.create({
        div: { color: "blue" },
      });
      return <div {...stylex.props(styles.div)}>Hello, bar!</div>;
    }

    export function App() {
      return (
        <>
          <Foo />
          <Bar />
        </>
      );
    }
  "#
);

stylex_test!(
  does_not_hoist_css_objects_that_reference_local_variables,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: { display: 'flex' },
    });
    function Foo({className}) {
      return <div {...stylex.props(styles.root, className && {$$css: true, __: className})}>Hello, foo!</div>;
    }
  "#
);

stylex_test!(
  does_not_hoist_css_objects_that_read_a_local_variable_inside_an_expression,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: { display: 'flex' },
    });
    function Foo({size, map}) {
      return <div {...stylex.props(styles.root, size && {$$css: true, a: `x${size}`}, map && {$$css: true, [map.key]: 'x'})}>Hello, foo!</div>;
    }
  "#
);

stylex_test!(
  does_not_hoist_css_objects_that_read_this_or_arguments,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: { display: 'flex' },
    });
    export class Foo {
      render() {
        return stylex.props(styles.root, this.on && {$$css: true, a: `x${this.size}`});
      }
    }
    export function Bar() {
      return stylex.props(styles.root, {$$css: true, [arguments[0]]: 'x'});
    }
  "#
);

stylex_test!(
  does_not_hoist_css_objects_that_reference_constructor_parameters,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: { display: 'flex' },
    });
    export class Foo {
      constructor(className) {
        this.props = stylex.props(styles.root, className && {$$css: true, __: className});
      }
    }
  "#
);

stylex_test!(
  does_not_hoist_css_objects_that_reference_loop_or_switch_bindings,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: { display: 'flex' },
    });
    export const out = [];
    for (const className of classNames) {
      out.push(stylex.props(styles.root, className && {$$css: true, __: className}));
    }
    switch (kind) {
      case 'a':
        let name = getName();
        out.push(stylex.props(styles.root, name && {$$css: true, __: name}));
    }
  "#
);

stylex_test!(
  hoists_css_objects_that_reference_a_default_exported_function,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: { display: 'flex' },
    });
    export default function Foo({c}) {
      return stylex.props(styles.root, c && {$$css: true, __: Foo});
    }
  "#
);

stylex_test!(
  does_not_hoist_css_objects_that_reference_an_anonymous_default_function_parameter,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: { display: 'flex' },
    });
    export default function ({c}) {
      return stylex.props(styles.root, c && {$$css: true, __: c});
    }
  "#
);

stylex_test!(
  hoists_css_objects_after_the_program_level_binding_they_read,
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { get } from './x';
    const styles = stylex.create({
      root: { display: 'flex' },
    });
    const cls = get();
    export function F(c) {
      return stylex.props(styles.root, c && {$$css: true, __: cls});
    }
    const other = get();
    export const o = {
      f(c) {
        return stylex.props(c && {$$css: true, __: other});
      },
    };
  "#
);

// A hoisted object goes in front of the statement that holds it. A binding that
// this statement or a later one declares does not hold its value there yet, so
// the object stays.
stylex_test!(
  does_not_hoist_css_objects_that_read_a_binding_declared_later,
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { get } from './x';
    export function F(c) {
      return stylex.props(c && {$$css: true, __: later});
    }
    const later = get();
    export function G(c) {
      return stylex.props(c && {$$css: true, __: laterVar});
    }
    var laterVar = get();
    export const Bar = (c) => stylex.props(c && {$$css: true, __: Bar});
    export function H(c) {
      return stylex.props(c && {$$css: true, __: later, a: hoisted});
    }
    function hoisted() {}
  "#
);

// Thousands of program-level statements. Each `F` reads the binding of the
// statement in front of it, so its object moves and goes after that binding.
// Each `G` reads the binding of the next statement, so its object stays.
#[test]
fn places_each_hoist_in_a_module_of_thousands_of_statements() {
  const COUNT: usize = 2_000;

  let mut input = String::from("import * as stylex from '@stylexjs/stylex';\n");
  for index in 0..COUNT {
    let next = index + 1;
    input.push_str(&format!(
      "const c{index} = get();\n\
       export function F{index}(c) {{ return stylex.props(c && {{$$css: true, __: c{index}}}); }}\n\
       export function G{index}(c) {{ return stylex.props(c && {{$$css: true, __: c{next}}}); }}\n"
    ));
  }
  input.push_str(&format!("const c{COUNT} = get();\n"));

  let output = stringify_js(&input, ts_syntax(), |tr| {
    build_test_transform(tr.comments.clone(), |builder| builder)
  });

  assert_eq!(output.matches("const _temp").count(), COUNT);

  for index in [0, COUNT / 2, COUNT - 1] {
    let binding = output.find(&format!("const c{index} = get();"));
    let hoisted = output.find(&format!("__: c{index}\n}};\nexport function F{index}("));

    assert!(binding.is_some() && binding < hoisted, "F{index}");
    assert!(
      output.contains(&format!(
        "function G{index}(c) {{\n    return stylex.props(c && {{"
      )),
      "G{index}"
    );
  }
}

// TypeScript declarations that compile to a `var` or a `const`, a `using`, and a
// `var` inside a program-level block all have no value before their statement
// runs, so an object that reads one of them before it stays. A `declare` form is
// ambient: it has no value in the module at all, so a read of it still moves.
stylex_test!(
  does_not_hoist_css_objects_that_read_a_later_typescript_or_block_binding,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export function F(c) {
      return stylex.props(
        c && {$$css: true, __: Kind},
        c && {$$css: true, __: Space},
        c && {$$css: true, __: resource},
        c && {$$css: true, __: inBlock},
        c && {$$css: true, __: inLoop},
        c && {$$css: true, __: Ambient},
        c && {$$css: true, __: ambientConst},
        c && {$$css: true, __: ambientVar},
        c && {$$css: true, __: AmbientClass},
        c && {$$css: true, __: AmbientEnum},
      );
    }
    enum Kind { A }
    namespace Space { export const a = 1; }
    using resource = open();
    {
      var inBlock = 1;
    }
    for (var inLoop of []) {}
    declare namespace Ambient {}
    declare const ambientConst: string;
    declare var ambientVar: string;
    declare class AmbientClass {}
    declare enum AmbientEnum { A }
  "#
);

// A program-level binding that a later statement assigns again holds another
// value when the call runs than when the module loads, so the object stays.
// This departs from the reference deliberately: Babel hoists the object, and
// the hoisted copy holds `undefined` where the call expects 'xa'.
stylex_test!(
  does_not_hoist_css_objects_that_read_a_reassigned_binding,
  r#"
    import * as stylex from '@stylexjs/stylex';
    let v;
    export function f(c) {
      return stylex.props(c && {$$css: true, __: v});
    }
    v = 'xa';
  "#
);

stylex_test!(
  does_not_hoist_css_objects_that_reference_their_own_default_exported_class,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: { display: 'flex' },
    });
    export default class Foo {
      render(c) {
        return stylex.props(styles.root, c && {$$css: true, __: Foo});
      }
    }
  "#
);

stylex_test!(
  does_not_hoist_css_objects_that_reference_an_anonymous_default_class_binding,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      root: { display: 'flex' },
    });
    export default class {
      constructor(className) {
        this.props = stylex.props(styles.root, className && {$$css: true, __: className});
      }
    }
  "#
);

// The transform reads TypeScript syntax that is not stripped yet. A default
// exported interface binds no value, so the module compiles as it does without
// it.
stylex_test!(
  compiles_a_module_with_a_default_exported_interface,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export default interface Props {
      c: string;
    }
    export const styles = stylex.create({
      root: { display: 'flex' },
    });
  "#
);
