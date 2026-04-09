use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(b)
      .with_runtime_injection_option(RuntimeInjection::Boolean(true))
      .with_enable_font_size_px_to_rem(true)
  })
}

stylex_test!(
  normalize_whitespace_in_css_values_transform,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      x: {
        transform: '  rotate(10deg)  translate3d( 0 , 0 , 0 )  '
      }
    });
  "#
);

stylex_test!(
  normalize_whitespace_in_css_values_color,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { color: 'rgba( 1, 222,  33 , 0.5)' } });
  "#
);

stylex_test!(
  no_dimensions_for_zero_values,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: {
      margin: '0px',
      marginLeft: '1px'
    } });
  "#
);

stylex_test!(
  zero_timings_are_all_zero_s,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { transitionDuration: '500ms' } });
  "#
);

stylex_test!(
  zero_angles_are_all_zero_deg,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      x: { transform: '0rad' },
      y: { transform: '0turn' },
      z: { transform: '0grad' }
    });
  "#
);

stylex_test!(
  calc_preserves_spaces_around_plus_and_minus,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { width: 'calc((100% + 3% -   100px) / 7)' } });
  "#
);

stylex_test!(
  calc_preserves_spaces_around_minus_and_var,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    export const styles = stylex.create({ x: { width: 'calc(0 - var(--someVar))' } });
    export const styles2 = stylex.create({ x: { width: 'calc(0px - var(--someVar))' } });
  "#
);

stylex_test!(
  strip_leading_zeros,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: {
      transitionDuration: '0.01s',
      transitionTimingFunction: 'cubic-bezier(.08,.52,.52,1)'
    } });
  "#
);

stylex_test!(
  use_double_quotes_in_empty_strings,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { quotes: "''" } });
  "#
);

stylex_test!(
  timing_values_are_converted_to_seconds_unless_than_ten_ms,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({
      x: { transitionDuration: '1234ms' },
      y: { transitionDuration: '10ms' },
      z: { transitionDuration: '1ms' }
    });
  "#
);

stylex_test!(
  transforms_non_unitless_property_values,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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
        opacity: 0.5,
        zoom: 2
      },
    });
  "#
);

stylex_test!(
  number_values_rounded_down_to_four_decimal_points,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { height: 100 / 3 } });
  "#
);

stylex_test!(
  content_property_values_are_wrapped_in_quotes,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
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

stylex_test!(
  legacy_no_space_before_bang_important,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import stylex from 'stylex';
    const styles = stylex.create({ x: { color: 'red !important' } });
  "#
);
