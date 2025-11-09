use insta::assert_snapshot;
use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    named_import_source::{ImportSources, NamedImportSource},
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

use crate::utils::transform::stringify_js;

fn get_default_opts() -> StyleXOptionsParams {
  StyleXOptionsParams {
    unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
      "/stylex/packages/".to_string(),
    ))),
    ..StyleXOptionsParams::default()
  }
}

fn transform(input: &str) -> String {
  transform_with_options(input, None, None, None)
}

#[derive(Debug, Clone)]
struct FixtureImportMap {
  create: &'static str,
  create_theme: &'static str,
  define_consts: &'static str,
  define_vars: &'static str,
  first_that_works: &'static str,
  keyframes: &'static str,
  position_try: &'static str,
  props: &'static str,
  view_transition_class: &'static str,
}

impl Default for FixtureImportMap {
  fn default() -> Self {
    Self {
      create: "stylex.create",
      create_theme: "stylex.createTheme",
      define_consts: "stylex.defineConsts",
      define_vars: "stylex.defineVars",
      first_that_works: "stylex.firstThatWorks",
      keyframes: "stylex.keyframes",
      position_try: "stylex.positionTry",
      props: "stylex.props",
      view_transition_class: "stylex.viewTransitionClass",
    }
  }
}

/// Rust version of the JS createStylesFixture
fn create_styles_fixture(
  import_text: Option<&str>,
  import_source: Option<&str>,
  import_map: Option<FixtureImportMap>,
) -> String {
  let import_text = import_text.unwrap_or("* as stylex");
  let import_source = import_source.unwrap_or("@stylexjs/stylex");
  let import_map = import_map.unwrap_or_default();

  let from = import_source;

  // Generate the defineConsts and defineVars output first (like JS version)
  let define_consts_and_vars_input = format!(
    r#"import {} from '{}';
export const constants = {}({{
  mediaQuery: '@media (min-width: 768px)',
}});
export const vars = {}({{
  bar: 'left'
}});
"#,
    import_text, from, import_map.define_consts, import_map.define_vars
  );

  let define_consts_and_vars_output = transform_with_options(
    &define_consts_and_vars_input,
    Some(&mut StyleXOptionsParams {
      unstable_module_resolution: Some(StyleXOptions::get_common_js_module_resolution(Some(
        "/stylex/packages/".to_string(),
      ))),
      ..StyleXOptionsParams::default()
    }),
    Some(FileName::Real("/stylex/packages/vars.stylex.js".into())),
    None,
  );

  let formatted = format!(
    r#"{define_consts_and_vars_output}
        const viewTransition1 = {view_transition_class}({{
          group: {{
            transitionProperty: 'none',
          }},
          imagePair: {{
            borderRadius: 16,
          }},
          old: {{
            animationDuration: '0.5s',
          }},
          new: {{
            animationTimingFunction: 'ease-out',
          }},
        }});
        const fallback1 = {position_try}({{
          anchorName: '--myAnchor',
          positionArea: 'top left',
        }});
        const fallback2 = {position_try}({{
          anchorName: '--otherAnchor',
          top: 'anchor(bottom)',
          insetInlineStart: 'anchor(start)',
        }});
        const styles = {create}({{
          root: {{
            animationName: {keyframes}({{
              from: {{
                backgroundColor: 'yellow'
              }},
              to: {{
                backgroundColor: 'orange'
              }},
            }}),
            positionTryFallbacks: `${{fallback1}}, ${{fallback2}}`,
            color: {{
              default: 'red',
              [constants.mediaQuery]: 'blue'
            }},
            position: {first_that_works}('sticky', 'fixed')
          }}
        }});

        const theme = {create_theme}(vars, {{
          bar: 'green'
        }});

        {props}(styles.root, theme);
    "#,
    define_consts_and_vars_output = define_consts_and_vars_output,
    view_transition_class = import_map.view_transition_class,
    position_try = import_map.position_try,
    create = import_map.create,
    keyframes = import_map.keyframes,
    first_that_works = import_map.first_that_works,
    create_theme = import_map.create_theme,
    props = import_map.props,
  );

  formatted
}

fn transform_with_options(
  input: &str,
  options: Option<&mut StyleXOptionsParams>,
  filename: Option<FileName>,
  import_sources: Option<Vec<ImportSources>>,
) -> String {
  let mut opts = options.cloned().unwrap_or_else(get_default_opts);
  if let Some(sources) = import_sources {
    opts.import_sources = Some(sources);
  }

  let syntax = Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  });
  stringify_js(input, syntax, |tr| {
    stylex_shared::StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass {
        cwd: None,
        filename: filename.unwrap_or(FileName::Real("/stylex/packages/TestFile.js".into())),
      },
      Some(&mut opts),
    )
  })
}

// Tests matching the JS file structure

#[test]
fn import_none() {
  let input = r#"
        export const styles = stylex.create({
          root: {
            color: 'red'
          }
        });
    "#;
  let output = transform(input);
  assert_snapshot!(output);
}

#[test]
fn import_non_stylex() {
  let input = r#"
        import {foo, bar} from 'other';
    "#;
  let output = transform(input);
  assert_snapshot!(output);
}

#[test]
fn require_non_stylex() {
  let input = r#"
        const {foo, bar} = require('other');
    "#;
  let output = transform(input);
  assert_snapshot!(output);
}

