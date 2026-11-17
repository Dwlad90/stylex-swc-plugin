//! What the transform's test files share: a parse that matches the compiler's,
//! and a transform built over it.
//!
//! `RUST.md` permits a test prelude, because test code is not part of the crate
//! graph. The suites that read it are registered from the files they cover, so
//! they reach it by path rather than by being neighbours.

use std::rc::Rc;
use std::sync::Arc;

use swc_core::{
  common::{FileName, Mark, SourceMap, comments::SingleThreadedComments, input::StringInput},
  ecma::{
    ast::{Module, Pass, Program},
    parser::{Parser, Syntax, TsSyntax, lexer::Lexer},
    transforms::{base::resolver, typescript::strip},
  },
};

use crate::{StyleXTransform, StyleXTransformBuilder};

/// The comments a test transform carries: none, which is what a module with no
/// comment in it hands the compiler.
pub(crate) type TestComments = Rc<SingleThreadedComments>;

/// Parses `code` as TypeScript with JSX and runs SWC's resolver over it, the
/// way the compiler does before the StyleX pass, so identifiers carry the
/// syntax contexts the transform reads.
///
/// TypeScript with JSX reads every module a case here writes, JavaScript
/// included, so one parse serves them all.
pub(crate) fn resolved_module(code: &str) -> Module {
  resolved_module_in(
    code,
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    false,
  )
}

/// The same over TypeScript without JSX, which is the only syntax that reads
/// an angle-bracket cast: where JSX is on, `<any>value` opens an element.
pub(crate) fn resolved_ts_module(code: &str) -> Module {
  resolved_module_in(code, Syntax::Typescript(Default::default()), false)
}

/// The same as [`resolved_ts_module`], then with the types stripped, as the
/// compiler does before the StyleX pass. An `enum` or a `namespace` is a
/// function call after this.
pub(crate) fn stripped_ts_module(code: &str) -> Module {
  resolved_module_in(code, Syntax::Typescript(Default::default()), true)
}

fn resolved_module_in(code: &str, syntax: Syntax, strip_types: bool) -> Module {
  let source_map = SourceMap::default();

  // The name says which syntax read the module, so a parse failure names the
  // parse that failed.
  let file_name = match syntax {
    Syntax::Typescript(TsSyntax { tsx: true, .. }) => "input.tsx",
    _ => "input.ts",
  };

  let source_file = source_map.new_source_file(
    Arc::new(FileName::Custom(file_name.to_string())),
    code.to_string(),
  );

  let lexer = Lexer::new(
    syntax,
    Default::default(),
    StringInput::from(&*source_file),
    None,
  );

  let module = match Parser::new_from(lexer).parse_module() {
    Ok(module) => module,
    Err(error) => panic!("failed to parse fixture: {:?}", error),
  };

  let mut program = Program::Module(module);
  let unresolved_mark = Mark::new();
  let top_level_mark = Mark::new();
  resolver(unresolved_mark, top_level_mark, true).process(&mut program);

  if strip_types {
    strip(unresolved_mark, top_level_mark).process(&mut program);
  }

  match program {
    Program::Module(module) => module,
    Program::Script(_) => unreachable!("a parsed module never becomes a script"),
  }
}

/// A transform built for a test, with `customize` applied to its builder.
///
/// One builder for every case here, so a suite says what it changed rather than
/// repeating what every case sets.
pub(crate) fn test_transform(
  customize: impl FnOnce(StyleXTransformBuilder<TestComments>) -> StyleXTransformBuilder<TestComments>,
) -> StyleXTransform<TestComments> {
  customize(StyleXTransform::test(comments())).build()
}

pub(crate) fn comments() -> TestComments {
  Rc::new(SingleThreadedComments::default())
}
