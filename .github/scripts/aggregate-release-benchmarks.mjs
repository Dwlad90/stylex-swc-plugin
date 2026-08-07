#!/usr/bin/env node
/**
 * Aggregates paired release benchmark reports across all runnable targets.
 *
 * Contract:
 *   - Every expected target must have exactly one report artifact.
 *   - Missing artifact, missing file, unsupported schema, non-passing verdict,
 *     or mismatched identity fields fails the release.
 *   - Writes a combined Markdown summary (aggregate/summary.md) and a
 *     versioned JSON summary (aggregate/summary.json) that the publish job
 *     uses to verify checksums.
 *   - Appends one entry per target to the historical `benchmarks` branch
 *     data.js files. One commit is produced by the workflow after this
 *     script finishes.
 *
 * @typedef {Object} Identity
 * @property {string} target
 * @property {string} targetLabel
 * @property {string} node
 * @property {string} releaseRef
 * @property {string} candidateVersion
 * @property {string} previousVersion
 * @property {{path: string, sha256: string}[]} files
 */

import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

import {
  findNativeArtifact,
  parseReleaseBenchmarkIdentity,
  parseReleaseVerdict,
} from './lib/benchmark-artifacts.mjs';
import { fail, failWithErrors, requireEnv } from './lib/ci.mjs';
/**
 * Suite statuses emitted by `bench:verdict` (see benchmark/lib/verdict.ts).
 * Only `pass` may publish: `failed` is a reproduced breach, and `flagged`
 * means a breach was detected but the retry never resolved it.
 *
 * Both the vocabulary and the passing value come from `lib/json.mjs`, the one
 * CI-side mirror of the engine's statuses. Never hard-code a status string
 * here: checking against the shared set means a renamed status fails loudly
 * instead of silently inverting the gate in either direction.
 */
import { PASSING_SUITE_STATUS, SUITE_STATUSES } from './lib/json.mjs';

const EXPECTED_TARGETS = (process.env.EXPECTED_TARGETS ?? '')
  .split(',')
  .map(s => s.trim())
  .filter(Boolean);

const REPORTS_DIR = requireEnv('REPORTS_DIR');
const PAGES_DIR = requireEnv('PAGES_DIR');
const NODE_VERSION = requireEnv('NODE_VERSION');
const REPO_URL = requireEnv('REPO_URL');
const RELEASE_REF = requireEnv('RELEASE_REF');
const CANDIDATE_VERSION = requireEnv('CANDIDATE_VERSION');
const PREVIOUS_VERSION = requireEnv('PREVIOUS_VERSION');

if (EXPECTED_TARGETS.length === 0) {
  fail('EXPECTED_TARGETS is empty; refusing to aggregate.');
}

const releaseCommit = readReleaseCommit();

const results = [];
const errors = [];

for (const target of EXPECTED_TARGETS) {
  const reportDir = path.join(REPORTS_DIR, `paired-release-report-${target}`);
  if (!fs.existsSync(reportDir)) {
    errors.push(`Missing artifact directory for target ${target}: ${reportDir}`);
    continue;
  }
  const verdictPath = path.join(reportDir, 'compare-revisions.verdict.v1.json');
  const identityPath = path.join(reportDir, 'release-benchmark-identity.v1.json');
  const benchOutputPath = path.join(reportDir, 'output.json');

  if (!fs.existsSync(verdictPath)) {
    errors.push(`Missing verdict file for ${target}: ${verdictPath}`);
    continue;
  }
  if (!fs.existsSync(identityPath)) {
    errors.push(`Missing identity file for ${target}: ${identityPath}`);
    continue;
  }

  let verdict;
  let identity;
  try {
    verdict = parseReleaseVerdict(
      JSON.parse(fs.readFileSync(verdictPath, 'utf8')),
      `${target} verdict`
    );
    identity = parseReleaseBenchmarkIdentity(
      JSON.parse(fs.readFileSync(identityPath, 'utf8')),
      `${target} identity`
    );
  } catch (error) {
    errors.push(`Invalid versioned artifact for ${target}: ${error.message}`);
    continue;
  }
  const benchmarkOutput = fs.existsSync(benchOutputPath)
    ? JSON.parse(fs.readFileSync(benchOutputPath, 'utf8'))
    : null;

  const mismatch = findIdentityMismatch(identity, target);
  if (mismatch) {
    errors.push(mismatch);
    continue;
  }

  const status = verdict.suiteStatus;
  if (!SUITE_STATUSES.has(status)) {
    errors.push(
      `Target ${target} verdict suiteStatus=${status} is not a recognised status ` +
        `(expected one of ${[...SUITE_STATUSES].join(', ')})`
    );
  } else if (status !== PASSING_SUITE_STATUS) {
    errors.push(`Target ${target} verdict suiteStatus=${status}`);
  }
  results.push({ target, verdict, identity, benchmarkOutput });
}

writeSummary(results);

if (errors.length > 0) {
  failWithErrors('Aggregation failed:', errors);
}

updateHistory(results);
console.log('Aggregation succeeded.');

/**
 * Compare an identity against the expected release context. Returns the first
 * mismatch message, or undefined when everything lines up.
 *
 * @param {Partial<Identity>} identity
 * @param {string} target
 */
function findIdentityMismatch(identity, target) {
  const expected = {
    target,
    releaseRef: RELEASE_REF,
    candidateVersion: CANDIDATE_VERSION,
    previousVersion: PREVIOUS_VERSION,
  };
  for (const [field, want] of Object.entries(expected)) {
    if (identity[field] !== want) {
      return `Identity ${field} mismatch for ${target}: got ${identity[field]}, expected ${want}`;
    }
  }
  return undefined;
}

