# 22 — Settle the two CSS-affecting key corrections

**What to build:** Two corrections to how object keys are read landed or are
owed in the same reader, and both can change emitted CSS.

The first shipped inside a commit presented as a pure move: a digits-only
guard now stops a key like `'+0'` being treated as an array index, because
Rust's integer parser accepts a leading plus where JavaScript does not. That
changes own-key enumeration order, hence declaration order, hence which of two
equal-specificity rules wins. The direction is toward the reference
implementation and matches the function's own doc comment, so it is a latent
bug fix — but no fixture ever covered the old behaviour and nothing records
that it changed.

The second is still outstanding: numeric style keys are spelled with Rust's
formatting rather than JavaScript's, so a key like `1e21` renders as a long
digit string instead of the JS spelling. A sibling reader in the same crate
deliberately does this correctly and carries a comment naming the hazard. The
one existing test picks a value where both spellings agree, so its name claims
more than it asserts.

Land both deliberately, under one snapshot review.

**Blocked by:** 21

**Status:** resolved

- [x] Either the digits-only array-index guard is reverted and re-landed as
      its own commit, or it stays with an amendment recorded in the spec —
      it stays; the amendment sits under *Out of Scope* in `spec.md`
- [x] Numeric and computed-numeric prop keys use the JS spelling, matching the
      sibling reader that already does — three readers needed it, not one; see
      the Comments
- [x] A test covers a value where the two spellings differ, via both the plain
      and the computed key form
- [x] The existing numeric-key test's name no longer claims more than it
      asserts — it is now `reads_an_integer_number_key`
- [~] The pull request description states that property ordering can change
      for keys like `'+0'`, and that numeric key spelling changes — the text
      is drafted in the Comments below; it is copied into the pull request
      when the branch is opened. It needs a third paragraph from
      [ticket 32](./32-namespace-map-keeps-source-order.md), which applies the
      same ordering to the namespace names and so moves whole rule sets rather
      than single declarations
- [x] The snapshot suite is re-run and every diff is either empty or a
      reviewed ordering or key change — no existing snapshot changed; five new
      ones record the two corrections end to end
- [x] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code
- [x] The addon is rebuilt and the JavaScript suite re-run — it exercises the
      built artifact rather than the Rust sources, so a green Rust run is not
      evidence on its own

## Comments

### Parity against the reference implementation

Each case was run through both compilers and compared:

| Source | Reference | This compiler |
| --- | --- | --- |
| `create({ 1e21: … })` | `"1e+21"` | `"1e+21"` |
| `create({ [1e21]: … })` | `"1e+21"` | `"1e+21"` |
| `create({ 1e-7: … })` | `"1e-7"` | `"1e-7"` |
| `create({ 42: … })` | `"42"` | `"42"` |
| `create({ root: { '+0': …, 0: … } })` | `0` before `+0` | `0` before `+0` |

The last row is the array-index guard at work, and it agrees.

### Text for the pull request description

> **Two key readings change, and both can change emitted CSS.**
>
> *Property order.* The array-index test in the own-key reader now accepts
> digits only, so a key such as `'+0'` is an ordinary string key rather than
> index `0`. That decides own-key enumeration order, hence declaration order,
> hence which of two rules at equal specificity wins. The guard shipped inside
> an earlier move commit; it is deliberate, it matches the reference
> implementation, and it is recorded in the spec.
>
> *Numeric key spelling.* A numeric property key, plain or computed, is now
> spelled the way JavaScript spells it. A key written `1e21` names the
> property `1e+21`, not the twenty-two digit run Rust prints. Class names are
> hashed from the key, so the hash changes with the spelling.

### Three readers, not one

The review found the same Rust spelling on two more paths, both of them live:

- `convert_key_value_to_str` in `stylex-ast` spelled a plain number key with
  Rust while its own computed arm, one line below, already went through
  `to_js_string`.
- `prop_name_to_string` in the compiler builds a real JavaScript object to hand
  to a callback, so a key there reaches user code with the wrong name.

Both now call `to_js_string`, and both carry a test. A comment in the sibling
reader named `1e21` as the output where the output is `1e+21`; corrected.

### Out of scope, found while measuring

At the **namespace** level — `create({ '+0': …, 0: … })` — the reference
implementation emits `0` first and this compiler keeps source order, because
own-key ordering is applied to the style object and not to the namespace map.
This predates the ticket and is untouched by it. Worth its own ticket.

Filed and fixed as [ticket 32](./32-namespace-map-keeps-source-order.md), which
also corrects the assumption above that no CSS depends on the namespace order.
