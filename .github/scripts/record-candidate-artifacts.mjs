#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';

import { fail, requireEnv, sha256File } from './lib/ci.mjs';

const artifactDir = path.resolve(requireEnv('ARTIFACT_DIR'));
const outputPath = path.join(artifactDir, 'candidate-artifact-manifest.v1.json');
const requiredFiles = new Set(['index.d.ts', 'index.js', 'transform.d.ts', 'transform.js']);
const names = fs
  .readdirSync(artifactDir, { withFileTypes: true })
  .filter(entry => entry.isFile() && /\.(?:node|js|d\.ts)$/.test(entry.name))
  .map(entry => entry.name)
  .toSorted();

for (const required of requiredFiles) {
  if (!names.includes(required)) fail(`Candidate artifact is missing ${required}`);
}
if (names.filter(name => name.endsWith('.node')).length !== 1) {
  fail('Candidate artifact must contain exactly one native binding');
}

const manifest = {
  schemaVersion: 1,
  target: requireEnv('TARGET'),
  candidateVersion: requireEnv('CANDIDATE_VERSION'),
  releaseRef: requireEnv('RELEASE_REF'),
  files: names.map(name => ({ path: name, sha256: sha256File(path.join(artifactDir, name)) })),
};
fs.writeFileSync(outputPath, `${JSON.stringify(manifest, null, 2)}\n`);
console.log(fs.readFileSync(outputPath, 'utf8'));
