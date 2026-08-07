/**
 * Shared benchmark configuration.
 *
 * `bench.ts`, `bench-compare.ts`, and `bench-revisions.ts` previously
 * carried their own copies of `stylexOptions` and the tinybench option
 * blocks; they are unified here so that any divergence in measurement
 * conditions must be an explicit, intentional override.
 */

import type { BenchOptions } from 'tinybench';

import type { StyleXOptions } from '../../dist/index.js';

export function createStylexOptions(packageDir: string): StyleXOptions {
  return {
    dev: false,
    treeshakeCompensation: true,
    unstable_moduleResolution: {
      type: 'haste',
      rootDir: packageDir,
    },
  };
}

export interface PairedBenchConfigs {
  standard: BenchOptions;
  heavy: BenchOptions;
}

/**
 * Tinybench options used by the paired comparison entry points
 * (`bench:compare`, `bench:revisions`). Kept identical between them so
 * cross-entry-point divergence in measurement conditions is impossible
 * without editing this module.
 *
 * Sampling is deliberately modest *within* a round. The verdict engine
 * bootstraps per-round ratios, so statistical resolution comes from the
 * round count, not from the sample count inside one round. A 1000 ms
 * budget collected ~13,000 samples per round on the fastest fixtures to
 * estimate a single median, which cost ~12 min per paired run and bought
 * nothing the bootstrap could use. Heavy fixtures were worse: a 2 s
 * operation ran `iterations: 5` regardless of the time budget.
 *
 * Do not raise these to "get more samples" — add rounds instead, and
 * record the calibration evidence in guidelines/PERFORMANCE.md.
 */
export function createPairedBenchConfigs(timeBudgetMs: number): PairedBenchConfigs {
  return {
    standard: {
      retainSamples: true,
      warmup: true,
      warmupTime: 100,
      warmupIterations: 8,
      time: timeBudgetMs,
      iterations: 20,
    },
    heavy: {
      retainSamples: true,
      warmup: true,
      time: Math.min(timeBudgetMs, 200),
      iterations: 2,
      warmupIterations: 1,
      warmupTime: 100,
    },
  };
}
