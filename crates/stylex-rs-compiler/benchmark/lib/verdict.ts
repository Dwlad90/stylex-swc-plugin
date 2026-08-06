/**
 * Pure verdict engine over validated `raw-stats.v1.json`.
 *
 * For each fixture the engine:
 *   1. rebuilds per-round `candidate_p50 / base_p50` ratios;
 *   2. runs the deterministic median-ratio bootstrap from `stats.ts`;
 *   3. classifies the fixture against calibrated thresholds — a warn
 *      band (default 1.10), a fail band (default 1.20), and an
 *      improbable-improvement upper bound (default 0.50).
 *
 * The engine is deliberately I/O-free. `compare-revisions.ts` handles
 * argument parsing, artifact writing, and the targeted-retry loop.
 * A single-fixture reproduced fail block turns into a suite failure —
 * verdict evaluation itself never re-measures.
 */

import { escapeMarkdownCell } from './format.js';
import { parseRawStats } from './raw-stats.js';
import { bootstrapMedianRatio, ensureFinitePositive, median, roundRatios } from './stats.js';
import type {
  BootstrapConfig,
  BootstrapInterval,
  FixtureRawStats,
  SubjectDescriptor,
} from './types.js';

export const VERDICT_SCHEMA_VERSION = 1 as const;

export interface VerdictThresholds {
  /** Fixture is warned when the lower CI bound is at least this ratio. */
  warn: number;
  /** Fixture is flagged (retry candidate) at or above this ratio. */
  fail: number;
  /**
   * Fixture is warned when the upper CI bound is at most this ratio.
   * Guards against a broken benchmark that "improved" impossibly fast.
   */
  improvementWarn: number;
}

export const DEFAULT_THRESHOLDS: VerdictThresholds = {
  warn: 1.1,
  fail: 1.2,
  improvementWarn: 0.5,
};

export type FixtureStatus = 'pass' | 'warn' | 'improvement-warn' | 'flagged' | 'failed';

export type SuiteStatus = 'pass' | 'flagged' | 'failed';

export interface FixtureVerdict {
  name: string;
  category: FixtureRawStats['category'];
  weight: FixtureRawStats['weight'];
  batchSize: number;
  base: {
    label: string;
    perRoundP50: readonly number[];
  };
  candidate: {
    label: string;
    perRoundP50: readonly number[];
  };
  ratios: readonly number[];
  interval: BootstrapInterval;
  retryInterval?: BootstrapInterval;
  status: FixtureStatus;
  messages: readonly string[];
}

export interface VerdictReport {
  schemaVersion: typeof VERDICT_SCHEMA_VERSION;
  suiteStatus: SuiteStatus;
  thresholds: VerdictThresholds;
  bootstrap: BootstrapConfig;
  subjects: {
    base: SubjectDescriptor;
    candidate: SubjectDescriptor;
  };
  fixtures: readonly FixtureVerdict[];
  /** Fixture names flagged by the primary evaluation. */
  flagged: readonly string[];
  /** True when at least one flagged fixture reproduces the failure. */
  hasReproducedFailure: boolean;
}

export interface EvaluateOptions {
  thresholds?: VerdictThresholds;
  bootstrap: BootstrapConfig;
  /** Optional retry raw stats (same schema, same subject labels). */
  retry?: unknown;
}

/**
 * Validate the loaded raw-stats file, then evaluate the paired verdict.
 * Throws when the file cannot be interpreted (schema/subject mismatch,
 * malformed rounds, missing fixtures) — these are hard errors that must
 * fail loudly instead of appearing as a benign "pass".
 */
