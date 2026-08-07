import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

export const REPORT_MARKER = '<!-- stylex-paired-benchmark -->';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const fixtureManifestPath = path.resolve(
  scriptDir,
  '../../crates/stylex-rs-compiler/benchmark/fixtures.v1.json'
);

/**
 * Floor, not an exact count. This manifest comes from the trusted
 * default-branch checkout, so pinning its exact length guards nothing at the
 * trust boundary -- it only forces a second edit on every fixture change, and
 * a missed one fails CI. The boundary check is
 * `verdict.fixtures.length === BENCHMARK_FIXTURES.size`, which derives from
 * the manifest. This floor only catches a truncated or gutted manifest.
 *
 * Declared before the call below: `BENCHMARK_FIXTURES` initializes at module
 * load, so a later `const` would be in the temporal dead zone.
 */
const MIN_BENCHMARK_FIXTURES = 10;

export const BENCHMARK_FIXTURES = loadBenchmarkFixtures(fixtureManifestPath);

const FIXTURE_STATUSES = new Set(['pass', 'warn', 'improvement-warn', 'failed']);
const SUITE_STATUSES = new Set(['pass', 'failed']);

export function loadBenchmarkFixtures(manifestPath) {
  const input = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
  const manifest = record(input, 'fixture manifest');
  equal(manifest.schemaVersion, 1, 'fixture manifest.schemaVersion');
  const fixtures = array(manifest.fixtures, 'fixture manifest.fixtures');
  if (fixtures.length < MIN_BENCHMARK_FIXTURES) {
    fail(
      `fixture manifest must contain at least ${String(MIN_BENCHMARK_FIXTURES)} benchmarks, ` +
        `found ${String(fixtures.length)}`
    );
  }

  const result = new Map();
  for (const [index, value] of fixtures.entries()) {
    const context = `fixture manifest.fixtures[${String(index)}]`;
    const fixture = record(value, context);
    const name = shortString(fixture.name, `${context}.name`);
    oneOf(fixture.category, new Set(['transform', 'perf', 'rollup']), `${context}.category`);
    if (result.has(name)) fail(`${context}.name is duplicated`);
    result.set(name, fixture.category);
  }
  return result;
}

export function validateIdentity(input, expected) {
  const identity = record(input, 'identity');
  equal(identity.schemaVersion, 1, 'identity.schemaVersion');
  equal(shortString(identity.runId, 'identity.runId'), expected.runId, 'identity.runId');
  equal(
    positiveInteger(identity.prNumber, 'identity.prNumber'),
    expected.prNumber,
    'identity.prNumber'
  );
  equal(
    sha(identity.candidateSha, 'identity.candidateSha'),
    expected.candidateSha,
    'identity.candidateSha'
  );
  equal(sha(identity.baseSha, 'identity.baseSha'), expected.baseSha, 'identity.baseSha');
  equal(identity.target, 'aarch64-unknown-linux-gnu', 'identity.target');
  shortString(identity.nodeAbi, 'identity.nodeAbi');
  equal(identity.subjectSchemaVersion, 1, 'identity.subjectSchemaVersion');
  return identity;
}

