window.BENCHMARK_DATA = {
  "lastUpdate": 1788616038753,
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
      },
      {
        "commit": {
          "author": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "committer": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "id": "342673c8212d82324dca2f3df574a79a9c7fcd0a",
          "message": "Bump version to 0.18.4-dev.1",
          "timestamp": "2026-08-07T19:36:31Z",
          "tree_id": "11ea8f94cf9b8fe8a922e34ce6f394ec733914f4",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/342673c8212d82324dca2f3df574a79a9c7fcd0a"
        },
        "date": 1786132959858,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.904666,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 936.17 µs | 1,104.22 ops/sec | 1104 samples"
          },
          {
            "name": "card",
            "value": 0.428437,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 459.29 µs | 2,325.1 ops/sec | 2322 samples"
          },
          {
            "name": "consts",
            "value": 0.175429,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 207.77 µs | 5,570.32 ops/sec | 5537 samples"
          },
          {
            "name": "counter",
            "value": 1.57948,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.62 ms | 631.94 ops/sec | 632 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.110992,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 2.17 ms | 473.5 ops/sec | 474 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.493938,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 3.56 ms | 286.27 ops/sec | 287 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.827557,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 850.23 µs | 1,210.03 ops/sec | 1210 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.16605,
            "unit": "ms",
            "range": "±0.09%",
            "extra": "p95 1.2 ms | 856.06 ops/sec | 856 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.074177,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.1 ms | 928.49 ops/sec | 929 samples"
          },
          {
            "name": "page",
            "value": 0.540897,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 560.06 µs | 1,861.27 ops/sec | 1860 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.497834,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 3.56 ms | 285.54 ops/sec | 286 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.400659,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 3.46 ms | 293.74 ops/sec | 294 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.491682,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 1.53 ms | 669.85 ops/sec | 670 samples"
          },
          {
            "name": "typography",
            "value": 2.226108,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 2.27 ms | 448.46 ops/sec | 449 samples"
          },
          {
            "name": "use-memo",
            "value": 0.586628,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 606.36 µs | 1,716.77 ops/sec | 1716 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.574348,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 599.62 µs | 1,748.84 ops/sec | 1747 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.497145,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 524.59 µs | 2,013.74 ops/sec | 2012 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.92203,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 10.02 ms | 100.74 ops/sec | 101 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.530744,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 556.55 µs | 1,890.46 ops/sec | 1889 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.393254,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 6.49 ms | 156.3 ops/sec | 157 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3442.904388,
            "unit": "ms",
            "range": "±0.77%",
            "extra": "p95 3.52 s | 0.29 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.98661,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 24.33 ms | 41.61 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "27493c0e69ee77a815707fe7b4c953663b46018ecf8f056a5e28a4b993e862df"
        }
      },
      {
        "commit": {
          "author": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "committer": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "id": "9464405cbd7e6a4bcead8d4448dc8ac103ba486f",
          "message": "Bump version to 0.18.4-dev.1",
          "timestamp": "2026-08-07T20:19:58Z",
          "tree_id": "a2dea79e9d90e2735b35fe1f10c7ee8d8de98ecb",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/9464405cbd7e6a4bcead8d4448dc8ac103ba486f"
        },
        "date": 1786135515316,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.907839,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 937.23 µs | 1,100.32 ops/sec | 1100 samples"
          },
          {
            "name": "card",
            "value": 0.417252,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 447.68 µs | 2,366.33 ops/sec | 2363 samples"
          },
          {
            "name": "consts",
            "value": 0.179638,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 213.21 µs | 5,443.17 ops/sec | 5407 samples"
          },
          {
            "name": "counter",
            "value": 1.584559,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 1.62 ms | 630.63 ops/sec | 631 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.106895,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 2.16 ms | 473.68 ops/sec | 474 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.491792,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 3.55 ms | 286.09 ops/sec | 287 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.837376,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 867.4 µs | 1,193.9 ops/sec | 1194 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.185794,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.22 ms | 841.06 ops/sec | 841 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.091728,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.12 ms | 913.86 ops/sec | 914 samples"
          },
          {
            "name": "page",
            "value": 0.549436,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 571 µs | 1,827.67 ops/sec | 1826 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.545343,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 3.6 ms | 282 ops/sec | 282 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.46262,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 3.52 ms | 288.87 ops/sec | 289 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.515884,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.56 ms | 658.8 ops/sec | 659 samples"
          },
          {
            "name": "typography",
            "value": 2.23211,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 2.28 ms | 447.5 ops/sec | 448 samples"
          },
          {
            "name": "use-memo",
            "value": 0.587018,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 606.97 µs | 1,709.47 ops/sec | 1708 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.575665,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 599.27 µs | 1,742.21 ops/sec | 1741 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.504055,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 527.57 µs | 1,991.75 ops/sec | 1990 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.887132,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 9.99 ms | 101.15 ops/sec | 102 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.531936,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 556.56 µs | 1,882.35 ops/sec | 1881 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.419121,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 6.5 ms | 155.59 ops/sec | 156 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3429.112824,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 3.47 s | 0.29 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.987496,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 24.15 ms | 41.68 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "27493c0e69ee77a815707fe7b4c953663b46018ecf8f056a5e28a4b993e862df"
        }
      },
      {
        "commit": {
          "author": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "committer": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "id": "7f1606d03cd77e8fcb36bcc9f1d10ed3defec2c0",
          "message": "Bump version to 0.18.4-dev.1",
          "timestamp": "2026-08-09T14:41:44Z",
          "tree_id": "5697a039b3018f3a163b16dbd45fe096cdc8e05a",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/7f1606d03cd77e8fcb36bcc9f1d10ed3defec2c0"
        },
        "date": 1786287957693,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.884226,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 914.51 µs | 1,129.88 ops/sec | 1130 samples"
          },
          {
            "name": "card",
            "value": 0.406044,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 441.29 µs | 2,431.85 ops/sec | 2427 samples"
          },
          {
            "name": "consts",
            "value": 0.175585,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 209.66 µs | 5,571.41 ops/sec | 5532 samples"
          },
          {
            "name": "counter",
            "value": 1.554047,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.6 ms | 642.86 ops/sec | 643 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.117488,
            "unit": "ms",
            "range": "±0.09%",
            "extra": "p95 2.16 ms | 471.9 ops/sec | 472 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.401482,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 3.45 ms | 293.8 ops/sec | 294 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.821477,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 850.89 µs | 1,218.42 ops/sec | 1218 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.173212,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.21 ms | 850.97 ops/sec | 851 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.083781,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.11 ms | 922.01 ops/sec | 922 samples"
          },
          {
            "name": "page",
            "value": 0.536136,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 559.16 µs | 1,872.77 ops/sec | 1872 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.51145,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 3.58 ms | 284.49 ops/sec | 285 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.409082,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 3.47 ms | 293.19 ops/sec | 294 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.497373,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.54 ms | 667.15 ops/sec | 667 samples"
          },
          {
            "name": "typography",
            "value": 2.237131,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 2.27 ms | 447.07 ops/sec | 448 samples"
          },
          {
            "name": "use-memo",
            "value": 0.581706,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 604 µs | 1,725.82 ops/sec | 1725 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.561349,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 588.06 µs | 1,787.53 ops/sec | 1786 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.493679,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 522.08 µs | 2,028.58 ops/sec | 2026 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.924947,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 10.11 ms | 100.74 ops/sec | 101 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.532088,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 559.33 µs | 1,882.69 ops/sec | 1881 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.392885,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 7.26 ms | 155.37 ops/sec | 156 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3504.710601,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 3.52 s | 0.29 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 24.481989,
            "unit": "ms",
            "range": "±1.64%",
            "extra": "p95 25.58 ms | 40.7 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "fd734d491e65688400f895e0d7c1826b9876d0a81591d4ba173e60cfedc48df3"
        }
      },
      {
        "commit": {
          "author": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "committer": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "id": "a53185f3c8e9ecbbb31798b1e434a724e2358ef8",
          "message": "Bump version to 0.18.4-dev.1",
          "timestamp": "2026-08-09T15:20:44Z",
          "tree_id": "5697a039b3018f3a163b16dbd45fe096cdc8e05a",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/a53185f3c8e9ecbbb31798b1e434a724e2358ef8"
        },
        "date": 1786290037523,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.871243,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 900.28 µs | 1,146.21 ops/sec | 1146 samples"
          },
          {
            "name": "card",
            "value": 0.408231,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 439.88 µs | 2,435.52 ops/sec | 2432 samples"
          },
          {
            "name": "consts",
            "value": 0.167276,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 195.64 µs | 5,855.29 ops/sec | 5826 samples"
          },
          {
            "name": "counter",
            "value": 1.528728,
            "unit": "ms",
            "range": "±0.09%",
            "extra": "p95 1.56 ms | 653.17 ops/sec | 654 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.0568,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 2.1 ms | 485.4 ops/sec | 486 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.385765,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 3.44 ms | 295.11 ops/sec | 296 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.807206,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 829.94 µs | 1,239.94 ops/sec | 1240 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.144207,
            "unit": "ms",
            "range": "±0.08%",
            "extra": "p95 1.17 ms | 872.84 ops/sec | 873 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.054301,
            "unit": "ms",
            "range": "±0.08%",
            "extra": "p95 1.08 ms | 948.42 ops/sec | 949 samples"
          },
          {
            "name": "page",
            "value": 0.515103,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 535.29 µs | 1,948.79 ops/sec | 1947 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.441479,
            "unit": "ms",
            "range": "±0.09%",
            "extra": "p95 3.49 ms | 290.44 ops/sec | 291 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.356202,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 3.41 ms | 297.88 ops/sec | 298 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.458667,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.5 ms | 685.38 ops/sec | 686 samples"
          },
          {
            "name": "typography",
            "value": 2.18515,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 2.23 ms | 457.36 ops/sec | 458 samples"
          },
          {
            "name": "use-memo",
            "value": 0.561187,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 584.14 µs | 1,787.01 ops/sec | 1786 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.564175,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 588.47 µs | 1,776.6 ops/sec | 1775 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.480277,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 505.74 µs | 2,082.25 ops/sec | 2081 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.711259,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 9.82 ms | 102.91 ops/sec | 103 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.497457,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 518.48 µs | 2,011.2 ops/sec | 2010 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.290239,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 6.38 ms | 158.89 ops/sec | 159 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3248.428896,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 3.27 s | 0.31 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.123969,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 23.27 ms | 43.22 ops/sec | 22 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "fd734d491e65688400f895e0d7c1826b9876d0a81591d4ba173e60cfedc48df3"
        }
      },
      {
        "commit": {
          "author": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "committer": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "id": "1975be5a1f882cb2532a94ce645d59e1d9568a86",
          "message": "Bump version to 0.18.4-rc.1",
          "timestamp": "2026-08-11T22:26:26Z",
          "tree_id": "047e45f845578d3c1d843b5f59ea8bacf5cff39c",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/1975be5a1f882cb2532a94ce645d59e1d9568a86"
        },
        "date": 1786488826044,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.831072,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 861.9 µs | 1,201.25 ops/sec | 1201 samples"
          },
          {
            "name": "card",
            "value": 0.385978,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 413.81 µs | 2,585.82 ops/sec | 2581 samples"
          },
          {
            "name": "consts",
            "value": 0.160399,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 188.49 µs | 6,101.79 ops/sec | 6069 samples"
          },
          {
            "name": "counter",
            "value": 1.472189,
            "unit": "ms",
            "range": "±0.09%",
            "extra": "p95 1.51 ms | 678.59 ops/sec | 679 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.960221,
            "unit": "ms",
            "range": "±0.08%",
            "extra": "p95 2 ms | 509.77 ops/sec | 510 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.185099,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 3.25 ms | 313.47 ops/sec | 314 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.753198,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 773.33 µs | 1,330.22 ops/sec | 1330 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.087398,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 1.12 ms | 918.01 ops/sec | 918 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.996571,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 1.02 ms | 1,002.23 ops/sec | 1002 samples"
          },
          {
            "name": "page",
            "value": 0.488276,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 513.35 µs | 2,042.59 ops/sec | 2041 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.289222,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 3.34 ms | 303.63 ops/sec | 304 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.204267,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 3.26 ms | 311.68 ops/sec | 312 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.386658,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.42 ms | 719.68 ops/sec | 720 samples"
          },
          {
            "name": "typography",
            "value": 2.052059,
            "unit": "ms",
            "range": "±0.08%",
            "extra": "p95 2.09 ms | 486.76 ops/sec | 487 samples"
          },
          {
            "name": "use-memo",
            "value": 0.53347,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 553.41 µs | 1,884.28 ops/sec | 1884 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.514619,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 538.9 µs | 1,947.35 ops/sec | 1946 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.450294,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 477.49 µs | 2,216.52 ops/sec | 2214 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.581457,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 9.67 ms | 104.45 ops/sec | 105 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.477701,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 507.02 µs | 2,087.79 ops/sec | 2086 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.021494,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 6.1 ms | 165.9 ops/sec | 166 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3373.246313,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 3.42 s | 0.3 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 22.709059,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 23.01 ms | 44 ops/sec | 22 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-rc.1",
          "candidateVersion": "0.18.4-rc.1",
          "previousVersion": "0.18.3",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "1af10fe997ecd838a61c1f8a7e201e62d6959223d332b02a4664a98f73ba5545"
        }
      },
      {
        "commit": {
          "author": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "committer": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "id": "8724fa7eebebcde4d14f9d658660d28724e24994",
          "message": "Bump version to 0.18.4-rc.2",
          "timestamp": "2026-08-17T16:57:00Z",
          "tree_id": "331d10462e4110beb94192ea71940d4bbb6078a8",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/8724fa7eebebcde4d14f9d658660d28724e24994"
        },
        "date": 1786987572898,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.812556,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 843.29 µs | 1,231.24 ops/sec | 1231 samples"
          },
          {
            "name": "card",
            "value": 0.369696,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 401.78 µs | 2,704.81 ops/sec | 2698 samples"
          },
          {
            "name": "consts",
            "value": 0.168341,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 199.98 µs | 5,797.96 ops/sec | 5763 samples"
          },
          {
            "name": "counter",
            "value": 1.336033,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 1.37 ms | 747.2 ops/sec | 748 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.866732,
            "unit": "ms",
            "range": "±0.09%",
            "extra": "p95 1.9 ms | 535.82 ops/sec | 536 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.452724,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 3.51 ms | 289.24 ops/sec | 290 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.820831,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 843.02 µs | 1,219.6 ops/sec | 1220 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.252631,
            "unit": "ms",
            "range": "±0.08%",
            "extra": "p95 1.28 ms | 796.84 ops/sec | 797 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.159797,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.19 ms | 859.79 ops/sec | 860 samples"
          },
          {
            "name": "page",
            "value": 0.428772,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 460.34 µs | 2,303.25 ops/sec | 2301 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.221311,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 3.27 ms | 310.08 ops/sec | 311 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.149732,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 3.2 ms | 317.02 ops/sec | 317 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.255079,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.29 ms | 794.55 ops/sec | 795 samples"
          },
          {
            "name": "typography",
            "value": 2.003622,
            "unit": "ms",
            "range": "±0.07%",
            "extra": "p95 2.04 ms | 498.8 ops/sec | 499 samples"
          },
          {
            "name": "use-memo",
            "value": 0.509263,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 533.06 µs | 1,968.58 ops/sec | 1967 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.572675,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 596.63 µs | 1,752.91 ops/sec | 1752 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.493504,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 516.73 µs | 2,032.53 ops/sec | 2031 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.876261,
            "unit": "ms",
            "range": "±0.09%",
            "extra": "p95 9.96 ms | 101.2 ops/sec | 102 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.414022,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 444.02 µs | 2,390.98 ops/sec | 2389 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.796537,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 5.87 ms | 172.3 ops/sec | 173 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3275.599346,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 3.3 s | 0.31 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 22.432205,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 22.69 ms | 44.59 ops/sec | 23 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-rc.2",
          "candidateVersion": "0.18.4-rc.2",
          "previousVersion": "0.18.3",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "4cc2011553899ded89c52b67cd7610320fb6593f396e4695c4a33f581fa631a0"
        }
      },
      {
        "commit": {
          "author": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "committer": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "id": "a40d7428b39dfd5fa2ddbc773ed4b433da1c046c",
          "message": "Bump version to 0.18.4",
          "timestamp": "2026-08-19T19:08:00Z",
          "tree_id": "e1ee86fc6d193c9f2374200b5a71fcd02d402b47",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/a40d7428b39dfd5fa2ddbc773ed4b433da1c046c"
        },
        "date": 1787168120108,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.886228,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 916.42 µs | 1,128.37 ops/sec | 1128 samples"
          },
          {
            "name": "card",
            "value": 0.413163,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 462.07 µs | 2,409.67 ops/sec | 2402 samples"
          },
          {
            "name": "consts",
            "value": 0.172902,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 204.76 µs | 5,681.72 ops/sec | 5647 samples"
          },
          {
            "name": "counter",
            "value": 1.565571,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 1.6 ms | 638.24 ops/sec | 639 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.109339,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 2.18 ms | 473.34 ops/sec | 474 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.524325,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 3.63 ms | 283.1 ops/sec | 284 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.791885,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 816.06 µs | 1,263.91 ops/sec | 1264 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.151876,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 1.19 ms | 866.17 ops/sec | 866 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.041159,
            "unit": "ms",
            "range": "±0.09%",
            "extra": "p95 1.07 ms | 959.33 ops/sec | 960 samples"
          },
          {
            "name": "page",
            "value": 0.518882,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 544.45 µs | 1,929.23 ops/sec | 1928 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.722822,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 3.84 ms | 268.37 ops/sec | 269 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.575583,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 3.7 ms | 280.23 ops/sec | 281 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.462698,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 1.51 ms | 682.22 ops/sec | 682 samples"
          },
          {
            "name": "typography",
            "value": 2.214131,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 2.29 ms | 450.9 ops/sec | 451 samples"
          },
          {
            "name": "use-memo",
            "value": 0.559937,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 585.93 µs | 1,787.67 ops/sec | 1787 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.547701,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 573.52 µs | 1,831.7 ops/sec | 1830 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.475437,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 507.66 µs | 2,097.74 ops/sec | 2094 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 10.92383,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 11.16 ms | 91.69 ops/sec | 92 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.501524,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 525.67 µs | 1,997.68 ops/sec | 1996 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 7.01136,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 7.16 ms | 142.53 ops/sec | 143 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3673.568592,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 3.71 s | 0.27 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 27.95737,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 28.81 ms | 35.67 ops/sec | 18 samples"
          }
        ],
        "release": {
          "ref": "0.18.4",
          "candidateVersion": "0.18.4",
          "previousVersion": "0.18.3",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "0f68b7c58c1b392d0b912d583e2509f19973817742aff2d06375d933836a301c"
        }
      },
      {
        "commit": {
          "author": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "committer": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "id": "abb0b3e5233399b9ba50ff1be2e8e68656488cfc",
          "message": "Bump version to 0.18.5-rc.1",
          "timestamp": "2026-08-26T22:18:32Z",
          "tree_id": "4f87097fd05a263b722f17299275925428e65f62",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/abb0b3e5233399b9ba50ff1be2e8e68656488cfc"
        },
        "date": 1787784545113,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.866835,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 895.03 µs | 1,153.05 ops/sec | 1153 samples"
          },
          {
            "name": "card",
            "value": 0.388916,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 419.15 µs | 2,544.54 ops/sec | 2540 samples"
          },
          {
            "name": "consts",
            "value": 0.168501,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 210.74 µs | 5,724.28 ops/sec | 5669 samples"
          },
          {
            "name": "counter",
            "value": 1.511903,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.54 ms | 660.66 ops/sec | 661 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.028816,
            "unit": "ms",
            "range": "±0.08%",
            "extra": "p95 2.06 ms | 492.36 ops/sec | 493 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.293419,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 3.34 ms | 303.22 ops/sec | 304 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.788593,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 815 µs | 1,269.15 ops/sec | 1269 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.118683,
            "unit": "ms",
            "range": "±0.08%",
            "extra": "p95 1.15 ms | 892.24 ops/sec | 893 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.036592,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.06 ms | 963.14 ops/sec | 963 samples"
          },
          {
            "name": "page",
            "value": 0.508352,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 530.98 µs | 1,971.77 ops/sec | 1970 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.39261,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 3.44 ms | 294.47 ops/sec | 295 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.321888,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 3.38 ms | 300.73 ops/sec | 301 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.430013,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 1.47 ms | 697.93 ops/sec | 698 samples"
          },
          {
            "name": "typography",
            "value": 2.141867,
            "unit": "ms",
            "range": "±0.08%",
            "extra": "p95 2.18 ms | 466.39 ops/sec | 467 samples"
          },
          {
            "name": "use-memo",
            "value": 0.554089,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 577.03 µs | 1,812.01 ops/sec | 1811 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.559218,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 584.57 µs | 1,793.25 ops/sec | 1792 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.475715,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 499.16 µs | 2,107.38 ops/sec | 2106 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.709952,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 9.82 ms | 102.98 ops/sec | 103 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.502371,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 524.07 µs | 1,997.82 ops/sec | 1996 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.180382,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 6.26 ms | 161.6 ops/sec | 162 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3393.499846,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 3.48 s | 0.29 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.479024,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 23.77 ms | 42.58 ops/sec | 22 samples"
          }
        ],
        "release": {
          "ref": "0.18.5-rc.1",
          "candidateVersion": "0.18.5-rc.1",
          "previousVersion": "0.18.4",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "4420f7449ccdd1b2fe3c7b886f63c70968eb66f6f7d74f025837f0459eab8566"
        }
      },
      {
        "commit": {
          "author": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "committer": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "id": "7e8a7bcef58ab6ae6dce65faef576e44eb15317d",
          "message": "Bump version to 0.18.5",
          "timestamp": "2026-08-29T11:49:30Z",
          "tree_id": "729c009d17453aa3189c3362fadcfa6d2cf9f019",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/7e8a7bcef58ab6ae6dce65faef576e44eb15317d"
        },
        "date": 1788005791782,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.928505,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 959.15 µs | 1,076.78 ops/sec | 1077 samples"
          },
          {
            "name": "card",
            "value": 0.442438,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 472.76 µs | 2,256.5 ops/sec | 2254 samples"
          },
          {
            "name": "consts",
            "value": 0.179637,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 211.98 µs | 5,450.35 ops/sec | 5417 samples"
          },
          {
            "name": "counter",
            "value": 1.620711,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 1.66 ms | 616.69 ops/sec | 617 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.159861,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 2.21 ms | 462.45 ops/sec | 463 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.513899,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 3.58 ms | 284.36 ops/sec | 285 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.849971,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 878.68 µs | 1,177.73 ops/sec | 1177 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.182927,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 1.22 ms | 843.77 ops/sec | 844 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.09271,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.13 ms | 913.02 ops/sec | 913 samples"
          },
          {
            "name": "page",
            "value": 0.547962,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 576.09 µs | 1,828.14 ops/sec | 1827 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.580781,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 3.63 ms | 278.98 ops/sec | 279 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.496095,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 3.55 ms | 285.71 ops/sec | 286 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.550022,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 1.59 ms | 644.81 ops/sec | 645 samples"
          },
          {
            "name": "typography",
            "value": 2.283165,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 2.33 ms | 437.48 ops/sec | 438 samples"
          },
          {
            "name": "use-memo",
            "value": 0.590785,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 613.58 µs | 1,702.76 ops/sec | 1702 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.594632,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 619.3 µs | 1,687.45 ops/sec | 1686 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.507366,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 535.69 µs | 1,976.64 ops/sec | 1975 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 10.019536,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 10.15 ms | 99.67 ops/sec | 100 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.555798,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 601.61 µs | 1,793.08 ops/sec | 1788 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.43207,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 6.51 ms | 155.42 ops/sec | 156 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3448.544805,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 3.48 s | 0.29 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 24.15034,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 24.39 ms | 41.45 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "0.18.5",
          "candidateVersion": "0.18.5",
          "previousVersion": "0.18.4",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "d259671f11f31c1f037a2e05552f414d08b675e5d3c4a698f0b95ba8ad780312"
        }
      },
      {
        "commit": {
          "author": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "committer": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "id": "930f307e337c7a87e38e70eeef334592ec0fd7ed",
          "message": "Bump version to 0.18.6-rc.1",
          "timestamp": "2026-08-29T20:41:41Z",
          "tree_id": "1660cc3da400d674d292f0be8fa9fc3eefad8eda",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/930f307e337c7a87e38e70eeef334592ec0fd7ed"
        },
        "date": 1788037838041,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.864293,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 895.96 µs | 1,155.71 ops/sec | 1155 samples"
          },
          {
            "name": "card",
            "value": 0.39948,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 430.04 µs | 2,493.28 ops/sec | 2489 samples"
          },
          {
            "name": "consts",
            "value": 0.166146,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 197.86 µs | 5,897.46 ops/sec | 5862 samples"
          },
          {
            "name": "counter",
            "value": 1.507601,
            "unit": "ms",
            "range": "±0.09%",
            "extra": "p95 1.54 ms | 662.41 ops/sec | 663 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.011251,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 2.06 ms | 496.11 ops/sec | 496 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.24861,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 3.34 ms | 307.07 ops/sec | 308 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.773571,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 796.38 µs | 1,294.4 ops/sec | 1294 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.103592,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.13 ms | 904.98 ops/sec | 905 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.02658,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 1.05 ms | 972.39 ops/sec | 973 samples"
          },
          {
            "name": "page",
            "value": 0.500755,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 521.79 µs | 2,004.03 ops/sec | 2003 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.395402,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 3.51 ms | 293.95 ops/sec | 294 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.312359,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 3.38 ms | 301.72 ops/sec | 302 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.434321,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 1.47 ms | 696.02 ops/sec | 696 samples"
          },
          {
            "name": "typography",
            "value": 2.093518,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 2.13 ms | 477.02 ops/sec | 477 samples"
          },
          {
            "name": "use-memo",
            "value": 0.543872,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 565.24 µs | 1,846.48 ops/sec | 1845 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.540168,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 565.9 µs | 1,852.68 ops/sec | 1851 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.465709,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 493.98 µs | 2,147.85 ops/sec | 2145 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 10.103451,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 10.46 ms | 98.88 ops/sec | 99 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.489867,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 516.9 µs | 2,038.1 ops/sec | 2036 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.661643,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 6.84 ms | 150.18 ops/sec | 151 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3600.846678,
            "unit": "ms",
            "range": "±0.94%",
            "extra": "p95 3.66 s | 0.28 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 26.697505,
            "unit": "ms",
            "range": "±1.16%",
            "extra": "p95 27.5 ms | 37.64 ops/sec | 19 samples"
          }
        ],
        "release": {
          "ref": "0.18.6-rc.1",
          "candidateVersion": "0.18.6-rc.1",
          "previousVersion": "0.18.5",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "b4a344b0dda0fafde06706fc968c7d40e64f208b7bde43718a434d69a5385341"
        }
      },
      {
        "commit": {
          "author": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "committer": {
            "name": "Dwlad90",
            "email": "22377836+Dwlad90@users.noreply.github.com",
            "username": "Dwlad90"
          },
          "id": "876d0fcba65996edd011f6594c178fdf41fd87d8",
          "message": "Bump version to 0.18.6",
          "timestamp": "2026-08-31T15:10:45Z",
          "tree_id": "7a9705a58eec879f1e1ccc01660f524fe7806bd4",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/876d0fcba65996edd011f6594c178fdf41fd87d8"
        },
        "date": 1788194482996,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.874537,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 902.41 µs | 1,142.07 ops/sec | 1142 samples"
          },
          {
            "name": "card",
            "value": 0.412857,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 442.16 µs | 2,417.27 ops/sec | 2414 samples"
          },
          {
            "name": "consts",
            "value": 0.169071,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 198.72 µs | 5,802.69 ops/sec | 5771 samples"
          },
          {
            "name": "counter",
            "value": 1.527408,
            "unit": "ms",
            "range": "±0.09%",
            "extra": "p95 1.56 ms | 654.54 ops/sec | 655 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.027659,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 2.08 ms | 491.66 ops/sec | 492 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.291703,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 3.35 ms | 303.54 ops/sec | 304 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.782087,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 803.24 µs | 1,279.8 ops/sec | 1280 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.128631,
            "unit": "ms",
            "range": "±0.09%",
            "extra": "p95 1.16 ms | 884.83 ops/sec | 885 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.038421,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.06 ms | 962.03 ops/sec | 962 samples"
          },
          {
            "name": "page",
            "value": 0.513531,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 539.62 µs | 1,944.38 ops/sec | 1943 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.391093,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 3.45 ms | 294.8 ops/sec | 295 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.302338,
            "unit": "ms",
            "range": "±0.09%",
            "extra": "p95 3.35 ms | 302.59 ops/sec | 303 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.452329,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.49 ms | 687.53 ops/sec | 688 samples"
          },
          {
            "name": "typography",
            "value": 2.150933,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 2.19 ms | 464.32 ops/sec | 465 samples"
          },
          {
            "name": "use-memo",
            "value": 0.556884,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 579.01 µs | 1,802.98 ops/sec | 1802 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.531785,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 560.05 µs | 1,880.75 ops/sec | 1879 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.466501,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 495.8 µs | 2,139.33 ops/sec | 2137 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.760104,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 9.93 ms | 102.37 ops/sec | 103 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.505231,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 533.14 µs | 1,975.75 ops/sec | 1974 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.208035,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 6.28 ms | 160.95 ops/sec | 161 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3490.185119,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 3.52 s | 0.29 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 24.098118,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 24.47 ms | 41.52 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "0.18.6",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.5",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "d5f2a5157afb80abac00b4c23e6263be1f0bd0a8c2ba73173b383cb2ec2a34d6"
        }
      },
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
          "id": "68edbe2b055a843e26e1aa96b99ee7fe779895e0",
          "message": "perf(stylexswc/rs-compiler): give the musl target an allocator too",
          "timestamp": "2026-09-04T19:41:58+03:00",
          "tree_id": "8fa51bfacbe55aeb8d89e89976b99caf97c7dc99",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/68edbe2b055a843e26e1aa96b99ee7fe779895e0"
        },
        "date": 1788542504368,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.48776,
            "unit": "ms",
            "range": "±1.17%",
            "extra": "p95 513.83 µs | 2,041.58 ops/sec | 2028 samples"
          },
          {
            "name": "card",
            "value": 0.227847,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 255.02 µs | 4,310.05 ops/sec | 4300 samples"
          },
          {
            "name": "consts",
            "value": 0.119907,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 144.06 µs | 8,136 ops/sec | 8078 samples"
          },
          {
            "name": "counter",
            "value": 0.738215,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 757.65 µs | 1,356.12 ops/sec | 1355 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.915892,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 930.34 µs | 1,090.83 ops/sec | 1091 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.141706,
            "unit": "ms",
            "range": "±1.08%",
            "extra": "p95 2.31 ms | 460.81 ops/sec | 457 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.492599,
            "unit": "ms",
            "range": "±0.46%",
            "extra": "p95 524.59 µs | 2,022.34 ops/sec | 2014 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.762875,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 783.91 µs | 1,311.9 ops/sec | 1312 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.711486,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 734.85 µs | 1,407.5 ops/sec | 1407 samples"
          },
          {
            "name": "page",
            "value": 0.277384,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 303.79 µs | 3,545.03 ops/sec | 3539 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.755432,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 1.79 ms | 568.3 ops/sec | 569 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.715178,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 1.76 ms | 581.49 ops/sec | 582 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.679406,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 696.69 µs | 1,476.4 ops/sec | 1476 samples"
          },
          {
            "name": "typography",
            "value": 0.998504,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.02 ms | 998.75 ops/sec | 999 samples"
          },
          {
            "name": "use-memo",
            "value": 0.319522,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 345.2 µs | 3,080.67 ops/sec | 3077 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.325679,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 356.15 µs | 3,027.99 ops/sec | 3021 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.303465,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 332.66 µs | 3,244.74 ops/sec | 3238 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.478662,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 5.59 ms | 182.29 ops/sec | 183 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.242263,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 270.41 µs | 4,066.78 ops/sec | 4057 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.936945,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 3 ms | 339.96 ops/sec | 340 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 14.09385,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 14.46 ms | 70.85 ops/sec | 71 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2186.781928,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 2.22 s | 0.46 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 13.215811,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 13.49 ms | 75.55 ops/sec | 38 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.267942,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.3 ms | 787.21 ops/sec | 787 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.475157,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 1.52 ms | 675.67 ops/sec | 675 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.644931,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 668.97 µs | 1,552.68 ops/sec | 1551 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.809241,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 828.04 µs | 1,234.97 ops/sec | 1235 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.592333,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 613.33 µs | 1,691.36 ops/sec | 1690 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.710985,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 727.89 µs | 1,408.28 ops/sec | 1408 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.549923,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 1.58 ms | 643.88 ops/sec | 644 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.726707,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 746.59 µs | 1,377.37 ops/sec | 1377 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.451953,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 478.11 µs | 2,195.43 ops/sec | 2193 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.603622,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 624.79 µs | 1,660.21 ops/sec | 1660 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.314496,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 344.56 µs | 3,128.04 ops/sec | 3122 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.4402,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 467.43 µs | 2,248.24 ops/sec | 2246 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.440704,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 462.73 µs | 2,256.85 ops/sec | 2254 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.595117,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 620.34 µs | 1,680.71 ops/sec | 1680 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.494954,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 515.79 µs | 2,017.14 ops/sec | 2015 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.647423,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 667.22 µs | 1,545.81 ops/sec | 1545 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.314667,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 341.72 µs | 3,128.46 ops/sec | 3124 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.4504,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 476.85 µs | 2,200.44 ops/sec | 2199 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.19387,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 222.06 µs | 5,061.96 ops/sec | 5045 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.784585,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 803.9 µs | 1,274.73 ops/sec | 1275 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.440254,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 470.62 µs | 2,257.87 ops/sec | 2256 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.652829,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 672.09 µs | 1,533.53 ops/sec | 1533 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.635028,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 652.04 µs | 1,579.97 ops/sec | 1579 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.442471,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 467.77 µs | 2,254.58 ops/sec | 2251 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.482117,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 505.06 µs | 2,067.39 ops/sec | 2066 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.519794,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 542.62 µs | 1,924.92 ops/sec | 1923 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.711737,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 730.98 µs | 1,405.57 ops/sec | 1405 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.711813,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 730.26 µs | 1,408.01 ops/sec | 1408 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.526718,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 548.27 µs | 1,901.85 ops/sec | 1900 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.757579,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 783.04 µs | 1,318.19 ops/sec | 1318 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.519868,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 543.27 µs | 1,924.11 ops/sec | 1922 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.540109,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 616.59 µs | 1,840 ops/sec | 1836 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.611147,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 631.15 µs | 1,639.1 ops/sec | 1638 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.307146,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 334.81 µs | 3,208.47 ops/sec | 3202 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.528809,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 550.77 µs | 1,893.69 ops/sec | 1893 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.520824,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 544.95 µs | 1,921.12 ops/sec | 1919 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.514192,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 536.59 µs | 1,947.23 ops/sec | 1946 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.554908,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 579.52 µs | 1,799.12 ops/sec | 1789 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.739779,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 763.22 µs | 1,349.95 ops/sec | 1345 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 5.085203,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 5.23 ms | 196.49 ops/sec | 197 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 5.71673,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 5.86 ms | 174.67 ops/sec | 175 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.623118,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 644.53 µs | 1,605.35 ops/sec | 1604 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "89e582badff0ca48fc1ced6b930514f6473f91d1e0d95b9b9a52b68fb002497f"
        }
      },
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
          "id": "c52ce2e96a5bc20522bbd81706538e015947eb42",
          "message": "fix(stylexswc/rs-compiler): settle a path with one function everywhere",
          "timestamp": "2026-09-04T20:39:39+03:00",
          "tree_id": "8ab6a3c145bb412570c26af04e9e9a62c446fcd5",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/c52ce2e96a5bc20522bbd81706538e015947eb42"
        },
        "date": 1788546046412,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.482827,
            "unit": "ms",
            "range": "±1.12%",
            "extra": "p95 505.26 µs | 2,067.19 ops/sec | 2055 samples"
          },
          {
            "name": "card",
            "value": 0.224586,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 248.44 µs | 4,377.43 ops/sec | 4367 samples"
          },
          {
            "name": "consts",
            "value": 0.120537,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 143.17 µs | 8,132.84 ops/sec | 8080 samples"
          },
          {
            "name": "counter",
            "value": 0.730504,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 745.89 µs | 1,372.29 ops/sec | 1372 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.911374,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 929.06 µs | 1,096.2 ops/sec | 1096 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.120657,
            "unit": "ms",
            "range": "±1.36%",
            "extra": "p95 2.98 ms | 460.33 ops/sec | 455 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.486179,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 514.63 µs | 2,050.49 ops/sec | 2045 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.760995,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 782.99 µs | 1,315.45 ops/sec | 1315 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.711445,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 737.2 µs | 1,406.97 ops/sec | 1407 samples"
          },
          {
            "name": "page",
            "value": 0.272987,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 297.65 µs | 3,600.06 ops/sec | 3594 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.720465,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.76 ms | 580.19 ops/sec | 581 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.685364,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.72 ms | 592.44 ops/sec | 593 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.675952,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 692.86 µs | 1,485.1 ops/sec | 1485 samples"
          },
          {
            "name": "typography",
            "value": 1.005624,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.03 ms | 991.73 ops/sec | 992 samples"
          },
          {
            "name": "use-memo",
            "value": 0.315773,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 340.17 µs | 3,119.08 ops/sec | 3115 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.323513,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 351.72 µs | 3,049.13 ops/sec | 3044 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.299263,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 325.68 µs | 3,296.08 ops/sec | 3290 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.455548,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 5.55 ms | 183.04 ops/sec | 184 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.241082,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 266.78 µs | 4,084.78 ops/sec | 4074 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.921267,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 2.96 ms | 342.03 ops/sec | 342 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 14.037971,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 14.33 ms | 71.17 ops/sec | 72 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2077.586673,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 2.09 s | 0.48 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 13.274526,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 13.37 ms | 75.36 ops/sec | 38 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.279808,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.31 ms | 781.47 ops/sec | 782 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.475364,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 1.51 ms | 676.55 ops/sec | 676 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.650041,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 676.4 µs | 1,537.66 ops/sec | 1537 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.822711,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 838.52 µs | 1,216.95 ops/sec | 1217 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.606467,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 623.26 µs | 1,655.61 ops/sec | 1655 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.723577,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 739.09 µs | 1,385.64 ops/sec | 1386 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.563435,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.59 ms | 638.55 ops/sec | 639 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.733949,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 753.14 µs | 1,365.09 ops/sec | 1365 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.453881,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 476.56 µs | 2,187.3 ops/sec | 2186 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.614187,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 629.66 µs | 1,636.02 ops/sec | 1636 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.312981,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 338.38 µs | 3,142.98 ops/sec | 3139 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.441542,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 465.83 µs | 2,240.68 ops/sec | 2239 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.44913,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 471.97 µs | 2,211.98 ops/sec | 2210 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.603692,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 618.35 µs | 1,663.61 ops/sec | 1663 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.499322,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 518.3 µs | 2,004.83 ops/sec | 2004 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.658353,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 678.27 µs | 1,521.56 ops/sec | 1521 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.312558,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 339.84 µs | 3,147.25 ops/sec | 3143 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.450765,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 476.01 µs | 2,195.39 ops/sec | 2194 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.195496,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 216.87 µs | 5,039.91 ops/sec | 5028 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.798145,
            "unit": "ms",
            "range": "±0.09%",
            "extra": "p95 813.06 µs | 1,254.94 ops/sec | 1255 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.437286,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 459.96 µs | 2,266.24 ops/sec | 2265 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.658305,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 675.17 µs | 1,522.58 ops/sec | 1522 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.642274,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 657.54 µs | 1,563.26 ops/sec | 1563 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.446957,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 468.89 µs | 2,223.13 ops/sec | 2221 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.499718,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 516.62 µs | 2,006.58 ops/sec | 2006 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.537144,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 552.13 µs | 1,874.71 ops/sec | 1874 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.732709,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 752.9 µs | 1,367.12 ops/sec | 1367 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.732596,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 748.58 µs | 1,368.67 ops/sec | 1369 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.543339,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 557.97 µs | 1,852.7 ops/sec | 1852 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.779522,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 794.82 µs | 1,285.45 ops/sec | 1285 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.534352,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 552.54 µs | 1,881.31 ops/sec | 1880 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.552976,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 572.53 µs | 1,816.01 ops/sec | 1815 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.630759,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 647.95 µs | 1,592.6 ops/sec | 1592 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.315269,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 339.7 µs | 3,120.96 ops/sec | 3117 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.54344,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 559.85 µs | 1,850.39 ops/sec | 1849 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.533424,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 549.66 µs | 1,884.67 ops/sec | 1884 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.529137,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 547.11 µs | 1,900.45 ops/sec | 1899 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.573298,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 598.28 µs | 1,742.31 ops/sec | 1734 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.770602,
            "unit": "ms",
            "range": "±0.47%",
            "extra": "p95 792.15 µs | 1,295.95 ops/sec | 1292 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 5.052959,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 5.15 ms | 197.89 ops/sec | 198 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 5.74149,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 5.88 ms | 174.02 ops/sec | 174 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.637858,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 686.21 µs | 1,562.52 ops/sec | 1560 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "f0e63dd1d5bbb93ba026abaae5c669e32f59b35af921f55638c4701707f6f454"
        }
      },
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
          "id": "9995074fecb81a0b9ca7e7fb86040f9bab301964",
          "message": "docs: shorten the branch guidelines and correct their stale claims",
          "timestamp": "2026-09-05T01:02:16+03:00",
          "tree_id": "c0c225e1ad3092c1c55a32c4e69cabbb1543b920",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/9995074fecb81a0b9ca7e7fb86040f9bab301964"
        },
        "date": 1788562096910,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.470115,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 494.91 µs | 2,116.96 ops/sec | 2115 samples"
          },
          {
            "name": "card",
            "value": 0.213667,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 238.76 µs | 4,596.82 ops/sec | 4585 samples"
          },
          {
            "name": "consts",
            "value": 0.116849,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 143.16 µs | 8,337.39 ops/sec | 8274 samples"
          },
          {
            "name": "counter",
            "value": 0.719223,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 736.42 µs | 1,392.57 ops/sec | 1392 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.902621,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 918.52 µs | 1,106.39 ops/sec | 1106 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.091403,
            "unit": "ms",
            "range": "±0.92%",
            "extra": "p95 2.2 ms | 473.23 ops/sec | 470 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.47539,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 504.68 µs | 2,097.12 ops/sec | 2092 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.749689,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 770.21 µs | 1,335.61 ops/sec | 1335 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.703356,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 722.96 µs | 1,424.4 ops/sec | 1424 samples"
          },
          {
            "name": "page",
            "value": 0.263784,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 290.03 µs | 3,721.74 ops/sec | 3713 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.719534,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.75 ms | 580.55 ops/sec | 581 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.68364,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 1.73 ms | 592.98 ops/sec | 593 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.663975,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 685.45 µs | 1,510.07 ops/sec | 1509 samples"
          },
          {
            "name": "typography",
            "value": 0.989966,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 1.01 ms | 1,007.86 ops/sec | 1008 samples"
          },
          {
            "name": "use-memo",
            "value": 0.301709,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 327.48 µs | 3,259.63 ops/sec | 3255 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.313941,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 340.45 µs | 3,140.1 ops/sec | 3135 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.290406,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 316.99 µs | 3,390.43 ops/sec | 3384 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.449197,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 5.54 ms | 183.47 ops/sec | 184 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.231298,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 256.78 µs | 4,247.56 ops/sec | 4236 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.914762,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 2.97 ms | 342.7 ops/sec | 343 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 13.830766,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 14.09 ms | 72.17 ops/sec | 73 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2134.017003,
            "unit": "ms",
            "range": "±1.04%",
            "extra": "p95 2.21 s | 0.47 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 12.998607,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 13.19 ms | 76.87 ops/sec | 39 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.246745,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.28 ms | 801.54 ops/sec | 802 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.444924,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 1.48 ms | 690.4 ops/sec | 690 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.633773,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 662.45 µs | 1,579.16 ops/sec | 1578 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.792403,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 811.19 µs | 1,262.16 ops/sec | 1262 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.575314,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 595.22 µs | 1,742.63 ops/sec | 1742 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.692121,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 711.87 µs | 1,445.76 ops/sec | 1445 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.539001,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 1.57 ms | 647.97 ops/sec | 648 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.713058,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 733.46 µs | 1,403.02 ops/sec | 1403 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.434461,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 461.7 µs | 2,279.36 ops/sec | 2277 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.585753,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 604.65 µs | 1,712.36 ops/sec | 1712 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.300768,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 330.06 µs | 3,270.14 ops/sec | 3264 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.424737,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 448.02 µs | 2,335.01 ops/sec | 2333 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.434669,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 459.73 µs | 2,285.57 ops/sec | 2283 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.578755,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 597.41 µs | 1,729.85 ops/sec | 1729 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.479846,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 506.22 µs | 2,075.69 ops/sec | 2074 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.629834,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 646.89 µs | 1,591.48 ops/sec | 1591 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.30351,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 328.33 µs | 3,250 ops/sec | 3245 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.435566,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 459.64 µs | 2,280.88 ops/sec | 2279 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.186837,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 218.05 µs | 5,242.62 ops/sec | 5220 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.764339,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 782.19 µs | 1,309.47 ops/sec | 1309 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.418999,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 442.84 µs | 2,362.61 ops/sec | 2360 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.634312,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 650.03 µs | 1,581.66 ops/sec | 1581 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.6231,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 640.92 µs | 1,608.44 ops/sec | 1608 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.436781,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 461.06 µs | 2,278.65 ops/sec | 2276 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.481206,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 499.57 µs | 2,079.36 ops/sec | 2078 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.511268,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 530.41 µs | 1,961.05 ops/sec | 1960 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.698928,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 714.79 µs | 1,434.33 ops/sec | 1434 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.700288,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 715.96 µs | 1,431.92 ops/sec | 1432 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.517503,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 536.81 µs | 1,937.81 ops/sec | 1937 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.745425,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 761.44 µs | 1,343.4 ops/sec | 1343 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.505217,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 524.95 µs | 1,980.42 ops/sec | 1979 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.525583,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 542.68 µs | 1,910.41 ops/sec | 1909 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.603262,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 622.92 µs | 1,660.61 ops/sec | 1660 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.301117,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 328.5 µs | 3,270.4 ops/sec | 3265 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.514207,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 532.87 µs | 1,948.97 ops/sec | 1947 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.50508,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 525.29 µs | 1,982.83 ops/sec | 1981 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.500552,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 518.23 µs | 2,008.44 ops/sec | 2007 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.553713,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 577.92 µs | 1,803.16 ops/sec | 1794 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.739293,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 763.88 µs | 1,350.6 ops/sec | 1346 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 5.079278,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 5.2 ms | 196.83 ops/sec | 197 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 5.687276,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 5.83 ms | 175.63 ops/sec | 176 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.622065,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 639.9 µs | 1,611.91 ops/sec | 1611 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "04b5663da10e9d1a7ccf683d42309ccd8fd9bd04a9793d66b8c09695ab9db202"
        }
      },
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
          "id": "4eee173e9656c2f66863f89d1633771510640ce1",
          "message": "test(stylexswc/rs-compiler): run a generated child script from a file",
          "timestamp": "2026-09-05T14:51:43+03:00",
          "tree_id": "2160a1c93b6220db8a8676ae8498bfdcd6f81269",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/4eee173e9656c2f66863f89d1633771510640ce1"
        },
        "date": 1788611357250,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.473128,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 499.95 µs | 2,101.7 ops/sec | 2099 samples"
          },
          {
            "name": "card",
            "value": 0.221919,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 252.07 µs | 4,421.8 ops/sec | 4408 samples"
          },
          {
            "name": "consts",
            "value": 0.117892,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 144.23 µs | 8,286.24 ops/sec | 8224 samples"
          },
          {
            "name": "counter",
            "value": 0.725128,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 743.23 µs | 1,381.63 ops/sec | 1380 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.90647,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 927.16 µs | 1,101.42 ops/sec | 1101 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.113183,
            "unit": "ms",
            "range": "±1.15%",
            "extra": "p95 2.28 ms | 466.69 ops/sec | 463 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.493981,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 549.1 µs | 2,012.87 ops/sec | 2002 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.793203,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 819.98 µs | 1,260.42 ops/sec | 1260 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.733801,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 757.47 µs | 1,365.38 ops/sec | 1365 samples"
          },
          {
            "name": "page",
            "value": 0.284257,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 317.24 µs | 3,448.59 ops/sec | 3441 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.791805,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 1.83 ms | 557.08 ops/sec | 557 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.753332,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 1.8 ms | 569.08 ops/sec | 569 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.70624,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 724.81 µs | 1,420.63 ops/sec | 1420 samples"
          },
          {
            "name": "typography",
            "value": 1.047544,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 1.07 ms | 951.8 ops/sec | 952 samples"
          },
          {
            "name": "use-memo",
            "value": 0.324043,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 352.53 µs | 3,032.06 ops/sec | 3027 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.334788,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 363.66 µs | 2,942.34 ops/sec | 2936 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.312883,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 344.49 µs | 3,141.41 ops/sec | 3134 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.61477,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 5.82 ms | 177.64 ops/sec | 178 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.247409,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 275.5 µs | 3,978.46 ops/sec | 3968 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.055591,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 3.15 ms | 326.85 ops/sec | 327 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 15.999715,
            "unit": "ms",
            "range": "±0.78%",
            "extra": "p95 16.72 ms | 62.62 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2298.371281,
            "unit": "ms",
            "range": "±1.22%",
            "extra": "p95 2.36 s | 0.43 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 14.075443,
            "unit": "ms",
            "range": "±10.98%",
            "extra": "p95 14.76 ms | 69.49 ops/sec | 34 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.296844,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 1.34 ms | 768.85 ops/sec | 769 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.513579,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 1.56 ms | 659.24 ops/sec | 660 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.658807,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 688.19 µs | 1,515.9 ops/sec | 1515 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.835196,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 855.68 µs | 1,197.23 ops/sec | 1197 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.611093,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 632.4 µs | 1,639.6 ops/sec | 1638 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.736329,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 760.7 µs | 1,356.77 ops/sec | 1356 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.59131,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 1.62 ms | 626.97 ops/sec | 627 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.748509,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 766.47 µs | 1,338 ops/sec | 1338 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.458527,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 485.56 µs | 2,165.91 ops/sec | 2163 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.624669,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 652.36 µs | 1,599.89 ops/sec | 1599 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.318039,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 348.76 µs | 3,090.76 ops/sec | 3084 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.456503,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 481.93 µs | 2,181.18 ops/sec | 2179 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.450335,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 477.29 µs | 2,203.31 ops/sec | 2201 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.613957,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 639.81 µs | 1,627.94 ops/sec | 1627 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.503221,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 526.59 µs | 1,987.8 ops/sec | 1986 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.641582,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 666.34 µs | 1,557.53 ops/sec | 1556 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.301166,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 331.81 µs | 3,264.07 ops/sec | 3258 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.444032,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 467.5 µs | 2,242.15 ops/sec | 2240 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.18731,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 213.92 µs | 5,244.5 ops/sec | 5227 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.773594,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 800.58 µs | 1,291.85 ops/sec | 1291 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.426142,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 455.49 µs | 2,324.78 ops/sec | 2322 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.642406,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 667.73 µs | 1,555.72 ops/sec | 1555 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.627257,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 645.53 µs | 1,598.66 ops/sec | 1598 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.42999,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 456.02 µs | 2,303.71 ops/sec | 2301 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.485001,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 509.88 µs | 2,054.04 ops/sec | 2052 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.517018,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 540.66 µs | 1,935.75 ops/sec | 1934 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.715632,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 735.41 µs | 1,400.81 ops/sec | 1401 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.712904,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 734.2 µs | 1,404.96 ops/sec | 1405 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.523985,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 545.56 µs | 1,913.51 ops/sec | 1912 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.760522,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 784.04 µs | 1,315.84 ops/sec | 1316 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.514414,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 537.77 µs | 1,946.17 ops/sec | 1944 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.533398,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 553.83 µs | 1,880.96 ops/sec | 1880 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.609697,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 626.84 µs | 1,647.2 ops/sec | 1646 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.303738,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 330.66 µs | 3,237.62 ops/sec | 3233 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.521786,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 543.06 µs | 1,922.09 ops/sec | 1921 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.514409,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 535.27 µs | 1,948.8 ops/sec | 1947 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.510097,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 536.34 µs | 1,960.32 ops/sec | 1958 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.559971,
            "unit": "ms",
            "range": "±0.69%",
            "extra": "p95 587.71 µs | 1,781.25 ops/sec | 1770 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.755075,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 788.75 µs | 1,319.64 ops/sec | 1313 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 5.151667,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 5.32 ms | 193.66 ops/sec | 194 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 5.866356,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 6.05 ms | 170.21 ops/sec | 171 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.622894,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 643.31 µs | 1,609.92 ops/sec | 1609 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "6ab31f4cdcb12eb76789b018da3c23fa83ce99242ab6709330ab0083f99363b7"
        }
      },
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
          "id": "5903b7cb17f313e1545673ea54ce080feee6819c",
          "message": "test(stylexswc/rs-compiler): guard the parser path without guessing a stack",
          "timestamp": "2026-09-05T16:00:39+03:00",
          "tree_id": "cb8e49b07eae3cbb0d2b0f02f7f3166942099251",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/5903b7cb17f313e1545673ea54ce080feee6819c"
        },
        "date": 1788616038753,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.486823,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 513.11 µs | 2,046.79 ops/sec | 2044 samples"
          },
          {
            "name": "card",
            "value": 0.225021,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 254.13 µs | 4,356.15 ops/sec | 4343 samples"
          },
          {
            "name": "consts",
            "value": 0.118323,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 144.51 µs | 8,223.32 ops/sec | 8161 samples"
          },
          {
            "name": "counter",
            "value": 0.736281,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 758.19 µs | 1,358.68 ops/sec | 1358 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.910204,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 928.43 µs | 1,096.85 ops/sec | 1097 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.139993,
            "unit": "ms",
            "range": "±1.14%",
            "extra": "p95 2.31 ms | 460.83 ops/sec | 457 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.488899,
            "unit": "ms",
            "range": "±0.46%",
            "extra": "p95 520.37 µs | 2,036.71 ops/sec | 2028 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.760533,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 782.14 µs | 1,316.56 ops/sec | 1316 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.710688,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 730.39 µs | 1,411.27 ops/sec | 1411 samples"
          },
          {
            "name": "page",
            "value": 0.276454,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 306.72 µs | 3,545.05 ops/sec | 3537 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.732968,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 1.77 ms | 575.81 ops/sec | 576 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.701479,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 1.73 ms | 586.43 ops/sec | 587 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.675256,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 693.88 µs | 1,486.29 ops/sec | 1486 samples"
          },
          {
            "name": "typography",
            "value": 1.002118,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 1.02 ms | 994.61 ops/sec | 995 samples"
          },
          {
            "name": "use-memo",
            "value": 0.313887,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 344.5 µs | 3,126.49 ops/sec | 3121 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.3269,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 354.62 µs | 3,028.48 ops/sec | 3022 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.30051,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 329.47 µs | 3,273.1 ops/sec | 3266 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.484076,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 5.63 ms | 182.07 ops/sec | 183 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.238661,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 267.53 µs | 4,112.64 ops/sec | 4101 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.976639,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 3.05 ms | 335.4 ops/sec | 336 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 14.171196,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 14.52 ms | 70.55 ops/sec | 71 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2225.096125,
            "unit": "ms",
            "range": "±1.34%",
            "extra": "p95 2.32 s | 0.45 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 13.44757,
            "unit": "ms",
            "range": "±10.38%",
            "extra": "p95 13.84 ms | 72.99 ops/sec | 36 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.296109,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 1.34 ms | 770.41 ops/sec | 771 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.493865,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 1.55 ms | 667.4 ops/sec | 667 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.664995,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 693.07 µs | 1,503.02 ops/sec | 1502 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.83585,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 851.85 µs | 1,197.18 ops/sec | 1197 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.614777,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 632.33 µs | 1,631.01 ops/sec | 1630 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.731905,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 751.2 µs | 1,366.1 ops/sec | 1366 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.651582,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 1.68 ms | 604.4 ops/sec | 605 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.747297,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 765.76 µs | 1,339.05 ops/sec | 1339 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.458595,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 488.09 µs | 2,156.71 ops/sec | 2154 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.61811,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 635.58 µs | 1,622.01 ops/sec | 1622 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.311359,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 338.34 µs | 3,163.71 ops/sec | 3159 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.445362,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 467.77 µs | 2,229.25 ops/sec | 2228 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.449074,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 471.99 µs | 2,211.82 ops/sec | 2210 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.602849,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 626.47 µs | 1,658.97 ops/sec | 1658 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.496511,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 517.74 µs | 2,007.56 ops/sec | 2006 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.656087,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 673.92 µs | 1,527.11 ops/sec | 1527 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.311455,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 339.03 µs | 3,160.16 ops/sec | 3156 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.458794,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 485.04 µs | 2,167.12 ops/sec | 2165 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.194712,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 219.53 µs | 5,044.26 ops/sec | 5030 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.798154,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 815.95 µs | 1,252.79 ops/sec | 1253 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.442041,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 467.84 µs | 2,247.85 ops/sec | 2246 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.656982,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 676.61 µs | 1,523.59 ops/sec | 1523 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.64495,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 664.98 µs | 1,552.28 ops/sec | 1552 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.451545,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 475.7 µs | 2,201.64 ops/sec | 2199 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.497898,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 517.41 µs | 2,007.64 ops/sec | 2006 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.539176,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 561.37 µs | 1,857.48 ops/sec | 1856 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.741296,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 761.65 µs | 1,350.82 ops/sec | 1351 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.73968,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 764.6 µs | 1,351.07 ops/sec | 1351 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.547209,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 567.57 µs | 1,833.11 ops/sec | 1832 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.785818,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 806.11 µs | 1,272.82 ops/sec | 1273 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.53668,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 556.49 µs | 1,867.93 ops/sec | 1866 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.557068,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 578.52 µs | 1,798.24 ops/sec | 1797 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.634157,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 656.18 µs | 1,578.65 ops/sec | 1577 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.312035,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 339.81 µs | 3,152.81 ops/sec | 3148 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.538491,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 559.68 µs | 1,860.73 ops/sec | 1859 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.52882,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 548.73 µs | 1,896.36 ops/sec | 1895 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.523763,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 547.88 µs | 1,910.29 ops/sec | 1909 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.567252,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 599.2 µs | 1,755.45 ops/sec | 1744 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.758217,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 787.74 µs | 1,315.03 ops/sec | 1309 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 5.184929,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 5.32 ms | 192.67 ops/sec | 193 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 5.839952,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 5.99 ms | 170.81 ops/sec | 171 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.636122,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 653.73 µs | 1,576.74 ops/sec | 1576 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "aarch64-unknown-linux-gnu",
          "nativeSha256": "499c3ea7c5591dfe29c66aa5e0f8445520c52c46c188b2c042af272f03b38711"
        }
      }
    ]
  }
};
