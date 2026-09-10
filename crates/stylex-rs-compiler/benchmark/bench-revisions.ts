/**
 * Arbitrary base-vs-candidate benchmark entry point.
 *
 * Feeds the PR and release paired gates: both subjects come from explicit
 * on-disk package directories, share the shared fixture registry, and run
 * under balanced seeded-randomized subject order. The output is
 * `results/revisions-raw-stats.v1.json` — the input consumed by the
 * verdict engine. Never parse the human-readable summary.
 *
 * One process holds both subjects wherever it can, which is every platform but
 * macOS. There, two bindings that link mimalloc end the process, so each
 * subject is timed in a child of its own and the schedule is unchanged.
 * `--separate-processes` asks for that anywhere. `lib/subject-process.ts`
 * carries the reason and what it costs.
 *
 * Verdict statistics (ratios, bootstrap CI) are deliberately not computed
 * here: they belong to the verdict layer.
 *
 * Both legs that call this share one manifest while their bases differ: the
 * pull-request leg builds the merge base, and the release leg installs the last
 * published version. So they need different readings of a base that refuses a
 * fixture, and `--allow-base-refusals` is which one the caller wants. Under the
 * flag such a fixture is reported under `Not compared` and left out of the run,
 * because stopping the whole leg for a fixture that prices a feature the
 * published version does not carry gave up every other measurement for a fact
 * the manifest already states. Without it every refusal stops the run, which is
 * what keeps a fixture only this branch compiles out of the manifest.
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

import type { StyleXOptions } from '../dist/index.js';
import { parsePositiveInt } from './lib/cli.js';
import {
  createPairedBenchConfigs,
  createStylexOptions,
  DEFAULT_PAIRED_TIME_BUDGET_MS,
} from './lib/config.js';
import { captureEnvironment } from './lib/env.js';
import { loadAllFixtures } from './lib/fixtures.js';
import { formatLatency } from './lib/format.js';
import { findNativeBindings, subjectsCanShareProcess } from './lib/native-bindings.js';
import { runRounds, type RunOptions } from './lib/runner.js';
import { startSplitRun, type SplitRun } from './lib/subject-process.js';
import { loadSubject, type LoadedSubject } from './lib/subjects.js';
import {
  RAW_STATS_SCHEMA_VERSION,
  type FixtureCategory,
  type FixtureDescriptor,
  type RawStatsFile,
} from './lib/types.js';

const ALL_CATEGORIES: readonly FixtureCategory[] = ['transform', 'perf', 'rollup'];

interface RevisionInput {
  label: string;
  packageDir: string;
}

interface PairedRunOptions {
  base: RevisionInput;
  candidate: RevisionInput;
  /**
   * Whether a fixture the base subject cannot measure leaves the run instead of
   * stopping it. Off by default, so the strict reading is what a caller gets
   * without asking: the pull-request leg builds its base from the merge base,
   * where a fixture only this branch compiles is a manifest question and the
   * refusal is the guard that catches it.
   */
  allowBaseRefusals: boolean;
  /**
   * Whether each subject is timed in a process of its own even where one
   * process could hold both.
   *
   * The split is taken on its own where a platform needs it, so nothing has to
   * ask for it. The flag is how the path is exercised where it is not needed:
   * without it the code would run on macOS alone, and only against a base that
   * links mimalloc, which no leg has yet. A test names it, and so can anyone
   * comparing what the two paths measure.
   */
  separateProcesses: boolean;
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

  // One addon for every subject is not a comparison. The variable names a file
  // that `dist/transform.js` loads whatever package asked, so both revisions
  // would be the same binary and the run would report a ratio of about one
  // without saying why.
  if (process.env.NAPI_RS_NATIVE_LIBRARY_PATH) {
    throw new Error(
      'NAPI_RS_NATIVE_LIBRARY_PATH names one native binding, and both subjects ' +
        'would load it, so the two revisions would be the same binary. Unset it ' +
        'to compare two revisions.'
    );
  }

  // Asked before either subject is loaded, because the answer decides whether
  // loading them both is safe. On macOS two bindings that link mimalloc end the
  // process, and a process that ended reports nothing at all.
  const shareProcess =
    !options.separateProcesses &&
    subjectsCanShareProcess(
      findNativeBindings(options.base.packageDir),
      findNativeBindings(options.candidate.packageDir)
    );

  const env = captureEnvironment({ packageDir, workspaceRoot });
  console.log(
    `Node ${env.node} | ${env.os.type} ${env.os.release} ${env.os.arch} | ` +
      `${env.cpu.model} x${env.cpu.cores}`
  );

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
  const stylexOptions = createStylexOptions(packageDir);

  const placement = await placeSubjects({
    options,
    fixtures,
    stylexOptions,
    shareProcess,
  });

  for (const subject of placement.subjects) {
    console.log(`  ${subject.descriptor.label} v${subject.descriptor.version}`);
  }
  console.log(
    shareProcess
      ? ''
      : `\n  Each subject is timed in a process of its own, ${
          options.separateProcesses
            ? 'because --separate-processes asks for it.'
            : 'because this platform cannot hold\n  two bindings that link mimalloc.'
        }\n`
  );

  let rawFixtures;
  let excluded;
  try {
    ({ fixtures: rawFixtures, excluded } = await runRounds({
      subjects: placement.subjects,
      fixtures,
      stylexOptions,
      rounds: options.rounds,
      seed: options.seed,
      standardBench: benchConfigs.standard,
      heavyBench: benchConfigs.heavy,
      ...placement.strategies,
      // Under `--allow-base-refusals` the candidate is the only gate: it is the
      // code the numbers are about, so a fixture it refuses still stops the leg,
      // while one only the base refuses leaves the run with a line saying so.
      // Without the flag no subject is privileged and any refusal stops the run,
      // which is the reading the merge-base leg needs.
      ...(options.allowBaseRefusals ? { requiredSubject: options.candidate.label } : {}),
    }));
  } finally {
    placement.close();
  }

  if (excluded.length > 0) {
    console.log(chalk.yellow.bold('Not compared'));
    for (const entry of excluded) {
      console.log(`  ${entry.fixture} — ${entry.subject}: ${entry.reason}`);
    }
    console.log('');
  }

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
    subjects: placement.subjects.map(subject => subject.descriptor),
    fixtures: rawFixtures,
    // Written beside the numbers rather than only to the log, so a comparison
    // that measured fewer fixtures than the manifest holds says so in the file
    // a reviewer downloads. Read by nobody: the schema does not carry it, and
    // the parser keeps the keys it knows.
    ...(excluded.length > 0 ? { excluded } : {}),
  };

  const resultsDir = path.join(benchmarkDir, 'results');
  fs.mkdirSync(resultsDir, { recursive: true });
  const outputPath = path.join(resultsDir, 'revisions-raw-stats.v1.json');
  fs.writeFileSync(outputPath, `${JSON.stringify(rawStats, null, 2)}\n`, 'utf8');
  console.log(chalk.green(`Raw stats saved to ${outputPath}`));
}

