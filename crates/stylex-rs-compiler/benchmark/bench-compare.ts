/**
 * Head-to-head benchmark: `@stylexswc/rs-compiler` (NAPI-RS/SWC) vs
 * `@stylexjs/babel-plugin` (Babel) on identical fixtures with identical
 * options.
 *
 * Usage:
 *   pnpm bench:compare                     # both compilers, comparison table
 *   pnpm bench:compare --compiler rust     # only the Rust compiler
 *   pnpm bench:compare --compiler babel    # only the Babel plugin
 *   pnpm bench:compare --fixture create    # only fixtures matching "create"
 *   pnpm bench:compare --time 2000         # time budget per task in ms
 */

import { Bench, type BenchOptions, type Task, type TaskResultWithStatistics } from 'tinybench';
import * as babel from '@babel/core';
import stylexBabelPluginModule from '@stylexjs/babel-plugin';
import path from 'path';
import fs from 'fs';
import os from 'os';
import { parseArgs } from 'node:util';
import { createRequire } from 'node:module';
import chalk from 'chalk';
import { fileURLToPath } from 'url';
import { transform } from '../dist/index.js';
import type { StyleXOptions } from '../dist/index.js';

// Node's CJS interop hands back either the plugin itself or the module
// namespace depending on the loader; unwrap `.default` when present.
const stylexBabelPlugin: babel.PluginTarget =
  (stylexBabelPluginModule as unknown as { default?: babel.PluginTarget }).default ??
  (stylexBabelPluginModule as unknown as babel.PluginTarget);

type CompilerName = 'rust' | 'babel';

const COMPILERS: readonly CompilerName[] = ['rust', 'babel'];

// pnpm forwards a literal `--` separator; drop it so parseArgs sees only flags.
const rawArgs = process.argv.slice(2).filter(arg => arg !== '--');

