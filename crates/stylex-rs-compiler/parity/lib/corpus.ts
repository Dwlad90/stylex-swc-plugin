/**
 * Corpus loading.
 *
 * The corpus is four checked-in JSON files: values harvested from the Rust
 * test suites, the cases reported in issue #1256, a hand-written edge set, and
 * the whole-module cases a declaration cannot express. They are loaded as data
 * rather than assembled in code so that adding a case never means editing the
 * harness.
 */

import fs from 'node:fs';
import path from 'node:path';

import { arrayAt, configurationOptionAt, stringAt, verdictAt } from './guards.js';
import { subjectKey } from './subject.js';
import type {
  ConfigurationOption,
  CorpusEntry,
  LoadedCorpusEntry,
  LoadedCorpusFile,
  Verdict,
} from './types.js';

/** Load order is the report order, so the reported cases read first. */
export const CORPUS_FILES = [
  'reported.json',
  'modules.json',
  'edge.json',
  'harvested.json',
] as const;

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
      const key = subjectKey(entry);
      if (seen.has(key)) continue;
      seen.add(key);
      entries.push({ ...entry, set: file.set });
    }
  }

  return entries;
}

/** Narrow parsed JSON to a corpus file, naming the file when it is not one. */
function corpusFileFrom(raw: unknown, filePath: string): LoadedCorpusFile {
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

/**
 * Narrow one parsed entry to a corpus entry.
 *
 * The kind is read off the fields present rather than from a stored tag: a
 * `source` says the entry carries a module, and everything else is the
 * declaration the corpus is otherwise made of. That keeps the generated
 * `harvested.json` free of a field whose value never varies — see `CorpusEntry`
 * in `types.ts`.
 */
function corpusEntryFrom(raw: unknown, filePath: string, index: number): CorpusEntry {
  const id = stringAt(raw, 'id');
  const origin = stringAt(raw, 'origin');
  // An absent note is left out rather than set to `undefined`, so that an
  // entry without one does not carry the key into the report's JSON.
  const note = stringAt(raw, 'note');
  const noteField = note === undefined ? {} : { note };
  const expected = verdictAt(raw, 'expected', filePath);
  const expectedField = expected === undefined ? {} : { expected };
  const configuration = configurationAt(raw, filePath, id, expected, note);
  const configurationField = configuration === undefined ? {} : { configuration };

  const source = stringAt(raw, 'source');
  if (source !== undefined) {
    const label = stringAt(raw, 'label');
    if (id === undefined || origin === undefined || label === undefined) {
      throw new Error(
        `Corpus entry ${index} malformed in ${filePath} — a module entry expects { id, label, source, origin }.`
      );
    }
    return {
      kind: 'module',
      id,
      label,
      source,
      origin,
      ...noteField,
      ...expectedField,
      ...configurationField,
    };
  }

  const property = stringAt(raw, 'property');
  const value = stringAt(raw, 'value');
  if (id === undefined || property === undefined || value === undefined || origin === undefined) {
    throw new Error(
      `Corpus entry ${index} malformed in ${filePath} — expected { id, property, value, origin }.`
    );
  }

  return {
    kind: 'declaration',
    id,
    property,
    value,
    origin,
    ...noteField,
    ...expectedField,
    ...configurationField,
  };
}

/**
 * The option a row names as the reason it refuses, refused unless the row also
 * carries the verdict that option currently produces and the note saying what
 * raising it buys.
 *
 * Both are what make the field a claim rather than a label. Without the recorded
 * verdict the row says "raise this option" about behaviour nothing checks, and
 * the day the ceiling stops refusing — because the guard moved, or the default
 * rose past the input — the row would go on reading as accounted for rather than
 * as changed. Without the note the row names a knob and not a reason, and a
 * refusal of a build the reference compiler completes owes a reader the reason;
 * requiring it here rather than leaving it to the run's own gate says so at the
 * point the row is written.
 */
function configurationAt(
  raw: unknown,
  filePath: string,
  id: string | undefined,
  expected: Verdict | undefined,
  note: string | undefined
): ConfigurationOption | undefined {
  const configuration = configurationOptionAt(raw, 'configuration', filePath);
  if (configuration === undefined) return undefined;

  const where = `Corpus entry ${id ?? '(unnamed)'} in ${filePath} names the \`${configuration}\` configuration but`;
  if (expected === undefined) {
    throw new Error(
      `${where} records no expected verdict — a ceiling is a configuration claim only while the refusal it explains is measured.`
    );
  }
  if (note === undefined || note.trim() === '') {
    throw new Error(
      `${where} carries no note — say what raising the option buys, since the row records a build the reference compiler completes.`
    );
  }

  return configuration;
}