export function validateVerdict(input) {
  const report = record(input, 'verdict');
  equal(report.schemaVersion, 1, 'verdict.schemaVersion');
  oneOf(report.suiteStatus, SUITE_STATUSES, 'verdict.suiteStatus');

  const thresholds = record(report.thresholds, 'verdict.thresholds');
  equal(finite(thresholds.warn, 'verdict.thresholds.warn'), 1.1, 'verdict.thresholds.warn');
  equal(finite(thresholds.fail, 'verdict.thresholds.fail'), 1.2, 'verdict.thresholds.fail');
  equal(
    finite(thresholds.improvementWarn, 'verdict.thresholds.improvementWarn'),
    0.5,
    'verdict.thresholds.improvementWarn'
  );

  const bootstrap = record(report.bootstrap, 'verdict.bootstrap');
  integer(bootstrap.seed, 'verdict.bootstrap.seed');
  positiveInteger(bootstrap.resamples, 'verdict.bootstrap.resamples');
  const confidence = finite(bootstrap.confidence, 'verdict.bootstrap.confidence');
  if (confidence <= 0 || confidence >= 1) fail('verdict.bootstrap.confidence must be in (0, 1)');

  const subjects = record(report.subjects, 'verdict.subjects');
  validateSubject(subjects.base, 'base', 'verdict.subjects.base');
  validateSubject(subjects.candidate, 'candidate', 'verdict.subjects.candidate');

  const fixtures = array(report.fixtures, 'verdict.fixtures');
  if (fixtures.length !== BENCHMARK_FIXTURES.size) {
    fail(`verdict.fixtures must contain exactly ${String(BENCHMARK_FIXTURES.size)} fixtures`);
  }

  const seen = new Set();
  let failedCount = 0;
  for (const [index, value] of fixtures.entries()) {
    const context = `verdict.fixtures[${String(index)}]`;
    const fixture = record(value, context);
    const name = shortString(fixture.name, `${context}.name`);
    const expectedCategory = BENCHMARK_FIXTURES.get(name);
    if (expectedCategory === undefined) fail(`${context}.name is not an allowed benchmark`);
    if (seen.has(name)) fail(`${context}.name is duplicated`);
    seen.add(name);
    equal(fixture.category, expectedCategory, `${context}.category`);
    oneOf(fixture.weight, new Set(['standard', 'heavy']), `${context}.weight`);
    positiveInteger(fixture.batchSize, `${context}.batchSize`);

    const status = oneOf(fixture.status, FIXTURE_STATUSES, `${context}.status`);
    if (status === 'failed') failedCount += 1;
    validateSubjectStats(fixture.base, 'base', `${context}.base`);
    validateSubjectStats(fixture.candidate, 'candidate', `${context}.candidate`);
    const ratios = positiveNumberArray(fixture.ratios, `${context}.ratios`);
    if (ratios.length !== 10) fail(`${context}.ratios must contain 10 calibrated rounds`);
    interval(fixture.interval, `${context}.interval`);
    if (fixture.retryInterval !== undefined)
      interval(fixture.retryInterval, `${context}.retryInterval`);
    const messages = array(fixture.messages, `${context}.messages`);
    if (messages.length > 4) fail(`${context}.messages contains too many entries`);
    messages.forEach((message, messageIndex) =>
      shortString(message, `${context}.messages[${String(messageIndex)}]`)
    );
  }

  const flagged = array(report.flagged, 'verdict.flagged');
  if (flagged.length > BENCHMARK_FIXTURES.size) fail('verdict.flagged contains too many entries');
  const flaggedSet = new Set();
  for (const [index, value] of flagged.entries()) {
    const name = shortString(value, `verdict.flagged[${String(index)}]`);
    if (!BENCHMARK_FIXTURES.has(name)) fail(`verdict.flagged[${String(index)}] is not allowed`);
    if (flaggedSet.has(name)) fail(`verdict.flagged[${String(index)}] is duplicated`);
    flaggedSet.add(name);
  }

  const reproduced = boolean(report.hasReproducedFailure, 'verdict.hasReproducedFailure');
  const failed = failedCount > 0;
  if ((report.suiteStatus === 'failed') !== failed || reproduced !== failed) {
    fail('verdict suite status is inconsistent with fixture failures');
  }

  return report;
}

export function renderReport(input, { runUrl, conclusion }) {
  const report = validateVerdict(input);
  const rows = report.fixtures.map(fixture => {
    const notes = fixture.messages.map(escapeMarkdown).join('; ');
    return `| ${escapeMarkdown(fixture.name)} | ${escapeMarkdown(fixture.category)} | ${fixed(fixture.interval.point)} | ${fixed(fixture.interval.lower)} | ${fixed(fixture.interval.upper)} | ${escapeMarkdown(fixture.status)} | ${notes} |`;
  });

  return [
    REPORT_MARKER,
    '## Paired revision benchmark',
    '',
    `Suite status: **${escapeMarkdown(report.suiteStatus)}**`,
    `Workflow conclusion: **${escapeMarkdown(conclusion)}**`,
    `[Source workflow run](${escapeMarkdown(runUrl)})`,
    '',
    '| Fixture | Category | Point | Lower | Upper | Status | Notes |',
    '| --- | --- | --- | --- | --- | --- | --- |',
    ...rows,
    '',
    REPORT_MARKER,
    '',
  ].join('\n');
}

