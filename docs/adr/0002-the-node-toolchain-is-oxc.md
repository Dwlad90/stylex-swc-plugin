# The Node toolchain is Oxc

**Status:** accepted

Oxlint replaces ESLint, Oxfmt replaces Prettier, and declaration emit goes
through Oxc rather than the TypeScript compiler API. Each is configured once at
the root and runs as one process; `packages/eslint-config` is deleted.

Three things in the tree contradict that sentence on first reading, and each is
deliberate.

**`eslint-plugin-*` dependencies are still installed.** They are not ESLint.
Oxlint loads `@stylexjs/eslint-plugin`, `eslint-plugin-storybook` and
`eslint-plugin-turbo` through its JS plugin bridge, for rules it has no native
equivalent for. `dependabot.yml` says so at both places they appear, because the
`@stylexjs/*` pattern in the `stylex-core` group also matches the StyleX plugin.

**An `eslint-disable` comment suppresses nothing.**
`respectEslintDisableDirectives` is off and `reportUnusedDisableDirectives` is
`deny`, so a stale or copy-pasted directive is a lint error rather than silently
working. Every directive in the tree is `oxlint-*`. A suppression that no longer
suppresses is a small lie about the code — it hides that the underlying problem
was fixed, and it survives rule renames. Enforcement is free while the tree is
clean, and the drift it prevents is what made the original directive rename a
38-file job.

**`// prettier-ignore` was renamed, not deleted.** The Next.js example apps use
it to hold hand-aligned token tables in `globalTokens.stylex.ts`; without it the
columns collapse. Oxfmt honours both spellings, so `// oxfmt-ignore` leaves the
output byte-identical and stops the comment from advertising a tool that is
gone.

## Considered options

**Stay on ESLint and Prettier.** Rejected on cost, not correctness: the two run
per-package where Oxlint runs once from the root. That difference also fixed a
real gap — the old CI `--filter` arguments covered `packages/*` and `apps/*`
only, so root-level files were never linted at all.

**Migrate at parity and stop.** This is what the migration commits actually did,
and it was the right shape for review — the tool swap and the reformat stayed
separate diffs. It was not the right resting place: parity meant 19 explicitly
listed rules. Ratcheting to the `correctness` category took that to 235 errors
with the tree still clean, and added `unicorn`, `import` and `promise`, which
the ESLint config never had.

**Enable the remaining Oxlint categories.** Measured, then rejected. `style`
produces 25k findings that largely duplicate the formatter, `restriction`
produces 6.2k that ban ordinary modern JavaScript, and `nursery` is unstable by
definition. `suspicious` sits between: ~90 findings worth seeing that are not
defects, so it is `warn` rather than `error` — blocking it would have bought a
very large mechanical diff or a pile of suppressions.

## Consequences

**`pnpm lint:check` passing tells you nothing about the type-aware rules.** They
need `oxlint-tsgolint` and a prior build, so they live in `pnpm lint:type-aware`
and the fast check stays fast. Both must pass before TypeScript work is done;
`pnpm lint:all` runs the pair.

**Nothing may reintroduce the TypeScript compiler API.** `unplugin` builds with
`tsdown` under `isolatedDeclarations`, which is the only path on which tsdown
emits declarations through Oxc — TypeScript 7 does not ship that API. The CI
`dependency-graph` leg that would catch an old TypeScript copy creeping back in
behind a tool that still wants it is committed, but commented out.

**Oxfmt does not sort manifests.** `sortPackageJson` is off so it does not fight
syncpack over `package.json` ordering. See
[ADR 0001](./0001-internal-dependencies-live-in-a-catalog.md) for what else
governs those files.

**Taplo and rustfmt are untouched.** Oxc covers Node files; TOML and Rust keep
their own formatters, so `pnpm format:check` is three tools, not one.
