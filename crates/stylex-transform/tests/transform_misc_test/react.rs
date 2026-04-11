use crate::utils::prelude::*;
use swc_core::common::FileName;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| {
    customize(
      b.with_enable_inlined_conditional_merge(true)
        .with_style_resolution(StyleResolution::ApplicationOrder),
    )
  })
}

stylex_test!(
  stylex_call_using_styles_inside_use_memo,
  r#"
    import stylex from 'stylex';
    import { useMemo } from 'react';

    const styles = stylex.create({
      selected: {
        color: 'red',
      },
    });

    export default function MyComponent() {
      const isSelected = true;

      const innerComponent = useMemo(() => {
          return <Component {...stylex.props(isSelected && styles.selected)} />
        }, [isSelected]);

        return innerComponent;
      }
  "#
);

stylex_test!(
  stylex_call_using_styles_inside_use_memo_skip_conditional,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_enable_inlined_conditional_merge(false)
      .with_runtime_injection()
  }),
  r#"
    import stylex from 'stylex';
    import { useMemo } from 'react';

    const styles = stylex.create({
      selected: {
        color: 'red',
      },
    });

    export default function MyComponent() {
      const isSelected = true;

      const innerComponent = useMemo(() => {
          return <Component {...stylex.props(isSelected && styles.selected)} />
        }, [isSelected]);

        return innerComponent;
      }
  "#
);

stylex_test!(
  transform_style_extend_prop_with_stylex_class,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as sx from '@stylexjs/stylex';
    import * as React from 'react';

    const c = sx.create({
      descriptionTextLink: {
        color: "red",
      },
    });

    export default function CommentField() {
      const isBold = false;

      sx.props(isBold && c.descriptionTextLink, isBold && c.descriptionTextLink,isBold && c.descriptionTextLink,isBold && c.descriptionTextLink);

      return (
        <div styleExtend={[c.descriptionTextLink]} />
      );
    }
  "#
);

stylex_test!(
  transform_style_extend_with_dynamic_stylex_class,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as sx from '@stylexjs/stylex';
    import * as React from 'react';

    const c = sx.create({
      base: {
        display: 'grid',
      },
      regularGrid: {
        display: 'grid',
      },
      irregularGrid: {
        display: 'grid',
      },
    });

    export default function CommentField({ type }) {
      let gridType = 'regular';

      if(type === 'irregular') {
        gridType = 'irregular';
      }

      const grid = `${gridType}Grid`;

      return (
        <div styleExtend={[c.base, c[grid]]} />
      );
    }
  "#
);

stylex_test!(
  transform_style_extend_with_optional_chaining,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as sx from '@stylexjs/stylex';
    import * as React from 'react';

    const c = sx.create({
      base: {
        display: 'grid',
      },
    });

    export default function CommentField({ type }) {
      const result = useHook();
      const nullable = null;
      const undef = undefined;

      return (
        <div {...sx.props(
            nullable?.test && c.base,
            undef?.test && c.base,
            (()=>{
                const implementation = {
                  foo: ()=>null,
                  bar: ()=>c.base
                };
                return implementation[result?.value !== 'test' ? "foo" : result?.test] || implementation.foo;
              })()())} />
        );
      }
  "#
);

stylex_test!(
  transform_style_extend_with_promise,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import * as sx from '@stylexjs/stylex';
    import * as React from 'react';

    const c = sx.create({
      base: {
        display: 'grid',
      },
    });

    export default async function CommentField({ type }) {
      const resultPromise = promise();

      const result = await resultPromise;

      return (
        <div {...sx.props(result && c.base)} />
      );
    }
  "#
);

stylex_test!(
  transform_style_extend_with_theme_record,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
    import { BUTTON_PRIMARY, BUTTON_SECONDARY } from 'styles/themes/button.stylex';
    import * as stylex from '@stylexjs/stylex';

    const buttonTheme = {
      primary: BUTTON_PRIMARY,
      secondary: BUTTON_SECONDARY
    };

    export function Button_Record_From_Import ()  {

      return <button
      {...stylex.props(
          buttonTheme[state.theme],
      )}
      >
      Click Me!
    </button>
    };
  "#
);

stylex_test!(
  transform_css_variable_with_zero_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';

    const styles = stylex.create({
      container: {
        "--header-height": '0px',
        minHeight: 'calc(100dvh - var(--header-height, 0px))',
      },
    });

    export default function Example() {
      return <div className={stylex(styles.container)}>Content</div>;
    };
  "#
);

stylex_test!(
  transform_math_round_with_dynamic_value,
  |tr| stylex_transform(tr.comments.clone(), |b| b.with_runtime_injection()),
  r#"
    import * as stylex from '@stylexjs/stylex';
    const styles = stylex.create({
      round: (size) => ({
        fontSize: `${Math.round(size * (2 / 3))}px`,
      }),
      min: (size) => ({
        fontSize: `${Math.min(size * (2 / 3), 12)}px`,
      }),
      abs: (size) => ({
        fontSize: Math.abs(size * (2 / 3)) + "px",
      }),
      pow: (size) => ({
        fontSize: Math.pow(size * (2 / 3), 2) + "px",
      }),
    });

    export default function Component({round, min, abs, pow}) {
      return <div {...stylex.props(round && styles.round(12), min && styles.min(12), abs && styles.abs(12), pow && styles.pow(12))} />;
    }
  "#
);

stylex_test!(
  constant_names_with_template_literal,
  |tr| {
    let cwd_path = std::env::current_dir().unwrap();

    let fixture_path = cwd_path.join("tests/fixture");
    let filename = fixture_path.join("consts/constants.stylex");

    build_test_transform(tr.comments.clone(), move |b| {
      b.with_filename(FileName::Real(filename))
        .with_runtime_injection()
        .with_unstable_module_resolution(ModuleResolution::common_js(Some(
          fixture_path.to_string_lossy().to_string(),
        )))
    })
  },
  r#"
    import * as stylex from "@stylexjs/stylex";
    import { C } from "./constants.stylex.js";

    const styles = stylex.create({
      box: { height: `${C.INPUT_HEIGHT}px` },
    });
  "#
);
