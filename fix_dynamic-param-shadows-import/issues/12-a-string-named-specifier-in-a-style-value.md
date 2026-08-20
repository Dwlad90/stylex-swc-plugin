# 12 — A string-named import specifier read in a style value

Status: `needs-triage`
Blocked by: 07 — the fallback 07 deletes is what makes this reachable by the
aliased-away name, so the shape has to be re-measured after that lands.

**What to build:** A verdict on what a string-named theme import should do in a
style value.

```js
import { "color-lg" as colorLg } from 'vars.stylex.js';
export const styles = stylex.create({ wrapper: { color: colorLg } });
```

The reference implementation refuses with `A style value can only contain an
array, string or number.` We emit **nothing at all** — no rule, no refusal. A
property that silently declares nothing is the worst of the three outcomes: the
markup names a class the stylesheet does not define and nothing errors.

Measured while implementing 02, with no shadowing anywhere in the module. Found
because 02 needed to know whether the imported-name fallback was still live, and
the string-named arm is the half of it that is. Not in the spec's audit table.

Two things to separate, which is why this is its own ticket rather than a note on
07: whether the *lookup* should resolve `colorLg` at all, and why the emit path
answers with silence instead of either a rule or a refusal. 07 deletes the
fallback that resolves the aliased-away name `color-lg`; it does not explain the
silence for the local name.

- [ ] The empty result is traced to the code that produces it, not inferred
- [ ] A verdict: mirror the refusal, emit the rule, or state why silence is right
- [ ] Corpus entry with the verdict it reads
- [ ] Re-measured after 07, so the two changes are not confused for each other

## Comments

07 landed, and the re-measurement it was blocking on is half done: reading the
specifier by the name it was aliased *away* from no longer aborts, and refuses
the way the reference implementation refuses. That was a separate fault sharing
one lookup with this one.

This ticket's shape is untouched. `color: colorLg`, read by the *local* binding
`colorLg`, resolved through the local match before 07 and resolves through it
now -- 07 deleted only the imported-name arms. The empty result still stands, and
tracing it to the code that produces it is what remains.
