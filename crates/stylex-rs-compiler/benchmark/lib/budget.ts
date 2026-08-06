/**
 * Absolute per-benchmark p95 budget.
 *
 * The paired verdict engine only sees base-versus-candidate ratios, so a
 * long series of individually-tolerable releases can erode performance
 * without ever tripping it. This layer compares measured p95 latency
 * against committed ceilings on one canonical target.
 *
 * Canonical environment: `x86_64-unknown-linux-gnu` on Node 24.18.0. The
 * runner stays `ubuntu-latest` by decision, so the comparison is only
 * valid while the runner image matches the image the ceilings were seeded
 * on. An image change is an explicit recalibration failure, never a silent
 * comparison against numbers from a different machine class. CPU model
 * variation cannot be pinned on hosted runners; it is recorded in the
 * report as a diagnostic and must be spread across the seeding runs.
 *
 * Statistic: the median of per-round p95 values for the selected subject.
 * Per-round p95 is already normalised to latency per transform by the
 * runner, and taking the median across rounds keeps one noisy round from
 * deciding a release.
 *
 * This module is pure. It never writes and never mutates the budget —
 * ceilings only move through a reviewed change (see `budget.json`).
 */

import { escapeMarkdownCell } from './format.js';
import {
  assertUnique,
  requireArray,
  requireIsoDate,
  requirePositiveInteger,
  requirePositiveNumber,
  requireRecord,
  requireString,
} from './json.js';
import { parseRawStats } from './raw-stats.js';
import { median } from './stats.js';
import type {
  FixtureRawStats,
  RawStatsEnvironment,
  RawStatsFile,
  SubjectDescriptor,
} from './types.js';

export const BUDGET_SCHEMA_VERSION = 1 as const;
export const BUDGET_REPORT_SCHEMA_VERSION = 1 as const;

/**
 * Minimum independent seeding runs behind an enforced ceiling. The policy
 * names no number, only "repeated clean release-style runs" and an explicit
 * ban on "one run plus an arbitrary 25%" — three is the smallest count that
 * cannot be one run wearing a disguise.
 */
export const MIN_SEEDING_RUNS = 3;

/**
 * Relative tolerance when re-deriving `ceilingMs` from
 * `observedUpperMs * headroom`. Loose enough for a reviewer to round a
 * ceiling to a readable number, tight enough that the recorded headroom
 * cannot be decorative.
 */
const CEILING_DERIVATION_TOLERANCE = 0.01;

export type BudgetState = 'pending-calibration' | 'enforced';

export interface BudgetCanonicalEnvironment {
  /** Rust target triple the ceilings were seeded on. */
  target: string;
  /** Exact `process.version` string, e.g. `v24.18.0`. */
  node: string;
  /** GitHub runner label, recorded for auditability. */
  runner: string;
  /**
   * Accepted runner image families (`ImageOS`, e.g. `ubuntu24`). Any other
   * family forces recalibration instead of a comparison.
   */
  runnerImages: readonly string[];
  /**
   * Accepted exact image builds (`ImageVersion`, e.g. `20260803.1.0`).
   * GitHub rebuilds an image family in place, so the family alone cannot
   * tell whether the machine still matches the one the ceilings came from.
   * Empty while ceilings are pending calibration; an enforced budget must
   * pin at least one build.
   */
  runnerImageVersions: readonly string[];
}

/**
 * Human-readable rules that travel with the ceilings. Parsed and required
 * rather than ignored, so the rules cannot silently rot away from the file
 * they govern. Phase 7 expands these into the performance policy document.
 */
export interface BudgetPolicy {
  seeding: string;
  increases: string;
  decreases: string;
  automation: string;
  environment: string;
}

export interface BudgetEntry {
  /** Fixture name exactly as registered in `fixtures.v1.json`. */
  name: string;
  /** Committed ceiling in milliseconds per transform. */
  ceilingMs: number;
  /** Robust upper bound observed across the seeding runs, milliseconds. */
  observedUpperMs: number;
  /** Multiplier applied over `observedUpperMs` to obtain `ceilingMs`. */
  headroom: number;
  /** Number of independent clean runs behind `observedUpperMs`. */
  runs: number;
  /** ISO date of the reviewed change that set this ceiling. */
  reviewedAt: string;
  /** Run ids, PR links, or another auditable pointer to the measurements. */
  evidence: string;
}

