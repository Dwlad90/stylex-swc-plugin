/**
 * Producer-side guarantees of `runRounds`.
 *
 * The budget check resolves which subject its ceilings describe from
 * `fixtures[].paired`, and the release benchmark records roles without
 * bootstrap statistics. That combination was untested: the budget suite
 * hand-built raw stats that always carried a full `paired` block, so a
 * producer that emitted none still passed every test and failed the
 * release job instead. These assertions run the real producer.
 */

import { describe, expect, test } from 'vitest';

import { parseRawStats } from '../lib/raw-stats.js';
import { runRounds } from '../lib/runner.js';
import { createSubject } from '../lib/subjects.js';
import {
  RAW_STATS_SCHEMA_VERSION,
  type FixtureDescriptor,
  type RawStatsEnvironment,
} from '../lib/types.js';

const FIXTURE: FixtureDescriptor = {
  name: 'card',
  filePath: '/fixtures/card.js',
  code: 'const styles = 1;',
  weight: 'standard',
  category: 'transform',
  batchSize: 1,
};

// One iteration per task keeps the suite fast; the assertions are about
// the shape of the emitted stats, not the timings.
const BENCH = { retainSamples: true, warmup: false, time: 0, iterations: 1 } as const;

const ENVIRONMENT: RawStatsEnvironment = {
  timestamp: '2026-01-01T00:00:00.000Z',
  node: 'v24.18.0',
  os: { type: 'Linux', release: '6.0', arch: 'x64', platform: 'linux' },
  cpu: { model: 'test', cores: 2 },
  memoryGB: 16,
  packageVersion: '0.0.0',
  target: 'x86_64-unknown-linux-gnu',
  toolchain: {},
};

function subject(label: string) {
  return createSubject({ label, version: '1.0.0', resolvedFrom: `/${label}` }, () => 1);
}

async function run(subjects: ReturnType<typeof subject>[]) {
  return runRounds({
    subjects,
    fixtures: [FIXTURE],
    stylexOptions: {},
    rounds: 1,
    seed: 1,
    standardBench: BENCH,
    heavyBench: BENCH,
  });
}

describe('runRounds paired roles', () => {
  test('a two-subject run records roles even without bootstrap statistics', async () => {
    const { fixtures } = await run([subject('base'), subject('candidate')]);
    const paired = fixtures[0]?.paired;

    expect(paired).toBeDefined();
    expect(paired?.base).toBe('base');
    expect(paired?.candidate).toBe('candidate');
    expect(paired?.ratios).toBeUndefined();
    expect(paired?.confidence).toBeUndefined();
  });

  test('a single-subject run records no paired block', async () => {
    const { fixtures } = await run([subject('current')]);
    expect(fixtures[0]?.paired).toBeUndefined();
  });

  test('roles-only output survives a raw-stats round trip', async () => {
    const { fixtures } = await run([subject('base'), subject('candidate')]);
    const file = {
      schemaVersion: RAW_STATS_SCHEMA_VERSION,
      environment: ENVIRONMENT,
      subjects: [
        { label: 'base', version: '1.0.0', resolvedFrom: '/base' },
        { label: 'candidate', version: '1.0.0', resolvedFrom: '/candidate' },
      ],
      fixtures,
    };

    const parsed = parseRawStats(JSON.parse(JSON.stringify(file)), 'raw', { subjects: 'any' });
    expect(parsed.fixtures[0]?.paired?.candidate).toBe('candidate');
  });
});
