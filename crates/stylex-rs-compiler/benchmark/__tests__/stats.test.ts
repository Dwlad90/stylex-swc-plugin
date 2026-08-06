import { describe, expect, test } from 'vitest';

import {
  bootstrapMedianRatio,
  ensureFinitePositive,
  makeSeededRng,
  median,
  quantile,
  roundRatios,
} from '../lib/stats.js';

describe('quantile', () => {
  test('nearest-rank matches the old inline helper', () => {
    const samples = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    expect(quantile(samples, 50)).toBe(5);
    expect(quantile(samples, 95)).toBe(10);
    expect(quantile(samples, 100)).toBe(10);
  });

  test('returns NaN for empty input', () => {
    expect(quantile([], 50)).toBeNaN();
  });

  test('clamps below zero and above 100', () => {
    expect(quantile([1, 2, 3], 0)).toBe(1);
    expect(quantile([1, 2, 3], 150)).toBe(3);
  });
});

describe('median', () => {
  test('odd-length', () => {
    expect(median([3, 1, 2])).toBe(2);
  });
  test('even-length averages the two middle values', () => {
    expect(median([1, 2, 3, 4])).toBe(2.5);
  });
  test('empty is NaN', () => {
    expect(median([])).toBeNaN();
  });
});

describe('roundRatios', () => {
  test('elementwise candidate/base', () => {
    expect(roundRatios([2, 4, 5], [3, 4, 10])).toStrictEqual([1.5, 1, 2]);
  });

  test('throws on length mismatch', () => {
    expect(() => roundRatios([1, 2], [1])).toThrow(/Round count mismatch/);
  });

  test('throws on zero or non-finite latency', () => {
    expect(() => roundRatios([0, 1], [1, 1])).toThrow(/Invalid latency/);
    expect(() => roundRatios([1, 1], [Infinity, 1])).toThrow(/Invalid latency/);
    expect(() => roundRatios([1, 1], [Number.NaN, 1])).toThrow(/Invalid latency/);
  });
});

describe('makeSeededRng', () => {
  test('is deterministic for a given seed', () => {
    const a = makeSeededRng(42);
    const b = makeSeededRng(42);
    const seqA = Array.from({ length: 5 }, () => a());
    const seqB = Array.from({ length: 5 }, () => b());
    expect(seqA).toStrictEqual(seqB);
  });

  test('produces values in [0, 1)', () => {
    const rng = makeSeededRng(1);
    for (let i = 0; i < 100; i++) {
      const value = rng();
      expect(value).toBeGreaterThanOrEqual(0);
      expect(value).toBeLessThan(1);
    }
  });

  test('avoids the degenerate zero-seed lockup', () => {
    const rng = makeSeededRng(0);
    const first = rng();
    const second = rng();
    expect(first).not.toBe(0);
    expect(second).not.toBe(first);
  });
});

describe('bootstrapMedianRatio', () => {
  test('is deterministic for a given seed', () => {
    const ratios = [1.0, 1.05, 0.98, 1.02, 1.1, 0.99, 1.03, 1.0, 0.97, 1.04];
    const config = { seed: 123, resamples: 500, confidence: 0.95 };
    const first = bootstrapMedianRatio(ratios, config);
    const second = bootstrapMedianRatio(ratios, config);
    expect(second).toStrictEqual(first);
  });

  test('lower bound is at most the point estimate', () => {
    const ratios = [1.0, 1.05, 0.98, 1.02, 1.1, 0.99, 1.03];
    const result = bootstrapMedianRatio(ratios, {
      seed: 7,
      resamples: 500,
      confidence: 0.9,
    });
    expect(result.lower).toBeLessThanOrEqual(result.point);
    expect(result.upper).toBeGreaterThanOrEqual(result.point);
  });

  test('detects a 20% shift with sufficient rounds', () => {
    const base = Array.from({ length: 40 }, () => 1.2);
    const result = bootstrapMedianRatio(base, {
      seed: 3,
      resamples: 500,
      confidence: 0.95,
    });
    expect(result.point).toBeCloseTo(1.2, 5);
    expect(result.lower).toBeGreaterThan(1.1);
  });

  test('rejects invalid configuration', () => {
    expect(() => bootstrapMedianRatio([], { seed: 1, resamples: 10, confidence: 0.9 })).toThrow(
      /at least one ratio/
    );
    expect(() => bootstrapMedianRatio([1], { seed: 1, resamples: 0, confidence: 0.9 })).toThrow(
      /resamples > 0/
    );
    expect(() => bootstrapMedianRatio([1], { seed: 1, resamples: 10, confidence: 1 })).toThrow(
      /confidence in \(0, 1\)/
    );
  });
});

describe('ensureFinitePositive', () => {
  test('accepts positive finite values', () => {
    expect(() => ensureFinitePositive('ctx', 'v', 1)).not.toThrow();
  });

  test('rejects zero, negatives and non-finite values', () => {
    for (const value of [0, -1, Number.NaN, Infinity, -Infinity]) {
      expect(() => ensureFinitePositive('ctx', 'v', value)).toThrow();
    }
  });
});
