use stylex_swc_plugin::{
    shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
    ModuleTransformVisitor,
};
use swc_core::ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
};

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Some(StyleXOptionsParams {
            dev: Some(true),
            gen_conditional_classes: Some(true),
            ..StyleXOptionsParams::default()
        })
    ),
    basic_stylex_call,
    r#"
      import stylex from '@stylexjs/stylex';
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
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(
        tr.comments.clone(),
        PluginPass::default(),
        Some(StyleXOptionsParams {
            dev: Some(true),
            gen_conditional_classes: Some(true),
            ..StyleXOptionsParams::default()
        })
    ),
    basic_stylex_call_exported,
    r#"
        import stylex from '@stylexjs/stylex';
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
