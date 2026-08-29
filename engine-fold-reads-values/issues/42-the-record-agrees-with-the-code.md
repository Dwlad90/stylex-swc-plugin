# 42 — The record agrees with the code

**What to build:** The spec, the tickets, the glossary, the ADRs and the READMEs
say the same thing about what this compiler refuses and what crosses its bridge,
so the next reader can tell a decision from an oversight.

**Six places they disagree.**

*The refusal count.* Ticket 14's checkbox still reads "the only surviving
'reference compiles, we refuse' category is the locale-sensitive methods", and it
is unticked. Four categories shipped. ADR 0008 amends the count explicitly; the
ticket does not, and it is the only requirement in the tracker still recorded as
unmet. Whatever 28 rules on changes this number again, which is why this ticket
waits for it.

*The bridge's contents.* The spec says a theme reference *"does not cross at
all"* and lists it under "refused in both directions". A token group does cross,
as the string its own `toString` answers — measured against 0.19.0, compensated
by the property-read hand-back, and shipped deliberately. The refusal categories
got an amendment in ADR 0008 when they changed; the bridge section never did.

*Two terms with no entry.* ADR 0008 uses **token group** seven times, a commit
subject names it, and six test directories are named for it — the glossary has no
such entry and calls the thing **theme reference**, whose `_Avoid_` line lists
three other spellings but not this one. **Value bridge** is the same story
against **transport** and **carried value**.

*The harvest gate.* `parity:harvest:check` cannot see the capabilities this
effort added, because the harvester expresses `property`/`value` declarations and
the new folds are module-shaped. That is a limit of the gate, not staleness in
the corpus — and it should be stated in the parity README's harvest paragraph,
where someone reading a ±10-line diff will otherwise conclude the opposite.

*A README advertising deleted code.* `stylex-evaluator/README.md` still describes
`sort_numbers_factory`, which this effort removed along with its tests.

*The commit scope splits mid-branch.* Twenty-nine commits use
`stylexswc/transform`; the last four use `stylex_utils`, `stylex_structures`,
`stylex_js` and `stylex_constants`. Both spellings appear in the repo's history
and no `scope-enum` rule exists, so nothing failed — which is exactly why it will
keep happening. Pick one and write it where a committer will see it.

**Blocked by:** 28.

**Status:** resolved

- [x] Ticket 14's checkbox says what shipped and is closed
- [x] The spec's bridge section records the token group crossing, with the
      measurement that justified it
- [x] **token group** and **value bridge** each resolve to an entry, whether as
      their own or on an existing `_Avoid_` line
- [x] The parity README states what the harvest gate cannot see
- [x] `stylex-evaluator/README.md` describes only code that exists
- [x] One commit scope spelling is chosen and recorded in the git conventions

## What was changed, and where a reading was corrected

Six places, plus two the second of them dragged in.

**The refusal count.** Ticket 14's last checkbox now reads four rather than one
and is ticked, and its Answer section — which said three, having been written
before ticket 15 — is re-counted against the corpus and points at ADR 0008 for
the ruling rather than restating it.

**The bridge's contents.** The spec's bridge section records the theme reference
crossing as the string its own `toString` answers, with the measurement that
justified it and the property-read hand-back that pays for it. ADR 0008's value
bridge category had to move with it: it still said the row stayed open, which
would have been a fresh disagreement the moment the spec was amended. Two rows
counted there in ticket 09 have closed since — the theme reference and `String()`
of a spread holding a function — and one arrived with ticket 23, so the category
is three rows and all are wanted, which leaves the **held open** kind with no
occupant. It is kept, named,
and said to be empty, so the next row that needs it is filed against a name that
exists.

*The reading that was corrected, and then corrected again.* `CONTEXT.md` and
`global_conversion.rs` both explained the hand-back by saying this compiler's own
values have no JavaScript form to cross as. That stopped being true of the theme
reference — but the first replacement over-corrected in the other direction, and
review caught it: it read as though a conversion over a theme reference always
came back to that module. It does not. `String(group)` folds in the engine like
any other string, because the group crossed as one; only two shapes come back,
and both are where that string has lost what the call needs — an answer that is
still an object, which `Object(group)` is, and a property read as a value. The
environment object and the injected function map are the ones with no JavaScript
form at all, and every call over one comes back. `engine_fold/mod.rs` holds both
conditions and is what the prose now describes.

ADR 0008 said the same thing twice more and both were fixed with it: its dispatch
paragraph still listed a resolved theme reference among the values holding
nothing the engine could be handed, and its value bridge count read two where the
corpus has three — a function interpolated in a template is the same refusal as
`Number()` of a function, and both are the engine being built with no function
source text.

**Two terms with no entry.** `token group` joins the `_Avoid_` line of **Theme
reference**, and `value bridge` the one on **Transport**. Neither earns an entry
of its own: they name things the glossary already names, and a second entry for
one concept is the disagreement this ticket exists to remove. `theme reference`
outnumbers `token group` ninety-five to a handful in the Rust sources, so it is
the spelling that stands, and the one sentence of prose in the spec still using
the other was changed with it. Test module names and corpus row ids still carry
it; they are identifiers rather than prose, and renaming them churns snapshot
directories and pinned rows for nothing.

**The harvest gate.** The parity README's harvest section now states what the
scan cannot see. The first wording claimed a module-shaped test carries nothing
for the scan to take, which is too strong — shape 5 reads any embedded
`stylex.create` object and takes its declarations. The limit is narrower and is
what the paragraph says now: what the harvest takes is the *value* a test
carries, never the *capability* it was written for, so a run reporting no change
after such a suite lands is the gate answering truthfully rather than the corpus
confirming coverage.

**A README advertising deleted code.** `stylex-evaluator/README.md` listed
`sort_numbers_factory`, which exists nowhere, and five more capabilities that
live in `stylex-utils`, `stylex-ast` and `postcss-value-parser` rather than here.
The list now names what the crate exports, including the nested-configuration
readers and writers it never mentioned at all.

**The commit scope.** `guidelines/git/CONVENTIONS.md` picks the name the package
declares — the Cargo `name` for a crate, the npm name without its `@` for a Node
package — because it is derivable rather than remembered, and names the
historical spellings so a committer reading `git log` knows they are not the
model.
