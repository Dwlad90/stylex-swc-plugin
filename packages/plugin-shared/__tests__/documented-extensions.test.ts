import { readFileSync } from 'node:fs';
import path from 'node:path';

import { describe, expect, it } from 'vitest';

import { INCLUDE_EXTENSIONS } from '../src/constants';

const REPO_ROOT = path.join(__dirname, '..', '..', '..');

/**
 * Marks a place in the documentation that writes the extension set by hand. A
 * reader copies such a place, so it keeps the plain literal rather than a call
 * that reads the shared list. The marker lets the test below hold every one of
 * them to that list.
 */
const MARKER = '<!-- stylex:include-extensions -->';

/** Documentation the marker appears in, relative to the repository root. */
const DOCUMENTED_FILES = [
  'packages/nextjs-plugin/README.md',
  'packages/postcss-plugin/README.md',
  'packages/turbopack-plugin/README.md',
  'packages/unplugin/README.md',
];

/** How many lines after the marker the extension list must appear within. */
const WINDOW_LINES = 12;

/** The three forms the documentation writes the set in. */
const LIST_PATTERNS = [
  // A glob brace, such as `src/**/*.{js,jsx}`.
  { pattern: /\*\.\{([a-z0-9,]+)\}/g, token: /[a-z0-9]+/g },
  // A quoted list, such as `['js', 'jsx']`.
  { pattern: /\[\s*'[a-z0-9]+'(?:\s*,\s*'[a-z0-9]+')+\s*\]/g, token: /'([a-z0-9]+)'/g },
  // Prose that names the extensions in back ticks, joined by commas or "and".
  { pattern: /`[a-z0-9]+`(?:(?:,|\s+and)\s*`[a-z0-9]+`)+/g, token: /`([a-z0-9]+)`/g },
];

interface Marker {
  file: string;
  /** Line the marker sits on, which is what a reader looks for. */
  line: number;
  /** Every written-out extension list found under this one marker. */
  lists: string[][];
}

/**
 * Reads every marker in one file, with the extension lists that follow it. The
 * window is read as one block rather than line by line, because a list in prose
 * wraps over two lines and each line on its own holds only a part of it.
 *
 * Each marker keeps its own lists. A marker whose list drifts out of the window
 * therefore ends up with none, and the test below reports that marker rather
 * than passing on the lists of its neighbours.
 */
function readMarkers(file: string): Marker[] {
  const lines = readFileSync(path.join(REPO_ROOT, file), 'utf8').split('\n');
  const markers: Marker[] = [];

  lines.forEach((line, index) => {
    if (!line.includes(MARKER)) {
      return;
    }

    const block = lines.slice(index + 1, index + 1 + WINDOW_LINES).join('\n');
    const lists: string[][] = [];

    for (const { pattern, token } of LIST_PATTERNS) {
      for (const match of block.matchAll(pattern)) {
        lists.push([...match[0].matchAll(token)].map(found => found[1] ?? found[0]));
      }
    }

    markers.push({ file, line: index + 1, lists });
  });

  return markers;
}

// This test guards a known fault. The example configs and the plugin READMEs
// once told readers to scan four extensions. A reader who copied that line got
// StyleX that compiled. The page then got no CSS for an .mjs or a .cjs module.
describe('the extension set that the documentation writes by hand', () => {
  const markers = DOCUMENTED_FILES.flatMap(readMarkers);

  it('carries a marker in every documented file', () => {
    const marked = new Set(markers.map(marker => marker.file));

    expect([...marked].sort()).toEqual([...DOCUMENTED_FILES].sort());
  });

  describe.each(markers.map(marker => [`${marker.file}:${marker.line}`, marker] as const))(
    'the marker at %s',
    (_where, marker) => {
      // Without this the equality test below would pass without doing anything
      // when the list drifts out of the window, because there would be no list
      // left to compare.
      it('has an extension list under it', () => {
        expect(marker.lists.length).toBeGreaterThan(0);
      });

      it('names the whole shared list', () => {
        for (const list of marker.lists) {
          expect(list).toEqual([...INCLUDE_EXTENSIONS]);
        }
      });
    }
  );
});
