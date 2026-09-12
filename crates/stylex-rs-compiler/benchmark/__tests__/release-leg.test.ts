/**
 * The release leg, from the producer to the two gates that read it.
 *
 * The release benchmark compares the candidate against the last published
 * version, which is behind by whole features, so `--allow-base-refusals` lets
 * a fixture the published version cannot compile leave the comparison. The
 * absolute p95 budget is about the candidate alone and has a committed ceiling
 * for every fixture in the manifest, so a fixture that left the comparison and
 * the run together reached the budget as an entry nothing measured.
 *
 * These assertions run the real producer and hand what it wrote to the real
 * consumers, which is the only place the two readings meet.
 */

import { describe, expect, test } from 'vitest';

import {
  BUDGET_SCHEMA_VERSION,
  evaluateBudget,
  type BudgetEntry,
  type BudgetFile,
} from '../lib/budget.js';
import { runRounds, type UncomparedFixture } from '../lib/runner.js';
import { createSubject } from '../lib/subjects.js';
import {
  RAW_STATS_SCHEMA_VERSION,
  type BootstrapConfig,
  type FixtureDescriptor,
  type RawStatsEnvironment,
  type RawStatsFile,
} from '../lib/types.js';
import { evaluateRawStats, renderVerdictMarkdown } from '../lib/verdict.js';

const SHARED: FixtureDescriptor = {
  name: 'card',
  filePath: '/fixtures/card.js',
  code: 'const styles = 1;',
  weight: 'standard',
  category: 'transform',
  batchSize: 1,
};

/** A fixture that prices a feature the published base does not carry. */
const NEW_FEATURE: FixtureDescriptor = { ...SHARED, name: 'Feature - engine fold' };

const BENCH = { retainSamples: true, warmup: false, time: 0, iterations: 1 } as const;

const ENVIRONMENT: RawStatsEnvironment = {
  timestamp: '2026-09-12T00:00:00.000Z',
  node: 'v24.18.0',
  os: { type: 'Linux', release: '6.11.0', arch: 'x64', platform: 'linux' },
  cpu: { model: 'AMD EPYC 7763', cores: 4 },
  memoryGB: 16,
  packageVersion: '0.19.0-rc.3',
  target: 'x86_64-unknown-linux-gnu',
  toolchain: {},
  runnerImage: 'ubuntu24',
  runnerImageVersion: '20260803.1.0',
};

function entry(name: string): BudgetEntry {
  return {
    name,
    ceilingMs: 1000,
    observedUpperMs: 800,
    headroom: 1.25,
    runs: 5,
    reviewedAt: '2026-09-10',
    evidence: 'runs 1-5 of workflow 34710120184',
  };
}

const BUDGET: BudgetFile = {
  schemaVersion: BUDGET_SCHEMA_VERSION,
  state: 'enforced',
  subject: 'candidate',
  statistic: 'median-of-round-p95',
  canonical: {
    target: 'x86_64-unknown-linux-gnu',
    node: 'v24.18.0',
    runner: 'ubuntu-latest',
    runnerImages: ['ubuntu24'],
    runnerImageVersions: ['20260803.1.0'],
  },
  policy: {
    seeding: 'seed from repeated clean runs',
    increases: 'reviewed change with evidence',
    decreases: 'ratchet proven improvements',
    automation: 'never written by a task',
    environment: 'canonical target, Node, and image only',
  },
  entries: [entry(SHARED.name), entry(NEW_FEATURE.name)],
};

const BOOTSTRAP: BootstrapConfig = { seed: 42, resamples: 200, confidence: 0.95 };

interface ReleaseRun {
  rawStats: RawStatsFile;
  uncompared: readonly UncomparedFixture[];
}

/** The release leg's own run: a published base that refuses the new fixture. */
async function releaseRun(): Promise<ReleaseRun> {
  const base = createSubject(
    { label: 'npm@0.18.6', version: '0.18.6', resolvedFrom: '/npm' },
    fixture => {
      if (fixture.name === NEW_FEATURE.name) {
        throw new Error("[StyleX] The method 'trim' is not yet supported in static evaluation.");
      }
      return 1;
    }
  );
  const candidate = createSubject(
    { label: 'candidate@0.19.0-rc.3', version: '0.19.0-rc.3', resolvedFrom: '/candidate' },
    () => 1
  );

  const result = await runRounds({
    subjects: [base, candidate],
    fixtures: [SHARED, NEW_FEATURE],
    stylexOptions: {},
    rounds: 2,
    seed: 1,
    standardBench: BENCH,
    heavyBench: BENCH,
    requiredSubject: candidate.descriptor.label,
  });

  return {
    rawStats: {
      schemaVersion: RAW_STATS_SCHEMA_VERSION,
      environment: ENVIRONMENT,
      subjects: [base.descriptor, candidate.descriptor],
      fixtures: result.fixtures,
    },
    uncompared: result.uncompared,
  };
}

describe('release leg — a base that refuses a fixture', () => {
  test('the candidate is still measured for the absolute budget', async () => {
    const report = evaluateBudget((await releaseRun()).rawStats, BUDGET);

    expect(report.problems).toEqual([]);
    expect(report.status).toBe('pass');
    expect(report.fixtures.map(fixture => fixture.name)).toContain(NEW_FEATURE.name);
  });

  test('the run says which fixture lost its comparison, and why', async () => {
    expect((await releaseRun()).uncompared).toEqual([
      {
        fixture: NEW_FEATURE.name,
        subject: 'npm@0.18.6',
        reason: "[StyleX] The method 'trim' is not yet supported in static evaluation.",
      },
    ]);
  });

  test('the verdict scores what it can compare and names what it cannot', async () => {
    const report = evaluateRawStats((await releaseRun()).rawStats, { bootstrap: BOOTSTRAP });

    expect(report.suiteStatus).toBe('pass');
    expect(report.fixtures.map(fixture => fixture.name)).toStrictEqual([SHARED.name]);
    expect(report.uncompared).toStrictEqual([NEW_FEATURE.name]);
    expect(renderVerdictMarkdown(report)).toContain(
      `Not compared (measured for candidate@0.19.0-rc.3 only): ${NEW_FEATURE.name}`
    );
  });
});
