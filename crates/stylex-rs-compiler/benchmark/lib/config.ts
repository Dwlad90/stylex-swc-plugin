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
 */
export function createPairedBenchConfigs(timeBudgetMs: number): PairedBenchConfigs {
  return {
    standard: {
      retainSamples: true,
      warmup: true,
      time: timeBudgetMs,
      iterations: 20,
    },
    heavy: {
      retainSamples: true,
      warmup: true,
      time: Math.min(timeBudgetMs, 500),
      iterations: 5,
      warmupIterations: 1,
      warmupTime: 100,
    },
  };
}
