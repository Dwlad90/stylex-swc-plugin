/**
 * Absolute p95 budget gate.
 *
 * Reads a validated `raw-stats.v1.json` and the committed
 * `budget.json`, compares the median of per-round p95 values against the
 * reviewed ceilings, and writes a versioned report plus a Markdown
 * summary before exiting.
 *
 * Exit codes:
 *   0  within budget, or the budget is still pending calibration, or
 *      `--report-only` was requested
 *   1  breach, missing/extra entry, or canonical-environment drift
 *
 * The budget file is read-only here by design: a breach blocks the
 * release and must be resolved by optimization, rollback, or a separate
 * reviewed ceiling change.
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

import chalk from 'chalk';

import {
  BUDGET_REPORT_SCHEMA_VERSION,
  describeImages,
  describeMeasuredImage,
  evaluateBudget,
  renderBudgetMarkdown,
  type BudgetReport,
} from './lib/budget.js';
import {
  appendStepSummary,
  errorMessage,
  escapeFailureMessage,
  findArgument,
  isMainModule,
  writeArtifact,
} from './lib/cli.js';

const EXIT_PASS = 0;
const EXIT_FAILED = 1;

const benchmarkDir = path.dirname(fileURLToPath(import.meta.url));
const resultsDir = path.join(benchmarkDir, 'results');

interface CliOptions {
  raw: string;
  budget: string;
  outputJson: string;
  summaryMarkdown: string;
  reportOnly: boolean;
}

function main(): number {
  const options = parseCli(process.argv.slice(2));

  // The evaluator always reports the true status; this entry point owns
  // both the record of report-only mode and the exit code it implies.
  const report: BudgetReport = {
    ...evaluateBudget(readJson(options.raw), readJson(options.budget)),
    reportOnly: options.reportOnly,
  };

  writeArtifacts(options, report);
  printSummary(report);

  if (report.status === 'failed' && !options.reportOnly) return EXIT_FAILED;
  return EXIT_PASS;
}

function readJson(filePath: string): unknown {
  return JSON.parse(fs.readFileSync(path.resolve(filePath), 'utf8')) as unknown;
}

function writeArtifacts(options: CliOptions, report: BudgetReport): void {
  const markdown = renderBudgetMarkdown(report);
  writeArtifact(options.outputJson, `${JSON.stringify(report, null, 2)}\n`);
  writeArtifact(options.summaryMarkdown, markdown);
  appendStepSummary(markdown);
}

function printSummary(report: BudgetReport): void {
  console.log(chalk.bold(`\nAbsolute p95 budget: ${report.subject.label}`));
  const canonical = [
    report.canonical.target,
    `Node ${report.canonical.node}`,
    `image ${describeImages(report.canonical)}`,
  ].join(', ');
  const measured = [
    report.environment.target,
    `Node ${report.environment.node}`,
    `image ${describeMeasuredImage(report.environment)}`,
    `CPU ${report.environment.cpu.model}`,
  ].join(', ');
  console.log(`  canonical: ${canonical}`);
  console.log(`  measured:  ${measured}`);

  for (const fixture of report.fixtures) {
    const ceiling = fixture.ceilingMs === undefined ? 'none' : `${fixture.ceilingMs.toFixed(4)} ms`;
    const line = [
      `  ${fixture.name.padEnd(40)}`,
      `p95=${fixture.observedP95Ms.toFixed(4)} ms`,
      `ceiling=${ceiling}`,
      `status=${fixture.status}`,
    ].join(' ');
    if (fixture.status === 'breach') console.log(chalk.red(line));
    else if (fixture.status === 'unbudgeted') console.log(chalk.yellow(line));
    else console.log(line);
  }

  for (const problem of report.problems) {
    console.log(chalk.yellow(`  ${problem.kind}: ${problem.message}`));
  }

  console.log('');
  if (report.status === 'failed') {
    const message = `Budget FAILED — ${String(report.problems.length)} problem(s)`;
    console.log(
      report.reportOnly ? chalk.yellow.bold(`${message} (report-only)`) : chalk.red.bold(message)
    );
  } else if (report.status === 'unseeded') {
    console.log(
      chalk.yellow.bold(
        'Budget not enforced — ceilings are pending calibration. ' +
          'Archive this report as a seeding run.'
      )
    );
  } else {
    console.log(chalk.green.bold('Within budget'));
  }
}

function parseCli(argv: readonly string[]): CliOptions {
  const { values } = parseArgs({
    args: argv.filter(argument => argument !== '--'),
    options: {
      raw: { type: 'string', default: path.join(resultsDir, 'revisions-raw-stats.v1.json') },
      budget: { type: 'string', default: path.join(benchmarkDir, 'budget.json') },
      'output-json': { type: 'string', default: path.join(resultsDir, 'budget-report.v1.json') },
      'summary-md': { type: 'string', default: path.join(resultsDir, 'budget-report.md') },
      'report-only': { type: 'boolean', default: false },
      help: { type: 'boolean', short: 'h', default: false },
    },
  });

  if (values.help) {
    printUsage();
    process.exit(EXIT_PASS);
  }

  return {
    raw: values.raw,
    budget: values.budget,
    outputJson: values['output-json'],
    summaryMarkdown: values['summary-md'],
    reportOnly: values['report-only'],
  };
}

function printUsage(): void {
  console.log(`
${chalk.bold('StyleX absolute p95 budget check')}

Usage:
  pnpm bench:budget [options]

Options:
  --raw <path>            raw-stats file to check (default: results/revisions-raw-stats.v1.json)
  --budget <path>         committed ceilings (default: benchmark/budget.json)
  --output-json <path>    report output (default: results/budget-report.v1.json)
  --summary-md <path>     Markdown summary output (default: results/budget-report.md)
  --report-only           never exit non-zero; use for seeding runs
  -h, --help              show this help

Exit codes:
  0  within budget, pending calibration, or report-only
  1  breach, coverage mismatch, or canonical-environment drift
`);
}

if (isMainModule(import.meta.url)) {
  try {
    process.exitCode = main();
  } catch (error: unknown) {
    writeFailureArtifacts(process.argv.slice(2), error);
    console.error(chalk.red('check-budget failed:'), error);
    process.exitCode = EXIT_FAILED;
  }
}

function writeFailureArtifacts(argv: readonly string[], error: unknown): void {
  const message = errorMessage(error);
  const artifact = {
    schemaVersion: BUDGET_REPORT_SCHEMA_VERSION,
    status: 'error' as const,
    error: { message },
  };
  const markdown = [
    '## Absolute p95 budget',
    '',
    'Status: **error**',
    '',
    escapeFailureMessage(message),
    '',
  ].join('\n');

  try {
    const outputJson =
      findArgument(argv, '--output-json') ?? path.join(resultsDir, 'budget-report.v1.json');
    const summaryMd =
      findArgument(argv, '--summary-md') ?? path.join(resultsDir, 'budget-report.md');
    writeArtifact(outputJson, `${JSON.stringify(artifact, null, 2)}\n`);
    writeArtifact(summaryMd, markdown);
    appendStepSummary(markdown);
  } catch (artifactError: unknown) {
    console.error(chalk.red('Failed to write budget diagnostics:'), artifactError);
  }
}
