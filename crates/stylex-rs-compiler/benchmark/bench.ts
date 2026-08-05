import fs from 'fs';
import os from 'os';
import path from 'path';
import { fileURLToPath } from 'url';

import chalk from 'chalk';
import { Bench, type BenchOptions, type Task, type TaskResultWithStatistics } from 'tinybench';

import { transform } from '../dist/index.js';
import type { StyleXOptions } from '../dist/index.js';

const BENCHMARK_CONFIG: BenchOptions = {
  retainSamples: true,
  warmup: true,
};

const LOTS_OF_STYLES_CONFIG = {
  ...BENCHMARK_CONFIG,
  time: 500,
  iterations: 10,
  warmupIterations: 1,
  warmupTime: 100,
};

const benchmarkDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(benchmarkDir, '..');
const workspaceRoot = path.resolve(packageDir, '../..');
const rootDir = packageDir;

const benchRegular = new Bench({
  name: 'StyleX compiler - regular benchmark',
  ...BENCHMARK_CONFIG,
});

const benchPerformance = new Bench({
  name: 'StyleX compiler - performance benchmark',
  ...BENCHMARK_CONFIG,
});

const benchLotsOfStyles = new Bench({
  name: 'StyleX compiler - lots of styles benchmark',
  ...LOTS_OF_STYLES_CONFIG,
});

const stylexOptions: StyleXOptions = {
  dev: false,
  treeshakeCompensation: true,
  unstable_moduleResolution: {
    type: 'haste',
    rootDir,
  },
};

function getFixtureFilePaths(dir: string): string[] {
  let results: string[] = [];

  const list = fs.readdirSync(dir);

  list.forEach(file => {
    const filePath = path.join(dir, file);
    const stat = fs.statSync(filePath);

    if (stat && stat.isDirectory()) {
      results = results.concat(getFixtureFilePaths(filePath));
    } else if (file === 'input.stylex.js') {
      results.push(filePath);
    }
  });

  return results;
}

function addFixtureBenchmarks(bench: Bench, fixtureFilePaths: string[]) {
  fixtureFilePaths.forEach(file => {
    const content = fs.readFileSync(file, 'utf-8');
    const benchmarkName = file.split(path.sep).at(-2) ?? 'Default case';

    bench.add(benchmarkName, () => {
      transform(file, content, stylexOptions);
    });
  });
}

interface BenchmarkStats {
  name: string;
  opsPerSec: string;
  rme: string;
  samples: number;
  median: string;
  p95: string;
  medianMs: number;
  p95Ms: number;
}

/**
 * One entry of the JSON consumed by `benchmark-action/github-action-benchmark`
 * with `tool: 'customSmallerIsBetter'`, whose schema is
 * `{ name, value, unit, range?, extra? }`.
 */
interface BenchmarkEntry {
  name: string;
  value: number;
  unit: string;
  range: string;
  extra: string;
}

function getBenchmarkStats(task: Task): BenchmarkStats {
  const { name } = task;

  if (!('throughput' in task.result)) {
    throw new Error(`❌ ${name}: No results`);
  }

  const result = task.result as TaskResultWithStatistics;
  if (!result) {
    throw new Error(`❌ ${name}: No results`);
  }

  const medianMs = result.latency.p50;
  const p95Ms = percentile(result.latency.samples, 95);

  return {
    name,
    opsPerSec: result.throughput.mean.toLocaleString('en-US', {
      maximumFractionDigits: 2,
    }),
    rme: result.latency.rme.toFixed(2),
    samples: result.latency.samplesCount,
    median: formatLatency(medianMs),
    p95: formatLatency(p95Ms),
    medianMs,
    p95Ms,
  };
}

/**
 * Median latency, which is what CI compares against previous runs.
 *
 * Median rather than mean throughput: the mean is pulled around by a single
 * slow sample, and CI runs on shared runners where that happens. p50 over
 * retained samples is the robust statistic, and tinybench already computes it.
 *
 * `range`/`extra` are display-only — the action compares `value` alone.
 */
