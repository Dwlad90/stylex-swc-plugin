//! What a case about stack or about how deep a text nests is written with.
//!
//! Three things, and none of them is about evaluation: a thread of a stated
//! size, an expression parsed out of source, and the nested literal both are
//! usually pointed at. They sit here rather than beside the cases because two
//! suites need them, and because a case whose subject is a number of bytes must
//! not also be the place that says how a bracket is spelled.
//!
//! The evaluation suites read the same three through
//! [`source_evaluation`](crate::evaluate::source_evaluation), which re-exports
//! them beside its own assertions rather than restating them.

use swc_core::{
  common::{FileName, SourceFile, SourceMap, sync::Lrc},
  ecma::{
    ast::Expr,
    parser::{EsSyntax, Parser, StringInput, Syntax, lexer::Lexer},
  },
};

/// A thread small enough that a case has to be given room it did not start with.
///
/// Under a megabyte the runtime's own guard-page arithmetic starts to matter, so
/// this is the smallest honest floor rather than the smallest number that fits.
pub(crate) const SMALL_THREAD: usize = 1024 * 1024;

/// A thread large enough for the stages either side of the fold -- SWC's parse of
/// the source and the drop of the tree it answered -- to run on input nested as
/// deeply as the compiler will carry.
///
/// For a case whose subject is what the fold does with such input rather than
/// what those stages cost: both recurse on the bare thread stack and neither is
/// the fold's, so a case measuring the fold has to be given room for them.
pub(crate) const LARGE_THREAD: usize = 256 * 1024 * 1024;

/// An array literal nested `levels` deep around a string.
///
/// The shape every case about how deep the printer and the parser go is written
/// in, so a case can say which depth it is about without also saying how a
/// bracket is spelled.
pub(crate) fn nested_literal(levels: usize) -> String {
  "[".repeat(levels) + "'x'" + &"]".repeat(levels)
}

/// Runs `case` on a thread of `stack` bytes and hands back what it answered.
///
/// For a case whose subject is how much stack something needs. A test thread's
/// own size is not something a case can state, and the failure it is measuring
/// is an abort rather than an assertion, so the size has to be written down
/// beside the case that depends on it.
///
/// A panic inside `case` is resumed here rather than swallowed, so an assertion
/// that failed on the thread reads as a failure of the test that started it.
pub(crate) fn on_a_thread_of<R: Send + 'static>(
  stack: usize,
  case: impl FnOnce() -> R + Send + 'static,
) -> R {
  let started = std::thread::Builder::new().stack_size(stack).spawn(case);

  match started {
    Ok(thread) => match thread.join() {
      Ok(answer) => answer,
      Err(panic) => std::panic::resume_unwind(panic),
    },
    Err(error) => panic!("could not start the thread the case runs on: {}", error),
  }
}

/// Parses one expression out of source.
pub(crate) fn parse_expr(source: &str) -> Expr {
  let file = anonymous_file(source);

  match parser_for(&file).parse_expr() {
    Ok(expr) => *expr,
    Err(error) => panic!("failed to parse `{}`: {:?}", source, error),
  }
}

pub(crate) fn anonymous_file(source: &str) -> Lrc<SourceFile> {
  let source_map: Lrc<SourceMap> = Default::default();

  source_map.new_source_file(FileName::Anon.into(), source.to_string())
}

/// A parser over one file, in the syntax every suite here reads.
///
/// One copy for the same reason the assertions have one: an expression and the
/// module it was written in have to be read under the same syntax, or a suite
/// comes to disagree with another about what the author wrote.
pub(crate) fn parser_for(file: &SourceFile) -> Parser<Lexer<'_>> {
  Parser::new_from(Lexer::new(
    Syntax::Es(EsSyntax {
      jsx: true,
      ..Default::default()
    }),
    Default::default(),
    StringInput::from(file),
    None,
  ))
}
