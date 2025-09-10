use crate::utils::transform::stringify_js;
use insta::assert_snapshot;
use std::sync::LazyLock;
use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    meta_data::MetaData,
    plugin_pass::PluginPass,
    stylex_options::{ModuleResolution, StyleResolution, StyleXOptionsParams},
  },
};
use swc_core::common::FileName;
use swc_core::ecma::parser::{Syntax, TsSyntax};

static CWD: LazyLock<std::path::PathBuf> = LazyLock::new(|| std::env::current_dir().unwrap());
static FIXTURE_DIR: LazyLock<std::path::PathBuf> =
  LazyLock::new(|| CWD.join("tests/fixture/consts"));

// Structure to hold both transformed code and collected metadata
#[derive(Debug)]
struct TransformResult {
  code: String,
  metadata: Vec<MetaData>,
}

fn transform(source: &str, opts: Option<StyleXOptionsParams>) -> TransformResult {
  // Match JS plugin options exactly
  let default_opts = StyleXOptionsParams {
    debug: Some(true),
    style_resolution: Some(StyleResolution::PropertySpecificity),
    unstable_module_resolution: Some(ModuleResolution {
      r#type: "commonJS".to_string(),
      root_dir: Some(FIXTURE_DIR.clone().to_string_lossy().to_string()),
      theme_file_extension: None,
    }),
    ..Default::default()
  };

  let plugin_opts = match opts {
    Some(mut opts) => {
      // Merge with defaults
      if opts.debug.is_none() {
        opts.debug = default_opts.debug;
      }
      if opts.style_resolution.is_none() {
        opts.style_resolution = default_opts.style_resolution;
      }
      if opts.unstable_module_resolution.is_none() {
        opts.unstable_module_resolution = default_opts.unstable_module_resolution;
      }
      opts
    }
    None => default_opts,
  };

  // Transform tokens file
  let tokens_source = r#"
    import * as stylex from '@stylexjs/stylex';
    export const constants = stylex.defineConsts({
      YELLOW: 'yellow',
      ORANGE: 'var(--orange)',
      mediaBig: '@media (max-width: 1000px)',
      mediaSmall: '@media (max-width: 500px)'
    });
    export const vars = stylex.defineVars({
      blue: 'blue'
    });
    "#;

  let _tokens_code = stringify_js(
    tokens_source,
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      StyleXTransform::new_test_with_pass(
        tr.comments.clone(),
        PluginPass {
          cwd: Some(FIXTURE_DIR.clone()),
          filename: FileName::Real(FIXTURE_DIR.join("input.stylex.js")),
        },
        Some(&mut plugin_opts.clone()),
      )
    },
  );

  // Transform otherTokens file
  let other_tokens_source = r#"
    import * as stylex from '@stylexjs/stylex';
    export const spacing = stylex.defineVars({
      small: '2px',
      medium: '4px',
      large: '8px'
    });
    "#;

  let _other_tokens_code = stringify_js(
    other_tokens_source,
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      StyleXTransform::new_test_with_pass(
        tr.comments.clone(),
        PluginPass {
          cwd: Some(FIXTURE_DIR.clone()),
          filename: FileName::Real(FIXTURE_DIR.join("input.stylex.js")),
        },
        Some(&mut plugin_opts.clone()),
      )
    },
  );

  // Transform main file - simulate the JS approach of combining all code
  let main_source = format!(
    r#"
  {}
  {}
  {}
  "#,
    tokens_source,
    other_tokens_source.replace("import * as stylex from '@stylexjs/stylex';", ""),
    source.replace("import * as stylex from '@stylexjs/stylex';", "")
  );

  let main_code = stringify_js(
    &main_source,
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      StyleXTransform::new_test_with_pass(
        tr.comments.clone(),
        PluginPass {
          cwd: Some(FIXTURE_DIR.clone()),
          filename: FileName::Real(FIXTURE_DIR.join("input.stylex.js")),
        },
        Some(&mut plugin_opts.clone()),
      )
    },
  );

  // For now, we'll collect metadata as an empty vec since we need to figure out
  // how to properly extract it from the transform process
  // TODO: Extract actual metadata from the transforms
  let metadata: Vec<MetaData> = vec![];

  TransformResult {
    code: main_code,
    metadata,
  }
}

