/**
 * Verdict entry point for the paired PR and release gates.
 *
 * Reads validated `raw-stats.v1.json`, evaluates the calibrated
 * median-ratio bootstrap, retries only flagged fixtures once, writes a
 * versioned verdict JSON plus a Markdown summary, and only after the
 * artifacts land exits non-zero when the suite fails.
 *
 * Retry semantics — matching the plan:
 *   - primary pass ......................... exit 0
 *   - primary flagged, retry does not
 *     reproduce ............................. exit 0
 *   - primary flagged, retry
 *     any reproduced breach ................ exit 1
 *
 * Warnings never block. Impossible-improvement warnings surface but do
 * not gate publication.
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

import chalk from 'chalk';

import {
  appendStepSummary,
  errorMessage,
  escapeFailureMessage,
  findArgument,
  isMainModule,
  parseConfidence,
  parsePositiveFloat,
  parsePositiveInt,
  writeArtifact,
} from './lib/cli.js';
import {
  createPairedBenchConfigs,
  createStylexOptions,
  DEFAULT_PAIRED_TIME_BUDGET_MS,
} from './lib/config.js';
import { captureEnvironment } from './lib/env.js';
import { loadAllFixtures } from './lib/fixtures.js';
import { parseRawStats } from './lib/raw-stats.js';
import { runRounds } from './lib/runner.js';
import { loadSubject } from './lib/subjects.js';
import { RAW_STATS_SCHEMA_VERSION, type BootstrapConfig, type RawStatsFile } from './lib/types.js';
import {
  DEFAULT_THRESHOLDS,
  evaluateRawStats,
  renderVerdictMarkdown,
  reproducedFailures,
  VERDICT_SCHEMA_VERSION,
  type SuiteStatus,
  type VerdictReport,
  type VerdictThresholds,
} from './lib/verdict.js';

interface CliOptions {
  primary: string;
  retry: string | undefined;
  outputJson: string;
  summaryMarkdown: string;
  retryOutput: string;
  retrySeed: number;
  retryTimeBudgetMs: number;
  thresholds: VerdictThresholds;
  bootstrap: BootstrapConfig;
}

const EXIT_PASS = 0;
const EXIT_FAILED = 1;

const benchmarkDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(benchmarkDir, '..');
const workspaceRoot = path.resolve(packageDir, '../..');

export interface RetryRequest {
  primary: RawStatsFile;
  fixtureNames: readonly string[];
  rounds: number;
  seed: number;
  timeBudgetMs: number;
  bootstrap: BootstrapConfig;
  outputPath: string;
}

export type RetryRunner = (request: RetryRequest) => Promise<RawStatsFile>;

interface ComparisonOptions {
  thresholds: VerdictThresholds;
  bootstrap: BootstrapConfig;
  retry?: unknown;
  retryOutput: string;
  retrySeed: number;
  retryTimeBudgetMs: number;
}

async function main(): Promise<number> {
  const options = parseCli(process.argv.slice(2));

  const primary = readRawStats(options.primary);
  const retry = options.retry ? readRawStats(options.retry) : undefined;
  const report = await runComparison(primary, {
    thresholds: options.thresholds,
    bootstrap: options.bootstrap,
    retry,
    retryOutput: options.retryOutput,
    retrySeed: options.retrySeed,
    retryTimeBudgetMs: options.retryTimeBudgetMs,
  });

  writeArtifacts(options, report);
  printSummary(report);

  return exitCodeFor(report.suiteStatus);
}

/**
 * Every suite status this gate can exit on.
 *
 * `Record<SuiteStatus, number>` is what makes this exhaustive: adding a status
 * to the engine is a type error here rather than a silent zero.
 *
 * `flagged` should not reach here -- `runComparison` always resolves a flagged
 * primary through the targeted retry, which turns every flagged fixture into
 * `pass` or `failed`. If it does, the retry silently did not run and the gate
 * has no evidence either way, so it fails rather than publishing an unresolved
 * breach as a pass.
 */
const EXIT_CODE_BY_SUITE_STATUS: Record<SuiteStatus, number> = {
  pass: EXIT_PASS,
  flagged: EXIT_FAILED,
  failed: EXIT_FAILED,
};

