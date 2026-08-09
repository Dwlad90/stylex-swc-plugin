window.BENCHMARK_DATA = {
  "lastUpdate": 1786290037523,
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
        "date": 1785964844201,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "button-props",
            "value": 0.2098,
            "range": "±0.45%",
            "unit": "ms",
            "extra": "p95 276.9 µs | 4,548.52 ops/sec | 4480 samples"
          },
          {
            "name": "buttons-demo",
            "value": 0.7932,
            "range": "±0.64%",
            "unit": "ms",
            "extra": "p95 1.01 ms | 1,238.25 ops/sec | 1227 samples"
          },
          {
            "name": "card",
            "value": 0.3486,
            "range": "±0.38%",
            "unit": "ms",
            "extra": "p95 416.6 µs | 2,762.71 ops/sec | 2742 samples"
          },
          {
            "name": "consts",
            "value": 0.1601,
            "range": "±0.32%",
            "unit": "ms",
            "extra": "p95 215.2 µs | 5,995.76 ops/sec | 5930 samples"
          },
          {
            "name": "counter",
            "value": 1.4686,
            "range": "±0.63%",
            "unit": "ms",
            "extra": "p95 1.6 ms | 676.1 ops/sec | 673 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.92525,
            "range": "±3.13%",
            "unit": "ms",
            "extra": "p95 2.03 ms | 515.4 ops/sec | 508 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.5661,
            "range": "±3.25%",
            "unit": "ms",
            "extra": "p95 2.67 ms | 386.43 ops/sec | 381 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.6243,
            "range": "±0.99%",
            "unit": "ms",
            "extra": "p95 710.5 µs | 1,552.55 ops/sec | 1535 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.0871,
            "range": "±0.61%",
            "unit": "ms",
            "extra": "p95 1.18 ms | 914.58 ops/sec | 909 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.982,
            "range": "±0.50%",
            "unit": "ms",
            "extra": "p95 1.04 ms | 1,016.93 ops/sec | 1013 samples"
          },
          {
            "name": "page",
            "value": 0.4663,
            "range": "±0.33%",
            "unit": "ms",
            "extra": "p95 533.1 µs | 2,084.42 ops/sec | 2075 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.3262,
            "range": "±1.07%",
            "unit": "ms",
            "extra": "p95 3.66 ms | 296.22 ops/sec | 295 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.252,
            "range": "±0.44%",
            "unit": "ms",
            "extra": "p95 3.41 ms | 306.15 ops/sec | 306 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.3774,
            "range": "±0.39%",
            "unit": "ms",
            "extra": "p95 1.44 ms | 725.65 ops/sec | 725 samples"
          },
          {
            "name": "typography",
            "value": 1.9233,
            "range": "±2.24%",
            "unit": "ms",
            "extra": "p95 2.13 ms | 511.05 ops/sec | 503 samples"
          },
          {
            "name": "use-memo",
            "value": 0.50475,
            "range": "±2.54%",
            "unit": "ms",
            "extra": "p95 572.4 µs | 1,925.07 ops/sec | 1888 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.4294,
            "range": "±0.37%",
            "unit": "ms",
            "extra": "p95 491.6 µs | 2,261.98 ops/sec | 2249 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.4036,
            "range": "±0.45%",
            "unit": "ms",
            "extra": "p95 485.5 µs | 2,385.55 ops/sec | 2364 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.7431,
            "range": "±0.54%",
            "unit": "ms",
            "extra": "p95 10.13 ms | 102.35 ops/sec | 103 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.4578,
            "range": "±0.40%",
            "unit": "ms",
            "extra": "p95 528.8 µs | 2,113.11 ops/sec | 2099 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.7916,
            "range": "±0.55%",
            "unit": "ms",
            "extra": "p95 6.01 ms | 171.53 ops/sec | 172 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5101.44315,
            "range": "±0.78%",
            "unit": "ms",
            "extra": "p95 5,156.71 ms | 0.2 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 24.0876,
            "range": "±1.18%",
            "unit": "ms",
            "extra": "p95 25.02 ms | 41.36 ops/sec | 21 samples"
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
            "value": 0.7926,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 873.4 µs | 1,250.49 ops/sec | 1246 samples"
          },
          {
            "name": "card",
            "value": 0.3459,
            "unit": "ms",
            "range": "±2.62%",
            "extra": "p95 425.3 µs | 2,783.91 ops/sec | 2706 samples"
          },
          {
            "name": "consts",
            "value": 0.133,
            "unit": "ms",
            "range": "±2.59%",
            "extra": "p95 196.4 µs | 7,127.94 ops/sec | 6807 samples"
          },
          {
            "name": "counter",
            "value": 1.4442,
            "unit": "ms",
            "range": "±1.09%",
            "extra": "p95 1.57 ms | 684.32 ops/sec | 677 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.9093,
            "unit": "ms",
            "range": "±1.08%",
            "extra": "p95 2.39 ms | 510.83 ops/sec | 506 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.56,
            "unit": "ms",
            "range": "±0.68%",
            "extra": "p95 2.75 ms | 386.16 ops/sec | 385 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.6084,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 688.9 µs | 1,599.99 ops/sec | 1592 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.0473,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 1.1 ms | 954.67 ops/sec | 952 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.9641,
            "unit": "ms",
            "range": "±0.68%",
            "extra": "p95 1.12 ms | 1,024.04 ops/sec | 1016 samples"
          },
          {
            "name": "page",
            "value": 0.4578,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 532 µs | 2,114.97 ops/sec | 2102 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.3455,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 3.48 ms | 296.67 ops/sec | 297 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.23755,
            "unit": "ms",
            "range": "±1.84%",
            "extra": "p95 3.73 ms | 301.91 ops/sec | 298 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.3549,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 1.5 ms | 731.98 ops/sec | 729 samples"
          },
          {
            "name": "typography",
            "value": 1.9091,
            "unit": "ms",
            "range": "±1.16%",
            "extra": "p95 2.78 ms | 509.08 ops/sec | 503 samples"
          },
          {
            "name": "use-memo",
            "value": 0.4919,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 595.5 µs | 1,957.09 ops/sec | 1943 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.4055,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 465.5 µs | 2,407.33 ops/sec | 2397 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.3827,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 450.9 µs | 2,534.66 ops/sec | 2516 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.8103,
            "unit": "ms",
            "range": "±1.15%",
            "extra": "p95 10.45 ms | 100.74 ops/sec | 101 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.4502,
            "unit": "ms",
            "range": "±0.47%",
            "extra": "p95 522.8 µs | 2,148.2 ops/sec | 2129 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.91595,
            "unit": "ms",
            "range": "±1.27%",
            "extra": "p95 6.34 ms | 166.43 ops/sec | 166 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5448.6241,
            "unit": "ms",
            "range": "±1.10%",
            "extra": "p95 5.58 s | 0.18 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 26.4521,
            "unit": "ms",
            "range": "±2.38%",
            "extra": "p95 31.03 ms | 37.43 ops/sec | 19 samples"
          }
        ],
        "release": {
          "ref": "fix_release-benchmarks",
          "candidateVersion": "0.18.3",
          "previousVersion": "0.18.3",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "2bd5561e8d9ecdd23991ac8ed0c3da86a926360d5c35cf1627bc1409868381a1"
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
            "value": 0.8199,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 916.9 µs | 1,211.12 ops/sec | 1204 samples"
          },
          {
            "name": "card",
            "value": 0.3549,
            "unit": "ms",
            "range": "±2.95%",
            "extra": "p95 440.8 µs | 2,703.69 ops/sec | 2620 samples"
          },
          {
            "name": "consts",
            "value": 0.1606,
            "unit": "ms",
            "range": "±0.77%",
            "extra": "p95 216.7 µs | 5,971.26 ops/sec | 5861 samples"
          },
          {
            "name": "counter",
            "value": 1.477,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 1.59 ms | 672.43 ops/sec | 671 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.95295,
            "unit": "ms",
            "range": "±0.77%",
            "extra": "p95 2.12 ms | 503.87 ops/sec | 502 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.5317,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 2.69 ms | 391.58 ops/sec | 391 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.6271,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 730.7 µs | 1,541.07 ops/sec | 1529 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.1042,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 1.23 ms | 899.8 ops/sec | 895 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.0031,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 1.13 ms | 987.37 ops/sec | 980 samples"
          },
          {
            "name": "page",
            "value": 0.4691,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 569.2 µs | 2,049.72 ops/sec | 2034 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.3858,
            "unit": "ms",
            "range": "±0.94%",
            "extra": "p95 3.74 ms | 291.61 ops/sec | 291 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.2816,
            "unit": "ms",
            "range": "±0.98%",
            "extra": "p95 3.6 ms | 300.14 ops/sec | 299 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.3883,
            "unit": "ms",
            "range": "±0.70%",
            "extra": "p95 1.5 ms | 714.83 ops/sec | 712 samples"
          },
          {
            "name": "typography",
            "value": 1.9548,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 2.11 ms | 505.46 ops/sec | 504 samples"
          },
          {
            "name": "use-memo",
            "value": 0.5114,
            "unit": "ms",
            "range": "±0.47%",
            "extra": "p95 618.2 µs | 1,882.92 ops/sec | 1868 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.4266,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 499.9 µs | 2,266.33 ops/sec | 2249 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.4108,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 480.4 µs | 2,355.79 ops/sec | 2341 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.8889,
            "unit": "ms",
            "range": "±2.19%",
            "extra": "p95 12.89 ms | 99.48 ops/sec | 99 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.4588,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 577.4 µs | 2,095.1 ops/sec | 2071 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.9466,
            "unit": "ms",
            "range": "±1.24%",
            "extra": "p95 6.28 ms | 166.52 ops/sec | 167 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5294.1802,
            "unit": "ms",
            "range": "±2.83%",
            "extra": "p95 5.94 s | 0.19 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 25.72155,
            "unit": "ms",
            "range": "±1.56%",
            "extra": "p95 26.91 ms | 38.88 ops/sec | 20 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "ea8e479e1626abe7192d7bf00ec50726bc3ac7c9c4054739c96ffc4914a2253d"
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
            "value": 0.7889,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 890.1 µs | 1,252.93 ops/sec | 1246 samples"
          },
          {
            "name": "card",
            "value": 0.34965,
            "unit": "ms",
            "range": "±3.35%",
            "extra": "p95 414.8 µs | 2,763.64 ops/sec | 2686 samples"
          },
          {
            "name": "consts",
            "value": 0.135,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 198.6 µs | 7,047.69 ops/sec | 6888 samples"
          },
          {
            "name": "counter",
            "value": 1.4528,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 1.51 ms | 688.53 ops/sec | 688 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.8899,
            "unit": "ms",
            "range": "±0.96%",
            "extra": "p95 2.19 ms | 518.93 ops/sec | 515 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.57155,
            "unit": "ms",
            "range": "±1.02%",
            "extra": "p95 2.79 ms | 382.55 ops/sec | 380 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.62075,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 686.7 µs | 1,578.62 ops/sec | 1574 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.0534,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 1.1 ms | 954.08 ops/sec | 951 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.9711,
            "unit": "ms",
            "range": "±2.82%",
            "extra": "p95 1.06 ms | 1,025.3 ops/sec | 1003 samples"
          },
          {
            "name": "page",
            "value": 0.4564,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 544.3 µs | 2,112.6 ops/sec | 2097 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.3224,
            "unit": "ms",
            "range": "±0.73%",
            "extra": "p95 3.44 ms | 298.73 ops/sec | 298 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.23385,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 3.39 ms | 307.4 ops/sec | 308 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.35645,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 1.43 ms | 735.86 ops/sec | 734 samples"
          },
          {
            "name": "typography",
            "value": 1.91705,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 2.04 ms | 517.13 ops/sec | 516 samples"
          },
          {
            "name": "use-memo",
            "value": 0.4957,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 564 µs | 1,956.16 ops/sec | 1945 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.4108,
            "unit": "ms",
            "range": "±0.55%",
            "extra": "p95 480.4 µs | 2,364.47 ops/sec | 2339 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.3846,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 448.1 µs | 2,528.51 ops/sec | 2514 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 10.0148,
            "unit": "ms",
            "range": "±2.43%",
            "extra": "p95 12.91 ms | 97.49 ops/sec | 97 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.4508,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 525.3 µs | 2,148.57 ops/sec | 2132 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.8743,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 6.11 ms | 168.98 ops/sec | 169 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5274.6659,
            "unit": "ms",
            "range": "±1.38%",
            "extra": "p95 5.56 s | 0.19 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.21515,
            "unit": "ms",
            "range": "±0.85%",
            "extra": "p95 24.16 ms | 42.85 ops/sec | 22 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "688d18c8d6dee88c58751f699f900b987ef5a5788af24fc4b2ec92a068173a61"
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
            "value": 0.7652,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 937.2 µs | 1,298.24 ops/sec | 1285 samples"
          },
          {
            "name": "card",
            "value": 0.247,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 357.5 µs | 3,876.51 ops/sec | 3779 samples"
          },
          {
            "name": "consts",
            "value": 0.0986,
            "unit": "ms",
            "range": "±2.53%",
            "extra": "p95 133.3 µs | 9,988.74 ops/sec | 9616 samples"
          },
          {
            "name": "counter",
            "value": 1.4142,
            "unit": "ms",
            "range": "±0.89%",
            "extra": "p95 1.54 ms | 707.29 ops/sec | 702 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.80415,
            "unit": "ms",
            "range": "±0.65%",
            "extra": "p95 1.96 ms | 560.6 ops/sec | 558 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.318,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 2.49 ms | 432.14 ops/sec | 431 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.50225,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 592 µs | 1,980.69 ops/sec | 1960 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.94125,
            "unit": "ms",
            "range": "±0.96%",
            "extra": "p95 1.23 ms | 1,042.25 ops/sec | 1026 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.8659,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 968.5 µs | 1,152.5 ops/sec | 1147 samples"
          },
          {
            "name": "page",
            "value": 0.3967,
            "unit": "ms",
            "range": "±2.55%",
            "extra": "p95 513.9 µs | 2,477.57 ops/sec | 2393 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.1486,
            "unit": "ms",
            "range": "±0.85%",
            "extra": "p95 3.34 ms | 320.38 ops/sec | 319 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.0624,
            "unit": "ms",
            "range": "±0.92%",
            "extra": "p95 3.33 ms | 329.07 ops/sec | 328 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.2879,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 1.4 ms | 781.15 ops/sec | 777 samples"
          },
          {
            "name": "typography",
            "value": 1.8311,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 1.99 ms | 552.6 ops/sec | 551 samples"
          },
          {
            "name": "use-memo",
            "value": 0.4583,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 553 µs | 2,156.57 ops/sec | 2132 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.3382,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 411.5 µs | 2,902.65 ops/sec | 2866 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.3139,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 370.7 µs | 3,186.43 ops/sec | 3152 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 8.91695,
            "unit": "ms",
            "range": "±1.17%",
            "extra": "p95 9.38 ms | 112 ops/sec | 112 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.3463,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 434.4 µs | 2,837.84 ops/sec | 2795 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.22945,
            "unit": "ms",
            "range": "±1.73%",
            "extra": "p95 6.72 ms | 189.91 ops/sec | 188 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4491.66805,
            "unit": "ms",
            "range": "±1.80%",
            "extra": "p95 4.59 s | 0.22 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 19.514,
            "unit": "ms",
            "range": "±2.55%",
            "extra": "p95 22.43 ms | 50.7 ops/sec | 26 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "9136c2a531fcea562329abae09df74ea73ea8d3d0624f538af7238bcd0afe5b3"
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
            "value": 0.5962,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 654.4 µs | 1,642.94 ops/sec | 1636 samples"
          },
          {
            "name": "card",
            "value": 0.2638,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 314.7 µs | 3,670.14 ops/sec | 3645 samples"
          },
          {
            "name": "consts",
            "value": 0.1019,
            "unit": "ms",
            "range": "±2.01%",
            "extra": "p95 146.8 µs | 9,458.2 ops/sec | 9168 samples"
          },
          {
            "name": "counter",
            "value": 1.12515,
            "unit": "ms",
            "range": "±0.65%",
            "extra": "p95 1.19 ms | 885.68 ops/sec | 882 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.4963,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 1.55 ms | 666.32 ops/sec | 665 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.9891,
            "unit": "ms",
            "range": "±1.06%",
            "extra": "p95 2.43 ms | 491.49 ops/sec | 487 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.4657,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 520.3 µs | 2,100.51 ops/sec | 2092 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.8107,
            "unit": "ms",
            "range": "±0.79%",
            "extra": "p95 1.23 ms | 1,204.6 ops/sec | 1189 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.7548,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 900.7 µs | 1,299.06 ops/sec | 1289 samples"
          },
          {
            "name": "page",
            "value": 0.3581,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 433 µs | 2,687.26 ops/sec | 2662 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.6215,
            "unit": "ms",
            "range": "±0.74%",
            "extra": "p95 2.95 ms | 375.8 ops/sec | 375 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.5507,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 2.69 ms | 388.17 ops/sec | 387 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.0523,
            "unit": "ms",
            "range": "±1.03%",
            "extra": "p95 1.65 ms | 925.56 ops/sec | 910 samples"
          },
          {
            "name": "typography",
            "value": 1.4806,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 1.55 ms | 671.45 ops/sec | 670 samples"
          },
          {
            "name": "use-memo",
            "value": 0.3834,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 448.9 µs | 2,521.97 ops/sec | 2503 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.3133,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 371.7 µs | 3,085.19 ops/sec | 3062 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.2967,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 362.1 µs | 3,241.19 ops/sec | 3210 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 7.7216,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 8.28 ms | 128.14 ops/sec | 128 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.3505,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 470.8 µs | 2,731.73 ops/sec | 2695 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 4.59995,
            "unit": "ms",
            "range": "±0.89%",
            "extra": "p95 5.1 ms | 214.35 ops/sec | 214 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 4348.41255,
            "unit": "ms",
            "range": "±0.69%",
            "extra": "p95 4.45 s | 0.23 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 18.1765,
            "unit": "ms",
            "range": "±3.55%",
            "extra": "p95 23.86 ms | 52.95 ops/sec | 27 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "9136c2a531fcea562329abae09df74ea73ea8d3d0624f538af7238bcd0afe5b3"
        }
      }
    ]
  }
};
