import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  ensureViewerPage,
  historyDataDir,
  VIEWER_TEMPLATE,
  VIEWER_TEMPLATE_SOURCE,
} from './lib/benchmark-history.mjs';

const SCRIPT = fileURLToPath(new URL('./aggregate-release-benchmarks.mjs', import.meta.url));
const WORKFLOW = fileURLToPath(new URL('../workflows/pr-validation.yml', import.meta.url));
const TARGET = 'x86_64-unknown-linux-gnu';
const NODE_VERSION = '24.18.0';

const FILES = [
  { path: 'index.js', sha256: 'a'.repeat(64) },
  { path: 'rs-compiler.linux-x64-gnu.node', sha256: 'b'.repeat(64) },
];

function passingVerdict() {
  return {
    schemaVersion: 1,
    suiteStatus: 'pass',
    thresholds: { warn: 1.1, fail: 1.2, improvementWarn: 0.5 },
    bootstrap: { seed: 1, resamples: 10_000, confidence: 0.95 },
    subjects: {
      base: { label: '1.2.2', version: '1.2.2', resolvedFrom: '/base/dist/index.js' },
      candidate: { label: '1.2.3', version: '1.2.3', resolvedFrom: '/candidate/dist/index.js' },
    },
    fixtures: [
      {
        name: 'transform',
        category: 'transform',
        weight: 'standard',
        batchSize: 1,
        base: { label: '1.2.2', perRoundP50: [1, 1] },
        candidate: { label: '1.2.3', perRoundP50: [1.01, 1.01] },
        ratios: [1.01, 1.01],
        interval: { point: 1.01, lower: 1, upper: 1.02 },
        status: 'pass',
        messages: [],
      },
    ],
    flagged: [],
    hasReproducedFailure: false,
  };
}

/**
 * Lay out one release report artifact plus an empty history checkout in a
 * scratch git repo. The script reads release commit metadata from `git log`, so
 * the tree has to be a real repository with at least one commit.
 */
function setUp(t) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'aggregate-release-'));
  t.after(() => fs.rmSync(root, { recursive: true, force: true }));

  const reportDir = path.join(root, 'reports', `paired-release-report-${TARGET}`);
  const pagesDir = path.join(root, 'pages');
  fs.mkdirSync(reportDir, { recursive: true });
  fs.mkdirSync(pagesDir, { recursive: true });

  fs.writeFileSync(
    path.join(reportDir, 'compare-revisions.verdict.v1.json'),
    JSON.stringify(passingVerdict())
  );
  fs.writeFileSync(
    path.join(reportDir, 'release-benchmark-identity.v1.json'),
    JSON.stringify({
      schemaVersion: 1,
      target: TARGET,
      targetLabel: 'Linux x64',
      node: NODE_VERSION,
      releaseRef: '1.2.3',
      candidateVersion: '1.2.3',
      previousVersion: '1.2.2',
      runId: '123',
      subjectSchemaVersion: 1,
      files: FILES,
    })
  );
  fs.writeFileSync(
    path.join(reportDir, 'output.json'),
    JSON.stringify([{ name: 'transform', value: 1.5, unit: 'ms', range: '', extra: '' }])
  );

  const git = args => execFileSync('git', args, { cwd: root, stdio: 'pipe' });
  git(['init', '-q']);
  git(['config', 'user.email', 'test@example.com']);
  git(['config', 'user.name', 'Test']);
  git(['commit', '-q', '--allow-empty', '-m', 'release']);

  return { root, dataDir: historyDataDir(pagesDir, TARGET, NODE_VERSION) };
}

/** Run the aggregate script against a scratch tree laid out by `setUp`. */
function runAggregate(root) {
  execFileSync(process.execPath, [SCRIPT], {
    cwd: root,
    stdio: 'pipe',
    env: {
      ...process.env,
      EXPECTED_TARGETS: TARGET,
      REPORTS_DIR: path.join(root, 'reports'),
      PAGES_DIR: path.join(root, 'pages'),
      NODE_VERSION,
      REPO_URL: 'https://github.com/example/repo',
      RELEASE_REF: '1.2.3',
      CANDIDATE_VERSION: '1.2.3',
      PREVIOUS_VERSION: '1.2.2',
    },
  });
}

void test('viewer page is added to a history directory that has none', t => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'viewer-'));
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }));

  assert.equal(ensureViewerPage(dir), true);
  const page = fs.readFileSync(path.join(dir, 'index.html'), 'utf8');
  assert.match(page, /<script src="data\.js"><\/script>/);
  assert.match(page, /window\.BENCHMARK_DATA/);
});

void test('a viewer page the action wrote is left untouched', t => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'viewer-'));
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }));

  // Seeded before the first call, so this is a page github-action-benchmark
  // wrote and this code has never seen -- not one of our own copies.
  const page = path.join(dir, 'index.html');
  fs.writeFileSync(page, '<!-- action-owned -->');

  assert.equal(ensureViewerPage(dir), false);
  assert.equal(fs.readFileSync(page, 'utf8'), '<!-- action-owned -->');
});

void test('history update writes a viewer page alongside data.js', t => {
  const { root, dataDir } = setUp(t);
  runAggregate(root);

  // A target this script appends to but that github-action-benchmark never ran
  // on directly would otherwise accumulate data with no page to read it.
  assert.ok(fs.existsSync(path.join(dataDir, 'index.html')));
  assert.match(
    fs.readFileSync(path.join(dataDir, 'data.js'), 'utf8'),
    /^window\.BENCHMARK_DATA = /
  );
});

void test('the vendored viewer matches the pinned github-action-benchmark release', () => {
  // The template is only interchangeable with an action-written page for the
  // release it came from, so a bumped pin must fail here and force a refresh.
  const { action } = VIEWER_TEMPLATE_SOURCE;
  const pin = fs
    .readFileSync(WORKFLOW, 'utf8')
    .match(new RegExp(`uses:\\s*${action}@([0-9a-f]{40})\\s*#\\s*(v\\S+)`));

  assert.ok(pin, `no pinned \`${action}\` reference found in pr-validation.yml`);
  assert.deepEqual({ action, ref: pin[1], version: pin[2] }, { ...VIEWER_TEMPLATE_SOURCE });
  assert.ok(fs.existsSync(VIEWER_TEMPLATE));
});
