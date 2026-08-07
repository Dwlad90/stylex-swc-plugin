/**
 * Renders the authoritative paired-benchmark PR comment.
 *
 * This runs in `benchmark-report.yml`, from a trusted default-branch checkout,
 * over an artifact produced by untrusted PR code. Everything it reads is
 * hostile input, so the verdict is re-validated against the fixture manifest
 * (`parseVerdict` with the strict options below) and every value that reaches
 * Markdown goes through `escapeMarkdown`.
 *
 * `--unavailable` renders the fallback comment used when the artifact is
 * missing or fails validation, so the marker and the comment layout have a
 * single definition rather than a copy inside the workflow YAML.
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { parseVerdict } from './lib/benchmark-artifacts.mjs';
import {
  FIXTURE_CATEGORIES,
  array,
  equal,
  fail,
  finite,
  oneOf,
  positiveInteger,
  record,
  sha,
  shortString,
} from './lib/json.mjs';

export const REPORT_MARKER = '<!-- stylex-paired-benchmark -->';

/** Must match the sampling policy in guidelines/PERFORMANCE.md. */
const CALIBRATED_ROUNDS = 10;
const EXPECTED_THRESHOLDS = { warn: 1.1, fail: 1.2, improvementWarn: 0.5 };
const MAX_FIXTURE_MESSAGES = 4;
const EXPECTED_TARGET = 'aarch64-unknown-linux-gnu';

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
    oneOf(fixture.category, FIXTURE_CATEGORIES, `${context}.category`);
    if (result.has(name)) fail(`${context}.name is duplicated`);
    result.set(name, fixture.category);
  }
  return result;
}

/**
 * Bind the artifact to the source run.
 *
 * Only fields the reporter can independently derive from the `workflow_run`
 * event are asserted: the run id, the PR number, and the head SHA the run
 * checked out (`workflow_run.head_sha`, which is also the PR head once
 * staleness has been ruled out).
 *
 * `candidateSha` and `baseSha` are validated for shape but deliberately not
 * compared. `candidateSha` is the test-merge commit, which GitHub recomputes
 * asynchronously, and `baseSha` is `merge-base(origin/develop, candidate)` --
 * neither is derivable here without fetching the repository, and asserting
 * them against `pull_requests[0].base.sha` (the base *branch tip*, a different
 * quantity entirely) rejected valid reports whenever the base branch moved.
 * They are provenance recorded in the comment, not part of the trust boundary;
 * `headSha` is what actually binds the artifact to this PR revision.
 */
export function validateIdentity(input, expected) {
  const identity = record(input, 'identity');
  equal(identity.schemaVersion, 1, 'identity.schemaVersion');
  equal(shortString(identity.runId, 'identity.runId'), expected.runId, 'identity.runId');
  equal(
    positiveInteger(identity.prNumber, 'identity.prNumber'),
    expected.prNumber,
    'identity.prNumber'
  );
  equal(sha(identity.headSha, 'identity.headSha'), expected.headSha, 'identity.headSha');
  sha(identity.candidateSha, 'identity.candidateSha');
  sha(identity.baseSha, 'identity.baseSha');
  equal(identity.target, EXPECTED_TARGET, 'identity.target');
  shortString(identity.nodeAbi, 'identity.nodeAbi');
  equal(identity.subjectSchemaVersion, 1, 'identity.subjectSchemaVersion');
  return identity;
}

/**
 * Validate a verdict written by untrusted PR code.
 *
 * The strict options are what separate this from the release-side call: the
 * fixture set must be exactly the trusted manifest, the thresholds must be the
 * calibrated ones, and every fixture must carry the calibrated round count.
 * The status vocabulary itself is shared, so a status the engine emits can
 * never be rejected here while being accepted on the release side.
 */
export function validateVerdict(input) {
  return parseVerdict(input, 'verdict', {
    expectedFixtures: BENCHMARK_FIXTURES,
    expectedThresholds: EXPECTED_THRESHOLDS,
    expectedRounds: CALIBRATED_ROUNDS,
    maxMessages: MAX_FIXTURE_MESSAGES,
    expectSubjectLabels: true,
  });
}