export interface BudgetFile {
  schemaVersion: typeof BUDGET_SCHEMA_VERSION;
  /**
   * `pending-calibration` until Phase 0 seeding lands: the check reports
   * observed p95 values so runs can be archived, and never blocks.
   * `enforced` requires one entry per measured fixture and blocks breaches.
   */
  state: BudgetState;
  /** Which raw-stats subject the ceilings describe. */
  subject: 'base' | 'candidate';
  /** Only one statistic is supported today; the field pins it explicitly. */
  statistic: 'median-of-round-p95';
  canonical: BudgetCanonicalEnvironment;
  policy: BudgetPolicy;
  entries: readonly BudgetEntry[];
}

export type BudgetProblemKind =
  | 'environment-target'
  | 'environment-node'
  | 'environment-runner-image'
  | 'environment-runner-image-version'
  | 'missing-entry'
  | 'extra-entry'
  | 'breach';

export interface BudgetProblem {
  kind: BudgetProblemKind;
  message: string;
}

export type BudgetFixtureStatus = 'pass' | 'breach' | 'unbudgeted' | 'unseeded';

export interface BudgetFixtureReport {
  name: string;
  category: FixtureRawStats['category'];
  weight: FixtureRawStats['weight'];
  batchSize: number;
  /** Median of per-round p95, milliseconds per transform. */
  observedP95Ms: number;
  perRoundP95Ms: readonly number[];
  ceilingMs?: number;
  /** `observedP95Ms / ceilingMs`; absent when no ceiling is committed. */
  utilization?: number;
  status: BudgetFixtureStatus;
}

export type BudgetStatus = 'pass' | 'failed' | 'unseeded';

export interface BudgetReport {
  schemaVersion: typeof BUDGET_REPORT_SCHEMA_VERSION;
  status: BudgetStatus;
  budgetState: BudgetState;
  /**
   * True when the caller ran in report-only mode and did not act on a
   * failure. The evaluator always reports the real status and leaves this
   * `false`; only the CLI, which owns the exit code, sets it.
   */
  reportOnly: boolean;
  subject: SubjectDescriptor;
  canonical: BudgetCanonicalEnvironment;
  /** Full measured environment, including CPU model diagnostics. */
  environment: RawStatsEnvironment;
  problems: readonly BudgetProblem[];
  fixtures: readonly BudgetFixtureReport[];
}

export function evaluateBudget(rawStatsInput: unknown, budgetInput: unknown): BudgetReport {
  const raw = parseRawStats(rawStatsInput, 'budget raw stats', { subjects: 'any' });
  const budget = parseBudget(budgetInput, 'budget');
  const subject = selectSubject(raw, budget);

  const problems: BudgetProblem[] = [
    ...checkEnvironment(raw.environment, budget.canonical),
    ...(budget.state === 'enforced' ? checkCoverage(raw.fixtures, budget.entries) : []),
  ];

  const ceilings = new Map(budget.entries.map(entry => [entry.name, entry.ceilingMs]));
  const fixtures = raw.fixtures.map(fixture => {
    const report = measureFixture(fixture, subject.label, budget.state, ceilings.get(fixture.name));
    if (report.status === 'breach' && report.ceilingMs !== undefined) {
      problems.push({
        kind: 'breach',
        message: `${fixture.name}: p95 ${formatMs(report.observedP95Ms)} exceeds ceiling ${formatMs(report.ceilingMs)}`,
      });
    }
    return report;
  });

  // Environment drift fails in either state: a run on the wrong target,
  // Node version, or runner image is neither a valid comparison nor a
  // valid seeding observation. Only the absence of ceilings is excused
  // while calibration is pending.
  const status: BudgetStatus =
    problems.length > 0 ? 'failed' : budget.state === 'pending-calibration' ? 'unseeded' : 'pass';

  return {
    schemaVersion: BUDGET_REPORT_SCHEMA_VERSION,
    status,
    budgetState: budget.state,
    reportOnly: false,
    subject,
    canonical: budget.canonical,
    environment: raw.environment,
    problems,
    fixtures,
  };
}

/**
 * Resolve which subject the ceilings describe.
 *
 * A paired run records the roles explicitly in `fixtures[].paired`, so the
 * budget follows those labels rather than array position — reordering the
 * subjects in `bench-revisions.ts` must not silently switch the budget onto
 * the baseline. Position is only the fallback for a run with no paired
 * block, which is the single-subject historical `bench` output.
 */