function exitCodeFor(suiteStatus: SuiteStatus): number {
  if (suiteStatus === 'flagged') {
    console.error(
      chalk.red(
        'Suite status is "flagged": the targeted retry did not resolve a detected ' +
          'breach. Refusing to report a pass.'
      )
    );
  }
  return EXIT_CODE_BY_SUITE_STATUS[suiteStatus];
}

export async function runComparison(
  primaryInput: unknown,
  options: ComparisonOptions,
  retryRunner: RetryRunner = runTargetedRetry
): Promise<VerdictReport> {
  const primary = parseRawStats(primaryInput, 'primary raw stats');
  const initialReport = evaluateRawStats(primary, {
    thresholds: options.thresholds,
    bootstrap: options.bootstrap,
  });
  if (initialReport.flagged.length === 0) {
    if (options.retry !== undefined) {
      throw new Error('Retry raw stats were supplied, but the primary run has no flagged fixtures');
    }
    return initialReport;
  }

  const roundCounts = new Set(
    primary.fixtures
      .filter(fixture => initialReport.flagged.includes(fixture.name))
      .map(fixture => fixture.rounds.length)
  );
  if (roundCounts.size !== 1) {
    throw new Error('Flagged primary fixtures must use the same calibrated round count');
  }
  const rounds = [...roundCounts][0];
  if (rounds === undefined) throw new Error('Flagged primary fixtures have no rounds');

  const retry =
    options.retry !== undefined
      ? options.retry
      : await retryRunner({
          primary,
          fixtureNames: initialReport.flagged,
          rounds,
          seed: options.retrySeed,
          timeBudgetMs: options.retryTimeBudgetMs,
          bootstrap: options.bootstrap,
          outputPath: options.retryOutput,
        });

  return evaluateRawStats(primary, {
    thresholds: options.thresholds,
    bootstrap: options.bootstrap,
    retry,
  });
}

function readRawStats(filePath: string): unknown {
  const resolved = path.resolve(filePath);
  const contents = fs.readFileSync(resolved, 'utf8');
  return JSON.parse(contents) as unknown;
}

async function runTargetedRetry(request: RetryRequest): Promise<RawStatsFile> {
  const [baseDescriptor, candidateDescriptor] = request.primary.subjects;
  if (!baseDescriptor || !candidateDescriptor) {
    throw new Error('Primary raw stats must contain base and candidate subjects');
  }

  const base = await loadSubject({
    label: baseDescriptor.label,
    packageDir: packageDirectoryFor(baseDescriptor.resolvedFrom),
  });
  const candidate = await loadSubject({
    label: candidateDescriptor.label,
    packageDir: packageDirectoryFor(candidateDescriptor.resolvedFrom),
  });
  const requested = new Set(request.fixtureNames);
  const fixtures = loadAllFixtures({ packageDir, workspaceRoot }).filter(fixture =>
    requested.has(fixture.name)
  );
  if (fixtures.length !== requested.size) {
    const loaded = new Set(fixtures.map(fixture => fixture.name));
    const missing = request.fixtureNames.filter(name => !loaded.has(name));
    throw new Error(`Targeted retry fixtures not found: ${missing.join(', ')}`);
  }

  const benchConfigs = createPairedBenchConfigs(request.timeBudgetMs);
  const result = await runRounds({
    subjects: [base, candidate],
    fixtures,
    stylexOptions: createStylexOptions(packageDir),
    rounds: request.rounds,
    seed: request.seed,
    standardBench: benchConfigs.standard,
    heavyBench: benchConfigs.heavy,
    bootstrap: request.bootstrap,
  });
  const retry: RawStatsFile = {
    schemaVersion: RAW_STATS_SCHEMA_VERSION,
    environment: captureEnvironment({ packageDir, workspaceRoot }),
    subjects: [base.descriptor, candidate.descriptor],
    bootstrap: request.bootstrap,
    fixtures: result.fixtures,
  };

  const outputPath = path.resolve(request.outputPath);
  fs.mkdirSync(path.dirname(outputPath), { recursive: true });
  fs.writeFileSync(outputPath, `${JSON.stringify(retry, null, 2)}\n`, 'utf8');
  return retry;
}

