# 01 — Compile StyleX before the module graph is walked

**Status:** open

**What to build:** A pre-pass that compiles StyleX for every module up front and
caches the result by content, so `transform` serves the cache rather than doing
the work. The marker stylesheet then carries the real rules into the module
graph, and the host hashes it natively, at the point it names every other asset.

**Why:** This is the only correct fix for the whole class. Today the rules are
spliced in after the host has finished, which is what makes a rename necessary
at all. A pre-pass removes the need: it works in both `cssCodeSplit` modes, it
fixes esbuild for free, and it lets every plugin that reads CSS file names run
in whatever order it likes.

**Why it is not the bug fix:** It is a redesign of the plugin, not a repair. It
changes when compilation happens for every host, which is a much larger surface
than the stale name it would settle.

**Blocked by:** None.

- [ ] Rules reach the stylesheet before the host names it, in both
      `cssCodeSplit` modes.
- [ ] The rename in `injectPlaceholderIntoBundle` and `renameEsbuildStylesheets`
      is no longer reached, and is removed rather than left dormant.
- [ ] A StyleX-only edit changes the stylesheet name under Vite's defaults.
- [ ] Compiling twice for one module cannot happen; the cache is proved by a
      counted transform, not assumed.
