use stylex_swc_plugin::{
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
  ModuleTransformVisitor,
};
use swc_core::{
  common::FileName,
  ecma::{
    parser::{Syntax, TsSyntax},
    transforms::testing::test,
  },
};

const ROOT_DIR: &str = "/stylex/packages/";

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_variables_object_variables_object,
  r#"
        import stylex from 'stylex';
          export const buttonTheme = stylex.defineVars({
            '--bgColor': {
              default: 'blue',
              '@media (prefers-color-scheme: dark)': 'lightblue',
              '@media print': 'white',
            },
            '--bgColorDisabled': {
              default: 'grey',
              '@media (prefers-color-scheme: dark)': 'rgba(0, 0, 0, 0.8)',
            },
            '--cornerRadius': 10,
            '--fgColor': {
              default: 'pink',
            },
          });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        ROOT_DIR.to_string()
      ))),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        ROOT_DIR.to_string()
      ))),
      dev: Some(true),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        ROOT_DIR.to_string()
      ))),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      dev: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      dev: Some(true),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      dev: Some(true),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      dev: Some(true),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      dev: Some(true),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/utils/NestedTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        ROOT_DIR.to_string()
      ))),
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
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/utils/NestedTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        ROOT_DIR.to_string()
      ))),
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

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/utils/NestedTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(false),
      runtime_injection: Some(false),
      gen_conditional_classes: Some(true),
      treeshake_compensation: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        ROOT_DIR.to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_variables_object_in_commonjs_with_nested_file_path_and_extended_options,
  r#"
    import * as stylex from "@stylexjs/stylex";

    export const buttonTokens = stylex.defineVars({
      bgColor: "blue",
      textColor: "white",
      cornerRadius: "4px",
      paddingBlock: "4px",
      paddingInline: "8px",
    });
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_variables_object_with_nested_defined_vars_call,
  r#"
        import stylex from 'stylex';
        export const colors = stylex.defineVars({
            primary: '#ff0000',
        });
        export const buttonTheme = stylex.defineVars({
            bgColor: {
                default: colors.primary,
            },
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    &PluginPass {
      cwd: None,
      filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(false),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_variables_object_with_nested_defined_vars_calls,
  r#"
        import stylex from 'stylex';
        export const colors = stylex.defineVars({
            primary: '#ff0000',
        });
        export const designSystem = stylex.defineVars({
            primary: colors.primary,
        });
        export const buttonTheme = stylex.defineVars({
            bgColor: {
                default: designSystem.primary,
            },
        });
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    ModuleTransformVisitor::new_test(
      tr.comments.clone(),
      &PluginPass {
        cwd: None,
        filename: FileName::Real("/stylex/packages/TestTheme.stylex.js".into()),
      },
      None,
    )
  },
  transforms_variables_object_with_key_containing_differend_symbols,
  r#"
        import stylex from 'stylex';
        export const buttonTokens = stylex.defineVars({
          default: "blue",
          xl: "red",
          "2xl": "green",
          "xl3": "yellow",
          if: "purple",
          else: "orange",
          __underscore__: "black",
          "&lt": "white",
          1: "gray",
          gt$eq: "brown",
        });
    "#
);
