# 02 — Prove the reported divergence is gone and pin the reporter's input

**What to build:** The exact module from GitHub issue #1251 — a dynamic style
whose parameter sits behind `:hover` — produces the same `@property`
registration text under this compiler as under `@stylexjs/babel-plugin` v0.19.0,
and that input is pinned by a create-call snapshot so it cannot regress
unnoticed.

The behavioral fix already landed (`01`). What is missing is proof: the
regression pin added alongside that fix uses an input with no upstream
counterpart, so it currently asserts only that this compiler keeps doing what it
already does. This ticket converts that into a checked fact once, and leaves
behind a pin whose input is the one a real user reported.

The differential runner is a throwaway — it lives in the session scratchpad and
is deleted once green. The durable artifact is the snapshot.

**Blocked by:** None — can start immediately.

**Status:** resolved

- [x] The stale `fix_media_queries_are_not_canonicalized` branch is deleted
      (identical to `develop`, remote gone) and
      `fix_share-pseudo-element-predicate` is created from `develop` in this
      worktree
- [x] A release build of the compiler's native artifact exists, and the
      reported module is transformed through it — not through a debug build,
      and not through the crate sources, since the JS-facing entry point
      exercises the built artifact
- [x] The same module is transformed through `@stylexjs/babel-plugin` v0.19.0,
      already present in the workspace package store and requireable by
      absolute path — no install, no new dependency
- [x] Both sides use identical options, and the emitted `@property` rules are
      compared for byte equality; the `inherits` value and the variable hashes
      both match
- [x] The reported input is added as a create-call snapshot case in the
      dynamic-styles suite, alongside — not replacing — the existing
      bare-pseudo-class pin
- [x] The throwaway differential runner is deleted; nothing under `scripts/` or
      the Node packages gains a permanent second compiler
- [x] `cargo test --workspace` green; the new snapshot's baseline asserts
      `inherits: false`
- [x] Committed as `test(stylex-transform): …` — one commit, no behavior change

## Answer

**The reported divergence is gone, verified against the release artifact.**

`pnpm run --filter=@stylexswc/rs-compiler build` produced
`crates/stylex-rs-compiler/dist/rs-compiler.darwin-arm64.node`. A throwaway
runner in the session scratchpad transformed the reported module through that
artifact's `transform()` (options via `normalizeRsOptions`) and through
`@stylexjs/babel-plugin@0.19.0`, required by absolute path out of
`node_modules/.pnpm/@stylexjs+babel-plugin@0.19.0_supports-color@8.1.1`, with
`transformAsync` and `withOptions`. Both sides used
`{ dev: false, treeshakeCompensation: true, unstable_moduleResolution:
{ type: 'commonJS', rootDir: <repo> } }`.

The `@property` rules are byte-identical, hashes included:

```
--x-bcbnzo   @property --x-bcbnzo { syntax: "*"; inherits: false;}
--x-1e2mv7m  @property --x-1e2mv7m { syntax: "*"; inherits: false;}
```

That is the reported input's two variables — the `default` one and the
`:hover` one — both correctly registered as non-inheriting. The runner has
been deleted; nothing permanent was added under `scripts/` or the Node
packages.

**The durable pin** is
`default_and_hover_dynamic_values_generate_at_property_with_inherits_false` in
`crates/stylex-transform/tests/transform_stylex_create_test/dynamic_styles.rs`,
using the reporter's module verbatim, next to the existing
`dynamic_style_in_hover_generates_at_property_with_inherits_false` rather than
replacing it. Its baseline asserts `inherits: false` for both variables.

`cargo test --workspace` green (no snapshot movement outside the new
baseline); `cargo fmt --check` and `cargo clippy --workspace --all-targets`
clean. Committed as `62f3e8e25`.

The pinned module is the reporter's, re-indented to the file's 2-space step
(a `/code-review` standards finding — the repo indents 2 spaces everywhere,
and every sibling case in the file does). Whitespace inside the raw string is
not hashed, so the snapshot did not move; the pin is verbatim in every way
that reaches the output.

Note on the commit text: the maintainer asked mid-work that new code and
commit messages not carry claims about mirroring or porting another
implementation, so the test comment and commit body state the rule on its own
terms — the differential evidence lives here, not in the tree.
