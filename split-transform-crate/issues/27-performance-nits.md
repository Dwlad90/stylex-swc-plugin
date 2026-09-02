# 27 — Performance nits

**What to build:** Small costs the move introduced or exposed. None is a
measured regression; each is visible in the code.

A cache-key computation collects a vector of interned names and sorts it on
every style namespace, where a stack-allocated small vector — or sorting once
when the query is built — would do. An identifier helper builds one string
and then immediately formats a second from it, allocating twice where once
suffices. The three benches this branch moved are the allocation-heavy ones
and no longer link the allocator the shipped addon uses, so they model the
system allocator instead. The bench profile asks for debug information and
then inherits a setting that strips it, so every bench build generates
debuginfo and throws it away.

One open question rather than a defect: the candidate index does a linear
scan of a bucket on every record. That is free while buckets hold roughly one
entry, and nothing establishes what happens when many structural keys
collide.

**Blocked by:** 21

**Status:** ready-for-agent

- [ ] The per-namespace key sort stops heap-allocating on every namespace
- [ ] The identifier helper allocates once instead of twice
- [ ] The moved benches link the allocator the addon ships
- [ ] The bench profile stops discarding the debug information it asks for
- [ ] The candidate index's linear per-record bucket scan is either measured
      and dismissed, or replaced
- [ ] Benches are re-run; a lone failure is re-run before being believed, and
      the moved benches are not diffed against a pre-move baseline
- [ ] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code
- [ ] The addon is rebuilt and the JavaScript suite re-run — it exercises the
      built artifact rather than the Rust sources, so a green Rust run is not
      evidence on its own

## Comments

### A rescan the issue-24 tests exposed, measured and not worth changing yet

`CallLookup::query` (`stylex-state-index/src/key_span_index.rs`) resolves a
namespace's own value keys with a linear `find` over every property of the
call's object argument, cloning an `Atom` per property it looks at, then builds
an `FxHashSet`. `query` is called once per namespace, so a call with *n*
namespaces does *n x n* property visits plus two allocations per namespace —
and it runs *before* the span-cache check in `code_frame`, so a cache hit pays
for it too, which is the opposite of what the comment there intends.
`CallLookup` already hoists the sibling keys and the digest for exactly this
reason; the per-namespace value keys are the one thing left un-hoisted, and one
pass at construction time building `FxHashMap<Atom, FxHashSet<Atom>>` removes
the rescan.

Measured before recording: every fixture in the repository keeps *n* at three
to five, which puts this at a few hundred nanoseconds. It only pays if a real
module writes hundreds of namespaces in one `create`, which is the shape
`code_frame`'s own comment worries about. Left alone on those numbers.
