# 24 — An argument that is itself a call through a name

**What to build:** The dispatch below the fold answers a call to the author's
own function when one of its arguments is another such call.

```js
const inner = (y) => y + '!';
const other = (y) => y + '?';

export const styles = stylex.create({
  s0: { content: inner(inner('a')) },
  s1: { content: inner(other('a')) },
});
```

Both fail here with `Left expression is not a number: Identifier`, raised from
`transform_bin_expr_to_number` in `stylex-ast`'s convertors — a sentence that
names neither the call, nor the argument, nor anything an author wrote. Upstream
folds `s0` to `xjczvju` and the same shape one name along.

**It is the dispatch's, not the fold's.** The outermost call through a name is
handed back to `nodes::call_expression::evaluate` on purpose, and issue 22
measured why: that path resolves a name this compiler's own way, where a dynamic
style's own parameters and the injected function map are answered, and it already
folds `content: inner('a')` to upstream's rule. What it does not do is evaluate
an argument that is itself one of those calls. Every other argument shape works —
`inner('a'.toUpperCase())` and `inner(['a'].join(''))` both fold — and so does
the identical expression once anything the fold claims wraps it:
`[inner(inner('a'))].join('')` folds to `xjczvju`, because then the whole
expression is the fold's and the engine runs both calls.

So the fix is in the evaluator's own resolution of a user function, and the
message is half of what is owed: a refusal here has to name the call rather than
report an internal expectation about a binary operand.

**Found while building 22**, in the sweep that measured which named shapes still
refuse after the callee admission landed. It fails identically before that change,
so it is a gap of its own rather than a consequence of it.

**Blocked by:** none.

**Status:** resolved

- [x] `inner(inner('a'))` folds and agrees with upstream, class name included
- [x] The same through two different names, so the answer is not about one
      binding being reached twice
- [x] An argument that is a named call *beside* one that is not, since only one
      of the two is the shape that fails
- [x] Whatever still refuses names the call, not a binary operand's type
- [x] The corpus records the shape, so the day it closes is a changed verdict

## Answer

**The defect was where the call was applied, not how the argument evaluated.**
`call_expression::evaluate` answered a call through a name by handing back the
*arrow the name holds*, and `object_expression` applied it — with the arguments
of whatever call expression happened to sit in that value position. So the
answer depended on who asked. A style value ran the callback and folded
`inner('a')`; the same call one argument deeper reached the arrow as a
`Callback`, which has no expression form, so the parameter bound nothing and the
arrow handed its body back unevaluated. `y + '!'` then reached
`flatten_raw_style_object`, which tried to read it as a number and reported
`Left expression is not a number: Identifier`.

**The fix applies the call where it is written**, and the second half of the
rule — deciding a binding by the value's *expression form* rather than by its
variant — closed a second shape nobody had filed: an array argument bound
nothing for the same reason, so `inner(['a','b'])` failed identically where
upstream folds `a,b!`.

`object_expression`'s arm that applied a callback is deleted rather than left
beside the new one: a call no longer arrives there as a callback, and two places
applying the same function is what let them disagree.

**Measured against `@stylexjs/babel-plugin` 0.19.0**, every shape agreeing to
the class name: `inner(inner('a'))` → `xjczvju`, `inner(other('a'))` →
`xavg6lb`, `join2(inner('a'), 'b')` → `xzbf3kh`, `inner(inner('a') + 'z')` →
`xwfcpxd`, `inner(inner(inner('a')))` → `xg35vm4`, `inner(['a','b'])` and
`inner(arr)` → `x15vkifo`, `inner({})` → `x1gaedpw`, `inner(undefined)` →
`xbnrqzp`.

**What still refuses names the call.** A *function* argument —
`inner(inner)` — is the one argument with no expression form left, and it now
reads `Function argument must be a static expression.` Upstream folds it to the
function's own source text, which this compiler does not retain, so it is the
existing no-source-text divergence rather than a new one. Every other refusal
keeps its own sentence: a block-bodied declaration, a locale-sensitive method
inside the argument, a name the module never bound, a spread, and nesting past
the evaluation depth.

Tests are a section of `named_calls.rs`; the corpus row is
`modules-24-an-argument-that-is-a-named-call`, recording `identical`.

**Two things were widened deliberately rather than left as the same hole one
path over.** The expression-form rule is the *binding's*, so it fixes an array
argument at the same time as a nested call — `inner(['a','b'])` bound nothing for
exactly the reason `inner(inner('a'))` did, and upstream folds it to `a,b!`. And
the guard that turns an unbindable argument into a sentence is shared with the
one other site that applies an author's arrow, the callback held as an object
property value, because a rule written twice is how two call sites come to
disagree.