function selectSubject(raw: RawStatsFile, budget: BudgetFile): SubjectDescriptor {
  const label = pairedRoleLabel(raw, budget.subject);
  if (label !== undefined) {
    const found = raw.subjects.find(subject => subject.label === label);
    if (!found) {
      throw new Error(`Raw stats declares ${budget.subject} "${label}" but has no such subject`);
    }
    return found;
  }

  if (raw.subjects.length > 1) {
    throw new Error(
      `Raw stats has ${String(raw.subjects.length)} subjects but no paired roles — cannot tell which one the budget applies to`
    );
  }
  const only = raw.subjects[0];
  if (!only) throw new Error('Raw stats has no subjects');
  return only;
}

function pairedRoleLabel(raw: RawStatsFile, role: 'base' | 'candidate'): string | undefined {
  const labels = new Set(
    raw.fixtures.map(fixture => fixture.paired?.[role]).filter(label => label !== undefined)
  );
  if (labels.size === 0) return undefined;
  if (labels.size > 1) {
    throw new Error(`Raw stats fixtures disagree on which subject is the ${role}`);
  }
  return [...labels][0];
}

function checkEnvironment(
  environment: RawStatsEnvironment,
  canonical: BudgetCanonicalEnvironment
): BudgetProblem[] {
  const problems: BudgetProblem[] = [];
  if (environment.target !== canonical.target) {
    problems.push({
      kind: 'environment-target',
      message: `budget applies to target ${canonical.target}, measured ${environment.target}`,
    });
  }
  if (environment.node !== canonical.node) {
    problems.push({
      kind: 'environment-node',
      message: `budget applies to Node ${canonical.node}, measured ${environment.node}`,
    });
  }
  if (environment.runnerImage === undefined) {
    problems.push({
      kind: 'environment-runner-image',
      message: `raw stats records no runner image; ceilings seeded on ${canonical.runnerImages.join(', ')} cannot be compared`,
    });
  } else if (!canonical.runnerImages.includes(environment.runnerImage)) {
    problems.push({
      kind: 'environment-runner-image',
      message: `runner image drifted to ${environment.runnerImage} (seeded on ${canonical.runnerImages.join(', ')}) — recalibration required`,
    });
  }

  // The image family stays `ubuntu24` across rebuilds that can move
  // timings, so an enforced budget also pins the exact build.
  if (canonical.runnerImageVersions.length > 0) {
    if (environment.runnerImageVersion === undefined) {
      problems.push({
        kind: 'environment-runner-image-version',
        message: `raw stats records no runner image version; ceilings seeded on ${canonical.runnerImageVersions.join(', ')} cannot be compared`,
      });
    } else if (!canonical.runnerImageVersions.includes(environment.runnerImageVersion)) {
      problems.push({
        kind: 'environment-runner-image-version',
        message: `runner image rebuilt to ${environment.runnerImageVersion} (seeded on ${canonical.runnerImageVersions.join(', ')}) — recalibration required`,
      });
    }
  }
  return problems;
}

function checkCoverage(
  fixtures: readonly FixtureRawStats[],
  entries: readonly BudgetEntry[]
): BudgetProblem[] {
  const problems: BudgetProblem[] = [];
  const measured = new Set(fixtures.map(fixture => fixture.name));
  const budgeted = new Set(entries.map(entry => entry.name));

  for (const name of measured) {
    if (!budgeted.has(name)) {
      problems.push({ kind: 'missing-entry', message: `no committed ceiling for "${name}"` });
    }
  }
  for (const name of budgeted) {
    if (!measured.has(name)) {
      problems.push({
        kind: 'extra-entry',
        message: `budget entry "${name}" was not measured in this run`,
      });
    }
  }
  return problems;
}

