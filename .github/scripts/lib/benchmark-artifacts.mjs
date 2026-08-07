/**
 * Parsers for the versioned benchmark artifacts produced by `bench:verdict`
 * and the workflows around it.
 *
 * `parseVerdict` is the single description of the verdict artifact's shape.
 * The release aggregate job and the PR reporter apply it at different
 * strictness -- the reporter reads an artifact built by untrusted PR code and
 * so pins the fixture set, threshold values and round count, while the release
 * job reads its own artifacts and only needs the schema to hold. Both share
 * one status vocabulary and one set of primitives (`./json.mjs`), which is
 * what stops the two from drifting apart.
 */

import fs from 'node:fs';
import path from 'node:path';

import { sha256File } from './ci.mjs';
import {
  FIXTURE_CATEGORIES,
  FIXTURE_STATUSES,
  FIXTURE_WEIGHTS,
  SUITE_STATUSES,
  array,
  boolean,
  equal,
  fail,
  integer,
  interval,
  oneOf,
  positiveInteger,
  positiveNumber,
  positiveNumberArray,
  record,
  safeName,
  sha256,
  shortString,
  unique,
} from './json.mjs';

const PUBLISHABLE_ARTIFACT = /\.(?:node|js|d\.ts)$/;

export function parseCandidateArtifactManifest(input, context = 'candidate manifest') {
  const manifest = record(input, context);
  equal(manifest.schemaVersion, 1, `${context}.schemaVersion`);
  const target = safeName(manifest.target, `${context}.target`);
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
    target: safeName(identity.target, `${context}.target`),
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
    fail(`${context}.files must contain exactly one native artifact`);
  }
  return nativeFiles[0];
}

/**
 * Parse a `compare-revisions.verdict.v1.json` artifact.
 *
 * @param {unknown} input
 * @param {string} context field-path prefix used in error messages
 * @param {object} [options]
 * @param {Map<string, string>} [options.expectedFixtures]
 *   Exact fixture name -> category set the artifact must contain. Omit to
 *   accept any non-empty set of well-formed fixtures.
 * @param {Record<string, number>} [options.expectedThresholds]
 *   Exact threshold values the artifact must declare. Omit to accept any
 *   positive thresholds.
 * @param {number} [options.expectedRounds]
 *   Exact per-round sample count. Omit to only require that the base,
 *   candidate and ratio arrays agree with each other.
 * @param {number} [options.maxMessages] Cap on per-fixture message count.
 * @param {boolean} [options.expectSubjectLabels]
 *   Require the subject labels to be literally `base` and `candidate`. True
 *   for PR runs, where both labels are fixed; release runs label subjects by
 *   version, so they must not be pinned.
 */
