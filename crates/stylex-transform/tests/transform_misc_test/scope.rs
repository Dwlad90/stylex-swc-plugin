use crate::utils::prelude::*;

fn stylex_transform(
  comments: TestComments,
  customize: impl FnOnce(TestBuilder) -> TestBuilder,
) -> impl Pass {
  build_test_transform(comments, |b| customize(b.with_dev(false)))
}

stylex_test!(
  correct_transform_variables_with_same_name_in_different_scopes,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_dev(true)
      .with_enable_debug_class_names(true)
      .with_treeshake_compensation(true)
      .with_unstable_module_resolution(ModuleResolution::haste(None))
      .with_runtime_injection_option(RuntimeInjection::Boolean(false))
      .with_runtime_injection()
  }),
  r#"
    'use client';

    import * as stylex from '@stylexjs/stylex';
    import { display } from '@styles/utils';
    import foo from 'bar';
    import { foo as baz } from 'bar';

    const fn = () => ({ arg: () => { } })
    function func() { }

    {
      const display = null;
    }

    {
      {
        const display = null;
      }
    }

    {
      const { display } = { display: null };
    }

    {
      const [display] = [null];
    }

    const ArrowFunction = () => {
      const display = null;

      return display
    }

    export const Component = () => {
      const display = null;

      return display
    }

    const array = [1, 2, 3, 4];

    export const ComponentWithCallings = () => {
      array.forEach((item) => {
        if (fn(item).arg('str', 1, null, undefined, NaN, { foo: 'bar' }, [1, 2, 3], func())) {
          fn(item)
        }
      });

      return <div>{array.length > 0 ? <div {...stylex.props(s.div, display.flex)} >{array.map(_ => null)}</div> : null}</div>;
    };

    const s = stylex.create({
      div: {
        backgroundColor: '#F7F5F6',
      },
    });
  "#
);

stylex_test!(
  stylex_call_with_redaclare_import_declaration_in_dev_mode,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_dev(true).with_enable_debug_class_names(true)
  }),
  r#"
      'use client';

      import * as stylex from '@stylexjs/stylex';
      import { display } from '@styles/utils';
      import foo from 'bar';
      import { foo as baz } from 'bar';

      const fn = () => ({ arg: () => { } })
      function func() { }

      export const Component = () => {
        const display = null;

        return display
      }

      const array = [1, 2, 3, 4];

      export const ComponentWithCallings = () => {
        array.forEach((item) => {
          if (fn(item).arg('str', 1, null, undefined, NaN, { foo: 'bar' }, [1, 2, 3], func())) {
            fn(item)
          }
        });

        return <div>{array.length > 0 ? <div {...stylex.props(s.div, display.flex)} >{array.map(_ => null)}</div> : null}</div>;
      };

      const s = stylex.create({
        div: {
          backgroundColor: '#F7F5F6',
        },
      });
    "#
);

stylex_test!(
  stylex_call_with_redaclare_import_declaration_in_prod_mode,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
      'use client';

      import * as stylex from '@stylexjs/stylex';
      import { display } from '@styles/utils';
      import foo from 'bar';
      import { foo as baz } from 'bar';

      const fn = () => ({ arg: () => { } })
      function func() { }

      export const Component = () => {
        const display = null;

        return display
      }

      const array = [1, 2, 3, 4];

      export const ComponentWithCallings = () => {
        array.forEach((item) => {
          if (fn(item).arg('str', 1, null, undefined, NaN, { foo: 'bar' }, [1, 2, 3], func())) {
            fn(item)
          }
        });

        return <div>{array.length > 0 ? <div {...stylex.props(s.div, display.flex)} >{array.map(_ => null)}</div> : null}</div>;
      };

      const s = stylex.create({
        div: {
          backgroundColor: '#F7F5F6',
        },
      });
    "#
);

