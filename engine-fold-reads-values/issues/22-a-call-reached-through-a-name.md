# 22 — A call reached through a name

**What to build:** Calling a function through a name folds, as passing one now
does.

```js
const inner = (y) => y + '!';
const make = (s) => (x) => x + s;

export const styles = stylex.create({
  s0: { content: ['b', 'a'].map((x) => inner(x)).join(',') },
  s1: { content: ['a'].map(make('!')).join('') },
  s2: { content: [inner].map((f) => f('q')).join('') },
});
```

All three are refused here and folded upstream. None of them is about reaching
the function: ticket 19 made `inner` cross as its declaration, and in `s2` the
callee is a name the *callback* binds, which the engine would resolve without
anything crossing at all. What refuses is one line of `admit_call`, which asks
that a callee be a member expression or an unshadowed global and hands back
anything else.

**Not the same as 19.** 19 is a callback *passed* by name, and its answer is the
transport: a function has no value form, so the declaration crosses instead.
Here the function is already reachable and the call on it is not admitted, so
the answer is the guard's own dispatch rather than the bridge.

**Why it is a ticket and not part of 19.** Admitting a bare-name callee changes
what the fold *claims*. `try_fold` runs on the outermost call too, so a name
admitted as a callee there would take `someUserFunction(x)` away from the
dispatch below — which is where a dynamic style's own mappers and the injected
function map are answered. Narrowing the admission to a callee inside a callback
scope would close every case above without touching the top level, and deciding
whether that narrowing is the rule or a hedge is the work.

**Found while building 19**, in the sweep that measured which named shapes still
refuse. Pinned as `modules-19-a-call-reached-through-a-name`.

**Blocked by:** none.

**Status:** resolved

- [x] A callback body calling a module-named function folds and agrees with
      upstream
- [x] A callback calling a name the callback itself binds folds — the shape that
      needs nothing to cross
- [x] A call whose callee is itself a fold — `make('!')` returning an arrow —
      folds
- [x] Whether a bare-name callee is admitted outside a callback is decided and
      written down, measured against a dynamic style's own parameters and the
      injected function map
- [x] `modules-19-a-call-reached-through-a-name` flips to agreement

## Answer

**The narrowing is not callback scope, and measuring is what said so.** This
ticket proposed admitting a bare-name callee only inside a callback body, and
that would have closed one of the three reported shapes rather than all three:
`['a'].map(make('!'))` reaches `make('!')` through the *arguments* of `map`,
which are walked in the scope the call was written in — the module's. The line
that separates the calls the fold may own from the calls it may not is not scope
at all, it is **position**: the outermost call the caller asked about, against a
call nested inside an expression the fold has already claimed.

**And that line already existed.** `admit_a_stylex_function` draws it in the same
words — "A call written on its own never reaches here at all: the fold walks a
value, and the outermost call stays the dispatch's. That is deliberate — it
resolves its arguments this compiler's own way, a theme reference included, and
the engine holds no value for one of those." So the rule is the rule and not a
hedge: the same sentence answers a StyleX function and the author's own, and one
`Position` parameter is all that was added to say it.

**Measured against the two things the checklist named.** The dispatch below the
fold already answers `content: inner('a')` with upstream's own rule —
`.x1bt3ucs{content:"a!"}` — so admitting the outermost call would have replaced a
working answer with a narrower one, not gained a fold. A dynamic style's own
parameter is the other half: `base: (c) => ({ content: [inner(c)].join('') })`
resolves nothing for `c`, so the whole expression is handed back and the call
survives into the output for the runtime, which is the behaviour a build depends
on. The injected function map is untouched, because `a_stylex_function` is asked
about before a callee's name is. Both are asserted rather than described —
`the_outermost_named_call_is_answered_below_the_fold` and
`a_named_call_on_a_dynamic_parameter_is_left_for_the_runtime`.

**Position is a parameter, not something the guard carries.** Every other rule on
this bridge reads the call in front of it and nothing else, which is what lets a
static, a middle link of a chain and the outermost call be answered alike. Making
position a field of `Guard` would have offered that invariant to every rule; as an
argument to `admit_call` it is offered to one. The `Fold guard` glossary entry now
says which one.

**The declaration a call reaches runs once, and that had to be said.** An arrow
the guard walks is measured as a callback — once per element of a receiver — and
a *called* declaration is not one: it runs as often as the expression around it.
Left alone, `const big = () => 'ab'.repeat(20)` refused with a sentence about a
callback's unreadable element count, on a call with no callback anywhere near it.
The `Callback` the guard carries now says which of the two positions measured it,
and carries a count without a width for the called one, since a parameter there
holds an argument rather than an element.

**A callee that is itself a call is still not admitted, and that is agreement.**
The third criterion above reads "a call whose callee is itself a fold", and it is
met in the spelling the ticket wrote — `['a'].map(make('!'))`, where the fold
answers the arrow and `map` runs it. The other reading, a callee that literally
*is* a call — `make('!')('a')` — is refused here and upstream both, so nothing
was owed. `a_callee_that_is_a_call_is_not_a_candidate` pins it.

**One divergence the change did not close, and it was already refused before
it.** A length on a called *parameter* — `const big = (x) => x.repeat(20)` —
still refuses, because the parameter holds an argument and an argument's width is
not something this reading measures. Upstream folds it. That is the unreadable
amplification length ticket 12 recorded, reached by a third route, and the
refusal it now gets names the rule instead of the general sentence it used to
get. Pinned as `a_length_on_a_called_parameter_refuses`.

**One pre-existing gap closed on the way past, because a test claimed it was
already closed.** `unshadowed_global` read a name with `as_ident` and so did not
see through parentheses, while the new admission does — which made
`[(String)('a')].join('')` refuse beside `[(inner)('a')].join('')` folding, and
made this ticket's own comment about reading parens "exactly as every other
position on this bridge does" false. Both compilers fold both spellings and the
receiver position too, so the reading was widened rather than the comment
softened. `a_parenthesised_global_callee_folds` and
`a_parenthesised_global_receiver_folds`.

**Two names were weighed and left alone.** `Callback` now carries what two
positions measured, and only one of them is a callback — the argument for
renaming it is real, and it is a glossary term this effort's ticket 16 owns, so
the doc says which position is which and the name waits for the record.
`admit_a_named_call` takes the callee expression *and* the identifier inside it,
which are derivable from each other; deriving one inside would buy a parameter at
the cost of an unreachable branch, which is the trade the `Admitted::Named` arm
already makes once and does not need to make twice.

**Two rows flipped that this ticket did not list.** `two_declarations_naming
_each_other_refuse` now names the binding it could not reach rather than the
method, which is the sentence issue 19 wanted for it and could not have while the
call was handed back. And `a_local_function_of_the_same_name_is_not_this_one` —
a module function spelled `firstThatWorks` — now folds to upstream's own class
name, since it is an ordinary declaration reached through a name and never needed
the import's carriage at all.

**One gap was measured and is not this ticket's.** An outermost named call whose
*argument* is another named call — `content: inner(inner('a'))` — fails in the
dispatch below the fold with `Left expression is not a number: Identifier`, which
names neither the call nor anything an author wrote. It fails identically before
this change, and wrapping the same expression in anything the fold claims folds
it to upstream's `xjczvju`. That is the dispatch's own resolution of a user
function, in another file, so it is filed as issue 24 rather than answered here.
