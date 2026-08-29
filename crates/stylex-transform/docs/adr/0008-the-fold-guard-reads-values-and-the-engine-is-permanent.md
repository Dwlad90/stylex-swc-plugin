# The fold guard reads values, and the engine is permanent

**Status:** accepted

Two decisions, recorded together because the second is only defensible once the
first is made. A method call is folded by running it in an embedded JavaScript
engine, and which calls qualify is decided by reading the _values_ an expression
resolves to rather than the _syntax_ it was written with.

The commit that introduced the engine describes itself as a throwaway not for
merging, and it is on the default branch. No throwaway code survives it — the
temporary comparison script was deleted by the commit that shipped the fold —
but that sentence was the only written statement about why a large embedded
JavaScript engine sits in the default branch's dependency graph, and it said the
opposite of what is true. This replaces it.

## The guard reads values

The first version asked whether an expression was _written out_: a receiver had
to be a literal in the source, so `['a','b'].join(',')` folded and

```js
const parts = ['a', 'b'];
transitionProperty: parts.join(','),
```

did not. Below that guard sat two tables of method names — one for strings, one
for arrays — which answered the calls the guard refused. The tables were the
problem the engine existed to remove, and the syntactic guard is what kept them
alive: every named receiver fell through to them, so the surface they had to
cover was the whole prototype surface, and the method nobody listed was the next
bug report. Measured at the time, thirty-five methods across two receiver shapes
plus three static surfaces folded on a literal receiver and refused on a named
one.

**The rule is now: an expression qualifies when every leaf of it resolves to a
value the bridge can carry.** A leaf qualifies when it is written into the
expression, bound by a callback around it, or a name the module resolves to such
a value. Nothing about the _shape_ of the expression enters into it, which is
what makes one rule answer a receiver, an argument, a chain link and a callback
body alike. The tables are gone, and with them `ArrayJS`, `StringJS` and the two
dispatch arms that disagreed with each other about the same array.

What it costs is that the guard can evaluate. A read taken to decide whether a
fold is _possible_ is not the subtree's answer — the dispatch below may still be
asked the same question in earnest — so such a read puts back the evaluation's
confidence and deopt and withholds its refusal from the per-file memo. That is
the [speculative read](../../CONTEXT.md), and it is the whole of the price. Every
refusal answerable from a name alone is applied before the walk begins, so only
an expression the guard intends to fold pays to have its names read.

**This closes a ticket filed against the earlier effort** — its issue 12, _A
receiver reached through a binding gets none of the prototype surface_, filed
deferred and never triaged. (The tracker it lives in is local to a checkout and is not committed, so
the title is quoted here rather than linked.) It proposed two ways to build it and named the second — fold on
the resolved receiver — as the one to price. That is what was built, with the one
change its own warning asked for: the resolved value crosses as a _transport
argument_ rather than being printed into the source, and is bounded on the way in
by the same [allocation ceiling](../../../stylex-structures/CONTEXT.md) that
bounds what comes back. The ticket is left as it is; this is the record that it
is closed.

## The engine is a permanent dependency

**What it costs.** Measured on `aarch64-apple-darwin` and `x86_64-apple-darwin`
when the engine landed: the published artifact grows 5.6–6.1 MiB, 58–60%, and the
lock file by 49 packages. The 49 is stable — of the 425 crates the compiler
resolves under `--all-features`, exactly 49 are reachable only through
`boa_engine`, `regress`, `time` and `toml_edit` among them. Runtime cost is
close to nothing for input that folds nothing: the engine is built on first use,
so a file with no foldable method call never creates one, and the paired
benchmark reported every ratio between 0.989 and 1.017 across sixty fixtures for
that reason. Cold start is ~240 µs and a warm fold ~3.4 µs.

**Why the trade is worth it.** A table is finite and a language is not. The
alternative to the engine is a hand-written implementation of
`String.prototype`, `Array.prototype`, `Object.prototype` and the `Math` and
`Object` statics that has to agree with the language on every edge — the
whitespace `ToNumber` trims, the way a nested array flattens through a join, the
radix prefixes `f64::from_str` rejects — and has to keep agreeing as the language
grows. Every one of those is a wrong value rather than a missing one when it
drifts, and a wrong value is hashed into a class name that no later build
reproduces. 5.6 MiB buys the language's own answer.