const { values: cliOptions } = parseArgs({
  args: rawArgs,
  options: {
    compiler: { type: 'string', short: 'c', default: 'both' },
    fixture: { type: 'string', short: 'f', multiple: true },
    time: { type: 'string', short: 't', default: '1000' },
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
  -h, --help                        show this help
`);
  process.exit(0);
}

if (!['both', 'rust', 'babel'].includes(cliOptions.compiler)) {
  console.error(chalk.red(`Unknown --compiler value: ${cliOptions.compiler}`));
  process.exit(1);
}

const timeBudgetMs = Number.parseInt(cliOptions.time, 10);
if (Number.isNaN(timeBudgetMs) || timeBudgetMs <= 0) {
  console.error(chalk.red(`Invalid --time value: ${cliOptions.time}`));
  process.exit(1);
}

const selectedCompilers: readonly CompilerName[] =
  cliOptions.compiler === 'both' ? COMPILERS : [cliOptions.compiler as CompilerName];

const benchmarkDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(benchmarkDir, '..');
const workspaceRoot = path.resolve(packageDir, '../..');

const stylexOptions: StyleXOptions = {
  dev: false,
  treeshakeCompensation: true,
  unstable_moduleResolution: {
    type: 'haste',
    rootDir: packageDir,
  },
};

interface Fixture {
  name: string;
  filePath: string;
  code: string;
  /** Heavy fixtures run with a reduced iteration budget, matching bench.ts. */
  heavy: boolean;
}

function loadFixture(name: string, relativePath: string, heavy = false): Fixture {
  const filePath = path.join(workspaceRoot, relativePath);
  return { name, filePath, code: fs.readFileSync(filePath, 'utf-8'), heavy };
}

const allFixtures: readonly Fixture[] = [
  loadFixture('create-basic', 'crates/stylex-rs-compiler/benchmark/perf_fixtures/create-basic.js'),
  loadFixture(
    'create-complex',
    'crates/stylex-rs-compiler/benchmark/perf_fixtures/create-complex.js'
  ),
  loadFixture(
    'createTheme-basic',
    'crates/stylex-rs-compiler/benchmark/perf_fixtures/createTheme-basic.js'
  ),
  loadFixture(
    'createTheme-complex',
    'crates/stylex-rs-compiler/benchmark/perf_fixtures/createTheme-complex.js'
  ),
  loadFixture(
    'colors.stylex',
    'crates/stylex-rs-compiler/benchmark/perf_fixtures/colors.stylex.js'
  ),
  loadFixture('lotsOfStyles', 'apps/rollup-large-example/lotsOfStyles.js', true),
  loadFixture('lotsOfStylesDynamic', 'apps/rollup-large-example/lotsOfStylesDynamic.js', true),
];

const fixtureFilters = cliOptions.fixture ?? [];
const fixtures = fixtureFilters.length
  ? allFixtures.filter(fixture => fixtureFilters.some(filter => fixture.name.includes(filter)))
  : allFixtures;

if (fixtures.length === 0) {
  console.error(chalk.red(`No fixtures match: ${fixtureFilters.join(', ')}`));
  process.exit(1);
}

function transformWithRust(fixture: Fixture): number {
  const { metadata } = transform(fixture.filePath, fixture.code, stylexOptions);
  return metadata.stylex.length;
}

function transformWithBabel(fixture: Fixture): number {
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

const RUNNERS: Record<CompilerName, (fixture: Fixture) => number> = {
  rust: transformWithRust,
  babel: transformWithBabel,
};

/**
 * Both compilers must produce StyleX rules for every fixture, otherwise the
 * timing comparison would be meaningless (one side silently doing no work).
 */
function sanityCheck(): void {
  for (const fixture of fixtures) {
    for (const compiler of selectedCompilers) {
      const ruleCount = RUNNERS[compiler](fixture);
      if (ruleCount === 0) {
        throw new Error(
          `Fixture "${fixture.name}" produced no StyleX rules with the ${compiler} compiler`
        );
      }
    }
  }
}

const STANDARD_CONFIG: BenchOptions = {
  retainSamples: true,
  warmup: true,
  time: timeBudgetMs,
  iterations: 20,
};

const HEAVY_CONFIG: BenchOptions = {
  retainSamples: true,
  warmup: true,
  time: Math.min(timeBudgetMs, 500),
  iterations: 5,
  warmupIterations: 1,
  warmupTime: 100,
};

interface CompilerStats {
  medianMs: number;
  opsPerSec: number;
  rme: number;
  samples: number;
}

function getStats(task: Task): CompilerStats {
  if (!('throughput' in task.result)) {
    throw new Error(`${task.name}: no results`);
  }

  const result = task.result as TaskResultWithStatistics;

  return {
    medianMs: result.latency.p50 ?? Number.NaN,
    opsPerSec: result.throughput.mean,
    rme: result.latency.rme,
    samples: result.latency.samplesCount,
  };
}

function formatLatency(milliseconds: number): string {
  if (!Number.isFinite(milliseconds)) return 'n/a';
  if (milliseconds >= 1000) return `${(milliseconds / 1000).toFixed(2)} s`;
  if (milliseconds >= 1) return `${milliseconds.toFixed(2)} ms`;
  return `${(milliseconds * 1000).toFixed(0)} µs`;
}

function getPackageVersion(packageJsonPath: string): string {
  try {
    const pkg = JSON.parse(fs.readFileSync(packageJsonPath, 'utf-8')) as { version?: string };
    return pkg.version ?? 'unknown';
  } catch {
    return 'unknown';
  }
}

function getEnvironmentInfo(): string {
  const require = createRequire(import.meta.url);
  const rsVersion = getPackageVersion(path.join(packageDir, 'package.json'));
  const babelPluginVersion = getPackageVersion(
    path.join(path.dirname(require.resolve('@stylexjs/babel-plugin')), '../package.json')
  );

  return `
${chalk.bold('Benchmark environment:')}
  Node.js:  ${process.version}
  OS:       ${os.type()} ${os.release()} ${os.arch()}
  CPU:      ${os.cpus()[0]?.model} x ${os.cpus().length} cores
  Rust:     @stylexswc/rs-compiler v${rsVersion}
  Babel:    @stylexjs/babel-plugin v${babelPluginVersion} on @babel/core ${babel.version}
`;
}

async function runBenchmarks(): Promise<void> {
  console.log(
    chalk.bold(
      `Running StyleX compiler benchmark: ${selectedCompilers.join(' vs ')} ` +
        `(${fixtures.length} fixture${fixtures.length === 1 ? '' : 's'})\n`
    )
  );

  sanityCheck();
  console.log(chalk.green('Sanity check passed: all fixtures produce StyleX rules\n'));
  console.log(getEnvironmentInfo());

  const taskStats = new Map<string, Partial<Record<CompilerName, CompilerStats>>>();

  for (const heavy of [false, true]) {
    const groupFixtures = fixtures.filter(fixture => fixture.heavy === heavy);
    if (groupFixtures.length === 0) continue;

    const bench = new Bench({
      name: heavy ? 'heavy fixtures' : 'standard fixtures',
      ...(heavy ? HEAVY_CONFIG : STANDARD_CONFIG),
    });

    for (const fixture of groupFixtures) {
      for (const compiler of selectedCompilers) {
        bench.add(`${fixture.name} [${compiler}]`, () => {
          RUNNERS[compiler](fixture);
        });
      }
    }

    console.log(chalk.yellow.bold(`Running: ${bench.name}`));
    await bench.run();

    for (const task of bench.tasks) {
      const match = task.name.match(/^(.+) \[(rust|babel)\]$/);
      if (!match) continue;
      const [, fixtureName, compiler] = match;
      const entry = taskStats.get(fixtureName as string) ?? {};
      entry[compiler as CompilerName] = getStats(task);
      taskStats.set(fixtureName as string, entry);
    }
  }

  const rows: Record<string, string | number>[] = [];
  const reportLines: string[] = [];

  for (const fixture of fixtures) {
    const entry = taskStats.get(fixture.name) ?? {};
    const row: Record<string, string | number> = { fixture: fixture.name };

    for (const compiler of selectedCompilers) {
      const stats = entry[compiler];
      row[`${compiler} median`] = stats ? formatLatency(stats.medianMs) : 'n/a';
      row[`${compiler} ops/s`] = stats ? Math.round(stats.opsPerSec) : 'n/a';
    }

    const rust = entry.rust;
    const babelStats = entry.babel;
    if (rust && babelStats) {
      row.speedup = `${(babelStats.medianMs / rust.medianMs).toFixed(1)}x`;
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
    const speedups = fixtures
      .map(fixture => {
        const entry = taskStats.get(fixture.name);
        return entry?.rust && entry?.babel ? entry.babel.medianMs / entry.rust.medianMs : null;
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
