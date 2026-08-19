import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

import { afterEach, describe, expect, test } from 'vitest';

import { CORPUS_FILES, loadCorpus } from '../lib/corpus.js';
import { moduleFor, subjectKey, subjectLabel, subjectText } from '../lib/subject.js';
import type { CorpusEntry } from '../lib/types.js';

/**
 * `loadCorpus` reads a directory of JSON files, so a case is a throwaway
 * directory holding one file per set. Every set the loader knows about must be
 * present or it throws, so a case that cares about one of them writes the rest
 * empty.
 */
const directories: string[] = [];

function corpusDirOf(files: Record<string, unknown>): string {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'parity-corpus-'));
  directories.push(dir);
  for (const filename of CORPUS_FILES) {
    const contents = files[filename] ?? {
      set: filename.replace('.json', ''),
      description: 'empty',
      entries: [],
    };
    fs.writeFileSync(path.join(dir, filename), JSON.stringify(contents), 'utf8');
  }
  return dir;
}

function loadOf(files: Record<string, unknown>): CorpusEntry[] {
  return loadCorpus(corpusDirOf(files));
}

const MODULE_SOURCE = "import * as stylex from '@stylexjs/stylex';\n";

function moduleFile(entries: unknown[]): Record<string, unknown> {
  return { 'modules.json': { set: 'modules', description: 'modules', entries } };
}

afterEach(() => {
  for (const dir of directories.splice(0)) fs.rmSync(dir, { recursive: true, force: true });
});

describe('loading', () => {
  test('a declaration entry is tagged as one without the file saying so', () => {
    const [entry] = loadOf({
      'reported.json': {
        set: 'reported',
        description: 'reported',
        entries: [{ id: 'a', property: 'color', value: 'red', origin: 'here' }],
      },
    });

    expect(entry).toStrictEqual({
      kind: 'declaration',
      id: 'a',
      property: 'color',
      value: 'red',
      origin: 'here',
      set: 'reported',
    });
  });

  test('an entry carrying a source is a module entry', () => {
    const [entry] = loadOf(
      moduleFile([{ id: 'm', label: 'a module', source: MODULE_SOURCE, origin: 'issue #1265' }])
    );

    expect(entry).toStrictEqual({
      kind: 'module',
      id: 'm',
      label: 'a module',
      source: MODULE_SOURCE,
      origin: 'issue #1265',
      set: 'modules',
    });
  });

  test('a note is carried through and an absent one is not invented', () => {
    const [withNote, withoutNote] = loadOf(
      moduleFile([
        { id: 'm1', label: 'l', source: 'a', origin: 'o', note: 'why' },
        { id: 'm2', label: 'l', source: 'b', origin: 'o' },
      ])
    );

    expect(withNote).toHaveProperty('note', 'why');
    expect(withoutNote).not.toHaveProperty('note');
  });

  test('the checked-in corpus loads, and carries both kinds', () => {
    const entries = loadCorpus(path.join(import.meta.dirname, '../corpus'));

    expect(entries.filter(entry => entry.kind === 'declaration').length).toBeGreaterThan(0);
    expect(entries.filter(entry => entry.kind === 'module').length).toBeGreaterThan(0);
  });

  test('the load order is the file order, so the reported cases read first', () => {
    const entries = loadCorpus(path.join(import.meta.dirname, '../corpus'));
    const sets = [...new Set(entries.map(entry => entry.set))];

    expect(sets).toStrictEqual(['reported', 'modules', 'edge', 'harvested']);
  });
});

describe('the expected verdict', () => {
  test('a recorded verdict is carried onto the entry', () => {
    const [entry] = loadOf(
      moduleFile([
        {
          id: 'm',
          label: 'l',
          source: MODULE_SOURCE,
          origin: 'o',
          expected: 'acceptance-divergent',
        },
      ])
    );

    expect(entry).toMatchObject({ kind: 'module', expected: 'acceptance-divergent' });
  });

  /**
   * The key is left off rather than set to `undefined`, so that the report's
   * JSON does not grow an `expected: null` on every one of the 800-odd entries
   * that has no expectation.
   */
  test('an entry with no expectation does not carry the key at all', () => {
    const [entry] = loadOf(
      moduleFile([{ id: 'm', label: 'l', source: MODULE_SOURCE, origin: 'o' }])
    );

    expect(entry).not.toHaveProperty('expected');
  });
});

