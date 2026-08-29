# 41 — Every refusal has a module that reaches it

**What to build:** Every diagnostic this effort added is produced by a module
somebody could write, so a sentence that can never be reached is either reachable
or deleted.

**What is unreached.** `uncoercible_value` has five emission sites and no
transform test — the sentence appears nowhere outside its own string-shape unit
test. `UNEXPECTED_MEMBER_LOOKUP` gained four sites in the call arm and is reached
by none of them. `bound_value_has_too_many_entries` has a string-shape test and
no module; a named ten-element array against `maxFoldedEntries: 8` reaches it.
`ARGUMENT_WITHOUT_VALUE` has zero references anywhere in the workspace, including
tests: it is dead, and the condition its doc comment describes is the one that
currently emits `uncoercible_value` instead.

**And three inward hand-backs.** The bridge's rule is that a shape it does not
carry is handed back rather than refused, and the dispatch below answers for it.
Three shapes have no test at all: a single injected function config, a callback,
and the AST-keyed map variant. `EnvObject` and `FunctionConfigMap` have one
conversion each. These are the rule the whole bridge rests on, and the rule is
observed on two of five shapes.

**One refusal names no rule.** `s.constructor.name` answers `Unexpected error:`
rather than naming the escaping-property rule that refused it — which is spec
story 27, *"a fold that refuses names the rule that refused it, so that I am not
handed `Unsupported expression` and left to guess."* The rule exists and is
correct; only its sentence is missing.

**Two diagnostics changed and nothing records it.** `BUILT_IN_FUNCTION` and
`INVALID_ARRAY_LENGTH` were removed — they have no in-repo consumer, so nothing
broke — and `Array(-1)` now reports the engine's own `RangeError` wording
instead. Message text is not a parity obligation and this compiler's messages are
its own, so the wording is a choice rather than a regression; the ticket's job is
to make the choice deliberately and pin whichever wording it keeps.

**A string-shape unit test is not this.** The spec is explicit that a good test
here names something an author can observe: a module goes in, CSS and metadata
come out. A test asserting a constant's wording proves the constant exists.

**What not to test.** Some refusals are invariant breaks — a fold reaching a
state the guard already excluded. Those should be marked as uncovered
deliberately rather than reached by a contrived module; the ticket should say
which it judged to be which.

## While the file is open

A run of sentences in `evaluation_errors.rs` exceeds the hundred-character width
— one is two hundred and thirty — inside string literals, where rustfmt cannot
wrap them and the linters therefore never complain. `concat!` is already used a
few files over for exactly this.

**Blocked by:** none — can start immediately.

**Status:** resolved

- [x] A module reaching each of the five `uncoercible_value` sites -- three of
      them, plus one already reached; the other two are invariant breaks and are
      marked as such
- [x] A module reaching `UNEXPECTED_MEMBER_LOOKUP` -- two of its four sites; the
      other two are invariant breaks -- and one reaching
      `bound_value_has_too_many_entries`
- [x] `ARGUMENT_WITHOUT_VALUE` is deleted, or wired to the condition it describes
      and reached -- deleted
- [x] The three untested hand-back shapes each fold through the dispatch below --
      two of them; the AST-keyed map cannot be one, and is marked so
- [x] `s.constructor.name` names the rule that refused it
- [x] `Array(-1)`'s wording is chosen deliberately and pinned
- [x] Any refusal judged unreachable is marked so, with the invariant named
- [x] Over-width error sentences are wrapped with `concat!`

**Resolution:** `refusals_a_module_reaches` is the new file, and every sentence
in it comes out of a compiled module rather than off a constant.

*The five `uncoercible_value` sites split three ways.* Three are reached by a
module and pinned there: a function read off an object literal, a StyleX
function read as a value, and an inline arrow beside an argument the bridge
cannot carry all reach the `String` coercion with nothing to coerce, and a lone
surrogate beside such an argument reaches `Number`'s. The site inside the guard,
where a StyleX call in a callback is handed an argument the bridge cannot carry,
was already reached by `stylex_functions_in_a_fold`. The remaining two are
invariant breaks and are marked as such where they are written: the
argument-count guard fires only if `evaluate_func_call_args` could drop an
argument without deopting, which the confidence check above it has already
returned on, and the `ToObject` refusal only if a value reached it that is
neither an object nor a function -- and every shape the evaluator answers with is
one or the other. Neither is reachable from any module, which a run of the whole
suite against a marker sentence confirmed.

*The member lookup splits the same way.* Two of the four sites in the call arm
are reached and pinned -- a method on the namespace object a conversion handed
back, and the computed spelling of the same lookup on a theme reference. The
other two destructure a property one line after asking whether it is an
identifier, so each now says it is unreachable and why. `ARGUMENT_WITHOUT_VALUE`
is deleted: nothing referenced it, and the condition its text described is the
argument-count guard above.

*Two hand-back shapes fold, and the third cannot.* The single injected function
config and the callback each reach the conversion below the fold and are pinned
there. The AST-keyed map cannot: it is only ever the value a `stylex.create`
argument evaluates to, and an argument to a coercion is evaluated by the
ordinary dispatch, which never builds one. Its arm is kept and says so.

*The escaping-property rule now applies to a read.* `s.constructor` folded to
`undefined` -- a quietly wrong value, since a string's `constructor` is `String`
-- and `s.constructor.name` refused one property later with `Unexpected error:`.
Both are now the rule's own sentence, in either spelling and on any receiver,
which cost one further divergence from the reference compiler on a receiver
carrying the name as an own property. The guard already refused that same read
with a call on the end of it, so the parting is between the two compilers rather
than between this compiler's two paths; ADR 0008 carries the argument.

*The wordings are chosen.* `Array(-1)` keeps the engine's own `RangeError:
invalid array length` under this compiler's naming of the call, because a
negative length is the language refusing rather than one of this module's
ceilings. `BUILT_IN_FUNCTION` and `INVALID_ARRAY_LENGTH` stay deleted. All three
choices are recorded in ADR 0008 beside the message-text rule they rest on.

*And the file reads at a hundred characters.* Nine sentences over the width, one
of them two hundred and thirty, are wrapped with `concat!`. The tenth long line
is a URL inside a commented-out constant, which nothing can wrap.
