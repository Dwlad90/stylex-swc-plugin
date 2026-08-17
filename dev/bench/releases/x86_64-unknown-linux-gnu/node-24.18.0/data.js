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
        "date": 1785964764353,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "button-props",
            "value": 0.186384,
            "range": "±0.35%",
            "unit": "ms",
            "extra": "p95 223.67 µs | 5,182.7 ops/sec | 5128 samples"
          },
          {
            "name": "buttons-demo",
            "value": 0.723685,
            "range": "±0.20%",
            "unit": "ms",
            "extra": "p95 749.95 µs | 1,381.79 ops/sec | 1381 samples"
          },
          {
            "name": "card",
            "value": 0.328393,
            "range": "±0.20%",
            "unit": "ms",
            "extra": "p95 358.95 µs | 2,990.18 ops/sec | 2984 samples"
          },
          {
            "name": "consts",
            "value": 0.12936,
            "range": "±0.27%",
            "unit": "ms",
            "extra": "p95 158.45 µs | 7,461.14 ops/sec | 7391 samples"
          },
          {
            "name": "counter",
            "value": 1.373078,
            "range": "±0.20%",
            "unit": "ms",
            "extra": "p95 1.41 ms | 725.41 ops/sec | 725 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.852712,
            "range": "±0.15%",
            "unit": "ms",
            "extra": "p95 1.9 ms | 538.38 ops/sec | 539 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.560362,
            "range": "±0.26%",
            "unit": "ms",
            "extra": "p95 2.63 ms | 388.59 ops/sec | 389 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.591299,
            "range": "±0.25%",
            "unit": "ms",
            "extra": "p95 623.41 µs | 1,688.37 ops/sec | 1685 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.959881,
            "range": "±0.25%",
            "unit": "ms",
            "extra": "p95 1 ms | 1,035.56 ops/sec | 1035 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.883303,
            "range": "±0.26%",
            "unit": "ms",
            "extra": "p95 913.11 µs | 1,127.14 ops/sec | 1126 samples"
          },
          {
            "name": "page",
            "value": 0.43856,
            "range": "±0.28%",
            "unit": "ms",
            "extra": "p95 466.89 µs | 2,260.74 ops/sec | 2254 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.156877,
            "range": "±0.18%",
            "unit": "ms",
            "extra": "p95 3.22 ms | 315.95 ops/sec | 316 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.077183,
            "range": "±0.16%",
            "unit": "ms",
            "extra": "p95 3.15 ms | 323.96 ops/sec | 324 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.258071,
            "range": "±0.41%",
            "unit": "ms",
            "extra": "p95 1.31 ms | 788.55 ops/sec | 787 samples"
          },
          {
            "name": "typography",
            "value": 1.818686,
            "range": "±0.13%",
            "unit": "ms",
            "extra": "p95 1.86 ms | 548.59 ops/sec | 549 samples"
          },
          {
            "name": "use-memo",
            "value": 0.481198,
            "range": "±0.17%",
            "unit": "ms",
            "extra": "p95 502.47 µs | 2,079.59 ops/sec | 2077 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.381401,
            "range": "±0.20%",
            "unit": "ms",
            "extra": "p95 404.79 µs | 2,602.85 ops/sec | 2598 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.344832,
            "range": "±0.21%",
            "unit": "ms",
            "extra": "p95 372.45 µs | 2,862.4 ops/sec | 2856 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 8.113121,
            "range": "±0.26%",
            "unit": "ms",
            "extra": "p95 8.28 ms | 122.93 ops/sec | 123 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.425611,
            "range": "±0.19%",
            "unit": "ms",
            "extra": "p95 449.78 µs | 2,333.12 ops/sec | 2330 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.821888,
            "range": "±0.49%",
            "unit": "ms",
            "extra": "p95 5.94 ms | 171.06 ops/sec | 171 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4149.523279,
            "range": "±0.30%",
            "unit": "ms",
            "extra": "p95 4,170.59 ms | 0.24 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 22.661317,
            "range": "±1.91%",
            "unit": "ms",
            "extra": "p95 25.46 ms | 43.59 ops/sec | 22 samples"
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
            "value": 0.812622,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 858.45 µs | 1,224.75 ops/sec | 1223 samples"
          },
          {
            "name": "card",
            "value": 0.342095,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 361.78 µs | 2,885.33 ops/sec | 2881 samples"
          },
          {
            "name": "consts",
            "value": 0.146588,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 167.73 µs | 6,662.17 ops/sec | 6626 samples"
          },
          {
            "name": "counter",
            "value": 1.470339,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 1.52 ms | 677.43 ops/sec | 678 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.982161,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 2.07 ms | 502.02 ops/sec | 502 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.919829,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 3.02 ms | 342.29 ops/sec | 343 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.656261,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 679.5 µs | 1,523.86 ops/sec | 1523 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.038237,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 1.08 ms | 960.29 ops/sec | 960 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.951534,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 979.26 µs | 1,047.03 ops/sec | 1047 samples"
          },
          {
            "name": "page",
            "value": 0.462354,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 481.58 µs | 2,158.96 ops/sec | 2157 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.406686,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 3.53 ms | 292.92 ops/sec | 293 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.279072,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 3.38 ms | 303.79 ops/sec | 304 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.340725,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.38 ms | 742.78 ops/sec | 743 samples"
          },
          {
            "name": "typography",
            "value": 1.964204,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 2.01 ms | 507.69 ops/sec | 508 samples"
          },
          {
            "name": "use-memo",
            "value": 0.51574,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 542.7 µs | 1,935.35 ops/sec | 1934 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.417843,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 447.27 µs | 2,385.48 ops/sec | 2383 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.367047,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 393.93 µs | 2,696.86 ops/sec | 2692 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.57363,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 9.82 ms | 104.68 ops/sec | 105 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.444451,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 464.61 µs | 2,240.52 ops/sec | 2238 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.085953,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 6.37 ms | 163.3 ops/sec | 164 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4035.898154,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 4.1 s | 0.25 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 30.526341,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 31.54 ms | 32.59 ops/sec | 17 samples"
          }
        ],
        "release": {
          "ref": "fix_release-benchmarks",
          "candidateVersion": "0.18.3",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "32b6b4f4c1e0b9ca2243b5372447f2730cbd185a6950c35775700e7c83bbb832"
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
            "value": 0.791754,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 821.89 µs | 1,262.63 ops/sec | 1262 samples"
          },
          {
            "name": "card",
            "value": 0.354973,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 383.31 µs | 2,777.35 ops/sec | 2773 samples"
          },
          {
            "name": "consts",
            "value": 0.164057,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 192.79 µs | 5,919.32 ops/sec | 5878 samples"
          },
          {
            "name": "counter",
            "value": 1.484743,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.52 ms | 672.13 ops/sec | 672 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.99689,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 2.03 ms | 499.58 ops/sec | 500 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.808921,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 2.86 ms | 355.96 ops/sec | 356 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.67217,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 700.05 µs | 1,485.49 ops/sec | 1484 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.068857,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 1.11 ms | 931.93 ops/sec | 932 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.984378,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.01 ms | 1,013.43 ops/sec | 1014 samples"
          },
          {
            "name": "page",
            "value": 0.451683,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 477.09 µs | 2,195.54 ops/sec | 2193 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.346861,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 3.41 ms | 298.25 ops/sec | 299 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.266361,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 3.31 ms | 305.74 ops/sec | 306 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.391569,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 1.43 ms | 716.31 ops/sec | 717 samples"
          },
          {
            "name": "typography",
            "value": 1.966733,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 2.01 ms | 508.95 ops/sec | 509 samples"
          },
          {
            "name": "use-memo",
            "value": 0.50312,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 521.91 µs | 1,997.43 ops/sec | 1993 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.436726,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 467.1 µs | 2,274.55 ops/sec | 2271 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.386867,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 417.33 µs | 2,561.25 ops/sec | 2556 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 8.218227,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 8.45 ms | 121.24 ops/sec | 122 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.468996,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 495.75 µs | 2,125.95 ops/sec | 2123 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.259101,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 6.4 ms | 159.31 ops/sec | 160 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3995.349644,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 4.02 s | 0.25 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.299855,
            "unit": "ms",
            "range": "±0.88%",
            "extra": "p95 24.15 ms | 42.75 ops/sec | 22 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "115ad8d87a9a149741efcf4614d5088bbca607fa40a2d29122886926e8b16ea4"
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
            "value": 0.80647,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 836.98 µs | 1,239.07 ops/sec | 1239 samples"
          },
          {
            "name": "card",
            "value": 0.342869,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 364 µs | 2,891.03 ops/sec | 2887 samples"
          },
          {
            "name": "consts",
            "value": 0.143471,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 167.78 µs | 6,775.04 ops/sec | 6729 samples"
          },
          {
            "name": "counter",
            "value": 1.464296,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 1.5 ms | 681.37 ops/sec | 682 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.932276,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 1.97 ms | 516.32 ops/sec | 517 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.775256,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 2.85 ms | 358.53 ops/sec | 359 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.655386,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 679.1 µs | 1,526.62 ops/sec | 1526 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.048655,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 1.08 ms | 950.46 ops/sec | 950 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.955118,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 986.77 µs | 1,043.49 ops/sec | 1043 samples"
          },
          {
            "name": "page",
            "value": 0.45961,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 481.79 µs | 2,173.42 ops/sec | 2171 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.292682,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 3.35 ms | 303.15 ops/sec | 304 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.217273,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 3.27 ms | 310.16 ops/sec | 311 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.362051,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 1.4 ms | 732.08 ops/sec | 732 samples"
          },
          {
            "name": "typography",
            "value": 1.948075,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 1.98 ms | 512.57 ops/sec | 513 samples"
          },
          {
            "name": "use-memo",
            "value": 0.514362,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 539.96 µs | 1,941.69 ops/sec | 1939 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.422195,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 449.94 µs | 2,361.56 ops/sec | 2359 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.366855,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 387.15 µs | 2,701.16 ops/sec | 2698 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.249788,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 9.61 ms | 107.85 ops/sec | 108 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.44513,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 477.03 µs | 2,227.35 ops/sec | 2215 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.848816,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 5.96 ms | 170.41 ops/sec | 171 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3910.180519,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 3.93 s | 0.26 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 25.54702,
            "unit": "ms",
            "range": "±2.29%",
            "extra": "p95 27.77 ms | 38.87 ops/sec | 20 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "115ad8d87a9a149741efcf4614d5088bbca607fa40a2d29122886926e8b16ea4"
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
            "value": 0.782727,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 815.1 µs | 1,277.23 ops/sec | 1276 samples"
          },
          {
            "name": "card",
            "value": 0.347206,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 380.08 µs | 2,832.79 ops/sec | 2826 samples"
          },
          {
            "name": "consts",
            "value": 0.16019,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 189.75 µs | 6,044.11 ops/sec | 6004 samples"
          },
          {
            "name": "counter",
            "value": 1.474635,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.52 ms | 675.49 ops/sec | 675 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.992846,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 2.05 ms | 500.4 ops/sec | 501 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.857983,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 2.9 ms | 349.58 ops/sec | 350 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.695795,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 727.47 µs | 1,435.86 ops/sec | 1435 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.048506,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 1.08 ms | 949.22 ops/sec | 949 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.96564,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 999.39 µs | 1,030.63 ops/sec | 1030 samples"
          },
          {
            "name": "page",
            "value": 0.45873,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 486.57 µs | 2,173.9 ops/sec | 2171 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.375228,
            "unit": "ms",
            "range": "±0.82%",
            "extra": "p95 3.47 ms | 294.06 ops/sec | 294 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.284843,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 3.34 ms | 303.91 ops/sec | 304 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.371861,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.42 ms | 726.08 ops/sec | 726 samples"
          },
          {
            "name": "typography",
            "value": 1.994679,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 2.05 ms | 499.06 ops/sec | 499 samples"
          },
          {
            "name": "use-memo",
            "value": 0.506278,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 529.71 µs | 1,985.12 ops/sec | 1983 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.437521,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 466.71 µs | 2,270.29 ops/sec | 2267 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.396704,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 426.97 µs | 2,500.95 ops/sec | 2495 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 8.319255,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 8.47 ms | 120.15 ops/sec | 121 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.470647,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 496.83 µs | 2,124.78 ops/sec | 2122 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.308926,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 6.42 ms | 158.19 ops/sec | 159 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3954.457197,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 3.97 s | 0.25 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.380772,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 23.86 ms | 42.64 ops/sec | 22 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "ad5f7526ad6fba112b2ba76c7daa32aa615c1af7a7de81716ac06be71efbd099"
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
            "value": 0.803417,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 864.51 µs | 1,239.4 ops/sec | 1238 samples"
          },
          {
            "name": "card",
            "value": 0.353913,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 388.59 µs | 2,780.98 ops/sec | 2774 samples"
          },
          {
            "name": "consts",
            "value": 0.162757,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 195.92 µs | 5,946.51 ops/sec | 5901 samples"
          },
          {
            "name": "counter",
            "value": 1.467162,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 1.52 ms | 679.69 ops/sec | 680 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.99,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 2.11 ms | 498.82 ops/sec | 499 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.846006,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 3.04 ms | 347.85 ops/sec | 348 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.683462,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 723.65 µs | 1,465.72 ops/sec | 1464 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.078618,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.12 ms | 923.25 ops/sec | 923 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.008404,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 1.08 ms | 985.32 ops/sec | 985 samples"
          },
          {
            "name": "page",
            "value": 0.473985,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 509.45 µs | 2,110.33 ops/sec | 2106 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.561688,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 3.84 ms | 280.11 ops/sec | 280 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.385926,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 3.64 ms | 293.17 ops/sec | 293 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.441112,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 1.53 ms | 690 ops/sec | 689 samples"
          },
          {
            "name": "typography",
            "value": 2.015758,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 2.12 ms | 493.1 ops/sec | 493 samples"
          },
          {
            "name": "use-memo",
            "value": 0.519171,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 550.91 µs | 1,930.05 ops/sec | 1927 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.45077,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 485.83 µs | 2,212.37 ops/sec | 2208 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.403301,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 437.18 µs | 2,470.65 ops/sec | 2465 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 10.061227,
            "unit": "ms",
            "range": "±0.83%",
            "extra": "p95 10.73 ms | 98.82 ops/sec | 99 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.475728,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 512.44 µs | 2,101.38 ops/sec | 2097 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 7.369848,
            "unit": "ms",
            "range": "±0.88%",
            "extra": "p95 8.07 ms | 136.41 ops/sec | 137 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4197.23524,
            "unit": "ms",
            "range": "±1.71%",
            "extra": "p95 4.32 s | 0.24 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 24.001767,
            "unit": "ms",
            "range": "±2.94%",
            "extra": "p95 27.33 ms | 40.28 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "ad5f7526ad6fba112b2ba76c7daa32aa615c1af7a7de81716ac06be71efbd099"
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
            "value": 0.797157,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 851.83 µs | 1,247.34 ops/sec | 1244 samples"
          },
          {
            "name": "card",
            "value": 0.353096,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 386.42 µs | 2,791.31 ops/sec | 2785 samples"
          },
          {
            "name": "consts",
            "value": 0.163083,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 194.25 µs | 5,923.41 ops/sec | 5881 samples"
          },
          {
            "name": "counter",
            "value": 1.478368,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.53 ms | 674.79 ops/sec | 675 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.994838,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 2.08 ms | 498.99 ops/sec | 499 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.835721,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 2.97 ms | 350.35 ops/sec | 351 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.676728,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 714.98 µs | 1,476.85 ops/sec | 1475 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.079226,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.13 ms | 923.86 ops/sec | 924 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.99,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.03 ms | 1,007.2 ops/sec | 1007 samples"
          },
          {
            "name": "page",
            "value": 0.463327,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 492 µs | 2,162.99 ops/sec | 2160 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.363868,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 3.47 ms | 296.08 ops/sec | 296 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.291658,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 3.43 ms | 302.2 ops/sec | 303 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.384859,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 1.44 ms | 719.71 ops/sec | 720 samples"
          },
          {
            "name": "typography",
            "value": 1.99321,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 2.06 ms | 499.43 ops/sec | 500 samples"
          },
          {
            "name": "use-memo",
            "value": 0.508425,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 538.54 µs | 1,974.43 ops/sec | 1969 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.44589,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 480.5 µs | 2,238.81 ops/sec | 2234 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.394633,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 429.99 µs | 2,506.31 ops/sec | 2501 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 8.333559,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 9.23 ms | 118.74 ops/sec | 119 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.475033,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 503.36 µs | 2,115.1 ops/sec | 2112 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.34338,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 6.7 ms | 156.68 ops/sec | 157 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4110.029935,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 4.15 s | 0.24 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 24.288657,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 24.84 ms | 41.05 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-rc.1",
          "candidateVersion": "0.18.4-rc.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "918133a64cbbfd4afc8b3c72c2f11553968daae53629264a0474f079062a53e2"
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
            "value": 0.675079,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 732.01 µs | 1,489.04 ops/sec | 1483 samples"
          },
          {
            "name": "card",
            "value": 0.192001,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 232.76 µs | 5,137.77 ops/sec | 5078 samples"
          },
          {
            "name": "consts",
            "value": 0.090752,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 111.63 µs | 11,002.53 ops/sec | 10882 samples"
          },
          {
            "name": "counter",
            "value": 1.094324,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 1.2 ms | 896.62 ops/sec | 895 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.520494,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 1.64 ms | 644.97 ops/sec | 644 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.367053,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 2.58 ms | 415.01 ops/sec | 415 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.5149,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 574.51 µs | 1,919.08 ops/sec | 1913 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.007786,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 1.19 ms | 961.58 ops/sec | 955 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.916844,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 1.01 ms | 1,078.05 ops/sec | 1076 samples"
          },
          {
            "name": "page",
            "value": 0.280795,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 326.34 µs | 3,555.71 ops/sec | 3528 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.702881,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 2.93 ms | 362.8 ops/sec | 363 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.684073,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 2.91 ms | 367.18 ops/sec | 367 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.951955,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 1.05 ms | 1,029.26 ops/sec | 1026 samples"
          },
          {
            "name": "typography",
            "value": 1.496555,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 1.64 ms | 653.48 ops/sec | 653 samples"
          },
          {
            "name": "use-memo",
            "value": 0.377735,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 425.09 µs | 2,646.91 ops/sec | 2634 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.312263,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 348.84 µs | 3,175.08 ops/sec | 3161 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.283557,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 312.1 µs | 3,541.93 ops/sec | 3527 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 8.13891,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 8.65 ms | 122.23 ops/sec | 123 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.246249,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 291.82 µs | 4,024.27 ops/sec | 3978 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 4.856024,
            "unit": "ms",
            "range": "±0.79%",
            "extra": "p95 5.13 ms | 206.26 ops/sec | 206 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3679.165427,
            "unit": "ms",
            "range": "±0.97%",
            "extra": "p95 3.73 s | 0.27 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 19.976039,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 20.56 ms | 49.85 ops/sec | 25 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-rc.2",
          "candidateVersion": "0.18.4-rc.2",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "dd3bec037bd599727f2c689b117e7ec6344c0342e9c1a528e182292486e21d15"
        }
      }
    ]
  }
};
