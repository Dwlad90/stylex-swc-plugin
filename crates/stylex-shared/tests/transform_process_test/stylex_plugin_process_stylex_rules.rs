use crate::utils::transform::stringify_js;
use insta::assert_snapshot;
use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{ModuleResolution, StyleXOptionsParams},
  },
};
use swc_core::common::FileName;
use swc_core::ecma::parser::{Syntax, TsSyntax};

fn transform(source: &str, opts: Option<StyleXOptionsParams>) -> String {
  let cwd = std::env::current_dir().unwrap();
  let fixture_dir = cwd.join("tests/fixture/consts");

  let plugin_opts = opts.unwrap_or_else(|| StyleXOptionsParams {
    unstable_module_resolution: Some(ModuleResolution {
      r#type: "commonJS".to_string(),
      root_dir: Some(fixture_dir.to_string_lossy().to_string()),
      theme_file_extension: None,
    }),
    ..Default::default()
  });

  // Transform main file

  stringify_js(
    source,
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    |tr| {
      StyleXTransform::new_test_with_pass(
        tr.comments.clone(),
        PluginPass {
          cwd: Some(fixture_dir.clone()),
          filename: FileName::Real(fixture_dir.join("main.stylex.js")),
        },
        Some(&mut plugin_opts.clone()),
      )
    },
  )
}

const FIXTURE: &str = r#"
import * as stylex from '@stylexjs/stylex';
import { constants } from './input.stylex';
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
        default: 'blue',
        [constants.mediaSmall]: 'yellow',
      }
    },
    textShadow: {
      default: '1px 2px 3px 4px red',
      '@media (min-width:320px)': '10px 20px 30px 40px green'
    }
  }
});
"#;

#[test]
fn no_rules() {
  let input = r#"
        import * as stylex from '@stylexjs/stylex';
      "#;
  let code = transform(input, None);
  assert_snapshot!("no_rules_code", code);
}

#[test]
fn all_rules_use_layers_false() {
  let code = transform(FIXTURE, None);
  assert_snapshot!("all_rules_use_layers_false_code", code);
}

#[test]
fn all_rules_use_layers_true() {
  let opts = StyleXOptionsParams {
    unstable_module_resolution: Some(ModuleResolution {
      r#type: "commonJS".to_string(),
      root_dir: Some(
        std::env::current_dir()
          .unwrap()
          .join("tests/fixture/consts")
          .to_string_lossy()
          .to_string(),
      ),
      theme_file_extension: None,
    }),
    // TODO: Add useLayers option when available in StyleXOptionsParams
    ..Default::default()
  };
  let code = transform(FIXTURE, Some(opts));
  assert_snapshot!("all_rules_use_layers_true_code", code);
}
