/**
 * The identity of a corpus entry.
 *
 * A declaration is identified by what it says, not by where it was found, so
 * that re-harvesting after a test file moves does not renumber the corpus.
 * Both the harvest and the loader depend on that answer being the same one,
 * which is why it is stated here rather than in either of them.
 */

import { SEPARATOR } from './separator.js';
import type { CorpusEntry } from './types.js';

export function entry(property: string, value: string, origin: string): CorpusEntry {
  return { id: entryId(property, value), property, value, origin };
}

/**
 * The identity of a declaration, for deduplication and hashing.
 *
 * The separator is a NUL because it is the one character a CSS property name
 * and a CSS value cannot contain, so no pair of distinct declarations can
 * collide onto one key.
 */
export function declarationKey(property: string, value: string): string {
  return `${property}${SEPARATOR}${value}`;
}

/**
 * Identify an entry by its declaration rather than its position, so that
 * re-harvesting after a test file moves does not renumber the whole corpus and
 * bury the real diff.
 */
export function entryId(property: string, value: string): string {
  let hash = 0x811c9dc5;
  const text = declarationKey(property, value);
  for (let i = 0; i < text.length; i++) {
    hash ^= text.codePointAt(i) ?? 0;
    hash = Math.imul(hash, 0x01000193) >>> 0;
  }
  return hash.toString(16).padStart(8, '0');
}

/**
 * Collapse duplicates by declaration, keeping the first origin seen. Values
 * repeat heavily across suites and a corpus entry costs two compiler runs.
 */
export function dedupe(entries: CorpusEntry[]): CorpusEntry[] {
  const byId = new Map<string, CorpusEntry>();
  for (const candidate of entries) {
    if (!byId.has(candidate.id)) byId.set(candidate.id, candidate);
  }
  return [...byId.values()].toSorted((a, b) =>
    a.property === b.property
      ? a.value.localeCompare(b.value)
      : a.property.localeCompare(b.property)
  );
}
