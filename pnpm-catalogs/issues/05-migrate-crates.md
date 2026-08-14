# 05 — Migrate the crate manifests onto the catalogs

**What to build:** All 21 crate-level `package.json` manifests declare their
dependencies by catalog reference, and the Rust build, the NAPI binding and the
benchmark harness all still work.

Most of these are thin script-runner manifests carrying only internal
`@stylexswc/*` dependencies plus `@taplo/cli` and `npm-run-all2` — uniform and
mechanical. One is not: `stylex-rs-compiler` carries the real dependency set,
including the `@swc/core` dev/peer split that motivates the `peers` catalog.

Its `__test__` and `benchmark` sub-manifests sit outside the old syncpack glob,
so ticket 01 brought them into scope for the first time — but both are
`{"version", "type"}` and nothing more. They declare no dependencies at all, so
there is nothing in them to migrate.

The Rust side itself needs nothing. `[workspace.dependencies]` is already fully
adopted and crates consume it via `dep.workspace = true`, which is the same
pattern catalogs bring to the npm side. Do not touch `Cargo.toml`.

Watch the dev/peer split in `stylex-rs-compiler`: `@swc/core` is a
`devDependency` at a narrow range and a `peerDependency` at a wide one. Those
resolve to two different catalogs, and collapsing them to one silently narrows
what consumers are allowed to bring.

Do not touch the `@napi-rs/cli`-generated platform manifests under
`crates/stylex-rs-compiler/npm/`, or any fixture manifest — fixtures are
resolved as if they were real user projects, so a catalog specifier in one makes
it test something users never have.

This batch is independently green; unmigrated manifests keep literal ranges.

**Blocked by:** 03

**Status:** resolved

- [x] All 21 crate manifests reference catalogs, `__test__` and `benchmark`
      included — vacuously for those two, which declare no dependencies
- [x] The `@swc/core` dev/peer split is preserved as two distinct catalog
      references
- [x] Generated platform manifests and all fixture manifests are untouched
- [x] The lockfile resolves to the same versions as before the change
- [x] Rust build, NAPI binding, Rust and Node test suites, and the benchmark
      harness all pass
- [x] `syncpack lint` passes at all three call sites


## Comments

114 literal ranges across 19 manifests became `catalog:` references — 61
`internal`, 41 `tooling`, 4 `types`, 3 `runtime`, 2 `testing`, 2 `bundlers`, 1
`peers`. Every one already matched its catalogued range exactly, so the
assignment was mechanical and the batch is provably a refactor: the lockfile
diff carries zero removed lines and zero changed `version:` lines. Every added
line is a new entry in the lockfile's own `catalogs:` block.

The shape the ticket predicted held. 18 of the 19 are the uniform thin
manifests, drawing on `internal` and `tooling` only; `stylex-rs-compiler` alone
exercises all seven catalogs. `__test__` and `benchmark` needed no edit.

**The `peers` catalog gets its first exercise here.** `stylex-rs-compiler`
declares `@swc/core` as a `devDependency` at `^1.15.43` and a `peerDependency`
at `^1`; those became `catalog:bundlers` and `catalog:peers` respectively. This
is the first batch where one package legitimately references two catalogs, so
it is the first real test of the pairing rule — and of syncpack's ignore-
everything version group, which exists precisely so this does not read as
drift.

**One addition that is not a migration.** `@emnapi/runtime@2.0.0-alpha.3` joins
the `tooling` catalog. Ticket 04 added it to `crates/stylex-rs-compiler` as a
literal range and deferred cataloguing it to this batch. It sits in `tooling`
rather than `runtime` despite the name: no code we ship imports it. It exists
only because `@napi-rs/cli` pins it as an *exact* optional peer, so it belongs
beside the CLI that pins it, and its range is exact for the same reason — a
caret would not satisfy the pin. The reasoning is recorded as a comment on the
entry, because the name invites exactly the wrong conclusion.

**Publication was verified rather than assumed**, one ticket earlier than
planned. `stylex-rs-compiler` is a public package, and this batch puts catalog
references in both its `dependencies` and its `peerDependencies` — so the
property ticket 06 exists to prove is already load-bearing here. Packing it
yields `picomatch: ^4.0.4` and `@swc/core: ^1`: no `catalog:` specifier
survives, and the dev/peer split reaches consumers exactly as before.

`pnpm peers check` reports the same five unmet peers before and after, verified
by stashing the change and reinstalling. None involve `@emnapi/runtime`,
`@napi-rs/cli` or `@swc/core`, which confirms ticket 04's `@emnapi/runtime` pin
survived the move into the catalog.

**Pre-existing sort violations surfaced.** The pre-commit manifest job fixed
key ordering in five files — `@stylexswc/enums` out of alphabetical order in
three manifests, and root-key order in `atoms` and `transform`. These are
inherited, not introduced; they are in this change only because the hook stages
what it fixes. A parsed-content diff confirms the hunks reorder keys and change
no value.
