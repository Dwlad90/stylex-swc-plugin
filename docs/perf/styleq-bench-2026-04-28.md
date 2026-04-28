# Styleq Benchmark Comparison — 2026-04-28

## Commands

```bash
cargo bench -p stylex_styleq --bench performance_bench -- --sample-size 10
cd <local-styleq-checkout>
node benchmark/performance.js --outfile <session-files>/styleq-js-perf.json
```

## Results

The Rust benchmark mirrors the upstream JS `benchmark/performance.js` fixture
names. The JS numbers are reported by Benchmark.js as operations per second.
Rust operations per second are derived from Criterion mean latency.

| Benchmark | Rust mean | Rust ops/sec | JS ops/sec | Rust / JS |
|---|---:|---:|---:|---:|
| `small object` | 400.72 ns | 2,495,510 | 19,474,880 | 0.13x |
| `small object (cache miss)` | 497.02 ns | 2,011,973 | 2,062,827 | 0.98x |
| `small object (cache disabled)` | 539.03 ns | 1,855,174 | 14,046,414 | 0.13x |
| `large object` | 2,457.74 ns | 406,877 | 12,424,387 | 0.03x |
| `large object (cache miss)` | 3,063.68 ns | 326,404 | 799,489 | 0.41x |
| `large object (cache disabled)` | 4,486.00 ns | 222,916 | 1,654,836 | 0.13x |
| `small merge` | 885.78 ns | 1,128,953 | 13,136,284 | 0.09x |
| `small merge (cache miss)` | 1,078.58 ns | 927,141 | 1,702,174 | 0.54x |
| `small merge (cache disabled)` | 701.55 ns | 1,425,407 | 11,104,750 | 0.13x |
| `large merge` | 7,021.98 ns | 142,410 | 2,432,873 | 0.06x |
| `large merge (cache disabled)` | 9,989.01 ns | 100,110 | 630,988 | 0.16x |
| `large merge (transform)` | 17,037.45 ns | 58,694 | 1,917,673 | 0.03x |
| `small inline style` | 288.61 ns | 3,464,833 | 19,212,065 | 0.18x |
| `large inline style` | 3,674.73 ns | 272,129 | 1,932,913 | 0.14x |
| `merged inline style` | 3,839.94 ns | 260,420 | 1,815,813 | 0.14x |
| `merged inline style (mix disabled)` | 2,969.81 ns | 336,722 | 2,023,224 | 0.17x |

## Notes

- The `stylex-transform` adapter opts into object-identity cache keys for its
  borrowed style maps. The public `StyleqInput` benchmark keeps the content-hash
  fallback because owned temporary maps cannot safely use raw pointer identity in
  a long-lived cache.
- The Rust rows keep the upstream benchmark names for comparison. Cases labeled
  `cache miss` are not perfectly identity-equivalent to JS when run through the
  public owned `StyleqInput` API, because the safe fallback cache is content-hash
  based.
- Cached and inline-style JS cases remain faster in this microbenchmark. The
  Rust benchmark uses the generic `StyleMap<String, StyleValue>` API, while the
  upstream JS benchmark operates directly on JavaScript object shapes. Treat this
  as a baseline for future Rust-specific styleq optimizations rather than a
  release gate against JS throughput.
