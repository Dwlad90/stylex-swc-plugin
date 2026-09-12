# 01 — Share the module selection predicate across the bundler plugins

**What to build:** Nothing changes for a user of any bundler plugin. Every
plugin decides which modules to hand to the compiler exactly as it does today,
but it asks one shared function instead of carrying its own copy of the answer.

Five packages each hold their own copy of the same source-text scan: the Vite
and Rollup plugins, the shared loader behind webpack, Rspack and Next.js, the
Turbopack loader, and the PostCSS plugin. The copies have already drifted —
two of them inspect only one field of an object-shaped import source while the
others inspect both. This ticket makes them one implementation so the next
change to module selection lands everywhere at once.

The function goes in the shared plugin package, exported from a new leaf
subpath so that a consumer gets the predicate without pulling in the plugin
core. The one plugin that does not yet depend on that package gains the
dependency; the version catalog entry already exists, so no workspace change is
needed.

This is a prefactor. It carries no behaviour change of its own — it exists to
make ticket 04 a few lines rather than a five-file edit.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] One implementation of the predicate lives in the shared package and is
      reachable from a leaf subpath export
- [ ] All five plugin packages call it; no package retains a private copy
- [ ] The drift between copies is resolved by the shared version, and the
      resolution is toward the more permissive behaviour, so no module that is
      processed today stops being processed
- [ ] The holdout package's dependency on the shared package is declared
- [ ] Every existing plugin test passes unchanged — no test is edited in this
      ticket
- [ ] Type checking, linting, formatting and the JavaScript suites pass
