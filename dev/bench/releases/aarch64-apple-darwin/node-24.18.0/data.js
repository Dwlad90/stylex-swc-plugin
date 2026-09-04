window.BENCHMARK_DATA = {
  "lastUpdate": 1788542504368,
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
      }
    ]
  }
};
