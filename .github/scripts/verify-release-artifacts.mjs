#!/usr/bin/env node
/**
 * Verifies that the candidate artifacts about to be published match the
 * exact bytes that every paired benchmark leg measured.
 *
 * Contract:
 *   - Reads aggregate/summary.json (produced by the aggregate job artifact
 *     `paired-release-aggregate`) plus each `bindings-<target>` artifact
 *     under $ARTIFACTS_DIR.
 *   - For each identity in the summary, resolves the corresponding native
 *     binary in the bindings artifact and re-hashes it. Any mismatch fails
 *     the publish step before pnpm publish runs.
 */

import fs from 'node:fs';
import path from 'node:path';

import { fail, failWithErrors, requireEnv, sha256File } from './lib/ci.mjs';

const ARTIFACTS_DIR = requireEnv('ARTIFACTS_DIR');
const summaryPath = path.join(ARTIFACTS_DIR, 'paired-release-aggregate', 'summary.json');
if (!fs.existsSync(summaryPath)) {
  fail(`Missing aggregate summary: ${summaryPath}`);
}
const summary = JSON.parse(fs.readFileSync(summaryPath, 'utf8'));
if (!Array.isArray(summary.identities) || summary.identities.length === 0) {
  fail('Aggregate summary has no identities');
}

const errors = [];
for (const value of summary.identities) {
  const artifact = parseReleaseArtifact(value);
  if (artifact === undefined) {
    errors.push(`Incomplete identity entry: ${JSON.stringify(value)}`);
    continue;
  }
  const { target, expectedSha, nativeBasename } = artifact;
  const bindingsDir = path.join(ARTIFACTS_DIR, `bindings-${target}`);
  if (!fs.existsSync(bindingsDir)) {
    errors.push(`Missing bindings artifact directory for ${target}: ${bindingsDir}`);
    continue;
  }
  const candidatePath = path.join(bindingsDir, nativeBasename);
  if (!fs.existsSync(candidatePath)) {
    errors.push(`Missing native binary for ${target}: ${candidatePath}`);
    continue;
  }
  const actualSha = sha256File(candidatePath);
  if (actualSha !== expectedSha) {
    errors.push(
      `Checksum mismatch for ${target}: expected ${expectedSha}, ` +
        `got ${actualSha} (${candidatePath})`
    );
    continue;
  }
  console.log(`ok  ${target}  ${nativeBasename}  sha256=${actualSha.slice(0, 12)}`);
}

if (errors.length > 0) {
  failWithErrors('Release artifact verification failed:', errors);
}

console.log(`All ${summary.identities.length} target(s) verified against benchmark manifests.`);

/**
 * Convert the aggregate identity into the domain record used during
 * publication. Keeping this validation at the boundary prevents path and
 * checksum operations from depending on loosely shaped JSON values.
 */
function parseReleaseArtifact(value) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    return undefined;
  }
  if (
    typeof value.target !== 'string' ||
    !/^[a-zA-Z0-9._-]+$/.test(value.target) ||
    typeof value.nativeSha256 !== 'string' ||
    typeof value.nativeBinary !== 'string' ||
    value.nativeBinary.length === 0 ||
    value.nativeBinary !== path.basename(value.nativeBinary) ||
    !/^[a-f\d]{64}$/.test(value.nativeSha256)
  ) {
    return undefined;
  }
  return {
    target: value.target,
    expectedSha: value.nativeSha256,
    nativeBasename: value.nativeBinary,
  };
}
