window.BENCHMARK_DATA = {
  "lastUpdate": 1785964939285,
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
      }
    ]
  }
}