# 19 — A callback reached through a name

**What to build:** Naming a callback stops changing whether the call on it
folds.

```js
const upper = (part) => part.toUpperCase();

export const styles = stylex.create({
  x: { content: ['b', 'a'].map(upper).join(', ') },
});
```

Refused here, folded upstream. The same arrow written out in place folds —
ticket 10 prints it and the engine runs it — so this is the receiver question
this whole effort answered, asked one argument along: a value the guard resolved
becomes a parameter of the printed arrow and its value an argument to it, and a
function has no value form to carry across that bridge. The refusal is the
guard's general one, `Its receiver or one of its arguments is not in a form the
compiler can evaluate.`, so an author is not told which argument stopped it
either.

It costs a build the reference compiler completes, which is the direction the
parity harness treats as an obligation, and it is not a shape an author has to
go looking for: a named callback is how anyone writes the same transform twice.

**Found by the generated prototype sweep** (ticket 15), which reaches it through
`Object.groupBy` on named arguments — the one place the sweep names a *function*,
since a prototype subject names only its receiver. Nothing curated had named the
shape, which is the argument the sweep was built on. Pinned as
`modules-15-a-callback-reached-through-a-name` in the parity corpus, recorded as
a gap rather than as a refusal anyone wants, so the day it closes is a changed
verdict rather than silence.

**Not the same as 17.** Ticket 17 is a callback whose *body* names a StyleX
function, where nothing about the callback itself is unreachable — the function
map has no value the engine could hold. Here the callback is unreachable before
its body matters. Whoever takes both should decide whether one mechanism answers
them: running the callback outside the engine, which 17 argues for, would also
answer a named arrow if the arrow can be found.

**Blocked by:** none.

**Status:** resolved

- [x] An array method whose callback is a named arrow folds to the reference
      compiler's declaration text and class name
- [x] The same on a namespace static that takes a callback — `Object.groupBy`,
      `Array.from` — since that is where the sweep reaches it
- [x] A callback the guard still cannot reach names the argument that stopped
      it, rather than the call
- [x] Which bindings qualify is decided by what the name resolves to, and
      written down: a function declaration, a `let` reassigned later, and a
      parameter are three different answers
- [x] `modules-15-a-callback-reached-through-a-name` flips to agreement and the
      account in `parity/lib/prototype-accounts.ts` that names it goes away

## Comments

**Ticket 17 landed, and it does not answer this one.** 17 closed by handing the
engine `firstThatWorks` itself, as a native function of the engine's own built
over the same ordering core the evaluator's expression path calls — not by
running the callback outside the engine, which is the mechanism 17's body argued
for and this ticket's note asked about. That mechanism reaches a function the
*compiler* owns, whose Rust body is there to be handed over. A named arrow the
author wrote has no such body, so answering this shape still means finding the
arrow the name resolves to and printing it into the transport.

So the fourth category 16 is waiting on has not disappeared, and this ticket's
checklist is unchanged. What did narrow is the corpus row: its note now says the
neighbouring shape closed, so `modules-15-a-callback-reached-through-a-name` is
only about a callback the guard cannot reach at all.

## How it landed

**A function crosses as its declaration, not as a value.** `Transport` used to
hold one `Carried` per name; it now holds a `Crossing`, which is either that
value or the source a function was declared from. A value stays an argument, so
the printed text is the size of the expression however large the value is. A
function is printed as the *default* of the parameter its name became, and
nothing is passed for it -- so `['b','a'].map(upper)` reaches the engine as
`(upper = (part) => part.toUpperCase()) => ['b','a'].map(upper)` called with one
`undefined`.

**Printing it as a default rather than substituting it** is what keeps shadowing
the language's answer instead of the walk's. A callback parameter spelled like
the name shadows the default exactly as it shadowed the module binding, and a
substitution would have had to work that out for itself. Two tests hold that
from both sides: `a_callback_parameter_shadows_the_name_it_repeats` and
`a_declarations_parameter_shadows_a_carried_name`.

**Order is the one thing the parameter list has to get right.** A default is
evaluated where the parameter stands, so a name the declaration reads must
already be a parameter by then. `admit_a_named_function` therefore walks the
declaration *before* recording the name, which makes an alias chain come out in
dependency order for free -- `a_chain_of_aliases_folds` is four links deep.

**Which bindings qualify is the resolution's answer, and it is upstream's set.**
The guard asks the evaluator, and the evaluator answers a callback only for an
arrow with plain parameters and a single expression body that nothing wrote to.
Measured against `@stylexjs/babel-plugin` 0.19.0, that is exactly the set that
compiles there: a block body, a destructured, defaulted or rest parameter, a
`function` of either spelling, and a reassigned or mutated binding all refuse on
both sides. Eight tests pin the eight refusals.

**A refused one names the binding.** `unfoldable_function` is the new sentence,
and it is keyed on the name rather than on the method for the reason
`bound_value_too_large` is: the call is fine and the method is not what an
author has to change. It fires only where the module declares the name as a
function, so an ordinary unresolved name is still handed back to the dispatch
below rather than claimed.

**Nothing had to be added for the statics**, which is the other half of criterion
two: `Object.groupBy` and `Array.from` reach their callback through
`admit_argument` like any other method, so they folded the moment the identifier
arm did.

**Depth bounds a walk that goes round.** The declaration is walked one level in
rather than at a restarted budget, so a chain long enough to reach the configured
depth refuses there instead of running out of stack --
`an_alias_chain_past_the_configured_depth_refuses` walks two hundred links.
Mutual recursion never reaches that: the resolution refuses the first name for
being read above its own declaration, as upstream does.

**One gap was measured and is not this ticket's.** A callback *passed* by name
now folds; a function *called* through a name still does not, because
`admit_call` admits only a member callee or an unshadowed global. That is one
line of dispatch rather than anything about the bridge, and admitting it changes
what the fold claims at the top level -- where a dynamic style's own mappers
live. Filed as issue 22 and pinned as `modules-19-a-call-reached-through-a-name`,
so ticket 16's category count loses one and gains one.

**The three answers, written down.** A binding the module declares as a function
and the transport cannot take is *refused*, naming the binding. A name the module
declares nothing for — a dynamic style's own parameter above all — is *handed
back*, because what it holds is not knowable at compile time and refusing it
would fail a build over a call that was always going to run at runtime. Measured
on 0.19.0, that third case agrees exactly, down to the class name and the custom
property: `a_parameter_holding_the_callback_is_left_for_the_runtime`. The
`Named callback` glossary entry says the same in two sentences.

**One diagnostic outside a callback moved, and moving it was the point.**
`Object(value)` where `value` names an arrow used to refuse before the call with
`Only static values can be passed to Object().`; it now crosses, `Object` hands
the function straight back, and it ends where the arrow written out in place
already ended — `Cannot carry a folded function back from the engine.` Both
compilers refuse both spellings, so the change is this compiler agreeing with
itself. Pinned at `global_builtin_calls::object_of_a_declared_function_is_rejected`.

**The account is gone with the row it argued.** `prototype-accounts.ts` no longer
carries `a callback reached through a name`, and the `evidence` field it was the
only user of went with it, along with the test that exercised the field through
that account.
