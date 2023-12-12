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
    |tr| ModuleTransformVisitor::new_test(tr.comments.clone()),
    ignores_valid_imports,
    r#"
        import stylex from '@stylexjs/stylex';
        import {foo, bar} from 'other';
    "#,
    r#"
        import {foo, bar} from 'other';
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
        |tr| ModuleTransformVisitor::new_test(tr.comments.clone()),
        r#"
            import { foo } from "@stylexjs/stylex";

            foo('bar');
        "#,
        r#""#,
        false,
    )
}
