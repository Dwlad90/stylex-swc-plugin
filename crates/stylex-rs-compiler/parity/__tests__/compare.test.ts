import { describe, expect, test } from 'vitest';

import { styleObjectsAgree } from '../lib/compare.js';
import type { CompilerOutcome } from '../lib/types.js';

function accepted(styleObjects: string[]): CompilerOutcome {
  return { status: 'ok', classNames: [], rules: [], rtlRules: [], declarations: [], styleObjects };
}

const REJECTED: CompilerOutcome = { status: 'error', message: 'refused', sentence: 'refused' };

/**
 * The verdict and the report both need this answer — the verdict to decide
 * whether two compilers agreed, the report to decide whether the shapes are
 * worth printing. Asking it in two places is how the two would come to disagree,
 * so it is asked once and covered once.
 */
describe('styleObjectsAgree', () => {
  test('two outcomes carrying the same shapes agree', () => {
    expect(styleObjectsAgree(accepted(['{"k":null}']), accepted(['{"k":null}']))).toBe(true);
  });

  test('an absence on one side and a missing property on the other do not', () => {
    expect(styleObjectsAgree(accepted(['{"k":null}']), accepted(['{}']))).toBe(false);
  });

  test('the same shapes in a different order do not', () => {
    // Order is not incidental: within one style object it decides precedence.
    expect(
      styleObjectsAgree(
        accepted(['{"a":null}', '{"b":null}']),
        accepted(['{"b":null}', '{"a":null}'])
      )
    ).toBe(false);
  });

  test('a different number of style objects does not', () => {
    expect(styleObjectsAgree(accepted(['{}']), accepted(['{}', '{}']))).toBe(false);
  });

  test('two outcomes carrying no shapes at all agree', () => {
    expect(styleObjectsAgree(accepted([]), accepted([]))).toBe(true);
  });

  test('a rejection cannot agree with anything, itself included', () => {
    // A refusal has no shape to compare, so the answer is no rather than a
    // vacuous yes — which would let an acceptance divergence read as agreement
    // on this half and hide behind the CSS half.
    expect(styleObjectsAgree(REJECTED, accepted([]))).toBe(false);
    expect(styleObjectsAgree(accepted([]), REJECTED)).toBe(false);
    expect(styleObjectsAgree(REJECTED, REJECTED)).toBe(false);
  });
});
