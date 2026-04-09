use crate::utils::prelude::*;

fn file_transform(
  comments: std::rc::Rc<swc_core::common::comments::SingleThreadedComments>,
) -> impl swc_core::ecma::ast::Pass {
  build_test_transform(comments, |b| b.with_dev(true).with_runtime_injection())
}

stylex_test!(
  basic_stylex_call,
  |tr| file_transform(tr.comments.clone()),
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

stylex_test!(
  basic_stylex_call_exported,
  |tr| file_transform(tr.comments.clone()),
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

stylex_test!(
  stylex_call_in_debug_mode,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_dev(true).with_debug(true).with_runtime_injection()
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
    export const complex = stylex.attrs([
      styles.root,
      sidebar == null && !isSidebar ? styles.noSidebar : styles.withSidebar,
      isSidebar && styles.sidebar,
      isContent && styles.content,
    ]);
  "#
);

stylex_test!(
  stylex_call_in_debug_mode_with_debug_classnames_disabled,
  |tr| build_test_transform(tr.comments.clone(), |b| {
    b.with_dev(true)
      .with_debug(true)
      .with_enable_debug_class_names(false)
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
    export const complex = stylex.attrs([
      styles.root,
      sidebar == null && !isSidebar ? styles.noSidebar : styles.withSidebar,
      isSidebar && styles.sidebar,
      isContent && styles.content,
    ]);
  "#
);