export function evaluateRawStats(primaryInput: unknown, options: EvaluateOptions): VerdictReport {
  const primary = parseRawStats(primaryInput, 'primary raw stats');
  const [base, candidate] = primary.subjects;
  if (!base || !candidate) {
    throw new Error('Raw stats must expose exactly two subjects (base, candidate)');
  }

  const thresholds = options.thresholds ?? DEFAULT_THRESHOLDS;
  validateThresholds(thresholds);
  const fixtures: FixtureVerdict[] = [];
  const flagged: string[] = [];

  for (const rawFixture of primary.fixtures) {
    const verdict = evaluateFixture(rawFixture, base, candidate, thresholds, options.bootstrap);
    if (verdict.status === 'flagged') {
      flagged.push(verdict.name);
    }
    fixtures.push(verdict);
  }

  let hasReproducedFailure = false;
  if (options.retry !== undefined) {
    const retry = parseRawStats(options.retry, 'retry raw stats');
    validateRetry(primary.fixtures, retry.fixtures, flagged, base, candidate, retry.subjects);
    const retryByName = new Map(retry.fixtures.map(fixture => [fixture.name, fixture]));

    for (const verdict of fixtures) {
      if (verdict.status !== 'flagged') continue;
      const retryFixture = retryByName.get(verdict.name);
      if (!retryFixture)
        throw new Error(`Retry raw stats is missing flagged fixture "${verdict.name}"`);
      const retryVerdict = evaluateFixture(
        retryFixture,
        base,
        candidate,
        thresholds,
        options.bootstrap
      );
      verdict.retryInterval = retryVerdict.interval;
      if (retryVerdict.interval.lower >= thresholds.fail) {
        verdict.status = 'failed';
        verdict.messages = [
          ...verdict.messages,
          `retry reproduced lower bound ${retryVerdict.interval.lower.toFixed(3)} >= ${thresholds.fail.toFixed(2)}`,
        ];
        hasReproducedFailure = true;
      } else {
        verdict.status = 'pass';
        verdict.messages = [
          ...verdict.messages,
          `retry lower bound ${retryVerdict.interval.lower.toFixed(3)} did not reproduce the breach`,
        ];
      }
    }
  }

  const suiteStatus: SuiteStatus = hasReproducedFailure
    ? 'failed'
    : flagged.length > 0 && options.retry === undefined
      ? 'flagged'
      : 'pass';

  return {
    schemaVersion: VERDICT_SCHEMA_VERSION,
    suiteStatus,
    thresholds,
    bootstrap: options.bootstrap,
    subjects: { base, candidate },
    fixtures,
    flagged,
    hasReproducedFailure,
  };
}

function evaluateFixture(
  fixture: FixtureRawStats,
  base: SubjectDescriptor,
  candidate: SubjectDescriptor,
  thresholds: VerdictThresholds,
  bootstrap: BootstrapConfig
): FixtureVerdict {
  if (fixture.rounds.length === 0) {
    throw new Error(`Fixture "${fixture.name}" has no rounds`);
  }

  const basePerRound: number[] = [];
  const candidatePerRound: number[] = [];

  for (const round of fixture.rounds) {
    const baseSamples = round.perSubject[base.label];
    const candidateSamples = round.perSubject[candidate.label];
    if (!baseSamples || !candidateSamples) {
      throw new Error(
        `Fixture "${fixture.name}" round ${round.round} is missing samples for ${base.label} or ${candidate.label}`
      );
    }
    ensureFinitePositive(fixture.name, `${base.label}.p50`, baseSamples.p50);
    ensureFinitePositive(fixture.name, `${candidate.label}.p50`, candidateSamples.p50);
    basePerRound.push(baseSamples.p50);
    candidatePerRound.push(candidateSamples.p50);
  }

  const ratios = roundRatios(basePerRound, candidatePerRound);
  const interval = bootstrapMedianRatio(ratios, bootstrap);
  // Prefer the observed round-ratio median for `point` — the bootstrap point
  // already returns this, but assert it stays coherent with `median(ratios)`.
  ensureFinitePositive(fixture.name, 'ratio.point', interval.point);
  const observedMedian = median(ratios);

  const messages: string[] = [];
  let status: FixtureStatus = 'pass';

  if (interval.lower >= thresholds.fail) {
    status = 'flagged';
    messages.push(
      `lower bound ${interval.lower.toFixed(3)} >= ${thresholds.fail.toFixed(2)} — retry required`
    );
  } else if (interval.lower >= thresholds.warn) {
    status = 'warn';
    messages.push(`lower bound ${interval.lower.toFixed(3)} >= ${thresholds.warn.toFixed(2)}`);
  }

  if (interval.upper <= thresholds.improvementWarn) {
    // Improbable improvement never blocks, but it should not silently
    // downgrade a regression flag either.
    if (status === 'pass') status = 'improvement-warn';
    messages.push(
      `upper bound ${interval.upper.toFixed(3)} <= ${thresholds.improvementWarn.toFixed(2)} — possible broken benchmark`
    );
  }

  return {
    name: fixture.name,
    category: fixture.category,
    weight: fixture.weight,
    batchSize: fixture.batchSize,
    base: { label: base.label, perRoundP50: basePerRound },
    candidate: { label: candidate.label, perRoundP50: candidatePerRound },
    ratios,
    interval: { point: observedMedian, lower: interval.lower, upper: interval.upper },
    status,
    messages,
  };
}

