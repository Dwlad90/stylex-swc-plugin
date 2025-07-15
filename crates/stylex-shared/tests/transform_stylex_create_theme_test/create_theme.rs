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
    PluginPass::new(None, None),
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  theme_object,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

export const theme = stylex.createTheme(vars, {
  color: {
    default: 'green',
    '@media (prefers-color-scheme: dark)': 'lightgreen',
    '@media print': 'transparent',
  },
  otherColor: {
    default: 'antiquewhite',
    '@media (prefers-color-scheme: dark)': 'floralwhite',
  },
  radius: '6px'
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
    PluginPass::new(None, None),
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      ..StyleXOptionsParams::default()
    })
  ),
  theme_object_haste,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

export const theme = stylex.createTheme(vars, {
  color: {
    default: 'green',
    '@media (prefers-color-scheme: dark)': 'lightgreen',
    '@media print': 'transparent',
  },
  otherColor: {
    default: 'antiquewhite',
    '@media (prefers-color-scheme: dark)': 'floralwhite',
  },
  radius: '6px'
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
    PluginPass::new(
      None,
      Some(FileName::Real(
        "/stylex/packages/src/css/vars.stylex.js".into()
      ))
    ),
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  theme_object_deep_in_file_tree,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

export const theme = stylex.createTheme(vars, {
  color: {
    default: 'green',
    '@media (prefers-color-scheme: dark)': 'lightgreen',
    '@media print': 'transparent',
  },
  otherColor: {
    default: 'antiquewhite',
    '@media (prefers-color-scheme: dark)': 'floralwhite',
  },
  radius: '6px'
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
    PluginPass::new(None, None),
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  literal_tokens_theme_object,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  "--color": "var(--color)",
  "--otherColor": "var(--otherColor)",
  "--radius": "var(--radius)",
  __varGroupHash__: "xop34xu"
};

export const theme = stylex.createTheme(vars, {
  '--color': 'green',
  '--otherColor': 'purple',
  '--radius': 6
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
    PluginPass::new(None, None),
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  local_variable_theme_object,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

const themeObj = {
  color: {
    default: 'green',
    '@media (prefers-color-scheme: dark)': 'lightgreen',
    '@media print': 'transparent',
  },
  otherColor: {
    default: 'antiquewhite',
    '@media (prefers-color-scheme: dark)': 'floralwhite',
  },
  radius: '6px'
};
export const theme = stylex.createTheme(vars, themeObj);
  "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_with_pass(
    tr.comments.clone(),
    PluginPass::new(None, None),
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  local_variables_used_in_theme_objects,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

const RADIUS = 10;
export const theme = stylex.createTheme(vars, {
  radius: RADIUS
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
    PluginPass::new(None, None),
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  template_literals_used_in_theme_objects,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

const name = 'light';
export const theme = stylex.createTheme(vars, {
  color: `${name}green`
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
    PluginPass::new(None, None),
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  expressions_used_in_theme_objects,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

const RADIUS = 10;
export const theme = stylex.createTheme(vars, {
  radius: RADIUS * 2
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
    PluginPass::new(None, None),
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  stylex_types_used_in_theme_object,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

const RADIUS = 10;
export const theme = stylex.createTheme(vars, {
  color: stylex.types.color({
    default: 'green',
    '@media (prefers-color-scheme: dark)': 'lightgreen',
    '@media print': 'transparent',
  }),
  otherColor: stylex.types.color({
    default: 'antiquewhite',
    '@media (prefers-color-scheme: dark)': 'floralwhite',
  }),
  radius: stylex.types.length({ default: RADIUS * 2 })
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
    PluginPass::new(None, None),
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  multiple_theme_objects_same_vars,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

export const theme = stylex.createTheme(vars, {
  color: {
    default: 'green',
    '@media (prefers-color-scheme: dark)': 'lightgreen',
    '@media print': 'transparent',
  },
  otherColor: {
    default: 'antiquewhite',
    '@media (prefers-color-scheme: dark)': 'floralwhite',
  },
  radius: '6px'
});
export const otherTheme = stylex.createTheme(vars, {
  color: 'skyblue',
  radius: '8px',
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
    PluginPass::new(
      None,
      Some(FileName::Real(
        "/stylex/packages/otherVars.stylex.js".into()
      ))
    ),
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  multiple_theme_objects_different_vars,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

export const theme = stylex.createTheme(vars, {
  color: {
    default: 'green',
    '@media (prefers-color-scheme: dark)': 'lightgreen',
    '@media print': 'transparent',
  },
  otherColor: {
    default: 'antiquewhite',
    '@media (prefers-color-scheme: dark)': 'floralwhite',
  },
  radius: '6px'
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
    PluginPass::new(None, None),
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  themes_are_indifferent_to_order_of_keys,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

export const theme = stylex.createTheme(vars, {
  radius: '6px',
  otherColor: {
    default: 'antiquewhite',
    '@media (prefers-color-scheme: dark)': 'floralwhite',
  },
  color: {
    default: 'green',
    '@media (prefers-color-scheme: dark)': 'lightgreen',
    '@media print': 'transparent',
  }
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
      filename: FileName::Real("/html/js/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  debug_adds_debug_data,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

export const theme = stylex.createTheme(vars, {
  color: 'orange'
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
      filename: FileName::Real("/js/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  debug_adds_debug_data_for_npm_packages,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

export const theme = stylex.createTheme(vars, {
  color: 'orange'
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
      filename: FileName::Real("/html/js/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      ..StyleXOptionsParams::default()
    })
  ),
  debug_adds_debug_data_haste,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

export const theme = stylex.createTheme(vars, {
  color: 'orange'
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
      filename: FileName::Real("/node_modules/npm-package/dist/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      debug: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      ..StyleXOptionsParams::default()
    })
  ),
  debug_adds_debug_data_for_npm_packages_haste,
  r#"
import * as stylex from '@stylexjs/stylex';
export const vars = {
  color: "var(--xt4ziaz)",
  otherColor: "var(--x1e3it8h)",
  radius: "var(--x1onrunl)",
  __varGroupHash__: "x1xohuxq"
};

export const theme = stylex.createTheme(vars, {
  color: 'orange'
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
      filename: FileName::Real("/html/js/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  adds_dev_data,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xwx8imx)",
      otherColor: "var(--xaaua2w)",
      radius: "var(--xbbre8)",
      __varGroupHash__: "xop34xu"
    };

    export const theme = stylex.createTheme(vars, {
      color: 'orange'
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
      filename: FileName::Real("/html/js/components/Foo.react.js".into()),
    },
    Some(&mut StyleXOptionsParams {
      runtime_injection: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string()
      ))),
      ..StyleXOptionsParams::default()
    })
  ),
  options_runtime_injection_true,
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const vars = {
      color: "var(--xwx8imx)",
      otherColor: "var(--xaaua2w)",
      radius: "var(--xbbre8)",
      __varGroupHash__: "xop34xu"
    };

    export const theme = stylex.createTheme(vars, {
      color: 'orange'
    });
  "#
);
