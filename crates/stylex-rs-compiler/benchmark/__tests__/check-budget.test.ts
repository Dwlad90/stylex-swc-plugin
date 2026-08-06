import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { describe, expect, test } from 'vitest';

import {
  BUDGET_SCHEMA_VERSION,
  evaluateBudget,
  parseBudget,
  renderBudgetMarkdown,
  type BudgetEntry,
  type BudgetFile,
} from '../lib/budget.js';
import { escapeMarkdownCell } from '../lib/format.js';
import {
  RAW_STATS_SCHEMA_VERSION,
  type FixtureRawStats,
  type RawLatencySamples,
  type RawStatsEnvironment,
  type RawStatsFile,
  type SubjectDescriptor,
} from '../lib/types.js';

const BASE: SubjectDescriptor = { label: 'base', version: '0.18.2', resolvedFrom: '/base' };
const CANDIDATE: SubjectDescriptor = {
  label: 'candidate',
  version: '0.18.3',
  resolvedFrom: '/candidate',
};

const CANONICAL_ENV: RawStatsEnvironment = {
  timestamp: '2026-08-06T00:00:00.000Z',
  node: 'v24.18.0',
  os: { type: 'Linux', release: '6.11.0', arch: 'x64', platform: 'linux' },
  cpu: { model: 'AMD EPYC 7763', cores: 4 },
  memoryGB: 16,
  packageVersion: '0.18.3',
  target: 'x86_64-unknown-linux-gnu',
  toolchain: {},
  runnerImage: 'ubuntu24',
  runnerImageVersion: '20260803.1.0',
};

function samples(p95: number): RawLatencySamples {
  const values = [p95 * 0.8, p95 * 0.9, p95];
  return {
    samples: values,
    p50: values[1] ?? p95,
    p95,
    rme: 1,
    samplesCount: values.length,
    opsPerSec: 1000 / p95,
  };
}

function fixture(name: string, candidateP95PerRound: readonly number[]): FixtureRawStats {
  return {
    name,
    weight: 'standard',
    category: 'transform',
    batchSize: 1,
    rounds: candidateP95PerRound.map((p95, index) => ({
      round: index,
      subjectOrder: [BASE.label, CANDIDATE.label],
      perSubject: {
        [BASE.label]: samples(p95 * 2),
        [CANDIDATE.label]: samples(p95),
      },
    })),
    // The paired block is what names the roles; the budget follows it
    // rather than the order subjects happen to appear in.
    paired: {
      base: BASE.label,
      candidate: CANDIDATE.label,
      ratios: candidateP95PerRound.map(() => 0.5),
      confidence: { point: 0.5, lower: 0.45, upper: 0.55 },
    },
  };
}

function rawStats(
  fixtures: readonly FixtureRawStats[],
  environment: RawStatsEnvironment = CANONICAL_ENV
): RawStatsFile {
  return {
    schemaVersion: RAW_STATS_SCHEMA_VERSION,
    environment,
    subjects: [BASE, CANDIDATE],
    fixtures,
  };
}

function entry(name: string, ceilingMs: number): BudgetEntry {
  return {
    name,
    ceilingMs,
    observedUpperMs: ceilingMs / 1.25,
    headroom: 1.25,
    runs: 5,
    reviewedAt: '2026-08-06',
    evidence: 'runs 1-5 of workflow 31048969131',
  };
}

const POLICY = {
  seeding: 'seed from repeated clean runs',
  increases: 'reviewed change with evidence',
  decreases: 'ratchet proven improvements',
  automation: 'never written by a task',
  environment: 'canonical target, Node, and image only',
};

function budget(entries: readonly BudgetEntry[]): BudgetFile {
  return {
    schemaVersion: BUDGET_SCHEMA_VERSION,
    state: entries.length === 0 ? 'pending-calibration' : 'enforced',
    subject: 'candidate',
    statistic: 'median-of-round-p95',
    canonical: {
      target: 'x86_64-unknown-linux-gnu',
      node: 'v24.18.0',
      runner: 'ubuntu-latest',
      runnerImages: ['ubuntu24'],
      runnerImageVersions: entries.length === 0 ? [] : ['20260803.1.0'],
    },
    policy: POLICY,
    entries,
  };
}

