use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
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
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      debug: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  tokens_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: 'red',
      nextColor: {
        default: 'green'
      },
      otherColor: {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
    });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      debug: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  tokens_object_haste,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: 'red',
      nextColor: {
        default: 'green'
      },
      otherColor: {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
    });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/src/css/NestedTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      debug: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  tokens_object_deep_in_file_tree,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: 'red'
    });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      debug: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  literal_tokens_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      '--color': 'red',
      '--otherColor': {
        default: 'blue',
        ':hover': 'lightblue',
      },
    });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      debug: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  local_variable_tokens_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const tokens = {
      '--color': 'red',
      '--nextColor': {
        default: 'green'
      },
      '--otherColor': {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
    };
    export const vars = stylex.defineVars(tokens)
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      debug: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  local_variables_used_in_tokens_objects,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const COLOR = 'red';
    export const vars = stylex.defineVars({
      color: COLOR
    });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      debug: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  template_literals_used_in_tokens_objects,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const NUMBER = 10;
    export const vars = stylex.defineVars({
      size: `${NUMBER}rem`
    });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      debug: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  expressions_used_in_tokens_objects,
  r#"
    import * as stylex from '@stylexjs/stylex';
    const NUMBER = 10;
    export const vars = stylex.defineVars({
      radius: NUMBER * 2
    });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      debug: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_types_used_in_tokens_object,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: stylex.types.color({
        default: 'red',
        '@media (prefers-color-scheme: dark)': 'white',
        '@media print': 'black',
      })
    });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      debug: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  multiple_variables_objects,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = stylex.defineVars({
      color: 'red'
    });
    export const otherVars = stylex.defineVars({
      otherColor: 'orange'
    });
  "#
);
