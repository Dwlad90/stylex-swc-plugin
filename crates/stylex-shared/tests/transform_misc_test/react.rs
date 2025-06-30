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
