# 11 — Decide what a namespace import of a theme file means

Status: `needs-triage`
Blocked by: 04 — it changes the same chain step, and landing both at once is how
the two refusals stay distinguishable.

**What to build:** A verdict, and then either a refusal or a recorded reason not
to refuse.

```js
import * as tokens from 'tokens.stylex.js';
export const styles = stylex.create({ wrapper: { color: tokens.color } });
```

The reference implementation refuses this with `Referenced constant is not
defined.` We accept it and emit `.x…{color:var(--xe7srj8)}` — the same rule a
named import of the same variable produces.

Measured while implementing 02, with no shadowing anywhere in the module, so this
is about the import kind and nothing else. Found because 02's audit put a
namespace import through both compilers for the first time; the spec's audit
table does not have this row.

Unlike the default-import case in 06, it is not obvious which side is right. The
reference implementation reads a namespace object as a value and finds no
constant behind it; we resolve the member through the theme file, which is what
an author writing `tokens.color` means. Refusing would break modules that compile
today. So the deliverable is the decision, argued, before any code.

- [ ] The two behaviours are stated with the exact message text, and the reason
      the reference implementation refuses is read out of `evaluate-path.js`
      rather than guessed
- [ ] A decision: mirror the refusal, or keep accepting and record why at the
      chain step, the way 06 asks for
- [ ] Corpus entry with the verdict it reads, either way
- [ ] If we keep accepting, the comment on
      `dynamic_param_shadows_a_namespace_theme_import` in `dynamic_styles.rs`
      stops calling it "not a parity claim" and cites this decision instead
