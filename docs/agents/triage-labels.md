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

One label beyond the five is in use:

| Label     | Meaning                                            |
| --------- | -------------------------------------------------- |
| `backlog` | Accepted and specified, deliberately not scheduled |

It is not one of the canonical roles, so a skill asking for an AFK-ready ticket
will not pick it up. That is the point: it parks work that is ready to do but
should not be done next.

When a skill names a role (e.g. "apply the AFK-ready triage label"), write the
matching string above into the file's `Status:` line.

## If this repo's vocabulary ever diverges

Add a second column mapping each canonical role above to the string you
actually use, and the skills will apply yours instead. Until then, a mapping
column would just repeat the first one.
