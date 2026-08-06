import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { describe, expect, test } from 'vitest';

import { runComparison, type RetryRequest } from '../compare-revisions.js';
import {
  RAW_STATS_SCHEMA_VERSION,
  type BootstrapConfig,
  type FixtureRawStats,
  type RawLatencySamples,
  type RawStatsEnvironment,
  type RawStatsFile,
  type SubjectDescriptor,
} from '../lib/types.js';
import {
  DEFAULT_THRESHOLDS,
  escapeMarkdownCell,
  evaluateRawStats,
  renderVerdictMarkdown,
  type VerdictThresholds,
} from '../lib/verdict.js';

const BASE: SubjectDescriptor = { label: 'base', version: '1.0.0', resolvedFrom: '/base' };
const CANDIDATE: SubjectDescriptor = {
  label: 'candidate',
  version: '1.0.0',
  resolvedFrom: '/candidate',
};

const BOOTSTRAP: BootstrapConfig = { seed: 42, resamples: 500, confidence: 0.95 };

const ENV: RawStatsEnvironment = {
  timestamp: '2026-08-06T00:00:00.000Z',
  node: 'v24.18.0',
  os: { type: 'Linux', release: '6.0.0', arch: 'x64', platform: 'linux' },
  cpu: { model: 'test-cpu', cores: 4 },
  memoryGB: 8,
  packageVersion: '0.18.3',
  target: 'x86_64-unknown-linux-gnu',
  toolchain: {},
};

function samplesFor(p50: number): RawLatencySamples {
  const samples = [p50 * 0.9, p50, p50 * 1.1];
  return {
    samples,
    p50,
    p95: samples[2] ?? p50,
    rme: 1,
    samplesCount: samples.length,
    opsPerSec: 1000 / p50,
  };
}

function fixture(
  name: string,
  basePerRound: readonly number[],
  candidatePerRound: readonly number[]
): FixtureRawStats {
  const rounds = basePerRound.map((baseP50, i) => ({
    round: i,
    subjectOrder: [BASE.label, CANDIDATE.label] as const,
    perSubject: {
      [BASE.label]: samplesFor(baseP50),
      [CANDIDATE.label]: samplesFor(candidatePerRound[i] ?? Number.NaN),
    },
  }));
  return {
    name,
    weight: 'standard',
    category: 'transform',
    batchSize: 1,
    rounds,
  };
}

function rawStats(
  fixtures: readonly FixtureRawStats[],
  subjects: readonly SubjectDescriptor[] = [BASE, CANDIDATE]
): RawStatsFile {
  return {
    schemaVersion: RAW_STATS_SCHEMA_VERSION,
    environment: ENV,
    subjects,
    fixtures,
  };
}

const tightThresholds: VerdictThresholds = {
  warn: 1.1,
  fail: 1.2,
  improvementWarn: 0.5,
};

describe('evaluateRawStats — status boundaries', () => {
  test('flat 1.0 ratios pass', () => {
    const rounds = Array.from({ length: 10 }, () => 1);
    const report = evaluateRawStats(rawStats([fixture('flat', rounds, rounds)]), {
      thresholds: tightThresholds,
      bootstrap: BOOTSTRAP,
    });
    expect(report.suiteStatus).toBe('pass');
    expect(report.fixtures[0]?.status).toBe('pass');
    expect(report.flagged).toStrictEqual([]);
  });

  test('lower bound at exactly 1.20 flags for retry', () => {
    const base = Array.from({ length: 20 }, () => 1);
    const candidate = Array.from({ length: 20 }, () => 1.2);
    const report = evaluateRawStats(rawStats([fixture('slow', base, candidate)]), {
      thresholds: tightThresholds,
      bootstrap: BOOTSTRAP,
    });
    expect(report.suiteStatus).toBe('flagged');
    expect(report.fixtures[0]?.status).toBe('flagged');
    expect(report.flagged).toStrictEqual(['slow']);
  });

  test('lower bound at exactly 1.10 warns but does not flag', () => {
    const base = Array.from({ length: 20 }, () => 1);
    const candidate = Array.from({ length: 20 }, () => 1.1);
    const report = evaluateRawStats(rawStats([fixture('mild', base, candidate)]), {
      thresholds: tightThresholds,
      bootstrap: BOOTSTRAP,
    });
    expect(report.fixtures[0]?.status).toBe('warn');
    expect(report.suiteStatus).toBe('pass');
  });

  test('upper bound at exactly 0.50 warns without flagging', () => {
    const base = Array.from({ length: 20 }, () => 1);
    const candidate = Array.from({ length: 20 }, () => 0.5);
    const report = evaluateRawStats(rawStats([fixture('impossible', base, candidate)]), {
      thresholds: tightThresholds,
      bootstrap: BOOTSTRAP,
    });
    expect(report.fixtures[0]?.status).toBe('improvement-warn');
    expect(report.suiteStatus).toBe('pass');
  });
});