function validateThresholds(thresholds: VerdictThresholds): void {
  for (const [name, value] of Object.entries(thresholds)) {
    ensureFinitePositive('thresholds', name, value);
  }
  if (thresholds.warn > thresholds.fail) {
    throw new Error('Warning threshold must not exceed the failure threshold');
  }
}

function validateRetry(
  primaryFixtures: readonly FixtureRawStats[],
  retryFixtures: readonly FixtureRawStats[],
  flagged: readonly string[],
  base: SubjectDescriptor,
  candidate: SubjectDescriptor,
  retrySubjects: readonly SubjectDescriptor[]
): void {
  if (!sameSubject(retrySubjects[0], base) || !sameSubject(retrySubjects[1], candidate)) {
    throw new Error('Retry raw stats subjects must match the primary base/candidate identities');
  }

  const flaggedSet = new Set(flagged);
  for (const fixture of retryFixtures) {
    if (!flaggedSet.has(fixture.name)) {
      throw new Error(`Retry raw stats contains non-flagged fixture "${fixture.name}"`);
    }
  }
  for (const name of flagged) {
    const primary = primaryFixtures.find(fixture => fixture.name === name);
    const retry = retryFixtures.find(fixture => fixture.name === name);
    if (!retry) throw new Error(`Retry raw stats is missing flagged fixture "${name}"`);
    if (!primary) throw new Error(`Primary raw stats is missing flagged fixture "${name}"`);
    if (retry.rounds.length !== primary.rounds.length) {
      throw new Error(
        `Retry fixture "${name}" must contain ${primary.rounds.length} rounds (received ${retry.rounds.length})`
      );
    }
    if (
      retry.category !== primary.category ||
      retry.weight !== primary.weight ||
      retry.batchSize !== primary.batchSize
    ) {
      throw new Error(`Retry fixture "${name}" metadata must match the primary fixture`);
    }
  }
}

function sameSubject(actual: SubjectDescriptor | undefined, expected: SubjectDescriptor): boolean {
  return (
    actual?.label === expected.label &&
    actual.version === expected.version &&
    actual.resolvedFrom === expected.resolvedFrom
  );
}

export { escapeMarkdownCell } from './format.js';

const STATUS_LABEL: Record<FixtureStatus, string> = {
  pass: 'pass',
  warn: 'warn',
  'improvement-warn': 'improvement-warn',
  flagged: 'flagged (retry)',
  failed: 'FAIL',
};

/** Render the verdict as a Markdown table body suitable for `GITHUB_STEP_SUMMARY`. */
export function renderVerdictMarkdown(report: VerdictReport): string {
  const header = [
    `## Paired revision benchmark: ${escapeMarkdownCell(report.subjects.base.label)} vs ${escapeMarkdownCell(report.subjects.candidate.label)}`,
    '',
    `Suite status: **${STATUS_LABEL[report.suiteStatus]}**`,
    `Thresholds: warn \\>= ${report.thresholds.warn.toFixed(2)}, fail \\>= ${report.thresholds.fail.toFixed(2)}, improvement \\<= ${report.thresholds.improvementWarn.toFixed(2)}`,
    '',
    '| Fixture | Category | Point | Lower | Upper | Status | Notes |',
    '| --- | --- | --- | --- | --- | --- | --- |',
  ];

  const rows = report.fixtures.map(fixture => {
    const notes = fixture.messages.length > 0 ? fixture.messages.join('; ') : '';
    return `| ${escapeMarkdownCell(fixture.name)} | ${escapeMarkdownCell(fixture.category)} | ${fixture.interval.point.toFixed(3)} | ${fixture.interval.lower.toFixed(3)} | ${fixture.interval.upper.toFixed(3)} | ${escapeMarkdownCell(STATUS_LABEL[fixture.status])} | ${escapeMarkdownCell(notes)} |`;
  });

  return [...header, ...rows, ''].join('\n');
}