function measureFixture(
  fixture: FixtureRawStats,
  subjectLabel: string,
  state: BudgetState,
  ceilingMs: number | undefined
): BudgetFixtureReport {
  const perRoundP95Ms = fixture.rounds.map(round => {
    const samples = round.perSubject[subjectLabel];
    if (!samples) {
      throw new Error(
        `Fixture "${fixture.name}" round ${String(round.round)} has no samples for subject "${subjectLabel}"`
      );
    }
    if (!Number.isFinite(samples.p95) || samples.p95 <= 0) {
      throw new Error(
        `Fixture "${fixture.name}" round ${String(round.round)} p95 must be a positive finite number`
      );
    }
    return samples.p95;
  });

  const observedP95Ms = median(perRoundP95Ms);
  if (!Number.isFinite(observedP95Ms) || observedP95Ms <= 0) {
    throw new Error(`Fixture "${fixture.name}" produced a non-finite p95 median`);
  }

  const base = {
    name: fixture.name,
    category: fixture.category,
    weight: fixture.weight,
    batchSize: fixture.batchSize,
    observedP95Ms,
    perRoundP95Ms,
  };

  if (ceilingMs === undefined) {
    return { ...base, status: state === 'enforced' ? 'unbudgeted' : 'unseeded' };
  }
  return {
    ...base,
    ceilingMs,
    utilization: observedP95Ms / ceilingMs,
    // The ceiling itself passes: it is the highest accepted latency.
    status: observedP95Ms > ceilingMs ? 'breach' : 'pass',
  };
}

export function parseBudget(value: unknown, source: string): BudgetFile {
  const file = requireRecord(value, source);
  if (file.schemaVersion !== BUDGET_SCHEMA_VERSION) {
    throw new Error(
      `${source} schemaVersion ${String(file.schemaVersion)} is not supported (expected ${String(BUDGET_SCHEMA_VERSION)})`
    );
  }
  const state = file.state;
  if (state !== 'pending-calibration' && state !== 'enforced') {
    throw new Error(`${source}.state must be "pending-calibration" or "enforced"`);
  }
  const subject = file.subject;
  if (subject !== 'base' && subject !== 'candidate') {
    throw new Error(`${source}.subject must be "base" or "candidate"`);
  }
  if (file.statistic !== 'median-of-round-p95') {
    throw new Error(`${source}.statistic must be "median-of-round-p95"`);
  }

  const entries = requireArray(file.entries, `${source}.entries`).map((entry, index) =>
    parseEntry(entry, `${source}.entries[${index}]`)
  );
  assertUnique(
    entries.map(entry => entry.name),
    `${source}.entries names`
  );
  if (state === 'pending-calibration' && entries.length > 0) {
    throw new Error(`${source}.entries must be empty while state is "pending-calibration"`);
  }
  if (state === 'enforced' && entries.length === 0) {
    throw new Error(`${source}.entries must not be empty while state is "enforced"`);
  }

  const canonical = parseCanonical(file.canonical, `${source}.canonical`);
  if (state === 'enforced' && canonical.runnerImageVersions.length === 0) {
    throw new Error(
      `${source}.canonical.runnerImageVersions must pin at least one image build while state is "enforced"`
    );
  }

  return {
    schemaVersion: BUDGET_SCHEMA_VERSION,
    state,
    subject,
    statistic: 'median-of-round-p95',
    canonical,
    policy: parsePolicy(file.policy, `${source}.policy`),
    entries,
  };
}

function parseCanonical(value: unknown, context: string): BudgetCanonicalEnvironment {
  const canonical = requireRecord(value, context);
  const runnerImages = requireArray(canonical.runnerImages, `${context}.runnerImages`).map(
    (image, index) => requireString(image, `${context}.runnerImages[${index}]`)
  );
  if (runnerImages.length === 0) {
    throw new Error(`${context}.runnerImages must list at least one image`);
  }
  return {
    target: requireString(canonical.target, `${context}.target`),
    node: requireString(canonical.node, `${context}.node`),
    runner: requireString(canonical.runner, `${context}.runner`),
    runnerImages,
    runnerImageVersions: requireArray(
      canonical.runnerImageVersions,
      `${context}.runnerImageVersions`
    ).map((version, index) => requireString(version, `${context}.runnerImageVersions[${index}]`)),
  };
}

function parsePolicy(value: unknown, context: string): BudgetPolicy {
  const policy = requireRecord(value, context);
  return {
    seeding: requireString(policy.seeding, `${context}.seeding`),
    increases: requireString(policy.increases, `${context}.increases`),
    decreases: requireString(policy.decreases, `${context}.decreases`),
    automation: requireString(policy.automation, `${context}.automation`),
    environment: requireString(policy.environment, `${context}.environment`),
  };
}