describe('evaluateBudget — ceiling boundaries', () => {
  test('p95 below the ceiling passes', () => {
    const report = evaluateBudget(
      rawStats([fixture('card', [1, 1.1, 0.9])]),
      budget([entry('card', 2)])
    );
    expect(report.status).toBe('pass');
    expect(report.fixtures[0]?.status).toBe('pass');
    expect(report.fixtures[0]?.observedP95Ms).toBe(1);
    expect(report.fixtures[0]?.utilization).toBe(0.5);
  });

  test('p95 exactly at the ceiling passes', () => {
    const report = evaluateBudget(
      rawStats([fixture('card', [2, 2, 2])]),
      budget([entry('card', 2)])
    );
    expect(report.status).toBe('pass');
    expect(report.fixtures[0]?.status).toBe('pass');
  });

  test('p95 just above the ceiling fails', () => {
    const report = evaluateBudget(
      rawStats([fixture('card', [2.0001, 2.0001, 2.0001])]),
      budget([entry('card', 2)])
    );
    expect(report.status).toBe('failed');
    expect(report.fixtures[0]?.status).toBe('breach');
    expect(report.problems.map(problem => problem.kind)).toStrictEqual(['breach']);
  });

  test('the median of per-round p95 decides, not a single noisy round', () => {
    const report = evaluateBudget(
      rawStats([fixture('card', [1, 1, 9])]),
      budget([entry('card', 2)])
    );
    expect(report.fixtures[0]?.observedP95Ms).toBe(1);
    expect(report.status).toBe('pass');
  });
});

describe('evaluateBudget — coverage', () => {
  test('a measured benchmark without a ceiling fails', () => {
    const report = evaluateBudget(
      rawStats([fixture('card', [1]), fixture('page', [1])]),
      budget([entry('card', 2)])
    );
    expect(report.status).toBe('failed');
    expect(report.problems).toContainEqual({
      kind: 'missing-entry',
      message: 'no committed ceiling for "page"',
    });
    expect(report.fixtures[1]?.status).toBe('unbudgeted');
  });

  test('a ceiling for a benchmark that was not measured fails', () => {
    const report = evaluateBudget(
      rawStats([fixture('card', [1])]),
      budget([entry('card', 2), entry('removed', 2)])
    );
    expect(report.status).toBe('failed');
    expect(report.problems).toContainEqual({
      kind: 'extra-entry',
      message: 'budget entry "removed" was not measured in this run',
    });
  });
});

