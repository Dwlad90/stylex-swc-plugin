// End-to-end coverage for the source-map shape the compiler emits. The Rust
// unit tests cover flag resolution and `sourcesContent` removal in
// isolation; these assert the values actually survive the napi boundary and
// reach the serialized map.
import { expect, test } from 'vitest';

import { transform } from '../dist/index.js';
import type { StyleXOptions } from '../dist/index.js';
import { SourceMaps } from '../dist/transform.js';

const FILENAME = '/abs/path/page.tsx';

const FIXTURE = `import stylex from "@stylexjs/stylex";

export const styles = stylex.create({
  default: {
    backgroundColor: "red",
    color: "blue",
  },
});
`;

type Map = {
  version: number;
  sources: string[];
  sourcesContent?: (string | null)[];
  names: string[];
  mappings: string;
};

function compile(options: Partial<StyleXOptions> = {}, code = FIXTURE): Map {
  const result = transform(FILENAME, code, {
    unstable_moduleResolution: { type: 'commonJS' },
    ...options,
  } as StyleXOptions);

  if (result.map == null) {
    throw new Error('expected the compiler to emit a source map');
  }

  return JSON.parse(result.map) as Map;
}

/** A map produced by earlier tooling for the same file. */
function upstreamMap(overrides: Partial<Map> = {}): string {
  return JSON.stringify({
    version: 3,
    sources: [FILENAME],
    sourcesContent: [null],
    names: [],
    mappings: 'AAAA',
    ...overrides,
  });
}

/** Number of mapping segments — line-only maps produce one per line. */
function segmentCount(mappings: string): number {
  return mappings.split(/[;,]/).filter(Boolean).length;
}

test('sourcesContent is embedded by default', () => {
  const map = compile();

  expect(map.sourcesContent).toStrictEqual([FIXTURE]);
});

test('inlineSourcesContent: false omits the key entirely', () => {
  const map = compile({ inlineSourcesContent: false });

  // Omitted, not `[null]` — an explicit null would tell consumers the source
  // is known to be unavailable rather than simply not inlined.
  expect('sourcesContent' in map).toBe(false);
  expect(map.sources).toStrictEqual([FILENAME]);
});

test('sourcesContent is the authored text, not the transformed output', () => {
  const map = compile();

  expect(map.sourcesContent?.[0]).toContain('stylex.create');
});

test('an empty file yields an empty map rather than a null content entry', () => {
  const map = compile({}, '');

  // Nothing was emitted, so no source is ever registered with the builder and
  // there is nothing to inline. The map must still be valid JSON, not a
  // `sources: [] / sourcesContent: [null]` mismatch.
  expect(map.sources).toStrictEqual([]);
  expect('sourcesContent' in map).toBe(false);
  expect(map.mappings).toBe('');
});

test('non-ASCII source text survives the napi boundary intact', () => {
  const code = `${FIXTURE}\nexport const label = "日本語 — émoji 🎨";\n`;
  const map = compile({}, code);

  expect(map.sourcesContent).toStrictEqual([code]);
});

test('a file with no stylex usage still gets its content inlined', () => {
  const code = 'export const answer = 42;\n';
  const map = compile({}, code);

  expect(map.sourcesContent).toStrictEqual([code]);
});

test('sourceMap: false emits no map at all', () => {
  const result = transform(FILENAME, FIXTURE, {
    sourceMap: SourceMaps.False,
    unstable_moduleResolution: { type: 'commonJS' },
  } as StyleXOptions);

  expect(result.map).toBeUndefined();
});

test('inline source maps carry sourcesContent in the data URI', () => {
  const result = transform(FILENAME, FIXTURE, {
    sourceMap: SourceMaps.Inline,
    unstable_moduleResolution: { type: 'commonJS' },
  } as StyleXOptions);

  const encoded = result.code.match(
    /sourceMappingURL=data:application\/json;base64,([A-Za-z0-9+/=]+)/
  )?.[1];

  expect(encoded).toBeDefined();

  const map = JSON.parse(Buffer.from(encoded as string, 'base64').toString('utf8')) as Map;

  expect(map.sourcesContent).toStrictEqual([FIXTURE]);
});

test('columns are emitted by default and collapse when turned off', () => {
  const withColumns = compile();
  const lineOnly = compile({ emitSourceMapColumns: false });

  expect(segmentCount(withColumns.mappings)).toBeGreaterThan(segmentCount(lineOnly.mappings));
});

// ── chaining onto an input source map ───────────────────────────────
// `SourceMap::build_source_map_with_config` returns the *input* map with
// adjusted mappings, including any source text supplied by earlier tooling.

test('a chained map does not synthesize missing upstream source text', () => {
  const map = compile({ inputSourceMap: upstreamMap() });

  // The compiler only has the generated loader input. Attaching it to an
  // earlier authored source would produce a plausible but incorrect map.
  expect('sourcesContent' in map).toBe(false);
});

test('a chained map keeps the upstream text when it already has some', () => {
  const map = compile({
    inputSourceMap: upstreamMap({ sourcesContent: ['// authored elsewhere\n'] }),
  });

  expect(map.sourcesContent).toStrictEqual(['// authored elsewhere\n']);
});

test('inlineSourcesContent: false is honoured on the chained path too', () => {
  const map = compile({
    inputSourceMap: upstreamMap({ sourcesContent: ['// private source\n'] }),
    inlineSourcesContent: false,
  });

  expect('sourcesContent' in map).toBe(false);
});

test('an unparseable inputSourceMap is ignored and the map still has content', () => {
  const map = compile({ inputSourceMap: 'not json' });

  expect(map.sourcesContent).toStrictEqual([FIXTURE]);
});
