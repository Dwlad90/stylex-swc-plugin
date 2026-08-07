/**
 * The two identity documents the PR-side paired benchmark produces.
 *
 * Kept out of `pr-validation.yml` so the gate logic is testable and so the
 * writer and the verifier of the base-subject manifest stay one
 * implementation. `benchmark-artifacts.test.mjs` covers both directions.
 */

import fs from 'node:fs';
import path from 'node:path';

import { equal, positiveInteger, record, safeName, sha, shortString } from './json.mjs';

/** Files a usable base subject must contain, relative to the subject root. */
export const REQUIRED_BASE_SUBJECT_FILES = ['package.json', 'dist/index.js', 'dist/transform.js'];

/**
 * Describe an immutable base subject. The result is written next to the built
 * subject and is also the cache key's payload: any field that changes must
 * invalidate the cache, so this is exactly the set of things that make one
 * built base binary-incompatible with another.
 */
export function buildBaseSubjectManifest(input) {
  return {
    schemaVersion: positiveInteger(input.schemaVersion, 'base subject.schemaVersion'),
    baseSha: sha(input.baseSha, 'base subject.baseSha'),
    target: safeName(input.target, 'base subject.target'),
    nodeAbi: shortString(input.nodeAbi, 'base subject.nodeAbi'),
    toolchainHash: shortString(input.toolchainHash, 'base subject.toolchainHash'),
  };
}

/**
 * Assert that a restored cache entry describes the subject we asked for.
 *
 * Compared field by field rather than by `JSON.stringify` equality, so a
 * mismatch names the field that moved instead of printing two blobs.
 */
export function assertBaseSubjectManifest(actual, expected) {
  const manifest = record(actual, 'base subject manifest');
  for (const [field, want] of Object.entries(buildBaseSubjectManifest(expected))) {
    equal(manifest[field], want, `base subject manifest.${field}`);
  }
  return manifest;
}

/** Throw unless `directory` holds a loadable built subject. */
export function assertBaseSubjectContents(directory) {
  for (const required of REQUIRED_BASE_SUBJECT_FILES) {
    const file = path.join(directory, required);
    if (!fs.existsSync(file) || !fs.statSync(file).isFile()) {
      throw new Error(`Base subject is missing ${required}`);
    }
  }
  const native = fs
    .readdirSync(path.join(directory, 'dist'))
    .filter(name => name.endsWith('.node'));
  if (native.length !== 1) {
    throw new Error(
      `Base subject must contain exactly one native binding (found ${String(native.length)})`
    );
  }
  return native[0];
}

/**
 * Build the identity that binds a paired benchmark artifact to its source run.
 *
 * `headSha` is the PR head commit that was actually merged into the benchmarked
 * tree (`HEAD^2`). It is the only SHA here the trusted reporter can rederive
 * from the `workflow_run` event, so it is what the reporter asserts;
 * `candidateSha` (the test-merge commit) and `baseSha` (the merge-base) are
 * recorded as provenance. See `render-benchmark-report.mjs`.
 */
export function buildPairedBenchmarkIdentity(input) {
  return {
    schemaVersion: 1,
    runId: shortString(input.runId, 'identity.runId'),
    prNumber: positiveInteger(input.prNumber, 'identity.prNumber'),
    headSha: sha(input.headSha, 'identity.headSha'),
    candidateSha: sha(input.candidateSha, 'identity.candidateSha'),
    baseSha: sha(input.baseSha, 'identity.baseSha'),
    target: safeName(input.target, 'identity.target'),
    nodeAbi: shortString(input.nodeAbi, 'identity.nodeAbi'),
    subjectSchemaVersion: positiveInteger(
      input.subjectSchemaVersion,
      'identity.subjectSchemaVersion'
    ),
  };
}

/** Write a document as pretty JSON with a trailing newline, creating parents. */
export function writeJsonDocument(filePath, document) {
  const resolved = path.resolve(filePath);
  fs.mkdirSync(path.dirname(resolved), { recursive: true });
  fs.writeFileSync(resolved, `${JSON.stringify(document, null, 2)}\n`, 'utf8');
  return resolved;
}
