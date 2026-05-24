use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(comments: TestComments) -> impl Pass {
  build_test_transform(comments, |b| {
    b.with_filename(FileName::Real("/stylex/packages/tokens.stylex.js".into()))
      .with_runtime_injection()
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(
        "/stylex/packages/".to_string(),
      )))
  })
}

stylex_test!(
  transforms_nested_consts_with_named_alias_import,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import { unstable_defineConstsNested as defineConstsNested } from '@stylexjs/stylex';
    export const constants = defineConstsNested({
      spacing: {
        sm: 4,
        md: '8px',
      },
      color: {
        states: {
          default: '#00f',
          hovered: '#009',
        },
      },
    });
  "#
);

stylex_test!(
  transforms_nested_consts_from_commonjs_destructured_alias_require,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    const { unstable_defineConstsNested: defineConstsNested } = require('@stylexjs/stylex');
    export const constants = defineConstsNested({
      spacing: {
        sm: 4,
        md: '8px',
      },
    });
  "#
);

stylex_test!(
  basic_nested_consts_with_original_values_preserved,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const tokens = stylex.unstable_defineConstsNested({
      spacing: {
        sm: '4px',
        md: '8px',
        lg: '16px',
      },
    });
  "#
);

stylex_test!(
  deeply_nested_constants_3_levels,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const tokens = stylex.unstable_defineConstsNested({
      colors: {
        slate: {
          100: '#f1f5f9',
          800: '#1e293b',
        },
        brand: {
          primary: '#3b82f6',
        },
      },
    });
  "#
);

stylex_test!(
  j_malt_pr_1303_use_case_three_tiered_tokens_with_state_namespaces,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const tokens = stylex.unstable_defineConstsNested({
      button: {
        primary: {
          background: {
            default: '#00FF00',
            hovered: '#0000FF',
          },
          borderRadius: {
            default: '8px',
          },
        },
        secondary: {
          background: {
            default: '#CCCCCC',
          },
        },
      },
      input: {
        fill: {
          default: '#FFFFFF',
        },
        border: {
          default: '#000000',
        },
      },
    });
  "#
);

stylex_test!(
  handles_number_values,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const tokens = stylex.unstable_defineConstsNested({
      breakpoints: {
        mobile: 480,
        tablet: 768,
      },
    });
  "#
);

stylex_test!(
  works_with_named_import,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import { unstable_defineConstsNested } from '@stylexjs/stylex';
    export const tokens = unstable_defineConstsNested({
      radii: {
        sm: '0.25rem',
        xl: '1rem',
      },
    });
  "#
);

stylex_test!(
  mixed_string_and_number_values,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const tokens = stylex.unstable_defineConstsNested({
      theme: {
        spacing: 8,
        unit: 'px',
      },
    });
  "#
);

stylex_test!(
  system_font_family_list,
  |tr| stylex_transform(tr.comments.clone()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    export const tokens = stylex.unstable_defineConstsNested({
      font: {
        sans: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif',
      },
    });
  "#
);
