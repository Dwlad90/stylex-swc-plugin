use stylex_swc_plugin::ModuleTransformVisitor;
use swc_core::{ecma::transforms::testing::test, plugin::proxies::PluginCommentsProxy};

test!(
    Default::default(),
    |_| { ModuleTransformVisitor::new_test(PluginCommentsProxy) },
    should_remove_import_if_not_in_use,
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

#[test]
#[should_panic(expected = "Must be default import")]
fn throw_when_named_import() {
    swc_core::ecma::transforms::testing::test_transform(
        Default::default(),
        |_| ModuleTransformVisitor::new_test(PluginCommentsProxy),
        r#"
            import { foo } from "@stylexjs/stylex";

            foo('bar');
        "#,
        r#""#,
        false,
    )
}
