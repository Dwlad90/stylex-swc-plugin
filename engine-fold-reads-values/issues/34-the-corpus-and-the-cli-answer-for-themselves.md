# 34 — The corpus and the CLI answer for themselves

**What to build:** The parity harness refuses input it cannot act on, so a
mistyped corpus row or surface name fails loudly instead of quietly running
something else.

**A corpus row can borrow a prototype key.** `guards.ts` validates with
`found in VERDICTS` and `found in CONFIGURATION_OPTIONS`, and `in` walks
`Object.prototype`. So `expected: "toString"` and `configuration: "constructor"`
both pass validation and are cast to their branded types. The loader then hands
`stanceOf` a verdict that matches nothing, and the row reads as permanently
`changed` — or a bogus configuration prints. `Object.hasOwn` is the fix; the two
casts become sound once it is.

**The CLI has no test and takes a typo.** `harness-cli.ts` is imported by no test
file. `selectedOrExit` filters members by `selected.includes(nameOf(m))`, which
succeeds on a partial match: `--surface Math --surface Strnig` runs one surface
and exits zero. The doc comment claims a typo is refused; only an all-miss is.
`writeJsonReport` and `answerOf` are untested too — and `answerOf` is the
function the file header says was extracted *because* a two-line refusal was
being joined wrong.

**Blocked by:** none — can start immediately.

**Status:** resolved

- [x] `Object.hasOwn` replaces both `in` checks, and `corpus.test.ts` carries a
      row with `expected: 'constructor'` and one with `configuration: 'toString'`
- [x] `isRecord`, `stringAt` and `arrayAt` are tested directly rather than only
      through the loader
- [x] An unknown surface name alongside a known one exits non-zero
- [x] `writeJsonReport` resolves a relative target against the package rather
      than the working directory, creates nested directories, and ends its file
      with a newline — each pinned
- [x] `answerOf` joins a two-line refusal with ` / ` and an acceptance with
      ` | `

## Answer

**Membership in a closed table is asked with `Object.hasOwn`.** `in` walked
`Object.prototype`, so `expected: 'constructor'` and `configuration: 'toString'`
passed validation and were branded -- the first read as a verdict matching no
stance, so the row printed as permanently `changed`, and the second printed a
setting nobody can raise. Both casts after those checks are sound now rather than
merely conventional. `guards.test.ts` reaches `isRecord`, `stringAt` and
`arrayAt` directly, over the input the loader never hands them: a wrong type, an
absent key, an empty string that is not absence, and a prototype name arriving as
a value rather than as a key -- which is the shape the corpus actually carries,
and which the guard passes through for the table to refuse.

**Every name in a selection has to match.** `selectedOrExit` asked only whether
*something* matched, which left the hole one name wide: `--surface Math --surface
Strnig` ran one surface and exited zero, so a mistyped sweep read as a pass. The
unknown names are now named back, the known ones listed beside them, and the run
exits 1. `harness-cli.test.ts` covers the flag surface the file had no test for
at all -- an empty table, a name differing only in case, a name that is a prefix
of a known one, a repeated name -- plus the report writer's three claims (package
resolution, nested directories, trailing newline) and `answerOf` on refusals of
one, two and three lines. `answerOf` splits either line ending, so a carriage
return cannot ride into a report cell as a control character.

**The throwaway directory moved to `support.ts`.** Three suites had written the
same make-it, remember-it, remove-it shape; `temporaryDir` is the one copy, and
cleanup stays per test.
