# 06 — The prototype surface the reference implementation folds

Status: `resolved`
Phase: Deferred

**Blocked by:** 05

**What to build:** Static method calls on string, array and object literals
fold as they do upstream.

Not a regression — this gap shipped in `0.18.3` and `0.18.4-rc.1` and has never
been reported. It is recorded here so the measurements are not lost. Do not
start it before 05 answers how.

Measured across 70 expressions, reference implementation versus this compiler:

| receiver        | upstream folds                                              | this compiler folds       |
| --------------- | ----------------------------------------------------------- | ------------------------- |
| string literal  | 28 methods, `startsWith` … `normalize`, plus `.length`       | `concat`, `charCodeAt`    |
| numeric literal | none — all four **throw**, see `../spec.md` non-goals        | none, different message   |
| array literal   | 15, incl. `sort` `reverse` `reduce` `some` `flat` `at`       | `join`, `map`, `filter`   |
| object literal  | prototype methods: `hasOwnProperty`, `toString`, …           | own keys only             |
| globals         | `String` `Number` `Math` `Object` `Array`; rejects `random`, `assign`, `JSON`, `Boolean`, `parseInt` | **identical, no gap**     |

Agreed boundaries, if this is built by hand rather than via 05:

- **String (25):** `startsWith` `endsWith` `includes` `indexOf` `lastIndexOf`
  `slice` `substring` `substr` `at` `charAt` `charCodeAt` `codePointAt`
  `concat` `repeat` `padStart` `padEnd` `split` `replace` `replaceAll`
  `toLowerCase` `toUpperCase` `trim` `trimStart` `trimEnd` `length`.
  Excluded: `normalize`, `localeCompare`, `toLocale*` — these need ICU, which
  is a dependency decision of its own; `match` / `matchAll` / `search` — the
  reference implementation already errors on regex literals, so deopting is
  parity.
- **Array (16):** `includes` `indexOf` `lastIndexOf` `at` `slice` `concat`
  `join` `map` `filter` `find` `findIndex` `some` `every` `reduce` `flat`
  `length`. `evaluate_map` / `evaluate_filter` already invoke arrow callbacks,
  so the callback-taking ones are mostly reuse.
- **Object (6):** `hasOwnProperty` `toString` `valueOf` `isPrototypeOf`
  `propertyIsEnumerable` `toLocaleString`. `constructor` and `__proto__` deopt
  — they can only produce a value that is not a valid style, so folding them
  moves the error later and makes it worse.
- Unify the `EvaluateResultValue::Vec` and `Expr::Array` arms onto one dispatch
  **before** adding any method. They accept different sets today, which is why
  `["a","b"].map(x => x).join("-")` fails on the chained `join`. Two tables
  that must agree and are edited separately is the shape of the original bug.

**Deliberate divergences**, to be documented rather than quietly carried:

- Mutating methods keep deopting. The reference implementation folds
  `["a","b"].push("c")` to `3` and `["a","b"].sort().join("-")` to `"a-b"` by
  accident of reflection; `is_mutating_array_method` already refuses them.
  Matching that means implementing mutation semantics inside a pure evaluator
  to serve input nobody writes.
- Unpaired surrogates become `U+FFFD`. `"\u{1F600}a".slice(1)` is a lone low
  surrogate in JS; `Lit::Str`'s `Atom` is UTF-8 and cannot hold one.
  Substituting the replacement character keeps the declaration byte-identical
  to the reference implementation's — which itself becomes `EF BF BD` the
  moment the stylesheet is written to disk — and diverges only in the generated
  class name (`xn5tvdn` where upstream emits `xi08yer`), for input whose
  rendered output is a replacement character in both compilers. Deopting
  instead would fail a build that upstream completes. Representing the
  surrogate faithfully would mean the evaluator no longer carries values as SWC
  AST nodes, which is a rewrite of its core value type and needs its own spec.
  Pin the divergence in the corpus.

## Comments

### 05's answer, and what it means here

05 is resolved: the engine matches the table (69/70 against Node, byte-identical
class names against the reference implementation across 80 rules), folds chains
for free, and costs nothing on the release gate. It grows the published artifact
by 5.6–6.1 MiB, which is accepted — that artifact is a build-time dependency, so
no consumer bundle carries it. What stands in the way is dependency resolution:
`boa_engine` requires `icu_normalizer ~2.0.0` where `icu_collator 2.3.1` needs
`~2.3.0`. 05 §1 measures the way through — pinning `icu_collator` to `=2.0.0`
resolves upstream boa into this workspace and costs exactly one thing, the
`new_root` decision in `pre_rule.rs` — and names relaxing boa's bound upstream
as the option that costs nothing here.

So this ticket needs that dependency question settled first, or the boundaries
above built by hand. Two of 05's findings apply whichever way it is built:

- Locale-sensitive methods must stay excluded — the exclusion list above was
  right. `normalize` could move into scope only with an ICU dependency of its own.
- `"abc".charCodeAt(10)` folding to `NaN` emits `z-index:NaN`, and that is what
  the reference implementation emits too. Parity and a useful refusal disagree
  here; this ticket has to choose one and say why. Both answers are currently
  asserted by tests that name each other, so whichever way this goes, one test
  gets deleted deliberately rather than discovered.
- The mutating-method boundary above is load-bearing and easy to lose. 05's
  first hook ran ahead of `is_mutating_array_method` and folded
  `["a","b"].sort().join("-")` for a while before its code review caught it. A
  dispatch that reflects, whether an engine or a table, refuses mutation only
  because something in front of it says so — and a chain hides the mutating
  call in the middle, so the check belongs at every link.

