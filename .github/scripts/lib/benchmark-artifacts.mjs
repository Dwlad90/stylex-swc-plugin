import fs from 'node:fs';
import path from 'node:path';

import { sha256File } from './ci.mjs';

const SHA256 = /^[a-f\d]{64}$/;
const TARGET = /^[a-zA-Z0-9._-]+$/;
const SUITE_STATUSES = new Set(['pass', 'flagged', 'failed']);
const FIXTURE_STATUSES = new Set(['pass', 'warn', 'improvement-warn', 'flagged', 'failed']);
const FIXTURE_CATEGORIES = new Set(['transform', 'perf', 'rollup']);
const FIXTURE_WEIGHTS = new Set(['standard', 'heavy']);

export function parseCandidateArtifactManifest(input, context = 'candidate manifest') {
  const manifest = record(input, context);
  equal(manifest.schemaVersion, 1, `${context}.schemaVersion`);
  const target = targetName(manifest.target, `${context}.target`);
  const candidateVersion = shortString(manifest.candidateVersion, `${context}.candidateVersion`);
  const releaseRef = shortString(manifest.releaseRef, `${context}.releaseRef`);
  const files = artifactFiles(manifest.files, `${context}.files`);
  return { schemaVersion: 1, target, candidateVersion, releaseRef, files };
}

export function parseReleaseBenchmarkIdentity(input, context = 'identity') {
  const identity = record(input, context);
  equal(identity.schemaVersion, 1, `${context}.schemaVersion`);
  equal(identity.subjectSchemaVersion, 1, `${context}.subjectSchemaVersion`);
  const result = {
    schemaVersion: 1,
    target: targetName(identity.target, `${context}.target`),
    targetLabel: shortString(identity.targetLabel, `${context}.targetLabel`),
    node: shortString(identity.node, `${context}.node`),
    releaseRef: shortString(identity.releaseRef, `${context}.releaseRef`),
    candidateVersion: shortString(identity.candidateVersion, `${context}.candidateVersion`),
    previousVersion: shortString(identity.previousVersion, `${context}.previousVersion`),
    runId: shortString(identity.runId, `${context}.runId`),
    subjectSchemaVersion: 1,
    files: artifactFiles(identity.files, `${context}.files`),
  };
  findNativeArtifact(result, context);
  return result;
}

export function findNativeArtifact(identity, context = 'identity') {
  const nativeFiles = identity.files.filter(file => file.path.endsWith('.node'));
  if (nativeFiles.length !== 1) {
    throw new Error(`${context}.files must contain exactly one native artifact`);
  }
  return nativeFiles[0];
}

export function parseReleaseVerdict(input, context = 'verdict') {
  const verdict = record(input, context);
  equal(verdict.schemaVersion, 1, `${context}.schemaVersion`);
  const suiteStatus = oneOf(verdict.suiteStatus, SUITE_STATUSES, `${context}.suiteStatus`);

  const thresholds = record(verdict.thresholds, `${context}.thresholds`);
  equal(
    positiveNumber(thresholds.warn, `${context}.thresholds.warn`),
    1.1,
    `${context}.thresholds.warn`
  );
  equal(
    positiveNumber(thresholds.fail, `${context}.thresholds.fail`),
    1.2,
    `${context}.thresholds.fail`
  );
  equal(
    positiveNumber(thresholds.improvementWarn, `${context}.thresholds.improvementWarn`),
    0.5,
    `${context}.thresholds.improvementWarn`
  );

  const bootstrap = record(verdict.bootstrap, `${context}.bootstrap`);
  integer(bootstrap.seed, `${context}.bootstrap.seed`);
  positiveInteger(bootstrap.resamples, `${context}.bootstrap.resamples`);
  const confidence = positiveNumber(bootstrap.confidence, `${context}.bootstrap.confidence`);
  if (confidence >= 1) throw new Error(`${context}.bootstrap.confidence must be less than 1`);

  const subjects = record(verdict.subjects, `${context}.subjects`);
  subject(subjects.base, `${context}.subjects.base`);
  subject(subjects.candidate, `${context}.subjects.candidate`);

  const fixtures = array(verdict.fixtures, `${context}.fixtures`);
  if (fixtures.length === 0) throw new Error(`${context}.fixtures must not be empty`);
  let failedFixtures = 0;
  let unresolvedFixtures = 0;
  const fixtureNames = fixtures.map((value, index) => {
    const fixtureContext = `${context}.fixtures[${String(index)}]`;
    const fixture = record(value, fixtureContext);
    const name = shortString(fixture.name, `${fixtureContext}.name`);
    oneOf(fixture.category, FIXTURE_CATEGORIES, `${fixtureContext}.category`);
    oneOf(fixture.weight, FIXTURE_WEIGHTS, `${fixtureContext}.weight`);
    positiveInteger(fixture.batchSize, `${fixtureContext}.batchSize`);
    const base = subjectStats(fixture.base, `${fixtureContext}.base`);
    const candidate = subjectStats(fixture.candidate, `${fixtureContext}.candidate`);
    const ratios = positiveNumberArray(fixture.ratios, `${fixtureContext}.ratios`);
    if (base.length !== candidate.length || base.length !== ratios.length) {
      throw new Error(`${fixtureContext} per-round arrays must have equal lengths`);
    }
    interval(fixture.interval, `${fixtureContext}.interval`);
    if (fixture.retryInterval !== undefined) {
      interval(fixture.retryInterval, `${fixtureContext}.retryInterval`);
    }
    const status = oneOf(fixture.status, FIXTURE_STATUSES, `${fixtureContext}.status`);
    if (status === 'failed') failedFixtures += 1;
    if (status === 'flagged') unresolvedFixtures += 1;
    array(fixture.messages, `${fixtureContext}.messages`).forEach((message, messageIndex) =>
      shortString(message, `${fixtureContext}.messages[${String(messageIndex)}]`)
    );
    return name;
  });
  unique(fixtureNames, `${context}.fixtures`);

  const flagged = array(verdict.flagged, `${context}.flagged`).map((value, index) =>
    shortString(value, `${context}.flagged[${String(index)}]`)
  );
  unique(flagged, `${context}.flagged`);
  for (const name of flagged) {
    if (!fixtureNames.includes(name))
      throw new Error(`${context}.flagged contains unknown ${name}`);
  }
  const reproduced = boolean(verdict.hasReproducedFailure, `${context}.hasReproducedFailure`);
  if (
    (suiteStatus === 'failed') !== failedFixtures > 0 ||
    reproduced !== failedFixtures > 0 ||
    (suiteStatus === 'flagged') !== unresolvedFixtures > 0
  ) {
    throw new Error(`${context} suite status is inconsistent with fixture statuses`);
  }

  return { ...verdict, suiteStatus, flagged, fixtures };
}

