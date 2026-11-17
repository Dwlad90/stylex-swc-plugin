use crate::utils::prelude::*;
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
