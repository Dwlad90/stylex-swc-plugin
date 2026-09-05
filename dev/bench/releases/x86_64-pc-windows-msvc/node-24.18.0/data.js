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
            "value": 0.8023,
            "unit": "ms",
            "range": "±0.46%",
            "extra": "p95 893.7 µs | 1,240.71 ops/sec | 1235 samples"
          },
          {
            "name": "card",
            "value": 0.3495,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 443.5 µs | 2,754.1 ops/sec | 2727 samples"
          },
          {
            "name": "consts",
            "value": 0.1353,
            "unit": "ms",
            "range": "±0.65%",
            "extra": "p95 193.8 µs | 7,011.07 ops/sec | 6871 samples"
          },
          {
            "name": "counter",
            "value": 1.44925,
            "unit": "ms",
            "range": "±0.71%",
            "extra": "p95 1.69 ms | 679.9 ops/sec | 676 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.9308,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 2.04 ms | 515.57 ops/sec | 516 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.58325,
            "unit": "ms",
            "range": "±0.76%",
            "extra": "p95 2.74 ms | 383.29 ops/sec | 382 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.6204,
            "unit": "ms",
            "range": "±0.47%",
            "extra": "p95 709 µs | 1,578.02 ops/sec | 1568 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.06395,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 1.16 ms | 933.99 ops/sec | 930 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.9712,
            "unit": "ms",
            "range": "±2.64%",
            "extra": "p95 1.14 ms | 1,021.03 ops/sec | 999 samples"
          },
          {
            "name": "page",
            "value": 0.45325,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 521 µs | 2,143.46 ops/sec | 2132 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.3355,
            "unit": "ms",
            "range": "±0.73%",
            "extra": "p95 3.64 ms | 296.55 ops/sec | 296 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.23735,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 3.37 ms | 307.41 ops/sec | 308 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.3653,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 1.59 ms | 720.81 ops/sec | 717 samples"
          },
          {
            "name": "typography",
            "value": 1.91485,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 2.06 ms | 515.63 ops/sec | 514 samples"
          },
          {
            "name": "use-memo",
            "value": 0.50405,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 583 µs | 1,921.36 ops/sec | 1906 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.4182,
            "unit": "ms",
            "range": "±2.35%",
            "extra": "p95 500 µs | 2,318.46 ops/sec | 2263 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.3899,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 478.9 µs | 2,477.78 ops/sec | 2447 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.9043,
            "unit": "ms",
            "range": "±1.70%",
            "extra": "p95 12.52 ms | 98.24 ops/sec | 98 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.4572,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 539.2 µs | 2,116.63 ops/sec | 2101 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.9678,
            "unit": "ms",
            "range": "±1.31%",
            "extra": "p95 6.52 ms | 164.6 ops/sec | 164 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5346.50315,
            "unit": "ms",
            "range": "±3.96%",
            "extra": "p95 6.28 s | 0.18 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.7635,
            "unit": "ms",
            "range": "±3.67%",
            "extra": "p95 29.78 ms | 41.07 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-rc.1",
          "candidateVersion": "0.18.4-rc.1",
          "previousVersion": "0.18.3",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "dd7d6464c4a0089585a62417b6b2d275b15b99dd9918d268334538d450a3825f"
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
            "value": 0.7041,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 962 µs | 1,387.91 ops/sec | 1375 samples"
          },
          {
            "name": "card",
            "value": 0.2913,
            "unit": "ms",
            "range": "±1.12%",
            "extra": "p95 428.8 µs | 3,240.4 ops/sec | 3159 samples"
          },
          {
            "name": "consts",
            "value": 0.1353,
            "unit": "ms",
            "range": "±1.19%",
            "extra": "p95 220 µs | 6,840.44 ops/sec | 6553 samples"
          },
          {
            "name": "counter",
            "value": 1.2099,
            "unit": "ms",
            "range": "±1.25%",
            "extra": "p95 1.83 ms | 788.9 ops/sec | 771 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.6815,
            "unit": "ms",
            "range": "±1.15%",
            "extra": "p95 2.51 ms | 575.38 ops/sec | 568 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.6805,
            "unit": "ms",
            "range": "±1.47%",
            "extra": "p95 4.22 ms | 362.56 ops/sec | 358 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.6268,
            "unit": "ms",
            "range": "±0.87%",
            "extra": "p95 984.1 µs | 1,513.19 ops/sec | 1481 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.3157,
            "unit": "ms",
            "range": "±1.38%",
            "extra": "p95 2.04 ms | 722.68 ops/sec | 705 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.20885,
            "unit": "ms",
            "range": "±1.14%",
            "extra": "p95 1.85 ms | 800.7 ops/sec | 786 samples"
          },
          {
            "name": "page",
            "value": 0.3578,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 504.1 µs | 2,673.93 ops/sec | 2642 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.28025,
            "unit": "ms",
            "range": "±1.45%",
            "extra": "p95 4.98 ms | 296.43 ops/sec | 294 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.19465,
            "unit": "ms",
            "range": "±1.65%",
            "extra": "p95 4.91 ms | 302.14 ops/sec | 298 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.1273,
            "unit": "ms",
            "range": "±1.29%",
            "extra": "p95 1.71 ms | 842.78 ops/sec | 822 samples"
          },
          {
            "name": "typography",
            "value": 1.7039,
            "unit": "ms",
            "range": "±1.40%",
            "extra": "p95 2.61 ms | 560.57 ops/sec | 549 samples"
          },
          {
            "name": "use-memo",
            "value": 0.4294,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 622.5 µs | 2,221.55 ops/sec | 2191 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.4126,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 643.9 µs | 2,328.59 ops/sec | 2289 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.3876,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 626.9 µs | 2,450.73 ops/sec | 2396 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 10.335,
            "unit": "ms",
            "range": "±3.35%",
            "extra": "p95 15.93 ms | 92.62 ops/sec | 91 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.3386,
            "unit": "ms",
            "range": "±0.55%",
            "extra": "p95 509.8 µs | 2,823.07 ops/sec | 2780 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.6202,
            "unit": "ms",
            "range": "±2.03%",
            "extra": "p95 8.26 ms | 172.6 ops/sec | 171 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5146.8565,
            "unit": "ms",
            "range": "±1.39%",
            "extra": "p95 5.38 s | 0.19 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.7605,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 24.41 ms | 42.01 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-rc.2",
          "candidateVersion": "0.18.4-rc.2",
          "previousVersion": "0.18.3",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "0ff8955b1648d6353be4fa08dda6ede87c0ef4b02f2233cfbd289f4d91c054c1"
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
            "value": 0.7833,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 862.5 µs | 1,257.36 ops/sec | 1250 samples"
          },
          {
            "name": "card",
            "value": 0.3446,
            "unit": "ms",
            "range": "±2.22%",
            "extra": "p95 407.9 µs | 2,812.04 ops/sec | 2754 samples"
          },
          {
            "name": "consts",
            "value": 0.1356,
            "unit": "ms",
            "range": "±2.28%",
            "extra": "p95 199.8 µs | 7,011.53 ops/sec | 6764 samples"
          },
          {
            "name": "counter",
            "value": 1.4636,
            "unit": "ms",
            "range": "±0.82%",
            "extra": "p95 1.8 ms | 672.27 ops/sec | 667 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.923,
            "unit": "ms",
            "range": "±0.81%",
            "extra": "p95 2.19 ms | 511.89 ops/sec | 509 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.55915,
            "unit": "ms",
            "range": "±0.71%",
            "extra": "p95 2.7 ms | 386.66 ops/sec | 386 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.6069,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 677.5 µs | 1,611.57 ops/sec | 1602 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.04675,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 1.14 ms | 944.34 ops/sec | 938 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.96285,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 1.03 ms | 1,027.87 ops/sec | 1024 samples"
          },
          {
            "name": "page",
            "value": 0.4544,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 519.1 µs | 2,133.32 ops/sec | 2122 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.3657,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 3.52 ms | 294.91 ops/sec | 295 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.275,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 3.36 ms | 304.97 ops/sec | 305 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.3655,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 1.41 ms | 735.56 ops/sec | 735 samples"
          },
          {
            "name": "typography",
            "value": 1.92595,
            "unit": "ms",
            "range": "±1.23%",
            "extra": "p95 2.95 ms | 502.53 ops/sec | 496 samples"
          },
          {
            "name": "use-memo",
            "value": 0.5043,
            "unit": "ms",
            "range": "±0.80%",
            "extra": "p95 772.4 µs | 1,857.3 ops/sec | 1814 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.4116,
            "unit": "ms",
            "range": "±0.69%",
            "extra": "p95 666.8 µs | 2,323.55 ops/sec | 2278 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.38765,
            "unit": "ms",
            "range": "±0.83%",
            "extra": "p95 647.4 µs | 2,423.46 ops/sec | 2352 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.9989,
            "unit": "ms",
            "range": "±3.74%",
            "extra": "p95 16.13 ms | 95.22 ops/sec | 93 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.452,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 537 µs | 2,136.38 ops/sec | 2117 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.0266,
            "unit": "ms",
            "range": "±1.15%",
            "extra": "p95 6.28 ms | 164.47 ops/sec | 164 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5309.3645,
            "unit": "ms",
            "range": "±1.94%",
            "extra": "p95 5.67 s | 0.19 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.3257,
            "unit": "ms",
            "range": "±1.01%",
            "extra": "p95 24.46 ms | 42.7 ops/sec | 22 samples"
          }
        ],
        "release": {
          "ref": "0.18.4",
          "candidateVersion": "0.18.4",
          "previousVersion": "0.18.3",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "8e5015d50978654ca4f34c2c6a9f01551125e492f581a9b26f3fb2b660250f8a"
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
            "value": 0.71755,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 811.1 µs | 1,391.56 ops/sec | 1384 samples"
          },
          {
            "name": "card",
            "value": 0.2205,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 323.3 µs | 4,323.74 ops/sec | 4208 samples"
          },
          {
            "name": "consts",
            "value": 0.088,
            "unit": "ms",
            "range": "±2.22%",
            "extra": "p95 121.1 µs | 10,982.81 ops/sec | 10654 samples"
          },
          {
            "name": "counter",
            "value": 1.3008,
            "unit": "ms",
            "range": "±2.35%",
            "extra": "p95 1.45 ms | 760.57 ops/sec | 747 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.7225,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 1.86 ms | 577.45 ops/sec | 576 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.1417,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 2.29 ms | 461.33 ops/sec | 460 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.4933,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 573.1 µs | 1,996.48 ops/sec | 1978 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.8875,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 972.4 µs | 1,121.47 ops/sec | 1118 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.8217,
            "unit": "ms",
            "range": "±0.47%",
            "extra": "p95 893.4 µs | 1,210.8 ops/sec | 1205 samples"
          },
          {
            "name": "page",
            "value": 0.3593,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 467.2 µs | 2,683.45 ops/sec | 2646 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.869,
            "unit": "ms",
            "range": "±0.77%",
            "extra": "p95 3.22 ms | 344.06 ops/sec | 343 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.8,
            "unit": "ms",
            "range": "±0.80%",
            "extra": "p95 2.91 ms | 353.48 ops/sec | 353 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.18175,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 1.24 ms | 845.79 ops/sec | 844 samples"
          },
          {
            "name": "typography",
            "value": 1.6691,
            "unit": "ms",
            "range": "±0.47%",
            "extra": "p95 1.77 ms | 595.74 ops/sec | 595 samples"
          },
          {
            "name": "use-memo",
            "value": 0.4014,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 487.8 µs | 2,430.71 ops/sec | 2407 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.30165,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 375.4 µs | 3,193.94 ops/sec | 3156 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.2821,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 336.9 µs | 3,425.28 ops/sec | 3398 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 8.2197,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 8.7 ms | 120.74 ops/sec | 121 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.3167,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 390 µs | 3,045.09 ops/sec | 3011 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 4.8922,
            "unit": "ms",
            "range": "±1.10%",
            "extra": "p95 5.31 ms | 201.57 ops/sec | 201 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3921.77945,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 3.98 s | 0.25 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 18.9169,
            "unit": "ms",
            "range": "±1.75%",
            "extra": "p95 19.82 ms | 52.37 ops/sec | 27 samples"
          }
        ],
        "release": {
          "ref": "0.18.5-rc.1",
          "candidateVersion": "0.18.5-rc.1",
          "previousVersion": "0.18.4",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "1822b802e6095fdde5f81349b703e58a1ef4fc22f3b37552a2513e07a49eb1f2"
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
            "value": 0.8273,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 943.1 µs | 1,195.57 ops/sec | 1187 samples"
          },
          {
            "name": "card",
            "value": 0.3571,
            "unit": "ms",
            "range": "±2.39%",
            "extra": "p95 439.4 µs | 2,696.29 ops/sec | 2629 samples"
          },
          {
            "name": "consts",
            "value": 0.1645,
            "unit": "ms",
            "range": "±2.44%",
            "extra": "p95 225.4 µs | 5,810.26 ops/sec | 5616 samples"
          },
          {
            "name": "counter",
            "value": 1.4858,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 1.63 ms | 665.98 ops/sec | 663 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.9444,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 2.08 ms | 508.17 ops/sec | 507 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.5158,
            "unit": "ms",
            "range": "±0.78%",
            "extra": "p95 2.67 ms | 392.85 ops/sec | 392 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.6142,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 705.7 µs | 1,576.54 ops/sec | 1563 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.0741,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 1.16 ms | 927.91 ops/sec | 925 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.9938,
            "unit": "ms",
            "range": "±0.89%",
            "extra": "p95 1.42 ms | 988.87 ops/sec | 976 samples"
          },
          {
            "name": "page",
            "value": 0.4626,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 533.8 µs | 2,094.57 ops/sec | 2082 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.34975,
            "unit": "ms",
            "range": "±0.98%",
            "extra": "p95 3.67 ms | 294.65 ops/sec | 294 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.32155,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 3.46 ms | 299.77 ops/sec | 300 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.38355,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 1.5 ms | 715.53 ops/sec | 712 samples"
          },
          {
            "name": "typography",
            "value": 1.9595,
            "unit": "ms",
            "range": "±0.76%",
            "extra": "p95 2.11 ms | 502.64 ops/sec | 501 samples"
          },
          {
            "name": "use-memo",
            "value": 0.50655,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 590.1 µs | 1,917.25 ops/sec | 1906 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.4266,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 502.7 µs | 2,268.75 ops/sec | 2250 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.4005,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 467.9 µs | 2,424.96 ops/sec | 2411 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 10.1095,
            "unit": "ms",
            "range": "±1.26%",
            "extra": "p95 10.9 ms | 98.11 ops/sec | 98 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.4736,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 550.2 µs | 2,041.95 ops/sec | 2028 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 6.2385,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 6.62 ms | 159.35 ops/sec | 160 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5165.38885,
            "unit": "ms",
            "range": "±2.19%",
            "extra": "p95 5.61 s | 0.19 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 24.4267,
            "unit": "ms",
            "range": "±2.43%",
            "extra": "p95 26.47 ms | 40.92 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "0.18.5",
          "candidateVersion": "0.18.5",
          "previousVersion": "0.18.4",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "814591fae4e515a4207f4d7925a2059a81a72e3c1d67ae99c0fe4a39a22ae01f"
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
            "value": 0.8211,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 892.8 µs | 1,210.43 ops/sec | 1208 samples"
          },
          {
            "name": "card",
            "value": 0.3493,
            "unit": "ms",
            "range": "±2.34%",
            "extra": "p95 457.9 µs | 2,745.74 ops/sec | 2672 samples"
          },
          {
            "name": "consts",
            "value": 0.1626,
            "unit": "ms",
            "range": "±2.46%",
            "extra": "p95 243.7 µs | 5,730.78 ops/sec | 5421 samples"
          },
          {
            "name": "counter",
            "value": 1.4971,
            "unit": "ms",
            "range": "±1.62%",
            "extra": "p95 2.32 ms | 644.85 ops/sec | 629 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.9914,
            "unit": "ms",
            "range": "±2.76%",
            "extra": "p95 3.81 ms | 445.36 ops/sec | 419 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.5407,
            "unit": "ms",
            "range": "±1.40%",
            "extra": "p95 3.27 ms | 383.25 ops/sec | 379 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.65495,
            "unit": "ms",
            "range": "±2.45%",
            "extra": "p95 1.64 ms | 1,420.37 ops/sec | 1294 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.09545,
            "unit": "ms",
            "range": "±1.35%",
            "extra": "p95 1.28 ms | 900.51 ops/sec | 886 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 1.00515,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 1.11 ms | 992.75 ops/sec | 986 samples"
          },
          {
            "name": "page",
            "value": 0.4635,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 613.8 µs | 2,078.17 ops/sec | 2057 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.3288,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 3.49 ms | 298.72 ops/sec | 299 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.23805,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 3.38 ms | 306.49 ops/sec | 306 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.3485,
            "unit": "ms",
            "range": "±0.88%",
            "extra": "p95 1.49 ms | 733.35 ops/sec | 727 samples"
          },
          {
            "name": "typography",
            "value": 1.9036,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 1.99 ms | 521.37 ops/sec | 520 samples"
          },
          {
            "name": "use-memo",
            "value": 0.4989,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 566.8 µs | 1,949.63 ops/sec | 1941 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.4245,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 489.6 µs | 2,289.87 ops/sec | 2277 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.40135,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 462.6 µs | 2,424.63 ops/sec | 2414 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.7625,
            "unit": "ms",
            "range": "±1.11%",
            "extra": "p95 10.14 ms | 101.88 ops/sec | 102 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.4639,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 537.6 µs | 2,096.39 ops/sec | 2082 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.9209,
            "unit": "ms",
            "range": "±1.10%",
            "extra": "p95 6.22 ms | 167.15 ops/sec | 167 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5277.4403,
            "unit": "ms",
            "range": "±2.01%",
            "extra": "p95 5.41 s | 0.19 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.56585,
            "unit": "ms",
            "range": "±6.73%",
            "extra": "p95 33.57 ms | 40.3 ops/sec | 20 samples"
          }
        ],
        "release": {
          "ref": "0.18.6-rc.1",
          "candidateVersion": "0.18.6-rc.1",
          "previousVersion": "0.18.5",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "802a71a8668f0e410a4179dc93e4a3e908f604197541a9d8d7b5e6f5de2e9baf"
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
            "value": 0.8136,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 891.2 µs | 1,222.44 ops/sec | 1218 samples"
          },
          {
            "name": "card",
            "value": 0.353,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 413.2 µs | 2,750.74 ops/sec | 2737 samples"
          },
          {
            "name": "consts",
            "value": 0.1604,
            "unit": "ms",
            "range": "±0.83%",
            "extra": "p95 218.3 µs | 5,954.92 ops/sec | 5836 samples"
          },
          {
            "name": "counter",
            "value": 1.4583,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 1.59 ms | 679.34 ops/sec | 677 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.8965,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 2.01 ms | 522.15 ops/sec | 521 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.498,
            "unit": "ms",
            "range": "±0.75%",
            "extra": "p95 2.65 ms | 396.17 ops/sec | 395 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.6279,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 727.1 µs | 1,540.21 ops/sec | 1527 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.09,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 1.18 ms | 915.3 ops/sec | 913 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.9946,
            "unit": "ms",
            "range": "±2.41%",
            "extra": "p95 1.07 ms | 1,003.54 ops/sec | 987 samples"
          },
          {
            "name": "page",
            "value": 0.4634,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 541.5 µs | 2,084.29 ops/sec | 2071 samples"
          },
          {
            "name": "page-tsx",
            "value": 3.3608,
            "unit": "ms",
            "range": "±0.76%",
            "extra": "p95 3.52 ms | 294.87 ops/sec | 295 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 3.2871,
            "unit": "ms",
            "range": "±0.76%",
            "extra": "p95 3.43 ms | 301.26 ops/sec | 301 samples"
          },
          {
            "name": "spot-loader",
            "value": 1.4097,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 1.56 ms | 702.11 ops/sec | 699 samples"
          },
          {
            "name": "typography",
            "value": 1.94635,
            "unit": "ms",
            "range": "±0.85%",
            "extra": "p95 2.17 ms | 504.58 ops/sec | 502 samples"
          },
          {
            "name": "use-memo",
            "value": 0.502,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 597.9 µs | 1,920.49 ops/sec | 1907 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.4236,
            "unit": "ms",
            "range": "±2.32%",
            "extra": "p95 490 µs | 2,285.79 ops/sec | 2240 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.4009,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 468 µs | 2,413.88 ops/sec | 2400 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 9.80665,
            "unit": "ms",
            "range": "±4.64%",
            "extra": "p95 15.82 ms | 98.28 ops/sec | 96 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.4642,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 556 µs | 2,072.32 ops/sec | 2053 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 5.88275,
            "unit": "ms",
            "range": "±0.75%",
            "extra": "p95 6.46 ms | 168.33 ops/sec | 168 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 5006.7975,
            "unit": "ms",
            "range": "±7.04%",
            "extra": "p95 6.61 s | 0.2 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 23.627,
            "unit": "ms",
            "range": "±2.45%",
            "extra": "p95 25.69 ms | 41.81 ops/sec | 21 samples"
          }
        ],
        "release": {
          "ref": "0.18.6",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.5",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "26dac55b5c69ac1f1979ad23f6188ce6e24d66d26a3cf258bca6ca67ac4ea4a5"
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
            "value": 0.4935,
            "unit": "ms",
            "range": "±12.19%",
            "extra": "p95 654.8 µs | 1,999.73 ops/sec | 1842 samples"
          },
          {
            "name": "card",
            "value": 0.13695,
            "unit": "ms",
            "range": "±0.68%",
            "extra": "p95 238.4 µs | 6,742.87 ops/sec | 6422 samples"
          },
          {
            "name": "consts",
            "value": 0.0751,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 119 µs | 12,740.19 ops/sec | 12342 samples"
          },
          {
            "name": "counter",
            "value": 0.8434,
            "unit": "ms",
            "range": "±2.61%",
            "extra": "p95 1.06 ms | 1,171.13 ops/sec | 1142 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.04085,
            "unit": "ms",
            "range": "±2.74%",
            "extra": "p95 1.24 ms | 949.25 ops/sec | 928 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.3919,
            "unit": "ms",
            "range": "±1.33%",
            "extra": "p95 2.9 ms | 407.61 ops/sec | 403 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.438,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 597.5 µs | 2,248.56 ops/sec | 2201 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.7901,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 981 µs | 1,253.58 ops/sec | 1241 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.7446,
            "unit": "ms",
            "range": "±0.69%",
            "extra": "p95 972.7 µs | 1,331.71 ops/sec | 1314 samples"
          },
          {
            "name": "page",
            "value": 0.2132,
            "unit": "ms",
            "range": "±0.69%",
            "extra": "p95 335.4 µs | 4,414.81 ops/sec | 4248 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.966,
            "unit": "ms",
            "range": "±0.75%",
            "extra": "p95 2.33 ms | 497.78 ops/sec | 495 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.9468,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 2.16 ms | 507.87 ops/sec | 507 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.7089,
            "unit": "ms",
            "range": "±0.68%",
            "extra": "p95 926.8 µs | 1,387.72 ops/sec | 1370 samples"
          },
          {
            "name": "typography",
            "value": 1.05725,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 1.23 ms | 938.2 ops/sec | 934 samples"
          },
          {
            "name": "use-memo",
            "value": 0.2805,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 403.9 µs | 3,399.36 ops/sec | 3307 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.2327,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 318.1 µs | 4,098.27 ops/sec | 4023 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.2104,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 323.8 µs | 4,487.56 ops/sec | 4367 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.5497,
            "unit": "ms",
            "range": "±2.08%",
            "extra": "p95 8.13 ms | 172.96 ops/sec | 171 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.1607,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 251.5 µs | 5,825.79 ops/sec | 5639 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.0748,
            "unit": "ms",
            "range": "±1.35%",
            "extra": "p95 4.44 ms | 316.9 ops/sec | 314 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 18.9582,
            "unit": "ms",
            "range": "±2.55%",
            "extra": "p95 24.19 ms | 51.36 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2749.1022,
            "unit": "ms",
            "range": "±1.44%",
            "extra": "p95 2.84 s | 0.36 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 14.3095,
            "unit": "ms",
            "range": "±3.40%",
            "extra": "p95 18.79 ms | 67.82 ops/sec | 34 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.3158,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 1.53 ms | 749.56 ops/sec | 745 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.84315,
            "unit": "ms",
            "range": "±0.78%",
            "extra": "p95 2.36 ms | 532.91 ops/sec | 530 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.63355,
            "unit": "ms",
            "range": "±1.15%",
            "extra": "p95 778.5 µs | 1,564.8 ops/sec | 1540 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 1.0873,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 1.27 ms | 913.1 ops/sec | 908 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.5858,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 736.3 µs | 1,697.45 ops/sec | 1674 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.9705,
            "unit": "ms",
            "range": "±2.42%",
            "extra": "p95 1.44 ms | 997.01 ops/sec | 943 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.6301,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 1.81 ms | 607.39 ops/sec | 605 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.72075,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 925.8 µs | 1,364.56 ops/sec | 1348 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.4172,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 567 µs | 2,349.3 ops/sec | 2301 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.8755,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 1.02 ms | 1,135.53 ops/sec | 1127 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.2434,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 362.8 µs | 3,884.23 ops/sec | 3767 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.6665,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 817.7 µs | 1,484.63 ops/sec | 1470 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.39305,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 521.4 µs | 2,476.86 ops/sec | 2436 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.8663,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 1.05 ms | 1,138.58 ops/sec | 1129 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.4753,
            "unit": "ms",
            "range": "±0.79%",
            "extra": "p95 633 µs | 2,068.64 ops/sec | 2025 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.9426,
            "unit": "ms",
            "range": "±0.95%",
            "extra": "p95 1.3 ms | 1,038.31 ops/sec | 1021 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.2519,
            "unit": "ms",
            "range": "±0.93%",
            "extra": "p95 402.6 µs | 3,773.6 ops/sec | 3603 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.698,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 858.1 µs | 1,426.4 ops/sec | 1411 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.1273,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 199.6 µs | 7,358.71 ops/sec | 7112 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 1.1001,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 1.32 ms | 898.5 ops/sec | 893 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.6652,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 814.2 µs | 1,492.81 ops/sec | 1477 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.93945,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 1.15 ms | 1,051.92 ops/sec | 1044 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.6095,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 747.5 µs | 1,624.72 ops/sec | 1605 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.40515,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 532 µs | 2,427.37 ops/sec | 2386 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.4838,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 650.7 µs | 2,039.64 ops/sec | 2003 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.5475,
            "unit": "ms",
            "range": "±2.13%",
            "extra": "p95 811.9 µs | 1,769.02 ops/sec | 1655 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 1.061,
            "unit": "ms",
            "range": "±1.90%",
            "extra": "p95 1.34 ms | 917.75 ops/sec | 889 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 1.0824,
            "unit": "ms",
            "range": "±0.55%",
            "extra": "p95 1.29 ms | 914.12 ops/sec | 909 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.5798,
            "unit": "ms",
            "range": "±0.94%",
            "extra": "p95 843.7 µs | 1,687.68 ops/sec | 1644 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 1.1306,
            "unit": "ms",
            "range": "±2.33%",
            "extra": "p95 2.22 ms | 847.62 ops/sec | 804 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.5633,
            "unit": "ms",
            "range": "±2.79%",
            "extra": "p95 1.3 ms | 1,693.46 ops/sec | 1522 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.5851,
            "unit": "ms",
            "range": "±2.89%",
            "extra": "p95 1.38 ms | 1,619.43 ops/sec | 1451 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.6556,
            "unit": "ms",
            "range": "±3.22%",
            "extra": "p95 1.7 ms | 1,442.82 ops/sec | 1275 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.2483,
            "unit": "ms",
            "range": "±2.22%",
            "extra": "p95 394.5 µs | 3,783.74 ops/sec | 3415 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.5287,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 649.6 µs | 1,860.2 ops/sec | 1841 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.544,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 697 µs | 1,816.97 ops/sec | 1793 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.5426,
            "unit": "ms",
            "range": "±1.69%",
            "extra": "p95 769.1 µs | 1,796.71 ops/sec | 1721 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.5849,
            "unit": "ms",
            "range": "±0.78%",
            "extra": "p95 745.6 µs | 1,697.92 ops/sec | 1668 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 1.0901,
            "unit": "ms",
            "range": "±2.44%",
            "extra": "p95 1.57 ms | 888.36 ops/sec | 843 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 5.691,
            "unit": "ms",
            "range": "±1.70%",
            "extra": "p95 7.55 ms | 169.51 ops/sec | 168 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 6.62755,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 7.19 ms | 149.79 ops/sec | 150 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.6391,
            "unit": "ms",
            "range": "±1.42%",
            "extra": "p95 873.8 µs | 1,535.86 ops/sec | 1487 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "1fb439a0f2c5b6fa0c7bc7de4f4eca1b047f09646f49de9b66a7403300ac12b1"
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
            "value": 0.4896,
            "unit": "ms",
            "range": "±6.69%",
            "extra": "p95 629.7 µs | 1,974.34 ops/sec | 1882 samples"
          },
          {
            "name": "card",
            "value": 0.2343,
            "unit": "ms",
            "range": "±2.02%",
            "extra": "p95 538.7 µs | 3,914.45 ops/sec | 3502 samples"
          },
          {
            "name": "consts",
            "value": 0.1392,
            "unit": "ms",
            "range": "±1.97%",
            "extra": "p95 246.6 µs | 6,593.53 ops/sec | 5804 samples"
          },
          {
            "name": "counter",
            "value": 0.8306,
            "unit": "ms",
            "range": "±5.29%",
            "extra": "p95 2.18 ms | 1,105.17 ops/sec | 973 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.0519,
            "unit": "ms",
            "range": "±2.72%",
            "extra": "p95 2.69 ms | 880.92 ops/sec | 812 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.7836,
            "unit": "ms",
            "range": "±3.30%",
            "extra": "p95 5.06 ms | 329.21 ops/sec | 309 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.5078,
            "unit": "ms",
            "range": "±2.86%",
            "extra": "p95 1.5 ms | 1,782.73 ops/sec | 1553 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.8483,
            "unit": "ms",
            "range": "±3.13%",
            "extra": "p95 2.58 ms | 1,077.16 ops/sec | 957 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.7859,
            "unit": "ms",
            "range": "±2.44%",
            "extra": "p95 1.77 ms | 1,189.48 ops/sec | 1098 samples"
          },
          {
            "name": "page",
            "value": 0.2886,
            "unit": "ms",
            "range": "±2.42%",
            "extra": "p95 811.1 µs | 3,157.89 ops/sec | 2760 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.12215,
            "unit": "ms",
            "range": "±3.46%",
            "extra": "p95 4.16 ms | 411.8 ops/sec | 380 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.0476,
            "unit": "ms",
            "range": "±3.07%",
            "extra": "p95 4.04 ms | 437.87 ops/sec | 409 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.7365,
            "unit": "ms",
            "range": "±2.77%",
            "extra": "p95 1.95 ms | 1,247.96 ops/sec | 1121 samples"
          },
          {
            "name": "typography",
            "value": 1.112,
            "unit": "ms",
            "range": "±2.70%",
            "extra": "p95 2.65 ms | 818.79 ops/sec | 754 samples"
          },
          {
            "name": "use-memo",
            "value": 0.3297,
            "unit": "ms",
            "range": "±2.05%",
            "extra": "p95 726.8 µs | 2,805.07 ops/sec | 2550 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.3162,
            "unit": "ms",
            "range": "±2.70%",
            "extra": "p95 936 µs | 2,886.38 ops/sec | 2497 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.30145,
            "unit": "ms",
            "range": "±2.82%",
            "extra": "p95 850.2 µs | 3,002.81 ops/sec | 2572 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 7.07885,
            "unit": "ms",
            "range": "±2.28%",
            "extra": "p95 8.73 ms | 142.36 ops/sec | 140 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.2523,
            "unit": "ms",
            "range": "±2.18%",
            "extra": "p95 610.1 µs | 3,623.65 ops/sec | 3207 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 4.09115,
            "unit": "ms",
            "range": "±2.34%",
            "extra": "p95 5.85 ms | 233.81 ops/sec | 228 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 26.3687,
            "unit": "ms",
            "range": "±2.01%",
            "extra": "p95 29.61 ms | 38.11 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3282.3918,
            "unit": "ms",
            "range": "±4.72%",
            "extra": "p95 3.91 s | 0.3 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 17.0408,
            "unit": "ms",
            "range": "±16.57%",
            "extra": "p95 18.88 ms | 57.35 ops/sec | 30 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.3817,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 1.48 ms | 720.46 ops/sec | 717 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.9139,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 2.07 ms | 519.78 ops/sec | 519 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.6554,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 832.7 µs | 1,480.91 ops/sec | 1467 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 1.0838,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 1.22 ms | 913.19 ops/sec | 909 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.60955,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 684.8 µs | 1,601.08 ops/sec | 1594 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.9557,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 1.04 ms | 1,043.81 ops/sec | 1041 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.7603,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 1.88 ms | 565.22 ops/sec | 565 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.7807,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 936.3 µs | 1,259.64 ops/sec | 1248 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.4495,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 531.1 µs | 2,154.29 ops/sec | 2139 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.8647,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 944.6 µs | 1,151.26 ops/sec | 1147 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.3157,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 385 µs | 3,054.72 ops/sec | 3029 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.6645,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 760.4 µs | 1,478.66 ops/sec | 1471 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.451,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 523.5 µs | 2,148.95 ops/sec | 2133 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.8698,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 957.8 µs | 1,139.93 ops/sec | 1135 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.5068,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 581.6 µs | 1,917.83 ops/sec | 1907 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.93135,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 1.01 ms | 1,067.72 ops/sec | 1064 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.3111,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 378 µs | 3,104.92 ops/sec | 3081 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.692,
            "unit": "ms",
            "range": "±0.70%",
            "extra": "p95 867.7 µs | 1,417.29 ops/sec | 1403 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.2035,
            "unit": "ms",
            "range": "±0.73%",
            "extra": "p95 334.6 µs | 4,527.5 ops/sec | 4361 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 1.08565,
            "unit": "ms",
            "range": "±17.25%",
            "extra": "p95 1.57 ms | 898.32 ops/sec | 882 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 1.00845,
            "unit": "ms",
            "range": "±70.89%",
            "extra": "p95 2.24 ms | 971.36 ops/sec | 486 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 5.78395,
            "unit": "ms",
            "range": "±32.91%",
            "extra": "p95 37.73 ms | 222.34 ops/sec | 80 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 3.22595,
            "unit": "ms",
            "range": "±106.39%",
            "extra": "p95 37.22 ms | 321.02 ops/sec | 86 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 1.56,
            "unit": "ms",
            "range": "±118.81%",
            "extra": "p95 19.45 ms | 565.97 ops/sec | 97 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 1.5944,
            "unit": "ms",
            "range": "±45.19%",
            "extra": "p95 9.63 ms | 604.59 ops/sec | 255 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 1.69595,
            "unit": "ms",
            "range": "±88.25%",
            "extra": "p95 29.62 ms | 521.99 ops/sec | 144 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 1.58105,
            "unit": "ms",
            "range": "±2.54%",
            "extra": "p95 2.25 ms | 669.63 ops/sec | 630 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 1.58285,
            "unit": "ms",
            "range": "±1.96%",
            "extra": "p95 2.58 ms | 655.04 ops/sec | 620 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.8406,
            "unit": "ms",
            "range": "±0.84%",
            "extra": "p95 1.05 ms | 1,173.81 ops/sec | 1154 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 1.58645,
            "unit": "ms",
            "range": "±1.41%",
            "extra": "p95 1.8 ms | 705.68 ops/sec | 680 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.8107,
            "unit": "ms",
            "range": "±1.06%",
            "extra": "p95 928.6 µs | 1,392.22 ops/sec | 1335 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.5941,
            "unit": "ms",
            "range": "±1.12%",
            "extra": "p95 900.8 µs | 1,552.39 ops/sec | 1490 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.656,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 926.9 µs | 1,484.99 ops/sec | 1467 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.3204,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 386.3 µs | 3,019.15 ops/sec | 2995 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.55235,
            "unit": "ms",
            "range": "±0.69%",
            "extra": "p95 811.2 µs | 1,730.66 ops/sec | 1704 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.5465,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 795.1 µs | 1,770.89 ops/sec | 1749 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.5432,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 786.2 µs | 1,775.95 ops/sec | 1755 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.6069,
            "unit": "ms",
            "range": "±1.24%",
            "extra": "p95 916.2 µs | 1,574.13 ops/sec | 1531 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 1.10155,
            "unit": "ms",
            "range": "±1.15%",
            "extra": "p95 1.57 ms | 883.09 ops/sec | 868 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 7.42495,
            "unit": "ms",
            "range": "±1.86%",
            "extra": "p95 10.26 ms | 132.6 ops/sec | 132 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 9.0491,
            "unit": "ms",
            "range": "±2.25%",
            "extra": "p95 12.13 ms | 108.64 ops/sec | 108 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.668,
            "unit": "ms",
            "range": "±1.04%",
            "extra": "p95 1.07 ms | 1,418.38 ops/sec | 1380 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "6aea7d7c0e711198fb02421dfc3d1909c6180eed99813e17d7ca30cec7e59b9b"
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
            "value": 0.4945,
            "unit": "ms",
            "range": "±1.07%",
            "extra": "p95 842 µs | 1,883.44 ops/sec | 1815 samples"
          },
          {
            "name": "card",
            "value": 0.2309,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 371.5 µs | 4,072.49 ops/sec | 3967 samples"
          },
          {
            "name": "consts",
            "value": 0.1365,
            "unit": "ms",
            "range": "±0.92%",
            "extra": "p95 257.1 µs | 6,828.2 ops/sec | 6487 samples"
          },
          {
            "name": "counter",
            "value": 0.8064,
            "unit": "ms",
            "range": "±5.10%",
            "extra": "p95 1.38 ms | 1,157.73 ops/sec | 1061 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.9987,
            "unit": "ms",
            "range": "±1.00%",
            "extra": "p95 1.48 ms | 976.2 ops/sec | 960 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.5704,
            "unit": "ms",
            "range": "±2.57%",
            "extra": "p95 4.39 ms | 364 ops/sec | 351 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.5134,
            "unit": "ms",
            "range": "±1.46%",
            "extra": "p95 894.7 µs | 1,799.16 ops/sec | 1698 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.8433,
            "unit": "ms",
            "range": "±1.77%",
            "extra": "p95 1.4 ms | 1,096.68 ops/sec | 1043 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.7504,
            "unit": "ms",
            "range": "±0.81%",
            "extra": "p95 1.11 ms | 1,291.38 ops/sec | 1271 samples"
          },
          {
            "name": "page",
            "value": 0.2782,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 405.8 µs | 3,368.87 ops/sec | 3304 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.0113,
            "unit": "ms",
            "range": "±1.56%",
            "extra": "p95 3.19 ms | 477.69 ops/sec | 469 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.9383,
            "unit": "ms",
            "range": "±0.85%",
            "extra": "p95 2.2 ms | 505.94 ops/sec | 503 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.71,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 875.5 µs | 1,388.65 ops/sec | 1375 samples"
          },
          {
            "name": "typography",
            "value": 1.0528,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 1.14 ms | 946.62 ops/sec | 945 samples"
          },
          {
            "name": "use-memo",
            "value": 0.3216,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 390.1 µs | 2,999.91 ops/sec | 2976 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.3035,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 382.7 µs | 3,182.32 ops/sec | 3145 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.2855,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 371.8 µs | 3,362.12 ops/sec | 3322 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.7094,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 6.11 ms | 174.67 ops/sec | 175 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.2394,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 303 µs | 3,998.99 ops/sec | 3955 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.4184,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 3.76 ms | 289.65 ops/sec | 289 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 21.80325,
            "unit": "ms",
            "range": "±1.93%",
            "extra": "p95 23.39 ms | 45.36 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 3134.07205,
            "unit": "ms",
            "range": "±2.62%",
            "extra": "p95 3.44 s | 0.32 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 18.5721,
            "unit": "ms",
            "range": "±14.64%",
            "extra": "p95 28.76 ms | 50.91 ops/sec | 25 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.3529,
            "unit": "ms",
            "range": "±1.22%",
            "extra": "p95 2.01 ms | 704.79 ops/sec | 691 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.9148,
            "unit": "ms",
            "range": "±0.80%",
            "extra": "p95 2.18 ms | 513.15 ops/sec | 510 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.6734,
            "unit": "ms",
            "range": "±0.78%",
            "extra": "p95 991.5 µs | 1,434.27 ops/sec | 1411 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 1.0825,
            "unit": "ms",
            "range": "±1.09%",
            "extra": "p95 1.59 ms | 885.5 ops/sec | 869 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.63035,
            "unit": "ms",
            "range": "±1.11%",
            "extra": "p95 1.04 ms | 1,516.14 ops/sec | 1474 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.96675,
            "unit": "ms",
            "range": "±0.92%",
            "extra": "p95 1.39 ms | 1,003.46 ops/sec | 988 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.77825,
            "unit": "ms",
            "range": "±1.93%",
            "extra": "p95 3.22 ms | 530.89 ops/sec | 514 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.78285,
            "unit": "ms",
            "range": "±1.10%",
            "extra": "p95 1.33 ms | 1,235.14 ops/sec | 1206 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.44825,
            "unit": "ms",
            "range": "±0.93%",
            "extra": "p95 794.4 µs | 2,094.25 ops/sec | 2030 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.8696,
            "unit": "ms",
            "range": "±1.17%",
            "extra": "p95 1.4 ms | 1,092.17 ops/sec | 1061 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.3163,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 482.1 µs | 2,980.21 ops/sec | 2917 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.6712,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 904.7 µs | 1,450.91 ops/sec | 1435 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.46115,
            "unit": "ms",
            "range": "±0.76%",
            "extra": "p95 709.4 µs | 2,036.8 ops/sec | 1990 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.8706,
            "unit": "ms",
            "range": "±0.84%",
            "extra": "p95 1.23 ms | 1,114.44 ops/sec | 1098 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.5238,
            "unit": "ms",
            "range": "±1.01%",
            "extra": "p95 904.5 µs | 1,798.61 ops/sec | 1741 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.9592,
            "unit": "ms",
            "range": "±0.78%",
            "extra": "p95 1.18 ms | 1,019.4 ops/sec | 1007 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.3085,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 382.8 µs | 3,111.42 ops/sec | 3082 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.68345,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 816.9 µs | 1,442.53 ops/sec | 1432 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.1995,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 263.6 µs | 4,784.25 ops/sec | 4719 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 1.0826,
            "unit": "ms",
            "range": "±0.75%",
            "extra": "p95 1.35 ms | 908.62 ops/sec | 900 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.68395,
            "unit": "ms",
            "range": "±2.54%",
            "extra": "p95 1.59 ms | 1,365.06 ops/sec | 1246 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.9472,
            "unit": "ms",
            "range": "±1.95%",
            "extra": "p95 1.49 ms | 1,012 ops/sec | 970 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.7072,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 903.5 µs | 1,393.68 ops/sec | 1379 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.4642,
            "unit": "ms",
            "range": "±1.73%",
            "extra": "p95 736.5 µs | 2,038.22 ops/sec | 1934 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.5321,
            "unit": "ms",
            "range": "±2.55%",
            "extra": "p95 1.36 ms | 1,742.04 ops/sec | 1565 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.555,
            "unit": "ms",
            "range": "±2.59%",
            "extra": "p95 1.4 ms | 1,661.44 ops/sec | 1497 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 1.0417,
            "unit": "ms",
            "range": "±2.71%",
            "extra": "p95 2.61 ms | 880.1 ops/sec | 809 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 1.0361,
            "unit": "ms",
            "range": "±2.70%",
            "extra": "p95 2.51 ms | 891.36 ops/sec | 822 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.5649,
            "unit": "ms",
            "range": "±2.84%",
            "extra": "p95 1.63 ms | 1,636.14 ops/sec | 1451 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 1.1003,
            "unit": "ms",
            "range": "±3.02%",
            "extra": "p95 2.85 ms | 817.77 ops/sec | 738 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.55535,
            "unit": "ms",
            "range": "±2.78%",
            "extra": "p95 1.59 ms | 1,663.94 ops/sec | 1480 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.5741,
            "unit": "ms",
            "range": "±2.80%",
            "extra": "p95 1.69 ms | 1,602.6 ops/sec | 1417 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.6757,
            "unit": "ms",
            "range": "±3.17%",
            "extra": "p95 2.18 ms | 1,346.48 ops/sec | 1173 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.3284,
            "unit": "ms",
            "range": "±2.63%",
            "extra": "p95 952.4 µs | 2,769.09 ops/sec | 2413 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.5606,
            "unit": "ms",
            "range": "±2.64%",
            "extra": "p95 1.53 ms | 1,643.3 ops/sec | 1471 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.5703,
            "unit": "ms",
            "range": "±2.47%",
            "extra": "p95 1.44 ms | 1,626.33 ops/sec | 1473 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.5696,
            "unit": "ms",
            "range": "±2.50%",
            "extra": "p95 1.28 ms | 1,636.12 ops/sec | 1486 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.6227,
            "unit": "ms",
            "range": "±3.06%",
            "extra": "p95 1.83 ms | 1,477.43 ops/sec | 1297 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 1.1131,
            "unit": "ms",
            "range": "±3.38%",
            "extra": "p95 2.93 ms | 805.61 ops/sec | 718 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 8.2306,
            "unit": "ms",
            "range": "±1.94%",
            "extra": "p95 9.52 ms | 123.57 ops/sec | 123 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 9.61475,
            "unit": "ms",
            "range": "±2.25%",
            "extra": "p95 11.36 ms | 104.4 ops/sec | 104 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.68035,
            "unit": "ms",
            "range": "±2.62%",
            "extra": "p95 1.77 ms | 1,368.25 ops/sec | 1236 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "786f58b04fc4961068c772874f683e1c623a5d3043816431b1f42936cd570b6f"
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
            "value": 0.3574,
            "unit": "ms",
            "range": "±7.25%",
            "extra": "p95 410 µs | 2,728.83 ops/sec | 2614 samples"
          },
          {
            "name": "card",
            "value": 0.1635,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 213.8 µs | 5,897.45 ops/sec | 5837 samples"
          },
          {
            "name": "consts",
            "value": 0.0826,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 115.9 µs | 11,628.48 ops/sec | 11441 samples"
          },
          {
            "name": "counter",
            "value": 0.59365,
            "unit": "ms",
            "range": "±2.08%",
            "extra": "p95 1.21 ms | 1,585.99 ops/sec | 1480 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.74145,
            "unit": "ms",
            "range": "±2.46%",
            "extra": "p95 799.5 µs | 1,329.22 ops/sec | 1302 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.96745,
            "unit": "ms",
            "range": "±1.21%",
            "extra": "p95 2.21 ms | 499.17 ops/sec | 494 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.3548,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 418.6 µs | 2,733.46 ops/sec | 2708 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.59945,
            "unit": "ms",
            "range": "±2.12%",
            "extra": "p95 895.6 µs | 1,586.55 ops/sec | 1492 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.5525,
            "unit": "ms",
            "range": "±1.73%",
            "extra": "p95 795.5 µs | 1,727.21 ops/sec | 1649 samples"
          },
          {
            "name": "page",
            "value": 0.2119,
            "unit": "ms",
            "range": "±1.91%",
            "extra": "p95 317 µs | 4,424.18 ops/sec | 4047 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.5231,
            "unit": "ms",
            "range": "±2.74%",
            "extra": "p95 3.23 ms | 597.11 ops/sec | 557 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.45105,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 1.52 ms | 686.58 ops/sec | 686 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.5102,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 598.5 µs | 1,901.71 ops/sec | 1889 samples"
          },
          {
            "name": "typography",
            "value": 0.798,
            "unit": "ms",
            "range": "±1.07%",
            "extra": "p95 986.5 µs | 1,226.07 ops/sec | 1205 samples"
          },
          {
            "name": "use-memo",
            "value": 0.2418,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 292.6 µs | 3,991.45 ops/sec | 3927 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.2142,
            "unit": "ms",
            "range": "±1.00%",
            "extra": "p95 289.1 µs | 4,449.89 ops/sec | 4304 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.2078,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 262 µs | 4,623.9 ops/sec | 4569 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.4143,
            "unit": "ms",
            "range": "±1.30%",
            "extra": "p95 5.66 ms | 220.02 ops/sec | 219 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.1722,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 219.8 µs | 5,558.27 ops/sec | 5440 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.6002,
            "unit": "ms",
            "range": "±1.11%",
            "extra": "p95 3.31 ms | 376.1 ops/sec | 373 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 15.6971,
            "unit": "ms",
            "range": "±2.01%",
            "extra": "p95 18.55 ms | 61.4 ops/sec | 64 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2613.49745,
            "unit": "ms",
            "range": "±3.51%",
            "extra": "p95 3 s | 0.38 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 11.5906,
            "unit": "ms",
            "range": "±0.85%",
            "extra": "p95 12.41 ms | 85.59 ops/sec | 43 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.0352,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 1.18 ms | 953.92 ops/sec | 949 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.39095,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 1.48 ms | 715.32 ops/sec | 714 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.4739,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 534.7 µs | 2,054.86 ops/sec | 2044 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.7793,
            "unit": "ms",
            "range": "±1.14%",
            "extra": "p95 999.7 µs | 1,255.76 ops/sec | 1234 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.4439,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 527.3 µs | 2,182.61 ops/sec | 2167 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.69645,
            "unit": "ms",
            "range": "±1.35%",
            "extra": "p95 966.6 µs | 1,395.32 ops/sec | 1356 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.28125,
            "unit": "ms",
            "range": "±2.16%",
            "extra": "p95 2.55 ms | 727.76 ops/sec | 692 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.5699,
            "unit": "ms",
            "range": "±1.30%",
            "extra": "p95 668.6 µs | 1,699.37 ops/sec | 1656 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.3338,
            "unit": "ms",
            "range": "±1.89%",
            "extra": "p95 571 µs | 2,796.06 ops/sec | 2580 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.62705,
            "unit": "ms",
            "range": "±1.44%",
            "extra": "p95 807.6 µs | 1,551.7 ops/sec | 1506 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.2287,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 275.9 µs | 4,240.16 ops/sec | 4209 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.47955,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 549.7 µs | 2,026.98 ops/sec | 2014 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.3295,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 376.7 µs | 2,957.12 ops/sec | 2944 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.6169,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 676.2 µs | 1,589.91 ops/sec | 1585 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.3721,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 424.3 µs | 2,611.82 ops/sec | 2593 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.6645,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 729.3 µs | 1,484.18 ops/sec | 1478 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.2226,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 271.7 µs | 4,348.65 ops/sec | 4317 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.4876,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 542 µs | 2,005.84 ops/sec | 2000 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.1382,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 186.6 µs | 6,971.47 ops/sec | 6896 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.77835,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 835.4 µs | 1,275.26 ops/sec | 1272 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.4745,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 530.6 µs | 2,056.85 ops/sec | 2050 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.6547,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 804.6 µs | 1,487.05 ops/sec | 1479 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.497,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 544.1 µs | 1,976.34 ops/sec | 1971 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.3332,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 382.6 µs | 2,922.35 ops/sec | 2907 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.3714,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 423.3 µs | 2,621.93 ops/sec | 2607 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.3961,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 446.7 µs | 2,460.79 ops/sec | 2450 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.7329,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 802.6 µs | 1,344.67 ops/sec | 1340 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.7343,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 780.1 µs | 1,352.45 ops/sec | 1351 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.4022,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 447.6 µs | 2,432.15 ops/sec | 2425 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.7675,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 822.2 µs | 1,285.63 ops/sec | 1283 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.3919,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 443.1 µs | 2,484.09 ops/sec | 2472 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.40705,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 456.5 µs | 2,390.49 ops/sec | 2378 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.4637,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 513.4 µs | 2,108.42 ops/sec | 2101 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.2314,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 277.1 µs | 4,192.47 ops/sec | 4166 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.3977,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 447.9 µs | 2,451.82 ops/sec | 2442 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.3943,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 473.9 µs | 2,458.57 ops/sec | 2443 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.3916,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 448.2 µs | 2,477.75 ops/sec | 2463 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.4409,
            "unit": "ms",
            "range": "±0.86%",
            "extra": "p95 648 µs | 2,174.67 ops/sec | 2132 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.79585,
            "unit": "ms",
            "range": "±0.96%",
            "extra": "p95 1.09 ms | 1,198.23 ops/sec | 1174 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 5.0221,
            "unit": "ms",
            "range": "±2.13%",
            "extra": "p95 7.34 ms | 193.25 ops/sec | 190 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 5.636,
            "unit": "ms",
            "range": "±1.80%",
            "extra": "p95 7.89 ms | 171.22 ops/sec | 170 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.4762,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 578 µs | 2,034.66 ops/sec | 2020 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "x86_64-pc-windows-msvc",
          "nativeSha256": "b44b569e3b919642fa0307e79197ee805c91ff151b2336a22cec6a4ed71f3b87"
        }
      }
    ]
  }
};
