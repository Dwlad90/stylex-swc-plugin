import { Bench, Task } from 'tinybench';

import { transform } from '../dist/index.js';
import type { StyleXOptions } from '../dist/index.js';

import path from 'path';
import fs from 'fs';

const rootDir = process.cwd();

const benchRegular = new Bench({
  name: 'StyleX compiler - regular benchmark',
  warmup: true,
});

const benchLotsOfStyles = new Bench({
  name: 'StyleX compiler - lots of styles benchmark',
  warmup: true,
  time: 500,
  iterations: 10,
  warmupIterations: 1,
  warmupTime: 100,
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

const stylexFixturePath = path.join(rootDir, '../../crates/stylex-shared/tests/fixture');
const fixtureFilePaths = getFixtureFilePaths(stylexFixturePath);

fixtureFilePaths.forEach(file => {
  const content = fs.readFileSync(file, 'utf-8');
  const separator = file.includes('/') ? '/' : '\\';

  benchRegular.add(file.split(separator).at(-2) ?? 'Default case', () => {
    transform(file, content, stylexOptions);
  });
});

const rollupPluginApp = path.join(rootDir, '../../apps/rollup-example');

const rollupPluginAppFiles = ['lotsOfStyles.js', 'lotsOfStylesDynamic.js'];

rollupPluginAppFiles.forEach(file => {
  benchLotsOfStyles.add(`Rollup plugin - ${file}`, () => {
    const filePath = path.join(rollupPluginApp, file);

    transform(filePath, fs.readFileSync(filePath, 'utf-8'), stylexOptions);
  });
});

const resultsDir = path.resolve(rootDir, 'benchmark/results');

if (!fs.existsSync(resultsDir)) {
  fs.mkdirSync(resultsDir);
}

await benchRegular.run();
await benchLotsOfStyles.run();

console.table(benchRegular.table());
console.table(benchLotsOfStyles.table());

const output = [
  ...benchRegular.tasks.map(formatBenchmarkSummary),
  ...benchLotsOfStyles.tasks.map(formatBenchmarkSummary),
].join('\n');

fs.writeFileSync(path.join(resultsDir, 'output.txt'), output, 'utf8');
console.log('Benchmark results saved to output.txt');

function formatBenchmarkSummary(task: Task): string {
  const { name, result } = task;
  return `${name} x ${result?.throughput.mean || 0} ops/sec ±${result?.latency.rme || 0}% (${result?.latency?.samples?.length ?? 0} runs sampled)`;
}
