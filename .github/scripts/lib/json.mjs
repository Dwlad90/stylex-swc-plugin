/**
 * Strict JSON primitives and the benchmark status vocabulary, shared by every
 * CI-side artifact parser.
 *
 * Both parsers under `.github/scripts` read artifacts that cross a trust
 * boundary -- `benchmark-artifacts.mjs` reads release artifacts in the
 * aggregate job, `render-benchmark-report.mjs` reads a PR artifact from an
 * untrusted source run. Both share these helpers *and* the status sets, so
 * private copies cannot drift into accepting different vocabularies on the
 * two sides of the boundary.
 *
 * The status sets below are the CI-side mirror of `FixtureStatus` and
 * `SuiteStatus` in `crates/stylex-rs-compiler/benchmark/lib/verdict.ts`. They
 * must be kept in step with that file; `render-benchmark-report.test.mjs`
 * asserts the vocabulary so a rename fails loudly here instead of silently
 * rejecting valid artifacts in production.
 *
 * Every helper takes a `context` string and throws naming the exact field
 * path, which is what makes a malformed artifact debuggable from a CI log.
 */

/** Mirrors `SuiteStatus` in benchmark/lib/verdict.ts. */
export const SUITE_STATUSES = new Set(['pass', 'flagged', 'failed']);

/** Mirrors `FixtureStatus` in benchmark/lib/verdict.ts. */
export const FIXTURE_STATUSES = new Set(['pass', 'warn', 'improvement-warn', 'flagged', 'failed']);

export const FIXTURE_CATEGORIES = new Set(['transform', 'perf', 'rollup']);
export const FIXTURE_WEIGHTS = new Set(['standard', 'heavy']);

/** Only this suite status may publish a release or report a clean PR run. */
export const PASSING_SUITE_STATUS = 'pass';

const SHA1 = /^[a-f\d]{40}$/;
const SHA256 = /^[a-f\d]{64}$/;
const SAFE_NAME = /^[a-zA-Z0-9._-]+$/;

/**
 * Longest accepted string. Generous enough for a resolved module path,
 * short enough that a hostile artifact cannot blow up a PR comment.
 */
const MAX_STRING_LENGTH = 512;

export function fail(message) {
  throw new Error(message);
}

export function record(value, context) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    fail(`${context} must be an object`);
  }
  return value;
}

export function array(value, context) {
  if (!Array.isArray(value)) fail(`${context} must be an array`);
  return value;
}

export function shortString(value, context) {
  if (typeof value !== 'string' || value.length === 0 || value.length > MAX_STRING_LENGTH) {
    fail(
      `${context} must be a non-empty string of at most ${String(MAX_STRING_LENGTH)} characters`
    );
  }
  return value;
}

/** A full lowercase commit SHA. */
export function sha(value, context) {
  const result = shortString(value, context);
  if (!SHA1.test(result)) fail(`${context} must be a full lowercase commit SHA`);
  return result;
}

export function sha256(value, context) {
  const result = shortString(value, context);
  if (!SHA256.test(result)) fail(`${context} must be a lowercase SHA-256 digest`);
  return result;
}

/** A build target or similar identifier safe to interpolate into a path. */
export function safeName(value, context) {
  const result = shortString(value, context);
  if (!SAFE_NAME.test(result)) fail(`${context} contains unsafe characters`);
  return result;
}

export function finite(value, context) {
  if (typeof value !== 'number' || !Number.isFinite(value)) fail(`${context} must be finite`);
  return value;
}

export function positiveNumber(value, context) {
  const result = finite(value, context);
  if (result <= 0) fail(`${context} must be a positive finite number`);
  return result;
}

export function integer(value, context) {
  if (typeof value !== 'number' || !Number.isSafeInteger(value)) {
    fail(`${context} must be a safe integer`);
  }
  return value;
}

export function positiveInteger(value, context) {
  const result = integer(value, context);
  if (result <= 0) fail(`${context} must be positive`);
  return result;
}

export function boolean(value, context) {
  if (typeof value !== 'boolean') fail(`${context} must be a boolean`);
  return value;
}

export function oneOf(value, allowed, context) {
  if (!allowed.has(value)) fail(`${context} must be one of ${[...allowed].join(', ')}`);
  return value;
}

export function equal(actual, expected, context) {
  if (actual !== expected) fail(`${context} must equal ${JSON.stringify(expected)}`);
  return actual;
}

export function unique(values, context) {
  if (new Set(values).size !== values.length) fail(`${context} entries must be unique`);
  return values;
}

/** A non-empty array of positive finite numbers, e.g. per-round p50s. */
export function positiveNumberArray(value, context, { maxLength = 100 } = {}) {
  const values = array(value, context);
  if (values.length === 0 || values.length > maxLength) fail(`${context} has an invalid length`);
  return values.map((entry, index) => positiveNumber(entry, `${context}[${String(index)}]`));
}

/** A bootstrap confidence interval: three positive numbers with lower <= upper. */
export function interval(value, context) {
  const result = record(value, context);
  const point = positiveNumber(result.point, `${context}.point`);
  const lower = positiveNumber(result.lower, `${context}.lower`);
  const upper = positiveNumber(result.upper, `${context}.upper`);
  if (lower > upper) fail(`${context}.lower must not exceed upper`);
  return { point, lower, upper };
}