#[test]
fn import_wildcard_the_default() {
  let fixture = create_styles_fixture(None, None, None);
  let output = transform(&fixture);
  assert_snapshot!(output);
}

#[test]
fn import_wildcard_non_stylex_name() {
  let fixture = create_styles_fixture(
    Some("* as foo"),
    None,
    Some(FixtureImportMap {
      create: "foo.create",
      create_theme: "foo.createTheme",
      define_consts: "foo.defineConsts",
      define_vars: "foo.defineVars",
      first_that_works: "foo.firstThatWorks",
      keyframes: "foo.keyframes",
      position_try: "foo.positionTry",
      props: "foo.props",
      view_transition_class: "foo.viewTransitionClass",
    }),
  );
  let output = transform(&fixture);
  assert_snapshot!(output);
}

#[test]
fn import_named() {
  let fixture = create_styles_fixture(
    Some(
      "{create, createTheme, defineConsts, defineVars, firstThatWorks, keyframes, positionTry, props, viewTransitionClass}",
    ),
    None,
    Some(FixtureImportMap {
      create: "create",
      create_theme: "createTheme",
      define_consts: "defineConsts",
      define_vars: "defineVars",
      first_that_works: "firstThatWorks",
      keyframes: "keyframes",
      position_try: "positionTry",
      props: "props",
      view_transition_class: "viewTransitionClass",
    }),
  );
  let output = transform(&fixture);
  assert_snapshot!(output);
}

#[test]
fn import_named_alias() {
  let fixture = create_styles_fixture(
    Some(
      r#"{
          create as _create,
          createTheme as _createTheme,
          defineConsts as _defineConsts,
          defineVars as _defineVars,
          firstThatWorks as _firstThatWorks,
          keyframes as _keyframes,
          positionTry as _positionTry,
          props as _props,
          viewTransitionClass as _viewTransitionClass
        }"#,
    ),
    None,
    Some(FixtureImportMap {
      create: "_create",
      create_theme: "_createTheme",
      define_consts: "_defineConsts",
      define_vars: "_defineVars",
      first_that_works: "_firstThatWorks",
      keyframes: "_keyframes",
      position_try: "_positionTry",
      props: "_props",
      view_transition_class: "_viewTransitionClass",
    }),
  );
  let output = transform(&fixture);
  assert_snapshot!(output);
}

#[test]
fn import_sources_string() {
  let import_source = "foo-bar";
  let fixture = create_styles_fixture(Some("* as stylex"), Some(import_source), None);

  let output = transform_with_options(
    &fixture,
    Some(&mut get_default_opts()),
    Some(FileName::Real("/stylex/packages/vars.stylex.js".into())),
    Some(vec![ImportSources::Regular(import_source.to_string())]),
  );
  assert_snapshot!(output);
}

#[test]
fn import_sources_object() {
  let fixture = create_styles_fixture(
    Some("{css, html}"),
    Some("react-strict-dom"),
    Some(FixtureImportMap {
      create: "css.create",
      create_theme: "css.createTheme",
      define_consts: "css.defineConsts",
      define_vars: "css.defineVars",
      first_that_works: "css.firstThatWorks",
      keyframes: "css.keyframes",
      position_try: "css.positionTry",
      props: "css.props",
      view_transition_class: "css.viewTransitionClass",
    }),
  );

  let output = transform_with_options(
    &fixture,
    Some(&mut get_default_opts()),
    Some(FileName::Real("/stylex/packages/vars.stylex.js".into())),
    Some(vec![ImportSources::Named(NamedImportSource {
      r#as: "css".to_string(),
      from: "react-strict-dom".to_string(),
    })]),
  );
  assert_snapshot!(output);
}

#[test]
#[ignore = "META-ONLY test, not supported in OSS"]
fn meta_only_import_default() {
  let fixture = create_styles_fixture(Some("stylex"), Some("stylex"), None);
  let output = transform(&fixture);
  assert_snapshot!(output);
}

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut StyleXOptionsParams {
        inject_stylex_side_effects: Some(true),
        ..Default::default()
      }),
    )
  },
  inject_stylex_side_effects,
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { constants } from './constants.consts';
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut StyleXOptionsParams {
        inject_stylex_side_effects: Some(true),
        ..Default::default()
      }),
    )
  },
  inject_stylex_theme_side_effects,
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { theme } from './theme.stylex';
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut StyleXOptionsParams {
        inject_stylex_side_effects: Some(true),
        ..Default::default()
      }),
    )
  },
  inject_stylex_side_effects_ts,
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { constants } from './constants.consts.ts';
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| {
    StyleXTransform::new_test_with_pass(
      tr.comments.clone(),
      PluginPass::default(),
      Some(&mut StyleXOptionsParams {
        inject_stylex_side_effects: Some(true),
        ..Default::default()
      }),
    )
  },
  inject_stylex_theme_side_effects_ts,
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { theme } from './theme.stylex.ts';
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| { StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None,) },
  no_inject_stylex_side_effects_ts,
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { constants } from './constants.stylex';
    "#
);

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| { StyleXTransform::new_test_with_pass(tr.comments.clone(), PluginPass::default(), None,) },
  no_inject_stylex_side_effects,
  r#"
    import * as stylex from '@stylexjs/stylex';
    import { constants } from './constants.consts.ts';
    "#
);
