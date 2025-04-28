import { Bench, type BenchOptions, Task } from 'tinybench';
import { transform } from '../dist/index.js';
import type { StyleXOptions } from '../dist/index.js';
import path from 'path';
import fs from 'fs';
import os from 'os';
import chalk from 'chalk';

const BENCHMARK_CONFIG: BenchOptions = {
  warmup: true,
};

const LOTS_OF_STYLES_CONFIG = {
  ...BENCHMARK_CONFIG,
  time: 500,
  iterations: 10,
  warmupIterations: 1,
  warmupTime: 100,
};

const rootDir = process.cwd();

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
  genConditionalClasses: true,
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
    const separator = file.includes('/') ? '/' : '\\';
    const benchmarkName = file.split(separator).at(-2) ?? 'Default case';

    bench.add(benchmarkName, () => {
      transform(file, content, stylexOptions);
    });
  });
}

function formatBenchmarkSummary(task: Task): string {
  const { name, result } = task;
  if (!result) return chalk.red(`❌ ${name}: No results`);

  const opsPerSec = result.throughput.mean.toLocaleString('en-US', {
    maximumFractionDigits: 2,
  });

  const rme = result.latency.rme.toFixed(2);
  const samples = result.latency?.samples?.length ?? 0;

  return `${name} x ${opsPerSec} ops/sec ±${rme}% (${samples} runs sampled)`;
}

function getSystemInfo(): string {
  let version = 'unknown';
  try {
    const cargoToml = fs.readFileSync(path.join(rootDir, '../..', 'Cargo.toml'), 'utf-8');
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
  ${chalk.blue('⚡ CPU:')}      ${chalk.green(os.cpus()[0].model)} × ${os.cpus().length} cores
  ${chalk.blue('🧠 Memory:')}   ${chalk.green(Math.round(os.totalmem() / (1024 * 1024 * 1024)))}GB
`;
}

const stylexFixturePath = path.join(rootDir, '../../crates/stylex-shared/tests/fixture');
const fixtureFilePaths = getFixtureFilePaths(stylexFixturePath);

addFixtureBenchmarks(benchRegular, fixtureFilePaths);

const perfFixturesDir = path.join(rootDir, 'benchmark/perf_fixtures');
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

const rollupPluginApp = path.join(rootDir, '../../apps/rollup-example');
const rollupPluginAppFiles = ['lotsOfStyles.js', 'lotsOfStylesDynamic.js'];

rollupPluginAppFiles.forEach(file => {
  const filePath = path.join(rollupPluginApp, file);
  const content = fs.readFileSync(filePath, 'utf-8');

  benchLotsOfStyles.add(`Rollup plugin - ${file}`, () => {
    transform(filePath, content, stylexOptions);
  });
});

const resultsDir = path.resolve(rootDir, 'benchmark/results');
if (!fs.existsSync(resultsDir)) {
  fs.mkdirSync(resultsDir, { recursive: true });
}

async function runBenchmarks() {
  const benches = [benchRegular, benchPerformance, benchLotsOfStyles];
  const benchesExtendedOutputs = [];
  const benchesOutputs = [];

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
    });

    benchesOutputs.push(...bench.tasks.map(formatBenchmarkSummary));
  }

  benchesExtendedOutputs.push(chalk.dim('\n⎯'));
  benchesExtendedOutputs.push(chalk.bold.green('✓ All benchmarks completed successfully!\n'));

  const output = benchesOutputs.join('\n');
  const extendedOutput = benchesExtendedOutputs.join('\n');
  const outputPath = path.join(resultsDir, 'output.txt');

  console.log(extendedOutput);
  fs.writeFileSync(path.join(resultsDir, 'output.txt'), output, 'utf8');

  console.log(`\n${chalk.green(`📊 Benchmark results saved to ${outputPath}`)}`);
}

runBenchmarks().catch(err => {
  console.error(chalk.red('Benchmark failed:'), err);
  process.exit(1);
});
