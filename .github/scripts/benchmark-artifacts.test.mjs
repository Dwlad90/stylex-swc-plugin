import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import {
  findNativeArtifact,
  parseCandidateArtifactManifest,
  parseReleaseBenchmarkIdentity,
  parseReleaseVerdict,
  verifyArtifactFiles,
} from './lib/benchmark-artifacts.mjs';

const FILES = [
  { path: 'index.js', sha256: 'a'.repeat(64) },
  { path: 'rs-compiler.linux-x64-gnu.node', sha256: 'b'.repeat(64) },
];

function validVerdict() {
  return {
    schemaVersion: 1,
    suiteStatus: 'pass',
    thresholds: { warn: 1.1, fail: 1.2, improvementWarn: 0.5 },
    bootstrap: { seed: 1, resamples: 10_000, confidence: 0.95 },
    subjects: {
      base: { label: 'base', version: '1.2.2', resolvedFrom: '/base/dist/index.js' },
      candidate: {
        label: 'candidate',
        version: '1.2.3',
        resolvedFrom: '/candidate/dist/index.js',
      },
    },
    fixtures: [
      {
        name: 'transform',
        category: 'transform',
        weight: 'standard',
        batchSize: 1,
        base: { label: 'base', perRoundP50: [1, 1] },
        candidate: { label: 'candidate', perRoundP50: [1.01, 1.01] },
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

void test('release verdict validation accepts the complete gate artifact', () => {
  assert.equal(parseReleaseVerdict(validVerdict()).suiteStatus, 'pass');
});

void test('release verdict validation rejects an incomplete versioned artifact', () => {
  assert.throws(
    () => parseReleaseVerdict({ schemaVersion: 1, suiteStatus: 'pass' }, 'verdict'),
    /verdict\.thresholds must be an object/
  );
  assert.throws(
    () => parseReleaseVerdict({ schemaVersion: 2 }, 'verdict'),
    /verdict\.schemaVersion must equal 1/
  );
});

void test('candidate manifest validation accepts a complete safe file set', () => {
  const manifest = parseCandidateArtifactManifest({
    schemaVersion: 1,
    target: 'x86_64-unknown-linux-gnu',
    candidateVersion: '1.2.3',
    releaseRef: '1.2.3',
    files: FILES,
  });

  assert.equal(manifest.files.length, 2);
});

void test('release identity validation requires one native artifact and complete identity fields', () => {
  const identity = parseReleaseBenchmarkIdentity({
    schemaVersion: 1,
    target: 'x86_64-unknown-linux-gnu',
    targetLabel: 'Linux x64',
    node: '24.18.0',
    releaseRef: '1.2.3',
    candidateVersion: '1.2.3',
    previousVersion: '1.2.2',
    runId: '123',
    subjectSchemaVersion: 1,
    files: FILES,
  });

  assert.equal(findNativeArtifact(identity).path, 'rs-compiler.linux-x64-gnu.node');
  assert.throws(
    () => parseReleaseBenchmarkIdentity({ ...identity, files: FILES.slice(0, 1) }),
    /exactly one native artifact/
  );
});

void test('artifact verification detects changed and missing publication files', () => {
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), 'stylex-artifacts-'));
  fs.writeFileSync(path.join(directory, 'index.js'), 'candidate');
  fs.writeFileSync(path.join(directory, 'transform.js'), 'unlisted');

  const errors = verifyArtifactFiles(directory, [
    { path: 'index.js', sha256: '0'.repeat(64) },
    { path: 'index.d.ts', sha256: '1'.repeat(64) },
  ]);

  assert.equal(errors.length, 3);
  assert.match(errors[0], /Checksum mismatch/);
  assert.match(errors[1], /Missing artifact/);
  assert.match(errors[2], /Unmanifested artifact/);
});
