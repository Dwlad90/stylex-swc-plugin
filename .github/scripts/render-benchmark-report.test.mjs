import assert from 'node:assert/strict';
import test from 'node:test';

import { FIXTURE_STATUSES, SUITE_STATUSES } from './lib/json.mjs';
import {
  BENCHMARK_FIXTURES,
  REPORT_MARKER,
  renderReport,
  renderUnavailableReport,
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

function validIdentity() {
  return {
    schemaVersion: 1,
    runId: '123',
    prNumber: 42,
    headSha: 'a'.repeat(40),
    candidateSha: 'b'.repeat(40),
    baseSha: 'c'.repeat(40),
    target: 'aarch64-unknown-linux-gnu',
    nodeAbi: '137',
    subjectSchemaVersion: 1,
  };
}

void test('renders one marker-delimited escaped report', () => {
  const verdict = validVerdict();
  verdict.fixtures[0].messages = ['unsafe | `note` <tag> [link](url) *bold*'];
  const markdown = renderReport(verdict, {
    runUrl: 'https://github.com/example/repo/actions/runs/1',
    conclusion: 'success',
    identity: validIdentity(),
  });

  assert.equal(markdown.split(REPORT_MARKER).length - 1, 2);
  assert.match(markdown, /unsafe \\| \\`note\\` &lt;tag&gt; \\[link\\]\(url\) \\\*bold\\\*/);
  assert.doesNotMatch(markdown, /<tag>/);
});

void test('the unavailable fallback carries the same marker as a real report', () => {
  const markdown = renderUnavailableReport({
    runUrl: 'https://github.com/example/repo/actions/runs/1',
    conclusion: 'failure',
  });

  assert.equal(markdown.split('\n')[0], REPORT_MARKER);
  assert.equal(markdown.split(REPORT_MARKER).length - 1, 2);
  assert.match(markdown, /Suite status: \*\*unavailable\*\*/);
});

void test('binds identity to fields derivable from the workflow_run event', () => {
  const identity = validIdentity();
  const expected = { runId: '123', prNumber: 42, headSha: identity.headSha };

  assert.equal(validateIdentity(identity, expected), identity);
  assert.throws(
    () => validateIdentity({ ...identity, headSha: 'd'.repeat(40) }, expected),
    /headSha/
  );
  assert.throws(() => validateIdentity({ ...identity, runId: '124' }, expected), /runId/);
});

void test('records the merge and merge-base SHAs as provenance, not as assertions', () => {
  // Neither is derivable from the workflow_run event, so a changed value must
  // not reject an otherwise valid report -- only a malformed one may.
  const identity = validIdentity();
  const expected = { runId: '123', prNumber: 42, headSha: identity.headSha };

  assert.doesNotThrow(() =>
    validateIdentity({ ...identity, candidateSha: 'e'.repeat(40) }, expected)
  );
  assert.doesNotThrow(() => validateIdentity({ ...identity, baseSha: 'f'.repeat(40) }, expected));
  assert.throws(() => validateIdentity({ ...identity, baseSha: 'nope' }, expected), /baseSha/);
});

void test('a moved base branch does not invalidate a report for an unchanged head', () => {
  const sourceHeadSha = 'a'.repeat(40);

  assert.deepEqual(
    resolveBenchmarkSource({ sourceHeadSha, currentHeadSha: sourceHeadSha }),
    { stale: false, headSha: sourceHeadSha },
    'the base branch tip is not part of the staleness decision'
  );

  assert.deepEqual(resolveBenchmarkSource({ sourceHeadSha, currentHeadSha: 'd'.repeat(40) }), {
    stale: true,
    headSha: 'd'.repeat(40),
  });

  assert.throws(
    () => resolveBenchmarkSource({ sourceHeadSha, currentHeadSha: 'short' }),
    /currentHeadSha/
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

void test('accepts every status the verdict engine can emit', () => {
  // A vocabulary narrower than the engine's renders a real signal as
  // "unavailable", so assert the whole vocabulary round-trips.
  const flagged = validVerdict();
  flagged.suiteStatus = 'flagged';
  flagged.fixtures[0].status = 'flagged';
  flagged.fixtures[0].messages = ['lower bound 1.400 >= 1.20 — retry required'];
  flagged.flagged = [flagged.fixtures[0].name];

  assert.equal(validateVerdict(flagged).suiteStatus, 'flagged');
  assert.match(
    renderReport(flagged, { runUrl: 'https://example.test/1', conclusion: 'success' }),
    /Suite status: \*\*flagged\*\*/
  );

  for (const status of ['pass', 'warn', 'improvement-warn']) {
    const verdict = validVerdict();
    verdict.fixtures[0].status = status;
    assert.equal(validateVerdict(verdict).fixtures[0].status, status);
  }
});

void test('the shared vocabulary mirrors benchmark/lib/verdict.ts', () => {
  // Kept as an explicit list: a rename in verdict.ts must fail here rather
  // than silently rejecting valid artifacts in production.
  assert.deepEqual([...SUITE_STATUSES], ['pass', 'flagged', 'failed']);
  assert.deepEqual(
    [...FIXTURE_STATUSES],
    ['pass', 'warn', 'improvement-warn', 'flagged', 'failed']
  );
});
