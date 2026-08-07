import { describe, expect, test } from 'vitest';

import { parseConfidence, parsePositiveFloat, parsePositiveInt } from '../lib/cli.js';

describe('benchmark CLI number parsing', () => {
  test('parses positive finite numbers and safe integers', () => {
    expect(parsePositiveFloat('warn', '1.1')).toBe(1.1);
    expect(parsePositiveInt('rounds', '10')).toBe(10);
  });

  test.each(['0', '-1', '1.5', '1x', 'Infinity'])('rejects %s as a positive integer', value => {
    expect(() => parsePositiveInt('rounds', value)).toThrow(`Invalid --rounds value: ${value}`);
  });

  test('accepts confidence only inside the open unit interval', () => {
    expect(parseConfidence('confidence', '0.95')).toBe(0.95);
    expect(() => parseConfidence('confidence', '1')).toThrow(
      'Invalid --confidence value: 1 (must be in (0, 1))'
    );
  });
});
