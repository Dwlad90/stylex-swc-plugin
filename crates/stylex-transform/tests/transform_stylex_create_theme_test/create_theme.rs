use crate::utils::prelude::*;
use swc_core::{common::FileName, ecma::transforms::testing::test};

stylex_test!(
  theme_object,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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

stylex_test!(
  theme_object_haste,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_unstable_module_resolution(ModuleResolution::haste(None))
    .into_pass(),
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

stylex_test!(
  theme_object_deep_in_file_tree,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_filename(FileName::Real(
      "/stylex/packages/src/css/vars.stylex.js".into()
    ))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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

stylex_test!(
  literal_tokens_theme_object,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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

stylex_test!(
  local_variable_theme_object,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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

stylex_test!(
  local_variables_used_in_theme_objects,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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

stylex_test!(
  template_literals_used_in_theme_objects,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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

stylex_test!(
  expressions_used_in_theme_objects,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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

stylex_test!(
  stylex_types_used_in_theme_object,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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

stylex_test!(
  multiple_theme_objects_same_vars,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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

stylex_test!(
  multiple_theme_objects_different_vars,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_filename(FileName::Real(
      "/stylex/packages/otherVars.stylex.js".into()
    ))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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

stylex_test!(
  themes_are_indifferent_to_order_of_keys,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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

stylex_test!(
  debug_adds_debug_data,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_filename(FileName::Real("/html/js/components/Foo.react.js".into()))
    .with_debug(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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

stylex_test!(
  debug_adds_debug_data_for_npm_packages,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_filename(FileName::Real(
      "/js/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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

stylex_test!(
  debug_adds_debug_data_haste,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_filename(FileName::Real("/html/js/components/Foo.react.js".into()))
    .with_debug(true)
    .with_unstable_module_resolution(ModuleResolution::haste(None))
    .into_pass(),
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

stylex_test!(
  debug_adds_debug_data_for_npm_packages_haste,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_filename(FileName::Real(
      "/node_modules/npm-package/dist/components/Foo.react.js".into()
    ))
    .with_debug(true)
    .with_unstable_module_resolution(ModuleResolution::haste(None))
    .into_pass(),
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

stylex_test!(
  adds_dev_data,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_filename(FileName::Real("/html/js/components/Foo.react.js".into()))
    .with_dev(true)
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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

stylex_test!(
  options_runtime_injection_true,
  |tr| StyleXTransform::test(tr.comments.clone())
    .with_pass(PluginPass::test_default())
    .with_filename(FileName::Real("/html/js/components/Foo.react.js".into()))
    .with_runtime_injection_option(RuntimeInjection::Boolean(true))
    .with_unstable_module_resolution(ModuleResolution::common_js(Some(
      "/stylex/packages/".to_string()
    )))
    .into_pass(),
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
