# 32 — The namespace map keeps source order where the reference reorders it

**What to build:** Own-key ordering is applied to a style object but not to the
namespace map that `create` receives, so the two implementations disagree on
the order the namespaces reach the emitted object.

`create({ '+0': { color: 'red' }, 0: { color: 'blue' } })` emits `0` before
`+0` in the reference implementation, because it builds a real JavaScript
object and the language enumerates an array-index key first. This compiler
walks the properties in source order and emits `+0` first.

Found while measuring
[ticket 22](./22-settle-the-css-affecting-key-corrections.md), which proved the
same ordering **inside** a style object already agrees — `order_own_keys` runs
there, through the evaluator, and the array-index guard behaves. The namespace
map never reaches that reader.

No CSS changes: a namespace name is a key of the emitted object, not a
declaration. So this is an output-shape divergence rather than a style one, and
it is filed rather than fixed.

**Status:** needs-triage

- [ ] Decide whether the namespace map must enumerate the way the language
      does, or whether source order is acceptable because no CSS depends on it
- [ ] If it must, route the namespace properties through the same own-key
      ordering the style object uses, and add a test for an index-like
      namespace name
