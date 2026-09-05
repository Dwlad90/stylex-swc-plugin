window.BENCHMARK_DATA = {
  "lastUpdate": 1788611357250,
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
            "value": 0.591334,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 621.43 µs | 1,688.33 ops/sec | 1687 samples"
          },
          {
            "name": "card",
            "value": 0.263454,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 287.08 µs | 3,729.69 ops/sec | 3720 samples"
          },
          {
            "name": "consts",
            "value": 0.103416,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 123.13 µs | 9,415.27 ops/sec | 9363 samples"
          },
          {
            "name": "counter",
            "value": 1.11401,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 1.14 ms | 895.25 ops/sec | 896 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.503068,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 1.55 ms | 663.41 ops/sec | 664 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.131944,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 2.18 ms | 467.93 ops/sec | 468 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.479259,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 500.49 µs | 2,083.32 ops/sec | 2082 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.776234,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 797.55 µs | 1,287.02 ops/sec | 1287 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.712017,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 732.69 µs | 1,404.25 ops/sec | 1404 samples"
          },
          {
            "name": "page",
            "value": 0.347696,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 369.58 µs | 2,838.49 ops/sec | 2835 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.59863,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 2.66 ms | 384.25 ops/sec | 385 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.536611,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 2.61 ms | 393.6 ops/sec | 394 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.020864,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 1.05 ms | 976.75 ops/sec | 977 samples"
          },
          {
            "name": "typography",
            "value": 1.498496,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 1.58 ms | 659.09 ops/sec | 657 samples"
          },
          {
            "name": "use-memo",
            "value": 0.378963,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 401.18 µs | 2,612.3 ops/sec | 2609 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.306708,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 329.54 µs | 3,217.47 ops/sec | 3208 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.276127,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 298.16 µs | 3,570.87 ops/sec | 3565 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 6.81185,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 6.96 ms | 146.93 ops/sec | 147 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.339413,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 363.16 µs | 2,908.7 ops/sec | 2905 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 4.892458,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 5.04 ms | 204.07 ops/sec | 205 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3603.325679,
            "unit": "ms",
            "range": "±0.75%",
            "extra": "p95 3.69 s | 0.28 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 19.842682,
            "unit": "ms",
            "range": "±1.88%",
            "extra": "p95 21.41 ms | 50.35 ops/sec | 26 samples"
          }
        ],
        "release": {
          "ref": "0.18.4",
          "candidateVersion": "0.18.4",
          "previousVersion": "0.18.3",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "2c62bb3b01736243a63c2c07d021954e3d93a497787d351739e0f1a83aac462d"
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
            "value": 0.799447,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 831.98 µs | 1,248.47 ops/sec | 1246 samples"
          },
          {
            "name": "card",
            "value": 0.349479,
            "unit": "ms",
            "range": "±0.47%",
            "extra": "p95 386.57 µs | 2,804.59 ops/sec | 2780 samples"
          },
          {
            "name": "consts",
            "value": 0.159108,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 187.92 µs | 6,104.85 ops/sec | 6073 samples"
          },
          {
            "name": "counter",
            "value": 1.468177,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 1.51 ms | 679.3 ops/sec | 679 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.979475,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 2.04 ms | 503.14 ops/sec | 503 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.771306,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 2.82 ms | 360.21 ops/sec | 361 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.680349,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 702.58 µs | 1,476.89 ops/sec | 1476 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.073308,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.1 ms | 932.69 ops/sec | 933 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.990915,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.01 ms | 1,008.72 ops/sec | 1009 samples"
          },
          {
            "name": "page",
            "value": 0.457777,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 484.47 µs | 2,176.05 ops/sec | 2173 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.33913,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 3.4 ms | 298.99 ops/sec | 299 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.277248,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 3.33 ms | 304.04 ops/sec | 304 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.369979,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 1.41 ms | 727.95 ops/sec | 728 samples"
          },
          {
            "name": "typography",
            "value": 1.979074,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 2.02 ms | 504.2 ops/sec | 504 samples"
          },
          {
            "name": "use-memo",
            "value": 0.506898,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 529.64 µs | 1,980.29 ops/sec | 1979 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.442383,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 468.97 µs | 2,249.92 ops/sec | 2246 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.389409,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 417.4 µs | 2,555.75 ops/sec | 2551 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 8.249762,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 8.64 ms | 120.42 ops/sec | 121 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.459064,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 484.5 µs | 2,168.85 ops/sec | 2166 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.167925,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 6.31 ms | 161.35 ops/sec | 162 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4082.702452,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 4.13 s | 0.25 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.313089,
            "unit": "ms",
            "range": "±1.63%",
            "extra": "p95 24.8 ms | 42.4 ops/sec | 22 samples"
          }
        ],
        "release": {
          "ref": "0.18.5-rc.1",
          "candidateVersion": "0.18.5-rc.1",
          "previousVersion": "0.18.4",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "81dcf158d8b5792d6bcb4414d653f788247b65e5c132b4e0e21c307309cea904"
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
            "value": 0.736483,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 769.99 µs | 1,355.58 ops/sec | 1355 samples"
          },
          {
            "name": "card",
            "value": 0.337946,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 369.11 µs | 2,913.66 ops/sec | 2898 samples"
          },
          {
            "name": "consts",
            "value": 0.132136,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 157.27 µs | 7,346.4 ops/sec | 7302 samples"
          },
          {
            "name": "counter",
            "value": 1.402671,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 1.43 ms | 711.85 ops/sec | 712 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.891627,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 1.93 ms | 527.36 ops/sec | 528 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.692856,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 2.75 ms | 370.42 ops/sec | 371 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.604106,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 629.49 µs | 1,657.06 ops/sec | 1656 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.972818,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 998.14 µs | 1,024.28 ops/sec | 1024 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.898252,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 921.48 µs | 1,111.17 ops/sec | 1111 samples"
          },
          {
            "name": "page",
            "value": 0.440954,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 464.8 µs | 2,254.89 ops/sec | 2253 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.22175,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 3.27 ms | 309.86 ops/sec | 310 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.151226,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 3.21 ms | 316.66 ops/sec | 317 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.282093,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 1.32 ms | 777.15 ops/sec | 777 samples"
          },
          {
            "name": "typography",
            "value": 1.88418,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 1.93 ms | 529.65 ops/sec | 530 samples"
          },
          {
            "name": "use-memo",
            "value": 0.482756,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 501.44 µs | 2,078.27 ops/sec | 2077 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.393333,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 417.12 µs | 2,527.76 ops/sec | 2524 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.352803,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 378.55 µs | 2,801.65 ops/sec | 2797 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 8.458371,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 8.59 ms | 118.11 ops/sec | 119 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.428996,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 452.07 µs | 2,312.94 ops/sec | 2311 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.00525,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 6.09 ms | 166.34 ops/sec | 167 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4142.297343,
            "unit": "ms",
            "range": "±0.68%",
            "extra": "p95 4.23 s | 0.24 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 22.810361,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 23.01 ms | 43.72 ops/sec | 22 samples"
          }
        ],
        "release": {
          "ref": "0.18.5",
          "candidateVersion": "0.18.5",
          "previousVersion": "0.18.4",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "83b1f5e49c3a7d358177bfb9e56bdf8dae79c46fef61314526fad66e81ac408d"
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
            "value": 0.828395,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 871.69 µs | 1,200.8 ops/sec | 1198 samples"
          },
          {
            "name": "card",
            "value": 0.36186,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 396.96 µs | 2,724.37 ops/sec | 2718 samples"
          },
          {
            "name": "consts",
            "value": 0.164798,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 195.13 µs | 5,897.73 ops/sec | 5864 samples"
          },
          {
            "name": "counter",
            "value": 1.510477,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 1.55 ms | 660.58 ops/sec | 661 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.027998,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 2.09 ms | 491.38 ops/sec | 492 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.869733,
            "unit": "ms",
            "range": "±0.14%",
            "extra": "p95 2.93 ms | 347.97 ops/sec | 348 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.693559,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 720.34 µs | 1,449.38 ops/sec | 1448 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.097817,
            "unit": "ms",
            "range": "±0.11%",
            "extra": "p95 1.13 ms | 907.24 ops/sec | 907 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.014612,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 1.05 ms | 979.64 ops/sec | 978 samples"
          },
          {
            "name": "page",
            "value": 0.467738,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 496.82 µs | 2,129.26 ops/sec | 2126 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.383681,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 3.45 ms | 295.02 ops/sec | 295 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.352633,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 3.43 ms | 297.72 ops/sec | 298 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.420143,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 1.46 ms | 702.17 ops/sec | 702 samples"
          },
          {
            "name": "typography",
            "value": 2.026896,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 2.1 ms | 491.66 ops/sec | 492 samples"
          },
          {
            "name": "use-memo",
            "value": 0.523747,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 550.88 µs | 1,916.36 ops/sec | 1914 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.469556,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 498.5 µs | 2,123.12 ops/sec | 2121 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.411978,
            "unit": "ms",
            "range": "±0.70%",
            "extra": "p95 582.84 µs | 2,351.11 ops/sec | 2310 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 8.499798,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 8.95 ms | 117.19 ops/sec | 118 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.466771,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 494.05 µs | 2,134.17 ops/sec | 2131 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.334444,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 6.46 ms | 157.63 ops/sec | 158 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4078.929729,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 4.11 s | 0.25 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.699481,
            "unit": "ms",
            "range": "±0.91%",
            "extra": "p95 25.08 ms | 41.89 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "0.18.6-rc.1",
          "candidateVersion": "0.18.6-rc.1",
          "previousVersion": "0.18.5",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "895facd824a2565eeecff0f503e7e8f8b2dc37a31f93ca8cbd38d0d0dc18f0d1"
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
            "value": 0.854968,
            "unit": "ms",
            "range": "±0.69%",
            "extra": "p95 923.04 µs | 1,157.14 ops/sec | 1149 samples"
          },
          {
            "name": "card",
            "value": 0.374069,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 409.89 µs | 2,644.67 ops/sec | 2637 samples"
          },
          {
            "name": "consts",
            "value": 0.173664,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 213.47 µs | 5,556.96 ops/sec | 5514 samples"
          },
          {
            "name": "counter",
            "value": 1.564744,
            "unit": "ms",
            "range": "±0.85%",
            "extra": "p95 1.66 ms | 632.41 ops/sec | 629 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.109182,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 2.22 ms | 470.37 ops/sec | 470 samples"
          },
          {
            "name": "global-tokens",
            "value": 3.050497,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 3.2 ms | 325.93 ops/sec | 326 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.726478,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 769.97 µs | 1,375.05 ops/sec | 1374 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.11276,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.17 ms | 894.09 ops/sec | 894 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.029875,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.08 ms | 966.81 ops/sec | 966 samples"
          },
          {
            "name": "page",
            "value": 0.493091,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 529.37 µs | 2,031 ops/sec | 2026 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.578978,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 3.8 ms | 277.78 ops/sec | 278 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.441661,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 3.62 ms | 288.61 ops/sec | 289 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.437095,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.5 ms | 692.25 ops/sec | 692 samples"
          },
          {
            "name": "typography",
            "value": 2.074171,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 2.16 ms | 479.82 ops/sec | 480 samples"
          },
          {
            "name": "use-memo",
            "value": 0.533297,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 563.19 µs | 1,883.87 ops/sec | 1881 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.485412,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 522.61 µs | 2,056.3 ops/sec | 2052 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.424322,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 461.24 µs | 2,342.45 ops/sec | 2337 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.248381,
            "unit": "ms",
            "range": "±0.82%",
            "extra": "p95 9.89 ms | 107.94 ops/sec | 108 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.485221,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 511.31 µs | 2,068.27 ops/sec | 2066 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.655836,
            "unit": "ms",
            "range": "±0.68%",
            "extra": "p95 7.13 ms | 149.03 ops/sec | 149 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4337.759082,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 4.4 s | 0.23 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 27.89078,
            "unit": "ms",
            "range": "±1.41%",
            "extra": "p95 29.78 ms | 35.54 ops/sec | 18 samples"
          }
        ],
        "release": {
          "ref": "0.18.6",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.5",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "c3a4dab99c8f53f9ed835a3b4252272d56b10b6f1d9de4fab08c61df404421de"
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
            "value": 0.460031,
            "unit": "ms",
            "range": "±1.52%",
            "extra": "p95 482.02 µs | 2,169.49 ops/sec | 2151 samples"
          },
          {
            "name": "card",
            "value": 0.213636,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 238.63 µs | 4,574.09 ops/sec | 4557 samples"
          },
          {
            "name": "consts",
            "value": 0.110944,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 195.06 µs | 8,258.96 ops/sec | 7921 samples"
          },
          {
            "name": "counter",
            "value": 0.721082,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 755.55 µs | 1,381.51 ops/sec | 1380 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.916396,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 947.28 µs | 1,087.19 ops/sec | 1087 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.372251,
            "unit": "ms",
            "range": "±1.57%",
            "extra": "p95 3.19 ms | 406.27 ops/sec | 400 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.451468,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 478.24 µs | 2,210.57 ops/sec | 2203 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.74776,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 777.13 µs | 1,335.15 ops/sec | 1334 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.697161,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 724.84 µs | 1,431.95 ops/sec | 1431 samples"
          },
          {
            "name": "page",
            "value": 0.269369,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 294.22 µs | 3,639.2 ops/sec | 3630 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.856815,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 1.93 ms | 536.31 ops/sec | 537 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.8176,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 1.89 ms | 547.9 ops/sec | 548 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.645025,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 669.86 µs | 1,550.11 ops/sec | 1549 samples"
          },
          {
            "name": "typography",
            "value": 0.985448,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 1.02 ms | 1,009.29 ops/sec | 1009 samples"
          },
          {
            "name": "use-memo",
            "value": 0.311781,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 337.18 µs | 3,153.6 ops/sec | 3147 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.283459,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 310.73 µs | 3,472.53 ops/sec | 3461 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.269629,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 294.22 µs | 3,651.44 ops/sec | 3641 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.16541,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 5.32 ms | 193.35 ops/sec | 194 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.2234,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 261.1 µs | 4,361.01 ops/sec | 4327 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.972254,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 3.03 ms | 335.59 ops/sec | 336 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 14.39906,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 14.68 ms | 69.35 ops/sec | 70 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2747.493939,
            "unit": "ms",
            "range": "±0.65%",
            "extra": "p95 2.79 s | 0.36 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 12.924929,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 13.45 ms | 77.13 ops/sec | 39 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.212545,
            "unit": "ms",
            "range": "±5.98%",
            "extra": "p95 1.26 ms | 819.24 ops/sec | 795 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.409926,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 1.47 ms | 706.6 ops/sec | 706 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.60453,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 632.09 µs | 1,652.17 ops/sec | 1650 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.768507,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 799.84 µs | 1,296.34 ops/sec | 1295 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.582812,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 616.95 µs | 1,711.73 ops/sec | 1709 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.704433,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 730.26 µs | 1,416.14 ops/sec | 1415 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.603113,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 1.67 ms | 620.19 ops/sec | 620 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.713737,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 750.2 µs | 1,394.5 ops/sec | 1392 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.429055,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 453.38 µs | 2,319.34 ops/sec | 2315 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.59482,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 618.65 µs | 1,681.99 ops/sec | 1680 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.302488,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 384.14 µs | 3,194.72 ops/sec | 3160 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.444508,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 468.62 µs | 2,239.92 ops/sec | 2236 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.420332,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 448.48 µs | 2,363.96 ops/sec | 2358 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.584876,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 613.59 µs | 1,704.74 ops/sec | 1700 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.473571,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 497.9 µs | 2,106.52 ops/sec | 2101 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.634044,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 658.81 µs | 1,575.82 ops/sec | 1574 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.289809,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 316.18 µs | 3,392.59 ops/sec | 3383 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.437843,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 462.15 µs | 2,272.02 ops/sec | 2268 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.189,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 214.82 µs | 5,163.76 ops/sec | 5143 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.768993,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 796.43 µs | 1,296.19 ops/sec | 1296 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.43974,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 462.9 µs | 2,264.84 ops/sec | 2261 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.632797,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 657.71 µs | 1,576.09 ops/sec | 1571 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.620724,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 642.55 µs | 1,609.32 ops/sec | 1608 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.42583,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 494.28 µs | 2,323.57 ops/sec | 2311 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.472339,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 496.57 µs | 2,114.59 ops/sec | 2111 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.520199,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 540.96 µs | 1,926.4 ops/sec | 1924 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.717582,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 741.08 µs | 1,391.7 ops/sec | 1391 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.717928,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 745.78 µs | 1,389.49 ops/sec | 1388 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.52731,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 550.1 µs | 1,899.69 ops/sec | 1897 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.76339,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 790.92 µs | 1,306.15 ops/sec | 1305 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.517135,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 542.24 µs | 1,935.59 ops/sec | 1933 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.535643,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 555.69 µs | 1,871.15 ops/sec | 1869 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.595711,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 619.88 µs | 1,678.42 ops/sec | 1676 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.303549,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 329.01 µs | 3,238.08 ops/sec | 3230 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.524937,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 546.38 µs | 1,907.44 ops/sec | 1905 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.516179,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 536.47 µs | 1,941.93 ops/sec | 1940 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.513665,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 533.18 µs | 1,952.41 ops/sec | 1950 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.555392,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 586.24 µs | 1,794.1 ops/sec | 1785 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.753185,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 785.12 µs | 1,322.57 ops/sec | 1318 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 5.330108,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 5.48 ms | 187.28 ops/sec | 188 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 5.941353,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 6.1 ms | 168.04 ops/sec | 169 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.60111,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 624.64 µs | 1,664.57 ops/sec | 1663 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "5b014e9f2862c167f97defb0ef8591422ecc1d8a6da9d45becd448278048ccff"
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
            "value": 0.463336,
            "unit": "ms",
            "range": "±1.47%",
            "extra": "p95 503.65 µs | 2,143.42 ops/sec | 2115 samples"
          },
          {
            "name": "card",
            "value": 0.214061,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 239.31 µs | 4,554.1 ops/sec | 4540 samples"
          },
          {
            "name": "consts",
            "value": 0.129422,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 156.22 µs | 7,493.2 ops/sec | 7443 samples"
          },
          {
            "name": "counter",
            "value": 0.73254,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 776.55 µs | 1,357.38 ops/sec | 1353 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.91407,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 947.3 µs | 1,089.23 ops/sec | 1089 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.246268,
            "unit": "ms",
            "range": "±1.09%",
            "extra": "p95 2.46 ms | 438.93 ops/sec | 436 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.46217,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 493.2 µs | 2,154.6 ops/sec | 2146 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.762897,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 790.01 µs | 1,308.39 ops/sec | 1307 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.707995,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 733.02 µs | 1,413.96 ops/sec | 1413 samples"
          },
          {
            "name": "page",
            "value": 0.262632,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 295.33 µs | 3,691.39 ops/sec | 3664 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.825284,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 1.9 ms | 545.25 ops/sec | 546 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.774304,
            "unit": "ms",
            "range": "±0.68%",
            "extra": "p95 1.93 ms | 556.33 ops/sec | 554 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.657019,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 693.95 µs | 1,519.66 ops/sec | 1517 samples"
          },
          {
            "name": "typography",
            "value": 0.974534,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 999.3 µs | 1,021.98 ops/sec | 1022 samples"
          },
          {
            "name": "use-memo",
            "value": 0.305661,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 350.21 µs | 3,161.13 ops/sec | 3137 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.286406,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 314.83 µs | 3,411.69 ops/sec | 3402 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.275075,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 305.01 µs | 3,548.51 ops/sec | 3536 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.962717,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 5.17 ms | 200.65 ops/sec | 201 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.228462,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 255.5 µs | 4,273.59 ops/sec | 4258 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.026974,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 3.18 ms | 327.85 ops/sec | 328 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 14.898037,
            "unit": "ms",
            "range": "±0.78%",
            "extra": "p95 15.96 ms | 66.56 ops/sec | 67 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2752.358798,
            "unit": "ms",
            "range": "±1.06%",
            "extra": "p95 2.79 s | 0.36 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 12.962193,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 13.54 ms | 77.03 ops/sec | 39 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.219511,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.28 ms | 814.84 ops/sec | 815 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.405018,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 1.46 ms | 708.74 ops/sec | 708 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.61783,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 649.4 µs | 1,616.72 ops/sec | 1614 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.773698,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 808.35 µs | 1,286.32 ops/sec | 1284 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.592518,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 619.74 µs | 1,689.09 ops/sec | 1685 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.70007,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 741.05 µs | 1,422.08 ops/sec | 1418 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.637444,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 1.71 ms | 607.83 ops/sec | 608 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.720442,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 754.69 µs | 1,383.75 ops/sec | 1382 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.43356,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 462.33 µs | 2,293.13 ops/sec | 2289 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.586766,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 611.92 µs | 1,707.51 ops/sec | 1705 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.300607,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 332.51 µs | 3,245.38 ops/sec | 3236 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.428281,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 459.31 µs | 2,315.89 ops/sec | 2309 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.424805,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 452.67 µs | 2,329.28 ops/sec | 2325 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.575477,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 598.82 µs | 1,742 ops/sec | 1740 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.482513,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 507.85 µs | 2,077.06 ops/sec | 2073 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.629383,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 652.88 µs | 1,590.96 ops/sec | 1590 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.296565,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 327.13 µs | 3,291.64 ops/sec | 3283 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.426137,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 454.81 µs | 2,324.61 ops/sec | 2321 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.194263,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 220.35 µs | 5,008.83 ops/sec | 4989 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.771363,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 804.45 µs | 1,290.27 ops/sec | 1289 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.420932,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 450.84 µs | 2,350.33 ops/sec | 2346 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.631812,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 657.9 µs | 1,582.31 ops/sec | 1581 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.628697,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 650.24 µs | 1,593.24 ops/sec | 1591 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.42795,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 456.13 µs | 2,313.05 ops/sec | 2309 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.482423,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 503.37 µs | 2,080.12 ops/sec | 2077 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.524341,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 543.4 µs | 1,919.7 ops/sec | 1917 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.708456,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 732.63 µs | 1,410.83 ops/sec | 1410 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.708506,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 735.78 µs | 1,410.25 ops/sec | 1409 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.531925,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 551.37 µs | 1,892.35 ops/sec | 1890 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.753905,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 779.35 µs | 1,323.71 ops/sec | 1322 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.523029,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 543.44 µs | 1,926.81 ops/sec | 1925 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.542084,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 565.26 µs | 1,853.43 ops/sec | 1851 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.605212,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 630.25 µs | 1,654.75 ops/sec | 1653 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.303057,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 344.03 µs | 3,203.29 ops/sec | 3187 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.528935,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 549.13 µs | 1,902.89 ops/sec | 1900 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.523219,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 547.42 µs | 1,916.41 ops/sec | 1911 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.519181,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 539.27 µs | 1,937.89 ops/sec | 1935 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.565879,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 596.83 µs | 1,765.32 ops/sec | 1755 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.755293,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 788.29 µs | 1,318.48 ops/sec | 1314 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 5.544332,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 5.75 ms | 179.48 ops/sec | 180 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 6.194604,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 6.37 ms | 161.03 ops/sec | 162 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.623225,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 651.74 µs | 1,603.31 ops/sec | 1601 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "88794e2e40d619d77ce18093cdba3e5ae3198cdeda258288bd82e2ed74320afd"
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
            "value": 0.464254,
            "unit": "ms",
            "range": "±1.35%",
            "extra": "p95 490.87 µs | 2,144.31 ops/sec | 2127 samples"
          },
          {
            "name": "card",
            "value": 0.214294,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 243.15 µs | 4,536.21 ops/sec | 4520 samples"
          },
          {
            "name": "consts",
            "value": 0.128572,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 154.68 µs | 7,539.1 ops/sec | 7493 samples"
          },
          {
            "name": "counter",
            "value": 0.734749,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 768.26 µs | 1,355.11 ops/sec | 1352 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.928739,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 960.85 µs | 1,071.42 ops/sec | 1071 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.287276,
            "unit": "ms",
            "range": "±1.04%",
            "extra": "p95 2.44 ms | 430.95 ops/sec | 428 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.464936,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 490.54 µs | 2,150.16 ops/sec | 2143 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.767858,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 795.76 µs | 1,299.92 ops/sec | 1299 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.71348,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 736.38 µs | 1,403.01 ops/sec | 1402 samples"
          },
          {
            "name": "page",
            "value": 0.262285,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 288.81 µs | 3,720.48 ops/sec | 3712 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.8503,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.91 ms | 537.92 ops/sec | 538 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.813246,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.88 ms | 549.17 ops/sec | 549 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.660334,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 688.03 µs | 1,515.62 ops/sec | 1514 samples"
          },
          {
            "name": "typography",
            "value": 0.984529,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 1.02 ms | 1,009.86 ops/sec | 1010 samples"
          },
          {
            "name": "use-memo",
            "value": 0.305185,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 334.1 µs | 3,200.94 ops/sec | 3195 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.290647,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 320.88 µs | 3,359.93 ops/sec | 3348 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.275539,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 304.16 µs | 3,542.3 ops/sec | 3532 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.025816,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 5.2 ms | 198.53 ops/sec | 199 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.230153,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 254.72 µs | 4,242.29 ops/sec | 4229 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.02095,
            "unit": "ms",
            "range": "±0.69%",
            "extra": "p95 3.14 ms | 329.27 ops/sec | 329 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 14.599873,
            "unit": "ms",
            "range": "±0.79%",
            "extra": "p95 15.39 ms | 68.02 ops/sec | 68 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2721.932846,
            "unit": "ms",
            "range": "±0.91%",
            "extra": "p95 2.78 s | 0.37 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 13.391298,
            "unit": "ms",
            "range": "±0.80%",
            "extra": "p95 13.94 ms | 74.53 ops/sec | 38 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.245195,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 1.3 ms | 799.34 ops/sec | 799 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.42305,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 1.48 ms | 700.01 ops/sec | 700 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.609648,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 631.16 µs | 1,646.35 ops/sec | 1645 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.766363,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 797.63 µs | 1,300.62 ops/sec | 1299 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.595852,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 621.26 µs | 1,682.62 ops/sec | 1681 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.70338,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 729.74 µs | 1,420.85 ops/sec | 1420 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.650879,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 1.7 ms | 603.39 ops/sec | 604 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.71147,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 736.82 µs | 1,404.78 ops/sec | 1404 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.432815,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 459.17 µs | 2,293.03 ops/sec | 2289 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.586315,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 606.58 µs | 1,714.47 ops/sec | 1713 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.303471,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 331.84 µs | 3,217.59 ops/sec | 3211 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.428041,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 457.94 µs | 2,313.46 ops/sec | 2308 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.426358,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 453.68 µs | 2,320.36 ops/sec | 2316 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.576121,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 592.76 µs | 1,748.86 ops/sec | 1748 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.481857,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 500.9 µs | 2,084.24 ops/sec | 2082 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.630607,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 651.37 µs | 1,590.77 ops/sec | 1589 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.29741,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 325.66 µs | 3,283.55 ops/sec | 3277 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.427875,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 453.41 µs | 2,317.58 ops/sec | 2315 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.19628,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 222.1 µs | 4,959.7 ops/sec | 4942 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.761074,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 792.14 µs | 1,310.18 ops/sec | 1309 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.421329,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 450.92 µs | 2,341.8 ops/sec | 2338 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.63183,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 655.43 µs | 1,585.02 ops/sec | 1580 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.630157,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 650.9 µs | 1,590.63 ops/sec | 1589 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.431622,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 458.87 µs | 2,292.19 ops/sec | 2288 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.478721,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 499.82 µs | 2,096.41 ops/sec | 2093 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.531311,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 551.8 µs | 1,895.38 ops/sec | 1893 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.710679,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 734.23 µs | 1,408.36 ops/sec | 1407 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.711641,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 734.84 µs | 1,404.84 ops/sec | 1403 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.538664,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 558.82 µs | 1,870.26 ops/sec | 1868 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.756576,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 788.68 µs | 1,319.77 ops/sec | 1319 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.529918,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 550.05 µs | 1,899.14 ops/sec | 1897 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.548994,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 568.36 µs | 1,833.36 ops/sec | 1831 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.600116,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 625.58 µs | 1,668.24 ops/sec | 1666 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.305656,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 334.78 µs | 3,191.76 ops/sec | 3185 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.53599,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 562.66 µs | 1,869.68 ops/sec | 1863 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.526803,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 547.57 µs | 1,906.96 ops/sec | 1903 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.523396,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 543.1 µs | 1,924.92 ops/sec | 1923 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.565275,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 592.06 µs | 1,770.46 ops/sec | 1760 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.751256,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 782.42 µs | 1,326.3 ops/sec | 1320 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 5.864593,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 6.09 ms | 170.2 ops/sec | 171 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 6.362054,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 6.57 ms | 156.83 ops/sec | 157 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.617633,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 640.64 µs | 1,624.94 ops/sec | 1623 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "65cd960eb3da5f7d490b931e8a4bfb343eb4d885a131424db51bddc716a07dbd"
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
            "value": 0.472361,
            "unit": "ms",
            "range": "±1.48%",
            "extra": "p95 500.16 µs | 2,110.93 ops/sec | 2090 samples"
          },
          {
            "name": "card",
            "value": 0.217498,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 246.42 µs | 4,473.15 ops/sec | 4452 samples"
          },
          {
            "name": "consts",
            "value": 0.131106,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 159.65 µs | 7,389.77 ops/sec | 7335 samples"
          },
          {
            "name": "counter",
            "value": 0.740689,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 778.85 µs | 1,346.36 ops/sec | 1345 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.936661,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 978.2 µs | 1,062.51 ops/sec | 1062 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.286487,
            "unit": "ms",
            "range": "±1.27%",
            "extra": "p95 2.53 ms | 429.28 ops/sec | 425 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.471143,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 501.32 µs | 2,124.33 ops/sec | 2117 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.788098,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 823.05 µs | 1,265.73 ops/sec | 1265 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.729468,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 775.8 µs | 1,367.55 ops/sec | 1366 samples"
          },
          {
            "name": "page",
            "value": 0.264797,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 293.53 µs | 3,685.95 ops/sec | 3676 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.851512,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.95 ms | 536.59 ops/sec | 537 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.820333,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 1.9 ms | 546.78 ops/sec | 547 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.657623,
            "unit": "ms",
            "range": "±0.84%",
            "extra": "p95 700.37 µs | 1,511.25 ops/sec | 1493 samples"
          },
          {
            "name": "typography",
            "value": 0.994795,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 1.04 ms | 998.4 ops/sec | 998 samples"
          },
          {
            "name": "use-memo",
            "value": 0.307357,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 339.19 µs | 3,178.44 ops/sec | 3170 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.29345,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 324.02 µs | 3,330.84 ops/sec | 3321 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.27697,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 306.2 µs | 3,525.16 ops/sec | 3513 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.054796,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 5.29 ms | 196.49 ops/sec | 197 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.232035,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 261.33 µs | 4,200.35 ops/sec | 4182 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.032004,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 3.19 ms | 327.7 ops/sec | 328 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 15.730689,
            "unit": "ms",
            "range": "±1.09%",
            "extra": "p95 16.9 ms | 63.14 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2774.088368,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 2.81 s | 0.36 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 13.550527,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 13.9 ms | 73.64 ops/sec | 37 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.25108,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 1.3 ms | 795.68 ops/sec | 796 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.440954,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 1.51 ms | 690.5 ops/sec | 690 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.621173,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 653.47 µs | 1,609.08 ops/sec | 1607 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.777536,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 799.79 µs | 1,284.97 ops/sec | 1284 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.598331,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 631.48 µs | 1,670.12 ops/sec | 1665 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.708088,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 740.08 µs | 1,410.61 ops/sec | 1409 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.649578,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 1.73 ms | 602.11 ops/sec | 602 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.716703,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 747.45 µs | 1,393.8 ops/sec | 1392 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.432481,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 462.3 µs | 2,291.03 ops/sec | 2287 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.594163,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 620.85 µs | 1,685.53 ops/sec | 1683 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.305087,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 340.3 µs | 3,198.91 ops/sec | 3190 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.430397,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 459.6 µs | 2,295.52 ops/sec | 2291 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.427912,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 458.65 µs | 2,309.78 ops/sec | 2301 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.578824,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 603.6 µs | 1,731.72 ops/sec | 1729 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.479698,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 504.6 µs | 2,083.13 ops/sec | 2079 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.634648,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 662.4 µs | 1,577.16 ops/sec | 1575 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.300453,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 332.47 µs | 3,250.29 ops/sec | 3241 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.436297,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 464.36 µs | 2,274.21 ops/sec | 2270 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.1977,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 226.71 µs | 4,920.29 ops/sec | 4899 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.782918,
            "unit": "ms",
            "range": "±1.24%",
            "extra": "p95 867.67 µs | 1,257.44 ops/sec | 1235 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.427221,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 461.82 µs | 2,319.13 ops/sec | 2312 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.631624,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 656.85 µs | 1,586.06 ops/sec | 1585 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.631465,
            "unit": "ms",
            "range": "±0.46%",
            "extra": "p95 660.91 µs | 1,583.4 ops/sec | 1578 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.429475,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 456.58 µs | 2,310.53 ops/sec | 2306 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.480111,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 507.83 µs | 2,083.6 ops/sec | 2077 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.521708,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 547.53 µs | 1,926.07 ops/sec | 1923 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.710244,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 734.69 µs | 1,408.75 ops/sec | 1408 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.709272,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 733.76 µs | 1,412.26 ops/sec | 1411 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.528082,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 558 µs | 1,899.27 ops/sec | 1896 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.756431,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 783.78 µs | 1,321.2 ops/sec | 1320 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.518995,
            "unit": "ms",
            "range": "±1.02%",
            "extra": "p95 562.33 µs | 1,916.34 ops/sec | 1881 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.539406,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 563.08 µs | 1,866.2 ops/sec | 1864 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.609649,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 634.08 µs | 1,646.01 ops/sec | 1644 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.306385,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 374.36 µs | 3,153.31 ops/sec | 3130 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.525667,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 553.99 µs | 1,906.62 ops/sec | 1898 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.519139,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 546.35 µs | 1,935.94 ops/sec | 1932 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.517346,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 546.8 µs | 1,935.46 ops/sec | 1932 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.566603,
            "unit": "ms",
            "range": "±0.70%",
            "extra": "p95 595.81 µs | 1,763.07 ops/sec | 1751 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.754545,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 788.86 µs | 1,320.26 ops/sec | 1314 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 5.694158,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 5.83 ms | 175.29 ops/sec | 176 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 6.259398,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 6.42 ms | 159.52 ops/sec | 160 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.623073,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 650.62 µs | 1,607.91 ops/sec | 1606 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-unknown-linux-gnu",
          "nativeSha256": "fdcea2a1deef637896d838d093abd47f1a1359954001710a416071a3a965e23b"
        }
      }
    ]
  }
};
