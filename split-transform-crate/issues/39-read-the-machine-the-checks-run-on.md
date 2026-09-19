# 39 — Read the machine the checks run on

**What to build:** Three checks assumed one machine and the CI matrix has
several: two operating systems and two Node versions. Each failed on a runner
that was doing nothing wrong.

- The prototype sweep held a fixed floor for the number of engine methods it
  expects. Node 22 has no `Math.f16round`, so the count recorded on one Node
  fails on the other. The floor moves with the engine instead.
- The Rust-source scanner and the harvest check compared bytes. Git for Windows
  checks text out as CRLF, so a Windows checkout reported a file as stale while
  it held exactly what the generator writes.
- The dual-load guard compared binding paths as plain strings. Windows folds
  case, `realpath` may answer with a `\\?\` long-path prefix, and either reader
  may spell a separator the other way, so the process's own addon read as a
  second one — the reading the guard exists to prevent.

Filed after the fact. The work landed as `5414dbc61`, `65d34a225`, `c2ef2bc0f`,
`05caf0dd8` and `4b79119fa`, none with a ticket.

**Blocked by:** None.

**Status:** resolved

- [x] The sweep floor is derived from the engine it runs on, not recorded
- [x] The Rust-source scanner and the harvest check read content, not bytes
- [x] A binding path has one spelling, and one function settles it
- [x] `.gitattributes` asks for LF in every working tree, and no check relies
      on that rule alone
- [x] Each of the three has cases for the machine it failed on
- [x] A path is settled by one function, in the product and in the tests
      alike. The Windows binding job failed five cases on both Node versions
      because a test used `fs.realpathSync`, which leaves an 8.3 short name
      alone, while the code uses `fs.realpathSync.native`, which expands it

## Comments

**One ticket for three fixes** because they are one fault: a check that reads
the machine it was written on rather than the machine it runs on. Splitting
them would file three tickets whose whole content is that sentence.

**A fourth site was found later.** `stylex-utils` kept a bare `diff`, so the
CRLF repair reached two of three checks. Closed in ticket 41, which also makes
`scripts/git/lib/generated-fixtures.mjs` state the rule so the next check
cannot be written without it.

**A fifth site, found in CI rather than in review.** Run 33896665125 failed
`Test bindings on x86_64-pc-windows-msvc` on node@22 and node@24, and nowhere
else. The five failures all read
`- C:\Users\RUNNER~1\... / + C:\Users\runneradmin\...`: two realpath
functions, one of which expands a Windows short name and one of which does not.
`realPathOf` now lives in `benchmark/lib/paths.ts` and both sides call it.

**The same reading was wrong in the product.** `loadedNativeBindings` keyed the
raw `NAPI_RS_NATIVE_LIBRARY_PATH` against a report path the runtime had already
resolved. `bindingPathKey` cannot follow a symbolic link, and every temp
directory on macOS is reached through one, so the override was dropped and the
guard returned early on the dual load it exists to stop -- on the one platform
where that load ends the process. Found by asking the review where else two
spellings of one path could still meet, and pinned by a case that fails without
the fix.
