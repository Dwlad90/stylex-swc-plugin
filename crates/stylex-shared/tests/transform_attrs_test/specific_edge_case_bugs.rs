use stylex_shared::{
  StyleXTransform,
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
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
      gen_conditional_classes: Some(true),
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
      stylex.attrs([
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
      gen_conditional_classes: Some(true),
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
        export const complex = stylex.attrs([
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
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      debug: Some(true),
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_in_debug_mode,
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
    export const complex = stylex.attrs([
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
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      debug: Some(true),
      enable_debug_class_names: Some(false),
      gen_conditional_classes: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_in_debug_mode_with_debug_classnames_disabled,
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
    export const complex = stylex.attrs([
      styles.root,
      sidebar == null && !isSidebar ? styles.noSidebar : styles.withSidebar,
      isSidebar && styles.sidebar,
      isContent && styles.content,
    ]);
  "#
);