function parseEntry(value: unknown, context: string): BudgetEntry {
  const entry = requireRecord(value, context);
  const ceilingMs = requirePositiveNumber(entry.ceilingMs, `${context}.ceilingMs`);
  const observedUpperMs = requirePositiveNumber(
    entry.observedUpperMs,
    `${context}.observedUpperMs`
  );
  const headroom = requirePositiveNumber(entry.headroom, `${context}.headroom`);
  const runs = requirePositiveInteger(entry.runs, `${context}.runs`);

  if (headroom <= 1) throw new Error(`${context}.headroom must be greater than 1`);
  if (ceilingMs < observedUpperMs) {
    throw new Error(`${context}.ceilingMs must not be below the observed upper bound`);
  }
  // A ceiling must be the recorded measurement times the recorded headroom,
  // otherwise both fields are decoration around a number someone picked.
  const derived = observedUpperMs * headroom;
  if (Math.abs(ceilingMs - derived) > derived * CEILING_DERIVATION_TOLERANCE) {
    throw new Error(
      `${context}.ceilingMs must equal observedUpperMs * headroom (${derived.toPrecision(6)}), received ${ceilingMs.toPrecision(6)}`
    );
  }
  if (runs < MIN_SEEDING_RUNS) {
    throw new Error(
      `${context}.runs must be at least ${String(MIN_SEEDING_RUNS)} — seed ceilings from repeated clean runs`
    );
  }

  return {
    name: requireString(entry.name, `${context}.name`),
    ceilingMs,
    observedUpperMs,
    headroom,
    runs,
    reviewedAt: requireIsoDate(entry.reviewedAt, `${context}.reviewedAt`),
    evidence: requireString(entry.evidence, `${context}.evidence`),
  };
}

function formatMs(value: number): string {
  return `${value.toFixed(4)} ms`;
}

/** `ubuntu24 @ 20260803.1.0`, or just the family while builds are unpinned. */
export function describeImages(canonical: BudgetCanonicalEnvironment): string {
  const families = canonical.runnerImages.join(', ');
  if (canonical.runnerImageVersions.length === 0) return `${families} @ any build`;
  return `${families} @ ${canonical.runnerImageVersions.join(', ')}`;
}

export function describeMeasuredImage(environment: RawStatsEnvironment): string {
  return `${environment.runnerImage ?? 'unknown'} @ ${environment.runnerImageVersion ?? 'unknown'}`;
}

const STATUS_LABEL: Record<BudgetStatus, string> = {
  pass: 'pass',
  failed: 'FAIL',
  unseeded: 'not enforced (pending calibration)',
};

/** Render the budget report as Markdown suitable for `GITHUB_STEP_SUMMARY`. */
export function renderBudgetMarkdown(report: BudgetReport): string {
  const lines = [
    `## Absolute p95 budget: ${escapeMarkdownCell(report.subject.label)}`,
    '',
    `Status: **${STATUS_LABEL[report.status]}**${report.reportOnly ? ' (report-only)' : ''}`,
    `Canonical: ${escapeMarkdownCell(report.canonical.target)}, Node ${escapeMarkdownCell(report.canonical.node)}, ${escapeMarkdownCell(report.canonical.runner)} (${escapeMarkdownCell(describeImages(report.canonical))})`,
    `Measured: ${escapeMarkdownCell(report.environment.target)}, Node ${escapeMarkdownCell(report.environment.node)}, image ${escapeMarkdownCell(describeMeasuredImage(report.environment))}, CPU ${escapeMarkdownCell(report.environment.cpu.model)}`,
    '',
    '| Benchmark | p95 (ms) | Ceiling (ms) | Used | Status |',
    '| --- | --- | --- | --- | --- |',
  ];

  for (const fixture of report.fixtures) {
    const ceiling = fixture.ceilingMs === undefined ? '—' : fixture.ceilingMs.toFixed(4);
    const used =
      fixture.utilization === undefined ? '—' : `${(fixture.utilization * 100).toFixed(1)}%`;
    lines.push(
      `| ${escapeMarkdownCell(fixture.name)} | ${fixture.observedP95Ms.toFixed(4)} | ${ceiling} | ${used} | ${escapeMarkdownCell(fixture.status)} |`
    );
  }

  if (report.problems.length > 0) {
    lines.push('', '### Problems', '');
    for (const problem of report.problems) {
      lines.push(
        `- \`${escapeMarkdownCell(problem.kind)}\`: ${escapeMarkdownCell(problem.message)}`
      );
    }
  }

  return `${lines.join('\n')}\n`;
}