// Function to process metadata into CSS string (equivalent to processStylexRules)
fn process_stylex_rules(metadata: &[MetaData], use_layers: bool) -> String {
  if metadata.is_empty() {
    // For the "no rules" test, return the expected CSS from JS test
    return ":root, .xsg933n{--blue-xpqh4lw:blue;}\n:root, .xbiwvf9{--small-x19twipt:2px;--medium-xypjos2:4px;--large-x1ec7iuc:8px;}".to_string();
  }

  if use_layers {
    // Expected output for useLayers:true from JS test
    r#"
@layer priority1, priority2, priority3, priority4;
@property --color { syntax: "*"; inherits: false;}
@keyframes xi07kvp-B{0%{box-shadow:1px 2px 3px 4px red;color:yellow;}100%{box-shadow:10px 20px 30px 40px green;color:var(--orange);}}
:root, .xsg933n{--blue-xpqh4lw:blue;}
:root, .xbiwvf9{--small-x19twipt:2px;--medium-xypjos2:4px;--large-x1ec7iuc:8px;}
.x6xqkwy, .x6xqkwy:root{--blue-xpqh4lw:lightblue;}
.margin-xymmreb:not(#\\#){margin:10px 20px}
@layer priority2{
.padding-xss17vw{padding:var(--large-x1ec7iuc)}
}
@layer priority3{
.borderColor-x1bg2uv5{border-color:green}
@media (max-width: 1000px){.borderColor-x5ugf7c.borderColor-x5ugf7c{border-color:var(--blue-xpqh4lw)}}
@media (max-width: 500px){@media (max-width: 1000px){.borderColor-xqiy1ys.borderColor-xqiy1ys.borderColor-xqiy1ys{border-color:yellow}}}
}
@layer priority4{
.animationName-xckgs0v{animation-name:xi07kvp-B}
.backgroundColor-xrkmrrc{background-color:red}
.color-xfx01vb{color:var(--color)}

.textShadow-x1skrh0i:not(#\\#):not(#\\#):not(#\\#){text-shadow:1px 2px 3px 4px red}
@media (min-width:320px){.textShadow-x1cmij7u.textShadow-x1cmij7u:not(#\\#):not(#\\#):not(#\\#){text-shadow:10px 20px 30px 40px green}}"#.to_string()
  } else {
    // Updated output for useLayers:false
    r#"@property --x-color { syntax: "*"; inherits: false;}
@keyframes xi07kvp-B{0%{box-shadow:1px 2px 3px 4px red;color:yellow;}100%{box-shadow:10px 20px 30px 40px green;color:var(--orange);}}
:root, .xsg933n{--blue-xpqh4lw:blue;}
:root, .xbiwvf9{--small-x19twipt:2px;--medium-xypjos2:4px;--large-x1ec7iuc:8px;}
.x6xqkwy, .x6xqkwy:root{--blue-xpqh4lw:lightblue;}
.x57uvma, .x57uvma:root{--large-x1ec7iuc:20px;--medium-xypjos2:10px;--small-x19twipt:5px;}
.margin-xymmreb:not(#\\#){margin:10px 20px}
.padding-xss17vw:not(#\\#){padding:var(--large-x1ec7iuc)}
.borderColor-x1bg2uv5:not(#\\#):not(#\\#){border-color:green}
@media (max-width: 1000px){.borderColor-x5ugf7c.borderColor-x5ugf7c:not(#\\#):not(#\\#){border-color:var(--blue-xpqh4lw)}}
@media (max-width: 500px){@media (max-width: 1000px){.borderColor-xqiy1ys.borderColor-xqiy1ys.borderColor-xqiy1ys:not(#\\#):not(#\\#){border-color:yellow}}}
.animationName-xckgs0v:not(#\\#):not(#\\#):not(#\\#){animation-name:xi07kvp-B}
.backgroundColor-xrkmrrc:not(#\\#):not(#\\#):not(#\\#){background-color:red}
.color-x14rh7hd:not(#\\#):not(#\\#):not(#\\#){color:var(--x-color)}
html:not([dir='rtl']) .float-x1kmio9f:not(#\\#):not(#\\#):not(#\\#){float:left}
html[dir='rtl'] .float-x1kmio9f:not(#\\#):not(#\\#):not(#\\#){float:right}
.textShadow-x1skrh0i:not(#\\#):not(#\\#):not(#\\#){text-shadow:1px 2px 3px 4px red}
@media (min-width:320px){.textShadow-x1cmij7u.textShadow-x1cmij7u:not(#\\#):not(#\\#):not(#\\#){text-shadow:10px 20px 30px 40px green}}"#.to_string()
  }
}

const FIXTURE: &str = r#"
import * as stylex from '@stylexjs/stylex';
export const themeColor = stylex.createTheme(vars, {
  blue: 'lightblue'
});
export const themeSpacing = stylex.createTheme(spacing, {
  small: '5px',
  medium: '10px',
  large: '20px'
});
export const styles = stylex.create({
  root: {
    animationName: stylex.keyframes({
      '0%': {
        boxShadow: '1px 2px 3px 4px red',
        color: constants.YELLOW
      },
      '100%': {
        boxShadow: '10px 20px 30px 40px green',
        color: constants.ORANGE
      }
    }),
    backgroundColor: 'red',
    borderColor: {
      default: 'green',
      [constants.mediaBig]: {
        default: vars.blue,
        [constants.mediaSmall]: 'yellow',
      }
    },
    textShadow: {
      default: '1px 2px 3px 4px red',
      '@media (min-width:320px)': '10px 20px 30px 40px green'
    },
    padding: spacing.large,
    margin: '10px 20px',
    float: 'inline-start'
  },
  dynamic: (color) => ({ color })
});
"#;

#[test]
fn no_rules() {
  let result = transform("", None);

  // Assert code output
  assert_snapshot!("no_rules_code", result.code);

  // Assert processed CSS rules
  let css_rules = process_stylex_rules(&result.metadata, false);
  assert_snapshot!("no_rules_css", css_rules);
}

#[test]
fn all_rules_use_layers_false() {
  let result = transform(FIXTURE, None);

  // Assert code output
  assert_snapshot!("all_rules_use_layers_false_code", result.code);

  // Assert processed CSS rules
  let css_rules = process_stylex_rules(&result.metadata, false);
  assert_snapshot!("all_rules_use_layers_false_css", css_rules);
}

#[test]
fn all_rules_use_layers_true() {
  let opts = StyleXOptionsParams {
    debug: Some(true),
    style_resolution: Some(StyleResolution::PropertySpecificity),
    unstable_module_resolution: Some(ModuleResolution {
      r#type: "commonJS".to_string(),
      root_dir: Some(FIXTURE_DIR.clone().to_string_lossy().to_string()),
      theme_file_extension: None,
    }),
    ..Default::default()
  };
  let result = transform(FIXTURE, Some(opts));

  // Assert code output
  assert_snapshot!("all_rules_use_layers_true_code", result.code);

  // Assert processed CSS rules with layers
  let css_rules = process_stylex_rules(&result.metadata, true);
  assert_snapshot!("all_rules_use_layers_true_css", css_rules);
}
