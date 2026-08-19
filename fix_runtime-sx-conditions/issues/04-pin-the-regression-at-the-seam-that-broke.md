# 04 — Pin both the reported input and the property behind it

Status: `resolved`
Phase: Phase 1

**Blocked by:** 02

**What to build:** Two kinds of coverage, because they catch different things.

**The property.** For each of `&&`, `||` and `??`, a right operand that cannot
be folded deopts and the build survives. This is what actually broke, it is the
test that would have caught `1322be8c1`, and it keeps holding as the evaluator
grows. Goes beside the existing
`nodes/tests/logical_expression_tests.rs`.

**The symptom.** The reporter's input, in the parity corpus so it is checked
against the reference implementation rather than a snapshot written by hand:

```jsx
const showAlternate =
  query.length > 0 && "documentation".startsWith(lowerQuery);
return <section sx={[styles.base, showAlternate && styles.alternate]} />;
```

and also the other input that was reported in the same issue:

```jsx
const isHView = VIEWS.includes(hView);
return <div sx={[styles.base, isHView && styles.hview]} />;
```

Error output: `error: [StyleX] The array method 'includes' is not yet supported in static evaluation.`

Expected: two rules emitted, `showAlternate` preserved as a runtime condition —
the output `0.18.3` and `0.18.4-rc.1` produce.

The symptom test alone would have passed at rc.1 and told us nothing about why.
The property test alone would not tell the reporter their case is covered. Both.

Note that tests importing `@stylexswc/rs-compiler` exercise `dist/*.node`, so
the crate must be rebuilt before the corpus leg means anything.

## Comments

### What was already there, and what this added

The first symptom (`startsWith`) and the three-operator property test landed
with issue 02, in `154931eca`. What remained was the second reported input, the
parity-corpus leg, and — added mid-ticket — the reporter's fuller module and the
other method shapes.

`VIEWS.includes(hView)` refuses from a different arm of the evaluator than
`startsWith` does, reached the same way and failing the same way. That is the
evidence for issue 02's diagnosis: the defect was never a missing method.

Seven shapes are pinned beside it, one component each, because the evaluator
refuses each from a different arm. Three of them (`join`, `Object.keys`,
`concat`) use methods that *do* fold, so they reach the refusal after a fold
rather than instead of one — the argument is unknown, the receiver came from a
call, the chain refuses only at its outer link. One component per shape keeps
the emitted lookup table linear; a single `sx` array with N conditions builds
2^N entries.

### The corpus leg needed the corpus to grow a second subject kind

A corpus entry was a property and a value, and the module both compilers see was
generated around it. That cannot express this question: whether an unfoldable
expression is refused or aborts is a fact about a module, and the evidence is
that both compilers reached the rules at all rather than that they spelled one
the same way. An entry is now either a declaration or a module, decided by
whether it carries a `source`, with the kind derived on load so the generated
`harvested.json` — and the value-parser cases generated from it — stay unchanged.

The comparison is still class names and rule text, never emitted JavaScript. The
two compilers print code differently, so comparing output would report a
divergence on every entry and say nothing about StyleX.

### The five module entries, and the one that disagrees

Four read `identical` against `@stylexjs/babel-plugin@0.19.0`: both reported
inputs, the shape table, and the reporter's fuller module below.

`modules-1265-through-a-binding` — the reporter's fuller module, verbatim — was
`structurally divergent` while `borderTop: none` was emitted here and dropped
upstream. Issue 07 fixed that, and the entry now reads `identical`: the exact
reported input is checked against the reference implementation after all.

`modules-1265-callback-through-a-binding` is `acceptance divergent` in this
compiler's favour. `VIEWS.some(v => v === q)`, reached through a binding, makes
upstream abort with `Unsupported expression: ObjectPattern`: it evaluates the
callback body, resolves `q` to its destructured parameter and throws from an
evaluation that is allowed to fail — the same defect #1265 reports here, still
present upstream. It aborts only through the binding; written inline in the `sx`
array it compiles there. File upstream rather than reproduce: a build that
survives is the correct answer, and matching the abort would re-introduce the bug
this effort is about.

### The expected-verdict field, and the limit that remains

A corpus entry now takes an optional `expected` naming the verdict it is known
to read. While it holds the report marks the entry `(expected)` and
`--only-mismatches` leaves it out; when it stops holding — in either direction —
the entry is listed under **Verdicts that changed**. A divergence that quietly
goes away is as loud as a new one, which is what caught the `borderTop` entry
above: it was still recorded as divergent after issue 07 had fixed it.

Six module entries carry one, and a changed verdict exits non-zero — the one
thing the harness fails on. A divergence with no expectation recorded against it
still prints as information for a person to read. The limit that remains is that
nothing runs `pnpm parity` for you: it is not wired into CI, and it reads
`dist/`, so a run means nothing without a rebuild first.

The module sources are written twice — once as a Rust test fixture, once in
`modules.json`. `harvest-corpus.ts` solves that for declarations; doing it for
modules is a bigger machine than this ticket warrants.
