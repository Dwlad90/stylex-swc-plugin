# 45 — Settle the review of the key, bench and script work

**What to build:** A two-axis review of everything since `c52ce2e96` — the
tickets 17, 21, 22, 24, 27, 31, 33, 36 and 42 to 44 work — reported findings on
both axes. The findings that survived verification are below. The ones that did
not are recorded at the bottom, so the same arguments are not run again.

**Blocked by:** None.

**Status:** resolved

- [x] Two commits had put a new item **between** a doc block and the item that
      block describes, so the new item took the text and the old item was left
      with none. `state_manager.rs` did it to `push_declaration`, which is the
      only writer that keeps the declarator list and its index in step;
      `rust-source.ts` did it to `collectRustTestFiles`
- [x] `push_declaration` no longer says the index beside the list is
      `pub(crate)`. The list is; the index has no modifier and is private, and
      the comment twenty lines below already said so
- [x] `SKIPPED_DIRECTORIES` gives a reason for each of its six names, not for
      one of six
- [x] `flamegraph.sh` sets `set -euo pipefail` and keeps its arguments as `"$@"`,
      as its two siblings do. The comment in `lib/crate.sh` beside the
      empty-array guard names all three callers
- [x] `flamegraph.sh` has a test suite. It had none
- [x] The three per-crate script suites share one fixture rather than three
      copies of it, and `guidelines/SCRIPTS.md` records where it lives
- [x] `benches` and `declared_bench_count` read the crate list through one
      function rather than each holding a copy of the read
- [x] The workspace gate is green in **debug** — never `--release`:
      `format:check`, `lint:check`, `lint:shell`, `typecheck`, `pnpm test` and
      `pnpm test:crates:workspace`, each run directly rather than piped into a
      pager, whose exit code would mask a failure. Re-run `typecheck` after
      committing, because the pre-commit hook rewrites code

## Comments

**The empty-argument case had no teeth, in any of the three suites.** Each suite
carried a case named for the bash 3.2 rule, and none could fail. The fault needs
bash 3.2, which macOS keeps at `/bin/bash`, and each suite started the script
with the first bash on the search path — bash 5.3 on this machine. The case now
runs under every bash the machine has. Held against a script that sets `-u` and
keeps `args=("$@")`, bash 3.2 fails it and bash 5.3 passes it, which is the
proof the case was missing.

**The shell options in `flamegraph.sh` are a guard, not a repair.** Cargo is the
last command the script runs, so its status already reached the caller. Nothing
was broken and no test can show otherwise. The options keep the status honest if
a second command ever joins it, and the value of the change is that all three
scripts now read the same.

**A comparison of two counts can pass while nothing is measured.**
`the_reader_finds_every_bench_the_manifests_declare` compares the benches found
against the benches declared. Both come from one directory, so both are zero for
a directory neither reader can find. It now asks first that the manifests
declare a bench at all.

**Findings not taken, with the reason.**

- *The new flamegraph suite and the multi-bash reader are scope creep.* The
  review asked only to share the fixture between two suites. Both stay. A script
  this work changed had no test at all, and the repository's own rule is that a
  reader which drops a suite in silence must be held to what it claims. The
  multi-bash reader is what turned three toothless cases into real ones; without
  it the shared fixture would have spread a case that cannot fail to a third
  suite.
- *`assert!(declared > 0)` is scope creep.* It stays for the same reason. The
  finding asked for one read instead of two; sharing the read is exactly what
  made the vacuous pass reachable from one place and therefore worth closing.
