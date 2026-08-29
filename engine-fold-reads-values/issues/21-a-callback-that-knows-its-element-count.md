# 21 — A callback that knows how many times it runs

**What to build:** The two amplification rules inside a callback bound a product
rather than refusing outright.

Both length rules give up in the same place and for the same reason. A callback
body is evaluated once per element of a receiver the guard never measured, so a
length written into the source bounds one evaluation and not the call — and the
answer today is a blanket refusal:

```js
// refused: 'repeat' inside a callback, whatever the count says
content: ['a', 'b'].map(x => x.repeat(3)).join('-')
// refused: a declared length inside a callback, whatever it declares
content: ['a', 'b'].map(x => Array(2).fill(x).join('')).join('-')
```

Upstream folds both. Each is a small array whose element count is written out
one call away, and the product — two elements times a length of two — is four,
which is four orders of magnitude inside either ceiling. The guard refuses
because it is not carrying the number, not because the number is unknown.

**The number is usually knowable.** `admit_call` walks the receiver before it
walks the arguments, so by the time `admit_arrow` descends into the callback the
receiver has been admitted and, where it is a literal array or a resolved name,
counted. What is missing is a place to put the count: `Scope` carries the names
a callback binds and nothing about the call it belongs to. Give it the count and
both rules become the arithmetic they already are one level up — receiver
elements times declared length, compared to the entry ceiling; receiver elements
times built characters, compared to the character ceiling.

A receiver with no readable element count keeps today's refusal, which is the
honest remainder rather than a hole: a count that cannot be read is what the
blanket rule was standing in for all along.

**Why it is a ticket and not part of 18.** 18 answered *which calls declare a
length*, and its rule is arithmetic on one call. This changes the shape of what
the guard carries — a field on `Scope`, threaded through `Guard::binding` and
read by two rules in two units — so it is a change to the guard's structure
rather than to its arithmetic, and it should be reviewed as one. 18 tightened
the entry rule to reach an unreadable length inside a callback for a measured
reason (sixty-eight seconds on
`[{length: 100000000}].map(x => Array.from(x).length)`), which widened what this
ticket would win back.

**Found while building 18.** Recorded there as the sharper rule the blunt one
stands in for.

**Blocked by:** none.

- [x] A callback over a receiver whose element count can be read bounds the
      product, so `['a','b'].map(x => x.repeat(3))` and
      `['a','b'].map(x => Array(2).fill(x))` fold and agree with upstream
- [x] The count lives in one place both rules read, rather than being worked out
      twice in two units
- [x] A receiver whose element count cannot be read keeps the refusal, and the
      sentence says which of the two it is
- [x] A nested callback multiplies rather than resets — `a.map(x =>
      b.map(y => …))` is the product of both receivers
- [x] The corpus rows 18 and 12 opened for the blanket refusals flip to
      agreement, or narrow to the unreadable remainder

**Status:** resolved

## Comments

### What was built

`Guard` carries two new fields. `repeats: Repeats` is how many times the
expression under the walk is evaluated — `Times(1)` at module scope, the product
of every enclosing receiver's element count inside a callback, and `Unmeasured`
where a receiver was not counted. `callback: Option<Callback>` is what the call
under the walk measured for a callback among its arguments; it is set by
`admit_call` for its own arguments and dropped everywhere else, so the arrow that
reads it is the one written inside the call that measured it. Both amplification
rules then read one number: `Repeats::counted` either hands back the factor or
raises the blanket refusal, which is why the sentence is written once.