**Why it is vendored rather than taken from the registry.** Published
`boa_engine` 0.21.1 requires `icu_normalizer ~2.0.0` and `boa_parser` requires
`icu_properties ~2.0.0`; neither can coexist with the `~2.3.0` that
`icu_collator 2.3.1` needs, and the versions share a major, so Cargo has to pick
one and cannot. `vendor/boa` is that release with those two bounds relaxed to
`>= 2.0.0, < 3` and nothing else changed. Patching is sound here rather than a
workaround a published crate would apologise for: nothing in this repository
ships to crates.io, so no downstream Rust consumer can be handed a dependency
graph a `[patch]` section quietly rewrote. What ships is the compiled `.node`.
`vendor/boa/README.md` records the provenance and how to bump it.

**Why its instance is leaked per thread rather than dropped.** One engine per
thread, created on the first fold that needs it. It is held in `ManuallyDrop`,
and that is not a convenience. The engine's garbage collector lives in a
thread-local of its own, and the order two thread-locals are dropped in is not
defined; dropping the engine after the collector underflows a reference count,
and that panic runs inside a destructor, which aborts the process instead of
unwinding. Leaking one engine per thread at exit is the price of not aborting.
The [fold memo](../../CONTEXT.md) is leaked with it, because a compiled script
belongs to a realm and holds engine values.

**How many threads there are.** One leak per thread is only a bounded cost while
threads are, so how many there can be is worth stating. `transform` is this
compiler's single exported binding and it is synchronous: it takes `napi::Env`,
which is not `Send`, so it runs on the JavaScript thread that called it and
cannot be handed to libuv's pool. No package here spawns a thread of its own.
Every thread that ever folds is therefore one the host already had, and the leak
is bounded by how many of those the host runs rather than by how many files they
compile. This is the paragraph to revisit if the binding ever becomes an
`AsyncTask`, which would move folds onto a pool nobody here sizes.

**How long they live is the host's answer, not this compiler's.** A host that
retires and respawns a worker leaks one context, its interned sources and its
memo each time it does — `jest-worker` is the case in this repository's own
dependency graph that retires workers, though under its default child-process
mode a retired worker takes the whole process with it and frees everything. What
this compiler can say is the shape of the cost: it is per retired _thread_, not
per file, so it grows with how often a host churns its pool and not with the size
of the build. Nothing measured here suggests that is a cost worth a second engine
lifetime to avoid, and the alternative — dropping the engine — aborts the
process.

That every thread answers its own fold is observed rather than assumed:
`thread_isolation_tests` folds on eight threads at once, gives each of them an
answer only it may reach, and asks each engine's global object directly whether a
fold left a name behind.

Reuse across files is what makes the guard's boundaries load-bearing rather than
merely tidy: a fold that reached a prototype would be read by every later fold in
the build, so the escaping-read refusal is what keeps one shared engine safe.
Reuse also costs — the engine interns each distinct source it is handed and never
reclaims it, roughly half a kilobyte per distinct folded call site, which a real
corpus keeps in the low megabytes for the life of the process.

## The locale exception

Locale-sensitive methods are refused, and this is the one refusal in the effort
that was sanctioned in advance. Four measured reasons, each on its own
sufficient:

1. The engine's internationalisation feature reintroduces the exact dependency
   conflict that forced the vendoring: the vendored tree relaxes two version
   bounds, and the feature pins roughly eleven more against the line the rest of
   the workspace is on.
2. That feature carries no compiled locale data and the provider crate is not
   vendored either, so building a locale context would fail at runtime unless
   this project shipped a data blob of its own.
3. It would not fix the number-formatting method regardless — the engine ignores
   `toLocaleString`'s arguments unconditionally, with no feature gate at all.
4. With no locale argument the reference compiler takes the _host's_ default, so
   its own answer is machine-dependent, and folding it would pin one build
   machine's value into every stylesheet a project ships.

