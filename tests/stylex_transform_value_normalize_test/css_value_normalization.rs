use stylex_swc_plugin::{
  shared::structures::{
    named_import_source::RuntimeInjection, plugin_pass::PluginPass,
    stylex_options::StyleXOptionsParams,
  },
  ModuleTransformVisitor,
};
use swc_core::ecma::{
  parser::{Syntax, TsConfig},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  normalize_whitespace_in_css_values_transform,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        x: {
          transform: '  rotate(10deg)  translate3d( 0 , 0 , 0 )  '
        }
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
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  normalize_whitespace_in_css_values_color,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({ x: { color: 'rgba( 1, 222,  33 , 0.5)' } });
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  no_dimensions_for_zero_values,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({ x: {
        margin: '0px',
        marginLeft: '1px'
      } });
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  zero_timings_are_all_zero_s,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({ x: { transitionDuration: '500ms' } });
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  zero_angles_are_all_zero_deg,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        x: { transform: '0rad' },
        y: { transform: '0turn' },
        z: { transform: '0grad' }
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
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  calc_preserves_spaces_around_plus_and_minus,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({ x: { width: 'calc((100% + 3% -   100px) / 7)' } });
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  strip_leading_zeros,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({ x: {
        transitionDuration: '0.01s',
        transitionTimingFunction: 'cubic-bezier(.08,.52,.52,1)'
      } });
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  use_double_quotes_in_empty_strings,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({ x: { quotes: "''" } });
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  timing_values_are_converted_to_seconds_unless_than_ten_ms,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        x: { transitionDuration: '1234ms' },
        y: { transitionDuration: '10ms' },
        z: { transitionDuration: '1ms' }
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
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  transforms_non_unitless_property_values,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        normalize: {
          height: 500,
          margin: 10,
          width: 500
        },
        unitless: {
          fontWeight: 500,
          lineHeight: 1.5,
          opacity: 0.5
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
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  number_values_rounded_down_to_four_decimal_points,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({ x: { height: 100 / 3 } });
    "#
);

test!(
  Syntax::Typescript(TsConfig {
    tsx: true,
    ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(
    tr.comments.clone(),
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  content_property_values_are_wrapped_in_quotes,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({
        default: {
          content: '',
        },
        other: {
          content: 'next',
        },
        withQuotes: {
          content: '"prev"',
        }
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
    PluginPass::default(),
    Some(StyleXOptionsParams {
      runtime_injection: Option::Some(RuntimeInjection::Boolean(true)),
      use_rem_for_font_size: Option::Some(true),
      ..StyleXOptionsParams::default()
    })
  ),
  legacy_no_space_before_bang_important,
  r#"
      import stylex from 'stylex';
      const styles = stylex.create({ x: { color: 'red !important' } });
    "#
);
