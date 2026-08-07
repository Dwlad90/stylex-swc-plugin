window.BENCHMARK_DATA = {
  "lastUpdate": 1786132959858,
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
      }
    ]
  }
};
