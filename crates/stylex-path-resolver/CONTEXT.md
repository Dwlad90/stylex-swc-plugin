# stylex-path-resolver

Turns an import specifier into a filesystem path, so the compiler can follow a
`defineVars` import into the file that declares it. Wraps `oxc_resolver` and
adds the cases bundlers introduce.

## Language

**Root path**:
The project root that module resolution is based on.
_Avoid_: cwd, base dir, project dir

**Root dir**:
The `unstable_module_resolution.rootDir` option. Distinct from the root path,
and used for one thing only: rewriting Turbopack's `/ROOT/` placeholder inside
an aliased path. With no root dir, a `/ROOT/` alias cannot resolve at all.
_Avoid_: root, source root

**Alias**:
A configured specifier rewrite, applied before module resolution. An alias that
expands to an absolute path or to `/ROOT/` is resolved straight against the
filesystem, because module resolution expects a relative or module-style
specifier.
_Avoid_: mapping, path mapping, shortcut

**pnpm path**:
The `node_modules/.pnpm/<name>@<version>/node_modules/<name>` location behind a
symlinked package. Resolution prefers it where it exists, so two versions of one
package stay distinct.
_Avoid_: real path, store path

**Extended package.json**:
`PackageJsonExtended` — a manifest read for resolution purposes, carrying the
name, the entry points and the dependency lists rather than the whole file.
`exports` resolution is left to `oxc_resolver` and its condition names.
_Avoid_: manifest, package info