export function escapeMarkdown(value) {
  return String(value)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replace(/([\\`*_[\]{}()#+.!|>~-])/g, '\\$1')
    .replace(/[\p{Cc}\p{Cf}]/gu, ' ');
}

function validateSubject(value, expectedLabel, context) {
  const subject = record(value, context);
  equal(subject.label, expectedLabel, `${context}.label`);
  shortString(subject.version, `${context}.version`);
  shortString(subject.resolvedFrom, `${context}.resolvedFrom`);
}

function validateSubjectStats(value, expectedLabel, context) {
  const stats = record(value, context);
  equal(stats.label, expectedLabel, `${context}.label`);
  const values = positiveNumberArray(stats.perRoundP50, `${context}.perRoundP50`);
  if (values.length !== 10) fail(`${context}.perRoundP50 must contain 10 calibrated rounds`);
}

function interval(value, context) {
  const result = record(value, context);
  const point = positive(result.point, `${context}.point`);
  const lower = positive(result.lower, `${context}.lower`);
  const upper = positive(result.upper, `${context}.upper`);
  if (lower > upper) fail(`${context}.lower must not exceed upper`);
  return { point, lower, upper };
}

function positiveNumberArray(value, context) {
  const values = array(value, context);
  if (values.length === 0 || values.length > 100) fail(`${context} has an invalid length`);
  return values.map((entry, index) => positive(entry, `${context}[${String(index)}]`));
}

function record(value, context) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    fail(`${context} must be an object`);
  }
  return value;
}

function array(value, context) {
  if (!Array.isArray(value)) fail(`${context} must be an array`);
  return value;
}

function shortString(value, context) {
  if (typeof value !== 'string' || value.length === 0 || value.length > 500) {
    fail(`${context} must be a non-empty string no longer than 500 characters`);
  }
  return value;
}

function sha(value, context) {
  const result = shortString(value, context);
  if (!/^[a-f\d]{40}$/.test(result)) fail(`${context} must be a full lowercase commit SHA`);
  return result;
}

function finite(value, context) {
  if (typeof value !== 'number' || !Number.isFinite(value)) fail(`${context} must be finite`);
  return value;
}

function positive(value, context) {
  const result = finite(value, context);
  if (result <= 0) fail(`${context} must be positive`);
  return result;
}

function integer(value, context) {
  if (!Number.isSafeInteger(value)) fail(`${context} must be a safe integer`);
  return value;
}

function positiveInteger(value, context) {
  const result = integer(value, context);
  if (result <= 0) fail(`${context} must be positive`);
  return result;
}

function boolean(value, context) {
  if (typeof value !== 'boolean') fail(`${context} must be a boolean`);
  return value;
}

function oneOf(value, allowed, context) {
  if (!allowed.has(value)) fail(`${context} is not supported`);
  return value;
}

function equal(actual, expected, context) {
  if (actual !== expected) fail(`${context} must equal ${JSON.stringify(expected)}`);
}

function fixed(value) {
  return finite(value, 'rendered numeric value').toFixed(3);
}

function fail(message) {
  throw new Error(message);
}

function parseCli(argv) {
  const values = new Map();
  for (let index = 0; index < argv.length; index += 2) {
    const name = argv[index];
    const value = argv[index + 1];
    if (!name?.startsWith('--') || value === undefined) fail('Expected --name value arguments');
    values.set(name, value);
  }
  for (const required of [
    '--input',
    '--identity',
    '--output',
    '--run-url',
    '--conclusion',
    '--run-id',
    '--pr-number',
    '--candidate-sha',
    '--base-sha',
  ]) {
    if (!values.has(required)) fail(`${required} is required`);
  }
  return values;
}

function isMainModule() {
  return (
    process.argv[1] !== undefined &&
    path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
  );
}

if (isMainModule()) {
  try {
    const options = parseCli(process.argv.slice(2));
    const inputPath = options.get('--input');
    const identityPath = options.get('--identity');
    const outputPath = options.get('--output');
    const runUrl = options.get('--run-url');
    const conclusion = options.get('--conclusion');
    const expectedIdentity = {
      runId: options.get('--run-id'),
      prNumber: Number(options.get('--pr-number')),
      candidateSha: options.get('--candidate-sha'),
      baseSha: options.get('--base-sha'),
    };
    const identity = JSON.parse(fs.readFileSync(identityPath, 'utf8'));
    validateIdentity(identity, expectedIdentity);
    const input = JSON.parse(fs.readFileSync(inputPath, 'utf8'));
    const markdown = renderReport(input, { runUrl, conclusion });
    fs.writeFileSync(outputPath, markdown, 'utf8');
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exitCode = 1;
  }
}
