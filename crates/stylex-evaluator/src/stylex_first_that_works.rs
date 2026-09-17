use stylex_macros::stylex_panic;
use swc_core::ecma::ast::Expr;

use stylex_ast::ast::convertors::create_string_expr;
use stylex_ast::ast::factories::{create_array_expression, create_expr_or_spread};
use stylex_constants::constants::messages::EXPRESSION_IS_NOT_A_STRING;
use stylex_regex::regex::IS_CSS_VAR;
use stylex_state::resolution::convertors::convert_expr_to_str;
use stylex_state::{functions::FunctionMap, state_manager::StateManager};

/// The `var(` prefix a CSS variable reference starts with, and whose length is
/// what the name inside it begins after.
const CSS_VAR_PREFIX: &str = "var(";

/// The variable name `text` references, or `None` where it is not a bare
/// reference to a single one — the one shape of argument this function reads as a
/// variable rather than as a value. So `var(--x)` reads as `--x`, and
/// `var(--x, red)` as nothing.
///
/// One question rather than a test and a slice, because the slice is only in
/// range for text the test admitted, and two callers each spelling the pair is
/// how one of them comes to slice text the other would have rejected.
///
/// A regex failure reads as "not a variable", which keeps a broken match from
/// deciding a fallback order. It is read that way rather than reported: the
/// pattern is a literal this crate compiles itself, so the only failure it has
/// is a backtracking limit no `var(…)` can reach, and a report no case can
/// produce is a sentence nothing checks.
///
/// The prefix is stripped through `and_then` rather than `?` for the same
/// reason: the pattern is what admitted the text, so the prefix is there
/// whenever the strip runs.
pub(crate) fn css_variable_name(text: &str) -> Option<&str> {
  match IS_CSS_VAR.is_match(text).unwrap_or(false) {
    true => text
      .strip_prefix(CSS_VAR_PREFIX)
      .and_then(|inner| inner.strip_suffix(')')),
    false => None,
  }
}

/// What shape `firstThatWorks` answers with, and which arguments fill it — as
/// positions rather than as values.
///
/// Positions because two callers answer this function on two different kinds of
/// value — the expressions the evaluator holds, and the values the compile-time
/// engine holds — and the ordering arithmetic is the half they must not disagree
/// about. Each caller then reads its own values back out at these positions.
///
/// The *shape* is here for the same reason. Which of the three a call answers
/// with is policy, not assembly: a chain with nothing behind it stays one value
/// so that it can be concatenated like any other fold, and a call naming no
/// variable at all is its arguments reversed. Written out at each caller, those
/// two rules were two chances for the callers to part company over something
/// neither of them decides.
pub(crate) enum Fallbacks {
  /// No argument names a variable, so there is no chain to build and no value to
  /// strip a name out of. The answer is the arguments reversed.
  Reversed,
  /// One `var()` chain and nothing behind it, which stays a single value.
  Chain(Vec<usize>),
  /// The chain, followed by the arguments that stay declarations of their own,
  /// highest priority last.
  ChainAndRest(Vec<usize>, Vec<usize>),
}

/// How `count` arguments fall back to one another, given which of them name a
/// variable.
///
/// `is_var` is asked rather than handed in, because reading an argument can cost
/// an evaluation and this walk asks about each position at most once and about
/// nothing past the chain's end — where a list built up front would have read
/// every argument, including the ones the chain never reaches.
pub(crate) fn plan_fallbacks(count: usize, mut is_var: impl FnMut(usize) -> bool) -> Fallbacks {
  let Some(first_var) = (0..count).find(|&index| is_var(index)) else {
    return Fallbacks::Reversed;
  };

  // The chain ends on the first argument after the variables that is not one:
  // that value is the innermost fallback, and everything past it is a
  // declaration of its own.
  let end = ((first_var + 1)..count)
    .find(|&index| !is_var(index))
    .map_or(count, |first_value| first_value + 1);

  let chain = (first_var..end).rev().collect();
  let rest: Vec<usize> = (0..first_var).rev().collect();

  match rest.is_empty() {
    true => Fallbacks::Chain(chain),
    false => Fallbacks::ChainAndRest(chain, rest),
  }
}

