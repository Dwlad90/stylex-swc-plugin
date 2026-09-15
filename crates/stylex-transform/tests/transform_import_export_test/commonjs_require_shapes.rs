//! The `require` shapes the import reader is handed but takes nothing from.
//!
//! A `require` call reaches the reader whatever it is written against, so every
//! shape that names no module, and every binding form that names no local, has
//! to be answered rather than assumed away. Each module here also imports
//! StyleX the ordinary way and compiles one style, so a snapshot shows the
//! shape under test was passed over and not that the compiler stopped.

use crate::utils::prelude::*;
use insta::assert_snapshot;
use stylex_structures::named_import_source::NamedImportSource;
use swc_core::common::FileName;

use crate::utils::transform::stringify_js;

/// The module `input` compiles to, with `import_sources` configured.
fn transform_with_sources(input: &str, import_sources: Option<Vec<ImportSources>>) -> String {
  stringify_js(input, ts_syntax(), |tr| {
    let mut builder = StyleXTransform::test(tr.comments.clone())
      .with_filename(FileName::Real("/stylex/packages/TestFile.js".into()))
      .with_unstable_module_resolution(ModuleResolution::common_js(Some(
        "/stylex/packages/".to_string(),
      )));

    if let Some(sources) = import_sources {
      builder = builder.with_import_sources(sources);
    }

    builder.into_pass()
  })
}

fn transform(input: &str) -> String {
  transform_with_sources(input, None)
}

/// A call that is not a `require` of a module named at compile time: no
/// argument at all, a spread argument, a name only the run time knows, and a
/// number. None of them names a module, so none registers an import.
#[test]
fn a_require_call_that_names_no_module() {
  let output = transform(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const empty = require();
      export const spread = require(...sources);
      export const named = require(sourceName);
      export const numbered = require(42);
      export const styles = stylex.create({ base: { color: 'red' } });
    "#,
  );

  assert_snapshot!(output);
}

/// A `require` of the StyleX source bound to a shape that names no local the
/// reader can register: an array pattern, a rest property, a computed key, a
/// nested array value, and a value with a default.
#[test]
fn a_stylex_require_bound_to_a_shape_that_names_no_local() {
  let output = transform(
    r#"
      import * as stylex from '@stylexjs/stylex';
      export const [first] = require('@stylexjs/stylex');
      const { ...everything } = require('@stylexjs/stylex');
      const { [dynamicKey]: computed } = require('@stylexjs/stylex');
      const { create: [nested] } = require('@stylexjs/stylex');
      const { keyframes: withDefault = fallback } = require('@stylexjs/stylex');
      export const styles = stylex.create({ base: { color: 'red' } });
    "#,
  );

  assert_snapshot!(output);
}

/// A destructured `require` of a name the StyleX source does not export. The
/// source is an import path either way, and the local binds no API.
#[test]
fn a_destructured_require_of_a_name_that_is_no_api() {
  let output = transform(
    r#"
      import * as stylex from '@stylexjs/stylex';
      const { notAnApi } = require('@stylexjs/stylex');
      notAnApi({ base: { color: 'red' } });
      export const styles = stylex.create({ base: { color: 'red' } });
    "#,
  );

  assert_snapshot!(output);
}

/// With an `import_sources` alias configured, the alias is what counts as the
/// StyleX handle. A whole-module binding is not it, a destructured name that is
/// not the alias is not it, and the alias itself is.
#[test]
fn a_require_read_under_a_configured_import_alias() {
  let output = transform_with_sources(
    r#"
      export const whole = require('react-strict-dom');
      export const { html } = require('react-strict-dom');
      const { css } = require('react-strict-dom');
      export const styles = css.create({ base: { color: 'red' } });
    "#,
    Some(vec![ImportSources::Named(NamedImportSource {
      r#as: "css".to_string(),
      from: "react-strict-dom".to_string(),
    })]),
  );

  assert_snapshot!(output);
}

/// The atoms source is read by the same shapes, and answers for the two its
/// reader does not recognise: an array pattern and a rest property.
#[test]
fn an_atoms_require_bound_to_a_shape_that_names_no_local() {
  let output = transform(
    r#"
      import stylex from '@stylexjs/stylex';
      export const [firstAtom] = require('@stylexjs/atoms');
      const { ...everyAtom } = require('@stylexjs/atoms');
      const { color } = require('@stylexjs/atoms');
      stylex.props(color.blue);
    "#,
  );

  assert_snapshot!(output);
}
