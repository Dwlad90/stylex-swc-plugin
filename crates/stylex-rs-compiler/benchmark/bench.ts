import { Bench, type BenchOptions, type Task, type TaskResultWithStatistics } from 'tinybench';
import { transform } from '../dist/index.js';
import type { StyleXOptions } from '../dist/index.js';
import path from 'path';
import fs from 'fs';
import os from 'os';
import chalk from 'chalk';
import { fileURLToPath } from 'url';

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

  return {
    name,
    opsPerSec: result.throughput.mean.toLocaleString('en-US', {
      maximumFractionDigits: 2,
    }),
    rme: result.latency.rme.toFixed(2),
    samples: result.latency.samplesCount,
    median: formatLatency(result.latency.p50),
    p95: formatLatency(percentile(result.latency.samples, 95)),
  };
}

/**
 * Strict Benchmark.js-compatible single-line summary.
 *
 * Required by `benchmark-action/github-action-benchmark` with
 * `tool: 'benchmarkjs'`, whose parser regex is:
 *   /^(.+) x ([0-9,.]+) ops\/sec ±([0-9.]+)% \((\d+) runs? sampled\)$/
 *
 * Any deviation (e.g. `:` instead of ` x `, extra median/p95 fields, or
 * `0 runs sampled`) makes the action skip the line and ultimately fail with
 * "No benchmark result was found in <output.txt>". This is the format the
 * GitHub Action consumes; do not change it without updating the workflow.
 */
function formatBenchmarkjsLine(stats: BenchmarkStats): string {
  const samples = stats.samples > 0 ? stats.samples : 1;
  return `${stats.name} x ${stats.opsPerSec} ops/sec ±${stats.rme}% (${samples} runs sampled)`;
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
  const benchmarkjsLines: string[] = [];

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

  for await (const bench of benches) {
    console.log(`\n${chalk.yellow.bold(`Running: ${bench.name}`)}`);
    await bench.run();

    console.log('\nResults:');
    console.table(bench.table());

    benchesExtendedOutputs.push(`\n${chalk.cyan.bold('▶︎ ' + bench.name)}\n`);
    benchesExtendedOutputs.push(chalk.dim('⎯'.repeat(2)));

    bench.tasks.forEach(task => {
      benchesExtendedOutputs.push(formatBenchmarkSummary(task));
      benchmarkjsLines.push(formatBenchmarkjsLine(getBenchmarkStats(task)));
    });

    benchesOutputs.push(...bench.tasks.map(formatBenchmarkSummary));
  }

  benchesExtendedOutputs.push(chalk.dim('\n⎯'));
  benchesExtendedOutputs.push(chalk.bold.green('✓ All benchmarks completed successfully!\n'));

  const extendedOutput = benchesExtendedOutputs.join('\n');
  const outputPath = path.join(resultsDir, 'output.txt');
  const extendedOutputPath = path.join(resultsDir, 'output-extended.txt');

  console.log(extendedOutput);

  // `output.txt` MUST be in strict Benchmark.js format — it is consumed by
  // benchmark-action/github-action-benchmark (tool: 'benchmarkjs') in CI.
  // Human-readable extended output (median/p95) is written separately.
  fs.writeFileSync(outputPath, benchmarkjsLines.join('\n') + '\n', 'utf8');
  fs.writeFileSync(extendedOutputPath, benchesOutputs.join('\n') + '\n', 'utf8');

  console.log(`\n${chalk.green(`📊 Benchmark results (benchmarkjs) saved to ${outputPath}`)}`);
  console.log(`${chalk.green(`📊 Extended results saved to ${extendedOutputPath}`)}`);
}

runBenchmarks().catch(err => {
  console.error(chalk.red('Benchmark failed:'), err);
  process.exit(1);
});
