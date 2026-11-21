//! The search for a compiler call on a logical operand, and what a refused walk
//! of a losing side leaves behind.

use super::*;

use swc_core::{
  common::{DUMMY_SP, GLOBALS, Globals},
  ecma::visit::{VisitMut, VisitMutWith},
};

use crate::evaluate::source_evaluation::*;
use crate::tests::scaffolding::deep_or;
use stylex_structures::stylex_options::StyleXOptions;

/// Searches `expr` against `fns` and a state holding `declarations`, and
/// answers whether it calls a compiler function, with the answers it filed.
fn searched(
  expr: &Expr,
  declarations: &[(&str, &str)],
  fns: &FunctionMap,
) -> (bool, FxHashMap<Span, bool>) {
  let mut traversal_state = StateManager::new(StyleXOptions::default());

  for (name, init) in declarations {
    traversal_state.push_declaration(a_declaration_of(name, init));
  }

  let mut answers = FxHashMap::default();
  let mut search = CompilerCalls {
    traversal_state: &traversal_state,
    functions: fns,
    answers: &mut answers,
    following: Vec::new(),
    found: false,
    cut: false,
  };

  expr.visit_with(&mut search);

  let found = search.found;

  (found, answers)
}

/// Two functions that call each other are each followed once, and the search
/// ends. The answer under the cut is not filed, because it can be short.
#[test]
fn functions_that_call_each_other_are_followed_once() {
  GLOBALS.set(&Globals::new(), || {
    let declarations = [("f", "() => g()"), ("g", "() => [f(), own('a')]")];
    let expr = parse_expr("f()");

    let (found, answers) = searched(&expr, &declarations, &a_function_fold());

    assert!(found);
    assert!(!answers.contains_key(&expr.span()));

    let (found, _) = searched(&parse_expr("f()"), &declarations, &FunctionMap::default());

    assert!(!found);
  });
}

/// One search files an answer for every expression under the one it was asked
/// about, so the next question on a part of it is a lookup.
#[test]
fn one_search_answers_for_every_part() {
  GLOBALS.set(&Globals::new(), || {
    let Expr::Bin(bin) = parse_expr("a || (b || own('c'))") else {
      panic!("the source is not a binary expression");
    };

    let (found, answers) = searched(&bin.right, &[], &a_function_fold());

    assert!(found);

    let Expr::Paren(paren) = &*bin.right else {
      panic!("the right operand is not in parentheses");
    };
    let Expr::Bin(inner) = &*paren.expr else {
      panic!("the inner operand is not a binary expression");
    };

    assert_eq!(answers.get(&inner.left.span()), Some(&false));
    assert_eq!(answers.get(&inner.right.span()), Some(&true));
  });
}

/// A dynamic import has no callee expression, so it names no function.
#[test]
fn a_dynamic_import_calls_no_compiler_function() {
  GLOBALS.set(&Globals::new(), || {
    assert!(!searched(&parse_expr("import('a')"), &[], &a_function_fold()).0);
  });
}

/// An operand nested far deeper than a test thread can recurse is searched on
/// a stack that grows, so the search cannot overflow.
#[test]
fn a_deep_operand_is_searched_without_overflow() {
  on_a_thread_of(SMALL_THREAD, || {
    GLOBALS.set(&Globals::new(), || {
      let deep = deep_or(100_000);

      assert!(!searched(&deep, &[], &a_function_fold()).0);

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
  struct NoSpans;

  impl VisitMut for NoSpans {
    fn visit_mut_span(&mut self, span: &mut Span) {
      *span = DUMMY_SP;
    }
  }

  let source = "[true || own('a')].join()";

  let result = GLOBALS.set(&Globals::new(), || {
    let mut expr = parse_expr(source);

    expr.visit_mut_with(&mut NoSpans);

    let mut traversal_state = StateManager::new(StyleXOptions::default());

    crate::evaluate::evaluate(&expr, &mut traversal_state, &a_function_fold())
  });

  assert_eq!(folded_text_of(result, source), "true");
}
