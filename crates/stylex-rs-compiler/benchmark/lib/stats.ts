/**
 * Pure statistical helpers used by the benchmark harness and the verdict
 * engine. Nothing here reads files, spawns processes, or depends on
 * tinybench internals beyond the already-validated shape captured in
 * `extractLatencySamples`.
 *
 * Everything is deterministic given identical inputs. The bootstrap uses a
 * seeded xorshift32 PRNG so that CI results reproduce locally.
 */

import type { Task, TaskResultWithStatistics } from 'tinybench';

import type { BootstrapConfig, RawLatencySamples } from './types.js';

/**
 * Extract validated latency samples from a completed tinybench task.
 *
 * Tinybench 6 exposes retained samples through `result.latency.samples` as
 * a pre-sorted array. It does *not* expose p95 — do not attempt to parse
 * anything out of tinybench's `toString()` or table output. p95 comes from
 * the `quantile` helper below.
 */
export function extractLatencySamples(task: Task): RawLatencySamples {
  if (!task.result || !('throughput' in task.result)) {
    throw new Error(`Benchmark task "${task.name}" produced no results`);
  }

  const result = task.result as TaskResultWithStatistics;
  const samples = result.latency.samples ?? [];

  if (samples.length === 0) {
    throw new Error(`Benchmark task "${task.name}" produced zero samples`);
  }

  const p50 = result.latency.p50;
  const p95 = quantile(samples, 95);

  ensureFinitePositive(task.name, 'p50', p50);
  ensureFinitePositive(task.name, 'p95', p95);

  return {
    samples,
    p50,
    p95,
    rme: result.latency.rme,
    samplesCount: result.latency.samplesCount,
    opsPerSec: result.throughput.mean,
  };
}

/**
 * Nearest-rank percentile over a pre-sorted array of samples.
 *
 * Kept intentionally small and separately tested — the previous single
 * inline copy in `bench.ts` used the same convention and this replaces it.
 */
export function quantile(sortedSamples: readonly number[], percentile: number): number {
  if (sortedSamples.length === 0) return Number.NaN;
  if (percentile <= 0) return sortedSamples[0] ?? Number.NaN;
  if (percentile >= 100) return sortedSamples[sortedSamples.length - 1] ?? Number.NaN;

  const index = Math.min(
    sortedSamples.length - 1,
    Math.ceil((percentile / 100) * sortedSamples.length) - 1
  );
  return sortedSamples[index] ?? Number.NaN;
}

/** Median of a numeric array. Preserves the input; sorts a copy. */
export function median(values: readonly number[]): number {
  if (values.length === 0) return Number.NaN;
  const sorted = values.toSorted((a, b) => a - b);
  const mid = sorted.length >>> 1;
  if (sorted.length % 2 === 0) {
    return ((sorted[mid - 1] ?? Number.NaN) + (sorted[mid] ?? Number.NaN)) / 2;
  }
  return sorted[mid] ?? Number.NaN;
}

/**
 * Per-round ratio of candidate p50 to base p50. Guards against zero and
 * non-finite values so that a broken measurement fails loudly instead of
 * poisoning downstream statistics.
 */
export function roundRatios(
  basePerRound: readonly number[],
  candidatePerRound: readonly number[]
): number[] {
  if (basePerRound.length !== candidatePerRound.length) {
    throw new Error(
      `Round count mismatch: base ${basePerRound.length}, candidate ${candidatePerRound.length}`
    );
  }

  return basePerRound.map((base, index) => {
    const candidate = candidatePerRound[index];
    if (
      base === undefined ||
      candidate === undefined ||
      !Number.isFinite(base) ||
      !Number.isFinite(candidate) ||
      base <= 0 ||
      candidate <= 0
    ) {
      throw new Error(
        `Invalid latency at round ${index}: base=${String(base)}, candidate=${String(candidate)}`
      );
    }
    return candidate / base;
  });
}

/**
 * Seeded 32-bit xorshift PRNG. Deterministic given a fixed seed and pure
 * across platforms. Used only for bootstrap resampling — no cryptographic
 * claim is made.
 */
export function makeSeededRng(seed: number): () => number {
  let state = seed >>> 0;
  if (state === 0) state = 0x9e3779b9;
  return () => {
    state ^= state << 13;
    state >>>= 0;
    state ^= state >>> 17;
    state ^= state << 5;
    state >>>= 0;
    return state / 0x1_0000_0000;
  };
}

export interface BootstrapInterval {
  /** Median of the observed ratios. */
  point: number;
  /** One-sided lower confidence bound. */
  lower: number;
  /** One-sided upper confidence bound. */
  upper: number;
}

/**
 * Deterministic bootstrap of the median of round ratios.
 *
 * Returns both one-sided lower and upper bounds at the requested
 * confidence level; the verdict engine (Phase 3) reads both — the lower
 * bound gates regressions and the upper bound flags improbably-large
 * improvements.
 */
export function bootstrapMedianRatio(
  ratios: readonly number[],
  config: BootstrapConfig
): BootstrapInterval {
  if (ratios.length === 0) {
    throw new Error('bootstrapMedianRatio requires at least one ratio');
  }
  if (config.resamples <= 0) {
    throw new Error('bootstrapMedianRatio requires resamples > 0');
  }
  if (config.confidence <= 0 || config.confidence >= 1) {
    throw new Error('bootstrapMedianRatio requires confidence in (0, 1)');
  }

  const rng = makeSeededRng(config.seed);
  const n = ratios.length;
  const medians = Array.from<number>({ length: config.resamples });

  const buffer = Array.from<number>({ length: n });
  for (let i = 0; i < config.resamples; i++) {
    for (let j = 0; j < n; j++) {
      const pick = Math.floor(rng() * n);
      buffer[j] = ratios[pick] ?? Number.NaN;
    }
    medians[i] = median(buffer);
  }

  const sortedMedians = medians.toSorted((a, b) => a - b);

  const alpha = 1 - config.confidence;
  const lowerIndex = Math.max(0, Math.floor(alpha * config.resamples) - 1);
  const upperIndex = Math.min(config.resamples - 1, Math.ceil((1 - alpha) * config.resamples) - 1);

  return {
    point: median(ratios),
    lower: sortedMedians[lowerIndex] ?? Number.NaN,
    upper: sortedMedians[upperIndex] ?? Number.NaN,
  };
}

/**
 * Guard against non-finite or non-positive latencies. A non-finite median
 * serialises to `null` and `github-action-benchmark` coerces that to 0 ms
 * — recorded as an impossibly fast run, which then makes the *next* run
 * look infinitely slower. Fail loudly here instead of poisoning the
 * historical series.
 */
export function ensureFinitePositive(context: string, field: string, value: number): void {
  if (!Number.isFinite(value) || value <= 0) {
    throw new Error(`${context}: ${field} is not a positive finite number (${String(value)})`);
  }
}
