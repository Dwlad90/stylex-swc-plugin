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
    transforms::base::resolver,
  },
};

use crate::{StyleXTransform, StyleXTransformBuilder};

/// The comments a test transform carries: none, which is what a module with no
/// comment in it hands the compiler.
pub(crate) type TestComments = Rc<SingleThreadedComments>;

/// Parses `code` as TypeScript and runs SWC's resolver over it, the way the
/// compiler does before the StyleX pass, so identifiers carry the syntax
/// contexts the transform reads.
pub(crate) fn resolved_module(code: &str) -> Module {
  let source_map = SourceMap::default();
  let source_file = source_map.new_source_file(
    Arc::new(FileName::Custom("input.tsx".to_string())),
    code.to_string(),
  );

  let lexer = Lexer::new(
    Syntax::Typescript(TsSyntax {
      tsx: true,
      ..Default::default()
    }),
    Default::default(),
    StringInput::from(&*source_file),
    None,
  );

  let module = match Parser::new_from(lexer).parse_module() {
    Ok(module) => module,
    Err(error) => panic!("failed to parse fixture: {:?}", error),
  };

  let mut program = Program::Module(module);
  resolver(Mark::new(), Mark::new(), true).process(&mut program);

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
