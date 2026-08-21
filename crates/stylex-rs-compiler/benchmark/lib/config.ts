/**
 * Shared benchmark configuration.
 *
 * `bench.ts`, `bench-compare.ts`, and `bench-revisions.ts` share these
 * `stylexOptions` and tinybench option blocks, so any divergence in
 * measurement conditions must be an explicit, intentional override.
 */

import type { BenchOptions } from 'tinybench';

import type { StyleXOptions } from '../../dist/index.js';
import type { FixtureDescriptor } from './types.js';

export const DEFAULT_PAIRED_TIME_BUDGET_MS = 300;

/**
 * The shared, production-shaped options every fixture is measured under
 * unless it says otherwise.
 *
 * `dev: false` stays the default here rather than becoming a variant of this
 * function. A `dev` build costs 3-4x a production one on the same file, so
 * flipping it here would move every trend series in the repo onto a shape
 * nobody had been watching, and the two cannot be compared against each other
 * afterwards -- see `guidelines/PERFORMANCE.md`. A fixture that wants the
 * `dev` shape asks for it with `"dev": true` in `fixtures.v1.json`, arriving
 * here through `fixtureStylexOptions`.
 */
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

/**
 * `options` as one fixture is measured under, applying its own `dev` override
 * when it declares one.
 *
 * Used by the runner for both the sanity check and the timed run, so a
 * fixture cannot be validated under one configuration and timed under
 * another.
 */
export function fixtureStylexOptions(
  fixture: Pick<FixtureDescriptor, 'dev'>,
  options: StyleXOptions
): StyleXOptions {
  return fixture.dev === undefined ? options : { ...options, dev: fixture.dev };
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