function packageDirectoryFor(resolvedFrom: string): string {
  if (
    path.basename(resolvedFrom) !== 'index.js' ||
    path.basename(path.dirname(resolvedFrom)) !== 'dist'
  ) {
    throw new Error(`Subject resolvedFrom is not a dist/index.js entry: ${resolvedFrom}`);
  }
  return path.dirname(path.dirname(resolvedFrom));
}

function writeArtifacts(options: CliOptions, report: VerdictReport): void {
  const markdown = renderVerdictMarkdown(report);
  writeArtifact(options.outputJson, `${JSON.stringify(report, null, 2)}\n`);
  writeArtifact(options.summaryMarkdown, markdown);
  appendStepSummary(markdown);
}

function printSummary(report: VerdictReport): void {
  const base = report.subjects.base.label;
  const candidate = report.subjects.candidate.label;
  console.log(chalk.bold(`\nPaired verdict: ${base} vs ${candidate}`));
  const thresholdSummary = [
    `warn>=${report.thresholds.warn.toFixed(2)}`,
    `fail>=${report.thresholds.fail.toFixed(2)}`,
    `improvement<=${report.thresholds.improvementWarn.toFixed(2)}`,
  ].join(', ');
  console.log(`  thresholds: ${thresholdSummary}`);
  for (const fixture of report.fixtures) {
    const interval = [
      `point=${fixture.interval.point.toFixed(3)}`,
      `lower=${fixture.interval.lower.toFixed(3)}`,
      `upper=${fixture.interval.upper.toFixed(3)}`,
    ].join(' ');
    const line = `  ${fixture.name.padEnd(40)} ${interval} status=${fixture.status}`;
    if (fixture.status === 'failed') console.log(chalk.red(line));
    else if (fixture.status === 'flagged') console.log(chalk.yellow(line));
    else if (fixture.status === 'warn' || fixture.status === 'improvement-warn')
      console.log(chalk.yellow(line));
    else console.log(line);
    for (const message of fixture.messages) console.log(`      ${chalk.gray(message)}`);
  }

  console.log('');
  if (report.suiteStatus === 'failed') {
    console.log(
      chalk.red.bold(
        `Suite FAILED — reproduced breach in: ${reproducedFailures(report).join(', ')}`
      )
    );
  } else if (report.suiteStatus === 'flagged') {
    console.log(
      chalk.yellow.bold(`Suite FLAGGED — targeted retry required for: ${report.flagged.join(', ')}`)
    );
  } else {
    console.log(chalk.green.bold('Suite passed'));
  }
}

function parseCli(argv: readonly string[]): CliOptions {
  const rawArgs = argv.filter(arg => arg !== '--');

  const { values } = parseArgs({
    args: [...rawArgs],
    options: {
      primary: { type: 'string' },
      retry: { type: 'string' },
      'output-json': { type: 'string' },
      'summary-md': { type: 'string' },
      'retry-output': { type: 'string' },
      'retry-seed': { type: 'string', default: '1' },
      'retry-time': { type: 'string', default: String(DEFAULT_PAIRED_TIME_BUDGET_MS) },
      warn: { type: 'string', default: String(DEFAULT_THRESHOLDS.warn) },
      fail: { type: 'string', default: String(DEFAULT_THRESHOLDS.fail) },
      'improvement-warn': {
        type: 'string',
        default: String(DEFAULT_THRESHOLDS.improvementWarn),
      },
      seed: { type: 'string', default: '1' },
      resamples: { type: 'string', default: '10000' },
      confidence: { type: 'string', default: '0.95' },
      help: { type: 'boolean', short: 'h', default: false },
    },
  });

  if (values.help) {
    printUsage();
    process.exit(0);
  }

  if (!values.primary) {
    printUsage();
    throw new Error('--primary is required');
  }

  const outputJson = values['output-json'] ?? deriveDefaultOutput(values.primary);
  const summaryMarkdown =
    values['summary-md'] ?? path.join(path.dirname(outputJson), 'compare-revisions.summary.md');
  const retryOutput =
    values['retry-output'] ??
    path.join(path.dirname(outputJson), 'revisions-retry-raw-stats.v1.json');

  return {
    primary: values.primary,
    retry: values.retry,
    outputJson,
    summaryMarkdown,
    retryOutput,
    retrySeed: parsePositiveInt('retry-seed', values['retry-seed']),
    retryTimeBudgetMs: parsePositiveInt('retry-time', values['retry-time']),
    thresholds: {
      warn: parsePositiveFloat('warn', values.warn),
      fail: parsePositiveFloat('fail', values.fail),
      improvementWarn: parsePositiveFloat('improvement-warn', values['improvement-warn']),
    },
    bootstrap: {
      seed: parsePositiveInt('seed', values.seed),
      resamples: parsePositiveInt('resamples', values.resamples),
      confidence: parseConfidence('confidence', values.confidence),
    },
  };
}

