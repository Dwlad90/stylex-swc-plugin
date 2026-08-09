window.BENCHMARK_DATA = {
  "lastUpdate": 1786287957693,
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
      }
    ]
  }
};
