/**
 * Corpus loading.
 *
 * The corpus is three checked-in JSON files: values harvested from the Rust
 * test suites, the cases reported in issue #1256, and a hand-written edge set.
 * They are loaded as data rather than assembled in code so that adding a case
 * never means editing the harness.
 */

import fs from 'node:fs';
import path from 'node:path';

import { declarationKey } from './harvest.js';
import type { CorpusFile, LoadedCorpusEntry } from './types.js';

/** Load order is the report order, so the reported cases read first. */
export const CORPUS_FILES = ['reported.json', 'edge.json', 'harvested.json'] as const;

export function loadCorpus(corpusDir: string): LoadedCorpusEntry[] {
  const entries: LoadedCorpusEntry[] = [];
  const seen = new Set<string>();

  for (const filename of CORPUS_FILES) {
    const filePath = path.join(corpusDir, filename);
    if (!fs.existsSync(filePath)) {
      throw new Error(
        `Corpus file missing: ${filePath}${
          filename === 'harvested.json' ? ' — run `pnpm parity:harvest`.' : ''
        }`
      );
    }

    const file = JSON.parse(fs.readFileSync(filePath, 'utf8')) as CorpusFile;
    for (const entry of file.entries) {
      // A hand-written case that also appears in the harvest keeps its own
      // note and origin; the harvested duplicate adds nothing.
      const key = declarationKey(entry.property, entry.value);
      if (seen.has(key)) continue;
      seen.add(key);
      entries.push({ ...entry, set: file.set });
    }
  }

  return entries;
}
