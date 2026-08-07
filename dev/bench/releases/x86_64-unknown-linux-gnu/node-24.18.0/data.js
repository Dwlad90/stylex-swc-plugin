window.BENCHMARK_DATA = {
  "lastUpdate": 1786119658803,
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
      }
    ]
  }
};