function toBenchmarkEntry(stats: BenchmarkStats): BenchmarkEntry {
  const samples = stats.samples > 0 ? stats.samples : 1;

  // A non-finite median would serialise to `null` and the action coerces that to
  // 0 ms — recorded as an impossibly fast run, which then makes the *next* run
  // look infinitely slower. Fail loudly instead of poisoning the series.
  if (!Number.isFinite(stats.medianMs) || stats.medianMs <= 0) {
    throw new Error(
      `❌ ${stats.name}: median latency is not a positive number (${stats.medianMs})`
    );
  }

  return {
    name: stats.name,
    value: Number(stats.medianMs.toFixed(6)),
    unit: 'ms',
    range: `±${stats.rme}%`,
    extra: `p95 ${stats.p95} | ${stats.opsPerSec} ops/sec | ${samples} samples`,
  };
}

function formatBenchmarkSummary(task: Task): string {
  const stats = getBenchmarkStats(task);
  return `${stats.name}: median ${stats.median}, p95 ${stats.p95}, ${stats.opsPerSec} ops/sec ±${stats.rme}% (${stats.samples} runs sampled)`;
}

function percentile(samples: readonly number[] | undefined, percentile: number): number {
  if (!samples || samples.length === 0) return Number.NaN;

  const index = Math.min(samples.length - 1, Math.ceil((percentile / 100) * samples.length) - 1);
  return samples[index] ?? Number.NaN;
}

function formatLatency(milliseconds: number): string {
  if (!Number.isFinite(milliseconds)) return 'n/a';

  const nanoseconds = milliseconds * 1_000_000;
  if (nanoseconds >= 1_000_000) {
    return `${(nanoseconds / 1_000_000).toLocaleString('en-US', {
      maximumFractionDigits: 2,
    })} ms`;
  }

  if (nanoseconds >= 1_000) {
    return `${(nanoseconds / 1_000).toLocaleString('en-US', {
      maximumFractionDigits: 2,
    })} µs`;
  }

  return `${nanoseconds.toLocaleString('en-US', {
    maximumFractionDigits: 0,
  })} ns`;
}

function getSystemInfo(): string {
  let version = 'unknown';
  try {
    const cargoToml = fs.readFileSync(path.join(workspaceRoot, 'Cargo.toml'), 'utf-8');
    const versionMatch = cargoToml.match(/version\s*=\s*"([^"]+)"/);
    if (versionMatch && versionMatch[1]) {
      version = versionMatch[1];
    }
  } catch (error) {
    console.error('Failed to read Cargo.toml:', error);
  }

  return `
${chalk.bold.yellow('📊 Benchmark Environment:')}
  ${chalk.blue('🕒 Date:')}     ${new Date().toLocaleDateString()} ${new Date().toLocaleTimeString()}
  ${chalk.blue('🧩 Node.js:')}  ${chalk.green(process.version)}
  ${chalk.blue('🔌 Plugin:')}   ${chalk.green('v' + version)}
  ${chalk.blue('💻 OS:')}       ${chalk.green(os.type())} ${os.release()} ${os.arch()}
  ${chalk.blue('⚡ CPU:')}      ${chalk.green(os.cpus()[0]?.model)} × ${os.cpus().length} cores
  ${chalk.blue('🧠 Memory:')}   ${chalk.green(Math.round(os.totalmem() / (1024 * 1024 * 1024)))}GB
`;
}

const stylexFixturePath = path.join(workspaceRoot, 'crates/stylex-transform/tests/fixture');
const fixtureFilePaths = getFixtureFilePaths(stylexFixturePath);

addFixtureBenchmarks(benchRegular, fixtureFilePaths);

const perfFixturesDir = path.join(benchmarkDir, 'perf_fixtures');
const perfFixtures = [
  {
    path: path.join(perfFixturesDir, 'colors.stylex.js'),
    name: 'Colors StyleX transformation',
  },
  {
    path: path.join(perfFixturesDir, 'createTheme-basic.js'),
    name: 'Basic theme transformation',
  },
  {
    path: path.join(perfFixturesDir, 'createTheme-complex.js'),
    name: 'Complex theme transformation',
  },
  {
    path: path.join(perfFixturesDir, 'create-basic.js'),
    name: 'Basic create transformation',
  },
  {
    path: path.join(perfFixturesDir, 'create-complex.js'),
    name: 'Complex create transformation',
  },
] as const;

