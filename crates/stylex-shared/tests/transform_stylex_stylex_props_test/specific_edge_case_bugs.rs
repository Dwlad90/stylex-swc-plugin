use stylex_shared::{
  StyleXTransform,
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::test,
  },
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      enable_debug_class_names: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  basic_stylex_call,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      enable_debug_class_names: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  basic_stylex_call_exported,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/html/js/FooBar.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      debug: Some(true),
      enable_debug_class_names: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_with_debug_on,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/html/js/FooBar.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      debug: Some(true),
      enable_debug_class_names: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_with_debug_on_and_debug_classnames_off,
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
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
