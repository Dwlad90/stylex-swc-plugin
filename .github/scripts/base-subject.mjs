#!/usr/bin/env node
/**
 * Write or verify the manifest of the immutable base subject used by the
 * PR-side paired benchmark.
 *
 *   node .github/scripts/base-subject.mjs write
 *   node .github/scripts/base-subject.mjs verify
 *
 * `write` runs after the base subject is built; `verify` runs after it is
 * restored from cache and fails the job when the restored tree does not
 * describe the base commit this run needs. Both read the same environment,
 * so the writer and the verifier cannot disagree about what a manifest holds.
 */

import fs from 'node:fs';
import path from 'node:path';

import { fail, requireEnv } from './lib/ci.mjs';
import {
  assertBaseSubjectContents,
  assertBaseSubjectManifest,
  buildBaseSubjectManifest,
  writeJsonDocument,
} from './lib/paired-benchmark.mjs';

const MODES = new Set(['write', 'verify']);

const mode = process.argv[2];
if (!MODES.has(mode)) fail(`Usage: base-subject.mjs <${[...MODES].join('|')}>`);

const subjectDir = path.resolve(requireEnv('BASE_SUBJECT'));
const manifestPath = path.join(subjectDir, 'manifest.json');
const expected = {
  schemaVersion: Number(requireEnv('BENCHMARK_SUBJECT_SCHEMA_VERSION')),
  baseSha: requireEnv('BASE_SHA'),
  target: requireEnv('BENCHMARK_TARGET'),
  nodeAbi: requireEnv('NODE_ABI'),
  toolchainHash: requireEnv('TOOLCHAIN_HASH'),
};

try {
  if (mode === 'write') {
    writeJsonDocument(manifestPath, buildBaseSubjectManifest(expected));
    console.log(`Wrote base subject manifest for ${expected.baseSha}`);
  } else {
    assertBaseSubjectManifest(JSON.parse(fs.readFileSync(manifestPath, 'utf8')), expected);
    const native = assertBaseSubjectContents(subjectDir);
    console.log(`Restored base subject ${expected.baseSha} (${native})`);
  }
} catch (error) {
  fail(error instanceof Error ? error.message : String(error));
}
