use crate::utils::prelude::*;
use swc_core::{common::FileName, ecma::transforms::testing::test};

stylex_test!(
  basic_stylex_call,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_filename(FileName::Real("src/js/components/Foo.react.js".into()))
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_unstable_module_resolution(StyleXOptions::get_common_js_module_resolution(Some(
      "/".to_string()
    )))
    .with_runtime_injection()
    .into_pass(),
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
      stylex(
        styles.root,
        sidebar == null ? styles.noSidebar : styles.withSidebar,
      );
"#
);

stylex_test!(
  basic_exported_stylex_create_call,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_runtime_injection()
    .into_pass(),
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
      stylex(
        styles.root,
        sidebar == null ? styles.noSidebar : styles.withSidebar,
      );
"#
);

stylex_test!(
  basic_stylex_call_skip_conditional,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_enable_inlined_conditional_merge(false)
    .with_runtime_injection()
    .into_pass(),
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
      stylex(
        styles.root,
        sidebar == null ? styles.noSidebar : styles.withSidebar,
      );
"#
);

stylex_test!(
  basic_stylex_call_extended,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_dev(true)
    .with_enable_debug_class_names(true)
    .with_runtime_injection()
    .into_pass(),
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
      export const complex = stylex(
        styles.root,
        sidebar == null && !isSidebar ? styles.noSidebar : styles.withSidebar,
        isSidebar && styles.sidebar,
        isContent && styles.content,
      );
"#
);
