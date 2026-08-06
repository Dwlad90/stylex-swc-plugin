/**
 * Shared strict JSON validators for the versioned benchmark artifacts.
 *
 * `raw-stats.v1.json`, `budget.json`, and the reports derived from them
 * all cross a trust boundary — CI artifacts, npm packages, and reviewed
 * files edited by hand. Every parser in `lib/` funnels through these
 * helpers so one rule ("non-empty string", "positive finite number")
 * cannot drift between schemas, and so no parser reaches for a cast
 * where a type predicate belongs.
 *
 * Each helper takes a `context` string and throws an error naming the
 * exact field path, which is what makes a malformed artifact debuggable
 * from a CI log alone.
 */

export type JsonRecord = Record<string, unknown>;

export function isRecord(value: unknown): value is JsonRecord {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}

export function requireRecord(value: unknown, context: string): JsonRecord {
  if (!isRecord(value)) throw new Error(`${context} must be an object`);
  return value;
}

export function requireArray(value: unknown, context: string): unknown[] {
  if (!Array.isArray(value)) throw new Error(`${context} must be an array`);
  return value;
}

export function requireString(value: unknown, context: string): string {
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`${context} must be a non-empty string`);
  }
  return value;
}

export function optionalString(value: unknown, context: string): string | undefined {
  if (value === undefined) return undefined;
  return requireString(value, context);
}

export function requirePositiveNumber(value: unknown, context: string): number {
  if (typeof value !== 'number' || !Number.isFinite(value) || value <= 0) {
    throw new Error(`${context} must be a positive finite number`);
  }
  return value;
}

export function requireNonNegativeNumber(value: unknown, context: string): number {
  if (typeof value !== 'number' || !Number.isFinite(value) || value < 0) {
    throw new Error(`${context} must be a non-negative finite number`);
  }
  return value;
}

export function requireInteger(value: unknown, context: string): number {
  if (typeof value !== 'number' || !Number.isSafeInteger(value)) {
    throw new Error(`${context} must be a safe integer`);
  }
  return value;
}

export function requirePositiveInteger(value: unknown, context: string): number {
  const integer = requireInteger(value, context);
  if (integer <= 0) throw new Error(`${context} must be greater than zero`);
  return integer;
}

export function requireNonNegativeInteger(value: unknown, context: string): number {
  const integer = requireInteger(value, context);
  if (integer < 0) throw new Error(`${context} must not be negative`);
  return integer;
}

/** Calendar date without a time component, e.g. `2026-08-06`. */
export function requireIsoDate(value: unknown, context: string): string {
  const text = requireString(value, context);
  if (!/^\d{4}-\d{2}-\d{2}$/.test(text) || Number.isNaN(Date.parse(text))) {
    throw new Error(`${context} must be an ISO date (YYYY-MM-DD)`);
  }
  return text;
}

export function assertUnique(values: readonly string[], context: string): void {
  if (new Set(values).size !== values.length) throw new Error(`${context} must be unique`);
}
