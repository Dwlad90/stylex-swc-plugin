window.BENCHMARK_DATA = {
  "lastUpdate": 1788616038753,
  "repoUrl": "https://github.com/Dwlad90/stylex-swc-plugin",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "name": "Dwlad90",
            "username": "Dwlad90",
            "email": "dwlad90@gmail.com"
          },
          "committer": {
            "name": "Dwlad90",
            "username": "Dwlad90",
            "email": "dwlad90@gmail.com"
          },
          "id": "026965f4168b8f0494af05556d50ba1b06ffba28",
          "message": "ci(benchmarks): store release history outside the Actions cache\n\n`npm.yml` is only ever dispatched with `--ref <version>`, so it runs on a\ntag, and Actions caches are ref-scoped: a tag run can write only into its\nown tag's scope, which no later release can read. Every release restored\nnothing, logged `ENOENT ... cache/benchmark-data.json`, measured, then saved\na ~1.2 KB single data point into a scope nothing would ever open again.\n`gh cache list` still shows one identical key set per tag under\n`refs/heads/refs/tags/0.18.3` and `0.18.3-rc.3`. With an empty dataset there\nwas nothing to compare, so the `fail-on-alert` gate in front of `publish`\npassed silently for six runs.\n\nMove release history to the `benchmarks` branch under\n`dev/bench/releases/<target>/node-<version>`, which is readable and writable\nfrom any ref, so each release appends to one series and is compared against\nprevious releases. The branch must exist before the next release or the step\nfails and blocks `publish`.\n\nKeep PR history in the Actions cache, where ref scoping works, and move it\nbehind a `benchmark-pr-` key namespace. The previous widest fallback,\n`aarch64-unknown-linux-gnu-benchmark-`, would prefix-match a release key of\nthe form `<target>-benchmark-<node>` as soon as that target was added to the\nrelease matrix, mixing the two series. The namespaces can no longer collide.\n\nInclude `run_id` in the PR cache key. Keyed on `run_attempt` alone it\nresolved to `<prefix>-1` on every run, so after the first save every later\none failed with \"Unable to reserve cache\" and the baseline stayed frozen at\nwhatever the first run measured. Every PR since was compared against that one\nstale point.\n\nGrant the PR benchmark job `pull-requests: write`. Alerts post as a PR\nreview, and without it the action 403s with `Resource not accessible by\nintegration`, failing the job on a permissions error that hides the\nregression it just found. `contents` stays `read` because the job runs\nPR-authored code. Comment only on same-repo PRs: outside a PR the action\nposts a commit comment needing `contents: write`, and fork PRs always get a\nread-only token, so both would 403 regardless.\n\nDerive `fail-on-alert` from whether a baseline was actually restored and warn\nwhen none was, so an evicted cache cannot silently disable the PR gate the\nway the release gate was already disabled.",
          "timestamp": "2026-08-05T18:23:47Z",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/026965f4168b8f0494af05556d50ba1b06ffba28"
        },
        "date": 1785964934662,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "button-props",
            "value": 0.339896,
            "range": "±3.58%",
            "unit": "ms",
            "extra": "p95 1.36 ms | 2,574 ops/sec | 1780 samples"
          },
          {
            "name": "buttons-demo",
            "value": 1.813875,
            "range": "±3.99%",
            "unit": "ms",
            "extra": "p95 3.71 ms | 565.03 ops/sec | 473 samples"
          },
          {
            "name": "card",
            "value": 0.543541,
            "range": "±4.30%",
            "unit": "ms",
            "extra": "p95 2.05 ms | 1,662.52 ops/sec | 1171 samples"
          },
          {
            "name": "consts",
            "value": 0.246583,
            "range": "±4.37%",
            "unit": "ms",
            "extra": "p95 1.11 ms | 3,518.6 ops/sec | 2346 samples"
          },
          {
            "name": "counter",
            "value": 3.113708,
            "range": "±9.31%",
            "unit": "ms",
            "extra": "p95 6.64 ms | 318.82 ops/sec | 263 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 3.926521,
            "range": "±7.55%",
            "unit": "ms",
            "extra": "p95 8.06 ms | 246.21 ops/sec | 208 samples"
          },
          {
            "name": "global-tokens",
            "value": 5.255584,
            "range": "±4.69%",
            "unit": "ms",
            "extra": "p95 9.56 ms | 181.99 ops/sec | 168 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 1.290313,
            "range": "±8.49%",
            "unit": "ms",
            "extra": "p95 3.98 ms | 754.44 ops/sec | 544 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 2.242916,
            "range": "±5.74%",
            "unit": "ms",
            "extra": "p95 5.18 ms | 445.63 ops/sec | 360 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 2.073979,
            "range": "±6.73%",
            "unit": "ms",
            "extra": "p95 5.37 ms | 488.99 ops/sec | 382 samples"
          },
          {
            "name": "page",
            "value": 0.78825,
            "range": "±6.90%",
            "unit": "ms",
            "extra": "p95 2.64 ms | 1,226.73 ops/sec | 863 samples"
          },
          {
            "name": "page-tsx",
            "value": 6.592792,
            "range": "±5.33%",
            "unit": "ms",
            "extra": "p95 12.03 ms | 149.81 ops/sec | 139 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 5.8005,
            "range": "±15.17%",
            "unit": "ms",
            "extra": "p95 11.53 ms | 167.31 ops/sec | 138 samples"
          },
          {
            "name": "spot-loader",
            "value": 3.218166,
            "range": "±20.11%",
            "unit": "ms",
            "extra": "p95 9.58 ms | 320.04 ops/sec | 208 samples"
          },
          {
            "name": "typography",
            "value": 5.003709,
            "range": "±20.95%",
            "unit": "ms",
            "extra": "p95 22.09 ms | 196.3 ops/sec | 133 samples"
          },
          {
            "name": "use-memo",
            "value": 0.877126,
            "range": "±8.27%",
            "unit": "ms",
            "extra": "p95 2.86 ms | 1,104.4 ops/sec | 810 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.882312,
            "range": "±10.87%",
            "unit": "ms",
            "extra": "p95 2.42 ms | 1,082.35 ops/sec | 790 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.769083,
            "range": "±3.02%",
            "unit": "ms",
            "extra": "p95 1.91 ms | 1,236.75 ops/sec | 1067 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 20.704749,
            "range": "±6.66%",
            "unit": "ms",
            "extra": "p95 32.16 ms | 47.23 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.794167,
            "range": "±6.60%",
            "unit": "ms",
            "extra": "p95 2.86 ms | 1,206.21 ops/sec | 875 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 9.252875,
            "range": "±4.40%",
            "unit": "ms",
            "extra": "p95 14.67 ms | 102.44 ops/sec | 99 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 7956.305001,
            "range": "±7.80%",
            "unit": "ms",
            "extra": "p95 9,064.87 ms | 0.13 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 35.075875,
            "range": "±24.16%",
            "unit": "ms",
            "extra": "p95 87.24 ms | 27.61 ops/sec | 13 samples"
          }
        ]
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
            "value": 0.837146,
            "unit": "ms",
            "range": "±2.08%",
            "extra": "p95 1.45 ms | 1,120.21 ops/sec | 1062 samples"
          },
          {
            "name": "card",
            "value": 0.355958,
            "unit": "ms",
            "range": "±2.32%",
            "extra": "p95 719.67 µs | 2,591.89 ops/sec | 2341 samples"
          },
          {
            "name": "consts",
            "value": 0.203667,
            "unit": "ms",
            "range": "±1.34%",
            "extra": "p95 390.92 µs | 4,610.45 ops/sec | 4265 samples"
          },
          {
            "name": "counter",
            "value": 1.954584,
            "unit": "ms",
            "range": "±3.32%",
            "extra": "p95 3.87 ms | 497.19 ops/sec | 453 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.164438,
            "unit": "ms",
            "range": "±3.48%",
            "extra": "p95 4.49 ms | 427.14 ops/sec | 398 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.067291,
            "unit": "ms",
            "range": "±2.54%",
            "extra": "p95 4.5 ms | 315.99 ops/sec | 309 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.745437,
            "unit": "ms",
            "range": "±0.84%",
            "extra": "p95 959.5 µs | 1,293.81 ops/sec | 1276 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.2435,
            "unit": "ms",
            "range": "±1.38%",
            "extra": "p95 1.84 ms | 767.26 ops/sec | 746 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.054416,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 1.19 ms | 936.47 ops/sec | 932 samples"
          },
          {
            "name": "page",
            "value": 0.462333,
            "unit": "ms",
            "range": "±0.77%",
            "extra": "p95 684.83 µs | 2,063.02 ops/sec | 2017 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.814209,
            "unit": "ms",
            "range": "±1.36%",
            "extra": "p95 4.65 ms | 257.39 ops/sec | 255 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.499042,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 3.87 ms | 282.96 ops/sec | 283 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.686375,
            "unit": "ms",
            "range": "±1.67%",
            "extra": "p95 2.36 ms | 591 ops/sec | 573 samples"
          },
          {
            "name": "typography",
            "value": 2.112,
            "unit": "ms",
            "range": "±0.98%",
            "extra": "p95 2.67 ms | 462.98 ops/sec | 459 samples"
          },
          {
            "name": "use-memo",
            "value": 0.527605,
            "unit": "ms",
            "range": "±0.77%",
            "extra": "p95 743.33 µs | 1,841.74 ops/sec | 1802 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.52327,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 670.63 µs | 1,855.11 ops/sec | 1838 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.537,
            "unit": "ms",
            "range": "±1.13%",
            "extra": "p95 813.25 µs | 1,763.66 ops/sec | 1711 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 13.973042,
            "unit": "ms",
            "range": "±1.91%",
            "extra": "p95 15.66 ms | 72.19 ops/sec | 72 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.49725,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 679.62 µs | 1,932.77 ops/sec | 1903 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 7.080417,
            "unit": "ms",
            "range": "±1.97%",
            "extra": "p95 8.87 ms | 142 ops/sec | 141 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5577.100229,
            "unit": "ms",
            "range": "±7.92%",
            "extra": "p95 6.64 s | 0.18 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 41.654479,
            "unit": "ms",
            "range": "±6.38%",
            "extra": "p95 51.34 ms | 23.61 ops/sec | 12 samples"
          }
        ],
        "release": {
          "ref": "fix_release-benchmarks",
          "candidateVersion": "0.18.3",
          "previousVersion": "0.18.3",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "f22b2cae0db8de4040c9002b050bbcdebb49c2e1f50a39f4b6ff2cf75152aae8"
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
            "value": 1.09925,
            "unit": "ms",
            "range": "±3.13%",
            "extra": "p95 2.74 ms | 865.67 ops/sec | 771 samples"
          },
          {
            "name": "card",
            "value": 0.410333,
            "unit": "ms",
            "range": "±2.41%",
            "extra": "p95 870.79 µs | 2,322.4 ops/sec | 2047 samples"
          },
          {
            "name": "consts",
            "value": 0.199875,
            "unit": "ms",
            "range": "±2.09%",
            "extra": "p95 569.12 µs | 4,589.03 ops/sec | 3979 samples"
          },
          {
            "name": "counter",
            "value": 1.657042,
            "unit": "ms",
            "range": "±1.87%",
            "extra": "p95 2.49 ms | 582.65 ops/sec | 565 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.159125,
            "unit": "ms",
            "range": "±2.16%",
            "extra": "p95 2.97 ms | 466.47 ops/sec | 451 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.371208,
            "unit": "ms",
            "range": "±1.52%",
            "extra": "p95 4.1 ms | 301.7 ops/sec | 297 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.834125,
            "unit": "ms",
            "range": "±3.16%",
            "extra": "p95 1.63 ms | 1,131.47 ops/sec | 1035 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.344125,
            "unit": "ms",
            "range": "±5.76%",
            "extra": "p95 4.3 ms | 664.45 ops/sec | 541 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 2.557895,
            "unit": "ms",
            "range": "±7.74%",
            "extra": "p95 7.02 ms | 395.64 ops/sec | 298 samples"
          },
          {
            "name": "page",
            "value": 0.476145,
            "unit": "ms",
            "range": "±2.13%",
            "extra": "p95 822.08 µs | 2,004.51 ops/sec | 1850 samples"
          },
          {
            "name": "page-tsx",
            "value": 4.472354,
            "unit": "ms",
            "range": "±4.32%",
            "extra": "p95 7.93 ms | 229.92 ops/sec | 212 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.547625,
            "unit": "ms",
            "range": "±4.82%",
            "extra": "p95 6.86 ms | 265.63 ops/sec | 248 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.414604,
            "unit": "ms",
            "range": "±3.36%",
            "extra": "p95 3.16 ms | 635.3 ops/sec | 576 samples"
          },
          {
            "name": "typography",
            "value": 2.011792,
            "unit": "ms",
            "range": "±2.41%",
            "extra": "p95 3.21 ms | 467.08 ops/sec | 448 samples"
          },
          {
            "name": "use-memo",
            "value": 0.442229,
            "unit": "ms",
            "range": "±1.04%",
            "extra": "p95 661.04 µs | 2,130.95 ops/sec | 2070 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.495167,
            "unit": "ms",
            "range": "±1.44%",
            "extra": "p95 777.42 µs | 1,894.08 ops/sec | 1819 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.548,
            "unit": "ms",
            "range": "±1.79%",
            "extra": "p95 1 ms | 1,707.39 ops/sec | 1595 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 17.365167,
            "unit": "ms",
            "range": "±5.10%",
            "extra": "p95 25.31 ms | 57.23 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.512813,
            "unit": "ms",
            "range": "±1.38%",
            "extra": "p95 942.33 µs | 1,799.43 ops/sec | 1696 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.894062,
            "unit": "ms",
            "range": "±1.50%",
            "extra": "p95 8.17 ms | 144.89 ops/sec | 144 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4625.22125,
            "unit": "ms",
            "range": "±2.70%",
            "extra": "p95 4.91 s | 0.22 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 21.604729,
            "unit": "ms",
            "range": "±2.57%",
            "extra": "p95 22.46 ms | 45.79 ops/sec | 24 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "6ac977c5363abdb812775c1a91139f71e24557b7932fd878c4e6ab04f27d1791"
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
            "value": 0.85225,
            "unit": "ms",
            "range": "±0.93%",
            "extra": "p95 1.09 ms | 1,143.97 ops/sec | 1127 samples"
          },
          {
            "name": "card",
            "value": 0.470688,
            "unit": "ms",
            "range": "±1.28%",
            "extra": "p95 706.46 µs | 2,158.55 ops/sec | 2036 samples"
          },
          {
            "name": "consts",
            "value": 0.181333,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 213.29 µs | 5,384.03 ops/sec | 5325 samples"
          },
          {
            "name": "counter",
            "value": 1.470917,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 1.65 ms | 671.65 ops/sec | 670 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.993083,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 2.17 ms | 496.47 ops/sec | 495 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.977125,
            "unit": "ms",
            "range": "±0.68%",
            "extra": "p95 3.18 ms | 333.3 ops/sec | 333 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.732,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 911.96 µs | 1,330.9 ops/sec | 1316 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.159,
            "unit": "ms",
            "range": "±0.77%",
            "extra": "p95 1.49 ms | 833.6 ops/sec | 826 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.053666,
            "unit": "ms",
            "range": "±0.81%",
            "extra": "p95 1.26 ms | 929.24 ops/sec | 921 samples"
          },
          {
            "name": "page",
            "value": 0.447541,
            "unit": "ms",
            "range": "±1.13%",
            "extra": "p95 597.58 µs | 2,155.15 ops/sec | 2102 samples"
          },
          {
            "name": "page-tsx",
            "value": 4.208834,
            "unit": "ms",
            "range": "±1.50%",
            "extra": "p95 4.64 ms | 250.22 ops/sec | 247 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.546584,
            "unit": "ms",
            "range": "±1.97%",
            "extra": "p95 5.24 ms | 270.24 ops/sec | 265 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.443875,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 1.73 ms | 674.36 ops/sec | 671 samples"
          },
          {
            "name": "typography",
            "value": 2.071792,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 2.25 ms | 479.04 ops/sec | 479 samples"
          },
          {
            "name": "use-memo",
            "value": 0.484063,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 554.5 µs | 2,031.03 ops/sec | 2020 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.5165,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 580.62 µs | 1,908.15 ops/sec | 1903 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.532709,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 681.33 µs | 1,811.96 ops/sec | 1795 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 12.870812,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 13.64 ms | 77.69 ops/sec | 78 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.499791,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 639.58 µs | 1,927.3 ops/sec | 1908 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.27725,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 6.68 ms | 158.77 ops/sec | 159 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4437.210896,
            "unit": "ms",
            "range": "±1.80%",
            "extra": "p95 4.64 s | 0.22 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.92325,
            "unit": "ms",
            "range": "±1.75%",
            "extra": "p95 25.15 ms | 41.66 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "6ac977c5363abdb812775c1a91139f71e24557b7932fd878c4e6ab04f27d1791"
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
            "value": 0.891687,
            "unit": "ms",
            "range": "±2.76%",
            "extra": "p95 1.71 ms | 1,079.75 ops/sec | 990 samples"
          },
          {
            "name": "card",
            "value": 0.370938,
            "unit": "ms",
            "range": "±0.97%",
            "extra": "p95 512.75 µs | 2,628.62 ops/sec | 2552 samples"
          },
          {
            "name": "consts",
            "value": 0.178917,
            "unit": "ms",
            "range": "±0.81%",
            "extra": "p95 258.42 µs | 5,353.37 ops/sec | 5146 samples"
          },
          {
            "name": "counter",
            "value": 1.356708,
            "unit": "ms",
            "range": "±2.33%",
            "extra": "p95 1.95 ms | 697.71 ops/sec | 674 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.813438,
            "unit": "ms",
            "range": "±3.18%",
            "extra": "p95 3.44 ms | 506.96 ops/sec | 478 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.396541,
            "unit": "ms",
            "range": "±2.38%",
            "extra": "p95 5.03 ms | 299.42 ops/sec | 289 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.670333,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 720.79 µs | 1,466.77 ops/sec | 1464 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.045625,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 1.1 ms | 948.05 ops/sec | 947 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.948291,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 981.58 µs | 1,045.91 ops/sec | 1046 samples"
          },
          {
            "name": "page",
            "value": 0.404083,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 473.87 µs | 2,408.14 ops/sec | 2398 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.163771,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 3.29 ms | 313.87 ops/sec | 314 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.069375,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 3.25 ms | 322.92 ops/sec | 323 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.440708,
            "unit": "ms",
            "range": "±0.74%",
            "extra": "p95 1.65 ms | 698.13 ops/sec | 693 samples"
          },
          {
            "name": "typography",
            "value": 1.8695,
            "unit": "ms",
            "range": "±1.07%",
            "extra": "p95 2.46 ms | 509.58 ops/sec | 505 samples"
          },
          {
            "name": "use-memo",
            "value": 0.439604,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 548.25 µs | 2,205.76 ops/sec | 2192 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.473,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 522.67 µs | 2,075.14 ops/sec | 2068 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.4725,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 491.13 µs | 2,097.31 ops/sec | 2095 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 11.799792,
            "unit": "ms",
            "range": "±2.95%",
            "extra": "p95 16.59 ms | 81.1 ops/sec | 81 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.438333,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 456.33 µs | 2,265.21 ops/sec | 2264 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.596229,
            "unit": "ms",
            "range": "±0.91%",
            "extra": "p95 6.36 ms | 176.1 ops/sec | 176 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4039.128313,
            "unit": "ms",
            "range": "±1.82%",
            "extra": "p95 4.25 s | 0.25 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 21.237375,
            "unit": "ms",
            "range": "±0.70%",
            "extra": "p95 21.96 ms | 46.82 ops/sec | 24 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "7a83e6742bf37d4e6bdff959f6d99f9516bd296275eca4d1322ff2f0369ae9ed"
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
            "value": 0.923105,
            "unit": "ms",
            "range": "±1.46%",
            "extra": "p95 1.4 ms | 1,030.22 ops/sec | 994 samples"
          },
          {
            "name": "card",
            "value": 0.359333,
            "unit": "ms",
            "range": "±0.86%",
            "extra": "p95 508.79 µs | 2,687.16 ops/sec | 2626 samples"
          },
          {
            "name": "consts",
            "value": 0.195333,
            "unit": "ms",
            "range": "±0.95%",
            "extra": "p95 309.87 µs | 4,891.99 ops/sec | 4702 samples"
          },
          {
            "name": "counter",
            "value": 1.8095,
            "unit": "ms",
            "range": "±3.09%",
            "extra": "p95 3.34 ms | 513.09 ops/sec | 471 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.235792,
            "unit": "ms",
            "range": "±1.30%",
            "extra": "p95 2.79 ms | 447.73 ops/sec | 441 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.036625,
            "unit": "ms",
            "range": "±1.67%",
            "extra": "p95 4.16 ms | 318.09 ops/sec | 314 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.745833,
            "unit": "ms",
            "range": "±0.81%",
            "extra": "p95 872.37 µs | 1,315.26 ops/sec | 1300 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.178375,
            "unit": "ms",
            "range": "±2.74%",
            "extra": "p95 2.41 ms | 773.56 ops/sec | 721 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 2.175875,
            "unit": "ms",
            "range": "±4.57%",
            "extra": "p95 4.68 ms | 472.18 ops/sec | 403 samples"
          },
          {
            "name": "page",
            "value": 0.622583,
            "unit": "ms",
            "range": "±1.45%",
            "extra": "p95 1 ms | 1,575.28 ops/sec | 1487 samples"
          },
          {
            "name": "page-tsx",
            "value": 4.849042,
            "unit": "ms",
            "range": "±2.41%",
            "extra": "p95 6.1 ms | 212.15 ops/sec | 207 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 4.862167,
            "unit": "ms",
            "range": "±3.22%",
            "extra": "p95 7.11 ms | 202.59 ops/sec | 197 samples"
          },
          {
            "name": "spot-loader",
            "value": 2.047875,
            "unit": "ms",
            "range": "±5.41%",
            "extra": "p95 4.03 ms | 492.4 ops/sec | 431 samples"
          },
          {
            "name": "typography",
            "value": 2.882666,
            "unit": "ms",
            "range": "±2.83%",
            "extra": "p95 4.03 ms | 367.98 ops/sec | 347 samples"
          },
          {
            "name": "use-memo",
            "value": 0.684334,
            "unit": "ms",
            "range": "±3.30%",
            "extra": "p95 1.12 ms | 1,478.78 ops/sec | 1345 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.683313,
            "unit": "ms",
            "range": "±1.86%",
            "extra": "p95 1.02 ms | 1,492.31 ops/sec | 1394 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.623313,
            "unit": "ms",
            "range": "±1.15%",
            "extra": "p95 933.79 µs | 1,578.18 ops/sec | 1520 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 18.758395,
            "unit": "ms",
            "range": "±3.26%",
            "extra": "p95 23.83 ms | 53.36 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.493896,
            "unit": "ms",
            "range": "±1.65%",
            "extra": "p95 967.04 µs | 1,882.77 ops/sec | 1774 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 7.594625,
            "unit": "ms",
            "range": "±2.64%",
            "extra": "p95 9.48 ms | 134.31 ops/sec | 132 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5121.752875,
            "unit": "ms",
            "range": "±5.81%",
            "extra": "p95 5.59 s | 0.2 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.965187,
            "unit": "ms",
            "range": "±4.37%",
            "extra": "p95 29.33 ms | 39.51 ops/sec | 20 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "7a83e6742bf37d4e6bdff959f6d99f9516bd296275eca4d1322ff2f0369ae9ed"
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
            "value": 0.729708,
            "unit": "ms",
            "range": "±1.19%",
            "extra": "p95 942.08 µs | 1,329.96 ops/sec | 1301 samples"
          },
          {
            "name": "card",
            "value": 0.302792,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 324.83 µs | 3,251.57 ops/sec | 3230 samples"
          },
          {
            "name": "consts",
            "value": 0.163334,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 214.63 µs | 5,925.32 ops/sec | 5857 samples"
          },
          {
            "name": "counter",
            "value": 1.3625,
            "unit": "ms",
            "range": "±1.08%",
            "extra": "p95 1.95 ms | 678.59 ops/sec | 667 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.792355,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 2.05 ms | 550.31 ops/sec | 550 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.704563,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 2.8 ms | 367.43 ops/sec | 368 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.666042,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 689.75 µs | 1,491.03 ops/sec | 1490 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.034626,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.09 ms | 958.43 ops/sec | 958 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.946458,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 976.71 µs | 1,050.39 ops/sec | 1050 samples"
          },
          {
            "name": "page",
            "value": 0.401583,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 419.83 µs | 2,473.75 ops/sec | 2471 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.482709,
            "unit": "ms",
            "range": "±1.06%",
            "extra": "p95 3.92 ms | 290.33 ops/sec | 289 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.65925,
            "unit": "ms",
            "range": "±1.24%",
            "extra": "p95 4.1 ms | 286.16 ops/sec | 283 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.328895,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 1.55 ms | 723.57 ops/sec | 720 samples"
          },
          {
            "name": "typography",
            "value": 1.862562,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 1.93 ms | 533.98 ops/sec | 534 samples"
          },
          {
            "name": "use-memo",
            "value": 0.435958,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 457.62 µs | 2,274.58 ops/sec | 2271 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.476834,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 503.92 µs | 2,071.16 ops/sec | 2067 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.473833,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 495.92 µs | 2,088.3 ops/sec | 2084 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 11.498625,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 11.77 ms | 86.61 ops/sec | 87 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.43675,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 458.04 µs | 2,267.63 ops/sec | 2265 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.690042,
            "unit": "ms",
            "range": "±1.20%",
            "extra": "p95 6.98 ms | 170.64 ops/sec | 170 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3955.288333,
            "unit": "ms",
            "range": "±2.36%",
            "extra": "p95 4.26 s | 0.25 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 21.44552,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 21.7 ms | 46.53 ops/sec | 24 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-rc.1",
          "candidateVersion": "0.18.4-rc.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "2f7c1b36fd95efb198bfdbede09a74d1b29f9283498037b7fe53eb62b9c8b7ea"
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
            "value": 0.718375,
            "unit": "ms",
            "range": "±0.79%",
            "extra": "p95 922.75 µs | 1,356.11 ops/sec | 1335 samples"
          },
          {
            "name": "card",
            "value": 0.302792,
            "unit": "ms",
            "range": "±0.80%",
            "extra": "p95 418.58 µs | 3,221.84 ops/sec | 3137 samples"
          },
          {
            "name": "consts",
            "value": 0.183708,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 225.37 µs | 5,370.16 ops/sec | 5269 samples"
          },
          {
            "name": "counter",
            "value": 1.241292,
            "unit": "ms",
            "range": "±0.92%",
            "extra": "p95 1.61 ms | 796.3 ops/sec | 786 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.799813,
            "unit": "ms",
            "range": "±1.86%",
            "extra": "p95 2.57 ms | 546.11 ops/sec | 530 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.941959,
            "unit": "ms",
            "range": "±2.26%",
            "extra": "p95 5.23 ms | 252.42 ops/sec | 247 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.809792,
            "unit": "ms",
            "range": "±0.99%",
            "extra": "p95 1.1 ms | 1,214.28 ops/sec | 1187 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.497584,
            "unit": "ms",
            "range": "±0.97%",
            "extra": "p95 1.89 ms | 661.92 ops/sec | 653 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.330083,
            "unit": "ms",
            "range": "±0.99%",
            "extra": "p95 1.7 ms | 733.66 ops/sec | 724 samples"
          },
          {
            "name": "page",
            "value": 0.329,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 390.75 µs | 2,941.82 ops/sec | 2919 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.275875,
            "unit": "ms",
            "range": "±0.95%",
            "extra": "p95 3.86 ms | 299.2 ops/sec | 298 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.19825,
            "unit": "ms",
            "range": "±0.87%",
            "extra": "p95 3.74 ms | 306.04 ops/sec | 305 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.20575,
            "unit": "ms",
            "range": "±0.91%",
            "extra": "p95 1.48 ms | 825.83 ops/sec | 815 samples"
          },
          {
            "name": "typography",
            "value": 1.913208,
            "unit": "ms",
            "range": "±1.62%",
            "extra": "p95 2.46 ms | 517.81 ops/sec | 507 samples"
          },
          {
            "name": "use-memo",
            "value": 0.541313,
            "unit": "ms",
            "range": "±1.68%",
            "extra": "p95 912.79 µs | 1,786.19 ops/sec | 1678 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.539166,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 640.67 µs | 1,838.51 ops/sec | 1817 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.536833,
            "unit": "ms",
            "range": "±1.24%",
            "extra": "p95 705.04 µs | 1,823.56 ops/sec | 1778 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 13.195625,
            "unit": "ms",
            "range": "±1.65%",
            "extra": "p95 15.35 ms | 75.35 ops/sec | 75 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.386292,
            "unit": "ms",
            "range": "±1.13%",
            "extra": "p95 591.83 µs | 2,503.47 ops/sec | 2405 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.125104,
            "unit": "ms",
            "range": "±2.63%",
            "extra": "p95 7.96 ms | 160.18 ops/sec | 158 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4399.536938,
            "unit": "ms",
            "range": "±2.75%",
            "extra": "p95 4.64 s | 0.23 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 22.075459,
            "unit": "ms",
            "range": "±2.15%",
            "extra": "p95 23.94 ms | 44.89 ops/sec | 23 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-rc.2",
          "candidateVersion": "0.18.4-rc.2",
          "previousVersion": "0.18.3",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "78c8d1320227dee16e044a3db13fb4ad892cd26f319da9659d10b977a47fa2d3"
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
            "value": 0.908666,
            "unit": "ms",
            "range": "±3.53%",
            "extra": "p95 2.33 ms | 1,018.46 ops/sec | 901 samples"
          },
          {
            "name": "card",
            "value": 0.362459,
            "unit": "ms",
            "range": "±2.20%",
            "extra": "p95 833.38 µs | 2,549.84 ops/sec | 2289 samples"
          },
          {
            "name": "consts",
            "value": 0.198417,
            "unit": "ms",
            "range": "±1.75%",
            "extra": "p95 496.12 µs | 4,623.56 ops/sec | 4093 samples"
          },
          {
            "name": "counter",
            "value": 2.164125,
            "unit": "ms",
            "range": "±4.15%",
            "extra": "p95 4.64 ms | 474.16 ops/sec | 419 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.560125,
            "unit": "ms",
            "range": "±3.42%",
            "extra": "p95 4.45 ms | 373.79 ops/sec | 355 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.222271,
            "unit": "ms",
            "range": "±1.66%",
            "extra": "p95 4.12 ms | 308.79 ops/sec | 304 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.76025,
            "unit": "ms",
            "range": "±1.89%",
            "extra": "p95 1.06 ms | 1,290.23 ops/sec | 1234 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.28425,
            "unit": "ms",
            "range": "±1.21%",
            "extra": "p95 1.73 ms | 763.95 ops/sec | 749 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.512333,
            "unit": "ms",
            "range": "±4.43%",
            "extra": "p95 3.98 ms | 596.42 ops/sec | 510 samples"
          },
          {
            "name": "page",
            "value": 0.572042,
            "unit": "ms",
            "range": "±2.96%",
            "extra": "p95 1.67 ms | 1,584.54 ops/sec | 1358 samples"
          },
          {
            "name": "page-tsx",
            "value": 4.609417,
            "unit": "ms",
            "range": "±3.17%",
            "extra": "p95 7.81 ms | 211.7 ops/sec | 205 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 4.156958,
            "unit": "ms",
            "range": "±2.29%",
            "extra": "p95 5.05 ms | 239.29 ops/sec | 235 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.638896,
            "unit": "ms",
            "range": "±1.80%",
            "extra": "p95 2.33 ms | 588.27 ops/sec | 572 samples"
          },
          {
            "name": "typography",
            "value": 2.374875,
            "unit": "ms",
            "range": "±1.32%",
            "extra": "p95 2.99 ms | 416.11 ops/sec | 410 samples"
          },
          {
            "name": "use-memo",
            "value": 0.536833,
            "unit": "ms",
            "range": "±1.37%",
            "extra": "p95 896.87 µs | 1,783.72 ops/sec | 1705 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.570667,
            "unit": "ms",
            "range": "±1.26%",
            "extra": "p95 821.21 µs | 1,690.98 ops/sec | 1638 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.618583,
            "unit": "ms",
            "range": "±1.08%",
            "extra": "p95 837.5 µs | 1,597.39 ops/sec | 1548 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 16.907666,
            "unit": "ms",
            "range": "±4.21%",
            "extra": "p95 23.69 ms | 58.37 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.572167,
            "unit": "ms",
            "range": "±0.83%",
            "extra": "p95 806.92 µs | 1,687.57 ops/sec | 1648 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 7.250958,
            "unit": "ms",
            "range": "±0.86%",
            "extra": "p95 7.86 ms | 137.66 ops/sec | 138 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5221.586687,
            "unit": "ms",
            "range": "±4.45%",
            "extra": "p95 5.91 s | 0.19 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 35.287625,
            "unit": "ms",
            "range": "±4.83%",
            "extra": "p95 39.64 ms | 28.09 ops/sec | 14 samples"
          }
        ],
        "release": {
          "ref": "0.18.4",
          "candidateVersion": "0.18.4",
          "previousVersion": "0.18.3",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "1a8605fb2b11df2848d32f568d464c9433bf995d1db0d0f30451d7a320c5c82b"
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
            "value": 1.353833,
            "unit": "ms",
            "range": "±4.35%",
            "extra": "p95 3.58 ms | 706.44 ops/sec | 597 samples"
          },
          {
            "name": "card",
            "value": 0.528688,
            "unit": "ms",
            "range": "±3.55%",
            "extra": "p95 1.76 ms | 1,730.44 ops/sec | 1380 samples"
          },
          {
            "name": "consts",
            "value": 0.208541,
            "unit": "ms",
            "range": "±1.43%",
            "extra": "p95 407.79 µs | 4,480.28 ops/sec | 4087 samples"
          },
          {
            "name": "counter",
            "value": 2.154938,
            "unit": "ms",
            "range": "±4.45%",
            "extra": "p95 4.91 ms | 452.6 ops/sec | 404 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.696917,
            "unit": "ms",
            "range": "±1.78%",
            "extra": "p95 3.42 ms | 368.25 ops/sec | 361 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.635667,
            "unit": "ms",
            "range": "±2.21%",
            "extra": "p95 4.74 ms | 270.15 ops/sec | 265 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.881042,
            "unit": "ms",
            "range": "±1.57%",
            "extra": "p95 1.21 ms | 1,125.51 ops/sec | 1083 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.121458,
            "unit": "ms",
            "range": "±1.16%",
            "extra": "p95 1.65 ms | 845.53 ops/sec | 827 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.108958,
            "unit": "ms",
            "range": "±1.92%",
            "extra": "p95 1.83 ms | 856.61 ops/sec | 809 samples"
          },
          {
            "name": "page",
            "value": 0.48025,
            "unit": "ms",
            "range": "±1.47%",
            "extra": "p95 751.63 µs | 2,003.44 ops/sec | 1913 samples"
          },
          {
            "name": "page-tsx",
            "value": 4.032084,
            "unit": "ms",
            "range": "±3.40%",
            "extra": "p95 5.86 ms | 249.58 ops/sec | 238 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.600333,
            "unit": "ms",
            "range": "±1.53%",
            "extra": "p95 4.59 ms | 273.86 ops/sec | 270 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.658375,
            "unit": "ms",
            "range": "±1.87%",
            "extra": "p95 2.27 ms | 606.31 ops/sec | 586 samples"
          },
          {
            "name": "typography",
            "value": 2.370958,
            "unit": "ms",
            "range": "±1.95%",
            "extra": "p95 3.41 ms | 419.11 ops/sec | 408 samples"
          },
          {
            "name": "use-memo",
            "value": 0.497292,
            "unit": "ms",
            "range": "±0.93%",
            "extra": "p95 629.21 µs | 1,972.93 ops/sec | 1933 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.546709,
            "unit": "ms",
            "range": "±1.41%",
            "extra": "p95 790.96 µs | 1,771.37 ops/sec | 1708 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.521104,
            "unit": "ms",
            "range": "±1.53%",
            "extra": "p95 923.71 µs | 1,786.69 ops/sec | 1684 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 13.321916,
            "unit": "ms",
            "range": "±2.44%",
            "extra": "p95 15.77 ms | 74.92 ops/sec | 75 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.69898,
            "unit": "ms",
            "range": "±2.64%",
            "extra": "p95 1.69 ms | 1,433.32 ops/sec | 1244 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 7.302625,
            "unit": "ms",
            "range": "±3.96%",
            "extra": "p95 10.84 ms | 134.04 ops/sec | 129 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5094.01702,
            "unit": "ms",
            "range": "±4.83%",
            "extra": "p95 5.73 s | 0.2 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 25.835125,
            "unit": "ms",
            "range": "±3.92%",
            "extra": "p95 29.37 ms | 38.56 ops/sec | 20 samples"
          }
        ],
        "release": {
          "ref": "0.18.5-rc.1",
          "candidateVersion": "0.18.5-rc.1",
          "previousVersion": "0.18.4",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "62e871ae0d0a97234da298e52544726350ddea426b8ea29cee2c895b390344bf"
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
            "value": 1.608542,
            "unit": "ms",
            "range": "±3.10%",
            "extra": "p95 3.23 ms | 613.03 ops/sec | 555 samples"
          },
          {
            "name": "card",
            "value": 0.37625,
            "unit": "ms",
            "range": "±1.09%",
            "extra": "p95 549.38 µs | 2,572.44 ops/sec | 2480 samples"
          },
          {
            "name": "consts",
            "value": 0.190833,
            "unit": "ms",
            "range": "±0.78%",
            "extra": "p95 306 µs | 4,978.4 ops/sec | 4785 samples"
          },
          {
            "name": "counter",
            "value": 2.862791,
            "unit": "ms",
            "range": "±2.11%",
            "extra": "p95 4.04 ms | 352.08 ops/sec | 340 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 3.881104,
            "unit": "ms",
            "range": "±2.03%",
            "extra": "p95 5.1 ms | 255.9 ops/sec | 252 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.870042,
            "unit": "ms",
            "range": "±2.18%",
            "extra": "p95 5.46 ms | 253.82 ops/sec | 249 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.895313,
            "unit": "ms",
            "range": "±2.30%",
            "extra": "p95 1.71 ms | 1,054.18 ops/sec | 984 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.510041,
            "unit": "ms",
            "range": "±2.37%",
            "extra": "p95 2.53 ms | 631.18 ops/sec | 599 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.327125,
            "unit": "ms",
            "range": "±1.73%",
            "extra": "p95 2.05 ms | 744.1 ops/sec | 715 samples"
          },
          {
            "name": "page",
            "value": 0.670958,
            "unit": "ms",
            "range": "±1.76%",
            "extra": "p95 1.22 ms | 1,450.06 ops/sec | 1344 samples"
          },
          {
            "name": "page-tsx",
            "value": 4.278666,
            "unit": "ms",
            "range": "±1.71%",
            "extra": "p95 5.4 ms | 232.28 ops/sec | 229 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 4.392792,
            "unit": "ms",
            "range": "±1.81%",
            "extra": "p95 5.5 ms | 228.53 ops/sec | 225 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.985562,
            "unit": "ms",
            "range": "±1.51%",
            "extra": "p95 2.66 ms | 503.85 ops/sec | 492 samples"
          },
          {
            "name": "typography",
            "value": 3.019292,
            "unit": "ms",
            "range": "±1.95%",
            "extra": "p95 4.35 ms | 322.64 ops/sec | 315 samples"
          },
          {
            "name": "use-memo",
            "value": 0.621541,
            "unit": "ms",
            "range": "±1.58%",
            "extra": "p95 997.92 µs | 1,556.82 ops/sec | 1477 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.642667,
            "unit": "ms",
            "range": "±1.80%",
            "extra": "p95 1.13 ms | 1,477.15 ops/sec | 1387 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.618041,
            "unit": "ms",
            "range": "±0.97%",
            "extra": "p95 865.83 µs | 1,585.1 ops/sec | 1546 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 15.944875,
            "unit": "ms",
            "range": "±2.75%",
            "extra": "p95 20.04 ms | 61.44 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.757021,
            "unit": "ms",
            "range": "±2.80%",
            "extra": "p95 1.93 ms | 1,269.73 ops/sec | 1100 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 8.269583,
            "unit": "ms",
            "range": "±2.16%",
            "extra": "p95 10.43 ms | 119.77 ops/sec | 119 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 6798.083501,
            "unit": "ms",
            "range": "±11.82%",
            "extra": "p95 8.59 s | 0.15 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 28.115604,
            "unit": "ms",
            "range": "±12.92%",
            "extra": "p95 49.66 ms | 33.34 ops/sec | 16 samples"
          }
        ],
        "release": {
          "ref": "0.18.5",
          "candidateVersion": "0.18.5",
          "previousVersion": "0.18.4",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "9b0a9f9735ad139d286d4dfb70cf71d9bac2f537bc3c5be06e5de05630dd202b"
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
            "value": 0.726708,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 776.04 µs | 1,362.82 ops/sec | 1359 samples"
          },
          {
            "name": "card",
            "value": 0.308291,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 351.58 µs | 3,198.01 ops/sec | 3186 samples"
          },
          {
            "name": "consts",
            "value": 0.166125,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 193.12 µs | 5,875.86 ops/sec | 5827 samples"
          },
          {
            "name": "counter",
            "value": 1.339834,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 1.43 ms | 740.86 ops/sec | 741 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.788584,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 1.83 ms | 557.55 ops/sec | 558 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.744917,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 2.79 ms | 364.64 ops/sec | 365 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.711708,
            "unit": "ms",
            "range": "±1.29%",
            "extra": "p95 914.5 µs | 1,364.16 ops/sec | 1336 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.03925,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 1.08 ms | 955.23 ops/sec | 955 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.952292,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 984.92 µs | 1,043.41 ops/sec | 1043 samples"
          },
          {
            "name": "page",
            "value": 0.401667,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 420.71 µs | 2,471.65 ops/sec | 2470 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.263062,
            "unit": "ms",
            "range": "±1.09%",
            "extra": "p95 4 ms | 291.9 ops/sec | 290 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.079437,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 3.31 ms | 321.78 ops/sec | 322 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.29275,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 1.33 ms | 770.68 ops/sec | 771 samples"
          },
          {
            "name": "typography",
            "value": 1.866229,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.9 ms | 535.31 ops/sec | 536 samples"
          },
          {
            "name": "use-memo",
            "value": 0.439417,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 461.21 µs | 2,255.99 ops/sec | 2254 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.475167,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 501.08 µs | 2,083.3 ops/sec | 2081 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.474292,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 492.17 µs | 2,092.48 ops/sec | 2091 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 11.9225,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 12.09 ms | 84.26 ops/sec | 85 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.453583,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 525.79 µs | 2,148.43 ops/sec | 2136 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.554687,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 5.66 ms | 179.59 ops/sec | 180 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4050.245792,
            "unit": "ms",
            "range": "±1.01%",
            "extra": "p95 4.12 s | 0.25 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.116375,
            "unit": "ms",
            "range": "±1.02%",
            "extra": "p95 23.58 ms | 43.6 ops/sec | 22 samples"
          }
        ],
        "release": {
          "ref": "0.18.6-rc.1",
          "candidateVersion": "0.18.6-rc.1",
          "previousVersion": "0.18.5",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "046d14cb7ff4003ed0a2cbddbcfa9d01053e35dee2659dadc0528fea0c4950eb"
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
            "value": 0.882896,
            "unit": "ms",
            "range": "±3.43%",
            "extra": "p95 2.08 ms | 1,037.82 ops/sec | 920 samples"
          },
          {
            "name": "card",
            "value": 0.352458,
            "unit": "ms",
            "range": "±0.91%",
            "extra": "p95 556.71 µs | 2,685.47 ops/sec | 2593 samples"
          },
          {
            "name": "consts",
            "value": 0.220583,
            "unit": "ms",
            "range": "±2.78%",
            "extra": "p95 823.75 µs | 3,994.26 ops/sec | 3112 samples"
          },
          {
            "name": "counter",
            "value": 2.51975,
            "unit": "ms",
            "range": "±3.03%",
            "extra": "p95 4.46 ms | 390.69 ops/sec | 367 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 3.470854,
            "unit": "ms",
            "range": "±4.76%",
            "extra": "p95 7.38 ms | 269.63 ops/sec | 242 samples"
          },
          {
            "name": "global-tokens",
            "value": 7.254875,
            "unit": "ms",
            "range": "±6.11%",
            "extra": "p95 12.05 ms | 153.69 ops/sec | 136 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.976417,
            "unit": "ms",
            "range": "±3.21%",
            "extra": "p95 2.65 ms | 950.6 ops/sec | 841 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.916458,
            "unit": "ms",
            "range": "±3.89%",
            "extra": "p95 4.31 ms | 507.23 ops/sec | 448 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.783083,
            "unit": "ms",
            "range": "±3.82%",
            "extra": "p95 3.91 ms | 548.86 ops/sec | 485 samples"
          },
          {
            "name": "page",
            "value": 0.613022,
            "unit": "ms",
            "range": "±3.23%",
            "extra": "p95 1.57 ms | 1,550.17 ops/sec | 1342 samples"
          },
          {
            "name": "page-tsx",
            "value": 5.045896,
            "unit": "ms",
            "range": "±2.91%",
            "extra": "p95 7.78 ms | 193.35 ops/sec | 188 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 5.212291,
            "unit": "ms",
            "range": "±4.49%",
            "extra": "p95 10.29 ms | 181.01 ops/sec | 170 samples"
          },
          {
            "name": "spot-loader",
            "value": 2.084583,
            "unit": "ms",
            "range": "±2.73%",
            "extra": "p95 3.16 ms | 472.86 ops/sec | 450 samples"
          },
          {
            "name": "typography",
            "value": 2.978292,
            "unit": "ms",
            "range": "±3.16%",
            "extra": "p95 5.33 ms | 324.32 ops/sec | 312 samples"
          },
          {
            "name": "use-memo",
            "value": 0.899917,
            "unit": "ms",
            "range": "±3.34%",
            "extra": "p95 2.37 ms | 1,065.92 ops/sec | 883 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.610958,
            "unit": "ms",
            "range": "±1.19%",
            "extra": "p95 944.71 µs | 1,602.27 ops/sec | 1541 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.559437,
            "unit": "ms",
            "range": "±1.02%",
            "extra": "p95 871.88 µs | 1,708.58 ops/sec | 1656 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 16.974354,
            "unit": "ms",
            "range": "±6.67%",
            "extra": "p95 28.42 ms | 57.07 ops/sec | 64 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.672104,
            "unit": "ms",
            "range": "±4.03%",
            "extra": "p95 2.21 ms | 1,369.2 ops/sec | 1122 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 8.493,
            "unit": "ms",
            "range": "±3.73%",
            "extra": "p95 12.5 ms | 111.78 ops/sec | 109 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 6109.605729,
            "unit": "ms",
            "range": "±6.41%",
            "extra": "p95 7.41 s | 0.16 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 33.68225,
            "unit": "ms",
            "range": "±11.56%",
            "extra": "p95 61.12 ms | 29.09 ops/sec | 15 samples"
          }
        ],
        "release": {
          "ref": "0.18.6",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.5",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "4e3d7143825f800c5483c8104b777d2f8ba6d0585865b2924b6242926bb76768"
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
            "value": 0.506146,
            "unit": "ms",
            "range": "±2.05%",
            "extra": "p95 756.21 µs | 1,894.4 ops/sec | 1816 samples"
          },
          {
            "name": "card",
            "value": 0.222334,
            "unit": "ms",
            "range": "±0.89%",
            "extra": "p95 312.54 µs | 4,408.66 ops/sec | 4273 samples"
          },
          {
            "name": "consts",
            "value": 0.161875,
            "unit": "ms",
            "range": "±1.19%",
            "extra": "p95 228 µs | 6,063.47 ops/sec | 5807 samples"
          },
          {
            "name": "counter",
            "value": 0.790042,
            "unit": "ms",
            "range": "±1.75%",
            "extra": "p95 1.24 ms | 1,258.29 ops/sec | 1187 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.014188,
            "unit": "ms",
            "range": "±1.18%",
            "extra": "p95 1.39 ms | 1,006.25 ops/sec | 974 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.430729,
            "unit": "ms",
            "range": "±3.09%",
            "extra": "p95 3.64 ms | 409.08 ops/sec | 390 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.554084,
            "unit": "ms",
            "range": "±2.84%",
            "extra": "p95 1.46 ms | 1,705.6 ops/sec | 1481 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.721542,
            "unit": "ms",
            "range": "±2.67%",
            "extra": "p95 1.27 ms | 1,332.06 ops/sec | 1215 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.680458,
            "unit": "ms",
            "range": "±1.97%",
            "extra": "p95 1.11 ms | 1,448.87 ops/sec | 1355 samples"
          },
          {
            "name": "page",
            "value": 0.221833,
            "unit": "ms",
            "range": "±1.64%",
            "extra": "p95 411.5 µs | 4,364.51 ops/sec | 3989 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.010083,
            "unit": "ms",
            "range": "±1.36%",
            "extra": "p95 2.63 ms | 485.54 ops/sec | 478 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.938625,
            "unit": "ms",
            "range": "±2.17%",
            "extra": "p95 2.49 ms | 505.76 ops/sec | 491 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.650875,
            "unit": "ms",
            "range": "±1.33%",
            "extra": "p95 855 µs | 1,497.16 ops/sec | 1455 samples"
          },
          {
            "name": "typography",
            "value": 1.030667,
            "unit": "ms",
            "range": "±1.09%",
            "extra": "p95 1.28 ms | 948.95 ops/sec | 934 samples"
          },
          {
            "name": "use-memo",
            "value": 0.265542,
            "unit": "ms",
            "range": "±0.92%",
            "extra": "p95 338.5 µs | 3,715.12 ops/sec | 3602 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.355208,
            "unit": "ms",
            "range": "±2.89%",
            "extra": "p95 1.1 ms | 2,649.93 ops/sec | 2251 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.316375,
            "unit": "ms",
            "range": "±2.34%",
            "extra": "p95 599.75 µs | 3,044.35 ops/sec | 2740 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.292271,
            "unit": "ms",
            "range": "±1.76%",
            "extra": "p95 6.76 ms | 185.71 ops/sec | 184 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.2345,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 271.58 µs | 4,230.45 ops/sec | 4177 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.684167,
            "unit": "ms",
            "range": "±1.90%",
            "extra": "p95 4.26 ms | 270.35 ops/sec | 267 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 19.871875,
            "unit": "ms",
            "range": "±3.25%",
            "extra": "p95 27.14 ms | 48.77 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2587.114958,
            "unit": "ms",
            "range": "±5.76%",
            "extra": "p95 2.8 s | 0.4 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 11.049,
            "unit": "ms",
            "range": "±2.92%",
            "extra": "p95 12.37 ms | 88.84 ops/sec | 45 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.040021,
            "unit": "ms",
            "range": "±0.76%",
            "extra": "p95 1.32 ms | 934.88 ops/sec | 926 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.473895,
            "unit": "ms",
            "range": "±0.82%",
            "extra": "p95 1.78 ms | 660.82 ops/sec | 656 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.502771,
            "unit": "ms",
            "range": "±1.80%",
            "extra": "p95 887.88 µs | 1,808.61 ops/sec | 1700 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 1.013041,
            "unit": "ms",
            "range": "±1.21%",
            "extra": "p95 1.31 ms | 964.86 ops/sec | 945 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.664208,
            "unit": "ms",
            "range": "±2.96%",
            "extra": "p95 1.57 ms | 1,422.37 ops/sec | 1276 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.957354,
            "unit": "ms",
            "range": "±3.22%",
            "extra": "p95 2.69 ms | 990.41 ops/sec | 890 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.900625,
            "unit": "ms",
            "range": "±1.16%",
            "extra": "p95 2.35 ms | 523.49 ops/sec | 517 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.77702,
            "unit": "ms",
            "range": "±1.71%",
            "extra": "p95 1.09 ms | 1,255.24 ops/sec | 1206 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.394542,
            "unit": "ms",
            "range": "±1.14%",
            "extra": "p95 528.75 µs | 2,471.08 ops/sec | 2397 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.785084,
            "unit": "ms",
            "range": "±1.58%",
            "extra": "p95 1.14 ms | 1,241.96 ops/sec | 1197 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.276334,
            "unit": "ms",
            "range": "±1.71%",
            "extra": "p95 495.17 µs | 3,433.35 ops/sec | 3184 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.629604,
            "unit": "ms",
            "range": "±1.82%",
            "extra": "p95 987.79 µs | 1,520.1 ops/sec | 1438 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.454999,
            "unit": "ms",
            "range": "±0.94%",
            "extra": "p95 574.71 µs | 2,169.06 ops/sec | 2126 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.90425,
            "unit": "ms",
            "range": "±3.64%",
            "extra": "p95 2.28 ms | 1,042.42 ops/sec | 922 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.545125,
            "unit": "ms",
            "range": "±2.35%",
            "extra": "p95 1.08 ms | 1,749.71 ops/sec | 1601 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.91325,
            "unit": "ms",
            "range": "±0.89%",
            "extra": "p95 1.15 ms | 1,080.34 ops/sec | 1065 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.298292,
            "unit": "ms",
            "range": "±0.97%",
            "extra": "p95 394.33 µs | 3,291.77 ops/sec | 3197 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.632104,
            "unit": "ms",
            "range": "±1.43%",
            "extra": "p95 976.71 µs | 1,531.73 ops/sec | 1470 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.187667,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 226.37 µs | 5,215.79 ops/sec | 5116 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 1.023333,
            "unit": "ms",
            "range": "±2.02%",
            "extra": "p95 1.62 ms | 943.36 ops/sec | 901 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.577021,
            "unit": "ms",
            "range": "±0.79%",
            "extra": "p95 732.75 µs | 1,705.03 ops/sec | 1678 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.868,
            "unit": "ms",
            "range": "±1.56%",
            "extra": "p95 1.2 ms | 1,118.37 ops/sec | 1083 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.739083,
            "unit": "ms",
            "range": "±3.54%",
            "extra": "p95 2.34 ms | 1,234.04 ops/sec | 1055 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.475792,
            "unit": "ms",
            "range": "±2.39%",
            "extra": "p95 1.15 ms | 1,974.05 ops/sec | 1767 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.503604,
            "unit": "ms",
            "range": "±0.92%",
            "extra": "p95 697 µs | 1,921.54 ops/sec | 1876 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.45225,
            "unit": "ms",
            "range": "±1.04%",
            "extra": "p95 566.21 µs | 2,166.21 ops/sec | 2118 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.844625,
            "unit": "ms",
            "range": "±1.29%",
            "extra": "p95 1.21 ms | 1,138.42 ops/sec | 1112 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.859917,
            "unit": "ms",
            "range": "±1.42%",
            "extra": "p95 1.34 ms | 1,123.54 ops/sec | 1087 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.44025,
            "unit": "ms",
            "range": "±0.88%",
            "extra": "p95 575.92 µs | 2,221.23 ops/sec | 2171 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.911,
            "unit": "ms",
            "range": "±1.34%",
            "extra": "p95 1.46 ms | 1,052.22 ops/sec | 1017 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.462667,
            "unit": "ms",
            "range": "±2.22%",
            "extra": "p95 781.63 µs | 2,076.03 ops/sec | 1921 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.615146,
            "unit": "ms",
            "range": "±3.98%",
            "extra": "p95 1.75 ms | 1,568.1 ops/sec | 1278 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.60725,
            "unit": "ms",
            "range": "±1.37%",
            "extra": "p95 988.13 µs | 1,558.22 ops/sec | 1490 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.284021,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 388.96 µs | 3,418.99 ops/sec | 3346 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.466709,
            "unit": "ms",
            "range": "±1.01%",
            "extra": "p95 689.46 µs | 2,073.8 ops/sec | 2009 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.501645,
            "unit": "ms",
            "range": "±1.57%",
            "extra": "p95 805.12 µs | 1,927.11 ops/sec | 1830 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.503584,
            "unit": "ms",
            "range": "±4.64%",
            "extra": "p95 1.04 ms | 1,882.16 ops/sec | 1635 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.528417,
            "unit": "ms",
            "range": "±1.06%",
            "extra": "p95 758.21 µs | 1,821.72 ops/sec | 1771 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 1.022521,
            "unit": "ms",
            "range": "±1.46%",
            "extra": "p95 1.43 ms | 954.09 ops/sec | 924 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 6.985709,
            "unit": "ms",
            "range": "±4.46%",
            "extra": "p95 12.35 ms | 138.96 ops/sec | 133 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 7.977729,
            "unit": "ms",
            "range": "±3.01%",
            "extra": "p95 10.24 ms | 126.7 ops/sec | 124 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.63625,
            "unit": "ms",
            "range": "±0.96%",
            "extra": "p95 860.42 µs | 1,525.43 ops/sec | 1493 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "4b655752fc9cbfc8410abf8f746f4392dbf406a371b6f8a1a14683c1fae5d79d"
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
            "value": 0.549104,
            "unit": "ms",
            "range": "±2.29%",
            "extra": "p95 839.71 µs | 1,785.36 ops/sec | 1686 samples"
          },
          {
            "name": "card",
            "value": 0.20875,
            "unit": "ms",
            "range": "±1.24%",
            "extra": "p95 286.08 µs | 4,645.79 ops/sec | 4434 samples"
          },
          {
            "name": "consts",
            "value": 0.147583,
            "unit": "ms",
            "range": "±1.43%",
            "extra": "p95 234.54 µs | 6,462.46 ops/sec | 5977 samples"
          },
          {
            "name": "counter",
            "value": 0.961167,
            "unit": "ms",
            "range": "±3.76%",
            "extra": "p95 2.89 ms | 973.48 ops/sec | 836 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.115583,
            "unit": "ms",
            "range": "±1.49%",
            "extra": "p95 1.58 ms | 877.79 ops/sec | 851 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.642541,
            "unit": "ms",
            "range": "±1.94%",
            "extra": "p95 3.74 ms | 368.88 ops/sec | 361 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.476542,
            "unit": "ms",
            "range": "±1.83%",
            "extra": "p95 713.79 µs | 2,039.73 ops/sec | 1925 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.755749,
            "unit": "ms",
            "range": "±4.57%",
            "extra": "p95 2.26 ms | 1,224.84 ops/sec | 1046 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.710708,
            "unit": "ms",
            "range": "±1.13%",
            "extra": "p95 977.58 µs | 1,377.59 ops/sec | 1344 samples"
          },
          {
            "name": "page",
            "value": 0.23875,
            "unit": "ms",
            "range": "±0.71%",
            "extra": "p95 301.21 µs | 4,098.09 ops/sec | 4003 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.209166,
            "unit": "ms",
            "range": "±1.97%",
            "extra": "p95 2.76 ms | 451.73 ops/sec | 441 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.176417,
            "unit": "ms",
            "range": "±2.23%",
            "extra": "p95 2.7 ms | 457.26 ops/sec | 444 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.865541,
            "unit": "ms",
            "range": "±4.77%",
            "extra": "p95 3.23 ms | 1,034.98 ops/sec | 796 samples"
          },
          {
            "name": "typography",
            "value": 1.197416,
            "unit": "ms",
            "range": "±2.91%",
            "extra": "p95 2.49 ms | 798.3 ops/sec | 743 samples"
          },
          {
            "name": "use-memo",
            "value": 0.276,
            "unit": "ms",
            "range": "±0.94%",
            "extra": "p95 377.92 µs | 3,537.88 ops/sec | 3419 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.318604,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 380.58 µs | 3,096.79 ops/sec | 3044 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.294792,
            "unit": "ms",
            "range": "±0.88%",
            "extra": "p95 375.46 µs | 3,295.24 ops/sec | 3203 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.828001,
            "unit": "ms",
            "range": "±2.40%",
            "extra": "p95 7.9 ms | 168.94 ops/sec | 166 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.225084,
            "unit": "ms",
            "range": "±0.85%",
            "extra": "p95 275.96 µs | 4,342.61 ops/sec | 4220 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.910126,
            "unit": "ms",
            "range": "±5.66%",
            "extra": "p95 7.2 ms | 242.36 ops/sec | 228 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 20.669292,
            "unit": "ms",
            "range": "±3.37%",
            "extra": "p95 26.57 ms | 47.25 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2493.695729,
            "unit": "ms",
            "range": "±30.92%",
            "extra": "p95 6.41 s | 0.36 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 12.259937,
            "unit": "ms",
            "range": "±4.37%",
            "extra": "p95 15.81 ms | 79.81 ops/sec | 40 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.2785,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 1.47 ms | 786.59 ops/sec | 780 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.703666,
            "unit": "ms",
            "range": "±1.38%",
            "extra": "p95 2.43 ms | 575.38 ops/sec | 564 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.755021,
            "unit": "ms",
            "range": "±1.38%",
            "extra": "p95 1.07 ms | 1,321.84 ops/sec | 1268 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 1.13198,
            "unit": "ms",
            "range": "±1.19%",
            "extra": "p95 1.48 ms | 886.02 ops/sec | 864 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.689167,
            "unit": "ms",
            "range": "±1.38%",
            "extra": "p95 980.54 µs | 1,434.36 ops/sec | 1374 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.987042,
            "unit": "ms",
            "range": "±2.58%",
            "extra": "p95 2.33 ms | 976.27 ops/sec | 897 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 2.55675,
            "unit": "ms",
            "range": "±4.07%",
            "extra": "p95 5.37 ms | 376.38 ops/sec | 343 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.889708,
            "unit": "ms",
            "range": "±1.36%",
            "extra": "p95 1.33 ms | 1,114.31 ops/sec | 1073 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.448667,
            "unit": "ms",
            "range": "±1.31%",
            "extra": "p95 749.04 µs | 2,129.47 ops/sec | 2010 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 1.111875,
            "unit": "ms",
            "range": "±2.58%",
            "extra": "p95 2.22 ms | 877.49 ops/sec | 802 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.334438,
            "unit": "ms",
            "range": "±2.14%",
            "extra": "p95 921.88 µs | 2,810.64 ops/sec | 2436 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.855062,
            "unit": "ms",
            "range": "±2.66%",
            "extra": "p95 1.91 ms | 1,130.67 ops/sec | 1002 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.617083,
            "unit": "ms",
            "range": "±2.97%",
            "extra": "p95 1.52 ms | 1,557.57 ops/sec | 1371 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 1.213333,
            "unit": "ms",
            "range": "±5.12%",
            "extra": "p95 3.35 ms | 763.89 ops/sec | 617 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.613417,
            "unit": "ms",
            "range": "±1.31%",
            "extra": "p95 876.25 µs | 1,582.34 ops/sec | 1521 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 1.228791,
            "unit": "ms",
            "range": "±3.31%",
            "extra": "p95 3.02 ms | 773.37 ops/sec | 681 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.332167,
            "unit": "ms",
            "range": "±1.77%",
            "extra": "p95 692.33 µs | 2,863.62 ops/sec | 2585 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.801875,
            "unit": "ms",
            "range": "±1.90%",
            "extra": "p95 1.25 ms | 1,248.23 ops/sec | 1171 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.207375,
            "unit": "ms",
            "range": "±0.86%",
            "extra": "p95 346.29 µs | 4,645.11 ops/sec | 4420 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 1.100584,
            "unit": "ms",
            "range": "±2.31%",
            "extra": "p95 2.03 ms | 874.54 ops/sec | 817 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.635333,
            "unit": "ms",
            "range": "±3.09%",
            "extra": "p95 1.6 ms | 1,466.77 ops/sec | 1298 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 1.221958,
            "unit": "ms",
            "range": "±2.96%",
            "extra": "p95 2.77 ms | 788.92 ops/sec | 709 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.880105,
            "unit": "ms",
            "range": "±2.78%",
            "extra": "p95 1.96 ms | 1,087 ops/sec | 980 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.544167,
            "unit": "ms",
            "range": "±2.56%",
            "extra": "p95 1.41 ms | 1,716.68 ops/sec | 1512 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.891417,
            "unit": "ms",
            "range": "±3.95%",
            "extra": "p95 2.44 ms | 1,090.66 ops/sec | 859 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.638917,
            "unit": "ms",
            "range": "±2.72%",
            "extra": "p95 1.71 ms | 1,476.19 ops/sec | 1263 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 1.3335,
            "unit": "ms",
            "range": "±4.54%",
            "extra": "p95 2.71 ms | 733.51 ops/sec | 655 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.97975,
            "unit": "ms",
            "range": "±1.43%",
            "extra": "p95 1.41 ms | 1,012.42 ops/sec | 973 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.523333,
            "unit": "ms",
            "range": "±1.33%",
            "extra": "p95 827.25 µs | 1,875.89 ops/sec | 1774 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.973292,
            "unit": "ms",
            "range": "±1.33%",
            "extra": "p95 1.42 ms | 1,023.29 ops/sec | 987 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.412583,
            "unit": "ms",
            "range": "±0.87%",
            "extra": "p95 642.21 µs | 2,349.97 ops/sec | 2277 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.382333,
            "unit": "ms",
            "range": "±0.92%",
            "extra": "p95 601.29 µs | 2,421.95 ops/sec | 2343 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.482375,
            "unit": "ms",
            "range": "±1.23%",
            "extra": "p95 878.33 µs | 1,905 ops/sec | 1821 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.317625,
            "unit": "ms",
            "range": "±2.20%",
            "extra": "p95 772.71 µs | 2,985.12 ops/sec | 2635 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.444459,
            "unit": "ms",
            "range": "±1.02%",
            "extra": "p95 680.12 µs | 2,209.73 ops/sec | 2127 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.414834,
            "unit": "ms",
            "range": "±0.65%",
            "extra": "p95 533.54 µs | 2,381.49 ops/sec | 2335 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.445416,
            "unit": "ms",
            "range": "±1.00%",
            "extra": "p95 612 µs | 2,276.14 ops/sec | 2203 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.538541,
            "unit": "ms",
            "range": "±1.83%",
            "extra": "p95 770.58 µs | 1,856.41 ops/sec | 1754 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.996416,
            "unit": "ms",
            "range": "±1.08%",
            "extra": "p95 1.32 ms | 996.45 ops/sec | 973 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 5.694917,
            "unit": "ms",
            "range": "±1.94%",
            "extra": "p95 7.01 ms | 176.85 ops/sec | 175 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 5.664291,
            "unit": "ms",
            "range": "±5.18%",
            "extra": "p95 7.73 ms | 167.34 ops/sec | 160 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.614625,
            "unit": "ms",
            "range": "±1.57%",
            "extra": "p95 958.88 µs | 1,587.17 ops/sec | 1511 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "905d00d49cbb1319cd5853338a1ad2c63b6ed850c20c40f25607eec428b1778c"
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
            "value": 0.5875,
            "unit": "ms",
            "range": "±2.83%",
            "extra": "p95 1.16 ms | 1,647.37 ops/sec | 1484 samples"
          },
          {
            "name": "card",
            "value": 0.221458,
            "unit": "ms",
            "range": "±1.68%",
            "extra": "p95 512.29 µs | 4,179.76 ops/sec | 3717 samples"
          },
          {
            "name": "consts",
            "value": 0.151396,
            "unit": "ms",
            "range": "±2.59%",
            "extra": "p95 290.25 µs | 6,220.55 ops/sec | 5662 samples"
          },
          {
            "name": "counter",
            "value": 0.972396,
            "unit": "ms",
            "range": "±1.84%",
            "extra": "p95 1.57 ms | 1,007.65 ops/sec | 950 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.020479,
            "unit": "ms",
            "range": "±3.60%",
            "extra": "p95 4.06 ms | 492.17 ops/sec | 442 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.098084,
            "unit": "ms",
            "range": "±2.77%",
            "extra": "p95 4.71 ms | 321.04 ops/sec | 308 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.518041,
            "unit": "ms",
            "range": "±1.40%",
            "extra": "p95 842.21 µs | 1,843.69 ops/sec | 1748 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.861646,
            "unit": "ms",
            "range": "±1.83%",
            "extra": "p95 1.44 ms | 1,109.25 ops/sec | 1050 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.978417,
            "unit": "ms",
            "range": "±2.03%",
            "extra": "p95 1.64 ms | 1,027.06 ops/sec | 955 samples"
          },
          {
            "name": "page",
            "value": 0.28275,
            "unit": "ms",
            "range": "±1.18%",
            "extra": "p95 521.33 µs | 3,395.04 ops/sec | 3178 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.50725,
            "unit": "ms",
            "range": "±1.60%",
            "extra": "p95 3.23 ms | 393.89 ops/sec | 387 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.399354,
            "unit": "ms",
            "range": "±2.50%",
            "extra": "p95 3.85 ms | 400.09 ops/sec | 384 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.807583,
            "unit": "ms",
            "range": "±1.90%",
            "extra": "p95 1.41 ms | 1,186.65 ops/sec | 1112 samples"
          },
          {
            "name": "typography",
            "value": 1.72925,
            "unit": "ms",
            "range": "±2.96%",
            "extra": "p95 3.29 ms | 571.12 ops/sec | 523 samples"
          },
          {
            "name": "use-memo",
            "value": 0.332834,
            "unit": "ms",
            "range": "±2.25%",
            "extra": "p95 781.04 µs | 2,843.16 ops/sec | 2473 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.358042,
            "unit": "ms",
            "range": "±0.98%",
            "extra": "p95 580.29 µs | 2,663.79 ops/sec | 2543 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.330334,
            "unit": "ms",
            "range": "±1.13%",
            "extra": "p95 542.33 µs | 2,883.44 ops/sec | 2749 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 6.592917,
            "unit": "ms",
            "range": "±1.82%",
            "extra": "p95 8.23 ms | 149.44 ops/sec | 148 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.2875,
            "unit": "ms",
            "range": "±1.15%",
            "extra": "p95 493.04 µs | 3,349.92 ops/sec | 3149 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 4.089583,
            "unit": "ms",
            "range": "±1.21%",
            "extra": "p95 4.84 ms | 242.24 ops/sec | 241 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 22.194896,
            "unit": "ms",
            "range": "±2.04%",
            "extra": "p95 25 ms | 44.85 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2689.415396,
            "unit": "ms",
            "range": "±6.16%",
            "extra": "p95 3.02 s | 0.37 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 14.471833,
            "unit": "ms",
            "range": "±2.50%",
            "extra": "p95 17.06 ms | 68.5 ops/sec | 35 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.348937,
            "unit": "ms",
            "range": "±1.21%",
            "extra": "p95 1.93 ms | 712.67 ops/sec | 698 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 2.738395,
            "unit": "ms",
            "range": "±5.19%",
            "extra": "p95 5.79 ms | 359.25 ops/sec | 320 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.787646,
            "unit": "ms",
            "range": "±2.31%",
            "extra": "p95 1.5 ms | 1,226.91 ops/sec | 1124 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 1.301563,
            "unit": "ms",
            "range": "±3.26%",
            "extra": "p95 2.44 ms | 741.59 ops/sec | 676 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.7385,
            "unit": "ms",
            "range": "±1.76%",
            "extra": "p95 1.24 ms | 1,327.31 ops/sec | 1244 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 1.208687,
            "unit": "ms",
            "range": "±2.14%",
            "extra": "p95 1.99 ms | 813.43 ops/sec | 764 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 2.355541,
            "unit": "ms",
            "range": "±2.14%",
            "extra": "p95 3.33 ms | 424.56 ops/sec | 409 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 1.042292,
            "unit": "ms",
            "range": "±3.06%",
            "extra": "p95 2.07 ms | 931.92 ops/sec | 843 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.396209,
            "unit": "ms",
            "range": "±2.07%",
            "extra": "p95 821 µs | 2,313.48 ops/sec | 2089 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.920333,
            "unit": "ms",
            "range": "±2.69%",
            "extra": "p95 1.91 ms | 1,061.6 ops/sec | 961 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.368146,
            "unit": "ms",
            "range": "±3.51%",
            "extra": "p95 1.5 ms | 2,504.94 ops/sec | 1906 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.837229,
            "unit": "ms",
            "range": "±3.36%",
            "extra": "p95 2.01 ms | 1,171.26 ops/sec | 1018 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.581792,
            "unit": "ms",
            "range": "±1.61%",
            "extra": "p95 1.03 ms | 1,691.08 ops/sec | 1570 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.919542,
            "unit": "ms",
            "range": "±2.20%",
            "extra": "p95 1.67 ms | 1,052.39 ops/sec | 964 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.471708,
            "unit": "ms",
            "range": "±1.37%",
            "extra": "p95 896.33 µs | 1,952.51 ops/sec | 1841 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.847187,
            "unit": "ms",
            "range": "±1.69%",
            "extra": "p95 1.51 ms | 1,117.51 ops/sec | 1062 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.285875,
            "unit": "ms",
            "range": "±1.22%",
            "extra": "p95 524.96 µs | 3,313.92 ops/sec | 3103 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.584291,
            "unit": "ms",
            "range": "±1.50%",
            "extra": "p95 1.06 ms | 1,562.26 ops/sec | 1469 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.187584,
            "unit": "ms",
            "range": "±0.99%",
            "extra": "p95 355.13 µs | 4,957.88 ops/sec | 4654 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 1.259083,
            "unit": "ms",
            "range": "±3.03%",
            "extra": "p95 2.57 ms | 789.31 ops/sec | 708 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.703208,
            "unit": "ms",
            "range": "±3.13%",
            "extra": "p95 1.75 ms | 1,372.54 ops/sec | 1155 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.88825,
            "unit": "ms",
            "range": "±61.31%",
            "extra": "p95 2.37 ms | 1,028.49 ops/sec | 593 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.834625,
            "unit": "ms",
            "range": "±2.86%",
            "extra": "p95 1.85 ms | 1,161.25 ops/sec | 1023 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.43952,
            "unit": "ms",
            "range": "±2.06%",
            "extra": "p95 631.67 µs | 2,198.68 ops/sec | 2102 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.458292,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 589.08 µs | 2,127.48 ops/sec | 2105 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.473375,
            "unit": "ms",
            "range": "±0.96%",
            "extra": "p95 706.17 µs | 2,061.23 ops/sec | 1994 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.930854,
            "unit": "ms",
            "range": "±1.07%",
            "extra": "p95 1.28 ms | 1,056.91 ops/sec | 1030 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.896042,
            "unit": "ms",
            "range": "±0.78%",
            "extra": "p95 1.15 ms | 1,098.43 ops/sec | 1083 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.741958,
            "unit": "ms",
            "range": "±2.71%",
            "extra": "p95 1.7 ms | 1,417.63 ops/sec | 1201 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 1.637625,
            "unit": "ms",
            "range": "±3.81%",
            "extra": "p95 3.31 ms | 605.86 ops/sec | 537 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.661125,
            "unit": "ms",
            "range": "±1.67%",
            "extra": "p95 1.07 ms | 1,539.94 ops/sec | 1431 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.758583,
            "unit": "ms",
            "range": "±2.84%",
            "extra": "p95 1.88 ms | 1,341.83 ops/sec | 1138 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.891333,
            "unit": "ms",
            "range": "±2.28%",
            "extra": "p95 1.63 ms | 1,132.08 ops/sec | 1028 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.362541,
            "unit": "ms",
            "range": "±1.89%",
            "extra": "p95 774.54 µs | 2,626.37 ops/sec | 2337 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.762208,
            "unit": "ms",
            "range": "±3.03%",
            "extra": "p95 1.88 ms | 1,304.89 ops/sec | 1103 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.718417,
            "unit": "ms",
            "range": "±2.20%",
            "extra": "p95 1.31 ms | 1,406.48 ops/sec | 1269 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.8185,
            "unit": "ms",
            "range": "±2.89%",
            "extra": "p95 1.9 ms | 1,232.69 ops/sec | 1051 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 1.008333,
            "unit": "ms",
            "range": "±3.84%",
            "extra": "p95 2.74 ms | 991.72 ops/sec | 801 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 2.231667,
            "unit": "ms",
            "range": "±4.42%",
            "extra": "p95 4.8 ms | 452.58 ops/sec | 386 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 8.400625,
            "unit": "ms",
            "range": "±3.59%",
            "extra": "p95 12.27 ms | 117.22 ops/sec | 114 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 8.749333,
            "unit": "ms",
            "range": "±2.66%",
            "extra": "p95 11.11 ms | 112.96 ops/sec | 111 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.967229,
            "unit": "ms",
            "range": "±3.47%",
            "extra": "p95 2.62 ms | 1,022.79 ops/sec | 864 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "3f398770d5f0b72f70c122afa4cae25f45afc384cf4b066a7ef3b12ee75553a2"
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
            "value": 0.960333,
            "unit": "ms",
            "range": "±3.98%",
            "extra": "p95 2.46 ms | 1,041.43 ops/sec | 843 samples"
          },
          {
            "name": "card",
            "value": 0.244062,
            "unit": "ms",
            "range": "±1.30%",
            "extra": "p95 466.17 µs | 3,919.48 ops/sec | 3608 samples"
          },
          {
            "name": "consts",
            "value": 0.154792,
            "unit": "ms",
            "range": "±1.27%",
            "extra": "p95 325.62 µs | 6,029.84 ops/sec | 5498 samples"
          },
          {
            "name": "counter",
            "value": 1.044563,
            "unit": "ms",
            "range": "±3.17%",
            "extra": "p95 2.14 ms | 957.07 ops/sec | 850 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.569229,
            "unit": "ms",
            "range": "±3.37%",
            "extra": "p95 3.18 ms | 651.41 ops/sec | 574 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.434084,
            "unit": "ms",
            "range": "±4.30%",
            "extra": "p95 6.76 ms | 275.72 ops/sec | 254 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.592271,
            "unit": "ms",
            "range": "±2.75%",
            "extra": "p95 1.49 ms | 1,607.23 ops/sec | 1400 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.046209,
            "unit": "ms",
            "range": "±4.58%",
            "extra": "p95 3.21 ms | 907.39 ops/sec | 734 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.826375,
            "unit": "ms",
            "range": "±3.53%",
            "extra": "p95 2.18 ms | 1,166.03 ops/sec | 981 samples"
          },
          {
            "name": "page",
            "value": 0.360249,
            "unit": "ms",
            "range": "±4.00%",
            "extra": "p95 1.59 ms | 2,581.14 ops/sec | 1752 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.463979,
            "unit": "ms",
            "range": "±9.39%",
            "extra": "p95 9.67 ms | 275.81 ops/sec | 216 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.373563,
            "unit": "ms",
            "range": "±2.71%",
            "extra": "p95 3.73 ms | 409.23 ops/sec | 390 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.721542,
            "unit": "ms",
            "range": "±2.91%",
            "extra": "p95 1.79 ms | 1,269.21 ops/sec | 1101 samples"
          },
          {
            "name": "typography",
            "value": 1.0865,
            "unit": "ms",
            "range": "±4.36%",
            "extra": "p95 3.33 ms | 834.97 ops/sec | 702 samples"
          },
          {
            "name": "use-memo",
            "value": 0.252855,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 355.25 µs | 3,802.25 ops/sec | 3714 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.326458,
            "unit": "ms",
            "range": "±0.82%",
            "extra": "p95 445.21 µs | 2,980.94 ops/sec | 2901 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.308021,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 423.71 µs | 3,177.13 ops/sec | 3116 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.674042,
            "unit": "ms",
            "range": "±0.96%",
            "extra": "p95 6.22 ms | 175.83 ops/sec | 176 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.322625,
            "unit": "ms",
            "range": "±2.33%",
            "extra": "p95 976 µs | 2,911.37 ops/sec | 2415 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.263271,
            "unit": "ms",
            "range": "±3.67%",
            "extra": "p95 8.21 ms | 187.79 ops/sec | 180 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 24.700709,
            "unit": "ms",
            "range": "±2.24%",
            "extra": "p95 27.42 ms | 41.27 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3376.949562,
            "unit": "ms",
            "range": "±11.77%",
            "extra": "p95 4.36 s | 0.3 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 13.281563,
            "unit": "ms",
            "range": "±9.56%",
            "extra": "p95 25.11 ms | 66.39 ops/sec | 32 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.571791,
            "unit": "ms",
            "range": "±3.24%",
            "extra": "p95 2.89 ms | 628.83 ops/sec | 571 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 2.35375,
            "unit": "ms",
            "range": "±3.65%",
            "extra": "p95 4.65 ms | 427.84 ops/sec | 389 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.704104,
            "unit": "ms",
            "range": "±2.24%",
            "extra": "p95 1.3 ms | 1,363.74 ops/sec | 1262 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 1.162958,
            "unit": "ms",
            "range": "±3.89%",
            "extra": "p95 2.16 ms | 839.61 ops/sec | 769 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.689459,
            "unit": "ms",
            "range": "±1.87%",
            "extra": "p95 1.12 ms | 1,416.59 ops/sec | 1329 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.936875,
            "unit": "ms",
            "range": "±1.50%",
            "extra": "p95 1.43 ms | 1,040.72 ops/sec | 1001 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.891729,
            "unit": "ms",
            "range": "±3.56%",
            "extra": "p95 3.78 ms | 494.11 ops/sec | 448 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.732292,
            "unit": "ms",
            "range": "±2.02%",
            "extra": "p95 1.44 ms | 1,257.64 ops/sec | 1168 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.449437,
            "unit": "ms",
            "range": "±1.87%",
            "extra": "p95 889.54 µs | 2,147.5 ops/sec | 1922 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 1.039292,
            "unit": "ms",
            "range": "±4.07%",
            "extra": "p95 3.04 ms | 954.96 ops/sec | 773 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.327396,
            "unit": "ms",
            "range": "±2.72%",
            "extra": "p95 854.33 µs | 2,894.41 ops/sec | 2426 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.726042,
            "unit": "ms",
            "range": "±4.87%",
            "extra": "p95 2.12 ms | 1,316.66 ops/sec | 1055 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.650958,
            "unit": "ms",
            "range": "±3.84%",
            "extra": "p95 1.54 ms | 1,566.3 ops/sec | 1287 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.7975,
            "unit": "ms",
            "range": "±1.14%",
            "extra": "p95 1.13 ms | 1,213.38 ops/sec | 1183 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.481854,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 669.08 µs | 2,025.42 ops/sec | 1990 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.988188,
            "unit": "ms",
            "range": "±1.42%",
            "extra": "p95 1.48 ms | 997.03 ops/sec | 960 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.267541,
            "unit": "ms",
            "range": "±0.71%",
            "extra": "p95 360.12 µs | 3,631.94 ops/sec | 3556 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.597667,
            "unit": "ms",
            "range": "±1.37%",
            "extra": "p95 1.04 ms | 1,572.56 ops/sec | 1501 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.191625,
            "unit": "ms",
            "range": "±1.63%",
            "extra": "p95 394.83 µs | 4,844.41 ops/sec | 4371 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 1.494541,
            "unit": "ms",
            "range": "±4.02%",
            "extra": "p95 3.52 ms | 681.74 ops/sec | 579 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.554854,
            "unit": "ms",
            "range": "±1.34%",
            "extra": "p95 867.5 µs | 1,731.74 ops/sec | 1668 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.871041,
            "unit": "ms",
            "range": "±4.60%",
            "extra": "p95 1.29 ms | 1,103.27 ops/sec | 1045 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.647959,
            "unit": "ms",
            "range": "±0.90%",
            "extra": "p95 869.71 µs | 1,494.52 ops/sec | 1467 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.432042,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 572.08 µs | 2,245.12 ops/sec | 2210 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.495229,
            "unit": "ms",
            "range": "±1.69%",
            "extra": "p95 779.21 µs | 1,921.19 ops/sec | 1822 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.409417,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 550.96 µs | 2,366.59 ops/sec | 2330 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.797562,
            "unit": "ms",
            "range": "±1.67%",
            "extra": "p95 1.19 ms | 1,208.71 ops/sec | 1170 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.811938,
            "unit": "ms",
            "range": "±1.79%",
            "extra": "p95 1.59 ms | 1,120.31 ops/sec | 1056 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.777917,
            "unit": "ms",
            "range": "±2.33%",
            "extra": "p95 1.56 ms | 1,308.8 ops/sec | 1159 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.864583,
            "unit": "ms",
            "range": "±1.20%",
            "extra": "p95 1.36 ms | 1,101.83 ops/sec | 1073 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.49125,
            "unit": "ms",
            "range": "±1.40%",
            "extra": "p95 864.46 µs | 1,959.59 ops/sec | 1821 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.495375,
            "unit": "ms",
            "range": "±1.57%",
            "extra": "p95 934.96 µs | 1,904.83 ops/sec | 1754 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.54825,
            "unit": "ms",
            "range": "±1.37%",
            "extra": "p95 1.01 ms | 1,678.77 ops/sec | 1597 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.283625,
            "unit": "ms",
            "range": "±1.32%",
            "extra": "p95 557.08 µs | 3,274.16 ops/sec | 3041 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.535042,
            "unit": "ms",
            "range": "±1.52%",
            "extra": "p95 943.46 µs | 1,859.92 ops/sec | 1716 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.634458,
            "unit": "ms",
            "range": "±2.17%",
            "extra": "p95 1.27 ms | 1,588.34 ops/sec | 1403 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.637937,
            "unit": "ms",
            "range": "±2.99%",
            "extra": "p95 1.49 ms | 1,591.88 ops/sec | 1328 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.767042,
            "unit": "ms",
            "range": "±3.72%",
            "extra": "p95 2.27 ms | 1,309.69 ops/sec | 1039 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 1.085166,
            "unit": "ms",
            "range": "±1.79%",
            "extra": "p95 1.75 ms | 907.83 ops/sec | 861 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 7.348187,
            "unit": "ms",
            "range": "±2.65%",
            "extra": "p95 8.56 ms | 145.98 ops/sec | 142 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 8.342,
            "unit": "ms",
            "range": "±2.91%",
            "extra": "p95 10.22 ms | 123.66 ops/sec | 121 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.827459,
            "unit": "ms",
            "range": "±1.65%",
            "extra": "p95 1.3 ms | 1,208.24 ops/sec | 1137 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "1fadc683c449256ef5fb9cacc2a4eb88ab9d76f3e5547d018adfbb86a2a006d3"
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
            "value": 0.521313,
            "unit": "ms",
            "range": "±2.12%",
            "extra": "p95 1.03 ms | 1,792.93 ops/sec | 1608 samples"
          },
          {
            "name": "card",
            "value": 0.215166,
            "unit": "ms",
            "range": "±1.58%",
            "extra": "p95 550.83 µs | 4,278.36 ops/sec | 3761 samples"
          },
          {
            "name": "consts",
            "value": 0.150084,
            "unit": "ms",
            "range": "±2.36%",
            "extra": "p95 406.96 µs | 6,242.95 ops/sec | 5306 samples"
          },
          {
            "name": "counter",
            "value": 0.864334,
            "unit": "ms",
            "range": "±3.71%",
            "extra": "p95 2.63 ms | 1,060.7 ops/sec | 910 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.033771,
            "unit": "ms",
            "range": "±1.27%",
            "extra": "p95 1.27 ms | 955.77 ops/sec | 940 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.565833,
            "unit": "ms",
            "range": "±2.28%",
            "extra": "p95 3.93 ms | 378.34 ops/sec | 369 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.498833,
            "unit": "ms",
            "range": "±4.16%",
            "extra": "p95 2.3 ms | 1,826.64 ops/sec | 1437 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.876084,
            "unit": "ms",
            "range": "±5.04%",
            "extra": "p95 3.39 ms | 1,037.6 ops/sec | 835 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.710542,
            "unit": "ms",
            "range": "±1.53%",
            "extra": "p95 836.17 µs | 1,389.48 ops/sec | 1353 samples"
          },
          {
            "name": "page",
            "value": 0.237459,
            "unit": "ms",
            "range": "±1.01%",
            "extra": "p95 282.25 µs | 4,126.53 ops/sec | 4001 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.244062,
            "unit": "ms",
            "range": "±0.77%",
            "extra": "p95 2.68 ms | 438.27 ops/sec | 436 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.263771,
            "unit": "ms",
            "range": "±1.00%",
            "extra": "p95 2.68 ms | 435.16 ops/sec | 432 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.695667,
            "unit": "ms",
            "range": "±1.02%",
            "extra": "p95 900.5 µs | 1,406.85 ops/sec | 1381 samples"
          },
          {
            "name": "typography",
            "value": 1.117,
            "unit": "ms",
            "range": "±3.13%",
            "extra": "p95 2.39 ms | 846.97 ops/sec | 775 samples"
          },
          {
            "name": "use-memo",
            "value": 0.310812,
            "unit": "ms",
            "range": "±3.54%",
            "extra": "p95 1.03 ms | 2,990.07 ops/sec | 2434 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.340375,
            "unit": "ms",
            "range": "±2.98%",
            "extra": "p95 954.33 µs | 2,778.08 ops/sec | 2396 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.349959,
            "unit": "ms",
            "range": "±3.37%",
            "extra": "p95 1.41 ms | 2,595.73 ops/sec | 2046 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 6.303458,
            "unit": "ms",
            "range": "±1.24%",
            "extra": "p95 6.99 ms | 157.33 ops/sec | 157 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.236583,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 282.79 µs | 4,135.05 ops/sec | 4058 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.861458,
            "unit": "ms",
            "range": "±0.79%",
            "extra": "p95 4.26 ms | 256.03 ops/sec | 256 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 21.11875,
            "unit": "ms",
            "range": "±3.25%",
            "extra": "p95 24.62 ms | 46.5 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2594.518083,
            "unit": "ms",
            "range": "±5.12%",
            "extra": "p95 2.98 s | 0.38 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 14.807979,
            "unit": "ms",
            "range": "±0.84%",
            "extra": "p95 15.65 ms | 67.22 ops/sec | 34 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.457875,
            "unit": "ms",
            "range": "±0.77%",
            "extra": "p95 1.79 ms | 674.15 ops/sec | 669 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 2.044,
            "unit": "ms",
            "range": "±0.74%",
            "extra": "p95 2.3 ms | 482.52 ops/sec | 480 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.637958,
            "unit": "ms",
            "range": "±3.20%",
            "extra": "p95 1.65 ms | 1,460.79 ops/sec | 1292 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 1.035979,
            "unit": "ms",
            "range": "±0.70%",
            "extra": "p95 1.16 ms | 953.57 ops/sec | 948 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.71175,
            "unit": "ms",
            "range": "±3.59%",
            "extra": "p95 2.43 ms | 1,296.18 ops/sec | 1105 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.970041,
            "unit": "ms",
            "range": "±1.21%",
            "extra": "p95 1.23 ms | 1,008.91 ops/sec | 990 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.989292,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 2.23 ms | 499.06 ops/sec | 498 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.78227,
            "unit": "ms",
            "range": "±1.09%",
            "extra": "p95 895.79 µs | 1,258.94 ops/sec | 1240 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.394125,
            "unit": "ms",
            "range": "±1.23%",
            "extra": "p95 464.79 µs | 2,487.48 ops/sec | 2409 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.818938,
            "unit": "ms",
            "range": "±1.38%",
            "extra": "p95 1.02 ms | 1,198.83 ops/sec | 1172 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.282437,
            "unit": "ms",
            "range": "±0.93%",
            "extra": "p95 344.5 µs | 3,470.03 ops/sec | 3386 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.588459,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 674.79 µs | 1,679.07 ops/sec | 1664 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.449583,
            "unit": "ms",
            "range": "±0.88%",
            "extra": "p95 522.75 µs | 2,186.46 ops/sec | 2156 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.969375,
            "unit": "ms",
            "range": "±3.72%",
            "extra": "p95 3 ms | 928.7 ops/sec | 796 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.504333,
            "unit": "ms",
            "range": "±1.89%",
            "extra": "p95 698.17 µs | 1,916.52 ops/sec | 1815 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.889959,
            "unit": "ms",
            "range": "±0.94%",
            "extra": "p95 991.46 µs | 1,113.19 ops/sec | 1100 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.277667,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 317.88 µs | 3,551.76 ops/sec | 3498 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.614917,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 703.79 µs | 1,601.63 ops/sec | 1586 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.191542,
            "unit": "ms",
            "range": "±0.55%",
            "extra": "p95 220.71 µs | 5,134.03 ops/sec | 5053 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 1.071646,
            "unit": "ms",
            "range": "±1.09%",
            "extra": "p95 1.29 ms | 922.58 ops/sec | 910 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.58775,
            "unit": "ms",
            "range": "±1.45%",
            "extra": "p95 711.21 µs | 1,670.51 ops/sec | 1627 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.945208,
            "unit": "ms",
            "range": "±5.30%",
            "extra": "p95 1.35 ms | 1,028.12 ops/sec | 952 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.909583,
            "unit": "ms",
            "range": "±7.34%",
            "extra": "p95 3.95 ms | 981.24 ops/sec | 670 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.488916,
            "unit": "ms",
            "range": "±4.78%",
            "extra": "p95 1.68 ms | 1,881.53 ops/sec | 1525 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.501396,
            "unit": "ms",
            "range": "±0.79%",
            "extra": "p95 599.87 µs | 1,960.12 ops/sec | 1934 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.461667,
            "unit": "ms",
            "range": "±1.03%",
            "extra": "p95 532.79 µs | 2,138.02 ops/sec | 2099 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.9435,
            "unit": "ms",
            "range": "±2.30%",
            "extra": "p95 1.12 ms | 1,044.12 ops/sec | 1015 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.983708,
            "unit": "ms",
            "range": "±1.40%",
            "extra": "p95 1.22 ms | 999.55 ops/sec | 980 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.480833,
            "unit": "ms",
            "range": "±2.56%",
            "extra": "p95 703.5 µs | 2,017.99 ops/sec | 1903 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 1.025083,
            "unit": "ms",
            "range": "±1.13%",
            "extra": "p95 1.31 ms | 956.86 ops/sec | 942 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.494583,
            "unit": "ms",
            "range": "±2.11%",
            "extra": "p95 866.67 µs | 1,919.1 ops/sec | 1771 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.638208,
            "unit": "ms",
            "range": "±4.47%",
            "extra": "p95 2.63 ms | 1,448.41 ops/sec | 1097 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.666667,
            "unit": "ms",
            "range": "±3.32%",
            "extra": "p95 1.72 ms | 1,398.44 ops/sec | 1220 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.281959,
            "unit": "ms",
            "range": "±0.70%",
            "extra": "p95 352.42 µs | 3,458.97 ops/sec | 3388 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.474,
            "unit": "ms",
            "range": "±1.51%",
            "extra": "p95 661.79 µs | 2,049.13 ops/sec | 1965 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.452083,
            "unit": "ms",
            "range": "±2.20%",
            "extra": "p95 693.04 µs | 2,120.39 ops/sec | 1968 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.460416,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 533.42 µs | 2,150.21 ops/sec | 2127 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.541958,
            "unit": "ms",
            "range": "±1.48%",
            "extra": "p95 727.46 µs | 1,801.68 ops/sec | 1744 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 1.001812,
            "unit": "ms",
            "range": "±2.18%",
            "extra": "p95 1.48 ms | 966.68 ops/sec | 926 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 6.021105,
            "unit": "ms",
            "range": "±3.25%",
            "extra": "p95 9.74 ms | 161.37 ops/sec | 158 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 7.249208,
            "unit": "ms",
            "range": "±6.29%",
            "extra": "p95 14.54 ms | 122.25 ops/sec | 114 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.9905,
            "unit": "ms",
            "range": "±4.74%",
            "extra": "p95 3.23 ms | 930.83 ops/sec | 705 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-apple-darwin",
          "nativeSha256": "4fbb674d89d8d185a69b2181cd52e65ef70cf1a0440f08392febcaeafa6fcac6"
        }
      }
    ]
  }
};