What the refusal costs is visible: the engine answers
`'i'.toLocaleUpperCase('tr')` as `I` where the language says `İ`, so folding
would emit a wrong declaration rather than no declaration. A refusal costs a
build that would have compiled; a wrong fold costs a stylesheet nobody can find
the fault in.

## Three kinds of refusal, and four categories

The comparison harness treats every recorded divergence alike — a row carrying a
written reason passes, whatever the reason says. A reader needs the distinction
the harness does not make, so it is written here. **A written reason is not the
same thing as a wanted refusal.**

- **Wanted.** Folding would be worse than refusing. The locale methods above; the
  escaping property read, where the harmless half cannot be admitted without
  admitting the read that reaches `Function`; the amplification lengths the guard
  will not read, where reading them is exactly what lets two allowed lengths
  multiply into one that is neither.
- **Configurable.** A ceiling refuses, and raising it folds the same source to
  the reference compiler's own value. Nine rows, across `maxFoldedCharacters`,
  `maxFoldedEntries` and `maxEvaluationDepth`. These are not divergences in
  behaviour; they are a number a project sets.
- **Held open.** A gap, with a reason recorded so the harness reports it once
  rather than nightly, and no argument that it should stay. One row.

Four categories survive in this effort, and they map onto those kinds:

1. **Locale-sensitive methods** — 2 rows, wanted. The reasons are above.
2. **The value bridge** — 4 rows, three wanted and one held open. The environment
   object, the folded namespace map and a function held inside an
   already-evaluated value are this compiler's own values rather than JavaScript
   ones, so there is nothing to carry inward and refusing is right: upstream
   folds `[object Object]`, which no stylesheet can use, and a refusal is loud
   where a wrong class name is silent. **The token group is the exception, and
   the spec sentence it contradicts is amended here.** The spec says a resolved
   theme reference crosses inward; it cannot under one carriage of the name,
   because the value that answers `String(group)` — the variable-group hash —
   cannot also answer `group.token`, and answering both needs a name for a
   subexpression the author never wrote. That is a transport this effort chose
   against, so the row stays open rather than being argued closed.
3. **An unreadable amplification length** — 2 rows, wanted. Not configurable: no
   value of either allocation option folds them, because the length arrives
   through a parameter or the count comes from a receiver that is itself a call.
4. **An escaping property read** — 1 row, wanted. `constructor`, `call`, `apply`
   and `bind` walk off the value that was written onto the language's function
   graph, where `Function` compiles a string into a body that answers differently
   on every build and can write to a prototype the next fold reads.

   **The rule applies to a read with no call around it**, which is the one part
   of it that changed after the row was written. `s.constructor.name` used to
   answer `Could not determine the property being accessed.` one property later,
   which names the syntax rather than the reason; and `s.constructor` used to
   fold to `undefined`, which is a quietly wrong value of exactly the kind the
   fold exists to prevent — a string's `constructor` is `String`. Both are now
   the escaping-property refusal, and so are the bare `call`, `apply` and `bind`
   reads that used to answer `undefined` beside them: the rule is about the four
   names rather than about one of them, and leaving three folding to a wrong
   value would be a table of one.

   It costs two divergences beyond the row. A read reaching the prototype now
   refuses where upstream folds it — `s.constructor.name` is `"String"` there —
   and so does a receiver carrying one of the names as an _own_ property, where
   upstream folds `({ constructor: 'red' }).constructor` to `red`. The guard
   already refused that same own-property read with a call on the end of it, so
   the second is a parting between the two compilers rather than between this
   compiler's two paths. Both go the safe way: a refusal stops a build where a
   wrong fold names a class the other build never defines.

   Four names and not five. `__proto__` reaches the same prototype and is left
   out, because the step after it is one of these four: `s.__proto__` alone
   holds nothing a stylesheet can use, and `s.__proto__.constructor` is refused
   exactly as `s.constructor` is, so the chain is cut either way.

**The spec's "one category" claim is therefore amended to four.** It was written
before the value bridge and the allocation bound existed as they now do, and it
is the count rather than the reasoning that was wrong: each of the four carries
its argument on the row, and the harness fails any such row that carries none.