/// The single value a fallback chain collapses to, folded from its parts in the
/// order [`Fallbacks::chain`] lists them.
///
/// Each part is a variable's own name — `--x` for a `var(--x)` argument — or the
/// text of the value the chain bottoms out on.
pub(crate) fn fold_fallback_chain(parts: impl IntoIterator<Item = String>) -> String {
  parts
    .into_iter()
    .fold(String::new(), |so_far, part| match so_far.is_empty() {
      false => {
        let mut next = String::with_capacity(part.len() + so_far.len() + 7);
        next.push_str(CSS_VAR_PREFIX);
        next.push_str(&part);
        next.push_str(", ");
        next.push_str(&so_far);
        next.push(')');
        next
      },
      true if part.starts_with("--") => {
        let mut next = String::with_capacity(part.len() + 5);
        next.push_str(CSS_VAR_PREFIX);
        next.push_str(&part);
        next.push(')');
        next
      },
      true => part,
    })
}

/// Each argument's text, read at most once.
///
/// The plan asks about an argument, and the fold then reads the same argument
/// again. For a string literal that second read is one more `String`; for an
/// identifier it is a second binding lookup. The answer cannot change between
/// the two asks, so the first one stands.
struct ArgTexts<'a> {
  args: &'a [Expr],
  /// One slot per argument, filled by the first read of that argument.
  texts: Vec<Option<String>>,
  state: &'a mut StateManager,
  functions: &'a FunctionMap,
}

impl<'a> ArgTexts<'a> {
  fn new(args: &'a [Expr], state: &'a mut StateManager, functions: &'a FunctionMap) -> Self {
    ArgTexts {
      args,
      texts: vec![None; args.len()],
      state,
      functions,
    }
  }

  /// The text of `args[index]`.
  ///
  /// Reading is the one thing that can fail here, and it fails the same way
  /// wherever it is asked, so the sentence lives in one place.
  fn text(&mut self, index: usize) -> &str {
    let (args, state, functions) = (self.args, &mut self.state, self.functions);

    self.texts[index].get_or_insert_with(|| {
      match convert_expr_to_str(&args[index], state, functions) {
        Some(text) => text,
        None => stylex_panic!("{}", EXPRESSION_IS_NOT_A_STRING),
      }
    })
  }

  /// The text of `args[index]`, moved out rather than copied.
  ///
  /// The fold asks about each argument of a chain once -- [`plan_fallbacks`]
  /// lists every position once -- so nothing reads the slot after this.
  fn take_text(&mut self, index: usize) -> String {
    self.text(index);
    self.texts[index].take().unwrap_or_default()
  }
}

pub fn stylex_first_that_works(
  args: Vec<Expr>,
  state: &mut StateManager,
  functions: &FunctionMap,
) -> Expr {
  // The texts are read and folded in a scope of their own, so the answer below
  // is assembled with none of them still held.
  let (plan, chain_text) = {
    let mut texts = ArgTexts::new(&args, state, functions);

    let plan = plan_fallbacks(args.len(), |index| {
      css_variable_name(texts.text(index)).is_some()
    });

    // The chain's text is read the same way whichever shape holds it. A shape
    // with no chain folds nothing, and the empty text it stands for is never
    // read.
    let chain_text = match &plan {
      Fallbacks::Reversed => String::new(),
      Fallbacks::Chain(chain) | Fallbacks::ChainAndRest(chain, _) => {
        fold_fallback_chain(chain.iter().map(|&index| {
          let text = texts.take_text(index);

          // A variable contributes its name; anything else contributes itself,
          // which is where the chain bottoms out.
          match css_variable_name(&text) {
            Some(name) => name.to_string(),
            None => text,
          }
        }))
      },
    };

    (plan, chain_text)
  };

  match plan {
    // The arguments reversed, and each of them moved rather than copied. This
    // shape answers with the whole list, so a copy here is a copy of every
    // argument.
    Fallbacks::Reversed => {
      let elems = args
        .into_iter()
        .rev()
        .map(|arg| Some(create_expr_or_spread(arg)))
        .collect();

      create_array_expression(elems)
    },
    Fallbacks::Chain(_) => create_string_expr(&chain_text),
    Fallbacks::ChainAndRest(_, rest) => {
      let chain = create_string_expr(&chain_text);
      let elems = std::iter::once(chain)
        .chain(rest.iter().map(|&index| args[index].clone()))
        .map(|expr| Some(create_expr_or_spread(expr)))
        .collect();

      create_array_expression(elems)
    },
  }
}
