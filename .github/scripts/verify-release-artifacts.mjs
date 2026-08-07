#!/usr/bin/env node
/**
 * Verifies that the candidate artifacts about to be published match the
 * exact bytes that every paired benchmark leg measured.
 *
 * Contract:
 *   - Reads aggregate/summary.json (produced by the aggregate job artifact
 *     `paired-release-aggregate`) plus each `bindings-<target>` artifact
 *     under $ARTIFACTS_DIR.
 *   - For each identity in the summary, re-hashes every publishable file in
 *     the corresponding bindings artifact. Any mismatch fails before publish.
 */

import fs from 'node:fs';
import path from 'node:path';

import { parseReleaseBenchmarkIdentity, verifyArtifactFiles } from './lib/benchmark-artifacts.mjs';
import { fail, failWithErrors, requireEnv } from './lib/ci.mjs';

const ARTIFACTS_DIR = requireEnv('ARTIFACTS_DIR');
const summaryPath = path.join(ARTIFACTS_DIR, 'paired-release-aggregate', 'summary.json');
if (!fs.existsSync(summaryPath)) {
  fail(`Missing aggregate summary: ${summaryPath}`);
}
const summary = JSON.parse(fs.readFileSync(summaryPath, 'utf8'));
if (summary.schemaVersion !== 1) {
  fail(`Unsupported aggregate summary schemaVersion: ${String(summary.schemaVersion)}`);
}
if (!Array.isArray(summary.identities) || summary.identities.length === 0) {
  fail('Aggregate summary has no identities');
}
if (!Array.isArray(summary.expectedTargets) || summary.expectedTargets.length === 0) {
  fail('Aggregate summary has no expectedTargets');
}

const errors = [];
const identities = [];
for (const value of summary.identities) {
  let identity;
  try {
    identity = parseReleaseBenchmarkIdentity(value);
  } catch (error) {
    errors.push(`Invalid identity entry: ${error.message}`);
    continue;
  }
  identities.push(identity);
  const bindingsDir = path.join(ARTIFACTS_DIR, `bindings-${identity.target}`);
  if (!fs.existsSync(bindingsDir)) {
    errors.push(`Missing bindings artifact directory for ${identity.target}: ${bindingsDir}`);
    continue;
  }
  const fileErrors = verifyArtifactFiles(bindingsDir, identity.files);
  errors.push(...fileErrors.map(message => `${identity.target}: ${message}`));
  if (fileErrors.length === 0) {
    console.log(`ok  ${identity.target}  ${String(identity.files.length)} files`);
  }
}

const expectedTargets = new Set(summary.expectedTargets);
const actualTargets = new Set(identities.map(identity => identity.target));
if (
  expectedTargets.size !== summary.expectedTargets.length ||
  actualTargets.size !== identities.length ||
  expectedTargets.size !== actualTargets.size ||
  [...expectedTargets].some(target => !actualTargets.has(target))
) {
  errors.push('Aggregate identity targets do not exactly match expectedTargets');
}

if (errors.length > 0) {
  failWithErrors('Release artifact verification failed:', errors);
}

console.log(`All ${summary.identities.length} target(s) verified against benchmark manifests.`);
