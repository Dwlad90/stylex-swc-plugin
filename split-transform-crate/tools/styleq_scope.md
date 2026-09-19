# Measuring a longer-lived styleq merger

How the numbers in [ticket 60](../issues/60-give-the-styleq-cache-a-life-longer-than-one-merge.md)
were taken, so a later reader can take them again. Two measurements, and they
answer different questions: the probe says how often a longer-lived merger would
answer from the cache, and the bench says what that is worth.

Neither patch is in the tree. Both change what the compiler does, and the
answer they gave is "leave the cache off", so keeping them would leave dead
code and, in the bench's case, a leg that measures nothing -- which is what
tickets 57 to 59 removed from this repository.

## 1. The hit rate

In `crates/stylex-styleq/src/styleq.rs`, `process_compiled_style` names the key
by address where it can. Make it name the key by hash instead, because an
address cannot answer this question:

```rust
let cache_key = next_cache.as_ref().map(|_| CacheKey::Hash(hash_style(style)));
```

Then print the key, just before the `if let Some(cache_entry) = cached` arm:

```rust
if let Some(CacheKey::Hash(probe_hash)) = cache_key.as_ref() {
  eprintln!("STYLEQPROBE K {probe_hash:x}");
}
```

and print one line per merge in `Styleq::styleq`, **before** the `while let`
loop, next to the `let mut styles = ...` that fills it. Inside the loop it
prints one line per style and every merge then holds one key:

```rust
eprintln!("STYLEQPROBE MERGE");
```

and, in `crates/stylex-transform/src/transform/visit_mut.rs`, the first line of
both `visit_mut_program` and `visit_mut_module`:

```rust
eprintln!("STYLEQPROBE FILE");
```

The merger must also be built with `disable_cache: false` in
`crates/stylex-transform/src/shared/utils/core/styleq.rs`, or the walk computes
no key and the probe prints nothing.

```sh
cargo test -p stylex_transform --all-features -- --nocapture --test-threads=1 > probe.log 2>&1
python3 .scratch/split-transform-crate/tools/styleq_scope.py probe.log
```

The per-crate spelling is what makes the log readable, because it keeps the
other crates' merges out of it. It can fail to build on feature unification,
which the workspace run hides; if it does, run the workspace and keep only the
`stylex_transform` part of the log.

**A merge before the first file marker belongs to no file.** The suite calls the
merger directly as well as through the transform, and such a merge has no file
to scope one to. The reader counts those and leaves them out; pooling them into
one bucket makes 76 unrelated merges look like one very long file, and read the
file rate 10 points low.

**One test thread, or the log interleaves** and a merge collects another
thread's keys. The suite prints its own lines into the same stream, so the
script matches the marker anywhere in a line rather than at its start.

**The key is the hash, not the address.** A merger that outlives the merge
cannot use the address of a style the merge built, so the structural key is the
only one the question can be asked with. Leave the address arm in place and the
probe prints no key at all; the reader says so rather than dividing by zero.

**The reader is checked against a known answer.** The marker lines of the run
behind ticket 60 are kept at
[`baseline/styleq-scope-probe.log`](../baseline/styleq-scope-probe.log), and
every line below the header must come back:

```sh
python3 .scratch/split-transform-crate/tools/styleq_scope.py \
  .scratch/split-transform-crate/baseline/styleq-scope-probe.log
# 76 merges left out, then merge 0.0%, file 23.7%, process 64.3%,
# over 817 merges and 447 files
```

Run this after changing the reader.

## 2. What a hit is worth

A temporary criterion bench in `crates/stylex-transform/benches`, with both legs
in one process: the same parsed module transformed once with the merger built
per merge and once with a merger that is reset per file. It needs
`use swc_malloc as _;`, which `guidelines/PERFORMANCE.md` requires of every
bench, and a `thread_local!` merger plus a `CACHE_SCOPE` switch in
`shared/utils/core/styleq.rs`.

The module is generated rather than taken from the corpus, and that is
deliberate: the corpus says how often a merger would hit, and the generated
module says what the best case is worth. 100 components, each reading its styles
from nine `stylex.props` sites, repeat 8 merges in 9.

**Print both legs before timing and compare the text.** The cache is
transparent, so a leg that disagrees is a broken patch, not a faster merger.

**Read the machine first.** `top -l 2 -n 0 | grep "CPU usage"` -- the runs
behind ticket 60 were taken at 89.5% idle.