perfFixtures.forEach(fixture => {
  const content = fs.readFileSync(fixture.path, 'utf-8');
  benchPerformance.add(`Performance - ${fixture.name}`, () => {
    transform(fixture.path, content, stylexOptions);
  });
});

const rollupPluginApp = path.join(workspaceRoot, 'apps/rollup-large-example');
const rollupPluginAppFiles = ['lotsOfStyles.js', 'lotsOfStylesDynamic.js'];

rollupPluginAppFiles.forEach(file => {
  const filePath = path.join(rollupPluginApp, file);
  const content = fs.readFileSync(filePath, 'utf-8');

  benchLotsOfStyles.add(`Rollup plugin - ${file}`, () => {
    transform(filePath, content, stylexOptions);
  });
});

const resultsDir = path.resolve(benchmarkDir, 'results');
if (!fs.existsSync(resultsDir)) {
  fs.mkdirSync(resultsDir, { recursive: true });
}

async function runBenchmarks() {
  const benches = [benchRegular, benchPerformance, benchLotsOfStyles];
  const benchesExtendedOutputs: string[] = [];
  const benchesOutputs: string[] = [];
  const benchmarkEntries: BenchmarkEntry[] = [];

  console.log(chalk.bold('🚀 Running StyleX benchmarks...\n'));

  const timestamp = new Date().toLocaleString();
  benchesExtendedOutputs.push(
    chalk.bold.magenta(`
╔═══════════════════════════════════════════════════╗
║             STYLEX BENCHMARK RESULTS              ║
║             ${timestamp.padEnd(37, ' ')} ║
╚═══════════════════════════════════════════════════╝
`)
  );

  const sysInfo = getSystemInfo();
  benchesExtendedOutputs.push(sysInfo);

  for (const bench of benches) {
    console.log(`\n${chalk.yellow.bold(`Running: ${bench.name}`)}`);
    await bench.run();

    console.log('\nResults:');
    console.table(bench.table());

    benchesExtendedOutputs.push(`\n${chalk.cyan.bold('▶︎ ' + bench.name)}\n`);
    benchesExtendedOutputs.push(chalk.dim('⎯'.repeat(2)));

    bench.tasks.forEach(task => {
      benchesExtendedOutputs.push(formatBenchmarkSummary(task));
      benchmarkEntries.push(toBenchmarkEntry(getBenchmarkStats(task)));
    });

    benchesOutputs.push(...bench.tasks.map(formatBenchmarkSummary));
  }

  benchesExtendedOutputs.push(chalk.dim('\n⎯'));
  benchesExtendedOutputs.push(chalk.bold.green('✓ All benchmarks completed successfully!\n'));

  const extendedOutput = benchesExtendedOutputs.join('\n');
  const outputPath = path.join(resultsDir, 'output.json');
  const extendedOutputPath = path.join(resultsDir, 'output-extended.txt');

  console.log(extendedOutput);

  // `output.json` is consumed by benchmark-action/github-action-benchmark with
  // `tool: 'customSmallerIsBetter'`; keep it in sync with `output-file-path` in
  // .github/workflows/npm.yml and .github/workflows/pr-validation.yml.
  // Human-readable output (median/p95/ops per sec) is written separately.
  fs.writeFileSync(outputPath, `${JSON.stringify(benchmarkEntries, null, 2)}\n`, 'utf8');
  fs.writeFileSync(extendedOutputPath, benchesOutputs.join('\n') + '\n', 'utf8');

  console.log(`\n${chalk.green(`📊 Benchmark results (median latency) saved to ${outputPath}`)}`);
  console.log(chalk.green(`📊 Extended results saved to ${extendedOutputPath}`));
}

runBenchmarks().catch((err: unknown) => {
  console.error(chalk.red('Benchmark failed:'), err);
  process.exit(1);
});