export function renderReport(input, { runUrl, conclusion, identity }) {
  const report = validateVerdict(input);
  const rows = report.fixtures.map(fixture => {
    const notes = fixture.messages.map(escapeMarkdown).join('; ');
    return markdownTableRow([
      escapeMarkdown(fixture.name),
      escapeMarkdown(fixture.category),
      fixed(fixture.interval.point),
      fixed(fixture.interval.lower),
      fixed(fixture.interval.upper),
      escapeMarkdown(fixture.status),
      notes,
    ]);
  });

  return comment([
    `Suite status: **${escapeMarkdown(report.suiteStatus)}**`,
    `Workflow conclusion: **${escapeMarkdown(conclusion)}**`,
    ...(identity
      ? [
          `Candidate: \`${escapeMarkdown(identity.candidateSha)}\` vs base ` +
            `\`${escapeMarkdown(identity.baseSha)}\``,
        ]
      : []),
    `[Source workflow run](${escapeMarkdown(runUrl)})`,
    '',
    '| Fixture | Category | Point | Lower | Upper | Status | Notes |',
    '| --- | --- | --- | --- | --- | --- | --- |',
    ...rows,
  ]);
}

/** The fallback comment for a missing, oversized, or invalid artifact. */
export function renderUnavailableReport({ runUrl, conclusion }) {
  return comment([
    'Suite status: **unavailable**',
    `Workflow conclusion: **${escapeMarkdown(conclusion)}**`,
    `[Source workflow run](${escapeMarkdown(runUrl)})`,
    '',
    'The source run did not produce a valid paired benchmark report. ' +
      'Inspect the workflow diagnostics.',
  ]);
}

function comment(body) {
  return [REPORT_MARKER, '## Paired revision benchmark', '', ...body, '', REPORT_MARKER, ''].join(
    '\n'
  );
}

/**
 * Markdown escaping for values that came out of an untrusted artifact.
 *
 * Deliberately stricter than `benchmark/lib/format.ts`'s `escapeMarkdownCell`:
 * that one writes to the job summary of the run that produced the data, while
 * this one writes a comment on a pull request under the bot's identity, so it
 * also neutralises raw HTML and every Markdown construct that could forge
 * links or headings.
 */
export function escapeMarkdown(value) {
  return String(value)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replace(/([\\`*_[\]{}()#+.!|>~-])/g, '\\$1')
    .replace(/[\p{Cc}\p{Cf}]/gu, ' ');
}

function markdownTableRow(cells) {
  return `| ${cells.join(' | ')} |`;
}

function fixed(value) {
  return finite(value, 'rendered numeric value').toFixed(3);
}

function parseCli(argv) {
  const values = new Map();
  for (let index = 0; index < argv.length; index += 2) {
    const name = argv[index];
    const value = argv[index + 1];
    if (!name?.startsWith('--') || value === undefined) fail('Expected --name value arguments');
    values.set(name, value);
  }
  const required = values.has('--unavailable')
    ? ['--output', '--run-url', '--conclusion']
    : [
        '--input',
        '--identity',
        '--output',
        '--run-url',
        '--conclusion',
        '--run-id',
        '--pr-number',
        '--head-sha',
      ];
  for (const name of required) {
    if (!values.has(name)) fail(`${name} is required`);
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
    const runUrl = options.get('--run-url');
    const conclusion = options.get('--conclusion');
    const markdown = options.has('--unavailable')
      ? renderUnavailableReport({ runUrl, conclusion })
      : renderValidatedReport(options, { runUrl, conclusion });
    fs.writeFileSync(options.get('--output'), markdown, 'utf8');
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exitCode = 1;
  }
}

function renderValidatedReport(options, { runUrl, conclusion }) {
  const identity = validateIdentity(
    JSON.parse(fs.readFileSync(options.get('--identity'), 'utf8')),
    {
      runId: options.get('--run-id'),
      prNumber: Number(options.get('--pr-number')),
      headSha: options.get('--head-sha'),
    }
  );
  const input = JSON.parse(fs.readFileSync(options.get('--input'), 'utf8'));
  return renderReport(input, { runUrl, conclusion, identity });
}
