use stylex_swc_plugin::ModuleTransformVisitor;
use swc_core::ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::test,
};

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_classname(tr.comments.clone(), Option::None),
    ignores_valid_imports,
    r#"
        import stylex from '@stylexjs/stylex';
        import {foo, bar} from 'other';
    "#,
    r#"
        import stylex from '@stylexjs/stylex';
        import {foo, bar} from 'other';
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_classname(tr.comments.clone(), Option::None),
    ignores_valid_requires,
    r#"
        const stylex = require('@stylexjs/stylex');
        const {foo, bar} = require('other');
    "#,
    r#"
        const stylex = require('@stylexjs/stylex');
        const {foo, bar} = require('other');
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    named_declaration_export,
    r#"
        import stylex from '@stylexjs/stylex';
        export const styles = stylex.create({
        foo: {
            color: 'red'
        },
        bar: {
            color: 'blue'
        },
        });
    "#,
    r#"
        import stylex from '@stylexjs/stylex';
        stylex.inject(".x1e2nbdu{color:red}", 3000);
        stylex.inject(".xju2f9n{color:blue}", 3000);
        export const styles = {
        foo: {
            color: "x1e2nbdu",
            $$css: true
        },
        bar: {
            color: "xju2f9n",
            $$css: true
        }
        };
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    does_nothing_when_stylex_not_imported,
    r#"
        export const styles = stylex.create({
            foo: {
                color: 'red'
            },
        });
    "#,
    r#"
        export const styles = stylex.create({
            foo: {
                color: 'red'
            },
        });
    "#
);

test!(
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    }),
    |tr| ModuleTransformVisitor::new_test_styles(tr.comments.clone(), Option::None),
    named_property_export,
    r#"
        import stylex from '@stylexjs/stylex';
        const styles = stylex.create({
            foo: {
                color: 'red'
            },
        });
        export {styles}
    "#,
    r#"
        import _inject from "@stylexjs/stylex/lib/stylex-inject";
        import stylex from '@stylexjs/stylex';
        _inject(".x1e2nbdu{color:red}", 3000);
        const styles = {
        foo: {
            color: "x1e2nbdu",
            $$css: true
        }
        };
        export { styles };
    "#
);

#[test]
#[should_panic(expected = "Must be default import")]
fn throw_when_named_import() {
    swc_core::ecma::transforms::testing::test_transform(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        |tr| ModuleTransformVisitor::new_test_classname(tr.comments.clone(), Option::None),
        r#"
            import { foo } from "@stylexjs/stylex";

            foo('bar');
        "#,
        r#""#,
        false,
    )
}
