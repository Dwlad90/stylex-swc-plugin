window.BENCHMARK_DATA = {
  "lastUpdate": 1785964850519,
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
      }
    ]
  }
}