Two further categories were counted during the effort and are gone, which is the
outcome the record preferred to documenting them. _A callback reached through a
name_ closed when a named callback crossed as the declaration it came from. _A
call reached through a name_ closed when the fold admitted a callee it already
owned the position of. Two more were measured and closed rather than counted: an
argument that is itself a call through a name, and the join a `ToNumber` reaches
its number through.

## A shadowed name is ruled on twice, in opposite directions

`String`, `Number`, `Object`, `Array` and `Math` are folded by being called
rather than by a table of conversions, so whose name it is has to be decided
before anything else. The answer differs by **position**, which is the one place
in this module where it does.

**A callee honours every binding.** A `const`, a hoisted `function`, a `class`,
an import — each is the module's own name, and the call is the author's own
function. Folding there is the only direction that _invents_ output: this
compiler would name a class hashed from a declaration the reference compiler
never wrote, and a build mixing the two would carry markup naming a class the
stylesheet does not define. A name that fails this test reaches the ordinary
reference chain below the fold, which refuses it with the sentence the reference
compiler uses for the same declaration — or, where the binding holds an arrow,
_calls_ it, as that compiler does.

**A receiver honours only a declarator.** A receiver carries no value across the
bridge: the printed source names it and the language answers, so a `function` or
a `class` of the same name changes nothing about the static that folds. Measured,
the reference compiler folds `Math.max(1, 2)` under `function Math() {}`, and
this compiler now does too — as both fold `Math.trunc(1.5)` under an import of
the name, which is the same rule read on the other binding kind with no
declarator. Only a declarator is read, because only a declarator holds a value
the static could have been meant to read — `const String = 'abc';
String.toUpperCase()` folds to `ABC` on both.

**Where a declarator holds an object, the two part, and this one refuses.**
`const Math = { trunc: () => 9 }; Math.trunc(1.5)` is `1px` in the reference
compiler, which reads the shadow's _name_ and the global's _method_ and so
answers for neither. That is a bug there rather than a rule to match. Refusing
is the safe direction — the call is left to the runtime, where a wrong fold
writes a wrong declaration — and it is safe here precisely because a receiver
fold cannot invent a class name the way a callee fold can. The row is pinned in
the module corpus as `acceptance-divergent`.

## Consequences

**Position is a parameter, not something the guard carries.** Every rule reads
the call in front of it and nothing else, so a static, a middle chain link and
the call the caller asked about are answered alike. One question is passed beside
the guard rather than carried in it: a callee written as a bare name is admitted
only where the call sits inside an expression the fold already claimed, because
the outermost call is the dispatch's. That is not a second rule — an applied
StyleX function draws the same line for the same reason, and a reader who finds
two rules will look for a difference that is not there.

**The dispatch below the fold is not vestigial.** It resolves a name this
compiler's own way: a dynamic style's own parameters, the injected function map
and a resolved theme reference are all answered there and hold no value the
engine could be handed. Measured, it already folds `inner('a')` to the rule the
reference compiler emits, so admitting the outermost call would replace a working
answer with a narrower one.

**Message text is not a parity obligation.** A refusal carries the rule that
refused it, in this compiler's own words. The comparison harness compares class
name, rule text and style-object shape rather than sentences, and the two
compilers' refusal wordings are compared only for whether they complain about the
same thing.

**Two neighbouring records answer for what this one does not.** The stack the
engine's parser runs on, and the ceiling that sizes it, are
[0004](./0004-the-fold-owns-its-own-ceiling-and-its-own-stack.md) — including
where the claim is made, which is around the parse and the evaluation and around
nothing else. And [0001](./0001-a-refused-fold-borrows-a-later-diagnostic.md) is
superseded by this one: the borrowing it recorded is gone because the applied
globals it named are now folded by being called, which is this decision rather
than that one.

**The corpus is the register, and it cannot grow quietly.** A row recording that
this compiler refuses where the reference compiler compiles fails the harness
unless a reason exists — on the row, or as a refusal family that claims it.
Whether the reason is a good one is a person's judgement and not a thing a
harness can hold, which is why the three kinds above are written down rather than
inferred from the fact that a reason exists at all.