stylex_test!(
  stylex_call_with_redaclare_variable_from_other_scope_in_dev_mode,
  |tr| stylex_transform(tr.comments.clone(), |b| {
    b.with_dev(true).with_enable_debug_class_names(true)
  }),
  r#"
      'use client';

      import * as stylex from '@stylexjs/stylex';
      import { display } from '@styles/utils';
      import foo from 'bar';
      import { foo as baz } from 'bar';

      const fn = () => ({ arg: () => { } })
      function func() { }

      const declare = null;

      export const Component = () => {
        const declare = null;

        return declare
      }

      const array = [1, 2, 3, 4];

      export const ComponentWithCallings = () => {
        array.forEach((item) => {
          if (fn(item).arg('str', 1, null, undefined, NaN, { foo: 'bar' }, [1, 2, 3], func())) {
            fn(item)
          }
        });

        return <div>{array.length > 0 ? <div {...stylex.props(s.div, display.flex)} >{array.map(_ => null)}</div> : null}</div>;
      };

      const s = stylex.create({
        div: {
          backgroundColor: '#F7F5F6',
        },
      });
    "#
);

stylex_test!(
  stylex_call_with_redaclare_variable_from_other_scope_in_prod_mode,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"

      import * as stylex from '@stylexjs/stylex';
      import { display } from '@styles/utils';
      import foo from 'bar';
      import { foo as baz } from 'bar';

      const fn = () => ({ arg: () => { } })
      function func() { }

      const declare = null;

      export const Component = () => {
        const declare = null;

        return declare
      }

      const array = [1, 2, 3, 4];

      export const ComponentWithCallings = () => {
        array.forEach((item) => {
          if (fn(item).arg('str', 1, null, undefined, NaN, { foo: 'bar' }, [1, 2, 3], func())) {
            fn(item)
          }
        });

        return <div>{array.length > 0 ? <div {...stylex.props(s.div, display.flex)} >{array.map(_ => null)}</div> : null}</div>;
      };

      const s = stylex.create({
        div: {
          backgroundColor: '#F7F5F6',
        },
      });
    "#
);

stylex_test!(
  stylex_call_with_redaclare_function_from_other_scope_in_dev_mode,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"
      'use client';

      import * as stylex from '@stylexjs/stylex';
      import { display } from '@styles/utils';
      import foo from 'bar';
      import { foo as baz } from 'bar';

      const fn = () => ({ arg: () => { } })
      function func() { }

      function declare () {};

      export const Component = () => {
        const declare = null;

        return declare
      }

      const array = [1, 2, 3, 4];

      export const ComponentWithCallings = () => {
        array.forEach((item) => {
          if (fn(item).arg('str', 1, null, undefined, NaN, { foo: 'bar' }, [1, 2, 3], func())) {
            fn(item)
          }
        });

        return <div>{array.length > 0 ? <div {...stylex.props(s.div, display.flex)} >{array.map(_ => null)}</div> : null}</div>;
      };

      const s = stylex.create({
        div: {
          backgroundColor: '#F7F5F6',
        },
      });
    "#
);

stylex_test!(
  stylex_call_with_redaclare_function_from_other_scope_in_prod_mode,
  |tr| stylex_transform(tr.comments.clone(), |b| b),
  r#"

      import * as stylex from '@stylexjs/stylex';
      import { display } from '@styles/utils';
      import foo from 'bar';
      import { foo as baz } from 'bar';

      const fn = () => ({ arg: () => { } })
      function func() { }

      function declare () {};

      export const Component = () => {
        const declare = null;

        return declare
      }

      const array = [1, 2, 3, 4];

      export const ComponentWithCallings = () => {
        array.forEach((item) => {
          if (fn(item).arg('str', 1, null, undefined, NaN, { foo: 'bar' }, [1, 2, 3], func())) {
            fn(item)
          }
        });

        return <div>{array.length > 0 ? <div {...stylex.props(s.div, display.flex)} >{array.map(_ => null)}</div> : null}</div>;
      };

      const s = stylex.create({
        div: {
          backgroundColor: '#F7F5F6',
        },
      });
    "#
);
