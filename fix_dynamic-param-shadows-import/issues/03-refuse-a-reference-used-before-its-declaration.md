# 03 — Refuse a reference used before its declaration

Status: `ready-for-agent`
Blocked by: 02 — a textual gate only: both edit the identifier path in the same
function. The two are logically independent and could land in either order.

**What to build:** A style value that reads a binding declared *later* in the
module fails the build, instead of quietly compiling to CSS the reference
implementation refuses.

```js
export const styles = stylex.create({ a: { color: c } });
const c = 'red';
```

The reference implementation refuses this with `Referenced value is used before
declaration.` We emit `.x…{color:red}`. This is the only case in the whole
identifier audit where the divergence is *wrong output* rather than a differing
refusal, which is why it is its own ticket: a bisect deserves to land on it.

The cause is that declarations are collected module-wide with no position check,
so an initializer is inlined at a use site that precedes it. The reference
implementation compares the reference's start offset against the binding's end
offset. Mirror that with the parser's byte positions, ahead of the initializer
read.

**Skip the check when either span is dummy.** A synthesized node — shorthand
expansion produces them — has no authored position, so any comparison against it
is meaningless. Skipping degrades to today's behaviour for those nodes rather
than inventing a refusal.

The message text is already written and commented out beside the other
evaluation errors, byte-identical to the reference implementation's output;
revive it rather than composing a new string.

- [ ] The example above fails with the reference implementation's exact text
- [ ] A binding declared *before* the `create()` call still inlines
- [ ] A synthesized node with no authored position is unaffected
- [ ] Corpus entry with the verdict it is known to read
- [ ] A unit test covering the dummy-span skip, which the corpus cannot reach