describe('evaluateRawStats — targeted retry', () => {
  test('any single fixture failing on retry fails the whole suite', () => {
    const flat = Array.from({ length: 15 }, () => 1);
    const slowdown = Array.from({ length: 15 }, () => 1.3);
    const primary = rawStats([
      fixture('a', flat, flat),
      fixture('b', flat, slowdown),
      fixture('c', flat, flat),
    ]);
    const retry = rawStats([fixture('b', flat, slowdown)]);
    const report = evaluateRawStats(primary, {
      thresholds: tightThresholds,
      bootstrap: BOOTSTRAP,
      retry,
    });
    expect(report.flagged).toStrictEqual(['b']);
    expect(report.hasReproducedFailure).toBe(true);
    expect(report.suiteStatus).toBe('failed');
    expect(report.fixtures[1]?.status).toBe('failed');
    expect(report.fixtures[0]?.status).toBe('pass');
    expect(report.fixtures[2]?.status).toBe('pass');
  });

  test('flagged fixture recovering on retry does not fail the suite', () => {
    const flat = Array.from({ length: 15 }, () => 1);
    const slowdown = Array.from({ length: 15 }, () => 1.3);
    const primary = rawStats([fixture('flake', flat, slowdown)]);
    const retry = rawStats([fixture('flake', flat, flat)]);
    const report = evaluateRawStats(primary, {
      thresholds: tightThresholds,
      bootstrap: BOOTSTRAP,
      retry,
    });
    expect(report.hasReproducedFailure).toBe(false);
    expect(report.suiteStatus).toBe('pass');
    expect(report.fixtures[0]?.status).toBe('pass');
    expect(report.fixtures[0]?.retryInterval).toBeDefined();
  });

  test('rejects retry data for a non-flagged fixture', () => {
    const flat = Array.from({ length: 15 }, () => 1);
    const primary = rawStats([fixture('a', flat, flat)]);
    const retry = rawStats([
      fixture(
        'a',
        flat,
        Array.from({ length: 15 }, () => 5)
      ),
    ]);
    expect(() =>
      evaluateRawStats(primary, {
        thresholds: tightThresholds,
        bootstrap: BOOTSTRAP,
        retry,
      })
    ).toThrow(/contains non-flagged fixture "a"/);
  });

  test('rejects retry data missing a flagged fixture', () => {
    const flat = Array.from({ length: 15 }, () => 1);
    const slowdown = Array.from({ length: 15 }, () => 1.3);
    const primary = rawStats([fixture('a', flat, slowdown), fixture('b', flat, slowdown)]);
    const retry = rawStats([fixture('a', flat, flat)]);

    expect(() =>
      evaluateRawStats(primary, {
        thresholds: tightThresholds,
        bootstrap: BOOTSTRAP,
        retry,
      })
    ).toThrow(/missing flagged fixture "b"/);
  });

  test('rejects retry data with fewer rounds than the primary run', () => {
    const flat = Array.from({ length: 15 }, () => 1);
    const slowdown = Array.from({ length: 15 }, () => 1.3);
    const primary = rawStats([fixture('slow', flat, slowdown)]);
    const retry = rawStats([fixture('slow', flat.slice(1), flat.slice(1))]);

    expect(() =>
      evaluateRawStats(primary, {
        thresholds: tightThresholds,
        bootstrap: BOOTSTRAP,
        retry,
      })
    ).toThrow(/must contain 15 rounds/);
  });
});

