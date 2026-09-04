window.BENCHMARK_DATA = {
  "lastUpdate": 1788562096910,
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
            "value": 1.13647,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 1.19 ms | 874.75 ops/sec | 874 samples"
          },
          {
            "name": "card",
            "value": 0.495204,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 549.07 µs | 2,006.7 ops/sec | 1999 samples"
          },
          {
            "name": "consts",
            "value": 0.215902,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 263.73 µs | 4,484.51 ops/sec | 4436 samples"
          },
          {
            "name": "counter",
            "value": 2.16049,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 2.24 ms | 460.75 ops/sec | 461 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.667726,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 2.74 ms | 374.31 ops/sec | 375 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.554285,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 2.62 ms | 390.31 ops/sec | 391 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.668517,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 697.55 µs | 1,494.35 ops/sec | 1489 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.386241,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 1.44 ms | 719.87 ops/sec | 720 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.279857,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.33 ms | 779.17 ops/sec | 779 samples"
          },
          {
            "name": "page",
            "value": 0.572719,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 607.91 µs | 1,745.23 ops/sec | 1743 samples"
          },
          {
            "name": "page-tsx",
            "value": 4.495456,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 4.59 ms | 222.01 ops/sec | 222 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 4.386452,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 4.5 ms | 227.59 ops/sec | 228 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.71787,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 1.78 ms | 580.03 ops/sec | 580 samples"
          },
          {
            "name": "typography",
            "value": 2.115556,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 2.17 ms | 472.13 ops/sec | 472 samples"
          },
          {
            "name": "use-memo",
            "value": 0.57872,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 624.86 µs | 1,731.56 ops/sec | 1726 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.462493,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 497.79 µs | 2,143.35 ops/sec | 2139 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.417679,
            "unit": "ms",
            "range": "±0.92%",
            "extra": "p95 464.25 µs | 2,339.03 ops/sec | 2301 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 21.609327,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 22.15 ms | 46.11 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.720384,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 776.16 µs | 1,381.63 ops/sec | 1380 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 7.75674,
            "unit": "ms",
            "range": "±0.70%",
            "extra": "p95 7.94 ms | 128.25 ops/sec | 129 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 6019.405378,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 6.04 s | 0.17 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 30.59259,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 30.92 ms | 32.66 ops/sec | 17 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "b8d68600284cd7f758524a5827ca769390aba731b0d78e316193144f5b23c4b1"
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
            "value": 0.947408,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 1 ms | 1,048.11 ops/sec | 1047 samples"
          },
          {
            "name": "card",
            "value": 0.40927,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 459.21 µs | 2,421.91 ops/sec | 2413 samples"
          },
          {
            "name": "consts",
            "value": 0.205396,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 256.49 µs | 4,715.23 ops/sec | 4676 samples"
          },
          {
            "name": "counter",
            "value": 2.042261,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 2.15 ms | 485.01 ops/sec | 485 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.578476,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 2.68 ms | 386.61 ops/sec | 387 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.586858,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 2.66 ms | 385.34 ops/sec | 386 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.660447,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 696.35 µs | 1,507.57 ops/sec | 1506 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.171125,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 1.26 ms | 851.99 ops/sec | 851 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.061474,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 1.13 ms | 941.27 ops/sec | 941 samples"
          },
          {
            "name": "page",
            "value": 0.638044,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 703.6 µs | 1,579.18 ops/sec | 1573 samples"
          },
          {
            "name": "page-tsx",
            "value": 4.395317,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 4.55 ms | 227.44 ops/sec | 228 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 4.337442,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 4.44 ms | 230.34 ops/sec | 231 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.567355,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 1.63 ms | 636.5 ops/sec | 637 samples"
          },
          {
            "name": "typography",
            "value": 2.364448,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 2.48 ms | 421.45 ops/sec | 422 samples"
          },
          {
            "name": "use-memo",
            "value": 0.696189,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 761.61 µs | 1,426.6 ops/sec | 1425 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.449663,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 500.88 µs | 2,236.25 ops/sec | 2224 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.405093,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 448.53 µs | 2,446.06 ops/sec | 2440 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 22.933809,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 23.52 ms | 43.54 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.557406,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 626.99 µs | 1,805.36 ops/sec | 1793 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 7.948678,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 8.28 ms | 125.33 ops/sec | 126 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5886.961645,
            "unit": "ms",
            "range": "±0.46%",
            "extra": "p95 5.96 s | 0.17 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 30.07471,
            "unit": "ms",
            "range": "±1.02%",
            "extra": "p95 31.32 ms | 32.94 ops/sec | 17 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "b8d68600284cd7f758524a5827ca769390aba731b0d78e316193144f5b23c4b1"
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
            "value": 1.140389,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.22 ms | 868.68 ops/sec | 868 samples"
          },
          {
            "name": "card",
            "value": 0.496244,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 550.19 µs | 2,001.47 ops/sec | 1993 samples"
          },
          {
            "name": "consts",
            "value": 0.219238,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 272.79 µs | 4,395.99 ops/sec | 4351 samples"
          },
          {
            "name": "counter",
            "value": 2.222941,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 2.32 ms | 448.38 ops/sec | 449 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.715358,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 2.81 ms | 367.47 ops/sec | 368 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.602688,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 2.71 ms | 383.18 ops/sec | 383 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.675318,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 713.73 µs | 1,477.65 ops/sec | 1475 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.422709,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.49 ms | 700.4 ops/sec | 700 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.316036,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 1.37 ms | 758.12 ops/sec | 758 samples"
          },
          {
            "name": "page",
            "value": 0.583206,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 630.02 µs | 1,702.63 ops/sec | 1696 samples"
          },
          {
            "name": "page-tsx",
            "value": 4.599867,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 4.7 ms | 217.11 ops/sec | 218 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 4.495783,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 4.63 ms | 221.89 ops/sec | 222 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.758395,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.83 ms | 567.53 ops/sec | 568 samples"
          },
          {
            "name": "typography",
            "value": 2.152368,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 2.24 ms | 462.75 ops/sec | 463 samples"
          },
          {
            "name": "use-memo",
            "value": 0.584038,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 630.81 µs | 1,715.65 ops/sec | 1710 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.472143,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 501.77 µs | 2,116.6 ops/sec | 2114 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.418572,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 453.04 µs | 2,369.16 ops/sec | 2365 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 21.539217,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 22.57 ms | 46.19 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.732449,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 784.98 µs | 1,358.17 ops/sec | 1356 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 7.907997,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 8.08 ms | 126.27 ops/sec | 127 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 6044.275527,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 6.08 s | 0.17 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 30.895575,
            "unit": "ms",
            "range": "±4.24%",
            "extra": "p95 40.92 ms | 31.74 ops/sec | 16 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "fa1e035cf74225cea5e05619080e6261051ff0ef0bcf5fa5bdd9ccbbab0ce461"
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
            "value": 0.737881,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 785.55 µs | 1,345.21 ops/sec | 1344 samples"
          },
          {
            "name": "card",
            "value": 0.316986,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 353.78 µs | 3,127.75 ops/sec | 3115 samples"
          },
          {
            "name": "consts",
            "value": 0.158493,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 186.73 µs | 6,144.12 ops/sec | 6106 samples"
          },
          {
            "name": "counter",
            "value": 1.600139,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 1.68 ms | 621.04 ops/sec | 621 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.996508,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 2.07 ms | 497.99 ops/sec | 498 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.009703,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 2.05 ms | 496.46 ops/sec | 497 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.517997,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 556.22 µs | 1,917.21 ops/sec | 1915 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.891932,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 933.15 µs | 1,118.59 ops/sec | 1118 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.828723,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 874.74 µs | 1,206.16 ops/sec | 1205 samples"
          },
          {
            "name": "page",
            "value": 0.499429,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 543.66 µs | 2,029.02 ops/sec | 2021 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.37174,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 3.46 ms | 295.55 ops/sec | 296 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.322541,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 3.41 ms | 300.44 ops/sec | 301 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.172007,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 1.21 ms | 850.96 ops/sec | 851 samples"
          },
          {
            "name": "typography",
            "value": 1.796704,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 1.86 ms | 555.1 ops/sec | 555 samples"
          },
          {
            "name": "use-memo",
            "value": 0.516574,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 566.63 µs | 1,920.31 ops/sec | 1917 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.320711,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 339.36 µs | 3,088.69 ops/sec | 3084 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.311317,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 329.52 µs | 3,183.53 ops/sec | 3179 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 17.694822,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 17.87 ms | 56.47 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.400691,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 435.06 µs | 2,492.57 ops/sec | 2486 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.063009,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 6.16 ms | 164.88 ops/sec | 165 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4709.862228,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 4.76 s | 0.21 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 22.918226,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 23.23 ms | 43.57 ops/sec | 22 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "fa1e035cf74225cea5e05619080e6261051ff0ef0bcf5fa5bdd9ccbbab0ce461"
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
            "value": 1.215418,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 1.3 ms | 815.83 ops/sec | 814 samples"
          },
          {
            "name": "card",
            "value": 0.527726,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 600.41 µs | 1,869.77 ops/sec | 1857 samples"
          },
          {
            "name": "consts",
            "value": 0.232719,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 289.7 µs | 4,162.39 ops/sec | 4125 samples"
          },
          {
            "name": "counter",
            "value": 2.371985,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 2.51 ms | 419.32 ops/sec | 419 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.911173,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 3.02 ms | 342.47 ops/sec | 343 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.905397,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 3 ms | 343.96 ops/sec | 344 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.746609,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 791.12 µs | 1,334.9 ops/sec | 1332 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.515881,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 1.59 ms | 656.8 ops/sec | 657 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.392628,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.47 ms | 714.09 ops/sec | 714 samples"
          },
          {
            "name": "page",
            "value": 0.614426,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 667.22 µs | 1,614.62 ops/sec | 1611 samples"
          },
          {
            "name": "page-tsx",
            "value": 4.817797,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 4.95 ms | 206.78 ops/sec | 207 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 4.730464,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 4.86 ms | 211.07 ops/sec | 212 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.866998,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.94 ms | 533.9 ops/sec | 534 samples"
          },
          {
            "name": "typography",
            "value": 2.320203,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 2.42 ms | 429.48 ops/sec | 430 samples"
          },
          {
            "name": "use-memo",
            "value": 0.628386,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 683.17 µs | 1,592.3 ops/sec | 1587 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.503941,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 536.75 µs | 1,980.98 ops/sec | 1977 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.470689,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 529.42 µs | 2,105.98 ops/sec | 2094 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 23.823956,
            "unit": "ms",
            "range": "±0.80%",
            "extra": "p95 24.57 ms | 41.95 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.760672,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 832.17 µs | 1,306.46 ops/sec | 1304 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 8.586037,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 8.99 ms | 115.86 ops/sec | 116 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 6265.596669,
            "unit": "ms",
            "range": "±0.90%",
            "extra": "p95 6.44 s | 0.16 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 35.400283,
            "unit": "ms",
            "range": "±1.14%",
            "extra": "p95 36.82 ms | 28.19 ops/sec | 15 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-rc.1",
          "candidateVersion": "0.18.4-rc.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "0978701563471fd61b2e56c7cc879aa06eeeaad060f15a8b8924e5569b63c5e2"
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
            "value": 0.906026,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 965.34 µs | 1,096.03 ops/sec | 1094 samples"
          },
          {
            "name": "card",
            "value": 0.339464,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 376.62 µs | 2,887.93 ops/sec | 2879 samples"
          },
          {
            "name": "consts",
            "value": 0.222675,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 273.47 µs | 4,348.29 ops/sec | 4312 samples"
          },
          {
            "name": "counter",
            "value": 1.385456,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 1.45 ms | 716.48 ops/sec | 714 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.876767,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 1.95 ms | 531.63 ops/sec | 532 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.704518,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 2.78 ms | 369.11 ops/sec | 369 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.683977,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 717.86 µs | 1,459.64 ops/sec | 1458 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.378923,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 1.45 ms | 718.78 ops/sec | 716 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.27552,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 1.34 ms | 779.4 ops/sec | 779 samples"
          },
          {
            "name": "page",
            "value": 0.452665,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 495.46 µs | 2,194.86 ops/sec | 2189 samples"
          },
          {
            "name": "page-tsx",
            "value": 4.017157,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 4.15 ms | 248.27 ops/sec | 249 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.636136,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 3.76 ms | 274.28 ops/sec | 275 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.145097,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.2 ms | 869.81 ops/sec | 869 samples"
          },
          {
            "name": "typography",
            "value": 1.714059,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 1.78 ms | 581.1 ops/sec | 581 samples"
          },
          {
            "name": "use-memo",
            "value": 0.536852,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 585.57 µs | 1,851.16 ops/sec | 1846 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.446934,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 485.64 µs | 2,229.3 ops/sec | 2225 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.42579,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 457.95 µs | 2,328.95 ops/sec | 2324 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 21.126429,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 21.61 ms | 47.2 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.35861,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 395.22 µs | 2,737.56 ops/sec | 2731 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.237401,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 6.49 ms | 159.33 ops/sec | 160 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5613.44296,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 5.67 s | 0.18 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 26.66806,
            "unit": "ms",
            "range": "±1.00%",
            "extra": "p95 28.46 ms | 37.32 ops/sec | 19 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-rc.2",
          "candidateVersion": "0.18.4-rc.2",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "1385ccd9a1aec91073fea156a6fdb253feaccdd36c39af87bb835d15ad07c1ec"
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
            "value": 1.071369,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 1.16 ms | 923.36 ops/sec | 922 samples"
          },
          {
            "name": "card",
            "value": 0.432842,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 496.26 µs | 2,286.58 ops/sec | 2272 samples"
          },
          {
            "name": "consts",
            "value": 0.232759,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 289.34 µs | 4,161.06 ops/sec | 4126 samples"
          },
          {
            "name": "counter",
            "value": 2.468293,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 2.65 ms | 404.83 ops/sec | 405 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.952787,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 3.14 ms | 337.35 ops/sec | 338 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.037455,
            "unit": "ms",
            "range": "±0.65%",
            "extra": "p95 3.28 ms | 332.37 ops/sec | 332 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.733628,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 796.23 µs | 1,361.05 ops/sec | 1357 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.303937,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 1.43 ms | 762.89 ops/sec | 760 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.189358,
            "unit": "ms",
            "range": "±1.01%",
            "extra": "p95 1.59 ms | 823.82 ops/sec | 813 samples"
          },
          {
            "name": "page",
            "value": 0.713961,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 791.99 µs | 1,419.64 ops/sec | 1413 samples"
          },
          {
            "name": "page-tsx",
            "value": 4.954271,
            "unit": "ms",
            "range": "±0.46%",
            "extra": "p95 5.19 ms | 201 ops/sec | 201 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 5.026144,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 5.27 ms | 198.7 ops/sec | 199 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.71064,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.8 ms | 583.15 ops/sec | 583 samples"
          },
          {
            "name": "typography",
            "value": 2.773533,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 2.99 ms | 358.66 ops/sec | 359 samples"
          },
          {
            "name": "use-memo",
            "value": 0.779469,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 851.41 µs | 1,274.12 ops/sec | 1272 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.522193,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 558.09 µs | 1,909.2 ops/sec | 1905 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.469576,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 511.13 µs | 2,138.24 ops/sec | 2130 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 24.006694,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 24.57 ms | 41.73 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.559083,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 606.81 µs | 1,804.26 ops/sec | 1797 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 8.734399,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 9.11 ms | 114.11 ops/sec | 115 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 6214.312353,
            "unit": "ms",
            "range": "±0.69%",
            "extra": "p95 6.33 s | 0.16 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 32.344416,
            "unit": "ms",
            "range": "±1.38%",
            "extra": "p95 33.94 ms | 30.82 ops/sec | 16 samples"
          }
        ],
        "release": {
          "ref": "0.18.4",
          "candidateVersion": "0.18.4",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "e7905834cf0c154b13e67107905dedf389976d4c971cd39c39e7effe71942f22"
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
            "value": 0.779909,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 842.15 µs | 1,268.32 ops/sec | 1264 samples"
          },
          {
            "name": "card",
            "value": 0.329253,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 368.96 µs | 3,008.74 ops/sec | 2998 samples"
          },
          {
            "name": "consts",
            "value": 0.165713,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 199.89 µs | 5,868.43 ops/sec | 5819 samples"
          },
          {
            "name": "counter",
            "value": 1.680841,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 1.79 ms | 589.94 ops/sec | 590 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.105879,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 2.21 ms | 472.35 ops/sec | 473 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.12694,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 2.28 ms | 467.21 ops/sec | 467 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.543873,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 573.08 µs | 1,832.1 ops/sec | 1830 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.955697,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 1.04 ms | 1,043.76 ops/sec | 1042 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.875129,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 931.29 µs | 1,140.7 ops/sec | 1140 samples"
          },
          {
            "name": "page",
            "value": 0.524885,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 574.39 µs | 1,928.73 ops/sec | 1920 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.542732,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 3.67 ms | 281.77 ops/sec | 282 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.524436,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 3.67 ms | 283.58 ops/sec | 284 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.250754,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 1.31 ms | 795.97 ops/sec | 796 samples"
          },
          {
            "name": "typography",
            "value": 1.931344,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 2.06 ms | 515.62 ops/sec | 515 samples"
          },
          {
            "name": "use-memo",
            "value": 0.544228,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 607.27 µs | 1,822.96 ops/sec | 1820 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.333018,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 353.3 µs | 2,976.73 ops/sec | 2971 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.328172,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 350.06 µs | 3,019.71 ops/sec | 3014 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 18.393861,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 19.06 ms | 54.31 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.448197,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 507.46 µs | 2,243.72 ops/sec | 2227 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.437929,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 6.67 ms | 155.34 ops/sec | 156 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4891.680284,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 4.92 s | 0.2 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.941582,
            "unit": "ms",
            "range": "±1.13%",
            "extra": "p95 24.68 ms | 41.82 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "0.18.5-rc.1",
          "candidateVersion": "0.18.5-rc.1",
          "previousVersion": "0.18.4",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "dbc22a13fccf8bb2a90f858f44efd0e94354ad0dab2722872b4cdf779295f2b0"
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
            "value": 1.123185,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 1.19 ms | 883.06 ops/sec | 882 samples"
          },
          {
            "name": "card",
            "value": 0.49085,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 544.63 µs | 2,020.83 ops/sec | 2012 samples"
          },
          {
            "name": "consts",
            "value": 0.212356,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 254.65 µs | 4,557.8 ops/sec | 4529 samples"
          },
          {
            "name": "counter",
            "value": 2.180457,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 2.27 ms | 456.63 ops/sec | 457 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.679888,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 2.75 ms | 372.27 ops/sec | 373 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.558356,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 2.64 ms | 389.95 ops/sec | 390 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.644777,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 672.39 µs | 1,551.31 ops/sec | 1550 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.390403,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 1.45 ms | 716.54 ops/sec | 717 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.282742,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 1.34 ms | 775.5 ops/sec | 775 samples"
          },
          {
            "name": "page",
            "value": 0.667495,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 713.87 µs | 1,490.85 ops/sec | 1489 samples"
          },
          {
            "name": "page-tsx",
            "value": 5.179404,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 5.29 ms | 193.11 ops/sec | 194 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 5.016525,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 5.11 ms | 199.17 ops/sec | 200 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.944747,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 2.01 ms | 512.84 ops/sec | 513 samples"
          },
          {
            "name": "typography",
            "value": 2.526822,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 2.61 ms | 394.7 ops/sec | 395 samples"
          },
          {
            "name": "use-memo",
            "value": 0.713245,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 766.03 µs | 1,395.69 ops/sec | 1394 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.462462,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 499.98 µs | 2,147.19 ops/sec | 2144 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.405776,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 446.44 µs | 2,422.09 ops/sec | 2411 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 22.542535,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 22.72 ms | 44.37 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.717729,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 767.83 µs | 1,388.39 ops/sec | 1387 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 8.502351,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 8.67 ms | 117.44 ops/sec | 118 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5829.919168,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 5.87 s | 0.17 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 29.526043,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 29.91 ms | 33.86 ops/sec | 17 samples"
          }
        ],
        "release": {
          "ref": "0.18.5",
          "candidateVersion": "0.18.5",
          "previousVersion": "0.18.4",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "fbd95f3fa24ace3cd5e047ded74894b6e0087e3ca925a4979805427d157fc58c"
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
            "value": 0.755398,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 801.13 µs | 1,315.89 ops/sec | 1314 samples"
          },
          {
            "name": "card",
            "value": 0.322832,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 361.97 µs | 3,072.28 ops/sec | 3062 samples"
          },
          {
            "name": "consts",
            "value": 0.160411,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 200.33 µs | 6,043.63 ops/sec | 5985 samples"
          },
          {
            "name": "counter",
            "value": 1.636015,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 1.73 ms | 606.69 ops/sec | 607 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.034275,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 2.12 ms | 488.48 ops/sec | 489 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.039173,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 2.09 ms | 488.82 ops/sec | 489 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.523348,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 551.43 µs | 1,904.41 ops/sec | 1903 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.915259,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 975.35 µs | 1,091.93 ops/sec | 1091 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.844062,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 894.08 µs | 1,183.47 ops/sec | 1183 samples"
          },
          {
            "name": "page",
            "value": 0.512082,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 557.79 µs | 1,975.7 ops/sec | 1967 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.477905,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 3.6 ms | 287.14 ops/sec | 288 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.416457,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 3.54 ms | 292.1 ops/sec | 292 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.219507,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 1.28 ms | 816.52 ops/sec | 816 samples"
          },
          {
            "name": "typography",
            "value": 1.845315,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 1.93 ms | 539.96 ops/sec | 540 samples"
          },
          {
            "name": "use-memo",
            "value": 0.530038,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 584.41 µs | 1,872.12 ops/sec | 1869 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.320578,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 341.45 µs | 3,086.68 ops/sec | 3080 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.315736,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 336.87 µs | 3,133.88 ops/sec | 3129 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 18.534319,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 19.03 ms | 53.83 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.433033,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 489.21 µs | 2,325.16 ops/sec | 2307 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.143989,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 6.26 ms | 162.58 ops/sec | 163 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4792.681036,
            "unit": "ms",
            "range": "±1.23%",
            "extra": "p95 4.96 s | 0.21 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 25.389361,
            "unit": "ms",
            "range": "±1.05%",
            "extra": "p95 26.13 ms | 39.48 ops/sec | 20 samples"
          }
        ],
        "release": {
          "ref": "0.18.6-rc.1",
          "candidateVersion": "0.18.6-rc.1",
          "previousVersion": "0.18.5",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "243e2b8bb0ff8a97464a9dd8a97417e5387e97e6677e1cde1ad85e6116d496d4"
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
            "value": 0.844752,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 905.28 µs | 1,175.91 ops/sec | 1174 samples"
          },
          {
            "name": "card",
            "value": 0.328542,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 389.51 µs | 2,994.42 ops/sec | 2975 samples"
          },
          {
            "name": "consts",
            "value": 0.113979,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 137.86 µs | 8,525.39 ops/sec | 8435 samples"
          },
          {
            "name": "counter",
            "value": 1.700689,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 1.91 ms | 584.8 ops/sec | 583 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.960507,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 2.05 ms | 511.01 ops/sec | 511 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.06015,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 2.11 ms | 484.15 ops/sec | 484 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.521618,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 574.15 µs | 1,903.35 ops/sec | 1899 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.187927,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 1.27 ms | 849.91 ops/sec | 848 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.070989,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 1.15 ms | 955.1 ops/sec | 950 samples"
          },
          {
            "name": "page",
            "value": 0.421757,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 466.34 µs | 2,353.98 ops/sec | 2349 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.20441,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 3.47 ms | 310.1 ops/sec | 310 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.152172,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 3.28 ms | 316.31 ops/sec | 317 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.27515,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 1.34 ms | 781.88 ops/sec | 782 samples"
          },
          {
            "name": "typography",
            "value": 1.6701,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.76 ms | 596.3 ops/sec | 596 samples"
          },
          {
            "name": "use-memo",
            "value": 0.469683,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 524.41 µs | 2,144.32 ops/sec | 2131 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.294734,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 334.92 µs | 3,360.73 ops/sec | 3345 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.277156,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 312.05 µs | 3,569.62 ops/sec | 3555 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 14.081817,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 14.42 ms | 70.88 ops/sec | 71 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.478258,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 531.25 µs | 2,073.66 ops/sec | 2069 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.683349,
            "unit": "ms",
            "range": "±0.95%",
            "extra": "p95 6.14 ms | 178.44 ops/sec | 178 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4148.723197,
            "unit": "ms",
            "range": "±0.88%",
            "extra": "p95 4.21 s | 0.24 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 21.83367,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 22.53 ms | 45.56 ops/sec | 23 samples"
          }
        ],
        "release": {
          "ref": "0.18.6",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.5",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "54cbffaf2d957ee9724deb4e66451fdb34b35b849000923144f2908cada51869"
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
            "value": 0.740898,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 850.96 µs | 1,334.46 ops/sec | 1331 samples"
          },
          {
            "name": "card",
            "value": 0.337439,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 407.19 µs | 2,913.96 ops/sec | 2891 samples"
          },
          {
            "name": "consts",
            "value": 0.187143,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 235.89 µs | 5,168.9 ops/sec | 5117 samples"
          },
          {
            "name": "counter",
            "value": 0.924811,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 988.8 µs | 1,072.86 ops/sec | 1070 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.167242,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 1.24 ms | 851.48 ops/sec | 851 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.577439,
            "unit": "ms",
            "range": "±1.12%",
            "extra": "p95 2.78 ms | 382.49 ops/sec | 380 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.581498,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 637.79 µs | 1,703.25 ops/sec | 1697 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.202515,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.3 ms | 824.63 ops/sec | 824 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.100291,
            "unit": "ms",
            "range": "±0.76%",
            "extra": "p95 1.22 ms | 889.5 ops/sec | 883 samples"
          },
          {
            "name": "page",
            "value": 0.347244,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 393.64 µs | 2,842.94 ops/sec | 2835 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.244067,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 2.32 ms | 444.32 ops/sec | 445 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.207411,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 2.31 ms | 450.56 ops/sec | 450 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.042884,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.13 ms | 948.7 ops/sec | 948 samples"
          },
          {
            "name": "typography",
            "value": 1.674227,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.78 ms | 591.2 ops/sec | 591 samples"
          },
          {
            "name": "use-memo",
            "value": 0.435558,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 502.68 µs | 2,263.64 ops/sec | 2255 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.349156,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 388.1 µs | 2,829.29 ops/sec | 2821 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.358111,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 407.03 µs | 2,760.91 ops/sec | 2753 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.480601,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 5.6 ms | 182.26 ops/sec | 183 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.27838,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 321.16 µs | 3,519.83 ops/sec | 3508 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.656659,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 3.79 ms | 272.64 ops/sec | 273 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 23.154592,
            "unit": "ms",
            "range": "±2.35%",
            "extra": "p95 24.56 ms | 42.53 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4161.806965,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 4.19 s | 0.24 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 20.696878,
            "unit": "ms",
            "range": "±14.38%",
            "extra": "p95 21.06 ms | 46.9 ops/sec | 24 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.704358,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 1.8 ms | 582.23 ops/sec | 582 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.975331,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 2.09 ms | 503.23 ops/sec | 503 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.743106,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 794.88 µs | 1,336.42 ops/sec | 1334 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.942262,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 986.43 µs | 1,058.45 ops/sec | 1058 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.678134,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 710.18 µs | 1,469.44 ops/sec | 1467 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.816703,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 854.44 µs | 1,218.74 ops/sec | 1217 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.909712,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 1.97 ms | 521.59 ops/sec | 522 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.879418,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 933.46 µs | 1,129.25 ops/sec | 1128 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.621914,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 691.35 µs | 1,596.33 ops/sec | 1594 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.796383,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 853.13 µs | 1,247.52 ops/sec | 1247 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.441382,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 507.9 µs | 2,241.1 ops/sec | 2234 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.590992,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 647.93 µs | 1,679.73 ops/sec | 1677 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.547757,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 606.08 µs | 1,807.26 ops/sec | 1803 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.723382,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 772.42 µs | 1,374.59 ops/sec | 1373 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.598633,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 653.23 µs | 1,655 ops/sec | 1652 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.78167,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 831.37 µs | 1,271.63 ops/sec | 1271 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.455548,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 527.61 µs | 2,172.07 ops/sec | 2167 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.614508,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 673.85 µs | 1,616.65 ops/sec | 1615 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.265946,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 306.05 µs | 3,685.84 ops/sec | 3672 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.938336,
            "unit": "ms",
            "range": "±0.74%",
            "extra": "p95 1.03 ms | 1,047.33 ops/sec | 1039 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.588546,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 646.32 µs | 1,687.24 ops/sec | 1684 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.77901,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 826.84 µs | 1,275.41 ops/sec | 1274 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.771334,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 822.16 µs | 1,286.19 ops/sec | 1283 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.553295,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 608.18 µs | 1,792.28 ops/sec | 1789 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.600001,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 652.54 µs | 1,655.92 ops/sec | 1654 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.825206,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 904.91 µs | 1,200.26 ops/sec | 1199 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 1.052568,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.13 ms | 941.53 ops/sec | 941 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 1.051739,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 1.13 ms | 942.57 ops/sec | 942 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.831732,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 916.75 µs | 1,190.19 ops/sec | 1188 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 1.106051,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.18 ms | 897.12 ops/sec | 897 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.904702,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 998.8 µs | 1,093.69 ops/sec | 1092 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.842723,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 920.57 µs | 1,176.81 ops/sec | 1176 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.735551,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 782.63 µs | 1,353.04 ops/sec | 1351 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.444542,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 512.51 µs | 2,226.92 ops/sec | 2221 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.82812,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 909.89 µs | 1,197.99 ops/sec | 1197 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.82102,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 903.63 µs | 1,206.6 ops/sec | 1205 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.820544,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 901.87 µs | 1,207.62 ops/sec | 1206 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.697733,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 745.86 µs | 1,425.07 ops/sec | 1419 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.918647,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 973.86 µs | 1,079.4 ops/sec | 1076 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 6.513125,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 6.68 ms | 153.33 ops/sec | 154 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 7.17851,
            "unit": "ms",
            "range": "±1.45%",
            "extra": "p95 7.93 ms | 137.38 ops/sec | 137 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.748951,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 800.61 µs | 1,327.18 ops/sec | 1326 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "2dc5b4789fe8692c2fcb1d3b9e83fdb3f5faa9a502c9977ff697ab974933058f"
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
            "value": 0.761571,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 829.56 µs | 1,304.42 ops/sec | 1303 samples"
          },
          {
            "name": "card",
            "value": 0.316885,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 392.4 µs | 3,067.38 ops/sec | 3035 samples"
          },
          {
            "name": "consts",
            "value": 0.197742,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 246.83 µs | 4,879.36 ops/sec | 4835 samples"
          },
          {
            "name": "counter",
            "value": 0.944565,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 1.01 ms | 1,048.13 ops/sec | 1047 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.192195,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.26 ms | 832.96 ops/sec | 832 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.555172,
            "unit": "ms",
            "range": "±1.24%",
            "extra": "p95 2.86 ms | 384.74 ops/sec | 382 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.577335,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 615.52 µs | 1,726.38 ops/sec | 1722 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.199675,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 1.28 ms | 824.96 ops/sec | 825 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.104536,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 1.18 ms | 896.24 ops/sec | 895 samples"
          },
          {
            "name": "page",
            "value": 0.325341,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 375.76 µs | 3,006.68 ops/sec | 2992 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.248455,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 2.38 ms | 442.24 ops/sec | 442 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.197529,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 2.3 ms | 452.61 ops/sec | 453 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.052047,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 1.13 ms | 940.82 ops/sec | 940 samples"
          },
          {
            "name": "typography",
            "value": 1.651468,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 1.74 ms | 600.1 ops/sec | 600 samples"
          },
          {
            "name": "use-memo",
            "value": 0.423376,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 476.18 µs | 2,338.16 ops/sec | 2332 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.347503,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 381.25 µs | 2,822.71 ops/sec | 2815 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.353554,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 400.31 µs | 2,781.01 ops/sec | 2773 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.449094,
            "unit": "ms",
            "range": "±0.90%",
            "extra": "p95 5.66 ms | 182.19 ops/sec | 182 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.273294,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 310.34 µs | 3,559.63 ops/sec | 3550 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.710322,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 3.81 ms | 268.96 ops/sec | 269 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 23.353398,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 24.62 ms | 42.57 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4361.231654,
            "unit": "ms",
            "range": "±0.46%",
            "extra": "p95 4.41 s | 0.23 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 21.091968,
            "unit": "ms",
            "range": "±0.95%",
            "extra": "p95 22.28 ms | 47.23 ops/sec | 24 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.803068,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 1.9 ms | 551.25 ops/sec | 551 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 2.000368,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 2.09 ms | 497.51 ops/sec | 497 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.773013,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 819.47 µs | 1,287.8 ops/sec | 1286 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.960455,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 1.01 ms | 1,034.63 ops/sec | 1034 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.699815,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 744.26 µs | 1,423.5 ops/sec | 1421 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.830741,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 885.23 µs | 1,195.54 ops/sec | 1193 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.953391,
            "unit": "ms",
            "range": "±0.74%",
            "extra": "p95 2.02 ms | 508.99 ops/sec | 508 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.896214,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 939.12 µs | 1,108.08 ops/sec | 1107 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.650653,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 707.09 µs | 1,530.59 ops/sec | 1528 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.82,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 880.17 µs | 1,209.05 ops/sec | 1207 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.456844,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 515.02 µs | 2,167.29 ops/sec | 2160 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.597182,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 649.65 µs | 1,665.17 ops/sec | 1663 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.560022,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 608.62 µs | 1,777.29 ops/sec | 1774 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.730052,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 772.54 µs | 1,364.18 ops/sec | 1363 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.615426,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 665.38 µs | 1,617.87 ops/sec | 1615 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.789543,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 831.81 µs | 1,259.95 ops/sec | 1259 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.482898,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 552.41 µs | 2,044.66 ops/sec | 2034 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.631236,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 686.08 µs | 1,573.57 ops/sec | 1570 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.272491,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 321.38 µs | 3,585.02 ops/sec | 3570 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.950676,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1 ms | 1,045.53 ops/sec | 1045 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.583887,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 634.38 µs | 1,703.63 ops/sec | 1701 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.790556,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 831.99 µs | 1,258.96 ops/sec | 1258 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.788842,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 830.08 µs | 1,260.45 ops/sec | 1259 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.560859,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 609.77 µs | 1,772.05 ops/sec | 1768 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.609596,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 654.86 µs | 1,633.97 ops/sec | 1631 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.833767,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 907.01 µs | 1,185.94 ops/sec | 1184 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 1.05389,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 1.12 ms | 939.73 ops/sec | 939 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 1.050203,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 1.12 ms | 943.58 ops/sec | 943 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.842107,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 911.83 µs | 1,177.02 ops/sec | 1176 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 1.107782,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.18 ms | 893.99 ops/sec | 893 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.906984,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 1.04 ms | 1,078.7 ops/sec | 1073 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.847517,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 913.48 µs | 1,169.85 ops/sec | 1168 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.751843,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 792.27 µs | 1,325.76 ops/sec | 1324 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.456443,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 514.7 µs | 2,171.35 ops/sec | 2164 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.832995,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 912.64 µs | 1,188.62 ops/sec | 1187 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.82988,
            "unit": "ms",
            "range": "±0.47%",
            "extra": "p95 907.19 µs | 1,188.84 ops/sec | 1184 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.826283,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 898.75 µs | 1,195.47 ops/sec | 1192 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.713421,
            "unit": "ms",
            "range": "±0.46%",
            "extra": "p95 755.78 µs | 1,395.36 ops/sec | 1391 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.934075,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 983.88 µs | 1,063.1 ops/sec | 1061 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 6.96064,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 7.21 ms | 142.87 ops/sec | 143 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 7.419216,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 7.93 ms | 134.09 ops/sec | 134 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.759006,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 807.09 µs | 1,308.99 ops/sec | 1306 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "2d40c13076f5cf555d7e6bfb4977c3847c0c944279676f25a5b1aaeba5c16eda"
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
            "value": 0.755262,
            "unit": "ms",
            "range": "±1.51%",
            "extra": "p95 822.15 µs | 1,312.78 ops/sec | 1302 samples"
          },
          {
            "name": "card",
            "value": 0.313862,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 373.53 µs | 3,121.49 ops/sec | 3108 samples"
          },
          {
            "name": "consts",
            "value": 0.194463,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 241.47 µs | 4,964.03 ops/sec | 4932 samples"
          },
          {
            "name": "counter",
            "value": 0.932183,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 970.48 µs | 1,068.46 ops/sec | 1068 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.177832,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 1.23 ms | 845.39 ops/sec | 845 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.502919,
            "unit": "ms",
            "range": "±0.74%",
            "extra": "p95 2.75 ms | 395.43 ops/sec | 394 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.584203,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 624.82 µs | 1,706.7 ops/sec | 1703 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.20331,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.28 ms | 825.19 ops/sec | 825 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.099566,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 1.17 ms | 900.98 ops/sec | 901 samples"
          },
          {
            "name": "page",
            "value": 0.326631,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 371.08 µs | 3,008.56 ops/sec | 3001 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.268151,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 2.33 ms | 439.75 ops/sec | 440 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.226778,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 2.29 ms | 447.72 ops/sec | 448 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.047318,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 1.11 ms | 947.28 ops/sec | 947 samples"
          },
          {
            "name": "typography",
            "value": 1.650155,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 1.74 ms | 601.24 ops/sec | 601 samples"
          },
          {
            "name": "use-memo",
            "value": 0.418853,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 472.49 µs | 2,349.89 ops/sec | 2343 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.354257,
            "unit": "ms",
            "range": "±0.91%",
            "extra": "p95 389.49 µs | 2,751.55 ops/sec | 2708 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.350275,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 393.89 µs | 2,804.83 ops/sec | 2799 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.244285,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 5.33 ms | 190.24 ops/sec | 191 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.274173,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 318.55 µs | 3,551.97 ops/sec | 3541 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.69556,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 3.76 ms | 270.24 ops/sec | 271 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 22.933686,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 24.45 ms | 43.15 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4211.32273,
            "unit": "ms",
            "range": "±3.24%",
            "extra": "p95 4.6 s | 0.23 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 20.11735,
            "unit": "ms",
            "range": "±0.69%",
            "extra": "p95 20.56 ms | 49.53 ops/sec | 25 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.761757,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 1.84 ms | 564.74 ops/sec | 565 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.959882,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 2.04 ms | 507.57 ops/sec | 508 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.755162,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 795.41 µs | 1,320.07 ops/sec | 1318 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.931753,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 968.5 µs | 1,069.36 ops/sec | 1069 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.675739,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 706.07 µs | 1,479.44 ops/sec | 1478 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.801551,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 833.56 µs | 1,243.11 ops/sec | 1242 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.918014,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 1.98 ms | 519.42 ops/sec | 520 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.883563,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 934.33 µs | 1,123.28 ops/sec | 1122 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.634768,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 688.22 µs | 1,568.8 ops/sec | 1567 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.807371,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 856.77 µs | 1,232.3 ops/sec | 1231 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.447338,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 506.34 µs | 2,208.87 ops/sec | 2202 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.591306,
            "unit": "ms",
            "range": "±0.80%",
            "extra": "p95 642.04 µs | 1,674.76 ops/sec | 1658 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.544309,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 590.65 µs | 1,828.48 ops/sec | 1825 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.720333,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 765.82 µs | 1,380.27 ops/sec | 1378 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.602919,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 647.99 µs | 1,652.02 ops/sec | 1650 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.785725,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 829.36 µs | 1,264.2 ops/sec | 1262 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.472194,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 536.83 µs | 2,095.05 ops/sec | 2085 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.621458,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 673.28 µs | 1,601.92 ops/sec | 1600 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.268338,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 318.21 µs | 3,632.17 ops/sec | 3618 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.937434,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 977.58 µs | 1,061.32 ops/sec | 1061 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.585997,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 635.16 µs | 1,699.76 ops/sec | 1697 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.781442,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 820.78 µs | 1,274.62 ops/sec | 1274 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.770132,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 809.57 µs | 1,295.16 ops/sec | 1295 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.552985,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 601.32 µs | 1,798.05 ops/sec | 1795 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.604056,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 651.29 µs | 1,644.17 ops/sec | 1642 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.822439,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 886.92 µs | 1,208.08 ops/sec | 1207 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 1.038063,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 1.1 ms | 956.2 ops/sec | 956 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 1.037306,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.1 ms | 955.92 ops/sec | 956 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.829022,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 892.11 µs | 1,197.53 ops/sec | 1197 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 1.088973,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 1.15 ms | 910.9 ops/sec | 910 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.889059,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 968.44 µs | 1,113.42 ops/sec | 1112 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.836185,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 901.73 µs | 1,184.5 ops/sec | 1183 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.739524,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 782.97 µs | 1,345.04 ops/sec | 1343 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.449081,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 508.23 µs | 2,204.44 ops/sec | 2199 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.820145,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 883.73 µs | 1,209.53 ops/sec | 1208 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.816148,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 881.54 µs | 1,215.51 ops/sec | 1214 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.811469,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 877.11 µs | 1,220.94 ops/sec | 1219 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.715239,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 760.46 µs | 1,386.21 ops/sec | 1377 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.926003,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 963.7 µs | 1,074.16 ops/sec | 1071 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 6.939672,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 7.35 ms | 143.83 ops/sec | 144 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 7.368425,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 7.62 ms | 135.17 ops/sec | 136 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.758449,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 803.21 µs | 1,310.85 ops/sec | 1309 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "bf35d1169305a104834312b9318a6bb05629e183d431f151f733002039e5a66b"
        }
      }
    ]
  }
};
