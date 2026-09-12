/**
 * How one round of one fixture is timed.
 *
 * Two callers time a round. The single-process runner puts every subject in one
 * tinybench instance, so the subjects of a round meet the same machine at the
 * same time. The split path puts in the one subject its process holds, because
 * macOS cannot hold the two together. What they must not do is time a round
 * differently, because a ratio compares a round of one subject against a round
 * of the other, and the two can come from different platforms.
 *
 * So the parts that decide what a measurement means -- the batch loop, the
 * tinybench options, the name of the round and how samples are read off a task
 * -- are here, and each caller supplies only the work to repeat.
 */

import { Bench, type BenchOptions } from 'tinybench';

import { extractLatencySamples } from './stats.js';
import type { FixtureDescriptor, RawLatencySamples } from './types.js';

/** One subject as a round sees it: a name, and the work to repeat. */
export interface RoundEntry {
  readonly label: string;
  /** One transform of the fixture, with the fixture's options already resolved. */
  readonly transform: () => void;
}

/** The tinybench budgets a round is given, one for each weight class. */
export interface RoundBudgets {
  readonly standard: BenchOptions;
  readonly heavy: BenchOptions;
}

/**
 * Times one round of one fixture, for the entries in the order given.
 *
 * The weight decides the budget here, so a heavy fixture cannot be given the
 * standard budget by one caller and the reduced one by the other.
 */
export async function timeRound(
  fixture: FixtureDescriptor,
  budgets: RoundBudgets,
  entries: readonly RoundEntry[]
): Promise<Record<string, RawLatencySamples>> {
  const bench = new Bench({
    name: `${fixture.name} (round)`,
    ...(fixture.weight === 'heavy' ? budgets.heavy : budgets.standard),
  });

  for (const entry of entries) {
    bench.add(entry.label, () => {
      // Batching lifts sub-millisecond fixtures above timer noise.
      for (let i = 0; i < fixture.batchSize; i++) entry.transform();
    });
  }

  await bench.run();

  const perSubject: Record<string, RawLatencySamples> = {};
  for (const task of bench.tasks) perSubject[task.name] = extractLatencySamples(task);

  return perSubject;
}
