# 11 — Cross-report both defects upstream

**What to build:** Two reports filed against the official compiler's
repository, one per defect, with their numbers recorded locally so that either
can be resolved without waiting on the other.

The first is the redundant wrapper: a ladder of exclusive breakpoints wrapped in
disjunctions of contradictory branches, semantically equivalent to the authored
queries, and rejected by lightningcss's minifier in its doubly parenthesised
form. The second is the collision that drops a declaration — the more serious of
the two, and worth saying plainly: redundant CSS is ugly, a missing style is
lost work. Both reports carry a minimal input, the observed output, and the
resolved reference version.

Filing is outward-facing and irreversible, so it does not happen without an
explicit go-ahead at the time. Do not modify the parent issue.

**Blocked by:** 07. Also requires explicit approval from the maintainer before
anything is filed.

**Status:** drafted, not filed — see `../upstream/`. The maintainer was asked
and chose to review the drafts and file them personally, so the two reports
exist locally and neither has been opened.

- [x] Explicit approval to file is obtained and recorded before either report is
      created. Asked; the answer was to draft and not file.
- [x] Two separate reports exist, one per defect, each with a minimal
      reproduction and the resolved reference version — as drafts in
      `../upstream/`, ready to paste.
- [x] The declaration-loss report states plainly that a declaration is silently
      lost.
- [ ] Both report numbers are recorded in this tracker directory. **Open until
      filed.** `../upstream/README.md` names the three places they go.
- [x] The parent issue is not modified or closed.
