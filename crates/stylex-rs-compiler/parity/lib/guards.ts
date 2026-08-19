import { VERDICTS, type Verdict } from './types.js';

/**
 * Runtime narrowing for data the type system cannot vouch for.
 *
 * Corpus files, compiler metadata and package manifests all arrive as
 * `unknown` — parsed JSON, or a third-party module's exports. Asserting them
 * into shape would move the failure to wherever the shape is first read, which
 * for the corpus is deep inside a loop; narrowing them here fails at the read
 * instead, and says which file was wrong.
 */

export function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

/** The string at `key`, or `undefined` when absent or another type. */
export function stringAt(value: unknown, key: string): string | undefined {
  if (!isRecord(value)) return undefined;
  const found = value[key];
  return typeof found === 'string' ? found : undefined;
}

/** The array at `key`, or `undefined` when absent or another type. */
export function arrayAt(value: unknown, key: string): unknown[] | undefined {
  if (!isRecord(value)) return undefined;
  const found = value[key];
  return Array.isArray(found) ? found : undefined;
}

/**
 * The verdict at `key`, or `undefined` when absent.
 *
 * Throws on a string that is not a verdict rather than dropping it: an
 * expectation the loader silently ignored would read in the corpus as a
 * divergence someone had already looked at, which is the opposite of what the
 * field is for.
 */
export function verdictAt(value: unknown, key: string, where: string): Verdict | undefined {
  const found = stringAt(value, key);
  if (found === undefined) return undefined;
  if (!(found in VERDICTS)) {
    throw new Error(
      `Corpus entry in ${where} names an unknown ${key} verdict: ${found} — expected one of ${Object.keys(VERDICTS).join(', ')}.`
    );
  }

  return found as Verdict;
}
