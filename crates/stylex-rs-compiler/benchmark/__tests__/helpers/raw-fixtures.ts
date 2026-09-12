/**
 * Builders for the raw-stats shapes a test states by hand.
 */

import type { FixtureRawStats, RawLatencySamples } from '../../lib/types.js';

export interface OneSubjectFixture {
  name: string;
  /** The subject that measured it. No other subject appears in its rounds. */
  label: string;
  /** One value per round, in the order the rounds carry. */
  perRound: readonly number[];
  /** How a value becomes samples. Each suite states latency its own way. */
  samplesOf: (value: number) => RawLatencySamples;
}

/**
 * A fixture only one subject measured.
 *
 * What the release leg writes for a fixture the published base cannot compile:
 * samples for one subject, and no paired block, because there is no pair. The
 * budget suite and the verdict suite both read such a fixture, so the shape is
 * stated one time here.
 */
export function fixtureMeasuredByOne(input: OneSubjectFixture): FixtureRawStats {
  return {
    name: input.name,
    weight: 'standard',
    category: 'transform',
    batchSize: 1,
    rounds: input.perRound.map((value, index) => ({
      round: index,
      subjectOrder: [input.label],
      perSubject: { [input.label]: input.samplesOf(value) },
    })),
  };
}
