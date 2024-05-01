use stylex_swc_plugin::{
  shared::structures::{
    named_import_source::RuntimeInjection,
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
  ModuleTransformVisitor,
};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
  },
};

const ROOT_DIR: &str = "/stylex/packages/";

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(false)),
      unstable_module_resolution: Option::Some(StyleXOptions::get_haste_module_resolution(
        Option::None
      )),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_variables_object,
  r#"
        import stylex from 'stylex';
        export const buttonTheme = stylex.defineVars({
            bgColor: {
                default: 'blue',
                '@media (prefers-color-scheme: dark)': 'lightblue',
                '@media print': 'white',
            },
            bgColorDisabled: {
                default: 'grey',
                '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
            },
            cornerRadius: 10,
            fgColor: {
                default: 'pink',
            },
        });
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(false)),
      unstable_module_resolution: Option::Some(StyleXOptions::get_haste_module_resolution(
        Option::None
      )),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_variables_object_with_import_asterisk,
  r#"
        import * as foo from 'stylex';
        export const buttonTheme = foo.defineVars({
            bgColor: {
                default: 'blue',
                '@media (prefers-color-scheme: dark)': 'lightblue',
                '@media print': 'white',
            },
            bgColorDisabled: {
                default: 'grey',
                '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
            },
            cornerRadius: 10,
            fgColor: {
                default: 'pink',
            },
        });
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(false)),
      unstable_module_resolution: Option::Some(StyleXOptions::get_haste_module_resolution(
        Option::None
      )),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_variables_object_with_named_import,
  r#"
        import {defineVars} from 'stylex';
        export const buttonTheme = defineVars({
            bgColor: {
                default: 'blue',
                '@media (prefers-color-scheme: dark)': 'lightblue',
                '@media print': 'white',
            },
            bgColorDisabled: {
                default: 'grey',
                '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
            },
            cornerRadius: 10,
            fgColor: {
                default: 'pink',
            },
        });
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(false)),
      unstable_module_resolution: Option::Some(StyleXOptions::get_haste_module_resolution(
        Option::None
      )),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_referenced_local_variables_object,
  r#"
        import stylex from 'stylex';
        const defaultButtonTokens = {
            bgColor: {
                default: 'blue',
                '@media (prefers-color-scheme: dark)': 'lightblue',
                '@media print': 'white',
            },
            bgColorDisabled: {
                default: 'grey',
                '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
            },
            cornerRadius: 10,
            fgColor: {
                default: 'pink',
            },
        };
        export const buttonTheme = stylex.defineVars(defaultButtonTokens);
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      unstable_module_resolution: Option::Some(StyleXOptions::get_haste_module_resolution(
        Option::None
      )),
      dev: Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_variables_object_and_add_stylex_inject_in_dev_mode,
  r#"
        import stylex from 'stylex';
        export const buttonTheme = stylex.defineVars({
            bgColor: {
                default: 'blue',
                '@media (prefers-color-scheme: dark)': 'lightblue',
                '@media print': 'white',
            },
            bgColorDisabled: {
                default: 'grey',
                '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
            },
            cornerRadius: 10,
            fgColor: {
                default: 'pink',
            },
        });
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(false)),
      unstable_module_resolution: Option::Some(StyleXOptions::get_common_js_module_resolution(
        Option::Some(ROOT_DIR.to_string())
      )),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_variables_object_in_non_haste_env,
  r#"
    import stylex from 'stylex';
    export const buttonTheme = stylex.defineVars({
      bgColor: {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
      bgColorDisabled: {
        default: 'grey',
        '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
      },
      cornerRadius: 10,
      fgColor: {
        default: 'pink',
      },
    });
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(false)),
      unstable_module_resolution: Option::Some(StyleXOptions::get_common_js_module_resolution(
        Option::Some(ROOT_DIR.to_string())
      )),
      dev: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_variables_object_in_non_haste_dev_env,
  r#"
    import stylex from 'stylex';
    export const buttonTheme = stylex.defineVars({
      bgColor: {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
      bgColorDisabled: {
        default: 'grey',
        '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
      },
      cornerRadius: 10,
      fgColor: {
        default: 'pink',
      },
    });
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(false)),
      unstable_module_resolution: Option::Some(StyleXOptions::get_common_js_module_resolution(
        Option::Some(ROOT_DIR.to_string())
      )),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_multiple_variables_objects_in_a_single_file,
  r#"
    import stylex from 'stylex';
    export const buttonTheme = stylex.defineVars({
      bgColor: {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
      bgColorDisabled: {
        default: 'grey',
        '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
      },
      cornerRadius: 10,
      fgColor: {
        default: 'pink',
      },
    });
    export const textInputTheme = stylex.defineVars({
      bgColor: 'white',
      labelColor: {
        default: 'black',
        '@media (prefers-color-scheme: dark)': 'white',
      },
      cornerRadius: 8,
    });
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(false)),
      dev: Option::Some(true),
      unstable_module_resolution: Option::Some(StyleXOptions::get_haste_module_resolution(
        Option::None
      )),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_multiple_variables_objects_in_a_single_file_in_dev_mode,
  r#"
    import stylex from 'stylex';
    export const buttonTheme = stylex.defineVars({
      bgColor: {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
      bgColorDisabled: {
        default: 'grey',
        '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
      },
      cornerRadius: 10,
      fgColor: {
        default: 'pink',
      },
    });
    export const textInputTheme = stylex.defineVars({
      bgColor: 'white',
      labelColor: {
        default: 'black',
        '@media (prefers-color-scheme: dark)': 'white',
      },
      cornerRadius: 8,
    });
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      unstable_module_resolution: Option::Some(StyleXOptions::get_haste_module_resolution(
        Option::None
      )),
      dev: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_variables_objects_with_references_to_local_variables,
  r#"
    import stylex from 'stylex';
    const RADIUS = 10;
    export const buttonTheme = stylex.defineVars({
      bgColor: {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
      bgColorDisabled: {
        default: 'grey',
        '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
      },
      cornerRadius: RADIUS,
      fgColor: {
        default: 'pink',
      },
    });
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      unstable_module_resolution: Option::Some(StyleXOptions::get_haste_module_resolution(
        Option::None
      )),
      dev: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  allows_references_to_local_variables_with_static_values,
  r#"
    import stylex from 'stylex';
    const color = 'blue';
    export const buttonTheme = stylex.defineVars({
      bgColor: {
        default: color,
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
      bgColorDisabled: {
        default: 'grey',
        '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
      },
      cornerRadius: 10,
      fgColor: {
        default: 'pink',
      },
    });
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      unstable_module_resolution: Option::Some(StyleXOptions::get_haste_module_resolution(
        Option::None
      )),
      dev: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  allows_template_literal_references,
  r#"
    import stylex from 'stylex';
    const name = 'light';
    export const buttonTheme = stylex.defineVars({
      bgColor: {
        default: `${name}blue`,
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
      bgColorDisabled: {
        default: 'grey',
        '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
      },
      cornerRadius: 10,
      fgColor: {
        default: 'pink',
      },
    });
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      unstable_module_resolution: Option::Some(StyleXOptions::get_haste_module_resolution(
        Option::None
      )),
      dev: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  allows_pure_complex_expressions,
  r#"
    import stylex from 'stylex';
    const RADIUS = 2;
    export const buttonTheme = stylex.defineVars({
      bgColor: {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
      bgColorDisabled: {
        default: 'grey',
        '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
      },
      cornerRadius: RADIUS * 2,
      fgColor: {
        default: 'pink',
      },
    });
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/utils/NestedTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      dev: Option::Some(true),
      unstable_module_resolution: Option::Some(StyleXOptions::get_common_js_module_resolution(
        Option::Some(ROOT_DIR.to_string())
      )),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_variables_object_in_commonjs_with_nested_file_path,
  r#"
    import stylex from 'stylex';
    export const buttonTheme = stylex.defineVars({
      bgColor: {
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      },
      bgColorDisabled: {
        default: 'grey',
        '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
      },
      cornerRadius: 10,
      fgColor: {
        default: 'pink',
      },
    });
  "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass {
      cwd: Option::None,
      filename: FileName::Real("/stylex/packages/utils/NestedTheme.stylex.js".into()),
    },
    Some(StyleXOptionsParams {
      dev: Option::Some(true),
      unstable_module_resolution: Option::Some(StyleXOptions::get_common_js_module_resolution(
        Option::Some(ROOT_DIR.to_string())
      )),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_variables_object_with_stylex_types_wrapper,
  r#"
    import stylex from 'stylex';
    export const buttonTheme = stylex.defineVars({
      bgColor: stylex.types.color({
        default: 'blue',
        '@media (prefers-color-scheme: dark)': 'lightblue',
        '@media print': 'white',
      }),
      bgColorDisabled: stylex.types.color({
        default: 'grey',
        '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
      }),
      cornerRadius: stylex.types.length('10px'),
      fgColor: stylex.types.color({
        default: 'pink',
      }),
    });
  "#
);
