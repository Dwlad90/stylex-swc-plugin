use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleResolution, StyleXOptionsParams},
  },
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
    None
  ),
  stylex_call_using_styles_inside_use_memo,
  r#"
    import stylex from 'stylex';
    import { useMemo } from 'react';

    const styles = stylex.create({
      selected: {
        color: 'red',
      },
    });

    export default function MyComponent() {
      const isSelected = true;

      const innerComponent = useMemo(() => {
        return <Component {...stylex.props(isSelected && styles.selected)} />
      }, [isSelected]);

      return innerComponent;
    }
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
      enable_inlined_conditional_merge: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_call_using_styles_inside_use_memo_skip_conditional,
  r#"
    import stylex from 'stylex';
    import { useMemo } from 'react';

    const styles = stylex.create({
      selected: {
        color: 'red',
      },
    });

    export default function MyComponent() {
      const isSelected = true;

      const innerComponent = useMemo(() => {
        return <Component {...stylex.props(isSelected && styles.selected)} />
      }, [isSelected]);

      return innerComponent;
    }
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      enable_inlined_conditional_merge: Some(true),
      style_resolution: Some(StyleResolution::ApplicationOrder),
      ..StyleXOptionsParams::default()
    })
  ),
  transform_style_extend_prop_with_stylex_class,
  r#"
    import * as sx from '@stylexjs/stylex';
    import * as React from 'react';


    const c = sx.create({
      descriptionTextLink: {
        color: "red",
      },
    });


    export default function CommentField() {
      const isBold = false;

      sx.props(isBold && c.descriptionTextLink, isBold && c.descriptionTextLink,isBold && c.descriptionTextLink,isBold && c.descriptionTextLink);

      return (
        <div styleExtend={[c.descriptionTextLink]} />
      );
    }
"#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      enable_inlined_conditional_merge: Some(true),
      style_resolution: Some(StyleResolution::ApplicationOrder),
      ..StyleXOptionsParams::default()
    })
  ),
  transform_style_extend_with_dynamic_stylex_class,
  r#"
    import * as sx from '@stylexjs/stylex';
    import * as React from 'react';


    const c = sx.create({
      base: {
        display: 'grid',
      },
      regularGrid: {
        display: 'grid',
      },
      irregularGrid: {
        display: 'grid',
      },
    });


    export default function CommentField({ type }) {
      let gridType = 'regular';

      if(type === 'irregular') {
        gridType = 'irregular';
      }

      const grid = `${gridType}Grid`;

      return (
        <div styleExtend={[c.base, c[grid]]} />
      );
    }
"#
);
