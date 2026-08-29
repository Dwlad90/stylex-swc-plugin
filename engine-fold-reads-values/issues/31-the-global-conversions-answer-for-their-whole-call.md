# 31 — The global conversions answer for their whole call

**What to build:** `String`, `Number`, `Object`, `Array` and `Math` answer for
the name that is actually in scope and for every argument they were handed.

**Three measured divergences, one module.**

*A function declaration does not shadow.* `const String = …` is honoured, but
this is not:

```js
function String(x) { return 'no'; }
color: String(1)     // upstream: refuses. here: folds to "1"
```

So does an imported one — `import { String } from './x'`. Spec story 29 asks for
exactly this: *"I want a locally-declared `String` to keep shadowing the global,
so that the compiler never folds a call to my own function."* Folding here is the
dangerous direction: this compiler produces output where the other produces none,
and the class name it produces is a hash of a declaration upstream never wrote.

*`Array` drops arguments.* `GlobalConversion::of` answers from `args.first()`, so
`Array(colors, 'blue')` becomes `[colors]` and the fallback is gone. The doc
comment's "surplus arguments are ignored, `String(1,2)` is `'1'`" is true for
three of the four conversions and false for the fourth, where every argument is
an element. Reached on the hand-back path — an argument the bridge does not
carry, such as a token group — which is why no test caught it.

*A shadowed `Math` receiver refuses where upstream folds.*
`const Math = { trunc: () => 9 }; Math.trunc(1.5)` folds to `1` upstream, which
ignores the shadow — an upstream bug — and refuses here. This is the safer
direction and may well be left alone; what it must not be is undecided. Rule on
it and write the ruling down, since it is the mirror image of the first finding
and a reader will ask why the two go opposite ways.

**Blocked by:** none — can start immediately.

**Status:** resolved

- [x] A `function` declaration and an import binding shadow the global as a
      `const` already does, for all four conversion names plus `Math` and `Object`
- [x] `Array(a, b, c)` keeps every argument; `Array(n)` with a single number
      keeps meaning a length
- [x] The shadowed-`Math`-receiver divergence is ruled on, and the ruling names
      why it differs from the shadowed-callee rule
- [x] The doc comment says what is true of each conversion rather than of three
      of them
- [x] Every shape is measured against `@stylexjs/babel-plugin` 0.19.0 and pinned
      in the corpus

## Answer

**One question became two, split by position.** `unshadowed_global` answered both
the callee of an applied global and the receiver of a static, and the ticket's
first and third findings are that those two want opposite rules. So there are now
two functions with the ruling written on each.

`unshadowed_applied_global` honours **every** binding — a `const`, a hoisted
`function`, a `class`, an import, a dynamic style's parameter — read through
`declares_binding`, the `Id`-keyed set the pre-scan already fills. Folding a
shadowed callee is the one direction that invents output: this build would name a
class hashed from a declaration the reference compiler never wrote. A name that
fails the test reaches the ordinary reference chain, which refuses it in
upstream's own sentence — measured, `Unsupported expression: FunctionDeclaration`,
`ClassDeclaration`, `Referenced value is not a constant.` and the imported-file
sentence all match byte for byte. Where the binding holds an arrow the call is
still *made*, on both compilers.

`unshadowed_receiver_global` keeps the declarator-only rule, and **the ruling the
ticket asked for is that this stays.** A receiver carries no value across the
bridge, so a `function Math() {}` changes nothing about the static that folds —
and upstream folds `Math.max(1, 2)` under exactly that declaration, which this
now does too. Where a declarator holds an object or a string, upstream reads the
shadow's name and the global's method and so answers for neither; refusing is the
safe direction, and it is safe *here* precisely because a receiver fold cannot
invent a class name the way a callee fold can. Pinned `acceptance-divergent` as
`modules-31-a-shadowed-static-receiver`, and both rulings are in
`ADR 0008`, section *A shadowed name is ruled on twice, in opposite directions*.

**`Array` keeps every argument.** `Conversion::of` took one argument and now
takes the list; it is one match on the conversion rather than two, so each arm
states both what it does with an argument and what it answers without one, and
`of_nothing` is gone. Measured, `Array(colors.primary, 'blue')` was
`color:var(--xa513j)` and is now `color:var(--xa513j);color:blue` — a style array
is a fallback list, so the dropped argument was a declaration the author wrote.

**`Array(n)` stays a length, and the reading stays the guard's — but a length
does reach the conversion behind the fold, and is refused there.** The first
answer written for this was that a number crosses the bridge, so the engine
always answers `Array(n)` and no length reading is duplicated. The review found
that false: the hand-back is decided by the *whole expression*, not by one
argument's carriage, so `Array([, 1].length)` declines for the array hole and the
number `2` arrived at `Conversion::of` as an element — `.length` answered `1`
where JavaScript says `2`. Reading the length there would allocate on a number no
ceiling had checked, and would be a second place deciding an allocation; so a
lone number is refused instead, with `unbounded_declared_length` — *"Write the
elements out, or keep the rest of the expression foldable so its length can be
checked."* The reference compiler refuses the same source for a reason of its
own, so both builds stop. Pinned `both-reject-divergent`. `Array(2).length`
still folds to `2` through the engine, `Array(colors, 1, 2, 3).length` to `4`,
and an argument that resolved to something other than a number is still the one
element both compilers build.

**The receiver rule reads no import either, and that is agreement.** The review
asked whether `import { Math } from './helpers'; Math.trunc(1.5)` should shadow.
Measured: the reference compiler folds it to `1px`, because it never resolves a
static's receiver at all — and so does this one. Pinned as its own row beside the
callee rule, where the identical import does shadow and both compilers refuse.

**Two divergences the edges turned up.** `(((String)))(1)` under a `function
String` now refuses where it used to fold, but at the catch-all rather than at
the declaration: the dispatch below reads a callee written as a bare name and a
parenthesised one falls past it. Both compilers refuse, so no output is invented,
and the sentence is pinned. The other is unrelated to this ticket and is filed as
46: a join over an array holding an array refuses on the hand-back path, with or
without `Array` in it.

**Where it is proved.** `transform_stylex_create_test/global_conversions_answer_for_their_call.rs`
carries twenty-four cases at the transform seam — both shadow positions on all
five names, every binding kind, the argument list at one, three and ten
arguments, nullish and `NaN` elements, a spread, the refused length, and the
two rulings.
`engine_fold/tests/shadowed_names_tests.rs` carries seven at the two rules
themselves, asserting each shape in **both** positions so a rule that has drifted
back into one fails. Five corpus rows measure it against the reference compiler
on every run.

Committed as `fix(stylex_transform): answer a global for the name in scope and
every argument`.
