/**
 * Historical single-subject benchmark entry point.
 *
 * Emits three observable artifacts, kept stable for downstream consumers:
 *  - `results/output.json`         — `customSmallerIsBetter` entries used
 *                                    by `github-action-benchmark`.
 *  - `results/output-extended.txt` — human-readable p50/p95/ops report.
 *  - `results/raw-stats.v1.json`   — validated numeric raw stats consumed
 *                                    by the budget check and the verdict
 *                                    engine. Never parse the historical
 *                                    JSON's `extra` string.
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import chalk from 'chalk';
import type { BenchOptions } from 'tinybench';

import { createStylexOptions } from './lib/config.js';
import { captureEnvironment } from './lib/env.js';
import { loadAllFixtures } from './lib/fixtures.js';
import { formatLatency } from './lib/format.js';
import { runRounds } from './lib/runner.js';
import { loadSubject } from './lib/subjects.js';
import {
  RAW_STATS_SCHEMA_VERSION,
  type FixtureRawStats,
  type RawLatencySamples,
  type RawStatsFile,
} from './lib/types.js';

const STANDARD_CONFIG: BenchOptions = {
  retainSamples: true,
  warmup: true,
};

const HEAVY_CONFIG: BenchOptions = {
  retainSamples: true,
  warmup: true,
  time: 500,
  iterations: 10,
  warmupIterations: 1,
  warmupTime: 100,
};

const SUBJECT_LABEL = 'current';

const benchmarkDir = path.dirname(fileURLToPath(import.meta.url));
const packageDir = path.resolve(benchmarkDir, '..');
const workspaceRoot = path.resolve(packageDir, '../..');
const resultsDir = path.resolve(benchmarkDir, 'results');

const stylexOptions = createStylexOptions(packageDir);

interface BenchmarkEntry {
  name: string;
  value: number;
  unit: string;
  range: string;
  extra: string;
}

function toBenchmarkEntry(fixture: FixtureRawStats): BenchmarkEntry {
  const samples = requireCurrent(fixture);
  return {
    name: fixture.name,
    value: Number(samples.p50.toFixed(6)),
    unit: 'ms',
    range: `±${samples.rme.toFixed(2)}%`,
    extra: `p95 ${formatLatency(samples.p95)} | ${samples.opsPerSec.toLocaleString('en-US', {
      maximumFractionDigits: 2,
    })} ops/sec | ${samples.samplesCount} samples`,
  };
}

function formatSummary(fixture: FixtureRawStats): string {
  const samples = requireCurrent(fixture);
  return (
    `${fixture.name}: median ${formatLatency(samples.p50)}, ` +
    `p95 ${formatLatency(samples.p95)}, ` +
    `${samples.opsPerSec.toLocaleString('en-US', { maximumFractionDigits: 2 })} ops/sec ` +
    `±${samples.rme.toFixed(2)}% (${samples.samplesCount} runs sampled)`
  );
}

function requireCurrent(fixture: FixtureRawStats): RawLatencySamples {
  const round = fixture.rounds[0];
  const samples = round?.perSubject[SUBJECT_LABEL];
  if (!samples) {
    throw new Error(`Fixture "${fixture.name}" has no samples for subject "${SUBJECT_LABEL}"`);
  }
  return samples;
}

async function runBenchmarks(): Promise<void> {
  if (!fs.existsSync(resultsDir)) {
    fs.mkdirSync(resultsDir, { recursive: true });
  }

  const fixtures = loadAllFixtures({ packageDir, workspaceRoot });
  const environment = captureEnvironment({ packageDir, workspaceRoot });
  const subject = await loadSubject({ label: SUBJECT_LABEL, packageDir });

  console.log(chalk.bold('Running StyleX benchmarks...\n'));
  console.log(
    `Node ${environment.node} | ${environment.os.type} ${environment.os.release} ` +
      `${environment.os.arch} | ${environment.cpu.model} x${environment.cpu.cores} | ` +
      `plugin v${environment.packageVersion}\n`
  );

  const { fixtures: rawFixtures } = await runRounds({
    subjects: [subject],
    fixtures,
    stylexOptions,
    rounds: 1,
    seed: 1,
    standardBench: STANDARD_CONFIG,
    heavyBench: HEAVY_CONFIG,
  });

  const entries = rawFixtures.map(toBenchmarkEntry);
  const summaryLines = rawFixtures.map(formatSummary);

  console.log(summaryLines.join('\n'));
  console.log(chalk.bold.green('\nAll benchmarks completed.'));

  const rawStats: RawStatsFile = {
    schemaVersion: RAW_STATS_SCHEMA_VERSION,
    environment,
    subjects: [subject.descriptor],
    fixtures: rawFixtures,
  };

  fs.writeFileSync(
    path.join(resultsDir, 'output.json'),
    `${JSON.stringify(entries, null, 2)}\n`,
    'utf8'
  );
  fs.writeFileSync(
    path.join(resultsDir, 'output-extended.txt'),
    summaryLines.join('\n') + '\n',
    'utf8'
  );
  fs.writeFileSync(
    path.join(resultsDir, 'raw-stats.v1.json'),
    `${JSON.stringify(rawStats, null, 2)}\n`,
    'utf8'
  );

  console.log(chalk.green(`\nResults saved to ${resultsDir}`));
}

runBenchmarks().catch((err: unknown) => {
  console.error(chalk.red('Benchmark failed:'), err);
  process.exit(1);
});