/** The subjects to measure, and where the runner does the work that touches them. */
interface SubjectPlacement {
  subjects: LoadedSubject[];
  strategies: Pick<RunOptions, 'countRules' | 'measureRound'>;
  close: () => void;
}

/**
 * Loads both subjects, or arranges for each to be measured on its own.
 *
 * The single process is the path every platform but macOS takes, and the one
 * every release has gated on. The split path exists because two bindings that
 * link mimalloc end a macOS process, and it changes only where a subject is
 * asked to compile: the schedule, the fixture selection and the statistics are
 * the same code either way.
 */
async function placeSubjects(input: {
  options: PairedRunOptions;
  fixtures: readonly FixtureDescriptor[];
  stylexOptions: StyleXOptions;
  shareProcess: boolean;
}): Promise<SubjectPlacement> {
  const revisions = [input.options.base, input.options.candidate];

  if (!input.shareProcess) {
    const split: SplitRun = startSplitRun({
      subjects: revisions,
      fixtures: input.fixtures,
      stylexOptions: input.stylexOptions,
      timeBudgetMs: input.options.timeBudgetMs,
    });

    return {
      subjects: split.subjects,
      strategies: { countRules: split.countRules, measureRound: split.measureRound },
      close: split.close,
    };
  }

  const subjects: LoadedSubject[] = [];
  for (const revision of revisions) {
    subjects.push(await loadSubject({ label: revision.label, packageDir: revision.packageDir }));
  }

  return { subjects, strategies: {}, close: () => undefined };
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
      time: { type: 'string', default: String(DEFAULT_PAIRED_TIME_BUDGET_MS) },
      fixture: { type: 'string', multiple: true },
      category: { type: 'string', multiple: true },
      'allow-base-refusals': { type: 'boolean', default: false },
      'separate-processes': { type: 'boolean', default: false },
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
    allowBaseRefusals: values['allow-base-refusals'],
    separateProcesses: values['separate-processes'],
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
  --time <ms>               tinybench time budget per task (default: ${DEFAULT_PAIRED_TIME_BUDGET_MS})
  --category <name>         restrict to a fixture category; repeatable
                            (transform | perf | rollup)
  --fixture <substring>     only fixtures whose name contains substring;
                            repeatable
  --allow-base-refusals     drop a fixture the base subject cannot measure
                            instead of stopping the run; for a base that is
                            behind by whole features, such as a published
                            release
  --separate-processes      time each subject in a process of its own, which
                            a platform that cannot hold two bindings does
                            anyway
  -h, --help                show this help
`);
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
