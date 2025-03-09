use stylex_shared::{
  StyleXTransform,
  shared::structures::{plugin_pass::PluginPass, stylex_options::StyleXOptionsParams},
};
use swc_core::ecma::{
  parser::{Syntax, TsSyntax},
  transforms::testing::{test, test_transform},
};

test!(
  Syntax::Typescript(TsSyntax {
    tsx: true,
    ..Default::default()
  }),
  |tr| StyleXTransform::new_test_force_runtime_injection_with_pass(
    tr.comments.clone(),
    PluginPass::default(),
    None
  ),
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

#[test]
#[should_panic(expected = "Variable 'display' conflicts with import name. Must be renamed.")]
fn stylex_call_with_redaclare_import_declaration_in_dev_mode() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
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
    "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Unimplemented case for object member access")]
fn stylex_call_with_redaclare_import_declaration_in_prod_mode() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
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
    "#,
    r#""#,
  )
}

#[test]
#[should_panic(expected = "Variable 'declare' already exists and must be renamed.")]
fn stylex_call_with_redaclare_variable_from_other_scope_in_dev_mode() {
  test_transform(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Option::None,
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
    "#,
    r#""#,
  )
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
