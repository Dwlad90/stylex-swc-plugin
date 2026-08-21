# 17 — The corpus cannot report a changed refusal

Status: `resolved`
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

- [x] A refusal whose wording differs no longer reads as agreement
- [x] Every `both-reject` row is re-measured and each one's verdict recorded, so
      the new verdict lands as a decision rather than as a wall of failures
- [x] The normalization has its own tests, including a refusal with no prefix and
      one whose message contains a colon
- [x] The notes that apologise for the gap are removed from the rows that carry
      them

## Answer

`lib/refusal.ts` reduces a refusal to the complaint inside it, and
`verdictFor` compares the two complaints instead of only the two outcomes: equal
is `both-reject`, unequal is the new `both-reject-divergent`. The reduced
sentence is carried on the outcome as `sentence` beside the message as thrown —
the verdict compares it, the human report prints it, and `--json` keeps the raw
message as the evidence for both. Printing the sentence is a second win the
ticket did not ask for: an `acceptance divergent` row used to print upstream's
whole code frame into the report.

What is stripped is decoration in the strict sense — text that says *where*.
Neither side's is hard-coded: this compiler's is found by the marker it brands
every diagnostic with (`STYLEX_LOG_PREFIX`), and upstream's by the filename the
harness itself handed both compilers. Also off: the `-->` location line, the
`[Stack trace]` line, the repaired rule text a CSS refusal carries, upstream's
code frame, and SGR escapes.

**Each side's rules apply only to what that side wrote.** The marker decides
which branch a message takes, because a refusal carrying neither wrapper — a
bare `TypeError` out of either compiler, or an upstream complaint quoting author
CSS — otherwise loses text to a rule written for the other side. The rule text
comes off before the breadcrumbs are looked for, because a rejected rule is
arbitrary author CSS and can spell `a > b`; that ordering is what keeps a child
selector from eating its own complaint. The location matchers tolerate an indent
change rather than pinning the two spaces `StyleXError` writes today, so a
presentation change cannot turn every located row divergent.

One limit stands, and is pinned as a test rather than asserted in prose:
`Invalid media query: {query}` echoes the author's query, and a media range
condition is spelled `(width > 600px)`, so a branded complaint of that one shape
loses its head. Harmless in the direction that matters — a verdict only ever
compares one subject's two messages, so a mangled sentence reads *unequal*
against upstream's intact one, and `both-reject-divergent` is the truthful
verdict for two compilers wording a media-query refusal differently. A false
*agreement* would need the mangling to land exactly on upstream's sentence,
which is the failure the harness already had before it compared wording at all.

A sentence that reduces to nothing is not compared: two messageless throws would
otherwise read as agreement about a complaint neither compiler made, so there the
raw messages are what decide.

Twenty-five tests in `parity/__tests__/refusal.test.ts` pin every step, the two
shapes the ticket named (no prefix at all, a complaint containing a colon)
included.

The count in the ticket was stale: the set had grown to 95 `both-reject` rows
across all four corpus files. Re-measured, **90 agree on the sentence and 5 do
not**:

| row | here | upstream |
| --- | --- | --- |
| `…-a-string-named-specifier-with-a-lone-surrogate` | `String value contains invalid UTF-8 encoding.` | `An export name cannot include a lone surrogate…` |
| `…-the-namespace-import-read-as-a-static-value` | `A style value can only contain an array, string or number.` | `Invalid pseudo or at-rule.` |
| `…-a-named-function-map-import-read-as-a-static-value` | as above | as above |
| `…-param-shadows-a-named-default-marker-import` | `[UNIMPLEMENTED] IndexMap values are not supported in this context.` | `A style value can only contain an array, string or number.` |
| `…-a-shadowed-param-spread-into-a-style-object` | `Referenced constant is not defined.` | `Only static values are allowed inside of a create() call.` |

Each carries `expected: both-reject-divergent` now, so the report reads `changed
0`. Three of the five are the open divergences already owned by
[15](./15-the-function-map-read-where-it-is-not-a-map.md) and
[21](./21-a-shadowed-default-marker-param-reports-an-internal-shape.md) — those
tickets' subjects are visible in the corpus for the first time rather than only
in a Rust test. The other two are decided divergences whose rows said so in
prose and can now say so in a verdict.

Nothing in `edge` or `harvested` diverged, and the 30-odd unclosed-construct
rows there are the reason the rule text had to come off: this compiler answers
`Rule contains an unclosed function, css rule: * { width: calc(1px }` where
upstream answers the same sentence with a code frame. Two shapes of the same
"where", so they reduce to one complaint. None of those rows gained an
`expected`, deliberately: `expected` records a divergence someone has looked at,
not an agreement — `harvested.json` is generated and could not carry one anyway,
and writing 30 expectations of agreement would invert what the field means.
`both-reject-divergent` sits outside `AGREED`, so a future flip in either set
still prints as a mismatch for a person to read.

### One row was measuring the wrong call

`modules-1266-param-shadows-a-named-default-marker-import` carried a second
create call, written to keep the `defaultMarker` specifier alive, whose
`default: defaultMarker()` **both compilers refuse** — so the module refused
before the shadowed parameter was ever read, and the row measured that call
while its note quoted sentences from the subject. Comparing the wording is what
caught it: the note's `A style value can only contain an array, string or
number.` was nowhere in either measured sentence. The guard call is gone (a
shadowing parameter as the specifier's only occurrence keeps the specifier now,
which `…-referenced-nowhere-else` holds), and the row reproduces 21's table
exactly. 21 is annotated with the finding.

A mechanical audit of every refusal row against the sentences quoted in its own
note found no others: the three remaining mismatches all quote a message the row
records as historical (`used to read …`).

Report: `parity/results/t17.json`, measured against `@stylexjs/babel-plugin`
0.19.0.
