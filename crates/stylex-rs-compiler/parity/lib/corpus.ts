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

import { declarationKey } from './declaration.js';
import { arrayAt, stringAt } from './guards.js';
import type { CorpusEntry, CorpusFile, LoadedCorpusEntry } from './types.js';

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

    const file = corpusFileFrom(JSON.parse(fs.readFileSync(filePath, 'utf8')), filePath);
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

/** Narrow parsed JSON to a corpus file, naming the file when it is not one. */
function corpusFileFrom(raw: unknown, filePath: string): CorpusFile {
  const set = stringAt(raw, 'set');
  const description = stringAt(raw, 'description');
  const entries = arrayAt(raw, 'entries');
  if (set === undefined || description === undefined || entries === undefined) {
    throw new Error(`Corpus file malformed: ${filePath} — expected { set, description, entries }.`);
  }

  return {
    set,
    description,
    entries: entries.map((entry, index) => corpusEntryFrom(entry, filePath, index)),
  };
}

function corpusEntryFrom(raw: unknown, filePath: string, index: number): CorpusEntry {
  const id = stringAt(raw, 'id');
  const property = stringAt(raw, 'property');
  const value = stringAt(raw, 'value');
  const origin = stringAt(raw, 'origin');
  if (id === undefined || property === undefined || value === undefined || origin === undefined) {
    throw new Error(
      `Corpus entry ${index} malformed in ${filePath} — expected { id, property, value, origin }.`
    );
  }

  const note = stringAt(raw, 'note');
  return { id, property, value, origin, ...(note === undefined ? {} : { note }) };
}
