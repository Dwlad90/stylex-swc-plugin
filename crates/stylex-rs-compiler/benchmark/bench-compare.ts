/**
 * Head-to-head benchmark: `@stylexswc/rs-compiler` (NAPI-RS/SWC) vs
 * `@stylexjs/babel-plugin` (Babel) — the README performance claim.
 *
 * The underlying comparer is subject-parameterized (`lib/runner.ts`
 * accepts arbitrary named subjects). This entry point wires the two
 * concrete subjects the README compares and prints a Rust-versus-Babel
 * speedup table. For arbitrary base-vs-candidate comparisons used by
 * PR/release gates, use `bench:revisions`.
 *
 * Usage:
 *   pnpm bench:compare                     # both compilers, comparison table
 *   pnpm bench:compare --compiler rust     # only the Rust compiler
 *   pnpm bench:compare --compiler babel    # only the Babel plugin
 *   pnpm bench:compare --fixture create    # only fixtures matching "create"
 *   pnpm bench:compare --time 2000         # time budget per task in ms
 */

import fs from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

import * as babel from '@babel/core';
import stylexBabelPluginModule from '@stylexjs/babel-plugin';
import chalk from 'chalk';

import { createPairedBenchConfigs, createStylexOptions } from './lib/config.js';
import { captureEnvironment } from './lib/env.js';
import { loadAllFixtures } from './lib/fixtures.js';
import { formatLatency } from './lib/format.js';
import { runRounds } from './lib/runner.js';
import { createSubject, loadSubject, type LoadedSubject } from './lib/subjects.js';
import type { FixtureRawStats, RawLatencySamples } from './lib/types.js';

// Node's CJS interop hands back either the plugin itself or the module
// namespace depending on the loader; unwrap `.default` when present.
const stylexBabelPlugin = ((stylexBabelPluginModule as unknown as { default?: unknown }).default ??
  stylexBabelPluginModule) as babel.PluginTarget;

type CompilerName = 'rust' | 'babel';

const COMPILERS: readonly CompilerName[] = ['rust', 'babel'];

// pnpm forwards a literal `--` separator; drop it so parseArgs sees only flags.
const rawArgs = process.argv.slice(2).filter(arg => arg !== '--');

const DEFAULT_ROUNDS = 3;
const DEFAULT_SEED = 1;

const { values: cliOptions } = parseArgs({
  args: rawArgs,
  options: {
    compiler: { type: 'string', short: 'c', default: 'both' },
    fixture: { type: 'string', short: 'f', multiple: true },
    time: { type: 'string', short: 't', default: '1000' },
    rounds: { type: 'string', default: String(DEFAULT_ROUNDS) },
    seed: { type: 'string', default: String(DEFAULT_SEED) },
    help: { type: 'boolean', short: 'h', default: false },
  },
});

if (cliOptions.help) {
  console.log(`
${chalk.bold('StyleX compiler comparison benchmark')}

Options:
  -c, --compiler <both|rust|babel>  which compiler(s) to run (default: both)
  -f, --fixture <substring>         only run fixtures whose name contains the
                                    substring; repeatable
  -t, --time <ms>                   time budget per task (default: 1000)
      --rounds <n>                  balanced rounds per fixture (default: ${DEFAULT_ROUNDS})
      --seed <n>                    subject-order permutation seed (default: ${DEFAULT_SEED})
  -h, --help                        show this help
`);
  process.exit(0);
}

if (!['both', 'rust', 'babel'].includes(cliOptions.compiler)) {
  console.error(chalk.red(`Unknown --compiler value: ${cliOptions.compiler}`));
  process.exit(1);
}

const timeBudgetMs = parsePositiveInt('time', cliOptions.time);
const rounds = parsePositiveInt('rounds', cliOptions.rounds);
const seed = parsePositiveInt('seed', cliOptions.seed);

const selectedCompilers: readonly CompilerName[] =
  cliOptions.compiler === 'both' ? COMPILERS : [cliOptions.compiler as CompilerName];

const benchmarkDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(benchmarkDir, '..');
const workspaceRoot = path.resolve(packageDir, '../..');

const stylexOptions = createStylexOptions(packageDir);
const benchConfigs = createPairedBenchConfigs(timeBudgetMs);

function parsePositiveInt(name: string, value: string): number {
  const parsed = Number.parseInt(value, 10);
  if (!Number.isFinite(parsed) || parsed <= 0) {
    console.error(chalk.red(`Invalid --${name} value: ${value}`));
    process.exit(1);
  }
  return parsed;
}

function getPackageVersion(packageJsonPath: string): string {
  try {
    const pkg = JSON.parse(fs.readFileSync(packageJsonPath, 'utf-8')) as { version?: string };
    return pkg.version ?? 'unknown';
  } catch {
    return 'unknown';
  }
}

