use std::path::PathBuf;

use stylex_swc_plugin::{ModuleTransformVisitor, StylexConfigParams};
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
        // &|tr| {
        &|_| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            chain!(
                resolver(unresolved_mark, top_level_mark, false),
                ModuleTransformVisitor::new_test_classname(
                    PluginCommentsProxy,
                    Option::Some(StylexConfigParams {
                        use_rem_for_font_size: Option::None,
                        runtime_injection: Option::Some(false),
                        class_name_prefix: Option::Some("x".to_string()),
                        defined_stylex_css_variables: Option::None,
                        import_sources: Option::None,
                    })
                ) // ModuleTransformVisitor::new_test(tr.comments.clone())
            )
        },
        &input,
        &output,
        Default::default(),
    );
}