describe('evaluateBudget — canonical environment', () => {
  test('a different target forces recalibration instead of comparison', () => {
    const report = evaluateBudget(
      rawStats([fixture('card', [1])], { ...CANONICAL_ENV, target: 'aarch64-apple-darwin' }),
      budget([entry('card', 2)])
    );
    expect(report.status).toBe('failed');
    expect(report.problems[0]?.kind).toBe('environment-target');
  });

  test('a different Node version fails', () => {
    const report = evaluateBudget(
      rawStats([fixture('card', [1])], { ...CANONICAL_ENV, node: 'v22.0.0' }),
      budget([entry('card', 2)])
    );
    expect(report.problems.map(problem => problem.kind)).toContain('environment-node');
  });

  test('runner image drift fails even when every benchmark is within budget', () => {
    const report = evaluateBudget(
      rawStats([fixture('card', [1])], { ...CANONICAL_ENV, runnerImage: 'ubuntu26' }),
      budget([entry('card', 2)])
    );
    expect(report.status).toBe('failed');
    expect(report.fixtures[0]?.status).toBe('pass');
    expect(report.problems[0]?.kind).toBe('environment-runner-image');
    expect(report.problems[0]?.message).toContain('recalibration required');
  });

  test('a missing runner image fails rather than comparing silently', () => {
    const environment = { ...CANONICAL_ENV };
    delete environment.runnerImage;
    const report = evaluateBudget(
      rawStats([fixture('card', [1])], environment),
      budget([entry('card', 2)])
    );
    expect(report.problems[0]?.kind).toBe('environment-runner-image');
  });

  test('a rebuilt image within the same family fails once builds are pinned', () => {
    const report = evaluateBudget(
      rawStats([fixture('card', [1])], { ...CANONICAL_ENV, runnerImageVersion: '20260901.2.0' }),
      budget([entry('card', 2)])
    );
    expect(report.status).toBe('failed');
    expect(report.problems[0]?.kind).toBe('environment-runner-image-version');
    expect(report.problems[0]?.message).toContain('recalibration required');
  });

  test('a missing image version fails once builds are pinned', () => {
    const environment = { ...CANONICAL_ENV };
    delete environment.runnerImageVersion;
    const report = evaluateBudget(
      rawStats([fixture('card', [1])], environment),
      budget([entry('card', 2)])
    );
    expect(report.problems[0]?.kind).toBe('environment-runner-image-version');
  });

  test('an unpinned budget ignores the image version', () => {
    const report = evaluateBudget(
      rawStats([fixture('card', [1])], { ...CANONICAL_ENV, runnerImageVersion: '20260901.2.0' }),
      budget([])
    );
    expect(report.problems).toStrictEqual([]);
    expect(report.status).toBe('unseeded');
  });

  test('environment drift fails even while ceilings are pending calibration', () => {
    const report = evaluateBudget(
      rawStats([fixture('card', [1])], { ...CANONICAL_ENV, target: 'aarch64-apple-darwin' }),
      budget([])
    );
    expect(report.status).toBe('failed');
    expect(report.problems[0]?.kind).toBe('environment-target');
  });

  test('the CPU model stays visible as a diagnostic', () => {
    const report = evaluateBudget(
      rawStats([fixture('card', [1])], {
        ...CANONICAL_ENV,
        cpu: { model: 'Intel Xeon', cores: 4 },
      }),
      budget([entry('card', 2)])
    );
    expect(report.status).toBe('pass');
    expect(report.environment.cpu.model).toBe('Intel Xeon');
  });
});

describe('evaluateBudget — pending calibration', () => {
  test('an unseeded budget reports observations without failing', () => {
    const report = evaluateBudget(rawStats([fixture('card', [1, 2, 3])]), budget([]));
    expect(report.status).toBe('unseeded');
    expect(report.budgetState).toBe('pending-calibration');
    expect(report.reportOnly).toBe(false);
    expect(report.fixtures[0]?.status).toBe('unseeded');
    expect(report.fixtures[0]?.observedP95Ms).toBe(2);
    expect(report.fixtures[0]?.ceilingMs).toBeUndefined();
  });
});

describe('evaluateBudget — subject selection', () => {
  test('the candidate role comes from the paired block', () => {
    const report = evaluateBudget(rawStats([fixture('card', [1])]), budget([entry('card', 2)]));
    expect(report.subject.label).toBe('candidate');
    expect(report.fixtures[0]?.observedP95Ms).toBe(1);
  });

  test('reordering the subjects does not switch the budget onto the baseline', () => {
    const reordered = { ...rawStats([fixture('card', [1])]), subjects: [CANDIDATE, BASE] };
    const report = evaluateBudget(reordered, budget([entry('card', 2)]));
    expect(report.subject.label).toBe('candidate');
    expect(report.fixtures[0]?.observedP95Ms).toBe(1);
  });

  test('a budget written against the base role follows the paired base', () => {
    const baseBudget = { ...budget([entry('card', 4)]), subject: 'base' as const };
    const report = evaluateBudget(rawStats([fixture('card', [1])]), baseBudget);
    expect(report.subject.label).toBe('base');
    expect(report.fixtures[0]?.observedP95Ms).toBe(2);
  });

  test('a paired role naming an unknown subject throws', () => {
    const broken = rawStats([fixture('card', [1])]);
    const withBadRole = {
      ...broken,
      fixtures: [
        { ...broken.fixtures[0]!, paired: { ...broken.fixtures[0]!.paired!, candidate: 'ghost' } },
      ],
    };
    expect(() => evaluateBudget(withBadRole, budget([entry('card', 2)]))).toThrow(
      /has no such subject/
    );
  });

  test('two subjects with no paired roles refuse to guess', () => {
    const paired = fixture('card', [1]);
    const unpaired = {
      ...rawStats([paired]),
      fixtures: [{ ...paired, paired: undefined }],
    };
    expect(() => evaluateBudget(unpaired, budget([entry('card', 2)]))).toThrow(/no paired roles/);
  });

  test('single-subject historical raw stats are accepted', () => {
    const paired = fixture('card', [1]);
    const single = {
      ...rawStats([paired]),
      subjects: [CANDIDATE],
      fixtures: [
        {
          ...paired,
          rounds: paired.rounds.map(round => ({
            round: round.round,
            subjectOrder: [CANDIDATE.label],
            perSubject: { [CANDIDATE.label]: round.perSubject[CANDIDATE.label]! },
          })),
        },
      ],
    };
    const report = evaluateBudget(single, budget([entry('card', 2)]));
    expect(report.subject.label).toBe('candidate');
    expect(report.status).toBe('pass');
  });
});

