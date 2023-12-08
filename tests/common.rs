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
    should_remove_default_import,
    r#"
      import Preact from "preact";
      import React from "react";
      import foo from "@stylexjs/stylex";
    "#,
    r#"
      import Preact from "preact";
      import React from "react";
    "#
);

test!(
  Syntax::Typescript(TsConfig {
      tsx: true,
      ..Default::default()
  }),
  |tr| ModuleTransformVisitor::new_test(tr.comments.clone()),
  should_remove_star_import,
  r#"
    import Preact from "preact";
    import React from "react";
    import * as foo from "@stylexjs/stylex";
  "#,
  r#"
    import Preact from "preact";
    import React from "react";
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
