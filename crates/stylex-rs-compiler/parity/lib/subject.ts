/**
 * What a corpus entry asks about, in the three forms the harness needs it.
 *
 * A subject is either a CSS declaration or a whole module, and the two are
 * read in three places that must agree: the loader dedupes on identity, the
 * `--filter` flag searches text, and the report prints a label. Deriving all
 * three here keeps a subject kind added later from being handled in two places
 * and forgotten in the third.
 */

import { declarationKey } from './declaration.js';
import { SEPARATOR } from './separator.js';
import type { CorpusEntry } from './types.js';

/**
 * The identity of a subject, for deduplication.
 *
 * A declaration's half of it is `declarationKey`, not a second spelling of it:
 * the same answer also decides a harvested entry's id, and two spellings that
 * drifted apart would renumber the corpus without either looking wrong. The
 * kind prefixes both so the two identity spaces cannot collide.
 */
export function subjectKey(entry: CorpusEntry): string {
  return entry.kind === 'module'
    ? `module${SEPARATOR}${entry.source}`
    : `declaration${SEPARATOR}${declarationKey(entry.property, entry.value)}`;
}

/** The text `--filter` searches: the authored value, or the module source. */
export function subjectText(entry: CorpusEntry): string {
  return entry.kind === 'module' ? entry.source : entry.value;
}

/** How a subject is named in one line of the report. */
export function subjectLabel(entry: CorpusEntry): string {
  return entry.kind === 'module'
    ? entry.label
    : `${entry.property}: ${JSON.stringify(entry.value)}`;
}

/**
 * The module both compilers are handed for one subject.
 *
 * A module subject is passed through verbatim — it is the subject. A
 * declaration is wrapped in the smallest module that carries it, so that what
 * differs between two declaration entries is only the declaration.
 */
export function moduleFor(entry: CorpusEntry): string {
  if (entry.kind === 'module') return entry.source;

  return [
    "import * as stylex from '@stylexjs/stylex';",
    `export const styles = stylex.create({ x: { ${JSON.stringify(entry.property)}: ${JSON.stringify(entry.value)} } });`,
    '',
  ].join('\n');
}
