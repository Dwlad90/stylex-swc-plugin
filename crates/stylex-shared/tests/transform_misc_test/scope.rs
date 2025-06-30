use stylex_shared::{
  StyleXTransform,
  shared::structures::{
    plugin_pass::PluginPass,
    stylex_options::{StyleXOptions, StyleXOptionsParams},
  },
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::test,
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    Some(&mut StyleXOptionsParams {
      dev: Some(true),
      treeshake_compensation: Some(true),
      unstable_module_resolution: Some(StyleXOptions::get_haste_module_resolution(None)),
      runtime_injection: Some(false),
      ..StyleXOptionsParams::default()
    })
  ),
  correct_transform_variables_with_same_name_in_different_scopes,
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
        background: '#F7F5F6',
      },
    });
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
        dev: Some(true),
        ..StyleXOptionsParams::default()
      }),
    )
  },
  stylex_call_with_redaclare_import_declaration_in_dev_mode,
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
          background: '#F7F5F6',
        },
      });
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
        dev: Some(false),
        ..StyleXOptionsParams::default()
      }),
    )
  },
  stylex_call_with_redaclare_import_declaration_in_prod_mode,
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
          background: '#F7F5F6',
        },
      });
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
        dev: Some(true),
        ..StyleXOptionsParams::default()
      }),
    )
  },
  stylex_call_with_redaclare_variable_from_other_scope_in_dev_mode,
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
          background: '#F7F5F6',
        },
      });
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
        dev: Some(false),
        ..StyleXOptionsParams::default()
      }),
    )
  },
  stylex_call_with_redaclare_variable_from_other_scope_in_prod_mode,
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
          background: '#F7F5F6',
        },
      });
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
        dev: Some(false),
        ..StyleXOptionsParams::default()
      }),
    )
  },
  stylex_call_with_redaclare_function_from_other_scope_in_dev_mode,
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
          background: '#F7F5F6',
        },
      });
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
        dev: Some(false),
        ..StyleXOptionsParams::default()
      }),
    )
  },
  stylex_call_with_redaclare_function_from_other_scope_in_prod_mode,
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
          background: '#F7F5F6',
        },
      });
    "#
);
