//! The search for a compiler call on a logical operand, and what a refused walk
//! of a losing side leaves behind.

use super::*;

use swc_core::{
  common::{DUMMY_SP, GLOBALS, Globals},
  ecma::{
    ast::VarDeclarator,
    visit::{VisitMut, VisitMutWith},
  },
};

use crate::evaluate::source_evaluation::*;
use crate::tests::scaffolding::deep_or;
use stylex_structures::stylex_options::StyleXOptions;

/// Sets every span it visits to the dummy span.
struct NoSpans;

impl VisitMut for NoSpans {
  fn visit_mut_span(&mut self, span: &mut Span) {
    *span = DUMMY_SP;
  }
}

/// What one search answered.
struct Searched {
  /// Whether the expression calls a compiler function.
  found: bool,
  /// The answers the search filed.
  answers: FxHashMap<Span, bool>,
  /// How many declarations the search walked.
  entered: usize,
}

/// Searches `expr` against `fns` and a state holding `declarations`.
fn searched_over(expr: &Expr, declarations: Vec<VarDeclarator>, fns: &FunctionMap) -> Searched {
  let mut traversal_state = StateManager::new(StyleXOptions::default());

  for declaration in declarations {
    traversal_state.push_declaration(declaration);
  }

  let mut answers = FxHashMap::default();
  let mut search = CompilerCalls::new(&traversal_state, fns, &mut answers);

  search.answer(expr);

  let (found, entered) = (search.found, search.entered);

  Searched {
    found,
    answers,
    entered,
  }
}

/// Searches `expr` against `fns` and a state that declares each name with the
/// source beside it.
fn searched<N: AsRef<str>, S: AsRef<str>>(
  expr: &Expr,
  declarations: &[(N, S)],
  fns: &FunctionMap,
) -> Searched {
  let declarations = declarations
    .iter()
    .map(|(name, init)| a_declaration_of(name.as_ref(), init.as_ref()))
    .collect();

  searched_over(expr, declarations, fns)
}

/// No declarations.
const NONE: [(&str, &str); 0] = [];

/// Declarations named `f0` to `f{count - 1}`, each of which calls all the
/// others.
fn functions_that_all_call_each_other(count: usize) -> Vec<(String, String)> {
  (0..count)
    .map(|own| {
      let calls = (0..count)
        .filter(|other| *other != own)
        .map(|other| format!("f{other}()"))
        .collect::<Vec<_>>()
        .join(", ");

      (format!("f{own}"), format!("() => [{calls}]"))
    })
    .collect()
}

/// Functions that all call each other are each walked one time. The search
/// does not walk each path through them, so its cost grows with the calls and
/// not with the count of paths.
#[test]
fn functions_that_all_call_each_other_are_each_entered_once() {
  GLOBALS.set(&Globals::new(), || {
    let count = 12;
    let declarations = functions_that_all_call_each_other(count);

    let search = searched(&parse_expr("f0()"), &declarations, &a_function_fold());

    assert!(!search.found);
    assert_eq!(search.entered, count);

    // A compiler call in the last one is found from the first one.
    let mut declarations = declarations;
    let last = &mut declarations[count - 1].1;

    last.insert_str(last.len() - 1, ", own('a')");

    let search = searched(&parse_expr("f0()"), &declarations, &a_function_fold());

    assert!(search.found);
    assert_eq!(search.entered, count);
  });
}

/// Two functions that call each other are each followed once, and the search
/// ends.
#[test]
fn functions_that_call_each_other_are_followed_once() {
  GLOBALS.set(&Globals::new(), || {
    let declarations = [("f", "() => g()"), ("g", "() => [f(), own('a')]")];
    let expr = parse_expr("f()");

    let search = searched(&expr, &declarations, &a_function_fold());

    assert!(search.found);
    assert_eq!(search.entered, 2);
    assert_eq!(search.answers.get(&expr.span()), Some(&true));

    let search = searched(&parse_expr("f()"), &declarations, &FunctionMap::default());

    assert!(!search.found);
  });
}

/// A function in a cycle that the search left before it found the call gets
/// the answer of the cycle when the search asks about it again.
#[test]
fn a_function_in_a_cycle_gets_the_answer_of_the_cycle() {
  GLOBALS.set(&Globals::new(), || {
    let declarations = [("f", "() => [g(), own('a')]"), ("g", "() => [f()]")];
    let Expr::Bin(bin) = parse_expr("f() || g()") else {
      panic!("the source is not a binary expression");
    };

    let search = searched(&Expr::Bin(bin.clone()), &declarations, &a_function_fold());

    assert!(search.found);
    assert_eq!(search.entered, 2);
    assert_eq!(search.answers.get(&bin.right.span()), Some(&true));
  });
}