export function verifyArtifactFiles(directory, files) {
  const errors = [];
  const expected = new Set(files.map(file => file.path));
  for (const file of files) {
    const artifactPath = path.join(directory, file.path);
    if (!fs.existsSync(artifactPath)) {
      errors.push(`Missing artifact: ${artifactPath}`);
      continue;
    }
    const actual = sha256File(artifactPath);
    if (actual !== file.sha256) {
      errors.push(
        `Checksum mismatch for ${file.path}: expected ${file.sha256}, got ${actual} (${artifactPath})`
      );
    }
  }
  const actual = fs
    .readdirSync(directory, { withFileTypes: true })
    .filter(entry => entry.isFile() && /\.(?:node|js|d\.ts)$/.test(entry.name))
    .map(entry => entry.name)
    .toSorted();
  for (const name of actual) {
    if (!expected.has(name)) errors.push(`Unmanifested artifact: ${path.join(directory, name)}`);
  }
  return errors;
}

function artifactFiles(input, context) {
  const files = array(input, context).map((value, index) => {
    const fileContext = `${context}[${String(index)}]`;
    const file = record(value, fileContext);
    const filePath = shortString(file.path, `${fileContext}.path`);
    if (filePath !== path.basename(filePath) || !/\.(?:node|js|d\.ts)$/.test(filePath)) {
      throw new Error(`${fileContext}.path must be a safe publishable dist filename`);
    }
    const checksum = shortString(file.sha256, `${fileContext}.sha256`);
    if (!SHA256.test(checksum)) throw new Error(`${fileContext}.sha256 must be lowercase SHA-256`);
    return { path: filePath, sha256: checksum };
  });
  if (files.length === 0) throw new Error(`${context} must not be empty`);
  unique(
    files.map(file => file.path),
    context
  );
  return files;
}

function subject(input, context) {
  const value = record(input, context);
  shortString(value.label, `${context}.label`);
  shortString(value.version, `${context}.version`);
  shortString(value.resolvedFrom, `${context}.resolvedFrom`);
}

function subjectStats(input, context) {
  const value = record(input, context);
  shortString(value.label, `${context}.label`);
  return positiveNumberArray(value.perRoundP50, `${context}.perRoundP50`);
}

function positiveNumberArray(input, context) {
  const values = array(input, context).map((value, index) =>
    positiveNumber(value, `${context}[${String(index)}]`)
  );
  if (values.length === 0) throw new Error(`${context} must not be empty`);
  return values;
}

function interval(input, context) {
  const value = record(input, context);
  positiveNumber(value.point, `${context}.point`);
  positiveNumber(value.lower, `${context}.lower`);
  positiveNumber(value.upper, `${context}.upper`);
}

function record(value, context) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    throw new Error(`${context} must be an object`);
  }
  return value;
}

function array(value, context) {
  if (!Array.isArray(value)) throw new Error(`${context} must be an array`);
  return value;
}

function shortString(value, context) {
  if (typeof value !== 'string' || value.length === 0 || value.length > 512) {
    throw new Error(`${context} must be a non-empty string of at most 512 characters`);
  }
  return value;
}

function targetName(value, context) {
  const name = shortString(value, context);
  if (!TARGET.test(name)) throw new Error(`${context} contains unsafe characters`);
  return name;
}

function positiveNumber(value, context) {
  if (typeof value !== 'number' || !Number.isFinite(value) || value <= 0) {
    throw new Error(`${context} must be a positive finite number`);
  }
  return value;
}

function integer(value, context) {
  if (typeof value !== 'number' || !Number.isSafeInteger(value)) {
    throw new Error(`${context} must be a safe integer`);
  }
  return value;
}

function positiveInteger(value, context) {
  const number = integer(value, context);
  if (number <= 0) throw new Error(`${context} must be positive`);
  return number;
}

function boolean(value, context) {
  if (typeof value !== 'boolean') throw new Error(`${context} must be a boolean`);
  return value;
}

function oneOf(value, allowed, context) {
  if (!allowed.has(value)) {
    throw new Error(`${context} must be one of ${[...allowed].join(', ')}`);
  }
  return value;
}

function equal(actual, expected, context) {
  if (actual !== expected) throw new Error(`${context} must equal ${String(expected)}`);
}

function unique(values, context) {
  if (new Set(values).size !== values.length) throw new Error(`${context} entries must be unique`);
}