function writeSummary(entries) {
  const summaryLines = [
    `## Paired release benchmark summary`,
    '',
    `- release ref: \`${RELEASE_REF}\``,
    `- candidate: \`${CANDIDATE_VERSION}\``,
    `- previous:  \`${PREVIOUS_VERSION}\``,
    '',
    '| target | verdict | flagged | native sha256 |',
    '| --- | --- | --- | --- |',
  ];
  for (const target of EXPECTED_TARGETS) {
    const r = entries.find(x => x.target === target);
    if (!r) {
      summaryLines.push(`| ${target} | **MISSING** | - | - |`);
      continue;
    }
    const flagged = (r.verdict.flagged ?? []).join(', ') || '-';
    const sha = findNativeArtifact(r.identity).sha256.slice(0, 12);
    summaryLines.push(`| ${target} | ${r.verdict.suiteStatus} | ${flagged} | \`${sha}\` |`);
  }
  summaryLines.push('');

  fs.mkdirSync('aggregate', { recursive: true });
  fs.writeFileSync('aggregate/summary.md', summaryLines.join('\n') + '\n');
  fs.writeFileSync(
    'aggregate/summary.json',
    JSON.stringify(
      {
        schemaVersion: 1,
        releaseRef: RELEASE_REF,
        candidateVersion: CANDIDATE_VERSION,
        previousVersion: PREVIOUS_VERSION,
        generatedAt: new Date().toISOString(),
        expectedTargets: EXPECTED_TARGETS,
        identities: entries.map(r => r.identity),
      },
      null,
      2
    ) + '\n'
  );
}

function updateHistory(entries) {
  const nowMs = Date.now();
  for (const { target, benchmarkOutput, identity } of entries) {
    if (!benchmarkOutput || !Array.isArray(benchmarkOutput)) {
      console.warn(`Skipping history update for ${target}: no output.json entries`);
      continue;
    }
    const dataDir = path.join(PAGES_DIR, 'dev/bench/releases', target, `node-${NODE_VERSION}`);
    fs.mkdirSync(dataDir, { recursive: true });
    const dataFile = path.join(dataDir, 'data.js');
    const existing = readDataFile(dataFile);
    const benches = benchmarkOutput.map(entry => ({
      name: String(entry.name ?? ''),
      value: Number(entry.value ?? 0),
      unit: String(entry.unit ?? ''),
      range: entry.range ? String(entry.range) : '',
      extra: entry.extra ? String(entry.extra) : '',
    }));
    const historyEntry = {
      commit: releaseCommit,
      date: nowMs,
      tool: 'customSmallerIsBetter',
      benches,
      release: {
        ref: RELEASE_REF,
        candidateVersion: CANDIDATE_VERSION,
        previousVersion: PREVIOUS_VERSION,
        target,
        nativeSha256: findNativeArtifact(identity).sha256,
      },
    };
    // Must match github-action-benchmark's `name` input, which neither
    // workflow sets, so it is the action's default "Benchmark". The files
    // already on the `benchmarks` branch use that key; appending under any
    // other name silently forks each target's history into two series
    // instead of continuing the existing one.
    const suiteName = 'Benchmark';
    if (!existing.entries[suiteName]) existing.entries[suiteName] = [];
    existing.entries[suiteName].push(historyEntry);
    existing.lastUpdate = nowMs;
    existing.repoUrl = REPO_URL;
    writeDataFile(dataFile, existing);
    console.log(`Updated history for ${target}: ${dataFile}`);
  }
}

function readDataFile(file) {
  if (!fs.existsSync(file)) {
    return { lastUpdate: 0, repoUrl: REPO_URL, entries: {} };
  }
  const raw = fs.readFileSync(file, 'utf8');
  // The trailing semicolon is optional: `writeDataFile` below emits one, but
  // the files github-action-benchmark already wrote to the `benchmarks`
  // branch have none. Requiring it would reject every pre-existing history.
  const match = raw.match(/window\.BENCHMARK_DATA\s*=\s*([\s\S]*?);?\s*$/);
  if (!match) {
    throw new Error(`Malformed benchmark history file: ${file}`);
  }
  return JSON.parse(match[1]);
}

function writeDataFile(file, data) {
  fs.writeFileSync(file, `window.BENCHMARK_DATA = ${JSON.stringify(data, null, 2)};\n`);
}

/**
 * Read release commit metadata from the checked-out release ref so the
 * historical data.js entries match what github-action-benchmark would have
 * written when it clones the repo itself.
 */
function readReleaseCommit() {
  const separator = '\u001F';
  const format = ['%H', '%T', '%an', '%ae', '%cn', '%ce', '%aI', '%s'].join(separator);
  try {
    const raw = execFileSync('git', ['log', '-1', `--format=${format}`, 'HEAD'], {
      encoding: 'utf8',
    }).trim();
    const [id, tree, authorName, authorEmail, committerName, committerEmail, timestamp, message] =
      raw.split(separator);
    return {
      author: { name: authorName, email: authorEmail, username: authorName },
      committer: { name: committerName, email: committerEmail, username: committerName },
      id,
      message,
      timestamp,
      tree_id: tree,
      url: `${REPO_URL}/commit/${id}`,
    };
  } catch (error) {
    // `fail` terminates the process; the throw keeps the function's
    // return type honest for callers and linters.
    fail(`Unable to read release commit metadata: ${error.message}`);
    throw error;
  }
}
