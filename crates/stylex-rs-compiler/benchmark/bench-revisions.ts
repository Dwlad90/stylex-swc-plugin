/**
 * Arbitrary base-vs-candidate benchmark entry point.
 *
 * Feeds the PR and release paired gates: both subjects are loaded through
 * `loadSubject` from explicit on-disk package directories, share the
 * shared fixture registry, and run under balanced seeded-randomized
 * subject order inside a single process. The output is
 * `results/revisions-raw-stats.v1.json` — the input consumed by the
 * Phase 3 verdict engine. Never parse the human-readable summary.
 *
 * Verdict statistics (ratios, bootstrap CI) are deliberately not
 * computed here: they belong to the Phase 3 verdict layer.
 *
 * Usage:
 *   pnpm bench:revisions --base <base-pkg-dir> --candidate <candidate-pkg-dir>
 *
 * Both paths must point to package directories laid out like
 * `@stylexswc/rs-compiler` (a `dist/index.js` exporting `transform` and a
 * `package.json`).
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

import chalk from 'chalk';

import { createPairedBenchConfigs, createStylexOptions } from './lib/config.js';
import { captureEnvironment } from './lib/env.js';
import { loadAllFixtures } from './lib/fixtures.js';
import { formatLatency } from './lib/format.js';
import { runRounds } from './lib/runner.js';
import { loadSubject } from './lib/subjects.js';
import { RAW_STATS_SCHEMA_VERSION, type FixtureCategory, type RawStatsFile } from './lib/types.js';

const ALL_CATEGORIES: readonly FixtureCategory[] = ['transform', 'perf', 'rollup'];

interface RevisionInput {
  label: string;
  packageDir: string;
}

interface PairedRunOptions {
  base: RevisionInput;
  candidate: RevisionInput;
  rounds: number;
  seed: number;
  timeBudgetMs: number;
  categories: readonly FixtureCategory[];
  fixtureFilter: readonly string[] | undefined;
}

const benchmarkDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(benchmarkDir, '..');
const workspaceRoot = path.resolve(packageDir, '../..');

async function main(): Promise<void> {
  const options = parseCli(process.argv.slice(2));

  console.log(chalk.bold('StyleX revisions benchmark\n'));
  console.log(`  base:       ${options.base.label} @ ${options.base.packageDir}`);
  console.log(`  candidate:  ${options.candidate.label} @ ${options.candidate.packageDir}`);
  console.log(`  rounds:     ${options.rounds}`);
  console.log(`  seed:       ${options.seed}`);
  console.log(`  categories: ${options.categories.join(', ')}\n`);

  const baseSubject = await loadSubject({
    label: options.base.label,
    packageDir: options.base.packageDir,
  });
  const candidateSubject = await loadSubject({
    label: options.candidate.label,
    packageDir: options.candidate.packageDir,
  });

  const env = captureEnvironment({ packageDir, workspaceRoot });
  console.log(
    `Node ${env.node} | ${env.os.type} ${env.os.release} ${env.os.arch} | ` +
      `${env.cpu.model} x${env.cpu.cores}`
  );
  console.log(`  ${options.base.label} v${baseSubject.descriptor.version}`);
  console.log(`  ${options.candidate.label} v${candidateSubject.descriptor.version}\n`);

  const fixtures = loadAllFixtures({
    packageDir,
    workspaceRoot,
    categories: options.categories,
    filter: options.fixtureFilter,
  });
  if (fixtures.length === 0) {
    throw new Error(`No fixtures match: ${(options.fixtureFilter ?? []).join(', ')}`);
  }

  const benchConfigs = createPairedBenchConfigs(options.timeBudgetMs);

  const { fixtures: rawFixtures } = await runRounds({
    subjects: [baseSubject, candidateSubject],
    fixtures,
    stylexOptions: createStylexOptions(packageDir),
    rounds: options.rounds,
    seed: options.seed,
    standardBench: benchConfigs.standard,
    heavyBench: benchConfigs.heavy,
  });

  for (const fixture of rawFixtures) {
    console.log(chalk.bold(fixture.name));
    for (const round of fixture.rounds) {
      const base = round.perSubject[options.base.label];
      const candidate = round.perSubject[options.candidate.label];
      const basePart = base ? formatLatency(base.p50) : 'n/a';
      const candidatePart = candidate ? formatLatency(candidate.p50) : 'n/a';
      console.log(
        `  round ${round.round}: order=[${round.subjectOrder.join(', ')}] ` +
          `${options.base.label}=${basePart} ${options.candidate.label}=${candidatePart}`
      );
    }
    console.log('');
  }

  const rawStats: RawStatsFile = {
    schemaVersion: RAW_STATS_SCHEMA_VERSION,
    environment: env,
    subjects: [baseSubject.descriptor, candidateSubject.descriptor],
    fixtures: rawFixtures,
  };

  const resultsDir = path.join(benchmarkDir, 'results');
  fs.mkdirSync(resultsDir, { recursive: true });
  const outputPath = path.join(resultsDir, 'revisions-raw-stats.v1.json');
  fs.writeFileSync(outputPath, `${JSON.stringify(rawStats, null, 2)}\n`, 'utf8');
  console.log(chalk.green(`Raw stats saved to ${outputPath}`));
}

function parseCli(argv: readonly string[]): PairedRunOptions {
  const rawArgs = argv.filter(arg => arg !== '--');

  const { values } = parseArgs({
    args: [...rawArgs],
    options: {
      base: { type: 'string' },
      candidate: { type: 'string' },
      'base-label': { type: 'string', default: 'base' },
      'candidate-label': { type: 'string', default: 'candidate' },
      rounds: { type: 'string', default: '10' },
      seed: { type: 'string', default: '1' },
      time: { type: 'string', default: '300' },
      fixture: { type: 'string', multiple: true },
      category: { type: 'string', multiple: true },
      help: { type: 'boolean', short: 'h', default: false },
    },
  });

  if (values.help) {
    printUsage();
    process.exit(0);
  }

  if (!values.base || !values.candidate) {
    printUsage();
    throw new Error('--base and --candidate are required');
  }

  const baseLabel = values['base-label'];
  const candidateLabel = values['candidate-label'];
  if (baseLabel === candidateLabel) {
    throw new Error('--base-label and --candidate-label must differ');
  }

  return {
    base: { label: baseLabel, packageDir: path.resolve(values.base) },
    candidate: { label: candidateLabel, packageDir: path.resolve(values.candidate) },
    rounds: parsePositiveInt('rounds', values.rounds),
    seed: parsePositiveInt('seed', values.seed),
    timeBudgetMs: parsePositiveInt('time', values.time),
    categories: parseCategories(values.category),
    fixtureFilter: values.fixture,
  };
}

function printUsage(): void {
  console.log(`
${chalk.bold('StyleX revisions benchmark')}

Usage:
  pnpm bench:revisions --base <dir> --candidate <dir> [options]

Required:
  --base <dir>              base package directory (contains dist/index.js)
  --candidate <dir>         candidate package directory

Options:
  --base-label <name>       label for the base subject (default: base)
  --candidate-label <name>  label for the candidate subject (default: candidate)
  --rounds <n>              rounds per fixture (default: 10)
  --seed <n>                subject-order permutation seed (default: 1)
  --time <ms>               tinybench time budget per task (default: 300)
  --category <name>         restrict to a fixture category; repeatable
                            (transform | perf | rollup)
  --fixture <substring>     only fixtures whose name contains substring;
                            repeatable
  -h, --help                show this help
`);
}

function parsePositiveInt(name: string, value: string): number {
  const parsed = Number.parseInt(value, 10);
  if (!Number.isFinite(parsed) || parsed <= 0) {
    throw new Error(`Invalid --${name} value: ${value}`);
  }
  return parsed;
}

function parseCategories(input: string[] | undefined): readonly FixtureCategory[] {
  if (!input || input.length === 0) return ALL_CATEGORIES;
  const allowed = new Set<string>(ALL_CATEGORIES);
  const out: FixtureCategory[] = [];
  for (const value of input) {
    if (!allowed.has(value)) {
      throw new Error(`Invalid --category value: ${value}`);
    }
    out.push(value as FixtureCategory);
  }
  return out;
}

main().catch((error: unknown) => {
  console.error(chalk.red('bench:revisions failed:'), error);
  process.exit(1);
});