async function buildSubjects(): Promise<LoadedSubject[]> {
  const subjects: LoadedSubject[] = [];

  for (const name of selectedCompilers) {
    if (name === 'rust') {
      subjects.push(await loadSubject({ label: 'rust', packageDir }));
      continue;
    }

    const require = createRequire(import.meta.url);
    const babelPluginPkg = path.join(
      path.dirname(require.resolve('@stylexjs/babel-plugin')),
      '../package.json'
    );

    subjects.push(
      createSubject(
        {
          label: 'babel',
          version: getPackageVersion(babelPluginPkg),
          resolvedFrom: require.resolve('@stylexjs/babel-plugin'),
        },
        fixture => {
          const result = babel.transformSync(fixture.code, {
            filename: fixture.filePath,
            babelrc: false,
            configFile: false,
            parserOpts: { sourceType: 'module', plugins: ['jsx'] },
            plugins: [[stylexBabelPlugin, stylexOptions]],
          });
          const metadata = result?.metadata as unknown as { stylex?: unknown[] } | undefined;
          return metadata?.stylex?.length ?? 0;
        }
      )
    );
  }

  return subjects;
}

interface AggregatedSamples {
  p50: number;
  opsPerSec: number;
}

function samplesFor(fixture: FixtureRawStats, subject: string): AggregatedSamples | undefined {
  const perRound: RawLatencySamples[] = [];
  for (const round of fixture.rounds) {
    const samples = round.perSubject[subject];
    if (samples) perRound.push(samples);
  }
  if (perRound.length === 0) return undefined;
  return {
    p50: median(perRound.map(s => s.p50)),
    opsPerSec: median(perRound.map(s => s.opsPerSec)),
  };
}

function median(values: readonly number[]): number {
  const sorted = values.toSorted((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 0 ? (sorted[mid - 1]! + sorted[mid]!) / 2 : sorted[mid]!;
}

async function runBenchmarks(): Promise<void> {
  console.log(
    chalk.bold(
      `Running StyleX compiler benchmark: ${selectedCompilers.join(' vs ')} ` +
        `(perf + rollup fixtures)\n`
    )
  );

  const subjects = await buildSubjects();
  const env = captureEnvironment({ packageDir, workspaceRoot });

  console.log(
    `${chalk.bold('Benchmark environment:')}\n` +
      `  Node.js:  ${env.node}\n` +
      `  OS:       ${env.os.type} ${env.os.release} ${env.os.arch}\n` +
      `  CPU:      ${env.cpu.model} x ${env.cpu.cores} cores\n` +
      subjects.map(s => `  ${s.descriptor.label}: v${s.descriptor.version}`).join('\n') +
      `\n  babel/core: ${babel.version}\n`
  );

  const fixtures = loadAllFixtures({
    packageDir,
    workspaceRoot,
    categories: ['perf', 'rollup'],
    filter: cliOptions.fixture,
  });
  if (fixtures.length === 0) {
    console.error(chalk.red(`No fixtures match: ${(cliOptions.fixture ?? []).join(', ')}`));
    process.exit(1);
  }

  console.log(`Rounds: ${rounds}, seed: ${seed}`);

  const { fixtures: rawFixtures } = await runRounds({
    subjects,
    fixtures,
    stylexOptions,
    rounds,
    seed,
    standardBench: benchConfigs.standard,
    heavyBench: benchConfigs.heavy,
  });

  for (const fixture of rawFixtures) {
    for (const round of fixture.rounds) {
      console.log(
        `  [${fixture.name}] round ${round.round} order=[${round.subjectOrder.join(', ')}]`
      );
    }
  }

  const rows: Record<string, string | number>[] = [];
  const reportLines: string[] = [];

  for (const fixture of rawFixtures) {
    const row: Record<string, string | number> = { fixture: fixture.name };
    for (const compiler of selectedCompilers) {
      const stats = samplesFor(fixture, compiler);
      row[`${compiler} median`] = stats ? formatLatency(stats.p50) : 'n/a';
      row[`${compiler} ops/s`] = stats ? Math.round(stats.opsPerSec) : 'n/a';
    }
    const rust = samplesFor(fixture, 'rust');
    const babelStats = samplesFor(fixture, 'babel');
    if (rust && babelStats) {
      row.speedup = `${(babelStats.p50 / rust.p50).toFixed(1)}x`;
    }
    rows.push(row);
    reportLines.push(
      Object.entries(row)
        .map(([key, value]) => `${key}=${value}`)
        .join(' ')
    );
  }

  console.log('\nResults:');
  console.table(rows);

  if (selectedCompilers.length === 2) {
    const speedups = rawFixtures
      .map(fixture => {
        const rust = samplesFor(fixture, 'rust');
        const babelStats = samplesFor(fixture, 'babel');
        return rust && babelStats ? babelStats.p50 / rust.p50 : null;
      })
      .filter((speedup): speedup is number => speedup !== null);

    if (speedups.length > 0) {
      const min = Math.min(...speedups).toFixed(1);
      const max = Math.max(...speedups).toFixed(1);
      console.log(
        chalk.bold.green(`\nRust compiler is ${min}x to ${max}x faster than Babel per file`)
      );
    }
  }

  const resultsDir = path.join(benchmarkDir, 'results');
  if (!fs.existsSync(resultsDir)) {
    fs.mkdirSync(resultsDir, { recursive: true });
  }
  const outputPath = path.join(resultsDir, 'compare-output.txt');
  fs.writeFileSync(outputPath, reportLines.join('\n') + '\n', 'utf8');
  console.log(chalk.green(`\nResults saved to ${outputPath}`));
}

runBenchmarks().catch((error: unknown) => {
  console.error(chalk.red('Benchmark failed:'), error);
  process.exit(1);
});
