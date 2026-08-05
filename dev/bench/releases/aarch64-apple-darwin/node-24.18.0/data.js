window.BENCHMARK_DATA = {
  "lastUpdate": 1785964756160,
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
        "date": 1785964751797,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "button-props",
            "value": 0.099792,
            "range": "±1.04%",
            "unit": "ms",
            "extra": "p95 162.67 µs | 9,490.45 ops/sec | 8999 samples"
          },
          {
            "name": "buttons-demo",
            "value": 0.373312,
            "range": "±7.68%",
            "unit": "ms",
            "extra": "p95 690.46 µs | 2,588.08 ops/sec | 2126 samples"
          },
          {
            "name": "card",
            "value": 0.171562,
            "range": "±1.56%",
            "unit": "ms",
            "extra": "p95 301.46 µs | 5,785.66 ops/sec | 5312 samples"
          },
          {
            "name": "consts",
            "value": 0.082375,
            "range": "±0.79%",
            "unit": "ms",
            "extra": "p95 144.08 µs | 11,578.71 ops/sec | 10818 samples"
          },
          {
            "name": "counter",
            "value": 0.781146,
            "range": "±1.57%",
            "unit": "ms",
            "extra": "p95 1.16 ms | 1,260.5 ops/sec | 1208 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.989292,
            "range": "±0.87%",
            "unit": "ms",
            "extra": "p95 1.29 ms | 990.21 ops/sec | 976 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.223833,
            "range": "±0.66%",
            "unit": "ms",
            "extra": "p95 1.47 ms | 798.57 ops/sec | 794 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.273125,
            "range": "±6.67%",
            "unit": "ms",
            "extra": "p95 355.67 µs | 3,516.04 ops/sec | 3209 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.461791,
            "range": "±2.84%",
            "unit": "ms",
            "extra": "p95 597.79 µs | 2,138.27 ops/sec | 2055 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.427625,
            "range": "±0.57%",
            "unit": "ms",
            "extra": "p95 561.67 µs | 2,309.24 ops/sec | 2275 samples"
          },
          {
            "name": "page",
            "value": 0.185709,
            "range": "±0.42%",
            "unit": "ms",
            "extra": "p95 236.17 µs | 5,316.88 ops/sec | 5230 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.8865,
            "range": "±2.57%",
            "unit": "ms",
            "extra": "p95 2.24 ms | 531.5 ops/sec | 517 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.865375,
            "range": "±1.09%",
            "unit": "ms",
            "extra": "p95 2.12 ms | 535.88 ops/sec | 530 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.660708,
            "range": "±3.67%",
            "unit": "ms",
            "extra": "p95 864.83 µs | 1,494.04 ops/sec | 1433 samples"
          },
          {
            "name": "typography",
            "value": 1.070875,
            "range": "±1.71%",
            "unit": "ms",
            "extra": "p95 1.41 ms | 915.4 ops/sec | 891 samples"
          },
          {
            "name": "use-memo",
            "value": 0.204292,
            "range": "±0.47%",
            "unit": "ms",
            "extra": "p95 286.33 µs | 4,739.96 ops/sec | 4650 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.224125,
            "range": "±0.37%",
            "unit": "ms",
            "extra": "p95 288.88 µs | 4,359.77 ops/sec | 4309 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.213292,
            "range": "±0.36%",
            "unit": "ms",
            "extra": "p95 276.58 µs | 4,591.72 ops/sec | 4536 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.477375,
            "range": "±1.01%",
            "unit": "ms",
            "extra": "p95 5.22 ms | 216.57 ops/sec | 216 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.196083,
            "range": "±0.58%",
            "unit": "ms",
            "extra": "p95 265.96 µs | 4,849.93 ops/sec | 4774 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.28875,
            "range": "±0.74%",
            "unit": "ms",
            "extra": "p95 3.62 ms | 299.88 ops/sec | 299 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 1987.423854,
            "range": "±6.00%",
            "unit": "ms",
            "extra": "p95 2,451.83 ms | 0.49 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 13.937396,
            "range": "±3.11%",
            "unit": "ms",
            "extra": "p95 16.44 ms | 72.26 ops/sec | 36 samples"
          }
        ]
      }
    ]
  }
}