function deriveDefaultOutput(primaryPath: string): string {
  const dir = path.dirname(path.resolve(primaryPath));
  return path.join(dir, 'compare-revisions.verdict.v1.json');
}

function printUsage(): void {
  console.log(`
${chalk.bold('StyleX revisions verdict')}

Usage:
  pnpm bench:verdict --primary <raw-stats.v1.json> [--retry <raw-stats.v1.json>] [options]

Required:
  --primary <path>            primary raw-stats file (produced by bench:revisions)

Options:
  --retry <path>              use an existing targeted-retry raw-stats file
                              instead of measuring automatically
  --retry-output <path>       automatic retry raw-stats output
  --retry-seed <n>            automatic retry subject-order seed (default: 1)
  --retry-time <ms>           automatic retry time budget per task (default: ${DEFAULT_PAIRED_TIME_BUDGET_MS})
  --output-json <path>        verdict JSON output (default: alongside primary)
  --summary-md <path>         Markdown summary output (default: alongside verdict JSON)
  --warn <n>                  lower-bound warn threshold (default: ${DEFAULT_THRESHOLDS.warn})
  --fail <n>                  lower-bound fail threshold (default: ${DEFAULT_THRESHOLDS.fail})
  --improvement-warn <n>      upper-bound impossible-improvement threshold
                              (default: ${DEFAULT_THRESHOLDS.improvementWarn})
  --seed <n>                  bootstrap seed (default: 1)
  --resamples <n>             bootstrap resamples (default: 10000)
  --confidence <n>            one-sided confidence in (0, 1) (default: 0.95)
  -h, --help                  show this help

Exit codes:
  0  suite passed (or flagged fixtures did not reproduce)
  1  reproduced breach — suite failed
`);
}

if (isMainModule(import.meta.url)) {
  main()
    .then(code => {
      process.exitCode = code;
      return code;
    })
    .catch((error: unknown) => {
      writeFailureArtifacts(process.argv.slice(2), error);
      console.error(chalk.red('bench:verdict failed:'), error);
      process.exitCode = EXIT_FAILED;
    });
}

interface FailureArtifact {
  schemaVersion: typeof VERDICT_SCHEMA_VERSION;
  suiteStatus: 'error';
  error: { message: string };
}

function writeFailureArtifacts(argv: readonly string[], error: unknown): void {
  const primary = findArgument(argv, '--primary');
  const outputJson =
    findArgument(argv, '--output-json') ??
    (primary
      ? deriveDefaultOutput(primary)
      : path.join(benchmarkDir, 'results', 'compare-revisions.verdict.v1.json'));
  const artifact: FailureArtifact = {
    schemaVersion: VERDICT_SCHEMA_VERSION,
    suiteStatus: 'error',
    error: { message: errorMessage(error) },
  };
  const markdown = [
    '## Paired revision benchmark',
    '',
    'Suite status: **error**',
    '',
    escapeFailureMessage(artifact.error.message),
    '',
  ].join('\n');

  try {
    const resolvedOutput = path.resolve(outputJson);
    writeArtifact(resolvedOutput, `${JSON.stringify(artifact, null, 2)}\n`);
    const summaryPath =
      findArgument(argv, '--summary-md') ??
      path.join(path.dirname(resolvedOutput), 'compare-revisions.summary.md');
    writeArtifact(summaryPath, markdown);
    appendStepSummary(markdown);
  } catch (artifactError: unknown) {
    console.error(chalk.red('Failed to write verdict diagnostics:'), artifactError);
  }
}
