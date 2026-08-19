# 08 — Reject a folded map as a namespace

Status: `ready-for-agent`
Blocked by: None — can start immediately

**What to build:** A dynamic style parameter that shadows the `stylex` namespace
import fails with `Invalid pseudo or at-rule.` — the reference implementation's
text for the same input.

```js
export const styles = stylex.create({ dyn: (stylex) => ({ height: stylex }) });
```

Both compilers refuse this, for the same reason and with different words. This
one is the reference implementation's *own* scope-blindness, not ours: it
registers the local `stylex` name in the name-keyed identifier map — checked
before scope — so the parameter folds to that object, and `height: { when: … }`
is then refused by namespace validation as a non-conditional key.

We reach the same fold and then abort at the style-value consumer, because our
function map has no expression form.

Mirror it at the **consumer** — the two style-value positions that abort when
the folded value has no expression form. Materialise the map there as an object
carrying its keys, so it reaches namespace validation and is refused by the
conditional-styles check with the text above. The placeholder values are never
emitted; validation refuses on the key first.

Not at the identifier seam. The map must keep its own form there, or
`stylex.when` as a callee stops working.

- [ ] The example fails with the reference implementation's exact text
- [ ] `stylex.when(...)` as a callee still works, bare and namespaced
- [ ] The abort at both style-value consumers is unreachable for a folded map
- [ ] Corpus entry with the verdict it is known to read
