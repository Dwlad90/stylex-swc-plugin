# Triage Labels

The skills speak in terms of five canonical triage roles. This repo uses the
default vocabulary, so **each label string is identical to its role name** --
there is nothing to translate.

Because the tracker is local markdown (see
[issue-tracker.md](./issue-tracker.md)), a label is the value of the `Status:`
line near the top of an issue file. There is no label API to call.

| Label             | Meaning                                  |
| ----------------- | ---------------------------------------- |
| `needs-triage`    | Maintainer needs to evaluate this issue  |
| `needs-info`      | Waiting on reporter for more information |
| `ready-for-agent` | Fully specified, ready for an AFK agent  |
| `ready-for-human` | Requires human implementation            |
| `wontfix`         | Will not be actioned                     |

Six labels beyond the five are in use. None is a canonical role, so a skill
asking for an AFK-ready ticket picks up none of them:

| Label         | Meaning                                               |
| ------------- | ----------------------------------------------------- |
| `backlog`     | Accepted and specified, deliberately not scheduled    |
| `in-review`   | Built and verified, waiting on a review to sign off   |
| `resolved`    | Closed: the work landed, or the question is answered  |
| `done`        | An older spelling of `resolved`, and the same thing   |
| `open`        | Filed and not yet triaged, so read it as needs-triage |
| `triage/perf` | One ticket only. Read it as `needs-triage`            |

`backlog` parks work that is ready to do but should not be done next, which is
the point of it.

`in-review` and `resolved` are the two states a ticket reaches **after** an
agent has built it, so neither is work to pick up. A ticket leaves `in-review`
for `resolved` when the review it waits on reports, and it may not sit in
`in-review` with a criterion still open -- an open criterion is work, and work
belongs in one of the five roles above.

`resolved` and `done` are one state under two names. Write `resolved` in a new
ticket. `done` stays in the table because a large minority of closed tickets
still carry it and rewriting them would change no meaning.

`open` and `triage/perf` are neither roles nor closed states, and nothing reads
them. Do not write either in a new ticket; the five roles above cover what they
were reaching for.

When a skill names a role (e.g. "apply the AFK-ready triage label"), write the
matching string above into the file's `Status:` line.

## Write the Status line one way

The line itself is spelled two ways across the tracker, and this matters more
than which label is on it:

```markdown
**Status:** resolved
Status: `resolved`
```

Both are in use. A skill that greps one spelling reads a tracker with the other
one as having no status at all, and a tally taken through one spelling reports
a fraction of the tickets as though it were all of them.

Write the first form -- `**Status:** <label>`, bold key, bare label, no
backticks -- because it is what most of the tracker already carries. Do not
rewrite the existing files to match; read both, and write one.

A reader that has to count or scan every ticket must accept both:

```sh
find -L .scratch -name node_modules -prune -o -name '*.md' \
  -exec grep -hoE '^(\*\*)?Status:(\*\*)? *`?[a-z/-]+`?$' {} +
```

Three details, and each one silently answers the wrong thing if it is dropped:

- `-L`, because `.scratch` is a symlink into the `scratch` worktree and `find`
  does not follow one by default. Without it the scan reads no ticket at all.
- `-name node_modules -prune`, because a built package under
  `.scratch/bench-base/` carries the ADRs of every crate it holds, and an ADR
  has a `Status:` line too. Its vocabulary is `accepted` and `rejected`, which
  is a different vocabulary from this document, and 16 of them read as tickets
  if they are not pruned.
- `find -exec` rather than `grep -r --include`, because `grep` on a developer
  machine here may be `ugrep`, which does not accept `--include`.

## A closed ticket is not a ticket for a human

`ready-for-human` means the work itself needs a person to build it. It does
**not** mean "built, but a person should read a decision recorded in the
comments" -- a decision that is written down is recorded, not pending, and a
ticket that carries one is `resolved`.

The distinction matters because it is the one a skill reads. A directory of
closed tickets mislabelled `ready-for-human` tells a skill scanning for work
that a person is blocking on every one of them, and that no ticket is ready for
an agent. Both readings are wrong, and neither is visible without opening the
files.

## If this repo's vocabulary ever diverges

Add a second column mapping each canonical role above to the string you
actually use, and the skills will apply yours instead. Until then, a mapping
column would just repeat the first one.
