window.BENCHMARK_DATA = {
  "lastUpdate": 1789252262389,
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
            "value": 0.317417,
            "unit": "ms",
            "range": "±0.46%",
            "extra": "p95 390.88 µs | 3,084.54 ops/sec | 3047 samples"
          },
          {
            "name": "card",
            "value": 0.122084,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 130.96 µs | 8,121.23 ops/sec | 8106 samples"
          },
          {
            "name": "consts",
            "value": 0.067667,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 86.58 µs | 14,406.74 ops/sec | 14276 samples"
          },
          {
            "name": "counter",
            "value": 0.606166,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 779.38 µs | 1,558.29 ops/sec | 1543 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.826333,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 876.04 µs | 1,198.46 ops/sec | 1197 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.163209,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 1.34 ms | 834.24 ops/sec | 832 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.258542,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 271.79 µs | 3,833.76 ops/sec | 3830 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.410916,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 435.29 µs | 2,407.05 ops/sec | 2403 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.386895,
            "unit": "ms",
            "range": "±0.91%",
            "extra": "p95 524.33 µs | 2,457.9 ops/sec | 2402 samples"
          },
          {
            "name": "page",
            "value": 0.198083,
            "unit": "ms",
            "range": "±1.23%",
            "extra": "p95 285.17 µs | 4,913.01 ops/sec | 4676 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.773374,
            "unit": "ms",
            "range": "±1.59%",
            "extra": "p95 2.59 ms | 547.73 ops/sec | 536 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.939584,
            "unit": "ms",
            "range": "±2.50%",
            "extra": "p95 3.01 ms | 498.65 ops/sec | 475 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.569084,
            "unit": "ms",
            "range": "±1.56%",
            "extra": "p95 826.87 µs | 1,672.3 ops/sec | 1621 samples"
          },
          {
            "name": "typography",
            "value": 0.869458,
            "unit": "ms",
            "range": "±0.91%",
            "extra": "p95 1.18 ms | 1,110.88 ops/sec | 1094 samples"
          },
          {
            "name": "use-memo",
            "value": 0.180792,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 198.75 µs | 5,441.92 ops/sec | 5426 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.194834,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 211.67 µs | 5,054.58 ops/sec | 5041 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.188208,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 211.33 µs | 5,176.91 ops/sec | 5158 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.232208,
            "unit": "ms",
            "range": "±0.81%",
            "extra": "p95 4.86 ms | 232.01 ops/sec | 232 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.188,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 198.54 µs | 5,278.98 ops/sec | 5273 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.104666,
            "unit": "ms",
            "range": "±0.65%",
            "extra": "p95 3.69 ms | 316.98 ops/sec | 317 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 1872.64498,
            "unit": "ms",
            "range": "±1.88%",
            "extra": "p95 1.97 s | 0.53 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 13.133625,
            "unit": "ms",
            "range": "±2.48%",
            "extra": "p95 13.72 ms | 79.96 ops/sec | 40 samples"
          }
        ],
        "release": {
          "ref": "fix_release-benchmarks",
          "candidateVersion": "0.18.3",
          "previousVersion": "0.18.3",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "dc5c18b9d15982e35c70c2591aaca7202537ffdb2687e9f54a1268dd478d480b"
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
            "value": 0.344666,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 450.38 µs | 2,828.77 ops/sec | 2783 samples"
          },
          {
            "name": "card",
            "value": 0.133542,
            "unit": "ms",
            "range": "±1.01%",
            "extra": "p95 203.83 µs | 7,077.13 ops/sec | 6690 samples"
          },
          {
            "name": "consts",
            "value": 0.078417,
            "unit": "ms",
            "range": "±1.20%",
            "extra": "p95 242.75 µs | 11,707.68 ops/sec | 10317 samples"
          },
          {
            "name": "counter",
            "value": 0.68025,
            "unit": "ms",
            "range": "±1.03%",
            "extra": "p95 913.63 µs | 1,416.26 ops/sec | 1389 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.978166,
            "unit": "ms",
            "range": "±0.95%",
            "extra": "p95 1.31 ms | 986.28 ops/sec | 971 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.367167,
            "unit": "ms",
            "range": "±1.19%",
            "extra": "p95 1.63 ms | 718.46 ops/sec | 711 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.295458,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 347.46 µs | 3,317.84 ops/sec | 3280 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.465146,
            "unit": "ms",
            "range": "±1.74%",
            "extra": "p95 594.96 µs | 2,078.83 ops/sec | 1988 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.417625,
            "unit": "ms",
            "range": "±0.73%",
            "extra": "p95 606.63 µs | 2,285.53 ops/sec | 2242 samples"
          },
          {
            "name": "page",
            "value": 0.186792,
            "unit": "ms",
            "range": "±0.65%",
            "extra": "p95 273.79 µs | 5,090.55 ops/sec | 4960 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.801395,
            "unit": "ms",
            "range": "±0.71%",
            "extra": "p95 2.14 ms | 544.57 ops/sec | 542 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.190437,
            "unit": "ms",
            "range": "±2.90%",
            "extra": "p95 3.93 ms | 443.73 ops/sec | 418 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.636146,
            "unit": "ms",
            "range": "±0.90%",
            "extra": "p95 776.04 µs | 1,531.77 ops/sec | 1510 samples"
          },
          {
            "name": "typography",
            "value": 0.980875,
            "unit": "ms",
            "range": "±0.91%",
            "extra": "p95 1.28 ms | 988.31 ops/sec | 975 samples"
          },
          {
            "name": "use-memo",
            "value": 0.200291,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 312.71 µs | 4,726.31 ops/sec | 4604 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.22225,
            "unit": "ms",
            "range": "±0.70%",
            "extra": "p95 296.96 µs | 4,367.35 ops/sec | 4256 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.207209,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 244.62 µs | 4,696.81 ops/sec | 4652 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.921208,
            "unit": "ms",
            "range": "±2.31%",
            "extra": "p95 5.72 ms | 200.48 ops/sec | 198 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.20775,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 256.92 µs | 4,655.34 ops/sec | 4568 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.814417,
            "unit": "ms",
            "range": "±1.39%",
            "extra": "p95 4.71 ms | 258.69 ops/sec | 257 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2263.822396,
            "unit": "ms",
            "range": "±7.87%",
            "extra": "p95 2.8 s | 0.42 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 21.363875,
            "unit": "ms",
            "range": "±14.38%",
            "extra": "p95 40.25 ms | 48.58 ops/sec | 23 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "580ccac0e8520aba6a9ef2e9ee7e3049bcc777724eb07a885999b2c8c30e5cb9"
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
            "value": 0.395521,
            "unit": "ms",
            "range": "±3.86%",
            "extra": "p95 1.68 ms | 2,241.94 ops/sec | 1732 samples"
          },
          {
            "name": "card",
            "value": 0.1475,
            "unit": "ms",
            "range": "±2.84%",
            "extra": "p95 654.04 µs | 5,974.71 ops/sec | 4498 samples"
          },
          {
            "name": "consts",
            "value": 0.080459,
            "unit": "ms",
            "range": "±2.33%",
            "extra": "p95 298.54 µs | 11,209.69 ops/sec | 8737 samples"
          },
          {
            "name": "counter",
            "value": 0.879501,
            "unit": "ms",
            "range": "±3.94%",
            "extra": "p95 2.8 ms | 1,029.82 ops/sec | 854 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.33175,
            "unit": "ms",
            "range": "±4.28%",
            "extra": "p95 3.56 ms | 700.65 ops/sec | 599 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.620333,
            "unit": "ms",
            "range": "±1.56%",
            "extra": "p95 2.23 ms | 602.15 ops/sec | 587 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.31525,
            "unit": "ms",
            "range": "±1.58%",
            "extra": "p95 640.08 µs | 2,944.97 ops/sec | 2718 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.563125,
            "unit": "ms",
            "range": "±3.30%",
            "extra": "p95 2.06 ms | 1,588 ops/sec | 1319 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.482625,
            "unit": "ms",
            "range": "±1.68%",
            "extra": "p95 883.87 µs | 1,984.05 ops/sec | 1851 samples"
          },
          {
            "name": "page",
            "value": 0.197,
            "unit": "ms",
            "range": "±1.30%",
            "extra": "p95 391.13 µs | 4,712.83 ops/sec | 4353 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.307375,
            "unit": "ms",
            "range": "±1.82%",
            "extra": "p95 3.12 ms | 427.2 ops/sec | 417 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.275208,
            "unit": "ms",
            "range": "±1.99%",
            "extra": "p95 3.3 ms | 428.48 ops/sec | 416 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.780084,
            "unit": "ms",
            "range": "±1.47%",
            "extra": "p95 1.21 ms | 1,250.66 ops/sec | 1199 samples"
          },
          {
            "name": "typography",
            "value": 1.183521,
            "unit": "ms",
            "range": "±1.79%",
            "extra": "p95 1.86 ms | 818.98 ops/sec | 786 samples"
          },
          {
            "name": "use-memo",
            "value": 0.220333,
            "unit": "ms",
            "range": "±2.45%",
            "extra": "p95 811.42 µs | 4,071.84 ops/sec | 3347 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.233959,
            "unit": "ms",
            "range": "±2.29%",
            "extra": "p95 855.54 µs | 3,842.63 ops/sec | 3215 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.23125,
            "unit": "ms",
            "range": "±2.26%",
            "extra": "p95 854.17 µs | 3,796.35 ops/sec | 3087 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.78575,
            "unit": "ms",
            "range": "±2.47%",
            "extra": "p95 8.41 ms | 168.73 ops/sec | 166 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.221833,
            "unit": "ms",
            "range": "±1.69%",
            "extra": "p95 430.37 µs | 4,199.17 ops/sec | 3831 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 4.359834,
            "unit": "ms",
            "range": "±1.77%",
            "extra": "p95 5.65 ms | 225.07 ops/sec | 222 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2820.571917,
            "unit": "ms",
            "range": "±6.24%",
            "extra": "p95 3.58 s | 0.35 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 16.690062,
            "unit": "ms",
            "range": "±5.11%",
            "extra": "p95 21.6 ms | 60.51 ops/sec | 30 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "580ccac0e8520aba6a9ef2e9ee7e3049bcc777724eb07a885999b2c8c30e5cb9"
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
            "value": 0.387833,
            "unit": "ms",
            "range": "±1.51%",
            "extra": "p95 616.17 µs | 2,485.44 ops/sec | 2343 samples"
          },
          {
            "name": "card",
            "value": 0.142458,
            "unit": "ms",
            "range": "±1.79%",
            "extra": "p95 230.17 µs | 6,681.47 ops/sec | 6008 samples"
          },
          {
            "name": "consts",
            "value": 0.077709,
            "unit": "ms",
            "range": "±1.52%",
            "extra": "p95 298.08 µs | 11,544.6 ops/sec | 9444 samples"
          },
          {
            "name": "counter",
            "value": 0.81125,
            "unit": "ms",
            "range": "±1.49%",
            "extra": "p95 1.17 ms | 1,208.23 ops/sec | 1164 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.401333,
            "unit": "ms",
            "range": "±3.85%",
            "extra": "p95 3.52 ms | 668.31 ops/sec | 584 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.7375,
            "unit": "ms",
            "range": "±4.14%",
            "extra": "p95 4 ms | 542.86 ops/sec | 489 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.325895,
            "unit": "ms",
            "range": "±2.24%",
            "extra": "p95 797.92 µs | 2,877.51 ops/sec | 2552 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.533624,
            "unit": "ms",
            "range": "±2.68%",
            "extra": "p95 1.28 ms | 1,733.26 ops/sec | 1534 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.450042,
            "unit": "ms",
            "range": "±1.68%",
            "extra": "p95 789.5 µs | 2,099.92 ops/sec | 1970 samples"
          },
          {
            "name": "page",
            "value": 0.199709,
            "unit": "ms",
            "range": "±1.52%",
            "extra": "p95 401.17 µs | 4,692.88 ops/sec | 4283 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.334729,
            "unit": "ms",
            "range": "±2.59%",
            "extra": "p95 3.7 ms | 410.85 ops/sec | 394 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.408333,
            "unit": "ms",
            "range": "±3.37%",
            "extra": "p95 4.73 ms | 397.96 ops/sec | 370 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.772562,
            "unit": "ms",
            "range": "±2.12%",
            "extra": "p95 1.39 ms | 1,240.99 ops/sec | 1156 samples"
          },
          {
            "name": "typography",
            "value": 1.195625,
            "unit": "ms",
            "range": "±1.73%",
            "extra": "p95 1.84 ms | 812.55 ops/sec | 779 samples"
          },
          {
            "name": "use-memo",
            "value": 0.208709,
            "unit": "ms",
            "range": "±1.31%",
            "extra": "p95 394.12 µs | 4,480.59 ops/sec | 4167 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.229791,
            "unit": "ms",
            "range": "±1.34%",
            "extra": "p95 432.08 µs | 4,083.34 ops/sec | 3801 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.219334,
            "unit": "ms",
            "range": "±1.38%",
            "extra": "p95 416.92 µs | 4,268.26 ops/sec | 3969 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.676875,
            "unit": "ms",
            "range": "±2.03%",
            "extra": "p95 7.28 ms | 174.4 ops/sec | 173 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.224083,
            "unit": "ms",
            "range": "±1.60%",
            "extra": "p95 492.42 µs | 4,153.53 ops/sec | 3764 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 4.449916,
            "unit": "ms",
            "range": "±2.96%",
            "extra": "p95 6.57 ms | 218.24 ops/sec | 211 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2660.228896,
            "unit": "ms",
            "range": "±7.01%",
            "extra": "p95 3.31 s | 0.36 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 19.394,
            "unit": "ms",
            "range": "±14.41%",
            "extra": "p95 36.15 ms | 48.94 ops/sec | 23 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "da44c5fdce7fdd761184e80d8835b2284c218078bd7963c1ee3f5933d003b755"
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
            "value": 0.296417,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 325.71 µs | 3,323.51 ops/sec | 3308 samples"
          },
          {
            "name": "card",
            "value": 0.1215,
            "unit": "ms",
            "range": "±0.12%",
            "extra": "p95 128.96 µs | 8,167.75 ops/sec | 8154 samples"
          },
          {
            "name": "consts",
            "value": 0.079,
            "unit": "ms",
            "range": "±0.55%",
            "extra": "p95 127.5 µs | 12,243.24 ops/sec | 11679 samples"
          },
          {
            "name": "counter",
            "value": 0.591916,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 619.08 µs | 1,674.77 ops/sec | 1673 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.827458,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 858.62 µs | 1,201.07 ops/sec | 1201 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.157167,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 1.19 ms | 860.72 ops/sec | 861 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.261375,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 305.67 µs | 3,718.54 ops/sec | 3697 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.406041,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 422.08 µs | 2,440.43 ops/sec | 2437 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.372,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 413.88 µs | 2,633.89 ops/sec | 2624 samples"
          },
          {
            "name": "page",
            "value": 0.167542,
            "unit": "ms",
            "range": "±0.18%",
            "extra": "p95 178.42 µs | 5,895 ops/sec | 5876 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.759479,
            "unit": "ms",
            "range": "±1.77%",
            "extra": "p95 2.45 ms | 553.35 ops/sec | 536 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.496666,
            "unit": "ms",
            "range": "±0.84%",
            "extra": "p95 1.88 ms | 633.42 ops/sec | 628 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.558084,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 577.71 µs | 1,779.39 ops/sec | 1779 samples"
          },
          {
            "name": "typography",
            "value": 0.824041,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 844.87 µs | 1,206.99 ops/sec | 1207 samples"
          },
          {
            "name": "use-memo",
            "value": 0.179125,
            "unit": "ms",
            "range": "±0.13%",
            "extra": "p95 189.13 µs | 5,546.31 ops/sec | 5539 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.195834,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 206.33 µs | 5,063.12 ops/sec | 5053 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.187292,
            "unit": "ms",
            "range": "±1.27%",
            "extra": "p95 231.87 µs | 5,186.74 ops/sec | 5079 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.783625,
            "unit": "ms",
            "range": "±4.70%",
            "extra": "p95 6.77 ms | 198.94 ops/sec | 191 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.208125,
            "unit": "ms",
            "range": "±0.99%",
            "extra": "p95 263.08 µs | 4,669.9 ops/sec | 4523 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 4.120166,
            "unit": "ms",
            "range": "±2.87%",
            "extra": "p95 6.19 ms | 231.42 ops/sec | 225 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 1901.052938,
            "unit": "ms",
            "range": "±3.14%",
            "extra": "p95 2.03 s | 0.53 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 13.345312,
            "unit": "ms",
            "range": "±2.23%",
            "extra": "p95 14.28 ms | 75.34 ops/sec | 38 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-dev.1",
          "candidateVersion": "0.18.4-dev.1",
          "previousVersion": "0.18.3",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "da44c5fdce7fdd761184e80d8835b2284c218078bd7963c1ee3f5933d003b755"
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
            "value": 0.336166,
            "unit": "ms",
            "range": "±0.98%",
            "extra": "p95 512.79 µs | 2,806.57 ops/sec | 2697 samples"
          },
          {
            "name": "card",
            "value": 0.136958,
            "unit": "ms",
            "range": "±1.55%",
            "extra": "p95 232.67 µs | 6,865.78 ops/sec | 6431 samples"
          },
          {
            "name": "consts",
            "value": 0.069625,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 94.12 µs | 13,850.25 ops/sec | 13551 samples"
          },
          {
            "name": "counter",
            "value": 0.62825,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 795.87 µs | 1,548.37 ops/sec | 1533 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.884125,
            "unit": "ms",
            "range": "±0.86%",
            "extra": "p95 1.22 ms | 1,089.78 ops/sec | 1075 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.212021,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 1.45 ms | 809.32 ops/sec | 806 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.271042,
            "unit": "ms",
            "range": "±0.55%",
            "extra": "p95 365.37 µs | 3,532.69 ops/sec | 3475 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.431521,
            "unit": "ms",
            "range": "±0.87%",
            "extra": "p95 583.29 µs | 2,229.34 ops/sec | 2188 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.390083,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 561.75 µs | 2,471.98 ops/sec | 2433 samples"
          },
          {
            "name": "page",
            "value": 0.205,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 281.75 µs | 4,780.49 ops/sec | 4671 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.727541,
            "unit": "ms",
            "range": "±1.14%",
            "extra": "p95 2.24 ms | 560.6 ops/sec | 553 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.569354,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 1.83 ms | 623.54 ops/sec | 622 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.587708,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 663.08 µs | 1,678.28 ops/sec | 1675 samples"
          },
          {
            "name": "typography",
            "value": 0.869583,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 993.58 µs | 1,131.9 ops/sec | 1129 samples"
          },
          {
            "name": "use-memo",
            "value": 0.186583,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 234.12 µs | 5,202.89 ops/sec | 5144 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.202209,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 218.29 µs | 4,876.11 ops/sec | 4864 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.19375,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 207.79 µs | 5,097.07 ops/sec | 5085 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.461083,
            "unit": "ms",
            "range": "±1.09%",
            "extra": "p95 5.07 ms | 216.21 ops/sec | 215 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.22775,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 295.75 µs | 4,409.47 ops/sec | 4323 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.239145,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 3.71 ms | 302.79 ops/sec | 302 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 1967.509687,
            "unit": "ms",
            "range": "±2.61%",
            "extra": "p95 2.1 s | 0.51 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 12.154958,
            "unit": "ms",
            "range": "±0.82%",
            "extra": "p95 12.83 ms | 81.39 ops/sec | 41 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-rc.1",
          "candidateVersion": "0.18.4-rc.1",
          "previousVersion": "0.18.3",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "3ee874f2bb02222c0686151cc5b526e425a8e19b386d2c8a0589b5ebe2a15350"
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
            "value": 0.319959,
            "unit": "ms",
            "range": "±2.01%",
            "extra": "p95 510.33 µs | 3,030.57 ops/sec | 2769 samples"
          },
          {
            "name": "card",
            "value": 0.11675,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 144.25 µs | 8,401.48 ops/sec | 8219 samples"
          },
          {
            "name": "consts",
            "value": 0.074917,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 92.42 µs | 13,208.63 ops/sec | 12913 samples"
          },
          {
            "name": "counter",
            "value": 0.540417,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 694.71 µs | 1,812.04 ops/sec | 1784 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.744375,
            "unit": "ms",
            "range": "±0.74%",
            "extra": "p95 1.02 ms | 1,266.39 ops/sec | 1248 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.448916,
            "unit": "ms",
            "range": "±1.62%",
            "extra": "p95 1.8 ms | 691.14 ops/sec | 674 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.278646,
            "unit": "ms",
            "range": "±1.29%",
            "extra": "p95 433.79 µs | 3,413.71 ops/sec | 3234 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.494042,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 521.62 µs | 1,995.15 ops/sec | 1990 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.457375,
            "unit": "ms",
            "range": "±1.05%",
            "extra": "p95 560.42 µs | 2,135.8 ops/sec | 2094 samples"
          },
          {
            "name": "page",
            "value": 0.148875,
            "unit": "ms",
            "range": "±0.88%",
            "extra": "p95 234.04 µs | 6,455.38 ops/sec | 6170 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.528395,
            "unit": "ms",
            "range": "±0.69%",
            "extra": "p95 1.87 ms | 631.87 ops/sec | 628 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.450313,
            "unit": "ms",
            "range": "±1.34%",
            "extra": "p95 2.13 ms | 660.01 ops/sec | 648 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.479626,
            "unit": "ms",
            "range": "±1.09%",
            "extra": "p95 783.54 µs | 1,925.2 ops/sec | 1854 samples"
          },
          {
            "name": "typography",
            "value": 0.748375,
            "unit": "ms",
            "range": "±1.14%",
            "extra": "p95 1.02 ms | 1,269.65 ops/sec | 1241 samples"
          },
          {
            "name": "use-memo",
            "value": 0.156584,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 224 µs | 5,878.45 ops/sec | 5762 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.198375,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 213.04 µs | 4,986.11 ops/sec | 4970 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.195458,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 234.75 µs | 4,957.69 ops/sec | 4923 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.206354,
            "unit": "ms",
            "range": "±0.68%",
            "extra": "p95 4.76 ms | 231.94 ops/sec | 232 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.15,
            "unit": "ms",
            "range": "±0.10%",
            "extra": "p95 158.42 µs | 6,633.92 ops/sec | 6627 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.851375,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 3.18 ms | 347.32 ops/sec | 347 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 1758.343687,
            "unit": "ms",
            "range": "±2.30%",
            "extra": "p95 1.89 s | 0.56 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 10.832896,
            "unit": "ms",
            "range": "±0.93%",
            "extra": "p95 11.03 ms | 91.67 ops/sec | 46 samples"
          }
        ],
        "release": {
          "ref": "0.18.4-rc.2",
          "candidateVersion": "0.18.4-rc.2",
          "previousVersion": "0.18.3",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "3f9f944480169ad5160226297aa8ab42810123e9a160c819a556c19ecbeef713"
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
            "value": 0.409063,
            "unit": "ms",
            "range": "±0.77%",
            "extra": "p95 574.71 µs | 2,417.49 ops/sec | 2350 samples"
          },
          {
            "name": "card",
            "value": 0.134208,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 165.33 µs | 7,174.8 ops/sec | 7087 samples"
          },
          {
            "name": "consts",
            "value": 0.073417,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 90.17 µs | 13,135.55 ops/sec | 12954 samples"
          },
          {
            "name": "counter",
            "value": 0.697208,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 870.29 µs | 1,400.69 ops/sec | 1387 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.087229,
            "unit": "ms",
            "range": "±0.75%",
            "extra": "p95 1.35 ms | 905.38 ops/sec | 896 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.383604,
            "unit": "ms",
            "range": "±1.05%",
            "extra": "p95 1.82 ms | 705.77 ops/sec | 696 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.294292,
            "unit": "ms",
            "range": "±0.99%",
            "extra": "p95 419.87 µs | 3,250.87 ops/sec | 3159 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.50625,
            "unit": "ms",
            "range": "±1.57%",
            "extra": "p95 766.96 µs | 1,904.51 ops/sec | 1819 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.423167,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 534.13 µs | 2,300.7 ops/sec | 2279 samples"
          },
          {
            "name": "page",
            "value": 0.20425,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 303.71 µs | 4,736.65 ops/sec | 4591 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.860583,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 2.09 ms | 533.66 ops/sec | 532 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.796333,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 2.02 ms | 553.9 ops/sec | 552 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.661833,
            "unit": "ms",
            "range": "±0.47%",
            "extra": "p95 805.13 µs | 1,476.65 ops/sec | 1467 samples"
          },
          {
            "name": "typography",
            "value": 1.007645,
            "unit": "ms",
            "range": "±0.81%",
            "extra": "p95 1.29 ms | 970.27 ops/sec | 958 samples"
          },
          {
            "name": "use-memo",
            "value": 0.201458,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 257.5 µs | 4,810.45 ops/sec | 4749 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.218792,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 277.17 µs | 4,437.21 ops/sec | 4391 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.212542,
            "unit": "ms",
            "range": "±0.98%",
            "extra": "p95 288.33 µs | 4,549.23 ops/sec | 4394 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.197125,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 5.55 ms | 192.25 ops/sec | 192 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.21075,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 262.08 µs | 4,605.92 ops/sec | 4561 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.882146,
            "unit": "ms",
            "range": "±0.89%",
            "extra": "p95 4.2 ms | 258.37 ops/sec | 258 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2189.005208,
            "unit": "ms",
            "range": "±1.37%",
            "extra": "p95 2.26 s | 0.46 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 14.388417,
            "unit": "ms",
            "range": "±1.16%",
            "extra": "p95 16.1 ms | 69.17 ops/sec | 35 samples"
          }
        ],
        "release": {
          "ref": "0.18.4",
          "candidateVersion": "0.18.4",
          "previousVersion": "0.18.3",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "d5b6826214877c9b2c1c26a830c00037a7df0bf2808d67d85da66ef4aa5969d6"
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
            "value": 0.325605,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 363.88 µs | 3,039.24 ops/sec | 3026 samples"
          },
          {
            "name": "card",
            "value": 0.132625,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 146.42 µs | 7,483.35 ops/sec | 7433 samples"
          },
          {
            "name": "consts",
            "value": 0.073,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 80.42 µs | 13,631.47 ops/sec | 13559 samples"
          },
          {
            "name": "counter",
            "value": 0.743813,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 903.87 µs | 1,340.89 ops/sec | 1326 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.976271,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 1.19 ms | 1,018.84 ops/sec | 1010 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.258416,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 1.35 ms | 792.51 ops/sec | 791 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.282208,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 336.25 µs | 3,475.12 ops/sec | 3450 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.455583,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 579.67 µs | 2,123.7 ops/sec | 2101 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.405292,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 462 µs | 2,437.28 ops/sec | 2426 samples"
          },
          {
            "name": "page",
            "value": 0.181917,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 201.04 µs | 5,441.49 ops/sec | 5410 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.675541,
            "unit": "ms",
            "range": "±0.40%",
            "extra": "p95 1.81 ms | 595.41 ops/sec | 595 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.640875,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 1.93 ms | 596.37 ops/sec | 594 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.627,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 797.5 µs | 1,550.39 ops/sec | 1537 samples"
          },
          {
            "name": "typography",
            "value": 0.912875,
            "unit": "ms",
            "range": "±0.73%",
            "extra": "p95 1.17 ms | 1,059.76 ops/sec | 1049 samples"
          },
          {
            "name": "use-memo",
            "value": 0.194375,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 214.5 µs | 5,099.49 ops/sec | 5075 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.211625,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 231.29 µs | 4,690.22 ops/sec | 4674 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.203125,
            "unit": "ms",
            "range": "±0.55%",
            "extra": "p95 230.21 µs | 4,858.58 ops/sec | 4804 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.559375,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 4.74 ms | 219.97 ops/sec | 220 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.205917,
            "unit": "ms",
            "range": "±1.14%",
            "extra": "p95 262.29 µs | 4,729.01 ops/sec | 4613 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.425896,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 3.68 ms | 290.47 ops/sec | 290 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2045.258771,
            "unit": "ms",
            "range": "±1.99%",
            "extra": "p95 2.15 s | 0.48 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 12.871541,
            "unit": "ms",
            "range": "±1.89%",
            "extra": "p95 14.98 ms | 76.95 ops/sec | 39 samples"
          }
        ],
        "release": {
          "ref": "0.18.5-rc.1",
          "candidateVersion": "0.18.5-rc.1",
          "previousVersion": "0.18.4",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "3e5611add9f6d6acce558d2b5c4397707de9105fc711e338a5ee51905905e040"
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
            "value": 0.323,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 355.42 µs | 3,060.7 ops/sec | 3051 samples"
          },
          {
            "name": "card",
            "value": 0.132416,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 147.04 µs | 7,461.49 ops/sec | 7428 samples"
          },
          {
            "name": "consts",
            "value": 0.073542,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 82.83 µs | 13,419.95 ops/sec | 13334 samples"
          },
          {
            "name": "counter",
            "value": 0.734437,
            "unit": "ms",
            "range": "±1.75%",
            "extra": "p95 1.25 ms | 1,301.32 ops/sec | 1230 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.917709,
            "unit": "ms",
            "range": "±0.80%",
            "extra": "p95 1.19 ms | 1,047.86 ops/sec | 1036 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.295167,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 1.55 ms | 748.6 ops/sec | 745 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.28025,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 305.96 µs | 3,523.53 ops/sec | 3513 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.578396,
            "unit": "ms",
            "range": "±2.92%",
            "extra": "p95 1.14 ms | 1,636.23 ops/sec | 1474 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.407042,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 475.71 µs | 2,410.48 ops/sec | 2399 samples"
          },
          {
            "name": "page",
            "value": 0.181583,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 198.37 µs | 5,436.07 ops/sec | 5411 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.671459,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 1.83 ms | 593.93 ops/sec | 593 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.625584,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 1.74 ms | 610.99 ops/sec | 611 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.65825,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 809.54 µs | 1,499.59 ops/sec | 1485 samples"
          },
          {
            "name": "typography",
            "value": 0.9025,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 997.33 µs | 1,098.85 ops/sec | 1097 samples"
          },
          {
            "name": "use-memo",
            "value": 0.196042,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 241.58 µs | 4,923.06 ops/sec | 4869 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.21275,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 241.54 µs | 4,616.63 ops/sec | 4587 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.204708,
            "unit": "ms",
            "range": "±0.15%",
            "extra": "p95 221.58 µs | 4,833.86 ops/sec | 4824 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.576458,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 4.72 ms | 218.94 ops/sec | 219 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.2035,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 238.67 µs | 4,795.4 ops/sec | 4750 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.422645,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 3.58 ms | 291.86 ops/sec | 292 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2047.659,
            "unit": "ms",
            "range": "±6.77%",
            "extra": "p95 2.64 s | 0.47 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 13.45875,
            "unit": "ms",
            "range": "±2.40%",
            "extra": "p95 16.07 ms | 72.87 ops/sec | 37 samples"
          }
        ],
        "release": {
          "ref": "0.18.5",
          "candidateVersion": "0.18.5",
          "previousVersion": "0.18.4",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "dcc31824e42e1a0fde9aef75f9d350cdea67d55aa0089aa43c8e5a1696e17a55"
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
            "value": 0.383167,
            "unit": "ms",
            "range": "±0.94%",
            "extra": "p95 545.33 µs | 2,581.33 ops/sec | 2501 samples"
          },
          {
            "name": "card",
            "value": 0.148875,
            "unit": "ms",
            "range": "±1.36%",
            "extra": "p95 338.75 µs | 6,145.17 ops/sec | 5543 samples"
          },
          {
            "name": "consts",
            "value": 0.078833,
            "unit": "ms",
            "range": "±0.92%",
            "extra": "p95 173.17 µs | 11,844.26 ops/sec | 10805 samples"
          },
          {
            "name": "counter",
            "value": 0.78548,
            "unit": "ms",
            "range": "±1.83%",
            "extra": "p95 1.16 ms | 1,228.68 ops/sec | 1176 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.128208,
            "unit": "ms",
            "range": "±1.83%",
            "extra": "p95 1.66 ms | 861.33 ops/sec | 830 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.749875,
            "unit": "ms",
            "range": "±3.85%",
            "extra": "p95 4.27 ms | 535.73 ops/sec | 483 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.324042,
            "unit": "ms",
            "range": "±3.41%",
            "extra": "p95 1.26 ms | 2,815.19 ops/sec | 2287 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.522458,
            "unit": "ms",
            "range": "±1.49%",
            "extra": "p95 794.04 µs | 1,863.23 ops/sec | 1783 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.533,
            "unit": "ms",
            "range": "±2.71%",
            "extra": "p95 1.63 ms | 1,748.7 ops/sec | 1495 samples"
          },
          {
            "name": "page",
            "value": 0.213833,
            "unit": "ms",
            "range": "±1.88%",
            "extra": "p95 684.33 µs | 4,229.57 ops/sec | 3609 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.146791,
            "unit": "ms",
            "range": "±1.77%",
            "extra": "p95 2.94 ms | 456.42 ops/sec | 446 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 2.1825,
            "unit": "ms",
            "range": "±2.64%",
            "extra": "p95 3.86 ms | 438.68 ops/sec | 419 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.716042,
            "unit": "ms",
            "range": "±1.52%",
            "extra": "p95 1.05 ms | 1,356.72 ops/sec | 1305 samples"
          },
          {
            "name": "typography",
            "value": 1.081542,
            "unit": "ms",
            "range": "±1.07%",
            "extra": "p95 1.35 ms | 911.08 ops/sec | 898 samples"
          },
          {
            "name": "use-memo",
            "value": 0.219959,
            "unit": "ms",
            "range": "±1.57%",
            "extra": "p95 448.42 µs | 4,211.44 ops/sec | 3828 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.235854,
            "unit": "ms",
            "range": "±1.14%",
            "extra": "p95 411 µs | 3,995.55 ops/sec | 3770 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.2395,
            "unit": "ms",
            "range": "±1.44%",
            "extra": "p95 554.13 µs | 3,973.85 ops/sec | 3634 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.611542,
            "unit": "ms",
            "range": "±3.92%",
            "extra": "p95 10.75 ms | 169.34 ops/sec | 163 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.241291,
            "unit": "ms",
            "range": "±4.27%",
            "extra": "p95 903.37 µs | 3,795.16 ops/sec | 2994 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 4.4095,
            "unit": "ms",
            "range": "±4.65%",
            "extra": "p95 6.79 ms | 218.34 ops/sec | 209 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2702.081334,
            "unit": "ms",
            "range": "±3.80%",
            "extra": "p95 3.04 s | 0.36 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 17.838937,
            "unit": "ms",
            "range": "±5.91%",
            "extra": "p95 24.12 ms | 55.53 ops/sec | 28 samples"
          }
        ],
        "release": {
          "ref": "0.18.6-rc.1",
          "candidateVersion": "0.18.6-rc.1",
          "previousVersion": "0.18.5",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "293176452e30f81d39f9efa663d9846c7664dcb4223f16344d07077d7ab0a415"
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
            "value": 0.363333,
            "unit": "ms",
            "range": "±1.95%",
            "extra": "p95 790.88 µs | 2,544.91 ops/sec | 2303 samples"
          },
          {
            "name": "card",
            "value": 0.136916,
            "unit": "ms",
            "range": "±0.83%",
            "extra": "p95 205.71 µs | 6,920.74 ops/sec | 6627 samples"
          },
          {
            "name": "consts",
            "value": 0.074916,
            "unit": "ms",
            "range": "±0.97%",
            "extra": "p95 129.83 µs | 12,424.05 ops/sec | 11513 samples"
          },
          {
            "name": "counter",
            "value": 0.849125,
            "unit": "ms",
            "range": "±2.19%",
            "extra": "p95 1.53 ms | 1,134.57 ops/sec | 1053 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 1.002938,
            "unit": "ms",
            "range": "±1.41%",
            "extra": "p95 1.52 ms | 944.52 ops/sec | 914 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.635792,
            "unit": "ms",
            "range": "±1.41%",
            "extra": "p95 2.14 ms | 612 ops/sec | 597 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.3475,
            "unit": "ms",
            "range": "±1.16%",
            "extra": "p95 597.42 µs | 2,758.21 ops/sec | 2601 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.549,
            "unit": "ms",
            "range": "±1.41%",
            "extra": "p95 892.83 µs | 1,757.89 ops/sec | 1669 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.470854,
            "unit": "ms",
            "range": "±1.61%",
            "extra": "p95 820.92 µs | 2,027.07 ops/sec | 1906 samples"
          },
          {
            "name": "page",
            "value": 0.207708,
            "unit": "ms",
            "range": "±1.29%",
            "extra": "p95 401.83 µs | 4,498.15 ops/sec | 4174 samples"
          },
          {
            "name": "page-tsx",
            "value": 2.216459,
            "unit": "ms",
            "range": "±3.53%",
            "extra": "p95 3.35 ms | 435.93 ops/sec | 411 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.879479,
            "unit": "ms",
            "range": "±2.06%",
            "extra": "p95 2.98 ms | 513 ops/sec | 496 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.721938,
            "unit": "ms",
            "range": "±1.33%",
            "extra": "p95 1.1 ms | 1,341.98 ops/sec | 1290 samples"
          },
          {
            "name": "typography",
            "value": 1.3385,
            "unit": "ms",
            "range": "±2.62%",
            "extra": "p95 2.36 ms | 741.4 ops/sec | 685 samples"
          },
          {
            "name": "use-memo",
            "value": 0.21475,
            "unit": "ms",
            "range": "±1.39%",
            "extra": "p95 460.33 µs | 4,281.98 ops/sec | 3896 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.235666,
            "unit": "ms",
            "range": "±0.99%",
            "extra": "p95 396.33 µs | 4,043.67 ops/sec | 3850 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.226583,
            "unit": "ms",
            "range": "±0.77%",
            "extra": "p95 368.37 µs | 4,226.17 ops/sec | 4069 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.676125,
            "unit": "ms",
            "range": "±2.38%",
            "extra": "p95 7.5 ms | 178.14 ops/sec | 175 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.222792,
            "unit": "ms",
            "range": "±0.83%",
            "extra": "p95 331.62 µs | 4,307.96 ops/sec | 4153 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 4.25975,
            "unit": "ms",
            "range": "±1.76%",
            "extra": "p95 5.13 ms | 238.75 ops/sec | 235 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2632.055562,
            "unit": "ms",
            "range": "±5.46%",
            "extra": "p95 2.94 s | 0.39 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 17.104729,
            "unit": "ms",
            "range": "±8.57%",
            "extra": "p95 27.44 ms | 57.66 ops/sec | 28 samples"
          }
        ],
        "release": {
          "ref": "0.18.6",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.5",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "2a7b601d5772638973719da1552858b22fedcfcf91730a4690b4f9a0f5e5c01a"
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
            "value": 0.249042,
            "unit": "ms",
            "range": "±2.93%",
            "extra": "p95 933.92 µs | 3,747.14 ops/sec | 3023 samples"
          },
          {
            "name": "card",
            "value": 0.093708,
            "unit": "ms",
            "range": "±0.81%",
            "extra": "p95 137.71 µs | 10,233.68 ops/sec | 9702 samples"
          },
          {
            "name": "consts",
            "value": 0.067041,
            "unit": "ms",
            "range": "±1.81%",
            "extra": "p95 205.87 µs | 13,743.38 ops/sec | 11852 samples"
          },
          {
            "name": "counter",
            "value": 0.380709,
            "unit": "ms",
            "range": "±2.35%",
            "extra": "p95 619.04 µs | 2,515.77 ops/sec | 2301 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.463417,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 580.54 µs | 2,102.87 ops/sec | 2084 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.16348,
            "unit": "ms",
            "range": "±1.83%",
            "extra": "p95 1.91 ms | 820.74 ops/sec | 794 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.214875,
            "unit": "ms",
            "range": "±1.23%",
            "extra": "p95 301.08 µs | 4,463.19 ops/sec | 4245 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.349125,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 390.08 µs | 2,816.29 ops/sec | 2802 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.345167,
            "unit": "ms",
            "range": "±1.82%",
            "extra": "p95 592.88 µs | 2,726.78 ops/sec | 2535 samples"
          },
          {
            "name": "page",
            "value": 0.12075,
            "unit": "ms",
            "range": "±2.49%",
            "extra": "p95 535.92 µs | 7,378.83 ops/sec | 5625 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.095709,
            "unit": "ms",
            "range": "±1.29%",
            "extra": "p95 1.46 ms | 892.25 ops/sec | 876 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.068209,
            "unit": "ms",
            "range": "±1.31%",
            "extra": "p95 1.42 ms | 913.16 ops/sec | 894 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.327625,
            "unit": "ms",
            "range": "±1.19%",
            "extra": "p95 415.92 µs | 2,958.31 ops/sec | 2875 samples"
          },
          {
            "name": "typography",
            "value": 0.521,
            "unit": "ms",
            "range": "±0.75%",
            "extra": "p95 658.29 µs | 1,861.65 ops/sec | 1837 samples"
          },
          {
            "name": "use-memo",
            "value": 0.121416,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 147.54 µs | 7,979.16 ops/sec | 7796 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.158042,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 193.33 µs | 6,129.45 ops/sec | 6008 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.155583,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 183.46 µs | 6,264.73 ops/sec | 6159 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 3.178896,
            "unit": "ms",
            "range": "±0.92%",
            "extra": "p95 3.67 ms | 311.09 ops/sec | 310 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.140917,
            "unit": "ms",
            "range": "±1.58%",
            "extra": "p95 362.54 µs | 6,851.47 ops/sec | 6019 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 1.956375,
            "unit": "ms",
            "range": "±1.21%",
            "extra": "p95 2.56 ms | 492.71 ops/sec | 486 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 9.248562,
            "unit": "ms",
            "range": "±1.70%",
            "extra": "p95 11.3 ms | 105.59 ops/sec | 106 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 1611.919687,
            "unit": "ms",
            "range": "±8.18%",
            "extra": "p95 2.09 s | 0.59 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 9.791813,
            "unit": "ms",
            "range": "±5.79%",
            "extra": "p95 15.39 ms | 97.74 ops/sec | 48 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 0.739667,
            "unit": "ms",
            "range": "±0.84%",
            "extra": "p95 976.63 µs | 1,324.11 ops/sec | 1303 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 0.939187,
            "unit": "ms",
            "range": "±1.74%",
            "extra": "p95 1.48 ms | 1,026.42 ops/sec | 982 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.336875,
            "unit": "ms",
            "range": "±0.71%",
            "extra": "p95 460.54 µs | 2,882.9 ops/sec | 2823 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.431208,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 559.54 µs | 2,263.6 ops/sec | 2228 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.334375,
            "unit": "ms",
            "range": "±1.50%",
            "extra": "p95 498.54 µs | 2,887.19 ops/sec | 2744 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.4235,
            "unit": "ms",
            "range": "±0.94%",
            "extra": "p95 536.38 µs | 2,313.38 ops/sec | 2261 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.06902,
            "unit": "ms",
            "range": "±2.49%",
            "extra": "p95 1.4 ms | 906.78 ops/sec | 864 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.423375,
            "unit": "ms",
            "range": "±1.64%",
            "extra": "p95 620.46 µs | 2,289.5 ops/sec | 2169 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.214167,
            "unit": "ms",
            "range": "±2.10%",
            "extra": "p95 702.38 µs | 4,271.44 ops/sec | 3650 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.291895,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 386.75 µs | 3,323.71 ops/sec | 3262 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.133917,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 157.79 µs | 7,289.71 ops/sec | 7223 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.209959,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 247.21 µs | 4,623.76 ops/sec | 4575 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.226458,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 266.79 µs | 4,295.73 ops/sec | 4261 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.352292,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 423.88 µs | 2,795.2 ops/sec | 2761 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.250458,
            "unit": "ms",
            "range": "±0.37%",
            "extra": "p95 296.12 µs | 3,889.02 ops/sec | 3857 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.380833,
            "unit": "ms",
            "range": "±1.02%",
            "extra": "p95 563.25 µs | 2,514.77 ops/sec | 2430 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.135708,
            "unit": "ms",
            "range": "±1.08%",
            "extra": "p95 188.71 µs | 7,052.76 ops/sec | 6654 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.272125,
            "unit": "ms",
            "range": "±2.64%",
            "extra": "p95 921.54 µs | 3,374.59 ops/sec | 2809 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.095833,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 119.12 µs | 10,284.96 ops/sec | 10059 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.450813,
            "unit": "ms",
            "range": "±0.90%",
            "extra": "p95 568.38 µs | 2,162.42 ops/sec | 2118 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.219875,
            "unit": "ms",
            "range": "±0.98%",
            "extra": "p95 282.5 µs | 4,414.42 ops/sec | 4283 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.397917,
            "unit": "ms",
            "range": "±3.41%",
            "extra": "p95 1.11 ms | 2,344.31 ops/sec | 2030 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.381562,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 441.5 µs | 2,581.64 ops/sec | 2548 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.237834,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 310.5 µs | 4,087.86 ops/sec | 4011 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.269375,
            "unit": "ms",
            "range": "±0.68%",
            "extra": "p95 365.75 µs | 3,605.02 ops/sec | 3532 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.231167,
            "unit": "ms",
            "range": "±0.95%",
            "extra": "p95 313.96 µs | 4,181.12 ops/sec | 4052 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.407605,
            "unit": "ms",
            "range": "±3.63%",
            "extra": "p95 1.3 ms | 2,252.4 ops/sec | 1894 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.371583,
            "unit": "ms",
            "range": "±1.14%",
            "extra": "p95 543.71 µs | 2,602.46 ops/sec | 2511 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.236791,
            "unit": "ms",
            "range": "±1.77%",
            "extra": "p95 368.5 µs | 4,041.87 ops/sec | 3851 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.401,
            "unit": "ms",
            "range": "±6.72%",
            "extra": "p95 540.17 µs | 2,431.96 ops/sec | 2302 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.228625,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 291.79 µs | 4,250.89 ops/sec | 4167 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.245333,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 306.67 µs | 3,989.59 ops/sec | 3926 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.322146,
            "unit": "ms",
            "range": "±0.86%",
            "extra": "p95 430.46 µs | 3,013.07 ops/sec | 2944 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.139292,
            "unit": "ms",
            "range": "±0.47%",
            "extra": "p95 170.13 µs | 6,977.66 ops/sec | 6858 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.23552,
            "unit": "ms",
            "range": "±1.13%",
            "extra": "p95 295.92 µs | 4,127.29 ops/sec | 4010 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.26975,
            "unit": "ms",
            "range": "±2.74%",
            "extra": "p95 1.03 ms | 3,357.46 ops/sec | 2703 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.258125,
            "unit": "ms",
            "range": "±0.85%",
            "extra": "p95 332.96 µs | 3,799.42 ops/sec | 3696 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.305354,
            "unit": "ms",
            "range": "±0.69%",
            "extra": "p95 395.71 µs | 3,230.88 ops/sec | 3160 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.414417,
            "unit": "ms",
            "range": "±1.62%",
            "extra": "p95 706.92 µs | 2,304.58 ops/sec | 2175 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 3.846708,
            "unit": "ms",
            "range": "±2.28%",
            "extra": "p95 5.33 ms | 254.08 ops/sec | 249 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 4.195541,
            "unit": "ms",
            "range": "±1.67%",
            "extra": "p95 4.93 ms | 234.37 ops/sec | 233 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.327396,
            "unit": "ms",
            "range": "±1.96%",
            "extra": "p95 818.08 µs | 2,878.44 ops/sec | 2636 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "2432cf3a38e0ad5d37703d3fe42d4901adde88f9da75de75a1917a9cd86aa856"
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
            "value": 0.234458,
            "unit": "ms",
            "range": "±2.39%",
            "extra": "p95 499.37 µs | 4,034.85 ops/sec | 3549 samples"
          },
          {
            "name": "card",
            "value": 0.088792,
            "unit": "ms",
            "range": "±1.74%",
            "extra": "p95 201.33 µs | 10,227.12 ops/sec | 8929 samples"
          },
          {
            "name": "consts",
            "value": 0.279459,
            "unit": "ms",
            "range": "±2.15%",
            "extra": "p95 666.79 µs | 5,089.01 ops/sec | 3254 samples"
          },
          {
            "name": "counter",
            "value": 2.035479,
            "unit": "ms",
            "range": "±2.92%",
            "extra": "p95 2.92 ms | 611.22 ops/sec | 510 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 2.316292,
            "unit": "ms",
            "range": "±3.02%",
            "extra": "p95 3.46 ms | 507 ops/sec | 439 samples"
          },
          {
            "name": "global-tokens",
            "value": 5.258417,
            "unit": "ms",
            "range": "±3.04%",
            "extra": "p95 7.49 ms | 192.28 ops/sec | 185 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 1.188396,
            "unit": "ms",
            "range": "±3.37%",
            "extra": "p95 2.38 ms | 1,062.78 ops/sec | 812 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 1.459667,
            "unit": "ms",
            "range": "±3.08%",
            "extra": "p95 2.49 ms | 814.15 ops/sec | 675 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.373438,
            "unit": "ms",
            "range": "±3.76%",
            "extra": "p95 1.18 ms | 2,490.72 ops/sec | 2004 samples"
          },
          {
            "name": "page",
            "value": 0.116625,
            "unit": "ms",
            "range": "±0.85%",
            "extra": "p95 191.67 µs | 8,176.07 ops/sec | 7757 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.321896,
            "unit": "ms",
            "range": "±1.73%",
            "extra": "p95 1.97 ms | 760.27 ops/sec | 724 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.375333,
            "unit": "ms",
            "range": "±1.76%",
            "extra": "p95 1.98 ms | 746.45 ops/sec | 710 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.366187,
            "unit": "ms",
            "range": "±1.41%",
            "extra": "p95 682.12 µs | 2,587.75 ops/sec | 2382 samples"
          },
          {
            "name": "typography",
            "value": 0.633688,
            "unit": "ms",
            "range": "±3.28%",
            "extra": "p95 1.47 ms | 1,514.57 ops/sec | 1294 samples"
          },
          {
            "name": "use-memo",
            "value": 0.135459,
            "unit": "ms",
            "range": "±2.68%",
            "extra": "p95 269.08 µs | 7,046.54 ops/sec | 6196 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.185271,
            "unit": "ms",
            "range": "±1.00%",
            "extra": "p95 345.75 µs | 5,097.78 ops/sec | 4772 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.175167,
            "unit": "ms",
            "range": "±1.60%",
            "extra": "p95 421.58 µs | 5,297.3 ops/sec | 4679 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 5.313187,
            "unit": "ms",
            "range": "±6.22%",
            "extra": "p95 11.31 ms | 183.8 ops/sec | 162 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.1325,
            "unit": "ms",
            "range": "±1.47%",
            "extra": "p95 293.17 µs | 7,068.39 ops/sec | 6294 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.70075,
            "unit": "ms",
            "range": "±2.74%",
            "extra": "p95 4.18 ms | 370.89 ops/sec | 353 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 9.462938,
            "unit": "ms",
            "range": "±1.62%",
            "extra": "p95 10.56 ms | 107.96 ops/sec | 108 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2108.473125,
            "unit": "ms",
            "range": "±10.20%",
            "extra": "p95 2.58 s | 0.48 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 12.408333,
            "unit": "ms",
            "range": "±4.34%",
            "extra": "p95 15.5 ms | 79.9 ops/sec | 40 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.086021,
            "unit": "ms",
            "range": "±2.01%",
            "extra": "p95 1.8 ms | 918.19 ops/sec | 860 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.276375,
            "unit": "ms",
            "range": "±1.86%",
            "extra": "p95 1.91 ms | 788.17 ops/sec | 749 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.489334,
            "unit": "ms",
            "range": "±1.87%",
            "extra": "p95 981.37 µs | 1,990.18 ops/sec | 1794 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.724917,
            "unit": "ms",
            "range": "±2.34%",
            "extra": "p95 1.55 ms | 1,363.08 ops/sec | 1222 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.538167,
            "unit": "ms",
            "range": "±3.85%",
            "extra": "p95 1.97 ms | 1,713.71 ops/sec | 1258 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.613833,
            "unit": "ms",
            "range": "±3.55%",
            "extra": "p95 1.88 ms | 1,546.57 ops/sec | 1255 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.627771,
            "unit": "ms",
            "range": "±2.94%",
            "extra": "p95 3.25 ms | 605.68 ops/sec | 552 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.646875,
            "unit": "ms",
            "range": "±3.89%",
            "extra": "p95 1.84 ms | 1,489.28 ops/sec | 1181 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.288959,
            "unit": "ms",
            "range": "±2.61%",
            "extra": "p95 1.01 ms | 3,180.57 ops/sec | 2501 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.410083,
            "unit": "ms",
            "range": "±2.44%",
            "extra": "p95 1.07 ms | 2,302.87 ops/sec | 1958 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.158916,
            "unit": "ms",
            "range": "±1.60%",
            "extra": "p95 401.54 µs | 5,740.55 ops/sec | 4992 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.281625,
            "unit": "ms",
            "range": "±1.61%",
            "extra": "p95 601.25 µs | 3,342.9 ops/sec | 2996 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.291875,
            "unit": "ms",
            "range": "±1.46%",
            "extra": "p95 530.62 µs | 3,281.76 ops/sec | 3038 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.531417,
            "unit": "ms",
            "range": "±2.68%",
            "extra": "p95 1.44 ms | 1,806.01 ops/sec | 1531 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.325125,
            "unit": "ms",
            "range": "±1.60%",
            "extra": "p95 606.83 µs | 2,932.17 ops/sec | 2713 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.451833,
            "unit": "ms",
            "range": "±1.39%",
            "extra": "p95 755.96 µs | 2,154.36 ops/sec | 2019 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.144041,
            "unit": "ms",
            "range": "±0.95%",
            "extra": "p95 279.33 µs | 6,467.3 ops/sec | 6040 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.244041,
            "unit": "ms",
            "range": "±0.89%",
            "extra": "p95 409.29 µs | 3,969.94 ops/sec | 3791 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.085959,
            "unit": "ms",
            "range": "±1.05%",
            "extra": "p95 120.42 µs | 11,015.99 ops/sec | 10636 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.442625,
            "unit": "ms",
            "range": "±1.12%",
            "extra": "p95 683.29 µs | 2,174.88 ops/sec | 2095 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.238875,
            "unit": "ms",
            "range": "±2.24%",
            "extra": "p95 421.21 µs | 4,061.05 ops/sec | 3727 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.409958,
            "unit": "ms",
            "range": "±2.67%",
            "extra": "p95 753.58 µs | 2,339.9 ops/sec | 2101 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.449875,
            "unit": "ms",
            "range": "±2.54%",
            "extra": "p95 1.05 ms | 2,104.14 ops/sec | 1841 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.274541,
            "unit": "ms",
            "range": "±1.15%",
            "extra": "p95 472.96 µs | 3,455.87 ops/sec | 3263 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.298563,
            "unit": "ms",
            "range": "±0.83%",
            "extra": "p95 433.08 µs | 3,273.11 ops/sec | 3166 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.237,
            "unit": "ms",
            "range": "±0.87%",
            "extra": "p95 347.13 µs | 4,097 ops/sec | 3959 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.344542,
            "unit": "ms",
            "range": "±1.50%",
            "extra": "p95 680.62 µs | 2,702.43 ops/sec | 2520 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.346291,
            "unit": "ms",
            "range": "±1.90%",
            "extra": "p95 749.5 µs | 2,641.09 ops/sec | 2390 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.222208,
            "unit": "ms",
            "range": "±0.94%",
            "extra": "p95 369.25 µs | 4,229.7 ops/sec | 4048 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.358063,
            "unit": "ms",
            "range": "±1.26%",
            "extra": "p95 640.63 µs | 2,604.7 ops/sec | 2474 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.232416,
            "unit": "ms",
            "range": "±0.79%",
            "extra": "p95 351.96 µs | 4,203.92 ops/sec | 4063 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.246,
            "unit": "ms",
            "range": "±2.49%",
            "extra": "p95 647.13 µs | 3,745.1 ops/sec | 3189 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.352917,
            "unit": "ms",
            "range": "±2.59%",
            "extra": "p95 924.42 µs | 2,588.37 ops/sec | 2211 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.145875,
            "unit": "ms",
            "range": "±1.03%",
            "extra": "p95 294.88 µs | 6,384.67 ops/sec | 5914 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.289708,
            "unit": "ms",
            "range": "±1.56%",
            "extra": "p95 611.54 µs | 3,345.96 ops/sec | 2957 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.242146,
            "unit": "ms",
            "range": "±0.95%",
            "extra": "p95 434.96 µs | 3,874.72 ops/sec | 3678 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.240542,
            "unit": "ms",
            "range": "±1.09%",
            "extra": "p95 425.21 µs | 3,947.08 ops/sec | 3730 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.2605,
            "unit": "ms",
            "range": "±0.70%",
            "extra": "p95 351.42 µs | 3,718.03 ops/sec | 3641 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.378708,
            "unit": "ms",
            "range": "±1.29%",
            "extra": "p95 751.5 µs | 2,406.11 ops/sec | 2257 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 3.406959,
            "unit": "ms",
            "range": "±3.53%",
            "extra": "p95 4.54 ms | 284.9 ops/sec | 277 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 4.97325,
            "unit": "ms",
            "range": "±5.43%",
            "extra": "p95 10.02 ms | 197.27 ops/sec | 179 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.334041,
            "unit": "ms",
            "range": "±0.88%",
            "extra": "p95 520.67 µs | 2,854.93 ops/sec | 2756 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "2432cf3a38e0ad5d37703d3fe42d4901adde88f9da75de75a1917a9cd86aa856"
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
            "value": 0.211583,
            "unit": "ms",
            "range": "±89.01%",
            "extra": "p95 1.17 ms | 4,143.27 ops/sec | 1253 samples"
          },
          {
            "name": "card",
            "value": 0.0945,
            "unit": "ms",
            "range": "±1.07%",
            "extra": "p95 195.96 µs | 10,008.39 ops/sec | 9128 samples"
          },
          {
            "name": "consts",
            "value": 0.059,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 76.12 µs | 16,288.52 ops/sec | 16029 samples"
          },
          {
            "name": "counter",
            "value": 0.324334,
            "unit": "ms",
            "range": "±0.76%",
            "extra": "p95 459.63 µs | 2,932.08 ops/sec | 2857 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.585125,
            "unit": "ms",
            "range": "±4.45%",
            "extra": "p95 2.4 ms | 1,553.72 ops/sec | 1234 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.452541,
            "unit": "ms",
            "range": "±4.65%",
            "extra": "p95 4.42 ms | 646.84 ops/sec | 561 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.240458,
            "unit": "ms",
            "range": "±0.65%",
            "extra": "p95 296.33 µs | 4,128.81 ops/sec | 4039 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.390229,
            "unit": "ms",
            "range": "±1.15%",
            "extra": "p95 607.08 µs | 2,479.97 ops/sec | 2382 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.372625,
            "unit": "ms",
            "range": "±1.16%",
            "extra": "p95 623.5 µs | 2,589.6 ops/sec | 2462 samples"
          },
          {
            "name": "page",
            "value": 0.121042,
            "unit": "ms",
            "range": "±1.00%",
            "extra": "p95 153.21 µs | 8,124.71 ops/sec | 7794 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.195187,
            "unit": "ms",
            "range": "±1.80%",
            "extra": "p95 1.56 ms | 835.16 ops/sec | 808 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.133042,
            "unit": "ms",
            "range": "±1.57%",
            "extra": "p95 1.49 ms | 888.84 ops/sec | 863 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.351104,
            "unit": "ms",
            "range": "±1.22%",
            "extra": "p95 414.37 µs | 2,850.53 ops/sec | 2770 samples"
          },
          {
            "name": "typography",
            "value": 0.577584,
            "unit": "ms",
            "range": "±5.06%",
            "extra": "p95 3.01 ms | 1,563.7 ops/sec | 1157 samples"
          },
          {
            "name": "use-memo",
            "value": 0.146458,
            "unit": "ms",
            "range": "±3.39%",
            "extra": "p95 901.25 µs | 5,698.19 ops/sec | 3543 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.179208,
            "unit": "ms",
            "range": "±1.70%",
            "extra": "p95 463.08 µs | 5,126.74 ops/sec | 4483 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.175104,
            "unit": "ms",
            "range": "±3.31%",
            "extra": "p95 305.58 µs | 5,523.5 ops/sec | 4980 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 3.939771,
            "unit": "ms",
            "range": "±4.73%",
            "extra": "p95 7.55 ms | 245.61 ops/sec | 228 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.123417,
            "unit": "ms",
            "range": "±0.75%",
            "extra": "p95 191 µs | 7,969 ops/sec | 7617 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.066896,
            "unit": "ms",
            "range": "±1.54%",
            "extra": "p95 2.66 ms | 477.47 ops/sec | 468 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 8.896791,
            "unit": "ms",
            "range": "±2.32%",
            "extra": "p95 10.35 ms | 111.42 ops/sec | 111 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 1579.182042,
            "unit": "ms",
            "range": "±5.77%",
            "extra": "p95 1.99 s | 0.61 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 9.218833,
            "unit": "ms",
            "range": "±1.55%",
            "extra": "p95 10.05 ms | 108.45 ops/sec | 55 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 0.708042,
            "unit": "ms",
            "range": "±1.92%",
            "extra": "p95 970.29 µs | 1,360.86 ops/sec | 1299 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 0.961833,
            "unit": "ms",
            "range": "±2.07%",
            "extra": "p95 1.57 ms | 1,006.47 ops/sec | 955 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.339875,
            "unit": "ms",
            "range": "±1.60%",
            "extra": "p95 566.29 µs | 2,862.34 ops/sec | 2675 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.48175,
            "unit": "ms",
            "range": "±2.39%",
            "extra": "p95 1.16 ms | 1,976.19 ops/sec | 1790 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.414729,
            "unit": "ms",
            "range": "±3.37%",
            "extra": "p95 1.6 ms | 2,118.23 ops/sec | 1638 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.439604,
            "unit": "ms",
            "range": "±3.07%",
            "extra": "p95 1.33 ms | 2,122.36 ops/sec | 1782 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.181125,
            "unit": "ms",
            "range": "±3.44%",
            "extra": "p95 2.77 ms | 788.59 ops/sec | 710 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.450625,
            "unit": "ms",
            "range": "±2.88%",
            "extra": "p95 1.18 ms | 2,110.55 ops/sec | 1825 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.202083,
            "unit": "ms",
            "range": "±1.55%",
            "extra": "p95 332.88 µs | 4,817.03 ops/sec | 4475 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.289875,
            "unit": "ms",
            "range": "±2.56%",
            "extra": "p95 507.29 µs | 3,335.26 ops/sec | 3037 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.1435,
            "unit": "ms",
            "range": "±1.67%",
            "extra": "p95 266.5 µs | 6,716.71 ops/sec | 6023 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.222875,
            "unit": "ms",
            "range": "±2.10%",
            "extra": "p95 442.67 µs | 4,283.17 ops/sec | 3843 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.240584,
            "unit": "ms",
            "range": "±2.49%",
            "extra": "p95 748.33 µs | 3,801.87 ops/sec | 3172 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.402083,
            "unit": "ms",
            "range": "±3.48%",
            "extra": "p95 1.51 ms | 2,181.62 ops/sec | 1663 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.243145,
            "unit": "ms",
            "range": "±0.81%",
            "extra": "p95 352.71 µs | 3,960.59 ops/sec | 3836 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.344083,
            "unit": "ms",
            "range": "±1.07%",
            "extra": "p95 520.83 µs | 2,793.36 ops/sec | 2691 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.144208,
            "unit": "ms",
            "range": "±1.69%",
            "extra": "p95 376.58 µs | 6,376.61 ops/sec | 5523 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.232667,
            "unit": "ms",
            "range": "±0.88%",
            "extra": "p95 341.79 µs | 4,170.52 ops/sec | 4027 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.088042,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 116.62 µs | 10,783.27 ops/sec | 10394 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.422041,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 530.08 µs | 2,317.62 ops/sec | 2294 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.212417,
            "unit": "ms",
            "range": "±0.86%",
            "extra": "p95 251.21 µs | 4,576.57 ops/sec | 4466 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.360125,
            "unit": "ms",
            "range": "±1.09%",
            "extra": "p95 486.5 µs | 2,689.29 ops/sec | 2607 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.40575,
            "unit": "ms",
            "range": "±2.89%",
            "extra": "p95 1.39 ms | 2,194.71 ops/sec | 1839 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.255166,
            "unit": "ms",
            "range": "±1.53%",
            "extra": "p95 449.37 µs | 3,743.99 ops/sec | 3457 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.263375,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 341.67 µs | 3,693.15 ops/sec | 3624 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.225396,
            "unit": "ms",
            "range": "±0.69%",
            "extra": "p95 267.5 µs | 4,329.18 ops/sec | 4238 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.350395,
            "unit": "ms",
            "range": "±1.70%",
            "extra": "p95 488.46 µs | 2,747.24 ops/sec | 2606 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.352583,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 417.58 µs | 2,781.9 ops/sec | 2746 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.234625,
            "unit": "ms",
            "range": "±0.77%",
            "extra": "p95 348.92 µs | 4,107.75 ops/sec | 3993 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.362125,
            "unit": "ms",
            "range": "±8.23%",
            "extra": "p95 663.79 µs | 2,614.91 ops/sec | 2380 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.216,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 259.92 µs | 4,502.13 ops/sec | 4435 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.259875,
            "unit": "ms",
            "range": "±2.53%",
            "extra": "p95 985.04 µs | 3,463.47 ops/sec | 2847 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.340292,
            "unit": "ms",
            "range": "±2.49%",
            "extra": "p95 616.5 µs | 2,771.96 ops/sec | 2554 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.15075,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 245.75 µs | 6,380.16 ops/sec | 6135 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.222166,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 297.71 µs | 4,348.84 ops/sec | 4275 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.226208,
            "unit": "ms",
            "range": "±2.63%",
            "extra": "p95 704.25 µs | 4,095.01 ops/sec | 3516 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.213875,
            "unit": "ms",
            "range": "±1.79%",
            "extra": "p95 286.92 µs | 4,488.09 ops/sec | 4251 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.262792,
            "unit": "ms",
            "range": "±0.73%",
            "extra": "p95 395 µs | 3,651.51 ops/sec | 3552 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.387125,
            "unit": "ms",
            "range": "±1.02%",
            "extra": "p95 563.42 µs | 2,504.12 ops/sec | 2421 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 3.849271,
            "unit": "ms",
            "range": "±1.53%",
            "extra": "p95 4.64 ms | 256.72 ops/sec | 254 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 4.46325,
            "unit": "ms",
            "range": "±4.20%",
            "extra": "p95 8.55 ms | 211.1 ops/sec | 199 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.315875,
            "unit": "ms",
            "range": "±2.24%",
            "extra": "p95 585.46 µs | 2,968.84 ops/sec | 2673 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "3e40acc3cd55e52956bd93891bad522f2a94d19ab03324f88ab48cbfb59ae0d8"
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
            "value": 0.21025,
            "unit": "ms",
            "range": "±0.74%",
            "extra": "p95 320.71 µs | 4,598.56 ops/sec | 4443 samples"
          },
          {
            "name": "card",
            "value": 0.095292,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 134.46 µs | 10,223.84 ops/sec | 9862 samples"
          },
          {
            "name": "consts",
            "value": 0.065417,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 81.54 µs | 14,677.9 ops/sec | 14224 samples"
          },
          {
            "name": "counter",
            "value": 0.352771,
            "unit": "ms",
            "range": "±0.55%",
            "extra": "p95 485.42 µs | 2,733.54 ops/sec | 2690 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.471209,
            "unit": "ms",
            "range": "±0.65%",
            "extra": "p95 641.08 µs | 2,044.9 ops/sec | 2013 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.210042,
            "unit": "ms",
            "range": "±1.18%",
            "extra": "p95 1.67 ms | 809.16 ops/sec | 793 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.23375,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 331.83 µs | 4,163.94 ops/sec | 4066 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.481249,
            "unit": "ms",
            "range": "±2.18%",
            "extra": "p95 1.02 ms | 2,048.36 ops/sec | 1818 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.487,
            "unit": "ms",
            "range": "±1.55%",
            "extra": "p95 828.63 µs | 2,061.15 ops/sec | 1899 samples"
          },
          {
            "name": "page",
            "value": 0.126458,
            "unit": "ms",
            "range": "±1.04%",
            "extra": "p95 296.37 µs | 7,180.18 ops/sec | 6474 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.497875,
            "unit": "ms",
            "range": "±1.74%",
            "extra": "p95 2.09 ms | 674.45 ops/sec | 647 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.538083,
            "unit": "ms",
            "range": "±3.53%",
            "extra": "p95 3.7 ms | 626 ops/sec | 554 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.430042,
            "unit": "ms",
            "range": "±1.62%",
            "extra": "p95 806.79 µs | 2,252.15 ops/sec | 2059 samples"
          },
          {
            "name": "typography",
            "value": 0.759208,
            "unit": "ms",
            "range": "±2.57%",
            "extra": "p95 1.52 ms | 1,298.03 ops/sec | 1165 samples"
          },
          {
            "name": "use-memo",
            "value": 0.158354,
            "unit": "ms",
            "range": "±2.57%",
            "extra": "p95 660.54 µs | 5,644.8 ops/sec | 4226 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.206625,
            "unit": "ms",
            "range": "±1.85%",
            "extra": "p95 627.92 µs | 4,468.69 ops/sec | 3763 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.191625,
            "unit": "ms",
            "range": "±2.71%",
            "extra": "p95 857.83 µs | 4,410.58 ops/sec | 3103 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 6.135937,
            "unit": "ms",
            "range": "±5.84%",
            "extra": "p95 10.2 ms | 172.27 ops/sec | 156 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.137125,
            "unit": "ms",
            "range": "±2.00%",
            "extra": "p95 498.21 µs | 6,561.91 ops/sec | 5284 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 3.208458,
            "unit": "ms",
            "range": "±3.26%",
            "extra": "p95 5.28 ms | 306.91 ops/sec | 290 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 13.481667,
            "unit": "ms",
            "range": "±2.97%",
            "extra": "p95 17.29 ms | 74.05 ops/sec | 74 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 2167.788188,
            "unit": "ms",
            "range": "±11.58%",
            "extra": "p95 2.87 s | 0.46 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 9.004916,
            "unit": "ms",
            "range": "±5.14%",
            "extra": "p95 11.09 ms | 110.85 ops/sec | 55 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 0.696479,
            "unit": "ms",
            "range": "±1.47%",
            "extra": "p95 1.15 ms | 1,348.85 ops/sec | 1290 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 0.7935,
            "unit": "ms",
            "range": "±1.13%",
            "extra": "p95 1.21 ms | 1,190.74 ops/sec | 1160 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.448792,
            "unit": "ms",
            "range": "±2.48%",
            "extra": "p95 1.13 ms | 2,123.85 ops/sec | 1835 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.462291,
            "unit": "ms",
            "range": "±1.03%",
            "extra": "p95 705.04 µs | 2,090.56 ops/sec | 2013 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.363354,
            "unit": "ms",
            "range": "±1.40%",
            "extra": "p95 621.37 µs | 2,678.6 ops/sec | 2516 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.459375,
            "unit": "ms",
            "range": "±1.60%",
            "extra": "p95 782.67 µs | 2,099.43 ops/sec | 1971 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.149916,
            "unit": "ms",
            "range": "±1.43%",
            "extra": "p95 1.66 ms | 853.81 ops/sec | 829 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.458875,
            "unit": "ms",
            "range": "±1.38%",
            "extra": "p95 704.04 µs | 2,129.84 ops/sec | 2025 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.192917,
            "unit": "ms",
            "range": "±1.12%",
            "extra": "p95 353.42 µs | 4,844.46 ops/sec | 4560 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.30425,
            "unit": "ms",
            "range": "±2.54%",
            "extra": "p95 633.71 µs | 3,051.28 ops/sec | 2729 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.132208,
            "unit": "ms",
            "range": "±0.86%",
            "extra": "p95 228.88 µs | 7,068.25 ops/sec | 6717 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.258042,
            "unit": "ms",
            "range": "±2.33%",
            "extra": "p95 718.88 µs | 3,563.86 ops/sec | 3021 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.247083,
            "unit": "ms",
            "range": "±1.07%",
            "extra": "p95 411.83 µs | 3,842.22 ops/sec | 3647 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.390334,
            "unit": "ms",
            "range": "±2.01%",
            "extra": "p95 814.87 µs | 2,455.46 ops/sec | 2225 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.279542,
            "unit": "ms",
            "range": "±1.19%",
            "extra": "p95 462.71 µs | 3,443.98 ops/sec | 3282 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.355,
            "unit": "ms",
            "range": "±1.48%",
            "extra": "p95 596.33 µs | 2,652.16 ops/sec | 2515 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.131459,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 202.83 µs | 7,207.77 ops/sec | 6953 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.230125,
            "unit": "ms",
            "range": "±1.69%",
            "extra": "p95 507.29 µs | 4,025.39 ops/sec | 3623 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.097458,
            "unit": "ms",
            "range": "±0.75%",
            "extra": "p95 162.87 µs | 9,944.16 ops/sec | 9382 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.540417,
            "unit": "ms",
            "range": "±1.62%",
            "extra": "p95 1.01 ms | 1,785.56 ops/sec | 1655 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.305416,
            "unit": "ms",
            "range": "±3.19%",
            "extra": "p95 1.23 ms | 2,971.89 ops/sec | 2235 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.3815,
            "unit": "ms",
            "range": "±1.62%",
            "extra": "p95 696 µs | 2,470.26 ops/sec | 2293 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.395437,
            "unit": "ms",
            "range": "±2.15%",
            "extra": "p95 790.08 µs | 2,362.66 ops/sec | 2150 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.298896,
            "unit": "ms",
            "range": "±2.17%",
            "extra": "p95 690.54 µs | 3,206.21 ops/sec | 2772 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.344,
            "unit": "ms",
            "range": "±1.44%",
            "extra": "p95 614.46 µs | 2,852.49 ops/sec | 2627 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.24825,
            "unit": "ms",
            "range": "±1.17%",
            "extra": "p95 462.29 µs | 3,803.11 ops/sec | 3557 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.377459,
            "unit": "ms",
            "range": "±0.95%",
            "extra": "p95 606.88 µs | 2,546.77 ops/sec | 2455 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.386708,
            "unit": "ms",
            "range": "±1.34%",
            "extra": "p95 722.25 µs | 2,423 ops/sec | 2266 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.25152,
            "unit": "ms",
            "range": "±1.42%",
            "extra": "p95 481.21 µs | 3,725.85 ops/sec | 3424 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.61475,
            "unit": "ms",
            "range": "±7.41%",
            "extra": "p95 1.89 ms | 1,562.63 ops/sec | 1234 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.227313,
            "unit": "ms",
            "range": "±1.62%",
            "extra": "p95 441.04 µs | 4,088.09 ops/sec | 3764 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.25925,
            "unit": "ms",
            "range": "±1.39%",
            "extra": "p95 498.92 µs | 3,646 ops/sec | 3369 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.360875,
            "unit": "ms",
            "range": "±1.95%",
            "extra": "p95 739.33 µs | 2,633.65 ops/sec | 2393 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.1505,
            "unit": "ms",
            "range": "±1.29%",
            "extra": "p95 296.92 µs | 6,283.25 ops/sec | 5726 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.24375,
            "unit": "ms",
            "range": "±2.52%",
            "extra": "p95 573 µs | 3,835.44 ops/sec | 3317 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.244104,
            "unit": "ms",
            "range": "±1.69%",
            "extra": "p95 460 µs | 3,920.62 ops/sec | 3580 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.228208,
            "unit": "ms",
            "range": "±1.41%",
            "extra": "p95 445.46 µs | 4,086.05 ops/sec | 3773 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.280729,
            "unit": "ms",
            "range": "±1.43%",
            "extra": "p95 508.04 µs | 3,391.96 ops/sec | 3154 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.456166,
            "unit": "ms",
            "range": "±3.48%",
            "extra": "p95 1.75 ms | 1,989.53 ops/sec | 1552 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 4.353833,
            "unit": "ms",
            "range": "±4.93%",
            "extra": "p95 9 ms | 219.42 ops/sec | 201 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 4.741292,
            "unit": "ms",
            "range": "±3.14%",
            "extra": "p95 6.42 ms | 212.65 ops/sec | 205 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.35925,
            "unit": "ms",
            "range": "±1.42%",
            "extra": "p95 635.08 µs | 2,674.64 ops/sec | 2503 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "43c7cb1c76870449dbb8982723cfed1c699d60a7a655e51da6f24c5c04a83883"
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
            "value": 0.203125,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 255.83 µs | 4,835.76 ops/sec | 4766 samples"
          },
          {
            "name": "card",
            "value": 0.103291,
            "unit": "ms",
            "range": "±1.42%",
            "extra": "p95 292.79 µs | 8,737.12 ops/sec | 7441 samples"
          },
          {
            "name": "consts",
            "value": 0.070541,
            "unit": "ms",
            "range": "±1.04%",
            "extra": "p95 186.62 µs | 12,973.06 ops/sec | 11316 samples"
          },
          {
            "name": "counter",
            "value": 0.475188,
            "unit": "ms",
            "range": "±1.45%",
            "extra": "p95 781.54 µs | 2,079.56 ops/sec | 1942 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.653833,
            "unit": "ms",
            "range": "±2.75%",
            "extra": "p95 1.79 ms | 1,459.04 ops/sec | 1267 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.804604,
            "unit": "ms",
            "range": "±3.20%",
            "extra": "p95 3.48 ms | 542.75 ops/sec | 496 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.275375,
            "unit": "ms",
            "range": "±1.40%",
            "extra": "p95 527.83 µs | 3,483.41 ops/sec | 3183 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.504125,
            "unit": "ms",
            "range": "±1.58%",
            "extra": "p95 865.96 µs | 1,977.06 ops/sec | 1827 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.512729,
            "unit": "ms",
            "range": "±2.66%",
            "extra": "p95 1.39 ms | 1,913.37 ops/sec | 1600 samples"
          },
          {
            "name": "page",
            "value": 0.14125,
            "unit": "ms",
            "range": "±1.53%",
            "extra": "p95 346.17 µs | 6,561.27 ops/sec | 5659 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.850604,
            "unit": "ms",
            "range": "±4.06%",
            "extra": "p95 4.34 ms | 534.58 ops/sec | 460 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.777938,
            "unit": "ms",
            "range": "±3.71%",
            "extra": "p95 4.21 ms | 539.33 ops/sec | 482 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.532875,
            "unit": "ms",
            "range": "±2.99%",
            "extra": "p95 1.62 ms | 1,810.19 ops/sec | 1485 samples"
          },
          {
            "name": "typography",
            "value": 0.841167,
            "unit": "ms",
            "range": "±3.02%",
            "extra": "p95 2.05 ms | 1,154.09 ops/sec | 999 samples"
          },
          {
            "name": "use-memo",
            "value": 0.17075,
            "unit": "ms",
            "range": "±3.33%",
            "extra": "p95 856.87 µs | 5,126.15 ops/sec | 3410 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.220084,
            "unit": "ms",
            "range": "±2.09%",
            "extra": "p95 716.5 µs | 4,202.07 ops/sec | 3455 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.182563,
            "unit": "ms",
            "range": "±1.95%",
            "extra": "p95 512.42 µs | 4,984.07 ops/sec | 4232 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 4.74525,
            "unit": "ms",
            "range": "±4.41%",
            "extra": "p95 8.5 ms | 207.89 ops/sec | 193 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.144709,
            "unit": "ms",
            "range": "±1.60%",
            "extra": "p95 429.33 µs | 6,310.86 ops/sec | 5336 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 2.874167,
            "unit": "ms",
            "range": "±1.94%",
            "extra": "p95 3.7 ms | 347.06 ops/sec | 339 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 13.093417,
            "unit": "ms",
            "range": "±3.53%",
            "extra": "p95 17.31 ms | 76.27 ops/sec | 75 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 1945.707541,
            "unit": "ms",
            "range": "±12.72%",
            "extra": "p95 2.75 s | 0.51 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 9.4685,
            "unit": "ms",
            "range": "±5.11%",
            "extra": "p95 13.28 ms | 104.17 ops/sec | 51 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 1.032917,
            "unit": "ms",
            "range": "±2.37%",
            "extra": "p95 1.99 ms | 938.72 ops/sec | 865 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.053625,
            "unit": "ms",
            "range": "±1.68%",
            "extra": "p95 1.57 ms | 927.04 ops/sec | 891 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.347583,
            "unit": "ms",
            "range": "±1.37%",
            "extra": "p95 626.88 µs | 2,706.46 ops/sec | 2541 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.530354,
            "unit": "ms",
            "range": "±2.68%",
            "extra": "p95 1.33 ms | 1,790.34 ops/sec | 1556 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.344167,
            "unit": "ms",
            "range": "±2.82%",
            "extra": "p95 898.75 µs | 2,636.41 ops/sec | 2277 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.435833,
            "unit": "ms",
            "range": "±2.81%",
            "extra": "p95 1.19 ms | 2,083.43 ops/sec | 1782 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.154916,
            "unit": "ms",
            "range": "±3.19%",
            "extra": "p95 2.67 ms | 809.02 ops/sec | 725 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.5195,
            "unit": "ms",
            "range": "±2.82%",
            "extra": "p95 1.5 ms | 1,802.67 ops/sec | 1529 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.22525,
            "unit": "ms",
            "range": "±2.07%",
            "extra": "p95 630.83 µs | 4,028.58 ops/sec | 3427 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.492605,
            "unit": "ms",
            "range": "±3.96%",
            "extra": "p95 1.93 ms | 1,896.19 ops/sec | 1310 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.160125,
            "unit": "ms",
            "range": "±1.87%",
            "extra": "p95 496.33 µs | 5,687.36 ops/sec | 4791 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.253417,
            "unit": "ms",
            "range": "±1.61%",
            "extra": "p95 570.67 µs | 3,689.18 ops/sec | 3319 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.255146,
            "unit": "ms",
            "range": "±1.38%",
            "extra": "p95 533.87 µs | 3,639.6 ops/sec | 3344 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.339958,
            "unit": "ms",
            "range": "±1.36%",
            "extra": "p95 592.17 µs | 2,774.43 ops/sec | 2628 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.273417,
            "unit": "ms",
            "range": "±1.20%",
            "extra": "p95 510.63 µs | 3,410.15 ops/sec | 3199 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.371458,
            "unit": "ms",
            "range": "±0.93%",
            "extra": "p95 561.63 µs | 2,575.19 ops/sec | 2498 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.140459,
            "unit": "ms",
            "range": "±1.42%",
            "extra": "p95 250.58 µs | 6,721.12 ops/sec | 6231 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.264125,
            "unit": "ms",
            "range": "±2.58%",
            "extra": "p95 890.42 µs | 3,427.51 ops/sec | 2806 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.116459,
            "unit": "ms",
            "range": "±3.09%",
            "extra": "p95 667.79 µs | 7,237.41 ops/sec | 4381 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.495208,
            "unit": "ms",
            "range": "±1.98%",
            "extra": "p95 989.83 µs | 1,911.05 ops/sec | 1742 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.237,
            "unit": "ms",
            "range": "±2.34%",
            "extra": "p95 464.25 µs | 3,947.86 ops/sec | 3583 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.377375,
            "unit": "ms",
            "range": "±0.87%",
            "extra": "p95 556.96 µs | 2,535.15 ops/sec | 2465 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.39877,
            "unit": "ms",
            "range": "±2.27%",
            "extra": "p95 929.46 µs | 2,315.9 ops/sec | 2064 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.245042,
            "unit": "ms",
            "range": "±1.00%",
            "extra": "p95 403.75 µs | 3,876.78 ops/sec | 3702 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.303167,
            "unit": "ms",
            "range": "±1.69%",
            "extra": "p95 698.17 µs | 3,054.79 ops/sec | 2743 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.236396,
            "unit": "ms",
            "range": "±1.22%",
            "extra": "p95 452.71 µs | 3,968.95 ops/sec | 3706 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.383291,
            "unit": "ms",
            "range": "±1.53%",
            "extra": "p95 717.33 µs | 2,468.56 ops/sec | 2304 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.46425,
            "unit": "ms",
            "range": "±3.54%",
            "extra": "p95 1.8 ms | 1,950.23 ops/sec | 1519 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.265062,
            "unit": "ms",
            "range": "±1.18%",
            "extra": "p95 467.63 µs | 3,582.58 ops/sec | 3352 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.431355,
            "unit": "ms",
            "range": "±1.35%",
            "extra": "p95 745.25 µs | 2,213.94 ops/sec | 2086 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.236125,
            "unit": "ms",
            "range": "±1.50%",
            "extra": "p95 489.33 µs | 3,879.86 ops/sec | 3537 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.23925,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 363.54 µs | 3,980.99 ops/sec | 3858 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.32025,
            "unit": "ms",
            "range": "±0.78%",
            "extra": "p95 455.33 µs | 3,004.83 ops/sec | 2930 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.13925,
            "unit": "ms",
            "range": "±0.74%",
            "extra": "p95 204.21 µs | 6,841.29 ops/sec | 6607 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.240583,
            "unit": "ms",
            "range": "±1.44%",
            "extra": "p95 501.42 µs | 3,861.03 ops/sec | 3539 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.261833,
            "unit": "ms",
            "range": "±1.91%",
            "extra": "p95 562.71 µs | 3,592.07 ops/sec | 3202 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.281042,
            "unit": "ms",
            "range": "±3.03%",
            "extra": "p95 1.2 ms | 3,169.37 ops/sec | 2439 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.312916,
            "unit": "ms",
            "range": "±2.68%",
            "extra": "p95 787.13 µs | 3,013.63 ops/sec | 2610 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.513917,
            "unit": "ms",
            "range": "±3.75%",
            "extra": "p95 1.53 ms | 1,807.39 ops/sec | 1479 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 4.394459,
            "unit": "ms",
            "range": "±3.16%",
            "extra": "p95 5.96 ms | 222.08 ops/sec | 216 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 4.728354,
            "unit": "ms",
            "range": "±1.81%",
            "extra": "p95 6.03 ms | 206.64 ops/sec | 204 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.3895,
            "unit": "ms",
            "range": "±1.12%",
            "extra": "p95 624.29 µs | 2,505.02 ops/sec | 2382 samples"
          }
        ],
        "release": {
          "ref": "feat_split-transform-crate",
          "candidateVersion": "0.18.6",
          "previousVersion": "0.18.6",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "a96fefaef6d444adb6c7edbe6e431a22e4599728fd2be9e37d2917712a1a7c84"
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
          "id": "0b203aba05058db79e47689141749eb991ff7ae2",
          "message": "Bump version to 0.19.0-rc.1",
          "timestamp": "2026-09-06T16:58:39Z",
          "tree_id": "f09f5f55ebad1b93e68849bc7815d7720b912750",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/0b203aba05058db79e47689141749eb991ff7ae2"
        },
        "date": 1788716250315,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.189458,
            "unit": "ms",
            "range": "±0.96%",
            "extra": "p95 282.67 µs | 5,100.09 ops/sec | 4885 samples"
          },
          {
            "name": "card",
            "value": 0.085459,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 106.63 µs | 11,553.02 ops/sec | 11194 samples"
          },
          {
            "name": "consts",
            "value": 0.064792,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 87.13 µs | 15,091.3 ops/sec | 14392 samples"
          },
          {
            "name": "counter",
            "value": 0.319125,
            "unit": "ms",
            "range": "±1.46%",
            "extra": "p95 503.63 µs | 2,910.67 ops/sec | 2763 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.474,
            "unit": "ms",
            "range": "±0.77%",
            "extra": "p95 647.92 µs | 2,083.1 ops/sec | 2039 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.112625,
            "unit": "ms",
            "range": "±1.82%",
            "extra": "p95 1.82 ms | 860.41 ops/sec | 829 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.20875,
            "unit": "ms",
            "range": "±0.85%",
            "extra": "p95 335.46 µs | 4,571.11 ops/sec | 4380 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.345708,
            "unit": "ms",
            "range": "±0.82%",
            "extra": "p95 503.67 µs | 2,792.62 ops/sec | 2713 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.331375,
            "unit": "ms",
            "range": "±0.93%",
            "extra": "p95 532 µs | 2,913.07 ops/sec | 2794 samples"
          },
          {
            "name": "page",
            "value": 0.114375,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 181.71 µs | 8,346.92 ops/sec | 8006 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.107417,
            "unit": "ms",
            "range": "±1.55%",
            "extra": "p95 1.64 ms | 898 ops/sec | 863 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.325625,
            "unit": "ms",
            "range": "±2.78%",
            "extra": "p95 2.72 ms | 741.58 ops/sec | 679 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.356854,
            "unit": "ms",
            "range": "±2.25%",
            "extra": "p95 886.33 µs | 2,647.43 ops/sec | 2358 samples"
          },
          {
            "name": "typography",
            "value": 0.557459,
            "unit": "ms",
            "range": "±1.49%",
            "extra": "p95 865 µs | 1,729.17 ops/sec | 1646 samples"
          },
          {
            "name": "use-memo",
            "value": 0.137625,
            "unit": "ms",
            "range": "±3.57%",
            "extra": "p95 245.67 µs | 7,045.73 ops/sec | 6139 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.157833,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 212.25 µs | 6,126.51 ops/sec | 5973 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.146833,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 166.71 µs | 6,671.75 ops/sec | 6632 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 2.998459,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 3.42 ms | 329.36 ops/sec | 329 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.111292,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 143.29 µs | 8,658.78 ops/sec | 8472 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 1.844709,
            "unit": "ms",
            "range": "±1.16%",
            "extra": "p95 2.16 ms | 529.6 ops/sec | 525 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 9.439167,
            "unit": "ms",
            "range": "±6.51%",
            "extra": "p95 18.52 ms | 97.82 ops/sec | 93 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 1665.580896,
            "unit": "ms",
            "range": "±4.33%",
            "extra": "p95 1.79 s | 0.61 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 8.891021,
            "unit": "ms",
            "range": "±4.16%",
            "extra": "p95 11.38 ms | 109.25 ops/sec | 54 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 0.766209,
            "unit": "ms",
            "range": "±1.29%",
            "extra": "p95 1.15 ms | 1,278.06 ops/sec | 1233 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 1.066917,
            "unit": "ms",
            "range": "±3.34%",
            "extra": "p95 2.6 ms | 885.44 ops/sec | 783 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.404292,
            "unit": "ms",
            "range": "±1.34%",
            "extra": "p95 679.71 µs | 2,392.84 ops/sec | 2260 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.695125,
            "unit": "ms",
            "range": "±2.12%",
            "extra": "p95 1.26 ms | 1,433.57 ops/sec | 1312 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.422208,
            "unit": "ms",
            "range": "±2.60%",
            "extra": "p95 1.01 ms | 2,238.23 ops/sec | 1925 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.455959,
            "unit": "ms",
            "range": "±1.42%",
            "extra": "p95 725.33 µs | 2,097.5 ops/sec | 1996 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.200791,
            "unit": "ms",
            "range": "±1.89%",
            "extra": "p95 1.78 ms | 816.48 ops/sec | 782 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.446813,
            "unit": "ms",
            "range": "±1.12%",
            "extra": "p95 628.42 µs | 2,185.12 ops/sec | 2112 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.195791,
            "unit": "ms",
            "range": "±0.76%",
            "extra": "p95 263.5 µs | 4,953.17 ops/sec | 4825 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.297541,
            "unit": "ms",
            "range": "±1.76%",
            "extra": "p95 463.04 µs | 3,225.91 ops/sec | 3041 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.199,
            "unit": "ms",
            "range": "±2.56%",
            "extra": "p95 736.13 µs | 4,771.25 ops/sec | 3638 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.244188,
            "unit": "ms",
            "range": "±2.39%",
            "extra": "p95 601.83 µs | 3,807.43 ops/sec | 3272 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.24575,
            "unit": "ms",
            "range": "±1.86%",
            "extra": "p95 367.5 µs | 3,909.67 ops/sec | 3689 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.377416,
            "unit": "ms",
            "range": "±2.07%",
            "extra": "p95 893.62 µs | 2,447.12 ops/sec | 2186 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.25475,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 338.54 µs | 3,858.17 ops/sec | 3776 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.3455,
            "unit": "ms",
            "range": "±0.86%",
            "extra": "p95 513.5 µs | 2,794.78 ops/sec | 2711 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.130917,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 194.83 µs | 7,347.11 ops/sec | 7062 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.216125,
            "unit": "ms",
            "range": "±1.87%",
            "extra": "p95 351.88 µs | 4,455.69 ops/sec | 4218 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.08298,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 105.5 µs | 11,699.61 ops/sec | 11448 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.46625,
            "unit": "ms",
            "range": "±2.74%",
            "extra": "p95 1.31 ms | 2,000.09 ops/sec | 1755 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.208083,
            "unit": "ms",
            "range": "±3.18%",
            "extra": "p95 297.08 µs | 4,650.59 ops/sec | 4319 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.37802,
            "unit": "ms",
            "range": "±0.73%",
            "extra": "p95 544.83 µs | 2,589.27 ops/sec | 2526 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.323896,
            "unit": "ms",
            "range": "±0.89%",
            "extra": "p95 473.46 µs | 2,914.93 ops/sec | 2836 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.220396,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 287.5 µs | 4,441.41 ops/sec | 4350 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.247666,
            "unit": "ms",
            "range": "±0.68%",
            "extra": "p95 364.83 µs | 3,891.14 ops/sec | 3780 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.228334,
            "unit": "ms",
            "range": "±1.61%",
            "extra": "p95 497.83 µs | 4,122.38 ops/sec | 3710 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.392041,
            "unit": "ms",
            "range": "±2.97%",
            "extra": "p95 1.02 ms | 2,433.23 ops/sec | 2061 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.357583,
            "unit": "ms",
            "range": "±1.05%",
            "extra": "p95 566.21 µs | 2,706.66 ops/sec | 2598 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.264417,
            "unit": "ms",
            "range": "±2.26%",
            "extra": "p95 559.08 µs | 3,621.3 ops/sec | 3165 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.415875,
            "unit": "ms",
            "range": "±2.45%",
            "extra": "p95 874.63 µs | 2,290.65 ops/sec | 2041 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.22825,
            "unit": "ms",
            "range": "±1.39%",
            "extra": "p95 315.46 µs | 4,262.77 ops/sec | 4029 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.226854,
            "unit": "ms",
            "range": "±1.04%",
            "extra": "p95 304.37 µs | 4,293.01 ops/sec | 4134 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.300209,
            "unit": "ms",
            "range": "±1.04%",
            "extra": "p95 379.21 µs | 3,249.02 ops/sec | 3157 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.154708,
            "unit": "ms",
            "range": "±4.01%",
            "extra": "p95 350.25 µs | 6,129.91 ops/sec | 5046 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.231084,
            "unit": "ms",
            "range": "±1.88%",
            "extra": "p95 365.33 µs | 4,170.08 ops/sec | 3844 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.218542,
            "unit": "ms",
            "range": "±0.95%",
            "extra": "p95 301.08 µs | 4,411.52 ops/sec | 4261 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.222833,
            "unit": "ms",
            "range": "±1.96%",
            "extra": "p95 361.37 µs | 4,280.82 ops/sec | 3989 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.283791,
            "unit": "ms",
            "range": "±1.81%",
            "extra": "p95 494.21 µs | 3,406.27 ops/sec | 3111 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.462792,
            "unit": "ms",
            "range": "±3.29%",
            "extra": "p95 1.43 ms | 1,976.88 ops/sec | 1659 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 4.316042,
            "unit": "ms",
            "range": "±3.58%",
            "extra": "p95 7.54 ms | 223.28 ops/sec | 213 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 4.854521,
            "unit": "ms",
            "range": "±5.49%",
            "extra": "p95 10.02 ms | 199.03 ops/sec | 182 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.401604,
            "unit": "ms",
            "range": "±3.17%",
            "extra": "p95 1.25 ms | 2,262.95 ops/sec | 1888 samples"
          }
        ],
        "release": {
          "ref": "0.19.0-rc.1",
          "candidateVersion": "0.19.0-rc.1",
          "previousVersion": "0.18.6",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "ac79aefb37786860e62f8a760a3cbef2a16a4ad8f7f4274b6f3a494074e1e550"
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
          "id": "540a775cc17766c385c35643972be27d2e91e437",
          "message": "Bump version to 0.19.0-rc.2",
          "timestamp": "2026-09-08T18:50:45Z",
          "tree_id": "6d9101e52c16e158f8249ad15c6b6172009033d4",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/540a775cc17766c385c35643972be27d2e91e437"
        },
        "date": 1788896340456,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.187209,
            "unit": "ms",
            "range": "±1.04%",
            "extra": "p95 255.42 µs | 5,141.68 ops/sec | 5037 samples"
          },
          {
            "name": "card",
            "value": 0.085375,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 120.58 µs | 11,261.32 ops/sec | 11060 samples"
          },
          {
            "name": "consts",
            "value": 0.062417,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 97.46 µs | 15,134.34 ops/sec | 14687 samples"
          },
          {
            "name": "counter",
            "value": 0.389312,
            "unit": "ms",
            "range": "±1.13%",
            "extra": "p95 644 µs | 2,500.41 ops/sec | 2380 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.417958,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 548.54 µs | 2,374.47 ops/sec | 2323 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.23125,
            "unit": "ms",
            "range": "±1.08%",
            "extra": "p95 1.51 ms | 800.3 ops/sec | 787 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.22602,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 316.08 µs | 4,277.07 ops/sec | 4160 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.370937,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 470.67 µs | 2,660.8 ops/sec | 2620 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.322458,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 420.58 µs | 3,009.69 ops/sec | 2970 samples"
          },
          {
            "name": "page",
            "value": 0.1175,
            "unit": "ms",
            "range": "±0.65%",
            "extra": "p95 195.5 µs | 8,032.93 ops/sec | 7644 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.067521,
            "unit": "ms",
            "range": "±1.16%",
            "extra": "p95 1.47 ms | 926.26 ops/sec | 904 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 4.101271,
            "unit": "ms",
            "range": "±2.78%",
            "extra": "p95 5.53 ms | 257.5 ops/sec | 244 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.492604,
            "unit": "ms",
            "range": "±3.76%",
            "extra": "p95 1.85 ms | 1,967.92 ops/sec | 1394 samples"
          },
          {
            "name": "typography",
            "value": 0.605042,
            "unit": "ms",
            "range": "±1.97%",
            "extra": "p95 1.15 ms | 1,601.74 ops/sec | 1481 samples"
          },
          {
            "name": "use-memo",
            "value": 0.131667,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 192.54 µs | 7,303.72 ops/sec | 7075 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.169875,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 238.67 µs | 5,688.51 ops/sec | 5551 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.174125,
            "unit": "ms",
            "range": "±1.32%",
            "extra": "p95 400.25 µs | 5,405.06 ops/sec | 4862 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 3.230666,
            "unit": "ms",
            "range": "±0.90%",
            "extra": "p95 3.77 ms | 309.09 ops/sec | 308 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.109666,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 188.54 µs | 8,512.82 ops/sec | 8153 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 1.092062,
            "unit": "ms",
            "range": "±0.73%",
            "extra": "p95 1.29 ms | 912.59 ops/sec | 904 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 9.436271,
            "unit": "ms",
            "range": "±1.55%",
            "extra": "p95 11.1 ms | 104.06 ops/sec | 104 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 1350.263729,
            "unit": "ms",
            "range": "±3.38%",
            "extra": "p95 1.49 s | 0.73 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 5.961583,
            "unit": "ms",
            "range": "±2.94%",
            "extra": "p95 6.57 ms | 168.26 ops/sec | 84 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 0.484979,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 608.38 µs | 2,033.51 ops/sec | 2008 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 0.685917,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 838.08 µs | 1,445.53 ops/sec | 1429 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.312291,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 422 µs | 3,092.19 ops/sec | 3041 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.529167,
            "unit": "ms",
            "range": "±0.96%",
            "extra": "p95 752.5 µs | 1,890.26 ops/sec | 1821 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.325271,
            "unit": "ms",
            "range": "±0.46%",
            "extra": "p95 426.29 µs | 2,979.61 ops/sec | 2942 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.424125,
            "unit": "ms",
            "range": "±0.67%",
            "extra": "p95 582 µs | 2,281.1 ops/sec | 2237 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 0.939542,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 1.13 ms | 1,040.04 ops/sec | 1033 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.436542,
            "unit": "ms",
            "range": "±0.75%",
            "extra": "p95 586.79 µs | 2,261.59 ops/sec | 2213 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.205375,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 316.58 µs | 4,639.99 ops/sec | 4484 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.301479,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 439.12 µs | 3,183.48 ops/sec | 3112 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.1405,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 233.38 µs | 6,687.52 ops/sec | 6412 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.264583,
            "unit": "ms",
            "range": "±1.17%",
            "extra": "p95 499.46 µs | 3,649.47 ops/sec | 3362 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.231167,
            "unit": "ms",
            "range": "±0.65%",
            "extra": "p95 343.04 µs | 4,186.75 ops/sec | 4051 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.34898,
            "unit": "ms",
            "range": "±0.63%",
            "extra": "p95 481.38 µs | 2,808.3 ops/sec | 2742 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.257145,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 368.38 µs | 3,764.63 ops/sec | 3662 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.394042,
            "unit": "ms",
            "range": "±0.93%",
            "extra": "p95 566.42 µs | 2,481.05 ops/sec | 2397 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.152,
            "unit": "ms",
            "range": "±1.64%",
            "extra": "p95 440.21 µs | 6,017.22 ops/sec | 5144 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.231292,
            "unit": "ms",
            "range": "±0.59%",
            "extra": "p95 347.33 µs | 4,133.29 ops/sec | 4021 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.086708,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 137.96 µs | 10,864.87 ops/sec | 10527 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.437854,
            "unit": "ms",
            "range": "±1.02%",
            "extra": "p95 671 µs | 2,184.2 ops/sec | 2104 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.265583,
            "unit": "ms",
            "range": "±0.85%",
            "extra": "p95 425.21 µs | 3,657.72 ops/sec | 3480 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.368208,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 532.58 µs | 2,611.84 ops/sec | 2543 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.304167,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 410.75 µs | 3,176.63 ops/sec | 3130 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.219646,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 306.58 µs | 4,423.46 ops/sec | 4314 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.24148,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 337.08 µs | 4,003.46 ops/sec | 3920 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.20975,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 319.63 µs | 4,581.84 ops/sec | 4475 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.32873,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 414.42 µs | 2,949.99 ops/sec | 2922 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.320855,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 447.71 µs | 2,995.22 ops/sec | 2948 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.233125,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 351.25 µs | 4,171.46 ops/sec | 4026 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.337667,
            "unit": "ms",
            "range": "±0.56%",
            "extra": "p95 472.71 µs | 2,832.16 ops/sec | 2786 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.20175,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 267.33 µs | 4,774.06 ops/sec | 4709 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.208125,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 281.25 µs | 4,629.43 ops/sec | 4560 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.28575,
            "unit": "ms",
            "range": "±0.42%",
            "extra": "p95 375.04 µs | 3,371.85 ops/sec | 3335 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.128166,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 192.79 µs | 7,402.54 ops/sec | 7225 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.21027,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 306.58 µs | 4,544.76 ops/sec | 4440 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.20025,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 286.33 µs | 4,784.39 ops/sec | 4695 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.201334,
            "unit": "ms",
            "range": "±0.47%",
            "extra": "p95 285.38 µs | 4,767.49 ops/sec | 4681 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.309855,
            "unit": "ms",
            "range": "±1.07%",
            "extra": "p95 509.71 µs | 3,130.6 ops/sec | 2954 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.393334,
            "unit": "ms",
            "range": "±0.69%",
            "extra": "p95 553.83 µs | 2,444.68 ops/sec | 2391 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 2.135625,
            "unit": "ms",
            "range": "±0.99%",
            "extra": "p95 2.57 ms | 461.98 ops/sec | 458 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 2.609021,
            "unit": "ms",
            "range": "±0.75%",
            "extra": "p95 2.92 ms | 380.86 ops/sec | 380 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.3105,
            "unit": "ms",
            "range": "±0.54%",
            "extra": "p95 431.79 µs | 3,087.76 ops/sec | 3031 samples"
          }
        ],
        "release": {
          "ref": "0.19.0-rc.2",
          "candidateVersion": "0.19.0-rc.2",
          "previousVersion": "0.18.6",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "7d9b3538457555b539e2c6394032b052eae38eb052960429054a625b0feee759"
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
          "id": "bcd300ba56c2db6213f13a243b305e1007235d3b",
          "message": "feat(rs-compiler/benchmark): measure each subject in its own process",
          "timestamp": "2026-09-11T01:27:20+03:00",
          "tree_id": "9bacb7c6612c08e161f541e130bb83fddc0f8775",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/bcd300ba56c2db6213f13a243b305e1007235d3b"
        },
        "date": 1789162488843,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.248708,
            "unit": "ms",
            "range": "±2.42%",
            "extra": "p95 526.54 µs | 3,814.01 ops/sec | 3345 samples"
          },
          {
            "name": "card",
            "value": 0.097958,
            "unit": "ms",
            "range": "±1.88%",
            "extra": "p95 380.92 µs | 9,084.35 ops/sec | 7199 samples"
          },
          {
            "name": "consts",
            "value": 0.070875,
            "unit": "ms",
            "range": "±2.48%",
            "extra": "p95 357.75 µs | 12,378.6 ops/sec | 8754 samples"
          },
          {
            "name": "counter",
            "value": 0.521229,
            "unit": "ms",
            "range": "±2.88%",
            "extra": "p95 1.41 ms | 1,902.6 ops/sec | 1576 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.676375,
            "unit": "ms",
            "range": "±2.17%",
            "extra": "p95 1.29 ms | 1,481.17 ops/sec | 1335 samples"
          },
          {
            "name": "global-tokens",
            "value": 2.094188,
            "unit": "ms",
            "range": "±4.03%",
            "extra": "p95 4.53 ms | 464.85 ops/sec | 418 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.312626,
            "unit": "ms",
            "range": "±2.50%",
            "extra": "p95 948.21 µs | 3,086.62 ops/sec | 2534 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.527021,
            "unit": "ms",
            "range": "±1.82%",
            "extra": "p95 888.25 µs | 1,895.55 ops/sec | 1736 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.48125,
            "unit": "ms",
            "range": "±2.95%",
            "extra": "p95 1.46 ms | 1,997.21 ops/sec | 1639 samples"
          },
          {
            "name": "page",
            "value": 0.133958,
            "unit": "ms",
            "range": "±2.35%",
            "extra": "p95 621.5 µs | 6,625.81 ops/sec | 4872 samples"
          },
          {
            "name": "page-tsx",
            "value": 1.487771,
            "unit": "ms",
            "range": "±4.35%",
            "extra": "p95 3.68 ms | 671.13 ops/sec | 566 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 1.455,
            "unit": "ms",
            "range": "±4.49%",
            "extra": "p95 4.09 ms | 645.02 ops/sec | 534 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.435792,
            "unit": "ms",
            "range": "±2.88%",
            "extra": "p95 1.38 ms | 2,159.23 ops/sec | 1734 samples"
          },
          {
            "name": "typography",
            "value": 0.823417,
            "unit": "ms",
            "range": "±2.77%",
            "extra": "p95 1.94 ms | 1,209.05 ops/sec | 1033 samples"
          },
          {
            "name": "use-memo",
            "value": 0.178188,
            "unit": "ms",
            "range": "±1.88%",
            "extra": "p95 532.21 µs | 5,243.51 ops/sec | 4300 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.191042,
            "unit": "ms",
            "range": "±1.56%",
            "extra": "p95 444.5 µs | 4,872.89 ops/sec | 4314 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.181042,
            "unit": "ms",
            "range": "±1.45%",
            "extra": "p95 431.87 µs | 5,066.67 ops/sec | 4505 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 3.676229,
            "unit": "ms",
            "range": "±3.03%",
            "extra": "p95 5.77 ms | 268.46 ops/sec | 258 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.127166,
            "unit": "ms",
            "range": "±1.64%",
            "extra": "p95 323.88 µs | 7,281.85 ops/sec | 6324 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 1.221438,
            "unit": "ms",
            "range": "±2.13%",
            "extra": "p95 1.97 ms | 800.33 ops/sec | 754 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 13.023,
            "unit": "ms",
            "range": "±4.89%",
            "extra": "p95 21.69 ms | 75.41 ops/sec | 73 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 1553.190458,
            "unit": "ms",
            "range": "±5.16%",
            "extra": "p95 1.7 s | 0.65 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 5.847834,
            "unit": "ms",
            "range": "±5.70%",
            "extra": "p95 9.21 ms | 167.83 ops/sec | 81 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 0.463458,
            "unit": "ms",
            "range": "±1.62%",
            "extra": "p95 802.71 µs | 2,022.42 ops/sec | 1907 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 0.884271,
            "unit": "ms",
            "range": "±3.19%",
            "extra": "p95 2.36 ms | 1,091.53 ops/sec | 948 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.308042,
            "unit": "ms",
            "range": "±1.24%",
            "extra": "p95 526.5 µs | 3,070.63 ops/sec | 2909 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.427146,
            "unit": "ms",
            "range": "±0.92%",
            "extra": "p95 651.08 µs | 2,241.17 ops/sec | 2176 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.312666,
            "unit": "ms",
            "range": "±1.11%",
            "extra": "p95 455 µs | 3,049.77 ops/sec | 2939 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.435125,
            "unit": "ms",
            "range": "±1.31%",
            "extra": "p95 708.83 µs | 2,178.17 ops/sec | 2079 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 1.015,
            "unit": "ms",
            "range": "±1.53%",
            "extra": "p95 1.49 ms | 962.82 ops/sec | 928 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.551917,
            "unit": "ms",
            "range": "±9.23%",
            "extra": "p95 1.97 ms | 1,671.62 ops/sec | 1177 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.216417,
            "unit": "ms",
            "range": "±3.76%",
            "extra": "p95 446.33 µs | 4,345 ops/sec | 3721 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.398396,
            "unit": "ms",
            "range": "±3.64%",
            "extra": "p95 1.44 ms | 2,340.48 ops/sec | 1848 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.165354,
            "unit": "ms",
            "range": "±4.25%",
            "extra": "p95 699.46 µs | 5,622.67 ops/sec | 4332 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.268625,
            "unit": "ms",
            "range": "±4.40%",
            "extra": "p95 730 µs | 3,448.07 ops/sec | 2904 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.255,
            "unit": "ms",
            "range": "±2.31%",
            "extra": "p95 713.67 µs | 3,613.05 ops/sec | 3096 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.418208,
            "unit": "ms",
            "range": "±2.13%",
            "extra": "p95 944.71 µs | 2,226.63 ops/sec | 1997 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.321896,
            "unit": "ms",
            "range": "±1.99%",
            "extra": "p95 751.63 µs | 2,916.78 ops/sec | 2586 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.482333,
            "unit": "ms",
            "range": "±1.88%",
            "extra": "p95 994.75 µs | 1,982.33 ops/sec | 1818 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.177416,
            "unit": "ms",
            "range": "±1.69%",
            "extra": "p95 410.08 µs | 5,410.65 ops/sec | 4764 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.302896,
            "unit": "ms",
            "range": "±2.08%",
            "extra": "p95 587.13 µs | 3,161.11 ops/sec | 2860 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.104583,
            "unit": "ms",
            "range": "±1.22%",
            "extra": "p95 206.29 µs | 9,108.01 ops/sec | 8200 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.542583,
            "unit": "ms",
            "range": "±3.55%",
            "extra": "p95 1.73 ms | 1,708.33 ops/sec | 1435 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.228084,
            "unit": "ms",
            "range": "±2.24%",
            "extra": "p95 634.79 µs | 4,045.94 ops/sec | 3469 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.441625,
            "unit": "ms",
            "range": "±2.83%",
            "extra": "p95 1.42 ms | 2,096.65 ops/sec | 1777 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.383,
            "unit": "ms",
            "range": "±3.10%",
            "extra": "p95 1.33 ms | 2,468.74 ops/sec | 2038 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.255333,
            "unit": "ms",
            "range": "±2.06%",
            "extra": "p95 564.54 µs | 3,696.78 ops/sec | 3261 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.271041,
            "unit": "ms",
            "range": "±2.59%",
            "extra": "p95 842.71 µs | 3,412.57 ops/sec | 2876 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.273,
            "unit": "ms",
            "range": "±2.30%",
            "extra": "p95 547.29 µs | 3,572.27 ops/sec | 3153 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.378292,
            "unit": "ms",
            "range": "±2.47%",
            "extra": "p95 764.29 µs | 2,482.56 ops/sec | 2227 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.3735,
            "unit": "ms",
            "range": "±3.28%",
            "extra": "p95 732.21 µs | 2,526.28 ops/sec | 2236 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.257562,
            "unit": "ms",
            "range": "±2.44%",
            "extra": "p95 637.92 µs | 3,635.97 ops/sec | 3122 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.433125,
            "unit": "ms",
            "range": "±2.35%",
            "extra": "p95 893.67 µs | 2,222.45 ops/sec | 2002 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.255208,
            "unit": "ms",
            "range": "±2.39%",
            "extra": "p95 579.17 µs | 3,697.78 ops/sec | 3213 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.281167,
            "unit": "ms",
            "range": "±2.91%",
            "extra": "p95 777.12 µs | 3,329.56 ops/sec | 2741 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.389687,
            "unit": "ms",
            "range": "±2.60%",
            "extra": "p95 952.33 µs | 2,439.48 ops/sec | 2098 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.166958,
            "unit": "ms",
            "range": "±2.14%",
            "extra": "p95 535.17 µs | 5,653.49 ops/sec | 4715 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.27375,
            "unit": "ms",
            "range": "±1.91%",
            "extra": "p95 586.25 µs | 3,510.15 ops/sec | 3118 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.290417,
            "unit": "ms",
            "range": "±2.29%",
            "extra": "p95 809.75 µs | 3,211.56 ops/sec | 2731 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.300563,
            "unit": "ms",
            "range": "±2.95%",
            "extra": "p95 1.14 ms | 3,129.63 ops/sec | 2362 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.400667,
            "unit": "ms",
            "range": "±2.62%",
            "extra": "p95 1.2 ms | 2,435.99 ops/sec | 1987 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.567667,
            "unit": "ms",
            "range": "±2.29%",
            "extra": "p95 1.14 ms | 1,829.9 ops/sec | 1582 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 3.470896,
            "unit": "ms",
            "range": "±3.49%",
            "extra": "p95 5.94 ms | 291.74 ops/sec | 270 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 4.330833,
            "unit": "ms",
            "range": "±4.07%",
            "extra": "p95 7.75 ms | 226.14 ops/sec | 211 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.404729,
            "unit": "ms",
            "range": "±2.78%",
            "extra": "p95 1.28 ms | 2,265.31 ops/sec | 1868 samples"
          }
        ],
        "release": {
          "ref": "fix_reprice-the-addon-ci-and-budget",
          "candidateVersion": "0.19.0-rc.2",
          "previousVersion": "0.19.0-rc.2",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "f697f06dc445e9414fe9328b292543edfc2dc1d5a513fea369d26a10f1839559"
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
          "id": "253ca79b93331979e0097338ccab0d9a243fb486",
          "message": "fix(rs-compiler/benchmark): import the reader modules by file URL",
          "timestamp": "2026-09-12T00:32:34+03:00",
          "tree_id": "7db27f3563a28b07a9469b1f46e85ecf14bdff93",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/253ca79b93331979e0097338ccab0d9a243fb486"
        },
        "date": 1789206040017,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.19175,
            "unit": "ms",
            "range": "±1.56%",
            "extra": "p95 313.92 µs | 4,995.56 ops/sec | 4677 samples"
          },
          {
            "name": "card",
            "value": 0.085792,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 109.21 µs | 11,259.17 ops/sec | 10829 samples"
          },
          {
            "name": "consts",
            "value": 0.062333,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 77.79 µs | 15,572.9 ops/sec | 15132 samples"
          },
          {
            "name": "counter",
            "value": 0.319041,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 435.71 µs | 3,065.81 ops/sec | 3002 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.322666,
            "unit": "ms",
            "range": "±0.71%",
            "extra": "p95 514.12 µs | 2,914.37 ops/sec | 2840 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.006666,
            "unit": "ms",
            "range": "±0.94%",
            "extra": "p95 1.28 ms | 962.92 ops/sec | 950 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.224291,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 261.67 µs | 4,529.14 ops/sec | 4443 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.314583,
            "unit": "ms",
            "range": "±0.55%",
            "extra": "p95 421.04 µs | 3,039.98 ops/sec | 2995 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.28625,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 331.54 µs | 3,408.4 ops/sec | 3389 samples"
          },
          {
            "name": "page",
            "value": 0.099458,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 118.21 µs | 9,756.11 ops/sec | 9672 samples"
          },
          {
            "name": "page-tsx",
            "value": 0.900583,
            "unit": "ms",
            "range": "±0.70%",
            "extra": "p95 1.1 ms | 1,104.02 ops/sec | 1091 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 0.817605,
            "unit": "ms",
            "range": "±0.74%",
            "extra": "p95 1.08 ms | 1,184.88 ops/sec | 1170 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.255042,
            "unit": "ms",
            "range": "±1.05%",
            "extra": "p95 483.79 µs | 3,551.98 ops/sec | 3367 samples"
          },
          {
            "name": "typography",
            "value": 0.526979,
            "unit": "ms",
            "range": "±0.82%",
            "extra": "p95 700.5 µs | 1,983.07 ops/sec | 1922 samples"
          },
          {
            "name": "use-memo",
            "value": 0.10525,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 118.42 µs | 9,309.58 ops/sec | 9237 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.142417,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 199.58 µs | 6,634.05 ops/sec | 6502 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.155042,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 194.04 µs | 6,416.3 ops/sec | 6265 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 2.580271,
            "unit": "ms",
            "range": "±4.42%",
            "extra": "p95 3.29 ms | 389.52 ops/sec | 366 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.11175,
            "unit": "ms",
            "range": "±0.77%",
            "extra": "p95 176.12 µs | 8,805.05 ops/sec | 8339 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 1.178583,
            "unit": "ms",
            "range": "±1.98%",
            "extra": "p95 1.84 ms | 831.98 ops/sec | 796 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 10.128125,
            "unit": "ms",
            "range": "±2.47%",
            "extra": "p95 10.82 ms | 105.77 ops/sec | 104 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 1344.055875,
            "unit": "ms",
            "range": "±4.43%",
            "extra": "p95 1.44 s | 0.75 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 4.74175,
            "unit": "ms",
            "range": "±4.84%",
            "extra": "p95 6.44 ms | 198.16 ops/sec | 96 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 0.478625,
            "unit": "ms",
            "range": "±1.21%",
            "extra": "p95 685.04 µs | 2,071.03 ops/sec | 1987 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 0.746417,
            "unit": "ms",
            "range": "±1.38%",
            "extra": "p95 996.17 µs | 1,367.43 ops/sec | 1310 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.278188,
            "unit": "ms",
            "range": "±1.06%",
            "extra": "p95 457.08 µs | 3,397.7 ops/sec | 3254 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.351167,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 396.04 µs | 2,795.55 ops/sec | 2786 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.279459,
            "unit": "ms",
            "range": "±0.83%",
            "extra": "p95 395.25 µs | 3,369.85 ops/sec | 3277 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.411958,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 509.58 µs | 2,463.7 ops/sec | 2412 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 0.930833,
            "unit": "ms",
            "range": "±1.19%",
            "extra": "p95 1.3 ms | 1,040.96 ops/sec | 1014 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.408687,
            "unit": "ms",
            "range": "±0.60%",
            "extra": "p95 491.04 µs | 2,443.93 ops/sec | 2408 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.188083,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 229.58 µs | 5,276.38 ops/sec | 5186 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.300251,
            "unit": "ms",
            "range": "±0.58%",
            "extra": "p95 358.58 µs | 3,296.19 ops/sec | 3242 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.143708,
            "unit": "ms",
            "range": "±1.40%",
            "extra": "p95 276.21 µs | 6,738.76 ops/sec | 6117 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.188083,
            "unit": "ms",
            "range": "±0.74%",
            "extra": "p95 241.54 µs | 5,109.38 ops/sec | 5035 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.207625,
            "unit": "ms",
            "range": "±0.83%",
            "extra": "p95 310.87 µs | 4,823.51 ops/sec | 4627 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.312187,
            "unit": "ms",
            "range": "±0.65%",
            "extra": "p95 416.29 µs | 3,108.75 ops/sec | 3046 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.203042,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 266.25 µs | 4,687.15 ops/sec | 4629 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.314375,
            "unit": "ms",
            "range": "±0.99%",
            "extra": "p95 491 µs | 3,028.43 ops/sec | 2905 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.114834,
            "unit": "ms",
            "range": "±0.16%",
            "extra": "p95 124.96 µs | 8,595.84 ops/sec | 8569 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.234667,
            "unit": "ms",
            "range": "±0.74%",
            "extra": "p95 320.29 µs | 4,206.86 ops/sec | 4083 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.0775,
            "unit": "ms",
            "range": "±0.43%",
            "extra": "p95 100.75 µs | 12,273.23 ops/sec | 12012 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.400125,
            "unit": "ms",
            "range": "±0.73%",
            "extra": "p95 499.25 µs | 2,497.93 ops/sec | 2447 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.220792,
            "unit": "ms",
            "range": "±1.33%",
            "extra": "p95 309.5 µs | 4,458.36 ops/sec | 4251 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.370791,
            "unit": "ms",
            "range": "±0.71%",
            "extra": "p95 490 µs | 2,754.07 ops/sec | 2677 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.287375,
            "unit": "ms",
            "range": "±0.57%",
            "extra": "p95 381.46 µs | 3,380.35 ops/sec | 3309 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.229375,
            "unit": "ms",
            "range": "±1.86%",
            "extra": "p95 453.67 µs | 4,149.36 ops/sec | 3751 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.254687,
            "unit": "ms",
            "range": "±2.37%",
            "extra": "p95 519.58 µs | 3,703.71 ops/sec | 3312 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.213459,
            "unit": "ms",
            "range": "±1.23%",
            "extra": "p95 345.25 µs | 4,578.91 ops/sec | 4307 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.332333,
            "unit": "ms",
            "range": "±1.00%",
            "extra": "p95 464.21 µs | 2,909.73 ops/sec | 2813 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.349,
            "unit": "ms",
            "range": "±2.41%",
            "extra": "p95 633.75 µs | 2,791.2 ops/sec | 2506 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.226687,
            "unit": "ms",
            "range": "±1.24%",
            "extra": "p95 354.58 µs | 4,280.66 ops/sec | 4044 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.369542,
            "unit": "ms",
            "range": "±1.37%",
            "extra": "p95 642.08 µs | 2,588.92 ops/sec | 2427 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.227583,
            "unit": "ms",
            "range": "±4.48%",
            "extra": "p95 359.38 µs | 4,369 ops/sec | 3971 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.220417,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 315.17 µs | 4,467.47 ops/sec | 4328 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.305333,
            "unit": "ms",
            "range": "±1.04%",
            "extra": "p95 456.96 µs | 3,183.74 ops/sec | 3057 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.135333,
            "unit": "ms",
            "range": "±0.55%",
            "extra": "p95 186 µs | 7,212.17 ops/sec | 7023 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.200229,
            "unit": "ms",
            "range": "±0.76%",
            "extra": "p95 267 µs | 4,831.88 ops/sec | 4696 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.248917,
            "unit": "ms",
            "range": "±4.25%",
            "extra": "p95 1.52 ms | 3,459.71 ops/sec | 2321 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.24925,
            "unit": "ms",
            "range": "±3.19%",
            "extra": "p95 919.92 µs | 3,690.71 ops/sec | 2901 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.312833,
            "unit": "ms",
            "range": "±3.18%",
            "extra": "p95 913.5 µs | 2,926.15 ops/sec | 2463 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.4535,
            "unit": "ms",
            "range": "±1.94%",
            "extra": "p95 780.17 µs | 2,089.04 ops/sec | 1955 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 2.146896,
            "unit": "ms",
            "range": "±2.74%",
            "extra": "p95 3.85 ms | 447.52 ops/sec | 426 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 2.784729,
            "unit": "ms",
            "range": "±4.14%",
            "extra": "p95 5.75 ms | 354.89 ops/sec | 322 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.359625,
            "unit": "ms",
            "range": "±1.56%",
            "extra": "p95 700.83 µs | 2,620.84 ops/sec | 2430 samples"
          }
        ],
        "release": {
          "ref": "fix_reprice-the-addon-ci-and-budget",
          "candidateVersion": "0.19.0-rc.2",
          "previousVersion": "0.19.0-rc.2",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "f697f06dc445e9414fe9328b292543edfc2dc1d5a513fea369d26a10f1839559"
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
          "id": "f105006b86fbc176f11e4af36f6a904f401738ac",
          "message": "fix(stylexswc/rs-compiler): keep a base refusal out of the comparison only",
          "timestamp": "2026-09-12T23:12:18+03:00",
          "tree_id": "df71ee4e3962ca0e9f9420c68d12b7a6f11e1ec6",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/f105006b86fbc176f11e4af36f6a904f401738ac"
        },
        "date": 1789247450927,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.18425,
            "unit": "ms",
            "range": "±2.95%",
            "extra": "p95 254.25 µs | 5,201.18 ops/sec | 4981 samples"
          },
          {
            "name": "card",
            "value": 0.082709,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 91.87 µs | 11,953.57 ops/sec | 11870 samples"
          },
          {
            "name": "consts",
            "value": 0.0595,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 67.38 µs | 16,628.71 ops/sec | 16494 samples"
          },
          {
            "name": "counter",
            "value": 0.299791,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 332.08 µs | 3,313.78 ops/sec | 3299 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.353167,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 441.5 µs | 2,743.88 ops/sec | 2715 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.033125,
            "unit": "ms",
            "range": "±0.90%",
            "extra": "p95 1.24 ms | 945.79 ops/sec | 934 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.214937,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 281.29 µs | 4,513.75 ops/sec | 4442 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.321896,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 355.25 µs | 3,064.72 ops/sec | 3052 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.294104,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 329 µs | 3,355.49 ops/sec | 3336 samples"
          },
          {
            "name": "page",
            "value": 0.102875,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 114.21 µs | 9,626.24 ops/sec | 9562 samples"
          },
          {
            "name": "page-tsx",
            "value": 0.838771,
            "unit": "ms",
            "range": "±0.34%",
            "extra": "p95 957.21 µs | 1,183.69 ops/sec | 1180 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 0.822625,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 934.17 µs | 1,204.95 ops/sec | 1202 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.276292,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 300.75 µs | 3,578.51 ops/sec | 3568 samples"
          },
          {
            "name": "typography",
            "value": 0.492833,
            "unit": "ms",
            "range": "±0.88%",
            "extra": "p95 651.25 µs | 1,990.78 ops/sec | 1948 samples"
          },
          {
            "name": "use-memo",
            "value": 0.115709,
            "unit": "ms",
            "range": "±0.66%",
            "extra": "p95 149.71 µs | 8,282.13 ops/sec | 8075 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.1525,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 180.71 µs | 6,393.77 ops/sec | 6340 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.142667,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 155.67 µs | 6,948.88 ops/sec | 6920 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 2.262542,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 2.43 ms | 439.01 ops/sec | 439 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.102,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 113.04 µs | 9,687.83 ops/sec | 9638 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 0.910958,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 1.03 ms | 1,087.1 ops/sec | 1085 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 7.692667,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 8.22 ms | 129.82 ops/sec | 130 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 1215.174416,
            "unit": "ms",
            "range": "±2.90%",
            "extra": "p95 1.32 s | 0.81 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 4.810229,
            "unit": "ms",
            "range": "±1.33%",
            "extra": "p95 5.45 ms | 204.56 ops/sec | 102 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 0.418813,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 479.54 µs | 2,355.9 ops/sec | 2346 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 0.565729,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 638.29 µs | 1,749.22 ops/sec | 1744 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.282666,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 313 µs | 3,504.09 ops/sec | 3491 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.384667,
            "unit": "ms",
            "range": "±0.38%",
            "extra": "p95 464.92 µs | 2,549.69 ops/sec | 2532 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.296125,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 333.38 µs | 3,332.59 ops/sec | 3320 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.39875,
            "unit": "ms",
            "range": "±0.53%",
            "extra": "p95 513.17 µs | 2,456.69 ops/sec | 2423 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 0.887541,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 994.33 µs | 1,114.87 ops/sec | 1113 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.369708,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 462.12 µs | 2,631.72 ops/sec | 2607 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.17725,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 200.75 µs | 5,550.15 ops/sec | 5512 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.270708,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 312.33 µs | 3,642.36 ops/sec | 3620 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.123709,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 138.75 µs | 7,993.15 ops/sec | 7940 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.20075,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 222.96 µs | 4,928.12 ops/sec | 4900 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.193375,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 211.38 µs | 5,130.57 ops/sec | 5105 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.293667,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 384.42 µs | 3,272.81 ops/sec | 3227 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.218334,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 259.25 µs | 4,483.92 ops/sec | 4453 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.336041,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 429.46 µs | 2,896.72 ops/sec | 2863 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.125458,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 139 µs | 7,892.37 ops/sec | 7852 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.207375,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 228.67 µs | 4,770.31 ops/sec | 4744 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.080708,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 88.25 µs | 12,329.1 ops/sec | 12273 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.376333,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 420.87 µs | 2,624.88 ops/sec | 2615 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.198708,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 217.21 µs | 4,971.62 ops/sec | 4953 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.316541,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 353.46 µs | 3,120.87 ops/sec | 3108 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.299083,
            "unit": "ms",
            "range": "±0.47%",
            "extra": "p95 382.08 µs | 3,264 ops/sec | 3221 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.208833,
            "unit": "ms",
            "range": "±0.49%",
            "extra": "p95 277.87 µs | 4,662.29 ops/sec | 4574 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.221792,
            "unit": "ms",
            "range": "±0.61%",
            "extra": "p95 269 µs | 4,388.97 ops/sec | 4340 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.19375,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 213 µs | 5,090.12 ops/sec | 5069 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.320667,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 356.87 µs | 3,077.69 ops/sec | 3063 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.302916,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 341.12 µs | 3,253.64 ops/sec | 3238 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.195458,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 214.33 µs | 5,064.51 ops/sec | 5042 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.321167,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 358.17 µs | 3,080.29 ops/sec | 3065 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.195209,
            "unit": "ms",
            "range": "±0.33%",
            "extra": "p95 234.79 µs | 4,982.13 ops/sec | 4937 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.203875,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 259.04 µs | 4,752.74 ops/sec | 4696 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.297875,
            "unit": "ms",
            "range": "±0.41%",
            "extra": "p95 373.62 µs | 3,273.57 ops/sec | 3238 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.124667,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 137.88 µs | 7,930.54 ops/sec | 7893 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.195917,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 217.33 µs | 5,042.3 ops/sec | 5016 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.191792,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 209.83 µs | 5,164.97 ops/sec | 5137 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.189666,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 209.63 µs | 5,217.93 ops/sec | 5190 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.251666,
            "unit": "ms",
            "range": "±1.07%",
            "extra": "p95 351.08 µs | 3,817.86 ops/sec | 3687 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.353937,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 405.25 µs | 2,779.63 ops/sec | 2758 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 2.081333,
            "unit": "ms",
            "range": "±0.86%",
            "extra": "p95 2.38 ms | 485.32 ops/sec | 481 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 2.250625,
            "unit": "ms",
            "range": "±0.92%",
            "extra": "p95 2.78 ms | 431.56 ops/sec | 429 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.285583,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 360.63 µs | 3,414.19 ops/sec | 3381 samples"
          }
        ],
        "release": {
          "ref": "fix_sx-prop-is-skipped-for-compiled-jsx-shorthand",
          "candidateVersion": "0.19.0-rc.2",
          "previousVersion": "0.19.0-rc.2",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "ffcf5863ae5ab6425edbcf3d9202ea44b2f195ac92ee60840cabb5120c346292"
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
          "id": "a4b520b4552175afe3b80ff6c25ccfa41cb959cd",
          "message": "Bump version to 0.19.0-rc.3",
          "timestamp": "2026-09-12T21:46:06Z",
          "tree_id": "22128e88347270eb711ee3c52e40d190674f7a19",
          "url": "https://github.com/Dwlad90/stylex-swc-plugin/commit/a4b520b4552175afe3b80ff6c25ccfa41cb959cd"
        },
        "date": 1789252262389,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "buttons-demo",
            "value": 0.182833,
            "unit": "ms",
            "range": "±1.18%",
            "extra": "p95 219.25 µs | 5,338.43 ops/sec | 5263 samples"
          },
          {
            "name": "card",
            "value": 0.083292,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 98.29 µs | 11,813.15 ops/sec | 11710 samples"
          },
          {
            "name": "consts",
            "value": 0.064125,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 99.54 µs | 14,653.3 ops/sec | 14011 samples"
          },
          {
            "name": "counter",
            "value": 0.303125,
            "unit": "ms",
            "range": "±0.30%",
            "extra": "p95 346.17 µs | 3,261.3 ops/sec | 3244 samples"
          },
          {
            "name": "counter-with-dynamic-styles",
            "value": 0.363083,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 474.25 µs | 2,672.33 ops/sec | 2635 samples"
          },
          {
            "name": "global-tokens",
            "value": 1.034417,
            "unit": "ms",
            "range": "±1.02%",
            "extra": "p95 1.43 ms | 937.99 ops/sec | 923 samples"
          },
          {
            "name": "global-tokens-xs",
            "value": 0.201792,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 229.92 µs | 4,880.33 ops/sec | 4844 samples"
          },
          {
            "name": "namespace-cleaning",
            "value": 0.326416,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 375.08 µs | 3,023.42 ops/sec | 3009 samples"
          },
          {
            "name": "namespace-cleaning-no-unused",
            "value": 0.297333,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 342.38 µs | 3,320.37 ops/sec | 3305 samples"
          },
          {
            "name": "page",
            "value": 0.103334,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 120.5 µs | 9,511.37 ops/sec | 9413 samples"
          },
          {
            "name": "page-tsx",
            "value": 0.86025,
            "unit": "ms",
            "range": "±0.76%",
            "extra": "p95 1.14 ms | 1,104.81 ops/sec | 1089 samples"
          },
          {
            "name": "page-with-keyframes",
            "value": 0.835312,
            "unit": "ms",
            "range": "±0.46%",
            "extra": "p95 1 ms | 1,165.96 ops/sec | 1160 samples"
          },
          {
            "name": "spot-loader",
            "value": 0.291041,
            "unit": "ms",
            "range": "±0.51%",
            "extra": "p95 374.83 µs | 3,346.13 ops/sec | 3298 samples"
          },
          {
            "name": "typography",
            "value": 0.444083,
            "unit": "ms",
            "range": "±0.80%",
            "extra": "p95 594.25 µs | 2,154.04 ops/sec | 2113 samples"
          },
          {
            "name": "use-memo",
            "value": 0.115083,
            "unit": "ms",
            "range": "±1.02%",
            "extra": "p95 156.29 µs | 8,276.65 ops/sec | 7972 samples"
          },
          {
            "name": "Performance - Colors StyleX transformation",
            "value": 0.150334,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 168.67 µs | 6,557.69 ops/sec | 6512 samples"
          },
          {
            "name": "Performance - Basic theme transformation",
            "value": 0.142875,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 159.79 µs | 6,933.45 ops/sec | 6899 samples"
          },
          {
            "name": "Performance - Complex theme transformation",
            "value": 2.271375,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 2.44 ms | 440.26 ops/sec | 440 samples"
          },
          {
            "name": "Performance - Basic create transformation",
            "value": 0.101709,
            "unit": "ms",
            "range": "±0.21%",
            "extra": "p95 113.12 µs | 9,750.35 ops/sec | 9692 samples"
          },
          {
            "name": "Performance - Complex create transformation",
            "value": 1.068854,
            "unit": "ms",
            "range": "±0.62%",
            "extra": "p95 1.24 ms | 948.69 ops/sec | 940 samples"
          },
          {
            "name": "Debug data - lotsOfStyles.js (100 creates, dev)",
            "value": 7.701479,
            "unit": "ms",
            "range": "±0.72%",
            "extra": "p95 8.23 ms | 130.09 ops/sec | 130 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStyles.js",
            "value": 1220.181833,
            "unit": "ms",
            "range": "±2.99%",
            "extra": "p95 1.38 s | 0.81 ops/sec | 10 samples"
          },
          {
            "name": "Rollup plugin - lotsOfStylesDynamic.js",
            "value": 4.730938,
            "unit": "ms",
            "range": "±0.87%",
            "extra": "p95 5.03 ms | 211.59 ops/sec | 106 samples"
          },
          {
            "name": "Feature - dynamic styles",
            "value": 0.423291,
            "unit": "ms",
            "range": "±0.90%",
            "extra": "p95 483.58 µs | 2,330.48 ops/sec | 2313 samples"
          },
          {
            "name": "Feature - dynamic styles (dev)",
            "value": 0.570833,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 666.5 µs | 1,725.41 ops/sec | 1718 samples"
          },
          {
            "name": "Feature - nested conditions",
            "value": 0.283416,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 319.71 µs | 3,478.19 ops/sec | 3465 samples"
          },
          {
            "name": "Feature - nested conditions (dev)",
            "value": 0.442521,
            "unit": "ms",
            "range": "±0.65%",
            "extra": "p95 581.17 µs | 2,260.24 ops/sec | 2214 samples"
          },
          {
            "name": "Feature - media queries",
            "value": 0.303791,
            "unit": "ms",
            "range": "±0.44%",
            "extra": "p95 400.67 µs | 3,163.73 ops/sec | 3125 samples"
          },
          {
            "name": "Feature - media queries (dev)",
            "value": 0.37475,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 432.12 µs | 2,632.98 ops/sec | 2622 samples"
          },
          {
            "name": "Feature - media query order",
            "value": 0.889583,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 975 µs | 1,114.32 ops/sec | 1113 samples"
          },
          {
            "name": "Feature - media query order off",
            "value": 0.361833,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 418 µs | 2,718.74 ops/sec | 2706 samples"
          },
          {
            "name": "Feature - keyframes and animations",
            "value": 0.177958,
            "unit": "ms",
            "range": "±0.22%",
            "extra": "p95 195.96 µs | 5,548.82 ops/sec | 5523 samples"
          },
          {
            "name": "Feature - keyframes and animations (dev)",
            "value": 0.272416,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 306.5 µs | 3,617.69 ops/sec | 3601 samples"
          },
          {
            "name": "Feature - theme tokens",
            "value": 0.124208,
            "unit": "ms",
            "range": "±0.20%",
            "extra": "p95 138.54 µs | 7,951.64 ops/sec | 7912 samples"
          },
          {
            "name": "Feature - theme tokens (dev)",
            "value": 0.206291,
            "unit": "ms",
            "range": "±0.35%",
            "extra": "p95 267.5 µs | 4,668.15 ops/sec | 4612 samples"
          },
          {
            "name": "Feature - logical and RTL",
            "value": 0.195292,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 219 µs | 5,045.35 ops/sec | 5012 samples"
          },
          {
            "name": "Feature - logical and RTL (dev)",
            "value": 0.308604,
            "unit": "ms",
            "range": "±0.48%",
            "extra": "p95 398.67 µs | 3,155.11 ops/sec | 3112 samples"
          },
          {
            "name": "Feature - CSS value normalization",
            "value": 0.218834,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 244.83 µs | 4,502.18 ops/sec | 4480 samples"
          },
          {
            "name": "Feature - CSS value normalization (dev)",
            "value": 0.318958,
            "unit": "ms",
            "range": "±0.29%",
            "extra": "p95 361.08 µs | 3,087.87 ops/sec | 3073 samples"
          },
          {
            "name": "Feature - view transitions",
            "value": 0.124334,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 138.38 µs | 7,964.96 ops/sec | 7929 samples"
          },
          {
            "name": "Feature - view transitions (dev)",
            "value": 0.2075,
            "unit": "ms",
            "range": "±0.26%",
            "extra": "p95 233.79 µs | 4,765.2 ops/sec | 4739 samples"
          },
          {
            "name": "Feature - token definitions",
            "value": 0.081041,
            "unit": "ms",
            "range": "±0.17%",
            "extra": "p95 90.17 µs | 12,244.5 ops/sec | 12181 samples"
          },
          {
            "name": "Feature - debug class names over many namespaces",
            "value": 0.376459,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 420.04 µs | 2,633.71 ops/sec | 2625 samples"
          },
          {
            "name": "Feature - unminified debug keys",
            "value": 0.233062,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 356.54 µs | 4,163.93 ops/sec | 4040 samples"
          },
          {
            "name": "Feature - source text read from disk",
            "value": 0.329729,
            "unit": "ms",
            "range": "±0.52%",
            "extra": "p95 436.83 µs | 2,941.48 ops/sec | 2898 samples"
          },
          {
            "name": "Feature - legacy shorthand expansion",
            "value": 0.280958,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 317.04 µs | 3,504.89 ops/sec | 3491 samples"
          },
          {
            "name": "Feature - legacy value flipping",
            "value": 0.196958,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 231.33 µs | 4,970.32 ops/sec | 4930 samples"
          },
          {
            "name": "Feature - font size px to rem",
            "value": 0.219,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 246.79 µs | 4,500.05 ops/sec | 4479 samples"
          },
          {
            "name": "Feature - props and attrs",
            "value": 0.195542,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 219.25 µs | 5,043.92 ops/sec | 5019 samples"
          },
          {
            "name": "Feature - props and attrs (dev)",
            "value": 0.322,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 359.04 µs | 3,074.75 ops/sec | 3062 samples"
          },
          {
            "name": "Feature - debug data prop",
            "value": 0.305208,
            "unit": "ms",
            "range": "±0.31%",
            "extra": "p95 343.21 µs | 3,232.71 ops/sec | 3215 samples"
          },
          {
            "name": "Feature - debug without the data prop",
            "value": 0.201459,
            "unit": "ms",
            "range": "±0.36%",
            "extra": "p95 252.58 µs | 4,793.62 ops/sec | 4737 samples"
          },
          {
            "name": "Feature - debug and dev class names",
            "value": 0.324542,
            "unit": "ms",
            "range": "±0.28%",
            "extra": "p95 364.71 µs | 3,052.34 ops/sec | 3038 samples"
          },
          {
            "name": "Feature - inlined conditional merge off",
            "value": 0.204375,
            "unit": "ms",
            "range": "±0.45%",
            "extra": "p95 260.21 µs | 4,733.92 ops/sec | 4659 samples"
          },
          {
            "name": "Feature - runtime injection",
            "value": 0.203083,
            "unit": "ms",
            "range": "±0.25%",
            "extra": "p95 226.25 µs | 4,860.64 ops/sec | 4835 samples"
          },
          {
            "name": "Feature - readable test class names",
            "value": 0.280667,
            "unit": "ms",
            "range": "±0.27%",
            "extra": "p95 321.04 µs | 3,515.08 ops/sec | 3498 samples"
          },
          {
            "name": "Feature - stylex side effects injected",
            "value": 0.12475,
            "unit": "ms",
            "range": "±0.19%",
            "extra": "p95 138.42 µs | 7,954.22 ops/sec | 7916 samples"
          },
          {
            "name": "Feature - source maps inline",
            "value": 0.197125,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 220.96 µs | 5,018.87 ops/sec | 4993 samples"
          },
          {
            "name": "Feature - source maps without columns",
            "value": 0.192792,
            "unit": "ms",
            "range": "±0.24%",
            "extra": "p95 214.63 µs | 5,123.45 ops/sec | 5097 samples"
          },
          {
            "name": "Feature - source maps without source text",
            "value": 0.191167,
            "unit": "ms",
            "range": "±0.32%",
            "extra": "p95 224.13 µs | 5,144.95 ops/sec | 5103 samples"
          },
          {
            "name": "Feature - engine fold",
            "value": 0.311979,
            "unit": "ms",
            "range": "±2.29%",
            "extra": "p95 656.12 µs | 3,102.97 ops/sec | 2726 samples"
          },
          {
            "name": "Feature - engine fold (dev)",
            "value": 0.383709,
            "unit": "ms",
            "range": "±0.64%",
            "extra": "p95 502.33 µs | 2,541.45 ops/sec | 2495 samples"
          },
          {
            "name": "Feature - calls that do not fold",
            "value": 1.853979,
            "unit": "ms",
            "range": "±0.39%",
            "extra": "p95 2.01 ms | 536.19 ops/sec | 536 samples"
          },
          {
            "name": "Feature - calls that do not fold (dev)",
            "value": 2.274416,
            "unit": "ms",
            "range": "±0.50%",
            "extra": "p95 2.56 ms | 433.53 ops/sec | 433 samples"
          },
          {
            "name": "Feature - class name prefix",
            "value": 0.284833,
            "unit": "ms",
            "range": "±0.23%",
            "extra": "p95 323.38 µs | 3,459.06 ops/sec | 3446 samples"
          }
        ],
        "release": {
          "ref": "0.19.0-rc.3",
          "candidateVersion": "0.19.0-rc.3",
          "previousVersion": "0.18.6",
          "target": "aarch64-apple-darwin",
          "nativeSha256": "c04899d917260fbb2a390d45b21cd14d2cde1a75a3d1e326e39832f42c34618d"
        }
      }
    ]
  }
};
