// Every media-query expectation in the repository, one entry per expectation.
//
// `props` is spliced into `stylex.create({ x: <props> })`, so a unit-seam
// styles object and an end-to-end source describe the same subject. `ours` is
// the repository's expectation, transcribed from the file `origin` names: an
// ordered list of the `@media` prelude texts it pins, nested preludes joined
// with " >> ".

const UNIT = 'crates/stylex-css-parser/src/tests/at_queries/media_query_transform_test.rs';
const CANON = 'crates/stylex-transform/tests/transform_stylex_create_test/media_query_canonicalization.rs';
const BOUNDS = 'crates/stylex-transform/tests/transform_stylex_create_test/media_query_computed_bounds.rs';
const STATIC = 'crates/stylex-transform/tests/transform_stylex_create_test/static_styles.rs';
const ORDER = 'crates/stylex-transform/tests/transform_stylex_create_test/nested_pseudo_ordering.rs';

module.exports = [
  // ---- the transform's own unit seam -------------------------------------
  {
    id: 'u01', seam: 'unit', origin: `${UNIT}::basic_usage_multiple_widths`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (max-width: 1440px)': '1 / 4',
      '@media (max-width: 1024px)': '1 / 3',
      '@media (max-width: 768px)': '1 / -1',
    } },
    ours: [
      '@media (min-width: 1024.01px) and (max-width: 1440px)',
      '@media (min-width: 768.01px) and (max-width: 1024px)',
      '@media (max-width: 768px)',
    ],
  },
  {
    id: 'u02', seam: 'unit', origin: `${UNIT}::basic_usage_nested_query`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (max-width: 1440px)': {
        '@media (max-height: 1024px)': '1 / 3',
        '@media (max-height: 768px)': '1 / -1',
      },
      '@media (max-width: 1024px)': '1 / 3',
      '@media (max-width: 768px)': '1 / -1',
    } },
    ours: [
      '@media (min-width: 1024.01px) and (max-width: 1440px) >> @media (min-height: 768.01px) and (max-height: 1024px)',
      '@media (min-width: 1024.01px) and (max-width: 1440px) >> @media (max-height: 768px)',
      '@media (min-width: 768.01px) and (max-width: 1024px)',
      '@media (max-width: 768px)',
    ],
  },
  {
    id: 'u03', seam: 'unit', origin: `${UNIT}::basic_usage_nested_query_with_padding`,
    props: {
      gridColumn: {
        default: '1 / 2',
        '@media (max-width: 1440px)': {
          '@media (max-width: 1024px)': '1 / 3',
          '@media (max-width: 768px)': '1 / -1',
        },
        '@media (max-width: 1024px)': '1 / 3',
        '@media (max-width: 768px)': '1 / -1',
      },
      padding: '10px',
    },
    ours: [
      '@media (min-width: 1024.01px) and (max-width: 1440px) >> @media (min-width: 768.01px) and (max-width: 1024px)',
      '@media (min-width: 1024.01px) and (max-width: 1440px) >> @media (max-width: 768px)',
      '@media (min-width: 768.01px) and (max-width: 1024px)',
      '@media (max-width: 768px)',
    ],
  },
  {
    id: 'u04', seam: 'unit', origin: `${UNIT}::basic_usage_complex_object`,
    props: {
      gridColumn: {
        default: '1 / 2',
        '@media (max-width: 1440px)': '1 / 4',
        '@media (max-width: 1024px)': '1 / 3',
        '@media (max-width: 768px)': '1 / -1',
      },
      grid: { default: '1 / 2', '@media (max-width: 1440px)': '1 / 4' },
    },
    note: 'The third property of the original, `gridRow`, carries no media key.',
    ours: [
      '@media (min-width: 1024.01px) and (max-width: 1440px)',
      '@media (min-width: 768.01px) and (max-width: 1024px)',
      '@media (max-width: 768px)',
      '@media (max-width: 1440px)',
    ],
  },
  {
    id: 'u05', seam: 'unit', origin: `${UNIT}::basic_usage_lots_and_lots_of_max_widths`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (max-width: 1440px)': '1 / 4',
      '@media (max-width: 1024px)': '1 / 3',
      '@media (max-width: 768px)': '1 / -1',
      '@media (max-width: 458px)': '1 / -1',
    } },
    ours: [
      '@media (min-width: 1024.01px) and (max-width: 1440px)',
      '@media (min-width: 768.01px) and (max-width: 1024px)',
      '@media (min-width: 458.01px) and (max-width: 768px)',
      '@media (max-width: 458px)',
    ],
  },
  {
    id: 'u06', seam: 'unit', origin: `${UNIT}::basic_usage_lots_and_lots_of_min_widths`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (min-width: 768px)': '1 / -1',
      '@media (min-width: 1024px)': '1 / 3',
      '@media (min-width: 1440px)': '1 / 4',
    } },
    ours: [
      '@media (min-width: 768px) and (max-width: 1023.99px)',
      '@media (min-width: 1024px) and (max-width: 1439.99px)',
      '@media (min-width: 1440px)',
    ],
  },
  {
    id: 'u07', seam: 'unit', origin: `${UNIT}::basic_usage_multiple_heights`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (max-height: 1200px)': '1 / 4',
      '@media (max-height: 900px)': '1 / 3',
      '@media (max-height: 600px)': '1 / -1',
    } },
    ours: [
      '@media (min-height: 900.01px) and (max-height: 1200px)',
      '@media (min-height: 600.01px) and (max-height: 900px)',
      '@media (max-height: 600px)',
    ],
  },
  {
    id: 'u08', seam: 'unit', origin: `${UNIT}::basic_usage_min_max_heights`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (min-height: 1200px) and (max-height: 1400px)': '1 / 4',
      '@media (max-height: 900px)': '1 / 3',
      '@media (max-height: 600px)': '1 / -1',
    } },
    ours: [
      '@media (min-height: 1200px) and (max-height: 1400px)',
      '@media (min-height: 600.01px) and (max-height: 900px)',
      '@media (max-height: 600px)',
    ],
  },
  {
    id: 'u09', seam: 'unit', origin: `${UNIT}::single_word_condition`,
    props: { mixBlendMode: {
      default: 'normal',
      '@media (color)': 'color-burn',
      '@media (monochrome)': 'luminosity',
    } },
    note: 'The property is `mode` upstream of the compiler; a real property is used so the end-to-end run emits a rule.',
    ours: [
      '@media (color) and (not (monochrome))',
      '@media (monochrome)',
    ],
  },
  {
    id: 'u10', seam: 'unit', origin: `${UNIT}::handles_comma_separated_or_media_queries`,
    props: { width: {
      default: '100%',
      '@media screen, (max-width: 800px)': '80%',
      '@media (max-width: 500px)': '60%',
    } },
    ours: [
      '@media (screen) and (not (max-width: 500px)), (min-width: 500.01px) and (max-width: 800px)',
      '@media (max-width: 500px)',
    ],
  },
  {
    id: 'u11', seam: 'unit', origin: `${UNIT}::basic_usage_does_not_modify_single_queries`,
    props: { gridColumn: { default: '1 / 2', '@media (max-width: 1440px)': '1 / 4' } },
    ours: ['@media (max-width: 1440px)'],
  },
  {
    id: 'u12', seam: 'unit', origin: `${UNIT}::ignores_legacy_media_query_syntax`,
    props: { width: '100%', '@media (min-width: 600px)': { width: '50%' } },
    ours: ['@media (min-width: 600px)'],
  },
  {
    id: 'u13', seam: 'unit', origin: `${UNIT}::mixed_min_max_width_and_height`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (max-width: 1440px) and (max-height: 900px)': '1 / 4',
      '@media (max-width: 1024px)': '1 / 3',
      '@media (max-width: 768px)': '1 / -1',
    } },
    ours: [
      '@media (min-width: 1024.01px) and (max-width: 1440px) and (max-height: 900px)',
      '@media (min-width: 768.01px) and (max-width: 1024px)',
      '@media (max-width: 768px)',
    ],
  },
  {
    id: 'u14', seam: 'unit', origin: `${UNIT}::mixed_min_max_width_and_height_with_only_height_changing`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (max-width: 1440px) and (max-height: 900px)': '1 / 4',
      '@media (max-height: 700px)': '1 / 3',
      '@media (max-height: 500px)': '1 / -1',
    } },
    ours: [
      '@media (max-width: 1440px) and (min-height: 700.01px) and (max-height: 900px)',
      '@media (min-height: 500.01px) and (max-height: 700px)',
      '@media (max-height: 500px)',
    ],
  },
  {
    id: 'u15', seam: 'unit', origin: `${UNIT}::mixed_min_max_width_with_disjoint_ranges`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (max-width: 1440px) and (min-width: 900px)': '1 / 4',
      '@media (max-width: 800px) and (min-width: 600px)': '1 / 3',
    } },
    ours: [
      '@media (min-width: 900px) and (max-width: 1440px)',
      '@media (min-width: 600px) and (max-width: 800px)',
    ],
  },
  {
    id: 'u16', seam: 'unit', origin: `${UNIT}::mixed_min_max_width_with_many_disjoint_ranges`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (max-width: 1440px) and (min-width: 900px)': '1 / 4',
      '@media (max-width: 800px) and (min-width: 600px)': '1 / 3',
      '@media (max-width: 500px) and (min-width: 400px)': '1 / 1',
    } },
    ours: [
      '@media (min-width: 900px) and (max-width: 1440px)',
      '@media (min-width: 600px) and (max-width: 800px)',
      '@media (min-width: 400px) and (max-width: 500px)',
    ],
  },
  {
    id: 'u17', seam: 'unit', origin: `${UNIT}::mixed_min_max_width_with_mixed_ranges`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (max-width: 1440px) and (min-width: 900px)': '1 / 4',
      '@media (max-width: 1100px) and (min-width: 1000px)': '1 / 3',
      '@media (max-width: 500px) and (min-width: 400px)': '1 / 1',
    } },
    ours: [
      '@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))',
      '@media (min-width: 1000px) and (max-width: 1100px)',
      '@media (min-width: 400px) and (max-width: 500px)',
    ],
  },
  {
    id: 'u18', seam: 'unit', origin: `${UNIT}::mixed_min_max_width_with_intersecting_ranges`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (max-width: 1440px) and (min-width: 900px)': '1 / 4',
      '@media (max-width: 1100px) and (min-width: 1000px)': '1 / 3',
    } },
    ours: [
      '@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))',
      '@media (min-width: 1000px) and (max-width: 1100px)',
    ],
  },
  {
    id: 'u19', seam: 'unit', origin: `${UNIT}::mixed_min_max_width_with_many_intersecting_ranges`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (max-width: 1440px) and (min-width: 900px)': '1 / 4',
      '@media (max-width: 1100px) and (min-width: 1000px)': '1 / 3',
      '@media (max-width: 1050px) and (min-width: 1010px)': '1 / -1',
    } },
    ours: [
      '@media ((min-width: 900px) and (max-width: 999.99px)) or ((min-width: 1100.01px) and (max-width: 1440px))',
      '@media ((min-width: 1000px) and (max-width: 1009.99px)) or ((min-width: 1050.01px) and (max-width: 1100px))',
      '@media (min-width: 1010px) and (max-width: 1050px)',
    ],
  },
  {
    id: 'u20', seam: 'unit', origin: `${UNIT}::mixed_min_max_width_with_overlapping_ranges`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (max-width: 1440px) and (min-width: 900px)': '1 / 4',
      '@media (max-width: 1040px) and (min-width: 600px)': '1 / 3',
    } },
    ours: [
      '@media (min-width: 1040.01px) and (max-width: 1440px)',
      '@media (min-width: 600px) and (max-width: 1040px)',
    ],
  },
  {
    id: 'u21', seam: 'unit', origin: `${UNIT}::handles_and_media_queries`,
    props: { width: {
      default: '100%',
      '@media (min-width: 900px)': '80%',
      '@media (min-width: 500px) and (max-width: 899px) and (max-height: 300px)': '50%',
    } },
    ours: [
      '@media (min-width: 900px) and (not ((min-width: 500px) and (max-width: 899px) and (max-height: 300px)))',
      '@media (min-width: 500px) and (max-width: 899px) and (max-height: 300px)',
    ],
  },
  {
    id: 'u22', seam: 'unit', origin: `${UNIT}::combination_of_keywords_and_rules`,
    props: { width: {
      default: '100%',
      '@media screen and (min-width: 900px)': '80%',
      '@media print and (max-width: 500px)': '50%',
    } },
    ours: [
      '@media ((screen) and (min-width: 900px) and (not (print))) or ((screen) and (min-width: 900px) and (not (max-width: 500px)))',
      '@media (print) and (max-width: 500px)',
    ],
  },
  {
    id: 'u23', seam: 'unit', origin: `${UNIT}::media_queries_with_em_units`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (max-width: 90em) and (min-width: 60em)': '1 / 4',
      '@media (max-width: 70em) and (min-width: 65em)': '1 / 3',
    } },
    ours: [
      '@media ((min-width: 60em) and (max-width: 64.99em)) or ((min-width: 70.01em) and (max-width: 90em))',
      '@media (min-width: 65em) and (max-width: 70em)',
    ],
  },
  {
    id: 'u24', seam: 'unit', origin: `${UNIT}::media_queries_with_mixed_units`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (max-width: 1200px) and (min-height: 50vh)': '1 / 4',
      '@media (max-width: 800px) and (min-height: 30vh)': '1 / 3',
    } },
    ours: [
      '@media (min-width: 800.01px) and (max-width: 1200px) and (min-height: 50vh)',
      '@media (max-width: 800px) and (min-height: 30vh)',
    ],
  },
  {
    id: 'u25', seam: 'unit',
    origin: `${UNIT}::skips_range_simplification_for_media_queries_with_conflicting_units_in_same_dimension_across_queries`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (min-width: 768px) and (max-width: 1200px)': '1 / 4',
      '@media (min-width: 50em)': '1 / 3',
    } },
    ours: [
      '@media (min-width: 768px) and (max-width: 1200px) and (not (min-width: 50em))',
      '@media (min-width: 50em)',
    ],
  },
  {
    id: 'u26', seam: 'unit',
    origin: `${UNIT}::skips_range_simplification_for_media_queries_with_conflicting_units_in_same_dimension_and_query`,
    props: { gridColumn: {
      default: '1 / 2',
      '@media (min-width: 768px) and (max-width: 1200em)': '1 / 4',
      '@media (min-width: 50em)': '1 / 3',
    } },
    ours: [
      '@media (min-width: 768px) and (max-width: 1200em) and (not (min-width: 50em))',
      '@media (min-width: 50em)',
    ],
  },
  {
    id: 'u27', seam: 'unit', origin: `${UNIT}::handles_only_screen_media_queries_without_parenthesizing_the_media_type`,
    props: { color: {
      default: null,
      '@media only screen and (max-width: 600px)': 'red',
      '@media only screen and (max-width: 400px)': 'blue',
    } },
    note: 'The repository pins only that `only screen` survives unparenthesized, not the whole text.',
    ours: ['<partial: contains "only screen", never "only (screen)">'],
  },
  {
    id: 'u28', seam: 'unit', origin: `${UNIT}::single_media_query_moves_after_the_default`,
    props: { color: { '@media (max-width: 900px) and (min-width: 100px)': 'red', default: 'blue' } },
    ours: ['@media (min-width: 100px) and (max-width: 900px)'],
  },
  {
    id: 'u29', seam: 'unit', origin: `${UNIT}::fractional_rem_breakpoints_derive_the_bounds_babel_derives`,
    props: { minHeight: {
      default: '100px',
      '@media (min-width: 25rem)': '200px',
      '@media (min-width: 28.81rem)': '300px',
      '@media (min-width: 32.88rem)': '400px',
    } },
    ours: [
      '@media (min-width: 25rem) and (max-width: 28.799999999999997rem)',
      '@media (min-width: 28.81rem) and (max-width: 32.870000000000005rem)',
      '@media (min-width: 32.88rem)',
    ],
  },
  {
    id: 'u30', seam: 'unit', origin: `${UNIT}::every_bound_in_a_long_fractional_chain_matches`,
    props: { width: {
      default: '1px',
      '@media (min-width: 1.1rem)': '2px',
      '@media (min-width: 2.2rem)': '3px',
      '@media (min-width: 3.3rem)': '4px',
      '@media (min-width: 4.4rem)': '5px',
    } },
    ours: [
      '@media (min-width: 1.1rem) and (max-width: 2.1900000000000004rem)',
      '@media (min-width: 2.2rem) and (max-width: 3.29rem)',
      '@media (min-width: 3.3rem) and (max-width: 4.390000000000001rem)',
      '@media (min-width: 4.4rem)',
    ],
  },
  {
    id: 'u31', seam: 'unit', origin: `${UNIT}::a_fractional_aspect_ratio_reprints_at_the_width_it_was_written`,
    props: { width: {
      default: '1px',
      '@media (aspect-ratio: 16.5/9)': '2px',
      '@media (aspect-ratio: 3000000000/1)': '3px',
    } },
    ours: [
      '@media (aspect-ratio: 16.5 / 9) and (not (aspect-ratio: 3000000000 / 1))',
      '@media (aspect-ratio: 3000000000 / 1)',
    ],
  },
  {
    id: 'u32', seam: 'unit', origin: `${UNIT}::round_breakpoints_are_undisturbed`,
    props: { width: {
      default: '1px',
      '@media (min-width: 1024px)': '2px',
      '@media (min-width: 1440px)': '3px',
    } },
    ours: [
      '@media (min-width: 1024px) and (max-width: 1439.99px)',
      '@media (min-width: 1440px)',
    ],
  },

  // ---- the canonicalization suite ----------------------------------------
  {
    id: 'c01', seam: 'end-to-end', origin: `${CANON}::authored_media_queries_are_canonicalized`,
    props: {
      display: { default: 'none', '@media (max-height:120px) and (min-width: 720px)': 'block' },
      color: { default: 'red', '@media (width >= 1460px)': 'blue' },
    },
    note: 'The two style namespaces of the original are merged into one; neither query negates the other.',
    ours: [
      '@media (min-width: 720px) and (max-height: 120px)',
      '@media (min-width: 1460px)',
    ],
  },
  {
    id: 'c02', seam: 'end-to-end', origin: `${CANON}::style_level_media_keys_are_left_verbatim`,
    props: {
      '@media (max-height:120px) and (min-width: 720px)': { display: 'block' },
      '@media (width >= 1460px)': { color: 'blue' },
    },
    ours: [
      '@media (max-height:120px) and (min-width: 720px)',
      '@media (width >= 1460px)',
    ],
  },
  {
    id: 'c03', seam: 'end-to-end', origin: `${CANON}::media_query_order_opt_out_keeps_queries_verbatim`,
    options: { enableMediaQueryOrder: false },
    props: {
      display: { default: 'none', '@media (max-height:120px) and (min-width: 720px)': 'block' },
      color: { default: 'red', '@media (width >= 1460px)': 'blue' },
    },
    ours: [
      '@media (max-height:120px) and (min-width: 720px)',
      '@media (width >= 1460px)',
    ],
  },

  // ---- the computed-bounds suite -----------------------------------------
  {
    id: 'b01', seam: 'end-to-end', origin: `${BOUNDS}::fractional_rem_breakpoints_derive_babels_upper_bounds`,
    props: { minHeight: {
      default: '100px',
      '@media (min-width: 25rem)': '200px',
      '@media (min-width: 28.81rem)': '300px',
      '@media (min-width: 32.88rem)': '400px',
    } },
    ours: [
      '@media (min-width: 25rem) and (max-width: 28.799999999999997rem)',
      '@media (min-width: 28.81rem) and (max-width: 32.870000000000005rem)',
      '@media (min-width: 32.88rem)',
    ],
  },
  {
    id: 'b02', seam: 'end-to-end', origin: `${BOUNDS}::strict_range_queries_nudge_in_double_precision`,
    props: {
      color: { default: 'red', '@media (width > 400.5px)': 'blue' },
      backgroundColor: { default: 'red', '@media (400.5px < width < 900.25px)': 'blue' },
    },
    note: 'The two style namespaces of the original are merged into one, one property each.',
    ours: [
      '@media (min-width: 400.51px)',
      '@media (min-width: 400.51px) and (max-width: 900.24px)',
    ],
  },
  {
    id: 'b03', seam: 'end-to-end', origin: `${BOUNDS}::every_bound_in_a_long_fractional_chain_matches`,
    props: { width: {
      default: '1px',
      '@media (min-width: 1.1rem)': '2px',
      '@media (min-width: 2.2rem)': '3px',
      '@media (min-width: 3.3rem)': '4px',
      '@media (min-width: 4.4rem)': '5px',
    } },
    ours: [
      '@media (min-width: 1.1rem) and (max-width: 2.1900000000000004rem)',
      '@media (min-width: 2.2rem) and (max-width: 3.29rem)',
      '@media (min-width: 3.3rem) and (max-width: 4.390000000000001rem)',
      '@media (min-width: 4.4rem)',
    ],
  },
  {
    id: 'b04', seam: 'end-to-end',
    origin: `${BOUNDS}::breakpoints_past_the_exponential_threshold_spell_the_bound_as_javascript_does`,
    props: { minHeight: {
      default: '0px',
      '@media (min-width: 1e21px)': '10px',
      '@media (min-width: 2e21px)': '20px',
    } },
    ours: [
      '@media (min-width: 1e+21px) and (max-width: 2e+21px)',
      '@media (min-width: 2e+21px)',
    ],
  },
  {
    id: 'b05', seam: 'end-to-end', origin: `${BOUNDS}::a_fractional_aspect_ratio_reaches_the_stylesheet_intact`,
    props: { color: {
      default: 'red',
      '@media (aspect-ratio: 16.5/9)': 'blue',
      '@media (aspect-ratio: 3000000000/1)': 'green',
    } },
    ours: [
      '@media (aspect-ratio: 16.5 / 9) and (not (aspect-ratio: 3000000000 / 1))',
      '@media (aspect-ratio: 3000000000 / 1)',
    ],
  },

  // ---- the reported input, which no expectation in the repository covers yet -
  {
    id: 'r01', seam: 'reported', origin: 'https://github.com/Dwlad90/stylex-swc-plugin/issues/1268',
    note: 'No repository expectation exists for this yet; the pin column records what this compiler emits today, which is every authored query unchanged.',
    props: { color: {
      default: 'black',
      '@media (min-width: 1440px)': 'c1',
      '@media (min-width: 1200px) and (max-width: 1439px)': 'c2',
      '@media (min-width: 1024px) and (max-width: 1199px)': 'c3',
      '@media (min-width: 768px) and (max-width: 1023px)': 'c4',
      '@media (min-width: 480px) and (max-width: 767px)': 'c5',
      '@media (max-width: 479px)': 'c6',
    } },
    ours: ['<partial: recorded from this compiler, not pinned anywhere>'],
  },

  // ---- expectations carried by a generated snapshot -----------------------
  //
  // These pin emitted CSS in an `__swc_snapshots__` file rather than in a
  // literal, and that file is this compiler's own output. Transcribing it into
  // an `ours` list would only restate the third column, so `ours` is the
  // sentinel `@snapshot` and the row's verdict is whether the two compilers
  // agree -- which is the same question, asked without a transcription step in
  // between.
  {
    id: 's01', seam: 'snapshot', origin: `${STATIC}::media_queries`,
    props: { backgroundColor: {
      default: 'red',
      '@media (min-width: 1000px)': 'blue',
      '@media (min-width: 2000px)': 'purple',
    } },
    ours: ['@snapshot'],
  },
  {
    id: 's02', seam: 'snapshot', origin: `${STATIC}::media_queries_with_last_query_wins`,
    props: { backgroundColor: {
      default: 'red',
      '@media (max-width: 900px)': 'blue',
      '@media (max-width: 500px)': 'purple',
      '@media (max-width: 400px)': 'green',
    } },
    ours: ['@snapshot'],
  },
  {
    id: 's03', seam: 'snapshot', origin: `${STATIC}::media_queries_without_last_query_wins`,
    options: { enableMediaQueryOrder: false },
    props: { backgroundColor: {
      default: 'red',
      '@media (max-width: 900px)': 'blue',
      '@media (max-width: 500px)': 'purple',
      '@media (max-width: 400px)': 'green',
    } },
    ours: ['@snapshot'],
  },
  {
    id: 's04', seam: 'snapshot', origin: `${STATIC}::media_queries_without_last_query_wins_v2`,
    props: { backgroundColor: {
      default: 'red',
      '@media screen and (max-width: 900px)': 'blue',
      '@media screen and (max-width: 500px)': 'purple',
      '@media screen and (max-width: 400px)': 'green',
    } },
    ours: ['@snapshot'],
  },
  {
    id: 's05', seam: 'snapshot', origin: `${STATIC}::media_query_with_pseudo_classes`,
    props: { fontSize: {
      default: '1rem',
      '@media (min-width: 800px)': { default: '2rem', ':hover': '2.2rem' },
    } },
    ours: ['@snapshot'],
  },
  {
    id: 's06', seam: 'snapshot', origin: `${STATIC}::media_query_with_array_fallbacks`,
    props: { position: { default: 'fixed', '@media (min-width: 768px)': ['sticky', 'fixed'] } },
    ours: ['@snapshot'],
  },
  {
    id: 's07', seam: 'snapshot', origin: `${ORDER}::at_rules_differing_only_in_case_sort_by_their_code_units`,
    note: 'An at-rule ordering case: two keys differing only in case. Ticket 09 reads this row.',
    props: { color: { '@media (MIN-WIDTH: 1px)': 'red', '@media (min-width: 1px)': 'blue' } },
    ours: ['@snapshot'],
  },
  {
    id: 's08', seam: 'snapshot', origin: `${ORDER}::a_non_ascii_at_rule_sorts_by_its_code_points`,
    note: 'An at-rule ordering case mixing `@supports` with `@media`. Ticket 09 reads this row.',
    props: { color: {
      '@supports (--ü: 1)': 'red',
      '@supports (--z: 1)': 'blue',
      '@media (min-width: 1px)': 'green',
    } },
    ours: ['@snapshot'],
  },
  {
    id: 's09', seam: 'snapshot', origin: `${ORDER}::at_rules_between_pseudo_classes_do_not_split_the_run`,
    props: { color: { ':hover': { '@media (min-width: 1px)': {
      ':focus': { '@supports (color: red)': { ':active': 'red' } },
    } } } },
    ours: ['@snapshot'],
  },
];
