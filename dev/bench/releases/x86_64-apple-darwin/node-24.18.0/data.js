window.BENCHMARK_DATA = {
  "lastUpdate": 1786987572898,
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
      }
    ]
  }
};