describe('evaluateBudget — malformed input', () => {
  test('a non-finite p95 throws instead of passing', () => {
    const paired = fixture('card', [1]);
    const broken = {
      ...rawStats([paired]),
      fixtures: [
        {
          ...paired,
          rounds: paired.rounds.map(round => ({
            round: round.round,
            subjectOrder: round.subjectOrder,
            perSubject: {
              [BASE.label]: round.perSubject[BASE.label]!,
              [CANDIDATE.label]: { ...round.perSubject[CANDIDATE.label]!, p95: Number.NaN },
            },
          })),
        },
      ],
    };
    expect(() => evaluateBudget(broken, budget([entry('card', 2)]))).toThrow(/p95/);
  });

  test('missing samples for the selected subject throw', () => {
    const paired = fixture('card', [1]);
    // The candidate is declared as a subject but never measured — the
    // validator must reject this rather than skip the fixture.
    const broken = {
      ...rawStats([paired]),
      fixtures: [
        {
          ...paired,
          rounds: paired.rounds.map(round => ({
            round: round.round,
            subjectOrder: round.subjectOrder,
            perSubject: { [BASE.label]: round.perSubject[BASE.label]! },
          })),
        },
      ],
    };
    expect(() => evaluateBudget(broken, budget([entry('card', 2)]))).toThrow(
      /must be an object|no samples/
    );
  });

  test('an unsupported raw-stats schema version throws', () => {
    const broken = { ...rawStats([fixture('card', [1])]), schemaVersion: 2 };
    expect(() => evaluateBudget(broken, budget([entry('card', 2)]))).toThrow(/schemaVersion 2/);
  });
});

