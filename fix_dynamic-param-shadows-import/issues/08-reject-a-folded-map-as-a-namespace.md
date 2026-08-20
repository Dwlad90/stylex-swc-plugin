# 08 — Reject a folded map as a namespace

Status: `resolved`
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

- [x] The example fails with the reference implementation's exact text
- [x] `stylex.when(...)` as a callee still works, bare and namespaced
- [x] The abort at both style-value consumers is unreachable for a folded map
- [x] Corpus entry with the verdict it is known to read

## Resolution

`materialize_style_value` in
`shared/utils/core/evaluate_stylex_create_arg.rs` is the one reader of an
evaluated style value at both positions. It answers the expression a value
carries, and for a `FunctionConfigMap` it builds the object the map stands for
-- its keys, with `null` placeholders validation never reads. Both positions go
through it, so neither can abort on a folded map.

`FunctionConfigType::Map` and `EvaluateResultValue::FunctionConfigMap` were
changed from `FxHashMap` to `IndexMap`. The map stands for a JS object whose keys
are read in insertion order, and the object materialized from it decides which
key a message names -- a hashed order would make that arbitrary. Nothing keyed
the map by hash for a reason: the outer `functions.identifiers` lookup is
untouched and stays `FxHashMap`.

Measured on all four shapes of the reported input -- bare, aliased namespace,
beside a static prop, under a condition, inside a pseudo, in a shorthand -- and
each now reads `Invalid pseudo or at-rule.`, the reference implementation's text.
Asserted in `validation_stylex_create_test::invalid_values`, which is where a
message can be pinned: the corpus verdict `both-reject` compares acceptance and
not wording. That the corpus cannot report a refusal whose wording changed --
which is exactly what this ticket changed -- is its own gap, filed as
[17](./17-the-corpus-cannot-report-a-changed-refusal.md).

Three guards came with it, as snapshots rather than refusals: `when` read as a
callee off the shadowed parameter, off the unshadowed namespace, and off a bare
import. The first is why the map is materialized at the value position rather
than at the identifier seam.

### Found while measuring, not fixed here

- Every array style value inside a dynamic style's body aborts at the same
  consumer, folded map or not -- eleven shapes the reference implementation
  compiles. Owned by
  [14](./14-an-array-style-value-inside-a-dynamic-style.md), which is what the
  `height: [stylex, '1px']` corpus row waits on.
- A dynamic parameter shadowing a named import of a function-map entry emits
  CSS upstream refuses -- the entry is a `Regular` config with no key set, so the
  identifier step deopts and the value becomes an inline style. The one
  wrong-output case beside the fold, owned by
  [16](./16-a-shadowed-function-import-emits-css-upstream-refuses.md).
- Three further positions read the same fold and diverge in wording only -- a
  static namespace value, a `FunctionConfig` read off the map, and the fold
  coerced to a string. Owned by
  [15](./15-the-function-map-read-where-it-is-not-a-map.md).
- A theme object read as a style value with no member access is *dropped* --
  no rule, no error -- where the reference implementation refuses. The same
  consumer seam, reached by the shape this ticket did not audit, and the worst of
  the three because it changes output rather than wording. Owned by
  [18](./18-a-theme-object-read-as-a-style-value-is-dropped.md).
