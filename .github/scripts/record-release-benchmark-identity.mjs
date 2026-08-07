#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';

import { parseCandidateArtifactManifest, verifyArtifactFiles } from './lib/benchmark-artifacts.mjs';
import { fail, failWithErrors, requireEnv } from './lib/ci.mjs';

const artifactDir = path.resolve(requireEnv('ARTIFACT_DIR'));
const manifestPath = path.join(artifactDir, 'candidate-artifact-manifest.v1.json');
if (!fs.existsSync(manifestPath)) fail(`Missing candidate artifact manifest: ${manifestPath}`);

const manifest = parseCandidateArtifactManifest(
  JSON.parse(fs.readFileSync(manifestPath, 'utf8')),
  'candidate manifest'
);
const expected = {
  target: requireEnv('TARGET'),
  candidateVersion: requireEnv('CANDIDATE_VERSION'),
  releaseRef: requireEnv('RELEASE_REF'),
};
for (const [field, value] of Object.entries(expected)) {
  if (manifest[field] !== value) {
    fail(`Candidate manifest ${field}=${manifest[field]} does not match ${value}`);
  }
}

const errors = verifyArtifactFiles(artifactDir, manifest.files);
if (errors.length > 0) failWithErrors('Candidate artifact verification failed:', errors);

const identity = {
  schemaVersion: 1,
  target: manifest.target,
  targetLabel: requireEnv('TARGET_LABEL'),
  node: requireEnv('NODE_VERSION'),
  releaseRef: manifest.releaseRef,
  candidateVersion: manifest.candidateVersion,
  previousVersion: requireEnv('PREVIOUS_VERSION'),
  runId: requireEnv('GITHUB_RUN_ID'),
  subjectSchemaVersion: Number(requireEnv('SUBJECT_SCHEMA_VERSION')),
  files: manifest.files,
};
const outputPath = path.resolve(requireEnv('IDENTITY_OUTPUT'));
fs.mkdirSync(path.dirname(outputPath), { recursive: true });
fs.writeFileSync(outputPath, `${JSON.stringify(identity, null, 2)}\n`);
console.log(`Verified ${String(manifest.files.length)} candidate files for ${manifest.target}`);
