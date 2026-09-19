# 02 — Rename the injected stylesheet with `cssCodeSplit: true`

**Status:** open

**What to build:** Renaming for a Vite build that splits CSS per chunk, which is
Vite's default. Today that mode is detected and left alone, so a StyleX-only
edit keeps the stylesheet name there.

**Why it was left out:** Vite's `augmentChunkHash` folds the names of a chunk's
stylesheets into that chunk's own hash, and writes those names inside the chunk
for dynamic-import preloading. Renaming a stylesheet therefore has to rewrite
JavaScript that is already hashed, and rehashing that JavaScript moves the next
chunk, and so on. The bug fix could not carry that.

**A narrower first step worth measuring:** a stylesheet whose name appears in no
chunk -- the entry stylesheet of an app with no CSS behind a dynamic import --
is reachable from the documents and the manifests only, so it could be renamed
under the same rules as the single-stylesheet path. That covers the common app
without touching a chunk.

**Blocked by:** None, but `issues/01` makes it unnecessary.

- [ ] A StyleX-only edit changes the stylesheet name with `cssCodeSplit: true`.
- [ ] Every chunk that names a renamed stylesheet is rewritten, and its own name
      stays correct for its contents.
- [ ] Two builds of the same input still give the same names.
- [ ] The Vite test that pins today's behaviour for a split build is replaced,
      not deleted.
