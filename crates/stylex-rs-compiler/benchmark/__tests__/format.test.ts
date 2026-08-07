import { describe, expect, test } from 'vitest';

import { formatLatency, markdownTableRow } from '../lib/format.js';

describe('formatLatency', () => {
  test('nanosecond scale', () => {
    expect(formatLatency(0.0000005)).toBe('1 ns');
    expect(formatLatency(0.0005)).toBe('500 ns');
  });

  test('microsecond scale', () => {
    expect(formatLatency(0.001)).toBe('1 µs');
    expect(formatLatency(0.05)).toBe('50 µs');
  });

  test('millisecond scale', () => {
    expect(formatLatency(1)).toBe('1 ms');
    expect(formatLatency(12.345)).toBe('12.35 ms');
  });

  test('second scale', () => {
    expect(formatLatency(1000)).toBe('1 s');
    expect(formatLatency(2500)).toBe('2.5 s');
  });

  test('non-finite input', () => {
    expect(formatLatency(Number.NaN)).toBe('n/a');
    expect(formatLatency(Infinity)).toBe('n/a');
    expect(formatLatency(-Infinity)).toBe('n/a');
  });
});

describe('markdownTableRow', () => {
  test('joins pre-escaped cells into a table row', () => {
    expect(markdownTableRow(['name', 'pass'])).toBe('| name | pass |');
  });
});
