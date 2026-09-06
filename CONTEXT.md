# StyleX SWC Plugin

A Rust reimplementation of Facebook's StyleX CSS-in-JS compiler, published as
npm packages built from a Cargo workspace. Root glossary: cross-context terms
only — each crate and package is modelled separately, and
[CONTEXT-MAP.md](./CONTEXT-MAP.md) lists them one row apiece.

## Language

**Catalog**:
A named group of dependency version constants in `pnpm-workspace.yaml`,
referenced from a manifest as `catalog:<name>`. There is no default one. An
entry nothing references is removed on the next install.
_Avoid_: version group, alias, dependency set

**Peers catalog**:
The catalog of wide ranges accepted from consumers.
_Avoid_: peer group, external catalog

**Internal catalog**:
The catalog of our own `@stylexswc/*` packages, held at one exact version. See
[ADR 0001](./docs/adr/0001-internal-dependencies-live-in-a-catalog.md).
_Avoid_: workspace catalog, local catalog

**Manifest**:
A `package.json` the catalog convention governs — `.syncpackrc`'s `source`. Not
the same set as the pnpm workspace members.
_Avoid_: package file, package descriptor

**Bumper**:
`scripts/git/bump-version.mjs`, which owns the release version everywhere it is
declared.
_Avoid_: version script, release script
