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
            "value": 1.213113,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 1.3 ms | 820.26 ops/sec | 819 samples"
          },
          {
            "name": "card",
            "value": 0.451754,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 525.3 µs | 2,193 ops/sec | 2177 samples"
          },
          {
            "name": "consts",
            "value": 0.176391,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 222.73 µs | 5,526.71 ops/sec | 5425 samples"
          },
          {
            "name": "counter",
            "value": 2.344773,
            "unit": "ms",
            "range": "±0.68%",
            "extra": "p95 2.49 ms | 422.63 ops/sec | 422 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.904987,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 3.03 ms | 343.42 ops/sec | 344 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.99656,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 3.15 ms | 332.31 ops/sec | 332 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.763027,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 815.46 µs | 1,306.91 ops/sec | 1304 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.52273,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 1.64 ms | 655.76 ops/sec | 655 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.386653,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.46 ms | 719.46 ops/sec | 719 samples"
          },
          {
            "name": "page",
            "value": 0.751116,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 853.94 µs | 1,323.76 ops/sec | 1318 samples"
          },
          {
            "name": "page-tsx",
            "value": 5.038786,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 5.23 ms | 198.05 ops/sec | 198 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 4.927429,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 5.1 ms | 202.7 ops/sec | 203 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.959517,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 2.03 ms | 509.66 ops/sec | 510 samples"
          },
          {
            "name": "typography",
            "value": 2.836722,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 2.96 ms | 351.43 ops/sec | 351 samples"
          },
          {
            "name": "use-memo",
            "value": 0.817082,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 911.47 µs | 1,212.67 ops/sec | 1210 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.505726,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 537.82 µs | 1,971.31 ops/sec | 1968 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.473084,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 530.55 µs | 2,101.44 ops/sec | 2091 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 20.644899,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 21.59 ms | 48.22 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.641579,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 716.59 µs | 1,570.07 ops/sec | 1561 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 8.410515,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 8.72 ms | 118.42 ops/sec | 119 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 6289.200529,
            "unit": "ms",
            "range": "±0.86%",
            "extra": "p95 6.34 s | 0.16 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 33.416574,
            "unit": "ms",
            "range": "±0.83%",
            "extra": "p95 34.58 ms | 29.87 ops/sec | 15 samples"
          }
        ],
        "release": {
          "ref": "fix_release-benchmarks",
          "candidateVersion": "0.18.3",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "341297928dad501ae6d2ae80b75710a9fd5941ec0ac1f86bb1748ab78e732163"
        }
      }
    ]
  }
};
