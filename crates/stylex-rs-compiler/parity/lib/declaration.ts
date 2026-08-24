/**
 * The identity of a corpus entry.
 *
 * A declaration is identified by what it says, not by where it was found, so
 * that re-harvesting after a test file moves does not renumber the corpus.
 * Both the harvest, which spends it on an id, and `subjectKey`, which spends
 * it on deduplication, depend on that answer being the same one — which is why
 * it is stated here rather than in either of them.
 */

import { SEPARATOR } from './separator.js';
import type { DeclarationEntry } from './types.js';

export function entry(property: string, value: string, origin: string): DeclarationEntry {
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
    // `charCodeAt`, which is what the loop bounds describe: `text.length` counts
    // UTF-16 code units, and `codePointAt` reads a whole astral character at a
    // surrogate index. So an emoji mixed in its full code point and then, one
    // index later, its own trailing surrogate again -- a hash over a sequence
    // the string does not contain.
    //
    // The lint prefers `codePointAt` in general and is right to; it is wrong
    // here, and the loop is why. Disabled rather than worked around, because
    // `--fix` reverts this line otherwise -- which it did once, leaving the code
    // disagreeing with the comment above it.
    // oxlint-disable-next-line unicorn/prefer-code-point
    hash ^= text.charCodeAt(i);
    hash = Math.imul(hash, 0x01000193) >>> 0;
  }
  return hash.toString(16).padStart(8, '0');
}

/**
 * Order two strings by code point.
 *
 * Deliberately not `localeCompare`: called with no locale it reads whatever
 * collation the running ICU offers, so a `small-icu` Node or a different `LANG`
 * orders the corpus differently. That would make `parity:harvest --check` fail
 * against an unchanged tree and rewrite all of `harvested.json`, which in turn
 * rewrites `postcss-value-parser`'s generated `cases.rs`, whose row order is
 * the corpus order. Code-point order is the same everywhere.
 */
function byCodePoint(a: string, b: string): number {
  return a < b ? -1 : a > b ? 1 : 0;
}

/**
 * Collapse duplicates by declaration, keeping the first origin seen. Values
 * repeat heavily across suites and a corpus entry costs two compiler runs.
 */
export function dedupe(entries: DeclarationEntry[]): DeclarationEntry[] {
  // Keyed on the declaration itself, not on the id derived from it. The id is a
  // 32-bit hash, so keying on it let two *different* declarations collide and
  // one of them vanish from the corpus -- with nothing to show it but a count
  // nobody checks. At 823 entries the birthday probability is around 1e-4, and
  // it grows with the square of the harvest.
  //
  // The exact identity was already to hand, and it is the same key the id is
  // computed from.
  const byDeclaration = new Map<string, DeclarationEntry>();
  for (const candidate of entries) {
    const key = declarationKey(candidate.property, candidate.value);
    if (!byDeclaration.has(key)) byDeclaration.set(key, candidate);
  }
  return [...byDeclaration.values()].toSorted((a, b) =>
    a.property === b.property ? byCodePoint(a.value, b.value) : byCodePoint(a.property, b.property)
  );
}