/// A declaration with no span is walked once in a search too, but its answer
/// is not filed by span.
#[test]
fn a_declaration_with_no_span_is_walked_once() {
  GLOBALS.set(&Globals::new(), || {
    let mut declaration = a_declaration_of("f", "() => own('a')");

    declaration.visit_mut_with(&mut NoSpans);

    let search = searched_over(
      &parse_expr("f() || f()"),
      vec![declaration],
      &a_function_fold(),
    );

    assert!(search.found);
    assert_eq!(search.entered, 1);
    assert!(!search.answers.contains_key(&DUMMY_SP));
  });
}

/// One search files an answer for each operand under the one it was asked
/// about, so the next question on a part of it is a lookup. It files nothing
/// else.
#[test]
fn one_search_answers_for_every_part() {
  GLOBALS.set(&Globals::new(), || {
    let Expr::Bin(bin) = parse_expr("a || (b || own('c'))") else {
      panic!("the source is not a binary expression");
    };

    let Searched { found, answers, .. } = searched(&bin.right, &NONE, &a_function_fold());

    assert!(found);

    let Expr::Paren(paren) = &*bin.right else {
      panic!("the right operand is not in parentheses");
    };
    let Expr::Bin(inner) = &*paren.expr else {
      panic!("the inner operand is not a binary expression");
    };

    assert_eq!(answers.get(&inner.left.span()), Some(&false));
    assert_eq!(answers.get(&inner.right.span()), Some(&true));
    assert_eq!(answers.len(), 3);
  });
}

/// The search files the test of a conditional, because the guard asks about
/// it, and walks both branches.
#[test]
fn a_conditional_files_its_test() {
  GLOBALS.set(&Globals::new(), || {
    let Expr::Cond(cond) = parse_expr("a ? b : own('c')") else {
      panic!("the source is not a conditional");
    };

    let Searched { found, answers, .. } =
      searched(&Expr::Cond(cond.clone()), &NONE, &a_function_fold());

    assert!(found);
    assert_eq!(answers.get(&cond.test.span()), Some(&false));
  });
}

/// A dynamic import has no callee expression, so it names no function.
#[test]
fn a_dynamic_import_calls_no_compiler_function() {
  GLOBALS.set(&Globals::new(), || {
    assert!(!searched(&parse_expr("import('a')"), &NONE, &a_function_fold()).found);
  });
}

/// An operand nested far deeper than a test thread can recurse is searched on
/// a stack that grows, so the search cannot overflow.
#[test]
fn a_deep_operand_is_searched_without_overflow() {
  on_a_thread_of(SMALL_THREAD, || {
    GLOBALS.set(&Globals::new(), || {
      let deep = deep_or(100_000);

      assert!(!searched(&deep, &NONE, &a_function_fold()).found);

      // A tree this deep drops by recursion too, so it is not dropped here.
      std::mem::forget(deep);
    });
  });
}

/// A losing side that the walk refuses after it carried a value gives the
/// value back. So the value does not count against the entry ceiling, and the
/// live part of the call still folds.
#[test]
fn a_refused_losing_side_gives_back_what_it_carried() {
  let source = "[true || [first, own(missing)], second.length].join()";
  let result = evaluated_in_a_state(
    |state| {
      state.options.core.max_folded_entries = 5;
      state.push_declaration(a_declaration_of("first", "[1, 2, 3]"));
      state.push_declaration(a_declaration_of("second", "[1, 2, 3]"));
    },
    &a_function_fold(),
    source,
  );

  assert_eq!(folded_text_of(result, source), "true,3");
}

/// An operator that a transform built without a span has no stable name for
/// the printer, so the fold hands the call back to the dispatch below, which
/// evaluates both operands itself.
#[test]
fn an_operator_with_no_span_is_handed_back() {
  let source = "[true || own('a')].join()";

  let result = GLOBALS.set(&Globals::new(), || {
    let mut expr = parse_expr(source);

    expr.visit_mut_with(&mut NoSpans);

    let mut traversal_state = StateManager::new(StyleXOptions::default());

    crate::evaluate::evaluate(&expr, &mut traversal_state, &a_function_fold())
  });

  assert_eq!(folded_text_of(result, source), "true");
}