**The width was the part the ticket's own headline needed.** `['a','b'].map(x =>
x.repeat(3))` needs `x`'s length, and `x` is a name no module resolves — so the
element count alone would not have folded the ticket's first example. The same
reading of the same receiver answers both: `Measured` carries the element count
and the characters the widest element renders to, and `Scope::Names` gained an
`Elements { named, characters }` recording which of a scope's names hold one.
Only the leading names — everything the callback's *first* parameter binds — carry
it, since a later parameter is the index or the receiver, and a name a block
declares holds whatever the body built. That last exclusion is what keeps the
bound sound: `const y = x + x` is twice an element wide and Boa grows it natively,
outside every ceiling this compiler owns.

`PER_ELEMENT_METHODS` is the ten names whose callback runs at most once per
element *and* is handed the element first. `sort` is out because a comparator runs
more often than its array is long; `reduce` and `reduceRight` are out because the
element is their second parameter, so a width read off the receiver would name the
accumulator. A name not listed leaves the callback unmeasured, so the list is safe
by default.

`admit_a_named_function` needed no change, which was the check that the seam was
in the right place: it walks the declaration with `..guard`, so `callback` rides
along and `admit_arrow` does the promotion — `const big = x => x.repeat(3);
['a','b'].map(big)` folds for the same reason the inline spelling does.

### The remainder, measured against upstream

Four things keep the refusal, each pinned:

1. **A receiver that is itself a call** — `'ab'.split('').map(…)`. The
   `modules-12` rule in the other unit.
2. **A method outside the counted set** — `sort`, `reduce`.
3. **A block declaration as the amplifying receiver** — no element's width bounds
   it.
4. **A declared length the guard cannot read** — unreadable in the *length*
   rather than in the repeats, so it is refused whatever the element count came
   to. This is what keeps `[{length: 100000000}].map(x => Array.from(x).length)`
   from spending 18's sixty-eight seconds.

Which refusal an author gets depends on which rule asks first, and that is worth
knowing: `repeat` reads its receiver's own length before the repeats, so on an
unmeasured receiver it names the length it could not read rather than the
callback. `padStart`, `padEnd` and both declared-length spellings need no
receiver, so they reach the blanket refusal — which is where the uncounted
repeats really are the whole of the reason. Both sentences are asserted.

### Two shapes upstream folds that are still refused, and were before

- `['a','b'].map(x => (x + y).repeat(2))` — the amplifying receiver is a binary
  expression rather than a name, so no width is read. Bounding a *grown* string
  inside a callback is 20's question one layer in, not this one's.
- `[1, 22].map(x => String(x).repeat(2))` — the receiver is a call. Same
  `modules-12` rule.

Neither is a regression; both were refused as "inside a callback" before.

### One tightening, deliberate

A named function passed to a per-element method whose receiver could *not* be
counted now refuses where it used to admit, because `admit_a_named_function` used
to reset the scope to the module and so lost the callback rule entirely. That was
a hole: `x.split('').map(f)` runs `f` once per code unit of a bounded string —
ten thousand times at the default — and a declared length inside `f` was compared
against the ceiling once. Nothing in the corpus or the suite depended on it.

### Numbers

Nineteen shapes were run through `@stylexjs/babel-plugin` 0.19.0 and this
compiler side by side; every folding case in the new file carries the class name
upstream produced. The two headline examples, the three amplifying spellings, all
ten counted methods, a named receiver, a named declaration, shadowing in both
directions, a destructured first parameter, an astral element, a two-hundred
element receiver and an empty one all agree.

Corpus: `modules-18-a-declared-length-inside-a-callback` flipped to agreement and
is kept as the subject; `modules-21-an-unmeasured-callback-receiver` pins the
remainder. `parity` reports `changed 0`, `unexpected 0`; the prototype sweep
reports `unexpected 0`.

The criterion named rows 18 **and 12**, and only 18 moved.
`modules-12-amplification-across-a-chain` is `'x'.repeat(1000).repeat(1000)` — a
chained *receiver*, never a callback — so no row 12 ever carried the blanket
refusal the criterion was about. Nothing is owed there; the ticket's own text says
so one paragraph up ("This is 12's distinction on the other ceiling"), and the
criterion read it as a row rather than as a rule.

`bench:revisions` + `bench:verdict` against HEAD, 10 rounds, seed 1, on the built
artifact:

```
  Feature - engine fold                    point=1.025 lower=1.015 upper=1.027 status=pass
  Feature - engine fold (dev)              point=0.997 lower=0.988 upper=1.006 status=pass
  Suite passed
```

Inside the ~1.11x warn trigger, and inside this machine's ~34% cross-run noise.
Run twice — an earlier pass on the pre-review build read 1.013 / 1.024, so the
two orderings bracket each other rather than showing a trend. The receiver
measurement is the only added work, it is skipped for a global receiver and for
every method outside the counted set, and where it does run it lands on a value
`admit_value` resolved a moment earlier — so the read is a memo hit and the walk
is one pass over elements the fold is about to evaluate anyway.

### What the review changed

Two review findings were acted on, and one was withdrawn.

**A soundness near-miss, closed by stating it rather than by guarding it.**
`measured_receiver` resolves the receiver through the evaluator, and the receiver
inside a callback may be a name the callback binds — a module
`const parts = ['q']` beside `big.map(parts => parts.map(…))` would count one
evaluation against ten thousand. I first wrote a 60-line conservative walk
refusing to resolve any expression reading a bound name. Then measured it:
removing the walk changed no test outcome, because
`StateManager::declaration_of` is keyed by the full SWC `Id` — symbol *and*
`SyntaxContext` — so the parameter and the module binding are different keys and
the parameter's holds no initializer. Hygiene is a structural guarantee, not luck,
and the walk was defending a case the resolver cannot produce. It is gone; what
replaced it is `module_value_of`, one home for the paren unwrapping and the
chained-receiver refusal both readers share, whose doc says which property of the
resolver this depends on. `a_receiver_the_callback_binds_is_never_counted_from_the_module`
pins it, and its own comment says plainly that it passed before this change too —
its job is to fail if hygiene stops holding.

**A real undercount, guarded.** The literal arm of `measured_receiver` counted
`elems.len()`, which a spread makes short. Refused there now rather than resting
on the evaluator refusing spreads a layer earlier.

**Standards.** Three markdown lines over the 80-column rule, rewrapped; the
repeats clause was copied verbatim into both refusal builders, extracted as
`per_element(built, repeats, unit)`; `Measured` collapsed into `Callback`, which
differed from it only in field names; the hole-and-spread reading of one array
element extracted as `rendered_element` so the two arms that had it cannot drift.