describe('runComparison', () => {
  test('measures all and only flagged fixtures once with the full round count', async () => {
    const flat = Array.from({ length: 15 }, () => 1);
    const slowdown = Array.from({ length: 15 }, () => 1.3);
    const primary = rawStats([
      fixture('pass', flat, flat),
      fixture('slow-a', flat, slowdown),
      fixture('slow-b', flat, slowdown),
    ]);
    const requests: RetryRequest[] = [];

    const report = await runComparison(
      primary,
      {
        thresholds: tightThresholds,
        bootstrap: BOOTSTRAP,
        retry: undefined,
        retryOutput: '/unused/retry.json',
        retrySeed: 9,
        retryTimeBudgetMs: 1000,
      },
      async request => {
        requests.push(request);
        return rawStats([fixture('slow-a', flat, flat), fixture('slow-b', flat, flat)]);
      }
    );

    expect(requests).toHaveLength(1);
    expect(requests[0]).toMatchObject({
      fixtureNames: ['slow-a', 'slow-b'],
      rounds: 15,
      seed: 9,
      timeBudgetMs: 1000,
    });
    expect(report.suiteStatus).toBe('pass');
  });
});

describe('evaluateRawStats — validation', () => {
  test('rejects unsupported schema version', () => {
    const bad = {
      ...rawStats([fixture('x', [1, 1, 1], [1, 1, 1])]),
      schemaVersion: 999,
    };
    expect(() =>
      evaluateRawStats(bad, { thresholds: tightThresholds, bootstrap: BOOTSTRAP })
    ).toThrow(/schemaVersion 999 is not supported/);
  });

  test('rejects raw stats with wrong subject count', () => {
    const file = rawStats([fixture('x', [1], [1])], [BASE]);
    expect(() =>
      evaluateRawStats(file, { thresholds: tightThresholds, bootstrap: BOOTSTRAP })
    ).toThrow(/exactly two subjects/);
  });

  test('rejects retry with mismatched subject labels', () => {
    const primary = rawStats([fixture('x', [1, 1], [1, 1])]);
    const retry = rawStats([fixture('x', [1, 1], [1, 1])], [CANDIDATE, BASE]);
    expect(() =>
      evaluateRawStats(primary, {
        thresholds: tightThresholds,
        bootstrap: BOOTSTRAP,
        retry,
      })
    ).toThrow(/Retry raw stats subjects must match/);
  });

  test('rejects retry with a different subject identity under the same label', () => {
    const flat = Array.from({ length: 15 }, () => 1);
    const slowdown = Array.from({ length: 15 }, () => 1.3);
    const primary = rawStats([fixture('slow', flat, slowdown)]);
    const retry = rawStats(
      [fixture('slow', flat, flat)],
      [{ ...BASE, version: '2.0.0' }, CANDIDATE]
    );

    expect(() =>
      evaluateRawStats(primary, {
        thresholds: tightThresholds,
        bootstrap: BOOTSTRAP,
        retry,
      })
    ).toThrow(/subjects must match the primary base\/candidate identities/);
  });

  test('rejects fixture with missing candidate samples', () => {
    const validFixture = fixture('x', [1, 1], [1, 1]);
    const firstRound = validFixture.rounds[0];
    const file = rawStats([
      {
        ...validFixture,
        rounds: [
          {
            round: firstRound?.round ?? 0,
            subjectOrder: firstRound?.subjectOrder ?? [BASE.label, CANDIDATE.label],
            perSubject: { [BASE.label]: samplesFor(1) },
          },
        ],
      },
    ]);
    expect(() =>
      evaluateRawStats(file, { thresholds: tightThresholds, bootstrap: BOOTSTRAP })
    ).toThrow(/perSubject\["candidate"\] must be an object/);
  });

  test('rejects malformed nested latency values', () => {
    const validFixture = fixture('x', [1], [1]);
    const firstRound = validFixture.rounds[0];
    const malformed = {
      ...rawStats([validFixture]),
      fixtures: [
        {
          ...validFixture,
          rounds: [
            {
              ...firstRound,
              perSubject: {
                ...firstRound?.perSubject,
                [BASE.label]: { ...samplesFor(1), p50: 'fast' },
              },
            },
          ],
        },
      ],
    };

    expect(() =>
      evaluateRawStats(malformed, { thresholds: tightThresholds, bootstrap: BOOTSTRAP })
    ).toThrow(/base.*p50 must be a positive finite number/);
  });

  test('rejects non-contiguous round indices', () => {
    const validFixture = fixture('x', [1, 1], [1, 1]);
    const malformed = {
      ...rawStats([validFixture]),
      fixtures: [
        {
          ...validFixture,
          rounds: validFixture.rounds.map(round => Object.assign({}, round, { round: 1 })),
        },
      ],
    };

    expect(() =>
      evaluateRawStats(malformed, { thresholds: tightThresholds, bootstrap: BOOTSTRAP })
    ).toThrow(/contiguous zero-based indices/);
  });
});

