use crate::utils::prelude::*;
use swc_core::ecma::transforms::testing::{test, test_transform};

stylex_test_panic!(
  #[ignore],
  invalid_property_animation,
  "Unknown CSS property: animation",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { animation: "anim 1s" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_background,
  "Unknown CSS property: background",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { background: "red" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_border,
  "Unknown CSS property: border",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { border: "1px solid red" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_border_block,
  "Unknown CSS property: borderBlock",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderBlock: "1px solid red" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_border_block_end,
  "Unknown CSS property: borderBlockEnd",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderBlockEnd: "1px solid red" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_border_block_start,
  "Unknown CSS property: borderBlockStart",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderBlockStart: "1px solid red" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_border_bottom,
  "Unknown CSS property: borderBottom",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderBottom: "1px solid red" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_border_image,
  "Unknown CSS property: borderImage",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderImage: "url(./img.jpg) 30 space" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_border_inline,
  "Unknown CSS property: borderInline",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderInline: "1px solid red" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_border_inline_end,
  "Unknown CSS property: borderInlineEnd",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderInlineEnd: "1px solid red" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_border_inline_start,
  "Unknown CSS property: borderInlineStart",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderInlineStart: "1px solid red" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_border_left,
  "Unknown CSS property: borderLeft",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderLeft: "1px solid red" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_border_right,
  "Unknown CSS property: borderRight",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderRight: "1px solid red" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_border_top,
  "Unknown CSS property: borderTop",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { borderTop: "1px solid red" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_flex_flow,
  "Unknown CSS property: flexFlow",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { flexFlow: "row wrap" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_font,
  "Unknown CSS property: font",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { font: "16px/16 Arial" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_list_style,
  "Unknown CSS property: listStyle",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { listStyle: "square inside" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_text_decoration,
  "Unknown CSS property: textDecoration",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { textDecoration: "1px solid underline" } });
  "#
);

stylex_test_panic!(
  #[ignore],
  invalid_property_transition,
  "Unknown CSS property: transition",
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({ x: { transition: "opacity 1s" } });
  "#
);
