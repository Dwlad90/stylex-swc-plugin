# 09 — Callable globals move to the engine

**What to build:** The four callable globals are evaluated rather than
hand-coerced, and the hand-written coercions shrink to what still needs them.

`String(x)`, `Number(x)`, `Array(n)` and `Object(x)` are native JavaScript
functions, so under this effort's rule they belong to the engine. Their
hand-written implementations — a full specification-following string
conversion and number conversion, plus the per-callee behaviour around them —
go away.

What does **not** go away is the coercion those same operations perform for
the `+` operator, template literals and the unary forms, which stay in Rust
because they are not calls. So two conversion implementations remain in the
workspace, used in disjoint positions. That is a deliberate consequence of
where the line was drawn, and the drift it invites is guarded by a
differential test rather than by hoping.

`Array(n)` produces holes and `Object(x)` produces an object, so this depends
on both crossing the bridge.

**Blocked by:** 06.

**Status:** resolved

- [x] The four callable globals fold through the engine, matching the reference
      compiler on the cases the existing suite already pins — including
      numeric-string conversion, whitespace, hexadecimal, the empty string, and
      the spellings JavaScript rejects
- [x] A locally-declared shadow of any of them is called, never folded
- [x] A global that is not callable still fails with a diagnostic naming the
      real problem rather than leaking an engine error — and the question is now
      asked of the language rather than of a table: the global object holds the
      value, and the value says whether it can be applied
- [x] The callable-global name table and its dispatch are deleted, and
      `CallbackType` with them: one variant was the global and the other the
      arrow, so what is left is the arrow itself
- [x] The hand-written coercions are reduced to what the operators, template
      literals and unary forms call — the array-length pair, the
      number-value reader and three of `ToObject`'s four outcomes are gone
- [x] A differential test asserts the remaining hand-written conversions agree
      with the engine across the existing input matrix

## Comments

### What the arguments cross as, and what does not cross

The four globals fold by being called, so an argument has to cross the value
bridge — and the fold owns every call to an unbound global, so an argument it
cannot carry is a refusal rather than a shape handed back. Nothing below the
fold folds one, so handing it on would end the call at the catch-all's
`Unsupported expression` with the reason lost.

Two things were added to the bridge to keep the pinned cases folding:

- `undefined`, `NaN` and `Infinity` are printed rather than carried, like a
  global receiver: the grammar has no literal for any of them, so an author
  writes a name and the engine holds the value. This also fixed
  `n.toFixed(undefined)`, which the reference compiler folds and this compiler
  refused.
- An object literal's spread is walked as a value and printed as written, so the
  language does the spreading.

And a number crossing back is now *spelled*: `NaN` and the infinities have no
numeric literal, so the emitter wrote `0 / 0` and a numeral no author wrote. A
class name is a hash of the declaration text, so the spelling is the value. That
was a latent defect in the outward bridge, reachable before this ticket by any
fold answering a non-finite number.

### The engine has no function source text

`String(fn)` is the one shape where folding faithfully is worse than refusing.
The reference compiler answers it with the source of a wrapper from inside its
own evaluator — an implementation artifact that gets hashed into a class name —
and this compiler would answer with its own minified printing of the arrow.
Neither is what the author wrote and the two never agree.

So the engine is built with `Function.prototype.toString` assigned a throw, and
every conversion that would read a function's source refuses. A function reached
only to be *called* is untouched, which is what
`String({ toString: () => 'red' })` needs — and that is the whole of the
difference between a function used as a value and one used as a method. One
statement, enforced by the language, in place of a rule the guard would have had
to spell.

The price is `Number(fn)` and `Number([fn])`, which the reference compiler folds
to `NaN` because it reaches a number *through* the string and no source text is
a numeric literal. Those now refuse. The divergence is in the safe direction and
is the one place the differential test records a deliberate disagreement: the
operators keep the `NaN`, because they coerce in Rust and never ask the engine.

### Divergences this ticket accepted

Each is a value that is this compiler's own rather than JavaScript's, so it has
no form the bridge carries and the call refuses where the reference compiler
folds. All are refusals, which is the safe direction — a refused build never
names a class another build does not define. Each carries a row in
`parity/corpus/modules.json` with its reason, as the spec's harness rule
requires; 14 re-pins them rather than discovering them.

- `String(stylex.env)` — the environment object
- `` `x${Object(stylex)}y` `` — the namespace map. The interpolation one line
  away still agrees, because a template coerces in Rust; the corpus entry
  records the pair
- `String({ ...base })` where `base` holds a function — the spread's operand is
  a name, so its value has to cross, and a function does not. The arrow written
  out in place still folds
- `Number(fn)` — the one place the two conversion implementations part company,
  recorded as such in the differential test

And one that is **not** sanctioned by the spec, flagged rather than buried. The
bridge section says *"the string a theme reference has already resolved to"*
crosses inward, and `String(colors.primary)` now refuses. It cannot be satisfied
under one carriage of the name: upstream folds `String(colors)` to the token
group's variable-group hash and `String(colors.primary)` to the token's
`var(…)`, and a value answering the first cannot be read through a property to
answer the second. Carrying the group as its hash makes `.primary` read off a
string; carrying it as an object of resolved strings loses the hash. Answering
both needs a name for a subexpression the author never wrote — a transport this
effort chose against, twice. `Object(colors)` is the same wall from the other
side: an object crossing back is a plain object literal, so the identity could
not preserve a token group even if it crossed inward. **16 owns deciding whether
the spec line or the refusal moves.**

Three verdicts moved the other way and are parity *gains*:
`n.toFixed(undefined)` and `Object.keys([x => x])` now fold and agree, and
`String(Object.getPrototypeOf({a: 1}))` folds to `[object Object]` as it does
upstream, because the prototype never crosses the bridge when the whole
expression is one fold.

`String('\uD800')` reaches a decision ticket 06 already took rather than making
a new one: the engine's strings are UTF-16 and `Lit::Str` carries UTF-8, so a
value crossing back substitutes the replacement character. Both compilers now
compile it and the class names differ, which is the one divergence here that is
*not* in the safe direction — so it has a corpus row of its own. Changing it is
a decision about the outward bridge for every fold, not about this call.

### Two bounds moved as a side effect, for 12

Deleting the `Array(n)` dispatch deleted its own length budget with it, so the
count a folded array may carry is the fold's entry bound rather than a bound of
its own: 10 000 where it was 65 536. That is a narrowing nobody asked for, and
12 owns re-deriving both defaults and stating each in terms of what it costs.

The nesting ceiling moved the same way. `Array(<deep arithmetic>)` used to be
walked by the evaluator's descent, which a project can raise; it is now walked
by the fold's guard, whose ceiling is the engine parser's stack and does not
move with that option. The two ceilings also count different things — nodes
rather than evaluation levels, and `(x + 1)` is two nodes — so the boundary is
fifteen additions where the bare arithmetic beside it reaches three hundred and
seventeen. The numbers being equal at the shipped default does not make them
interchangeable. The two depth cases in `evaluation_depth_budget` say so and
point at 11.

### Where the differential test lives, and why not beside the predicates

The spec asks for *"the existing matrix beside the predicates"* extended with
the differential pass. It cannot be: that matrix is in `stylex-js`, which does
not depend on the engine and must not, so the only crate that can compare the
two is one holding both. The matrix is therefore a second list in
`stylex-transform`, and the comment above it says a value added to the
coercions' own matrix has to be added here too. A generated cross of the two
would be better and belongs with the prototype sweep in 15.