describe('evaluateRawStats — determinism', () => {
  test('same inputs yield identical bootstrap intervals', () => {
    const base = Array.from({ length: 12 }, () => 1);
    const candidate = Array.from({ length: 12 }, () => 1.05);
    const file = rawStats([fixture('deterministic', base, candidate)]);
    const first = evaluateRawStats(file, {
      thresholds: DEFAULT_THRESHOLDS,
      bootstrap: BOOTSTRAP,
    });
    const second = evaluateRawStats(file, {
      thresholds: DEFAULT_THRESHOLDS,
      bootstrap: BOOTSTRAP,
    });
    expect(second.fixtures[0]?.interval).toStrictEqual(first.fixtures[0]?.interval);
  });
});

describe('renderVerdictMarkdown', () => {
  test('escapes pipes, backticks, and control characters in user-visible names', () => {
    const base = Array.from({ length: 8 }, () => 1);
    const candidate = Array.from({ length: 8 }, () => 1.3);
    const file = rawStats([fixture('a|b`c\ndef', base, candidate)]);
    const report = evaluateRawStats(file, {
      thresholds: tightThresholds,
      bootstrap: BOOTSTRAP,
    });
    const markdown = renderVerdictMarkdown(report);
    expect(markdown).toContain('a\\|b\\`c def');
    expect(markdown).not.toContain('a|b`c\ndef');
  });

  test('escapeMarkdownCell handles backslashes before pipes', () => {
    expect(escapeMarkdownCell('a\\|b')).toBe('a\\\\\\|b');
  });
});

describe('compare-revisions CLI diagnostics', () => {
  test('writes JSON and Markdown artifacts before failing malformed input', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'stylex-verdict-'));
    const primaryPath = path.join(tempDir, 'primary.json');
    const outputPath = path.join(tempDir, 'verdict.json');
    const summaryPath = path.join(tempDir, 'compare-revisions.summary.md');
    const scriptPath = fileURLToPath(new URL('../compare-revisions.ts', import.meta.url));

    try {
      fs.writeFileSync(primaryPath, '{"schemaVersion":999}\n', 'utf8');
      const result = spawnSync(
        process.execPath,
        [
          '--import',
          'tsx/esm',
          scriptPath,
          '--primary',
          primaryPath,
          `--output-json=${outputPath}`,
        ],
        { encoding: 'utf8' }
      );

      expect(result.status).toBe(1);
      expect(JSON.parse(fs.readFileSync(outputPath, 'utf8'))).toMatchObject({
        schemaVersion: 1,
        suiteStatus: 'error',
        error: { message: expect.stringMatching(/schemaVersion 999 is not supported/) },
      });
      expect(fs.readFileSync(summaryPath, 'utf8')).toContain('Suite status: **error**');
    } finally {
      fs.rmSync(tempDir, { recursive: true, force: true });
    }
  });

  test('rejects numeric options with trailing characters and honors equals-style summary path', () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'stylex-verdict-'));
    const primaryPath = path.join(tempDir, 'primary.json');
    const outputPath = path.join(tempDir, 'verdict.json');
    const summaryPath = path.join(tempDir, 'custom-summary.md');
    const scriptPath = fileURLToPath(new URL('../compare-revisions.ts', import.meta.url));

    try {
      fs.writeFileSync(primaryPath, `${JSON.stringify(rawStats([fixture('x', [1], [1])]))}\n`);
      const result = spawnSync(
        process.execPath,
        [
          '--import',
          'tsx/esm',
          scriptPath,
          `--primary=${primaryPath}`,
          `--output-json=${outputPath}`,
          `--summary-md=${summaryPath}`,
          '--fail=1.2x',
        ],
        { encoding: 'utf8' }
      );

      expect(result.status).toBe(1);
      expect(JSON.parse(fs.readFileSync(outputPath, 'utf8'))).toMatchObject({
        suiteStatus: 'error',
        error: { message: expect.stringMatching(/Invalid --fail value/) },
      });
      expect(fs.readFileSync(summaryPath, 'utf8')).toContain('Suite status: **error**');
    } finally {
      fs.rmSync(tempDir, { recursive: true, force: true });
    }
  });
});