export function parseVerdict(input, context = 'verdict', options = {}) {
  const {
    expectedFixtures,
    expectedThresholds,
    expectedRounds,
    maxMessages,
    expectSubjectLabels = false,
  } = options;
  const baseLabel = expectSubjectLabels ? 'base' : undefined;
  const candidateLabel = expectSubjectLabels ? 'candidate' : undefined;

  const verdict = record(input, context);
  equal(verdict.schemaVersion, 1, `${context}.schemaVersion`);
  const suiteStatus = oneOf(verdict.suiteStatus, SUITE_STATUSES, `${context}.suiteStatus`);

  const thresholds = record(verdict.thresholds, `${context}.thresholds`);
  for (const name of ['warn', 'fail', 'improvementWarn']) {
    const value = positiveNumber(thresholds[name], `${context}.thresholds.${name}`);
    if (expectedThresholds) equal(value, expectedThresholds[name], `${context}.thresholds.${name}`);
  }

  const bootstrap = record(verdict.bootstrap, `${context}.bootstrap`);
  integer(bootstrap.seed, `${context}.bootstrap.seed`);
  positiveInteger(bootstrap.resamples, `${context}.bootstrap.resamples`);
  const confidence = positiveNumber(bootstrap.confidence, `${context}.bootstrap.confidence`);
  if (confidence >= 1) fail(`${context}.bootstrap.confidence must be less than 1`);

  const subjects = record(verdict.subjects, `${context}.subjects`);
  subject(subjects.base, `${context}.subjects.base`, baseLabel);
  subject(subjects.candidate, `${context}.subjects.candidate`, candidateLabel);

  const fixtures = array(verdict.fixtures, `${context}.fixtures`);
  if (expectedFixtures) {
    if (fixtures.length !== expectedFixtures.size) {
      fail(`${context}.fixtures must contain exactly ${String(expectedFixtures.size)} fixtures`);
    }
  } else if (fixtures.length === 0) {
    fail(`${context}.fixtures must not be empty`);
  }

  let failedFixtures = 0;
  let unresolvedFixtures = 0;
  const fixtureNames = fixtures.map((value, index) => {
    const fixtureContext = `${context}.fixtures[${String(index)}]`;
    const fixture = record(value, fixtureContext);
    const name = shortString(fixture.name, `${fixtureContext}.name`);

    if (expectedFixtures) {
      const category = expectedFixtures.get(name);
      if (category === undefined) fail(`${fixtureContext}.name is not an allowed benchmark`);
      equal(fixture.category, category, `${fixtureContext}.category`);
    } else {
      oneOf(fixture.category, FIXTURE_CATEGORIES, `${fixtureContext}.category`);
    }

    oneOf(fixture.weight, FIXTURE_WEIGHTS, `${fixtureContext}.weight`);
    positiveInteger(fixture.batchSize, `${fixtureContext}.batchSize`);

    const base = subjectStats(fixture.base, `${fixtureContext}.base`, expectedRounds, baseLabel);
    const candidate = subjectStats(
      fixture.candidate,
      `${fixtureContext}.candidate`,
      expectedRounds,
      candidateLabel
    );
    const ratios = positiveNumberArray(fixture.ratios, `${fixtureContext}.ratios`);
    if (base.length !== candidate.length || base.length !== ratios.length) {
      fail(`${fixtureContext} per-round arrays must have equal lengths`);
    }
    if (expectedRounds !== undefined && ratios.length !== expectedRounds) {
      fail(`${fixtureContext}.ratios must contain ${String(expectedRounds)} calibrated rounds`);
    }

    interval(fixture.interval, `${fixtureContext}.interval`);
    if (fixture.retryInterval !== undefined) {
      interval(fixture.retryInterval, `${fixtureContext}.retryInterval`);
    }

    const status = oneOf(fixture.status, FIXTURE_STATUSES, `${fixtureContext}.status`);
    if (status === 'failed') failedFixtures += 1;
    if (status === 'flagged') unresolvedFixtures += 1;

    const messages = array(fixture.messages, `${fixtureContext}.messages`);
    if (maxMessages !== undefined && messages.length > maxMessages) {
      fail(`${fixtureContext}.messages contains too many entries`);
    }
    messages.forEach((message, messageIndex) =>
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
    if (!fixtureNames.includes(name)) fail(`${context}.flagged contains unknown ${name}`);
  }

  const reproduced = boolean(verdict.hasReproducedFailure, `${context}.hasReproducedFailure`);
  if (
    (suiteStatus === 'failed') !== failedFixtures > 0 ||
    reproduced !== failedFixtures > 0 ||
    (suiteStatus === 'flagged') !== unresolvedFixtures > 0
  ) {
    fail(`${context} suite status is inconsistent with fixture statuses`);
  }

  return { ...verdict, suiteStatus, flagged, fixtures };
}

export function parseReleaseVerdict(input, context = 'verdict') {
  return parseVerdict(input, context);
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
    .filter(entry => entry.isFile() && PUBLISHABLE_ARTIFACT.test(entry.name))
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
    if (filePath !== path.basename(filePath) || !PUBLISHABLE_ARTIFACT.test(filePath)) {
      fail(`${fileContext}.path must be a safe publishable dist filename`);
    }
    return { path: filePath, sha256: sha256(file.sha256, `${fileContext}.sha256`) };
  });
  if (files.length === 0) fail(`${context} must not be empty`);
  unique(
    files.map(file => file.path),
    context
  );
  return files;
}

function subject(input, context, expectedLabel) {
  const value = record(input, context);
  const label = shortString(value.label, `${context}.label`);
  if (expectedLabel !== undefined) equal(label, expectedLabel, `${context}.label`);
  shortString(value.version, `${context}.version`);
  shortString(value.resolvedFrom, `${context}.resolvedFrom`);
}

function subjectStats(input, context, expectedRounds, expectedLabel) {
  const value = record(input, context);
  const label = shortString(value.label, `${context}.label`);
  if (expectedLabel !== undefined) equal(label, expectedLabel, `${context}.label`);
  const perRound = positiveNumberArray(value.perRoundP50, `${context}.perRoundP50`);
  if (expectedRounds !== undefined && perRound.length !== expectedRounds) {
    fail(`${context}.perRoundP50 must contain ${String(expectedRounds)} calibrated rounds`);
  }
  return perRound;
}