describe('deduplication', () => {
  test('a repeated declaration keeps the first entry seen', () => {
    const entries = loadOf({
      'reported.json': {
        set: 'reported',
        description: 'reported',
        entries: [{ id: 'first', property: 'color', value: 'red', origin: 'hand-written' }],
      },
      'harvested.json': {
        set: 'harvested',
        description: 'harvested',
        entries: [{ id: 'second', property: 'color', value: 'red', origin: 'tests/case.rs:1' }],
      },
    });

    expect(entries).toHaveLength(1);
    expect(entries[0]?.id).toBe('first');
  });

  test('a repeated module source collapses too', () => {
    const entries = loadOf(
      moduleFile([
        { id: 'm1', label: 'first', source: MODULE_SOURCE, origin: 'o' },
        { id: 'm2', label: 'second', source: MODULE_SOURCE, origin: 'o' },
      ])
    );

    expect(entries).toHaveLength(1);
    expect(entries[0]?.id).toBe('m1');
  });

  /**
   * The two identity spaces are keyed apart. Without the kind in the key a
   * module whose source happened to spell a declaration's property and value
   * around the separator would silently drop one of the two.
   */
  test('a module and a declaration never collide', () => {
    const declaration: CorpusEntry = {
      kind: 'declaration',
      id: 'd',
      property: 'color',
      value: 'red',
      origin: 'o',
    };
    const asModule: CorpusEntry = {
      kind: 'module',
      id: 'm',
      label: 'l',
      source: 'color red',
      origin: 'o',
    };

    expect(subjectKey(declaration)).not.toBe(subjectKey(asModule));
  });
});

describe('malformed input', () => {
  test.each([
    ['a missing id', { label: 'l', source: 'a', origin: 'o' }],
    ['a missing label', { id: 'm', source: 'a', origin: 'o' }],
    ['a missing origin', { id: 'm', label: 'l', source: 'a' }],
    ['a non-string label', { id: 'm', label: 7, source: 'a', origin: 'o' }],
  ])('a module entry with %s names the file', (_label, entry) => {
    expect(() => loadOf(moduleFile([entry]))).toThrow(/a module entry expects/);
  });

  /**
   * A `source` that is not a string reads as a declaration entry, because the
   * kind is decided by whether one is present. The declaration branch then
   * reports what it found missing, which is the useful message either way.
   */
  test('a non-string source falls through to the declaration branch', () => {
    expect(() => loadOf(moduleFile([{ id: 'm', label: 'l', source: 7, origin: 'o' }]))).toThrow(
      /expected \{ id, property, value, origin \}/
    );
  });

  test.each([
    ['not an object', []],
    ['missing entries', { set: 's', description: 'd' }],
    ['entries not an array', { set: 's', description: 'd', entries: {} }],
  ])('a corpus file %s is rejected by name', (_label, contents) => {
    expect(() => loadOf({ 'modules.json': contents })).toThrow(/Corpus file malformed/);
  });

  test('an entry naming a verdict that does not exist is rejected by name', () => {
    expect(() =>
      loadOf(
        moduleFile([
          { id: 'm', label: 'l', source: MODULE_SOURCE, origin: 'o', expected: 'nearly-identical' },
        ])
      )
    ).toThrow(/unknown expected verdict: nearly-identical/);
  });

  test('a missing corpus file is named, and the generated one says how to make it', () => {
    const dir = corpusDirOf({});
    fs.rmSync(path.join(dir, 'harvested.json'));

    expect(() => loadCorpus(dir)).toThrow(/harvested\.json — run `pnpm parity:harvest`/);
  });
});

describe('the subject vocabulary', () => {
  const declaration: CorpusEntry = {
    kind: 'declaration',
    id: 'd',
    property: 'color',
    value: 'red',
    origin: 'o',
  };
  const wholeModule: CorpusEntry = {
    kind: 'module',
    id: 'm',
    label: 'a module',
    source: MODULE_SOURCE,
    origin: 'o',
  };

  test('a declaration is wrapped in the smallest module that carries it', () => {
    expect(moduleFor(declaration)).toBe(
      "import * as stylex from '@stylexjs/stylex';\n" +
        'export const styles = stylex.create({ x: { "color": "red" } });\n'
    );
  });

  test('a module subject is handed over verbatim', () => {
    expect(moduleFor(wholeModule)).toBe(MODULE_SOURCE);
  });

  /**
   * A CSS value is arbitrary text and the wrapper is assembled by hand rather
   * than parsed, so a value carrying a quote or a backslash must not be able to
   * close the string it sits in and turn the rest of itself into code. The line
   * count is the assertion that does it: an escape that got through would end
   * the statement early and add a line.
   */
  test.each([
    ['a quote', 'a"b'],
    ['a backslash', 'a\\b'],
    ['a newline', 'a\nb'],
    ['a line separator', 'a\u2028b'],
    ['a NUL', 'a\u0000b'],
    ['a closing brace', 'a}b'],
    ['a template hole', 'a${b}c'],
    ['a comment opener', 'a/*b'],
    ['an unpaired surrogate', 'a\uD800b'],
  ])('a value containing %s cannot escape the wrapper', (_label, value) => {
    const wrapped = moduleFor({ ...declaration, value });

    expect(wrapped).toContain(JSON.stringify(value));
    expect(wrapped.split('\n')).toHaveLength(3);
  });

  test('the filter searches the value, or the whole module source', () => {
    expect(subjectText(declaration)).toBe('red');
    expect(subjectText(wholeModule)).toBe(MODULE_SOURCE);
  });

  test('the report names a declaration by its text and a module by its label', () => {
    expect(subjectLabel(declaration)).toBe('color: "red"');
    expect(subjectLabel(wholeModule)).toBe('a module');
  });
});