describe('parseBudget', () => {
  test('an unsupported budget schema version throws', () => {
    expect(() =>
      parseBudget({ ...budget([entry('card', 2)]), schemaVersion: 9 }, 'budget')
    ).toThrow(/schemaVersion 9/);
  });

  test('an unknown state throws', () => {
    expect(() => parseBudget({ ...budget([]), state: 'whatever' }, 'budget')).toThrow(
      /state must be/
    );
  });

  test('an unknown statistic throws', () => {
    expect(() => parseBudget({ ...budget([]), statistic: 'mean' }, 'budget')).toThrow(
      /statistic must be/
    );
  });

  test('a non-finite ceiling throws', () => {
    expect(() =>
      parseBudget(
        { ...budget([{ ...entry('card', 2), ceilingMs: Number.POSITIVE_INFINITY }]) },
        'budget'
      )
    ).toThrow(/ceilingMs must be a positive finite number/);
  });

  test('a ceiling below its observed upper bound throws', () => {
    expect(() =>
      parseBudget({ ...budget([{ ...entry('card', 2), observedUpperMs: 3 }]) }, 'budget')
    ).toThrow(/must not be below the observed upper bound/);
  });

  test('a ceiling seeded from too few runs throws', () => {
    expect(() => parseBudget({ ...budget([{ ...entry('card', 2), runs: 1 }]) }, 'budget')).toThrow(
      /repeated clean runs/
    );
  });

  test('duplicate entry names throw', () => {
    expect(() =>
      parseBudget({ ...budget([entry('card', 2), entry('card', 3)]) }, 'budget')
    ).toThrow(/names must be unique/);
  });

  test('entries are rejected while the budget is pending calibration', () => {
    expect(() =>
      parseBudget({ ...budget([entry('card', 2)]), state: 'pending-calibration' }, 'budget')
    ).toThrow(/must be empty/);
  });

  test('an enforced budget with no entries throws', () => {
    expect(() => parseBudget({ ...budget([]), state: 'enforced' }, 'budget')).toThrow(
      /must not be empty/
    );
  });

  test('a ceiling that is not the observed bound times the headroom throws', () => {
    expect(() =>
      parseBudget({ ...budget([{ ...entry('card', 2), headroom: 1.05 }]) }, 'budget')
    ).toThrow(/must equal observedUpperMs \* headroom/);
  });

  test('rounding a ceiling within tolerance is accepted', () => {
    const rounded = { ...entry('card', 2), observedUpperMs: 1.6, headroom: 1.25, ceilingMs: 2.01 };
    expect(parseBudget({ ...budget([rounded]) }, 'budget').entries[0]?.ceilingMs).toBe(2.01);
  });

  test('an enforced budget must pin at least one runner image build', () => {
    const unpinned = budget([entry('card', 2)]);
    expect(() =>
      parseBudget(
        { ...unpinned, canonical: { ...unpinned.canonical, runnerImageVersions: [] } },
        'budget'
      )
    ).toThrow(/must pin at least one image build/);
  });

  test('a missing policy block throws rather than being ignored', () => {
    const withoutPolicy: Record<string, unknown> = { ...budget([]) };
    delete withoutPolicy.policy;
    expect(() => parseBudget(withoutPolicy, 'budget')).toThrow(/policy must be an object/);
  });

  test('a malformed reviewedAt date throws', () => {
    expect(() =>
      parseBudget({ ...budget([{ ...entry('card', 2), reviewedAt: 'yesterday' }]) }, 'budget')
    ).toThrow(/ISO date/);
  });

  test('the committed budget.json parses', () => {
    const budgetPath = path.join(path.dirname(fileURLToPath(import.meta.url)), '..', 'budget.json');
    const committed = parseBudget(JSON.parse(fs.readFileSync(budgetPath, 'utf8')), 'budget.json');
    expect(committed.canonical.target).toBe('x86_64-unknown-linux-gnu');
    expect(committed.canonical.node).toBe('v24.18.0');
    expect(committed.subject).toBe('candidate');
    expect(committed.policy.automation).toMatch(/never/i);
  });
});

describe('renderBudgetMarkdown', () => {
  test('escapes benchmark names that would break the table', () => {
    const report = evaluateBudget(
      rawStats([fixture('weird | `name`', [1])]),
      budget([entry('weird | `name`', 2)])
    );
    const markdown = renderBudgetMarkdown(report);
    expect(markdown).toContain('weird \\| \\`name\\`');
    expect(markdown).toContain('| 1.0000 | 2.0000 | 50.0% | pass |');
  });

  test('lists problems and the measured environment', () => {
    const report = evaluateBudget(
      rawStats([fixture('card', [5])], { ...CANONICAL_ENV, runnerImage: 'ubuntu26' }),
      budget([entry('card', 2)])
    );
    const markdown = renderBudgetMarkdown(report);
    expect(markdown).toContain('Status: **FAIL**');
    expect(markdown).toContain('image ubuntu26 @ 20260803.1.0');
    expect(markdown).toContain('`breach`');
    expect(markdown).toContain('`environment-runner-image`');
  });

  test('escapeMarkdownCell strips control characters', () => {
    expect(escapeMarkdownCell('a\u0007b')).toBe('a b');
  });
});
