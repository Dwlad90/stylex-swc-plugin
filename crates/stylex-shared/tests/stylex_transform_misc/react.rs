use stylex_shared::{shared::structures::plugin_pass::PluginPass, StyleXTransform};
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
