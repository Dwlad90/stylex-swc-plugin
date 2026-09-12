window.BENCHMARK_DATA = {
  "lastUpdate": 1789252262389,
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
            "value": 0.569855,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 650.81 µs | 1,735.5 ops/sec | 1731 samples"
          },
          {
            "name": "card",
            "value": 0.255829,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 292.78 µs | 3,843.92 ops/sec | 3826 samples"
          },
          {
            "name": "consts",
            "value": 0.144064,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 172.43 µs | 6,736.65 ops/sec | 6683 samples"
          },
          {
            "name": "counter",
            "value": 0.716277,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 764.96 µs | 1,386.38 ops/sec | 1384 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.902598,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 960.03 µs | 1,094.89 ops/sec | 1092 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.056689,
            "unit": "ms",
            "range": "±1.13%",
            "extra": "p95 2.22 ms | 479.42 ops/sec | 475 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.442176,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 483.65 µs | 2,239.19 ops/sec | 2231 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.929278,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 1.02 ms | 1,055.1 ops/sec | 1048 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.85655,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 935.34 µs | 1,156.41 ops/sec | 1155 samples"
          },
          {
            "name": "page",
            "value": 0.265093,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 293.09 µs | 3,706.69 ops/sec | 3697 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.737516,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 1.8 ms | 571.98 ops/sec | 572 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.711232,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.77 ms | 582.01 ops/sec | 582 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.815829,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 901.97 µs | 1,212.93 ops/sec | 1211 samples"
          },
          {
            "name": "typography",
            "value": 1.29553,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 1.4 ms | 762.6 ops/sec | 762 samples"
          },
          {
            "name": "use-memo",
            "value": 0.334185,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 374.69 µs | 2,952.45 ops/sec | 2945 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.268583,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 305.52 µs | 3,661.07 ops/sec | 3648 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.275208,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 313.67 µs | 3,572.36 ops/sec | 3559 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.435737,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 4.66 ms | 225.33 ops/sec | 226 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.216235,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 259.88 µs | 4,489.29 ops/sec | 4452 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.96426,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 3.12 ms | 336 ops/sec | 336 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 19.317681,
            "unit": "ms",
            "range": "±0.82%",
            "extra": "p95 20.48 ms | 51.7 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3470.378328,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 3.51 s | 0.29 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 16.654566,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 17.14 ms | 60.09 ops/sec | 31 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.371214,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 1.48 ms | 721.16 ops/sec | 720 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.537951,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 1.63 ms | 646.21 ops/sec | 646 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.572499,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 618.64 µs | 1,731.11 ops/sec | 1727 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.725025,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 829.24 µs | 1,359.38 ops/sec | 1353 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.522019,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 554.97 µs | 1,902.92 ops/sec | 1900 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.631838,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 662.93 µs | 1,575.78 ops/sec | 1574 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.477316,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.53 ms | 674.01 ops/sec | 674 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.681211,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 730.75 µs | 1,455.28 ops/sec | 1453 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.47895,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 543.16 µs | 2,065.97 ops/sec | 2061 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.616524,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 674.27 µs | 1,609.96 ops/sec | 1608 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.339744,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 405.87 µs | 2,898.13 ops/sec | 2884 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.456177,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 510.3 µs | 2,172.84 ops/sec | 2169 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.423458,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 475.79 µs | 2,337.33 ops/sec | 2331 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.558859,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 601.98 µs | 1,777.41 ops/sec | 1775 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.465561,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 515.15 µs | 2,127.36 ops/sec | 2121 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.602153,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 647.43 µs | 1,649.05 ops/sec | 1647 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.35088,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 407.38 µs | 2,811.27 ops/sec | 2802 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.473332,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 545.55 µs | 2,074.91 ops/sec | 2062 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.205805,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 239.55 µs | 4,748.5 ops/sec | 4727 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.727849,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 797.07 µs | 1,354.3 ops/sec | 1347 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.452741,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 505.23 µs | 2,189.33 ops/sec | 2185 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.604236,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 647.69 µs | 1,645 ops/sec | 1643 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.597837,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 644.74 µs | 1,658.53 ops/sec | 1655 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.427389,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 481.93 µs | 2,311.73 ops/sec | 2300 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.46547,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 514.45 µs | 2,128.88 ops/sec | 2124 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.634171,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 735.13 µs | 1,542.71 ops/sec | 1531 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.811683,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 878.25 µs | 1,223.2 ops/sec | 1222 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.811873,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 882.45 µs | 1,221.97 ops/sec | 1221 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.638662,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 721.48 µs | 1,548.21 ops/sec | 1544 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.851823,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 914.82 µs | 1,164.83 ops/sec | 1164 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.69457,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 789.15 µs | 1,424.99 ops/sec | 1422 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.644931,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 724.63 µs | 1,535.1 ops/sec | 1532 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.564768,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 613.36 µs | 1,756.18 ops/sec | 1753 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.342418,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 391.03 µs | 2,882.27 ops/sec | 2872 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.635853,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 719 µs | 1,557.69 ops/sec | 1555 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.630565,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 713.22 µs | 1,570.25 ops/sec | 1567 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.627305,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 707.24 µs | 1,579 ops/sec | 1576 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.541082,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 590.89 µs | 1,828.43 ops/sec | 1819 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.71479,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 759.42 µs | 1,387.3 ops/sec | 1381 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 5.260996,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 5.39 ms | 189.92 ops/sec | 190 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 5.840039,
            "unit": "ms",
            "range": "±1.03%",
            "extra": "p95 6.37 ms | 168.79 ops/sec | 169 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.577817,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 648.53 µs | 1,705.28 ops/sec | 1696 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "bdbb5148bfe2f288fcaa69a10ee61e33d62dd4bdbf6ba563c497d7f75f730968"
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
            "value": 0.758141,
            "unit": "ms",
            "range": "±1.59%",
            "extra": "p95 830.49 µs | 1,307.73 ops/sec | 1296 samples"
          },
          {
            "name": "card",
            "value": 0.316189,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 367.79 µs | 3,105.31 ops/sec | 3094 samples"
          },
          {
            "name": "consts",
            "value": 0.195791,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 244.21 µs | 4,930.33 ops/sec | 4891 samples"
          },
          {
            "name": "counter",
            "value": 0.933288,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 974.22 µs | 1,065.8 ops/sec | 1065 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.171214,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 1.22 ms | 849.6 ops/sec | 850 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.614994,
            "unit": "ms",
            "range": "±1.16%",
            "extra": "p95 2.79 ms | 377.96 ops/sec | 375 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.576788,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 620.95 µs | 1,723.79 ops/sec | 1718 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.215773,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.31 ms | 815.02 ops/sec | 815 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.105799,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 1.18 ms | 895.97 ops/sec | 896 samples"
          },
          {
            "name": "page",
            "value": 0.326714,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 372.37 µs | 3,005.48 ops/sec | 2996 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.273617,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 2.37 ms | 438 ops/sec | 438 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.232046,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 2.31 ms | 446.35 ops/sec | 447 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.057352,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.13 ms | 936.91 ops/sec | 936 samples"
          },
          {
            "name": "typography",
            "value": 1.662078,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 1.76 ms | 596.67 ops/sec | 597 samples"
          },
          {
            "name": "use-memo",
            "value": 0.423051,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 475.64 µs | 2,342.15 ops/sec | 2336 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.354,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 395.2 µs | 2,774.51 ops/sec | 2767 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.355348,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 402.77 µs | 2,770.43 ops/sec | 2762 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.540624,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 5.72 ms | 179.74 ops/sec | 180 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.277375,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 322.77 µs | 3,516.2 ops/sec | 3503 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.844581,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 3.94 ms | 259.42 ops/sec | 260 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 24.147829,
            "unit": "ms",
            "range": "±0.47%",
            "extra": "p95 24.85 ms | 41.27 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4362.134619,
            "unit": "ms",
            "range": "±1.41%",
            "extra": "p95 4.48 s | 0.23 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 20.931088,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 21.39 ms | 47.66 ops/sec | 24 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.785941,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 1.9 ms | 555.76 ops/sec | 556 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.97748,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 2.07 ms | 503.42 ops/sec | 504 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.766461,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 822.54 µs | 1,293.19 ops/sec | 1290 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.947204,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 1.01 ms | 1,045.75 ops/sec | 1045 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.68139,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 718.6 µs | 1,461.94 ops/sec | 1460 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.809731,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 855.56 µs | 1,226.44 ops/sec | 1224 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.936954,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 2.01 ms | 514.4 ops/sec | 515 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.885485,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 953.03 µs | 1,118.9 ops/sec | 1117 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.636371,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 706.34 µs | 1,559.89 ops/sec | 1556 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.805696,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 862.4 µs | 1,232.7 ops/sec | 1231 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.451916,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 509.77 µs | 2,195.6 ops/sec | 2189 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.590386,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 638.91 µs | 1,686.33 ops/sec | 1684 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.555319,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 613.26 µs | 1,787.34 ops/sec | 1783 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.721113,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 769.13 µs | 1,379.82 ops/sec | 1378 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.609717,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 657.87 µs | 1,630.71 ops/sec | 1628 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.794775,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 848.09 µs | 1,249.33 ops/sec | 1248 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.472485,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 549.52 µs | 2,083.21 ops/sec | 2070 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.616377,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 673.91 µs | 1,611.66 ops/sec | 1609 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.267677,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 319.15 µs | 3,645.1 ops/sec | 3629 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.945731,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 986.36 µs | 1,051.46 ops/sec | 1051 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.587382,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 637.81 µs | 1,693.64 ops/sec | 1691 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.783704,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 829.37 µs | 1,267.99 ops/sec | 1267 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.780338,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 822.72 µs | 1,275.36 ops/sec | 1274 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.555031,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 606.93 µs | 1,788.32 ops/sec | 1784 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.608071,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 659.02 µs | 1,631.79 ops/sec | 1629 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.829561,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 899.57 µs | 1,196.02 ops/sec | 1195 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 1.048355,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 1.12 ms | 945.02 ops/sec | 944 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 1.048085,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 1.12 ms | 945.2 ops/sec | 945 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.835317,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 907.08 µs | 1,187.37 ops/sec | 1186 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 1.107713,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 1.18 ms | 896.13 ops/sec | 896 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.902702,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 985.77 µs | 1,096.88 ops/sec | 1096 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.845834,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 916.71 µs | 1,173.6 ops/sec | 1173 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.754611,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 800.55 µs | 1,318.59 ops/sec | 1317 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.453921,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 513.83 µs | 2,187.65 ops/sec | 2182 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.831035,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 909.44 µs | 1,191.43 ops/sec | 1189 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.826967,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 908.3 µs | 1,196.3 ops/sec | 1195 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.818972,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 898.95 µs | 1,210.09 ops/sec | 1209 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.716948,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 774.95 µs | 1,383.55 ops/sec | 1377 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.944078,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 1.01 ms | 1,050.89 ops/sec | 1047 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 7.137954,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 7.77 ms | 139.43 ops/sec | 140 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 7.594865,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 8.09 ms | 131.08 ops/sec | 131 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.759208,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 809.14 µs | 1,310.71 ops/sec | 1309 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "c20a4c5681f573eb08ddb7c8b5de886ed48d36e1a650fff1e3edff9db29c2a86"
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
          "id": "0b203aba05058db79e47689141749eb991ff7ae2",
          "message": "Bump version to 0.19.0-rc.1",
          "timestamp": "2026-09-06T16:58:39Z",
          "tree_id": "f09f5f55ebad1b93e68849bc7815d7720b912750",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/0b203aba05058db79e47689141749eb991ff7ae2"
        },
        "date": 1788716250315,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.590153,
            "unit": "ms",
            "range": "±1.46%",
            "extra": "p95 720.66 µs | 1,655.82 ops/sec | 1632 samples"
          },
          {
            "name": "card",
            "value": 0.208712,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 255.46 µs | 4,680.46 ops/sec | 4634 samples"
          },
          {
            "name": "consts",
            "value": 0.108881,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 135.36 µs | 9,113.26 ops/sec | 8954 samples"
          },
          {
            "name": "counter",
            "value": 0.819009,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 865.33 µs | 1,214.11 ops/sec | 1213 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.020902,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 1.08 ms | 970.69 ops/sec | 970 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.06202,
            "unit": "ms",
            "range": "±1.00%",
            "extra": "p95 2.38 ms | 477.07 ops/sec | 474 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.441906,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 496.24 µs | 2,238.57 ops/sec | 2227 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.908065,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 1.03 ms | 1,086.37 ops/sec | 1084 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.85169,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 942.78 µs | 1,165.04 ops/sec | 1163 samples"
          },
          {
            "name": "page",
            "value": 0.259128,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 309.53 µs | 3,808.47 ops/sec | 3784 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.874124,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 1.95 ms | 531.52 ops/sec | 532 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.807441,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 1.86 ms | 550.14 ops/sec | 550 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.825943,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 919.25 µs | 1,200.17 ops/sec | 1198 samples"
          },
          {
            "name": "typography",
            "value": 1.258981,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 1.39 ms | 783.78 ops/sec | 783 samples"
          },
          {
            "name": "use-memo",
            "value": 0.339165,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 391.12 µs | 2,921.1 ops/sec | 2909 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.233685,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 267.84 µs | 4,207.56 ops/sec | 4184 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.236125,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 278.19 µs | 4,178.04 ops/sec | 4154 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.316355,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 4.53 ms | 230.38 ops/sec | 231 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.174364,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 207.18 µs | 5,647.72 ops/sec | 5603 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.957676,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 3.07 ms | 336.96 ops/sec | 337 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 17.258133,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 17.76 ms | 58.01 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2972.211761,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 3 s | 0.34 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 14.982238,
            "unit": "ms",
            "range": "±5.65%",
            "extra": "p95 15.63 ms | 65.49 ops/sec | 33 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.415838,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 1.52 ms | 700.92 ops/sec | 700 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.599921,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 1.68 ms | 620.38 ops/sec | 620 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.639187,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 688.54 µs | 1,553.62 ops/sec | 1550 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.799849,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 849.71 µs | 1,241.17 ops/sec | 1240 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.568328,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 612.44 µs | 1,745.85 ops/sec | 1740 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.692944,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 737.28 µs | 1,434.22 ops/sec | 1431 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.566579,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.64 ms | 634.19 ops/sec | 634 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.712901,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 770.56 µs | 1,388.5 ops/sec | 1385 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.51525,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 575.36 µs | 1,928.02 ops/sec | 1922 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.68268,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 738.88 µs | 1,457.9 ops/sec | 1455 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.329699,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 397.35 µs | 2,996.53 ops/sec | 2975 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.471865,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 531.8 µs | 2,102.71 ops/sec | 2096 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.428758,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 481.44 µs | 2,313.28 ops/sec | 2304 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.595026,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 643.7 µs | 1,669.97 ops/sec | 1666 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.504993,
            "unit": "ms",
            "range": "±0.74%",
            "extra": "p95 668.93 µs | 1,926.24 ops/sec | 1897 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.670531,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 721.2 µs | 1,483.22 ops/sec | 1480 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.334181,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 411.74 µs | 2,942.3 ops/sec | 2920 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.482567,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 543.52 µs | 2,053.84 ops/sec | 2047 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.158379,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 187.58 µs | 6,237.05 ops/sec | 6177 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.800559,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 851.46 µs | 1,238.76 ops/sec | 1237 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.457446,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 516.49 µs | 2,167.09 ops/sec | 2160 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.67426,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 731.42 µs | 1,469.78 ops/sec | 1466 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.622383,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 670.96 µs | 1,599.08 ops/sec | 1596 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.440463,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 497.56 µs | 2,247.51 ops/sec | 2233 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.491923,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 544.53 µs | 2,016.53 ops/sec | 2010 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.649732,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 731.88 µs | 1,525.04 ops/sec | 1521 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.853981,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 1.1 ms | 1,139.86 ops/sec | 1131 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.855544,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 930.32 µs | 1,157.82 ops/sec | 1156 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.655903,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 772.71 µs | 1,497.08 ops/sec | 1487 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.922965,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 1 ms | 1,076.03 ops/sec | 1074 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.676219,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 785.73 µs | 1,457.39 ops/sec | 1453 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.682606,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 771.91 µs | 1,450.32 ops/sec | 1444 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.628568,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 680.92 µs | 1,579.44 ops/sec | 1576 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.323263,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 374.85 µs | 3,057.71 ops/sec | 3042 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.648687,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 732.35 µs | 1,524.24 ops/sec | 1521 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.64202,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 730.87 µs | 1,539.62 ops/sec | 1535 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.654519,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 738.09 µs | 1,518.22 ops/sec | 1514 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.610392,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 661.2 µs | 1,627.08 ops/sec | 1621 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.806374,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 856.41 µs | 1,232.09 ops/sec | 1229 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 5.164532,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 5.34 ms | 193.25 ops/sec | 194 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 5.799728,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 6.06 ms | 171.84 ops/sec | 172 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.649438,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 702.12 µs | 1,526.92 ops/sec | 1524 samples"
          }
        ],
        "release": {
          "ref": "0.19.0-rc.1",
          "candidateVersion": "0.19.0-rc.1",
          "previousVersion": "0.18.6",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "dbc939bb44e1a813a0d592f632a60ec0eaaca7cea3b54b65af59ff098c0cec96"
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
          "id": "540a775cc17766c385c35643972be27d2e91e437",
          "message": "Bump version to 0.19.0-rc.2",
          "timestamp": "2026-09-08T18:50:45Z",
          "tree_id": "6d9101e52c16e158f8249ad15c6b6172009033d4",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/540a775cc17766c385c35643972be27d2e91e437"
        },
        "date": 1788896340456,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.749349,
            "unit": "ms",
            "range": "±1.36%",
            "extra": "p95 848.64 µs | 1,317.18 ops/sec | 1305 samples"
          },
          {
            "name": "card",
            "value": 0.308296,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 373.16 µs | 3,167.78 ops/sec | 3147 samples"
          },
          {
            "name": "consts",
            "value": 0.192644,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 241.07 µs | 5,001.28 ops/sec | 4954 samples"
          },
          {
            "name": "counter",
            "value": 0.857496,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 941.67 µs | 1,154.38 ops/sec | 1152 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.990088,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.05 ms | 1,001.25 ops/sec | 1000 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.438942,
            "unit": "ms",
            "range": "±1.15%",
            "extra": "p95 2.76 ms | 401.84 ops/sec | 399 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.559244,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 605.22 µs | 1,774.91 ops/sec | 1767 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.155687,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 1.26 ms | 856.21 ops/sec | 855 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.071646,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 1.17 ms | 921.52 ops/sec | 920 samples"
          },
          {
            "name": "page",
            "value": 0.317041,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 367.6 µs | 3,091.31 ops/sec | 3079 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.023417,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 2.13 ms | 491.47 ops/sec | 492 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.983928,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 2.06 ms | 502.11 ops/sec | 502 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.987743,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.07 ms | 1,001.16 ops/sec | 1000 samples"
          },
          {
            "name": "typography",
            "value": 1.563187,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 1.7 ms | 632.36 ops/sec | 632 samples"
          },
          {
            "name": "use-memo",
            "value": 0.415916,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 484.83 µs | 2,365.1 ops/sec | 2343 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.340545,
            "unit": "ms",
            "range": "±0.55%",
            "extra": "p95 387.12 µs | 2,863.39 ops/sec | 2831 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.341852,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 394.43 µs | 2,859.96 ops/sec | 2848 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.056135,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 5.6 ms | 195.65 ops/sec | 196 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.261548,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 308.41 µs | 3,714.18 ops/sec | 3697 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.31686,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 2.46 ms | 428.5 ops/sec | 429 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 23.504675,
            "unit": "ms",
            "range": "±1.09%",
            "extra": "p95 25.94 ms | 42.02 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4150.318696,
            "unit": "ms",
            "range": "±0.71%",
            "extra": "p95 4.25 s | 0.24 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 16.08757,
            "unit": "ms",
            "range": "±8.30%",
            "extra": "p95 18.63 ms | 60.21 ops/sec | 31 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.371509,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 1.49 ms | 720.71 ops/sec | 720 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.645806,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 1.78 ms | 601.49 ops/sec | 600 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.724372,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 780.15 µs | 1,368.55 ops/sec | 1365 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.913735,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 965.88 µs | 1,087.59 ops/sec | 1086 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.670807,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 706.16 µs | 1,485.14 ops/sec | 1482 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.79859,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 840.22 µs | 1,245.12 ops/sec | 1243 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.89956,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 1.97 ms | 524.24 ops/sec | 524 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.847696,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 901.92 µs | 1,171.14 ops/sec | 1169 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.629024,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 700.35 µs | 1,577 ops/sec | 1572 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.799777,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 861.25 µs | 1,242.15 ops/sec | 1241 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.432487,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 493.99 µs | 2,283.42 ops/sec | 2275 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.57327,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 629.78 µs | 1,732.71 ops/sec | 1729 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.505533,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 561.61 µs | 1,960.67 ops/sec | 1955 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.677389,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 759.99 µs | 1,457.99 ops/sec | 1454 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.561909,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 619.07 µs | 1,768.19 ops/sec | 1763 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.746694,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 806.38 µs | 1,328.95 ops/sec | 1326 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.467473,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 537.55 µs | 2,118.19 ops/sec | 2111 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.617893,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 685.73 µs | 1,609.58 ops/sec | 1606 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.265947,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 334.47 µs | 3,648.39 ops/sec | 3614 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.909267,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 968.34 µs | 1,092.47 ops/sec | 1092 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.579111,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 644.08 µs | 1,713.94 ops/sec | 1710 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.749444,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 806.36 µs | 1,326.03 ops/sec | 1324 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.672961,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 737.16 µs | 1,472.16 ops/sec | 1468 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.520281,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 582.92 µs | 1,901.82 ops/sec | 1895 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.566818,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 624.26 µs | 1,749.93 ops/sec | 1745 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.795505,
            "unit": "ms",
            "range": "±0.80%",
            "extra": "p95 900.97 µs | 1,234.46 ops/sec | 1222 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 1.059678,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 1.18 ms | 928.37 ops/sec | 926 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 1.015173,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 1.1 ms | 973.67 ops/sec | 973 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.803303,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 903.85 µs | 1,226.92 ops/sec | 1222 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 1.058915,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.14 ms | 935.69 ops/sec | 935 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.871997,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 965.93 µs | 1,132.91 ops/sec | 1131 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.81487,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 892.06 µs | 1,215.39 ops/sec | 1214 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.734281,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 791.86 µs | 1,351.22 ops/sec | 1349 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.442596,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 506.99 µs | 2,238.65 ops/sec | 2231 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.794453,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 894.54 µs | 1,237.53 ops/sec | 1233 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.793542,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 889.06 µs | 1,244.2 ops/sec | 1241 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.788652,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 885.64 µs | 1,253.73 ops/sec | 1251 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.705708,
            "unit": "ms",
            "range": "±0.70%",
            "extra": "p95 764.6 µs | 1,404.1 ops/sec | 1395 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.931359,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 992.13 µs | 1,064.74 ops/sec | 1061 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 4.842197,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 5.23 ms | 204.82 ops/sec | 205 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 5.561616,
            "unit": "ms",
            "range": "±0.70%",
            "extra": "p95 6.24 ms | 177.98 ops/sec | 178 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.729282,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 788.97 µs | 1,359.88 ops/sec | 1357 samples"
          }
        ],
        "release": {
          "ref": "0.19.0-rc.2",
          "candidateVersion": "0.19.0-rc.2",
          "previousVersion": "0.18.6",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "62638f0b797921f5824f8cca8a2569f03367a12401a0853d2530bc0ccfba4a54"
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
          "id": "bcd300ba56c2db6213f13a243b305e1007235d3b",
          "message": "feat(rs-compiler/benchmark): measure each subject in its own process",
          "timestamp": "2026-09-11T01:27:20+03:00",
          "tree_id": "9bacb7c6612c08e161f541e130bb83fddc0f8775",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/bcd300ba56c2db6213f13a243b305e1007235d3b"
        },
        "date": 1789162488843,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.750428,
            "unit": "ms",
            "range": "±1.35%",
            "extra": "p95 817.87 µs | 1,322.39 ops/sec | 1313 samples"
          },
          {
            "name": "card",
            "value": 0.314656,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 372.6 µs | 3,112.86 ops/sec | 3099 samples"
          },
          {
            "name": "consts",
            "value": 0.193691,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 236.65 µs | 4,977.98 ops/sec | 4946 samples"
          },
          {
            "name": "counter",
            "value": 0.874739,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 913.76 µs | 1,139.3 ops/sec | 1139 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.004491,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 1.04 ms | 991.23 ops/sec | 991 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.382008,
            "unit": "ms",
            "range": "±0.81%",
            "extra": "p95 2.6 ms | 414.97 ops/sec | 413 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.570828,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 607.14 µs | 1,746.07 ops/sec | 1742 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.174488,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 1.25 ms | 844.86 ops/sec | 845 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.085863,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 1.16 ms | 912.89 ops/sec | 912 samples"
          },
          {
            "name": "page",
            "value": 0.325626,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 372.51 µs | 3,014.52 ops/sec | 3005 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.027743,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 2.11 ms | 491.34 ops/sec | 492 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.978952,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 2.04 ms | 503.35 ops/sec | 504 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.995003,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 1.07 ms | 995.6 ops/sec | 995 samples"
          },
          {
            "name": "typography",
            "value": 1.555186,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 1.65 ms | 637.5 ops/sec | 637 samples"
          },
          {
            "name": "use-memo",
            "value": 0.420213,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 473.14 µs | 2,351.98 ops/sec | 2346 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.340383,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 377.18 µs | 2,879.76 ops/sec | 2873 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.344186,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 390.97 µs | 2,849.6 ops/sec | 2840 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.426674,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 4.54 ms | 225.18 ops/sec | 226 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.268169,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 311.38 µs | 3,618.71 ops/sec | 3606 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.314646,
            "unit": "ms",
            "range": "±0.84%",
            "extra": "p95 2.45 ms | 427.36 ops/sec | 426 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 22.842015,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 23.22 ms | 43.59 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4134.764584,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 4.2 s | 0.24 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 16.401903,
            "unit": "ms",
            "range": "±0.74%",
            "extra": "p95 16.86 ms | 60.8 ops/sec | 31 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.377496,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 1.48 ms | 718.56 ops/sec | 718 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.642604,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.73 ms | 604.29 ops/sec | 604 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.730761,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 776.55 µs | 1,360.31 ops/sec | 1358 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.915159,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 955.27 µs | 1,088.19 ops/sec | 1088 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.675283,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 711.67 µs | 1,475.78 ops/sec | 1474 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.804728,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 832.55 µs | 1,239.42 ops/sec | 1239 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.903335,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 1.95 ms | 523.22 ops/sec | 523 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.853179,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 897.31 µs | 1,163.93 ops/sec | 1162 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.63339,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 695.14 µs | 1,567.96 ops/sec | 1565 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.80807,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 864.15 µs | 1,229.44 ops/sec | 1228 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.440365,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 502.38 µs | 2,247.14 ops/sec | 2240 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.582445,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 633.39 µs | 1,708.68 ops/sec | 1705 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.516136,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 565.01 µs | 1,925.33 ops/sec | 1920 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.688857,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 735.79 µs | 1,444.07 ops/sec | 1442 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.572296,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 645.57 µs | 1,726.08 ops/sec | 1721 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.7493,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 794.62 µs | 1,326.64 ops/sec | 1324 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.469294,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 538.78 µs | 2,107.66 ops/sec | 2101 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.623791,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 683.88 µs | 1,594.37 ops/sec | 1591 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.262128,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 310.11 µs | 3,704.62 ops/sec | 3687 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.901965,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 951.14 µs | 1,100.98 ops/sec | 1100 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.579153,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 635.86 µs | 1,715.37 ops/sec | 1712 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.754705,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 807.69 µs | 1,316.67 ops/sec | 1315 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.677757,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 747.81 µs | 1,454.22 ops/sec | 1444 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.522663,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 582.59 µs | 1,894.41 ops/sec | 1888 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.570282,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 621.62 µs | 1,743.04 ops/sec | 1739 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.804679,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 887.77 µs | 1,230.78 ops/sec | 1229 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 1.061517,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 1.14 ms | 933.45 ops/sec | 933 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 1.016884,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 1.09 ms | 973.44 ops/sec | 973 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.81102,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 900.05 µs | 1,218.25 ops/sec | 1214 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 1.05784,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.13 ms | 935.4 ops/sec | 935 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.880279,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 980.82 µs | 1,123.49 ops/sec | 1122 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.820664,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 899.53 µs | 1,206.34 ops/sec | 1204 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.726363,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 776.74 µs | 1,368.83 ops/sec | 1367 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.444423,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 508.62 µs | 2,227.93 ops/sec | 2220 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.80563,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 886.1 µs | 1,227.94 ops/sec | 1225 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.799549,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 882.37 µs | 1,239.17 ops/sec | 1237 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.791945,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 870.86 µs | 1,251.23 ops/sec | 1249 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.712041,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 765.76 µs | 1,392.54 ops/sec | 1384 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.938919,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 984.91 µs | 1,057.64 ops/sec | 1054 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 4.908762,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 5.13 ms | 202.88 ops/sec | 203 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 5.661864,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 5.89 ms | 176.22 ops/sec | 177 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.734528,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 782.41 µs | 1,354.31 ops/sec | 1353 samples"
          }
        ],
        "release": {
          "ref": "fix_reprice-the-addon-ci-and-budget",
          "candidateVersion": "0.19.0-rc.2",
          "previousVersion": "0.19.0-rc.2",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "fb6bfd38938501995957297e0bc5ea03860fbe96baf03917f9a4419ace126f56"
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
          "id": "253ca79b93331979e0097338ccab0d9a243fb486",
          "message": "fix(rs-compiler/benchmark): import the reader modules by file URL",
          "timestamp": "2026-09-12T00:32:34+03:00",
          "tree_id": "7db27f3563a28b07a9469b1f46e85ecf14bdff93",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/253ca79b93331979e0097338ccab0d9a243fb486"
        },
        "date": 1789206040017,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.588008,
            "unit": "ms",
            "range": "±1.36%",
            "extra": "p95 666.99 µs | 1,688.53 ops/sec | 1673 samples"
          },
          {
            "name": "card",
            "value": 0.192493,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 240.56 µs | 5,076.91 ops/sec | 5014 samples"
          },
          {
            "name": "consts",
            "value": 0.103999,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 120.12 µs | 9,569.65 ops/sec | 9454 samples"
          },
          {
            "name": "counter",
            "value": 0.773624,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 814.45 µs | 1,288.23 ops/sec | 1287 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.866277,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 911 µs | 1,148.03 ops/sec | 1147 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.962609,
            "unit": "ms",
            "range": "±0.75%",
            "extra": "p95 2.12 ms | 503.48 ops/sec | 501 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.43611,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 486.17 µs | 2,275.02 ops/sec | 2266 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.89206,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 971.67 µs | 1,114 ops/sec | 1113 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.816945,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 898.9 µs | 1,215.09 ops/sec | 1213 samples"
          },
          {
            "name": "page",
            "value": 0.258138,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 300.11 µs | 3,832.04 ops/sec | 3810 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.66824,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 1.72 ms | 596.83 ops/sec | 597 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.628881,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 1.68 ms | 609.88 ops/sec | 610 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.784896,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 880.87 µs | 1,259.14 ops/sec | 1256 samples"
          },
          {
            "name": "typography",
            "value": 1.202596,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 1.33 ms | 819.03 ops/sec | 817 samples"
          },
          {
            "name": "use-memo",
            "value": 0.351237,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 409.52 µs | 2,819.17 ops/sec | 2803 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.235704,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 282.54 µs | 4,158.61 ops/sec | 4130 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.230641,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 279.86 µs | 4,253.37 ops/sec | 4226 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 3.755298,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 3.89 ms | 265.17 ops/sec | 266 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.175195,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 212.82 µs | 5,594.63 ops/sec | 5552 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 1.842522,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 1.91 ms | 542.01 ops/sec | 542 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 16.25541,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 16.67 ms | 61.38 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2597.700926,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 2.62 s | 0.39 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 10.99989,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 11.34 ms | 90.67 ops/sec | 46 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.068798,
            "unit": "ms",
            "range": "±2.50%",
            "extra": "p95 1.17 ms | 921.64 ops/sec | 910 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.311084,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.4 ms | 754.47 ops/sec | 754 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.624384,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 666.29 µs | 1,593.29 ops/sec | 1591 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.794219,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 835.78 µs | 1,252.84 ops/sec | 1252 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.580326,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 631.73 µs | 1,713.6 ops/sec | 1711 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.722319,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 768.67 µs | 1,380.09 ops/sec | 1379 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.550643,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.62 ms | 640.46 ops/sec | 640 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.692349,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 742.34 µs | 1,435.54 ops/sec | 1434 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.499347,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 562.17 µs | 1,984.65 ops/sec | 1979 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.680146,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 735.7 µs | 1,460.67 ops/sec | 1459 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.31977,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 379.01 µs | 3,093.39 ops/sec | 3075 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.478819,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 533.65 µs | 2,074.37 ops/sec | 2067 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.398078,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 452.08 µs | 2,491.04 ops/sec | 2481 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.5782,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 636.46 µs | 1,708.22 ops/sec | 1699 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.45473,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 507.94 µs | 2,179.11 ops/sec | 2170 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.654078,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 692.71 µs | 1,523.56 ops/sec | 1522 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.328437,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 383.3 µs | 3,015.83 ops/sec | 3000 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.489736,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 546.47 µs | 2,029.66 ops/sec | 2024 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.152817,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 181.05 µs | 6,431.22 ops/sec | 6372 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.79366,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 834.19 µs | 1,253.67 ops/sec | 1253 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.456489,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 516.07 µs | 2,172.09 ops/sec | 2165 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.64615,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 693.77 µs | 1,538.81 ops/sec | 1534 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.548531,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 601.96 µs | 1,807.93 ops/sec | 1803 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.402867,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 459.8 µs | 2,457.69 ops/sec | 2448 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.459497,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 513.85 µs | 2,158.4 ops/sec | 2151 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.610235,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 694.78 µs | 1,622.76 ops/sec | 1618 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.862804,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 926.17 µs | 1,152.64 ops/sec | 1152 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.829289,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 897.74 µs | 1,197.69 ops/sec | 1196 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.628862,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 704.91 µs | 1,578.19 ops/sec | 1575 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.870206,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 934.32 µs | 1,141.38 ops/sec | 1140 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.648347,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 750.17 µs | 1,523.06 ops/sec | 1519 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.635656,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 712.86 µs | 1,560.71 ops/sec | 1557 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.611537,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 658.93 µs | 1,625.98 ops/sec | 1623 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.316667,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 365.39 µs | 3,123.14 ops/sec | 3107 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.618051,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 696.88 µs | 1,602.95 ops/sec | 1599 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.630457,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 711.92 µs | 1,572.22 ops/sec | 1567 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.613664,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 691.41 µs | 1,616.26 ops/sec | 1612 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.596234,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 644.46 µs | 1,666.22 ops/sec | 1660 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.818476,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 861.57 µs | 1,215.77 ops/sec | 1213 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 3.639551,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 3.79 ms | 274.05 ops/sec | 274 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 4.220926,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 4.39 ms | 236.1 ops/sec | 236 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.617087,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 665.41 µs | 1,612.65 ops/sec | 1610 samples"
          }
        ],
        "release": {
          "ref": "fix_reprice-the-addon-ci-and-budget",
          "candidateVersion": "0.19.0-rc.2",
          "previousVersion": "0.19.0-rc.2",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "77bf8ebd05a2627b5fbacf503201fa3567304f57043be673bd3c0e087d503033"
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
          "id": "f105006b86fbc176f11e4af36f6a904f401738ac",
          "message": "fix(stylexswc/rs-compiler): keep a base refusal out of the comparison only",
          "timestamp": "2026-09-12T23:12:18+03:00",
          "tree_id": "df71ee4e3962ca0e9f9420c68d12b7a6f11e1ec6",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/f105006b86fbc176f11e4af36f6a904f401738ac"
        },
        "date": 1789247450927,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.677934,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 828.95 µs | 1,451.14 ops/sec | 1445 samples"
          },
          {
            "name": "card",
            "value": 0.277377,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 318.64 µs | 3,565.97 ops/sec | 3540 samples"
          },
          {
            "name": "consts",
            "value": 0.163395,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 194.21 µs | 6,094.33 ops/sec | 6022 samples"
          },
          {
            "name": "counter",
            "value": 0.837893,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 910.76 µs | 1,185.15 ops/sec | 1184 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.947906,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.03 ms | 1,047.43 ops/sec | 1046 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.208308,
            "unit": "ms",
            "range": "±1.04%",
            "extra": "p95 2.38 ms | 446.05 ops/sec | 443 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.513614,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 570.85 µs | 1,929.01 ops/sec | 1922 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.024792,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 1.18 ms | 961.05 ops/sec | 959 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.94563,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 1.09 ms | 1,042.52 ops/sec | 1041 samples"
          },
          {
            "name": "page",
            "value": 0.290122,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 328.38 µs | 3,396.1 ops/sec | 3383 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.890126,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 1.99 ms | 525.18 ops/sec | 525 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.858823,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.95 ms | 535.93 ops/sec | 536 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.907743,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 1.06 ms | 1,086.13 ops/sec | 1083 samples"
          },
          {
            "name": "typography",
            "value": 1.405761,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 1.6 ms | 699.76 ops/sec | 699 samples"
          },
          {
            "name": "use-memo",
            "value": 0.376595,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 437.55 µs | 2,620.68 ops/sec | 2609 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.330769,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 373.34 µs | 2,991.18 ops/sec | 2983 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.325359,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 369.1 µs | 3,046.8 ops/sec | 3035 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.397159,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 4.61 ms | 226.44 ops/sec | 227 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.250429,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 279.07 µs | 3,977.05 ops/sec | 3961 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.08399,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 2.22 ms | 476.06 ops/sec | 476 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 20.889173,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 21.92 ms | 47.74 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3175.593486,
            "unit": "ms",
            "range": "±1.04%",
            "extra": "p95 3.22 s | 0.32 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 14.015174,
            "unit": "ms",
            "range": "±1.00%",
            "extra": "p95 14.72 ms | 71.03 ops/sec | 36 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.199028,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 1.32 ms | 823.46 ops/sec | 822 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.477834,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 1.6 ms | 669.91 ops/sec | 669 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.705436,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 762.5 µs | 1,407.77 ops/sec | 1405 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.878421,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 932.41 µs | 1,130.68 ops/sec | 1129 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.654895,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 694.29 µs | 1,516.62 ops/sec | 1514 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.774071,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 809.27 µs | 1,286.31 ops/sec | 1285 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.743302,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.8 ms | 571.08 ops/sec | 571 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.806435,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 870.06 µs | 1,229.31 ops/sec | 1228 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.565405,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 652.39 µs | 1,747 ops/sec | 1742 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.754658,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 834.34 µs | 1,310.56 ops/sec | 1305 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.361429,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 429.39 µs | 2,728.21 ops/sec | 2715 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.49588,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 570.59 µs | 1,996.69 ops/sec | 1992 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.461964,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 521.53 µs | 2,143.02 ops/sec | 2137 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.62212,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 681.37 µs | 1,595.51 ops/sec | 1593 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.523517,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 584.99 µs | 1,894.3 ops/sec | 1890 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.699831,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 755.64 µs | 1,421.21 ops/sec | 1419 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.375983,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 442.45 µs | 2,614.91 ops/sec | 2603 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.527489,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 597.05 µs | 1,880.66 ops/sec | 1876 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.222874,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 253.07 µs | 4,425.52 ops/sec | 4406 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.834861,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 901.68 µs | 1,185.51 ops/sec | 1182 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.484298,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 558.53 µs | 2,043.82 ops/sec | 2039 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.692816,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 750.62 µs | 1,433.14 ops/sec | 1431 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.613595,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 679.26 µs | 1,613.5 ops/sec | 1610 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.467905,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 533.32 µs | 2,112.28 ops/sec | 2106 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.531226,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 589.94 µs | 1,868.61 ops/sec | 1864 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.671891,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 783.94 µs | 1,465.7 ops/sec | 1458 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.932711,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 1.03 ms | 1,063.39 ops/sec | 1062 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.891638,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 976.05 µs | 1,114.27 ops/sec | 1113 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.692265,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 803.9 µs | 1,425.6 ops/sec | 1416 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.934028,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.01 ms | 1,063.73 ops/sec | 1062 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.707246,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 832.72 µs | 1,393.84 ops/sec | 1389 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.686562,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 788.9 µs | 1,439.8 ops/sec | 1436 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.685959,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 736.8 µs | 1,450.66 ops/sec | 1449 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.377143,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 440.71 µs | 2,623.73 ops/sec | 2614 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.681683,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 786.25 µs | 1,450.26 ops/sec | 1445 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.672426,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 769.2 µs | 1,477.31 ops/sec | 1473 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.657867,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 755.5 µs | 1,506.66 ops/sec | 1502 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.642924,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 701.36 µs | 1,540.66 ops/sec | 1533 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.850943,
            "unit": "ms",
            "range": "±0.46%",
            "extra": "p95 901.36 µs | 1,168.93 ops/sec | 1165 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 4.158438,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 4.36 ms | 239.5 ops/sec | 240 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 4.868433,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 5.17 ms | 204.55 ops/sec | 205 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.691906,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 744.69 µs | 1,437.3 ops/sec | 1435 samples"
          }
        ],
        "release": {
          "ref": "fix_sx-prop-is-skipped-for-compiled-jsx-shorthand",
          "candidateVersion": "0.19.0-rc.2",
          "previousVersion": "0.19.0-rc.2",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "a8c00403287cc43426bffefee9d06ac619a24c136ceb141b4774d36c03ef9e78"
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
          "id": "a4b520b4552175afe3b80ff6c25ccfa41cb959cd",
          "message": "Bump version to 0.19.0-rc.3",
          "timestamp": "2026-09-12T21:46:06Z",
          "tree_id": "22128e88347270eb711ee3c52e40d190674f7a19",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/a4b520b4552175afe3b80ff6c25ccfa41cb959cd"
        },
        "date": 1789252262389,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.75667,
            "unit": "ms",
            "range": "±1.49%",
            "extra": "p95 879.66 µs | 1,304.25 ops/sec | 1292 samples"
          },
          {
            "name": "card",
            "value": 0.341134,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 404.55 µs | 2,877.92 ops/sec | 2859 samples"
          },
          {
            "name": "consts",
            "value": 0.197477,
            "unit": "ms",
            "range": "±0.46%",
            "extra": "p95 277.58 µs | 4,854.57 ops/sec | 4777 samples"
          },
          {
            "name": "counter",
            "value": 0.873205,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 935.2 µs | 1,136.32 ops/sec | 1135 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.994758,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 1.06 ms | 995.92 ops/sec | 995 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.426815,
            "unit": "ms",
            "range": "±1.02%",
            "extra": "p95 2.76 ms | 405.42 ops/sec | 403 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.574676,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 627.8 µs | 1,724.34 ops/sec | 1718 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.199035,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 1.34 ms | 820.76 ops/sec | 819 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.107548,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 1.24 ms | 890.09 ops/sec | 889 samples"
          },
          {
            "name": "page",
            "value": 0.349887,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 401.5 µs | 2,808 ops/sec | 2799 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.015665,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 2.09 ms | 493.01 ops/sec | 493 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.985505,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 2.06 ms | 500.13 ops/sec | 500 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.014637,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 1.15 ms | 970.91 ops/sec | 969 samples"
          },
          {
            "name": "typography",
            "value": 1.628362,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 1.79 ms | 605.47 ops/sec | 605 samples"
          },
          {
            "name": "use-memo",
            "value": 0.44516,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 527.98 µs | 2,219.14 ops/sec | 2212 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.354429,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 393.09 µs | 2,779.61 ops/sec | 2772 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.364438,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 422.24 µs | 2,700.18 ops/sec | 2691 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.616562,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 4.81 ms | 215.88 ops/sec | 216 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.278299,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 325.72 µs | 3,508.03 ops/sec | 3494 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.352763,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 2.47 ms | 420.96 ops/sec | 421 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 23.429747,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 24.02 ms | 42.68 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4000.038363,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 4.04 s | 0.25 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 15.987143,
            "unit": "ms",
            "range": "±7.82%",
            "extra": "p95 16.3 ms | 61.38 ops/sec | 32 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.355437,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 1.46 ms | 730.96 ops/sec | 730 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.653094,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 1.75 ms | 600.18 ops/sec | 600 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.713875,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 764.08 µs | 1,389.66 ops/sec | 1386 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.908267,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 959.21 µs | 1,094.2 ops/sec | 1093 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.668972,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 698.71 µs | 1,489.8 ops/sec | 1488 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.808988,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 842.48 µs | 1,230.19 ops/sec | 1229 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.869265,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.93 ms | 532.9 ops/sec | 533 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.83673,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 886.24 µs | 1,188.06 ops/sec | 1187 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.613073,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 679.61 µs | 1,619.35 ops/sec | 1616 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.794246,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 852.77 µs | 1,250.55 ops/sec | 1249 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.433193,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 498.69 µs | 2,280.84 ops/sec | 2274 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.587535,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 643.13 µs | 1,691.18 ops/sec | 1688 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.504599,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 558.87 µs | 1,964.57 ops/sec | 1960 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.683389,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 736.1 µs | 1,450.11 ops/sec | 1447 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.553153,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 606.49 µs | 1,789.92 ops/sec | 1786 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.741276,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 787.61 µs | 1,339.29 ops/sec | 1337 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.451179,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 552.53 µs | 2,162.7 ops/sec | 2135 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.614175,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 678.54 µs | 1,612.58 ops/sec | 1609 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.261293,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 304.38 µs | 3,744.16 ops/sec | 3727 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.894066,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 958.67 µs | 1,102.23 ops/sec | 1097 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.585237,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 640.95 µs | 1,695.72 ops/sec | 1692 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.742568,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 797.19 µs | 1,336.34 ops/sec | 1333 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.658933,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 710.6 µs | 1,504.75 ops/sec | 1502 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.513083,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 567.95 µs | 1,932.04 ops/sec | 1927 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.553573,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 608.42 µs | 1,788.72 ops/sec | 1783 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.789554,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 871.74 µs | 1,253.87 ops/sec | 1252 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 1.05554,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 1.13 ms | 938.87 ops/sec | 938 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 1.02292,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 1.1 ms | 969.03 ops/sec | 968 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.800741,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 882.85 µs | 1,236.26 ops/sec | 1234 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 1.060307,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.14 ms | 933.64 ops/sec | 933 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.876855,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 970.24 µs | 1,129.14 ops/sec | 1128 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.809474,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 891.9 µs | 1,223.54 ops/sec | 1222 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.71042,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 756.75 µs | 1,400.64 ops/sec | 1399 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.436818,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 504.31 µs | 2,265.32 ops/sec | 2258 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.797351,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 878.54 µs | 1,242.83 ops/sec | 1241 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.790691,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 871.57 µs | 1,252.83 ops/sec | 1251 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.78702,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 868.38 µs | 1,260.12 ops/sec | 1258 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.689523,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 741.66 µs | 1,436.18 ops/sec | 1430 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.925404,
            "unit": "ms",
            "range": "±0.46%",
            "extra": "p95 970.72 µs | 1,074.19 ops/sec | 1071 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 4.875672,
            "unit": "ms",
            "range": "±1.26%",
            "extra": "p95 5.45 ms | 201.8 ops/sec | 201 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 5.619168,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 5.8 ms | 177.62 ops/sec | 178 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.716669,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 770.63 µs | 1,380.56 ops/sec | 1376 samples"
          }
        ],
        "release": {
          "ref": "0.19.0-rc.3",
          "candidateVersion": "0.19.0-rc.3",
          "previousVersion": "0.18.6",
          "target": "x86_64-unknown-linux-musl",
          "nativeSha256": "a45c0735b0faed6c73a594e413effde7440657cfff24bcc41ac6040eef03c8e4"
        }
      }
    ]
  }
};
