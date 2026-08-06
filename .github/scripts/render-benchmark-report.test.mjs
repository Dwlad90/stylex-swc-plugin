import assert from 'node:assert/strict';
import test from 'node:test';

import {
  BENCHMARK_FIXTURES,
  REPORT_MARKER,
  renderReport,
  validateIdentity,
  validateVerdict,
} from './render-benchmark-report.mjs';
import { resolveBenchmarkSource } from './resolve-benchmark-source.mjs';

function validVerdict() {
  return {
    schemaVersion: 1,
    suiteStatus: 'pass',
    thresholds: { warn: 1.1, fail: 1.2, improvementWarn: 0.5 },
    bootstrap: { seed: 1, resamples: 10_000, confidence: 0.95 },
    subjects: {
      base: { label: 'base', version: '1.0.0', resolvedFrom: '/base/dist/index.js' },
      candidate: { label: 'candidate', version: '1.0.1', resolvedFrom: '/candidate/dist/index.js' },
    },
    fixtures: [...BENCHMARK_FIXTURES].map(([name, category]) => ({
      name,
      category,
      weight: name.startsWith('Rollup') ? 'heavy' : 'standard',
      batchSize: 1,
      base: { label: 'base', perRoundP50: Array.from({ length: 10 }, () => 1) },
      candidate: { label: 'candidate', perRoundP50: Array.from({ length: 10 }, () => 1.01) },
      ratios: Array.from({ length: 10 }, () => 1.01),
      interval: { point: 1.01, lower: 1, upper: 1.02 },
      status: 'pass',
      messages: [],
    })),
    flagged: [],
    hasReproducedFailure: false,
  };
}

void test('renders one marker-delimited escaped report', () => {
  const verdict = validVerdict();
  verdict.fixtures[0].messages = ['unsafe | `note` <tag> [link](url) *bold*'];
  const markdown = renderReport(verdict, {
    runUrl: 'https://github.com/example/repo/actions/runs/1',
    conclusion: 'success',
  });

  assert.equal(markdown.split(REPORT_MARKER).length - 1, 2);
  assert.match(markdown, /unsafe \\| \\`note\\` &lt;tag&gt; \\[link\\]\(url\) \\\*bold\\\*/);
  assert.doesNotMatch(markdown, /<tag>/);
});

void test('binds identity metadata to the trusted source run', () => {
  const identity = {
    schemaVersion: 1,
    runId: '123',
    prNumber: 42,
    candidateSha: 'a'.repeat(40),
    baseSha: 'b'.repeat(40),
    target: 'aarch64-unknown-linux-gnu',
    nodeAbi: '137',
    subjectSchemaVersion: 1,
  };
  const expected = {
    runId: '123',
    prNumber: 42,
    candidateSha: 'a'.repeat(40),
    baseSha: 'b'.repeat(40),
  };

  assert.equal(validateIdentity(identity, expected), identity);
  assert.throws(
    () => validateIdentity({ ...identity, candidateSha: 'c'.repeat(40) }, expected),
    /candidateSha/
  );
});

void test('resolves a merge candidate separately from the source PR head', () => {
  const sourceHeadSha = 'a'.repeat(40);
  const sourceBaseSha = 'b'.repeat(40);
  const currentMergeSha = 'c'.repeat(40);

  assert.deepEqual(
    resolveBenchmarkSource({
      sourceHeadSha,
      sourceBaseSha,
      currentHeadSha: sourceHeadSha,
      currentBaseSha: sourceBaseSha,
      currentMergeSha,
    }),
    { stale: false, candidateSha: currentMergeSha, baseSha: sourceBaseSha }
  );

  assert.deepEqual(
    resolveBenchmarkSource({
      sourceHeadSha,
      sourceBaseSha,
      currentHeadSha: 'd'.repeat(40),
      currentBaseSha: sourceBaseSha,
      currentMergeSha,
    }),
    { stale: true, candidateSha: currentMergeSha, baseSha: sourceBaseSha }
  );

  assert.deepEqual(
    resolveBenchmarkSource({
      sourceHeadSha,
      sourceBaseSha,
      currentHeadSha: sourceHeadSha,
      currentBaseSha: 'e'.repeat(40),
      currentMergeSha,
    }),
    { stale: true, candidateSha: currentMergeSha, baseSha: sourceBaseSha }
  );
});

void test('rejects unsupported schemas and benchmark names', () => {
  const wrongSchema = validVerdict();
  wrongSchema.schemaVersion = 2;
  assert.throws(() => validateVerdict(wrongSchema), /schemaVersion/);

  const unknownFixture = validVerdict();
  unknownFixture.fixtures[0].name = 'untrusted';
  assert.throws(() => validateVerdict(unknownFixture), /allowed benchmark/);
});

void test('rejects non-finite numbers and inconsistent suite status', () => {
  const nonFinite = validVerdict();
  nonFinite.fixtures[0].interval.lower = Number.NaN;
  assert.throws(() => validateVerdict(nonFinite), /must be finite/);

  const inconsistent = validVerdict();
  inconsistent.suiteStatus = 'failed';
  assert.throws(() => validateVerdict(inconsistent), /inconsistent/);
});