### Built on the engine, and what it settled

Built the way 05 answered: the guard in `engine_fold.rs` decides what may be
evaluated, the engine answers everything it admits, and there is no method table
to fall behind. The three decisions this ticket was left to make:

**`NaN` reaching a declaration: parity wins.** `"abc".charCodeAt(10)` folds to
`NaN` and `z-index:NaN` is emitted, which is what the reference implementation
emits. The refusal was the more useful answer for an author, and it lost anyway,
for two reasons. A class name is a hash of the declaration text, so that text is
a compatibility contract that a *better* answer breaks just as surely as a worse
one. And the choice was already made next door: `Number("10px")` folds to `NaN`
in this evaluator for exactly that reason, and one evaluator cannot hold both
rules. The two tests that named each other are now one test, which argues it;
`char_code_at_past_the_end_refuses_rather_than_aborting` is gone deliberately.

**The exclusion list was right, and one name moved back in.** Measured against
the reference implementation rather than assumed: it folds all four
locale-sensitive methods, and the engine answers them from the root locale —
`'i'.toLocaleUpperCase('tr')` comes back `I` where the language says `İ`. Those
stay refused, because a wrong declaration is worse than no declaration.
`normalize` moved into scope: `icu_normalizer` is not optional in the engine, so
`NFC` and `NFKC` agree with the language, and the corpus pins that they agree
with upstream too.

**Mutation stays refused**, as written above, and now at every link rather than
only the outer one.

Three boundaries this ticket added that 05 did not name, each measured rather
than argued:

- **A number written into the source refuses.** 05 §6 reported
  `(5).toFixed(2)` folding to `"5.00"` as coverage arriving. It is not: upstream
  *throws* there, because it applies the method without a receiver. The rule is
  narrower than "a numeric receiver" — `(-5).toFixed(1)` is a unary expression
  rather than a literal and folds in both, and so does
  `[1,2].indexOf(2).toFixed(1)`, a number a fold produced. Only the written
  literal refuses, which is exactly what the spec's non-goal describes.
- **A length no declaration could use refuses.** 05 §8 called a size guard a
  precondition of shipping, and it is. A per-call bound alone is multiplied by a
  chain, so `"x".repeat(1000000).repeat(1000000)` needs the second half of the
  rule: an amplifying call on a receiver that is itself a call refuses whatever
  the counts are. A bounded string can still become one array element per code
  unit, so a folded array has a length bound too.
- **Nesting past the bound refuses.** Not in 05 at all, and the worst of the
  three: a hundred levels of nested array literal overflows the engine's parser
  and aborts the process from inside an evaluation whose contract is that it may
  fail. That is issue 02's defect family reached one layer further down, and the
  bound in front of the engine is what turns it back into a refusal.

Verified against `@stylexjs/babel-plugin` through the existing corpus rather
than through a harness of its own — 12 entries in `corpus/modules.json`, run by
`pnpm parity`. Six are byte-identical on class name and declaration text: the
string surface, the array surface, chains, the object prototype, the numeric
edges, and `normalize`. Six are divergences, each pinned with its reason, so a
later change cannot move one unnoticed: four acceptance divergences where this
compiler refuses and upstream folds (mutation, locale, amplified length,
nesting), one both-reject with different wording (the written number), and one
class-name divergence (the unpaired surrogate, unchanged from what this ticket
predicted). `pnpm parity --set modules` reports 0 unexpected and 0 changed.

05's throwaway harness is deleted rather than kept: the corpus asks the same
question on every pull request, which a script nobody runs does not.

The remaining review findings from 05 are addressed here: the module is
`engine_fold`, which names the concept rather than the vendor; the two
overlapping walks are one walk parameterised by what a bare identifier may read,
which also widened both correctly; and parameters are `Atom` rather than
`String`.

### What this did not do

**The unification bullet is not done, and is not moot.** This ticket asked to
"unify the `EvaluateResultValue::Vec` and `Expr::Array` arms onto one dispatch
**before** adding any method", and the engine went in front of both arms instead
of replacing either. For a receiver written as a literal that is enough — the
fold answers before dispatch is reached, and the cited example
`["a","b"].map(x => x).join("-")` now folds. For a receiver reached through a
binding it is not: `V.map(x => x + "px").join(" ")` still fails on the chained
`join`, which is that same example one resolution step away.

The reason it was not simply folded in here is that the two arms disagree about
the *shape* of `context` and not only about which names they accept, so
accepting `join` in the second arm turns a deopt into a panic inside
`evaluate_join`. Reconciling that means one shape agreed across `evaluate_map`,
`evaluate_filter`, `evaluate_join` and both arms. It is the same complaint one
level down and it deserves its own change, with the option of deleting both
tables rather than aligning them.

Filed as 12, with the measurement of what a bound receiver gets today and the
two ways to build it. So what shipped is the literal-receiver surface this
ticket's own scope sentence names, and the residue is tracked rather than
implied.

**Three divergences this added, beyond the two the ticket predicted**, each with
a corpus entry rather than only a test: the length bound applies to a folded
array as well as a string; a count is bounded only if it is written as a number,
so a computed count refuses even when it is small; and `toLocaleString` is
refused, which costs one of the six agreed object methods, because the name
carries no locale on an object and formats on a number and the receiver's kind
is not knowable before evaluating it. The "the exclusion list was right" line
above is true of the other three names and was too broad about this one.
