# 02 — A refused fold names its rule

**What to build:** An author whose call is refused learns which rule refused
it.

Today a refusal is silent: the fold answers nothing and the surrounding
evaluation reports `Unsupported expression: CallExpression`, which names the
syntax rather than the reason. The rules that refuse are specific and each
knows exactly why it fired, so the information exists and is discarded.

This also establishes the channel the rest of this effort needs. Later tickets
delete the method tables, and the tables are where today's more useful
messages come from — without this, deleting them makes diagnostics worse
before it makes them better.

Message text is not a parity obligation. The comparison harness compares class
name, rule text and style-object shape, never message text, and it already has
a verdict for the case where both compilers reject an input with different
wording. Where this compiler's message is better than the reference
compiler's, it stays better.

**Blocked by:** None — can start immediately.

**Status:** resolved

- [x] The fold answers either a value or a refusal carrying its reason; there
      is no third state
- [x] Every rule that refuses supplies its own reason, and a value the bridge
      cannot carry supplies one too
- [x] A call the engine throws on carries the engine's own message rather than
      being flattened into the generic refusal
- [x] The caller raises the refusal as an ordinary deopt, so where the call
      sat still decides whether the author sees a failed build or working
      runtime code
- [x] Each refusal reason is pinned by a test asserting the message an author
      sees, at the highest seam available

## Answer

The fold answers `Option<Result<EvaluateResultValue, Refusal>>`. The `Option` is
the candidacy question — a call the guard never recognised is not this module's,
and the dispatch behind it decides — and the `Result` is the fold's own answer:
a value, or the rule that declined it. There is no third state inside the fold.

Internally one `Decline` enum carries both, so the single guard walk answers
`NotACandidate` and `Rule(reason)` out of the same `?`-propagating traversal,
and a rule that fires in the middle of a chain reaches the caller with its own
words rather than the outermost link's.

The rules and the sentences each one hands the author:

| Rule | Sentence |
| --- | --- |
| locale-sensitive method | `Cannot fold '<method>' at compile time.` |
| numeric literal receiver | `Cannot call '<method>' on a number literal.` |
| unbounded amplifying length | `Cannot bound the string '<method>' would build.` |
| nesting past the bound | `Expression is too deeply nested to evaluate at compile time.` |
| a value the bridge cannot carry | `The folded value is <kind>, which has no compile-time representation.` |
| an array past the bound | `Array length is too large to evaluate at compile time.` |
| an object past the bound | `Object is too large to evaluate at compile time.` |
| the engine threw | `Cannot fold '<method>' at compile time.` + the engine's own line |

The throw case names the method as well as carrying the engine's sentence,
because the engine's sentence does not always name it: a call to a method that
does not exist reads `undefined` and calls it, so the language answers
`not a callable function`, which tells an author nothing the code frame has not
already shown them. That is the one place this compiler's message is
deliberately better than both the engine's and the reference compiler's, which
answers `Unsupported expression: CallExpression` there.

Seven of the eight are pinned at the highest seam,
`crates/stylex-transform/tests/transform_stylex_create_test/engine_fold_refusals.rs`,
asserting the sentence *and* the key path an author reads. The eighth,
`object_size_too_large`, is pinned at the evaluator seam instead: the only input
that reaches it is an object literal of 10,001 properties, which is a generated
source string rather than a fixture anybody would read. The evaluator-seam cases
in `engine_fold_tests.rs` also cover the result kinds a transform fixture cannot
carry.

The messages themselves are unit-tested in `stylex-constants`, which sits inside
the 100%-line-coverage gate that `stylex_transform` is excluded from — without
them the six new functions would be the only uncovered lines in that crate.

The caller raises the refusal as an ordinary `deopt`, so where the call sat
still decides whether the author sees a failed build or working runtime code;
the dynamic-style case is pinned in the same file. `try_fold` is asked before
the expression is cloned for a code frame, so a fold that succeeds pays for no
clone it does not use.
