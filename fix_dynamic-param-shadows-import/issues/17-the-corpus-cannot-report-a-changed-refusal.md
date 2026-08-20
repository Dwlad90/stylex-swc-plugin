# 17 — The corpus cannot report a changed refusal

Status: `needs-triage`
Blocked by: None

**What was found:** `both-reject` is 28 of 84 subjects in the `modules` set, and
it compares acceptance only. Two compilers that refuse the same input for
opposite reasons, with different words, read as agreement -- so a whole class of
parity regression is invisible to the harness.

`lib/compare.ts:201` decides it before any comparison runs:

```ts
if (rust.status === 'error' && babelOutcome.status === 'error') return 'both-reject';
```

The message is already captured (`{ status: 'error', message: messageOf(error) }`
at `:146`) and printed in the report. It is simply not part of the verdict.

Found while closing [08](./08-reject-a-folded-map-as-a-namespace.md), whose whole
subject is a refusal whose *wording* changed:

- before, `Style value must evaluate to a static expression.`
- after, `Invalid pseudo or at-rule.` -- the reference implementation's text

Both read `both-reject`, so the corpus row could not report the fix and cannot
report its regression. The wording is pinned in
`validation_stylex_create_test::invalid_values` instead, which works but splits
the question across two suites: the corpus owns *whether* the compilers agree and
a Rust test owns *what they say*. Several rows added under 08 say so in their
notes -- entries that hold an input rather than measuring one.

Why it was probably written this way: the two compilers prefix differently.
`[StyleX] a > color > Invalid pseudo or at-rule.` against
`/path/to/value.js: Invalid pseudo or at-rule.` A naive comparison marks nearly
every one of the 28 rows divergent, which is worse than saying nothing.

So the work is the normalization, not the comparison: strip each compiler's
prefix down to the sentence both share, and give the verdict a
`both-reject-divergent` alongside `both-reject`. Neither side's prefix is stable
enough to hard-code -- ours carries the evaluator's key path, theirs an absolute
file path -- so the rule has to be derived and pinned by tests of its own.

- [ ] A refusal whose wording differs no longer reads as agreement
- [ ] The 28 existing `both-reject` rows are re-measured and each one's verdict
      recorded, so the new verdict lands as a decision rather than as 28 failures
- [ ] The normalization has its own tests, including a refusal with no prefix and
      one whose message contains a colon
- [ ] The notes that apologise for the gap are removed from the rows that carry
      them
