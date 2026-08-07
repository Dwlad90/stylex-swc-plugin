window.BENCHMARK_DATA = {
  "lastUpdate": 1786119658803,
  "repoUrl": "https://github.com/Dwlad90/stylex-swc-plugin",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "name": "Dwlad90",
            "email": "dwlad90@gmail.com",
            "username": "Dwlad90"
          },
          "committer": {
            "name": "Dwlad90",
            "email": "dwlad90@gmail.com",
            "username": "Dwlad90"
          },
          "id": "6959d636c3fd1b0baed4637453a16c6dec7d76b4",
          "message": "fix(ci): compare release verdicts against the status the engine emits",
          "timestamp": "2026-08-07T18:31:06+03:00",
          "tree_id": "86236c6b67c240b99812e9dea86e4001bf5f7da9",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/6959d636c3fd1b0baed4637453a16c6dec7d76b4"
        },
        "date": 1786119658803,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.868978,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 905.28 µs | 1,147.66 ops/sec | 1147 samples"
          },
          {
            "name": "card",
            "value": 0.393895,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 424.77 µs | 2,508.78 ops/sec | 2505 samples"
          },
          {
            "name": "consts",
            "value": 0.169367,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 202.16 µs | 5,765.94 ops/sec | 5729 samples"
          },
          {
            "name": "counter",
            "value": 1.529203,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.57 ms | 652.9 ops/sec | 653 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.039185,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 2.07 ms | 489.91 ops/sec | 490 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.352959,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 3.41 ms | 298 ops/sec | 298 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.800039,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 828.48 µs | 1,251.23 ops/sec | 1251 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.141763,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.18 ms | 873.17 ops/sec | 873 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.052688,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 1.08 ms | 947.36 ops/sec | 947 samples"
          },
          {
            "name": "page",
            "value": 0.517655,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 541.67 µs | 1,937.91 ops/sec | 1936 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.427632,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 3.48 ms | 291.47 ops/sec | 292 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.337466,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 3.39 ms | 299.35 ops/sec | 300 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.460821,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.5 ms | 682.82 ops/sec | 683 samples"
          },
          {
            "name": "typography",
            "value": 2.254338,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 2.31 ms | 442.63 ops/sec | 443 samples"
          },
          {
            "name": "use-memo",
            "value": 0.585058,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 604.39 µs | 1,720.89 ops/sec | 1720 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.576475,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 606.9 µs | 1,739.22 ops/sec | 1737 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.498541,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 521.11 µs | 2,009.51 ops/sec | 2008 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 10.081787,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 10.3 ms | 99.13 ops/sec | 100 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.536218,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 563.23 µs | 1,872.29 ops/sec | 1870 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.510307,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 6.63 ms | 153.41 ops/sec | 154 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3520.925344,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 3.54 s | 0.28 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 24.368999,
            "unit": "ms",
            "range": "±0.68%",
            "extra": "p95 25.06 ms | 40.98 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "fix_release-benchmarks",
          "candidateVersion": "0.18.3",
          "previousVersion": "0.18.3",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "265116db9b2633106d522bbfe82a1e1e9468a54fa78f9aa4bbd431a59c07c6d2"
        }
      }
    ]
  }
};
