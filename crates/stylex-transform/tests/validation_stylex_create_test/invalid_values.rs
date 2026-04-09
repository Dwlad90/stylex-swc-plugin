use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::{test, test_transform};

stylex_test_panic!(
  #[ignore],
  invalid_value_display_important,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { display: "block !important" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_background_position_top_left,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { backgroundPosition: "top left" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_border_color_red_blue,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { borderColor: "red blue" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_border_radius_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { borderRadius: "1px 2px" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_border_style_solid_dashed,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { borderStyle: "solid dashed" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_border_width_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { borderWidth: "1px 2px" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_inset_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { inset: "1px 2px" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_inset_block_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { insetBlock: "1px 2px" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_inset_inline_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { insetInline: "1px 2px" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_flex_1_1_0,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { flex: "1 1 0" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_grid_1_1_0,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { grid: "1 1 0" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_margin_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { margin: "1px 2px" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_margin_block_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { marginBlock: "1px 2px" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_margin_inline_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { marginInline: "1px 2px" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_outline_1px_solid_red,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { outline: "1px solid red" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_overflow_hidden_visible,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { overflow: "hidden visible" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_padding_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { padding: "1px 2px" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_padding_block_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { paddingBlock: "1px 2px" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_padding_inline_1px_2px,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { paddingInline: "1px 2px" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_transition_property_all,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { transitionProperty: "all" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_transition_property_bottom,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { transitionProperty: "bottom" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_transition_property_end,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { transitionProperty: "end" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_transition_property_height,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { transitionProperty: "height" } });
          "#
);

stylex_test_panic!(
  #[ignore],
  invalid_value_transition_property_width,
  "A style value can only contain an array, string or number.",
  r#"
            import * as stylex from '@stylexjs/stylex';
            const styles = stylex.create({ x: { transitionProperty: "width" } });
          "#
);
