use std::path::PathBuf;

use stylex_swc_plugin::ModuleTransformVisitor;
use swc_core::{
    common::{chain, Mark},
    ecma::{
        parser::{Syntax, TsConfig},
        transforms::{base::resolver, testing::test_fixture},
    },
    plugin::proxies::PluginCommentsProxy,
};

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        &|tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            chain!(
                resolver(unresolved_mark, top_level_mark, false),
                ModuleTransformVisitor::new_test(PluginCommentsProxy) // ModuleTransformVisitor::new_test(tr.comments.clone())
            )
        },
        &input,
        &output,
        Default::default(),
    );
}
