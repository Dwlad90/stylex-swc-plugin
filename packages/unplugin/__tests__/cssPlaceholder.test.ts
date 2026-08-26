import { describe, expect, test, vi } from 'vitest';

import injectIntoCssTargets, {
  BUILD_CSS_PLACEHOLDER,
  pickCssAsset,
  replaceFirstMarker,
  stripMarkers,
  toBuildPlaceholder,
} from '../src/utils/cssPlaceholder';
import type { CssInjectionTarget } from '../src/utils/cssPlaceholder';

const MARKER = '/* @stylex-placeholder */';
const RULES = '.x1{color:red}';

/** A target backed by a string, standing in for whatever the host stores. */
function stringTarget(name: string, source: string) {
  const state = { name, source };

  return {
    state,
    target: {
      name,
      read: () => state.source,
      write: (next: string) => {
        state.source = next;
      },
    } satisfies CssInjectionTarget,
  };
}

const identityFinalize = async (css: string) => css;

describe('stripMarkers', () => {
  test('removes every occurrence of every marker', () => {
    expect(stripMarkers(`a${MARKER}b${MARKER}c@stylex;d`, [MARKER, '@stylex;'])).toBe('abcd');
  });

  test('leaves a source with no marker untouched', () => {
    expect(stripMarkers('.a{color:red}', [MARKER])).toBe('.a{color:red}');
  });

  test.each([
    ['an empty source', '', [MARKER]],
    ['an empty marker list', '.a{color:red}', []],
  ])('handles %s', (_label, source, markers) => {
    expect(stripMarkers(source, markers)).toBe(source);
  });

  // Splitting on an empty string would explode the source into characters, so
  // this documents that the empty marker is a no-op rather than a corruption.
  test('does not shred the source on an empty marker', () => {
    expect(stripMarkers('.a{color:red}', [''])).toBe('.a{color:red}');
  });

  test('removes overlapping markers without leaving fragments', () => {
    expect(stripMarkers('aaa', ['aa'])).toBe('a');
  });

  test('strips a marker repeated an extreme number of times', () => {
    const source = MARKER.repeat(20_000);

    expect(stripMarkers(source, [MARKER])).toBe('');
  });
});

describe('replaceFirstMarker', () => {
  test('replaces the first occurrence and removes the rest', () => {
    expect(replaceFirstMarker(`a${MARKER}b${MARKER}c`, MARKER, RULES)).toBe(`a${RULES}bc`);
  });

  test('returns the source unchanged when the marker is absent', () => {
    expect(replaceFirstMarker('.a{color:red}', MARKER, RULES)).toBe('.a{color:red}');
  });

  test('keeps the marker position rather than appending', () => {
    expect(replaceFirstMarker(`head${MARKER}tail`, MARKER, RULES)).toBe(`head${RULES}tail`);
  });

  // `String#replace` would treat these as replacement patterns and mangle the
  // CSS, which is why the implementation splits instead.
  test.each(['$&', "$'", '$`', '$1', '$$'])('keeps a %s sequence in the rules intact', sequence => {
    const rules = `.x1{content:"${sequence}"}`;

    expect(replaceFirstMarker(`a${MARKER}b`, MARKER, rules)).toBe(`a${rules}b`);
  });

  test('keeps a replacement-pattern sequence in the surrounding CSS intact', () => {
    expect(replaceFirstMarker(`.a{content:"$&"}${MARKER}`, MARKER, RULES)).toBe(
      `.a{content:"$&"}${RULES}`
    );
  });

  test('replaces with an empty string when there are no rules', () => {
    expect(replaceFirstMarker(`a${MARKER}b`, MARKER, '')).toBe('ab');
  });

  test('handles a marker at the very start and very end', () => {
    expect(replaceFirstMarker(`${MARKER}body{margin:0}${MARKER}`, MARKER, RULES)).toBe(
      `${RULES}body{margin:0}`
    );
  });

  test('handles an extremely large stylesheet', () => {
    const filler = '.pad{margin:0}'.repeat(60_000);
    const result = replaceFirstMarker(`${filler}${MARKER}${filler}`, MARKER, RULES);

    expect(result).toBe(`${filler}${RULES}${filler}`);
  });

  test('handles rules larger than the stylesheet holding the marker', () => {
    const rules = '.x{color:red}'.repeat(60_000);

    expect(replaceFirstMarker(MARKER, MARKER, rules)).toBe(rules);
  });
});

describe('toBuildPlaceholder', () => {
  test('replaces every marker with the build placeholder', () => {
    expect(toBuildPlaceholder(`a${MARKER}b${MARKER}`, MARKER)).toBe(
      `a${BUILD_CSS_PLACEHOLDER}b${BUILD_CSS_PLACEHOLDER}`
    );
  });

  test('is a statement at-rule, which is what survives minification', () => {
    expect(BUILD_CSS_PLACEHOLDER.startsWith('@')).toBe(true);
    expect(BUILD_CSS_PLACEHOLDER.endsWith(';')).toBe(true);
    expect(BUILD_CSS_PLACEHOLDER).not.toContain('/*');
  });
});

describe('pickCssAsset', () => {
  test('returns null for no assets', () => {
    expect(pickCssAsset([])).toBeNull();
  });

  test.each([
    [['a.css', 'index.css', 'style.css'], 'index.css'],
    [['a.css', 'style.css', 'main.css'], 'style.css'],
    [['a.css', 'main.css'], 'main.css'],
    [['a.css', 'b.css'], 'a.css'],
  ])('prefers the well-known name in %s', (assets, expected) => {
    expect(pickCssAsset(assets)).toBe(expected);
  });

  test('matches a well-known name at a path boundary', () => {
    expect(pickCssAsset(['assets/a.css', 'assets/index.css'])).toBe('assets/index.css');
  });

  test('does not mistake a suffix for a well-known name', () => {
    // `vendor-index.css` is not `index.css`, so the first asset wins instead.
    expect(pickCssAsset(['a.css', 'vendor-index.css'])).toBe('a.css');
  });

  test('handles an absolute path', () => {
    expect(pickCssAsset(['/out/a.css', '/out/index.css'])).toBe('/out/index.css');
  });

  test('handles a very large asset list', () => {
    const assets = Array.from({ length: 50_000 }, (_, index) => `chunk-${index}.css`);
    assets.push('index.css');

    expect(pickCssAsset(assets)).toBe('index.css');
  });
});

describe('injectIntoCssTargets', () => {
  test('replaces the marker in the first target that has one', async () => {
    const first = stringTarget('a.css', `head${MARKER}tail`);
    const second = stringTarget('b.css', '.b{outline:0}');

    const handled = await injectIntoCssTargets(
      [first.target, second.target],
      [MARKER],
      RULES,
      identityFinalize
    );

    expect(handled).toBe(true);
    expect(first.state.source).toBe(`head${RULES}tail`);
    expect(second.state.source).toBe('.b{outline:0}');
  });

  test('strips the marker from later targets rather than repeating the rules', async () => {
    const first = stringTarget('a.css', `a${MARKER}`);
    const second = stringTarget('b.css', `b${MARKER}`);

    await injectIntoCssTargets([first.target, second.target], [MARKER], RULES, identityFinalize);

    expect(first.state.source).toBe(`a${RULES}`);
    expect(second.state.source).toBe('b');
  });

  test('removes the marker when there are no rules to inject', async () => {
    const only = stringTarget('a.css', `body{margin:0}${MARKER}`);

    const handled = await injectIntoCssTargets([only.target], [MARKER], null, identityFinalize);

    expect(handled).toBe(true);
    expect(only.state.source).toBe('body{margin:0}');
  });

  test('accepts either marker form and prefers whichever the source holds', async () => {
    const build = stringTarget('a.css', `a${BUILD_CSS_PLACEHOLDER}`);

    await injectIntoCssTargets(
      [build.target],
      [BUILD_CSS_PLACEHOLDER, MARKER],
      RULES,
      identityFinalize
    );

    expect(build.state.source).toBe(`a${RULES}`);
  });

  test('appends to the preferred target when no marker reached the output', async () => {
    const other = stringTarget('a.css', '.a{outline:0}');
    const preferred = stringTarget('index.css', '.i{outline:0}');

    const handled = await injectIntoCssTargets(
      [other.target, preferred.target],
      [MARKER],
      RULES,
      identityFinalize
    );

    expect(handled).toBe(true);
    expect(preferred.state.source).toBe(`.i{outline:0}\n${RULES}`);
    expect(other.state.source).toBe('.a{outline:0}');
  });

  test('writes only the rules when the fallback target is empty', async () => {
    const empty = stringTarget('index.css', '');

    await injectIntoCssTargets([empty.target], [MARKER], RULES, identityFinalize);

    expect(empty.state.source).toBe(RULES);
  });

  test.each([
    ['no targets and rules to place', [] as const, RULES, false],
    ['no targets and no rules', [] as const, null, true],
  ])('reports %s correctly', async (_label, _targets, collected, expected) => {
    expect(await injectIntoCssTargets([], [MARKER], collected, identityFinalize)).toBe(expected);
  });

  test('names the receiving target when finalizing the rules', async () => {
    const finalize = vi.fn<(css: string, name: string) => Promise<string>>(
      async (css, name) => `${css}/*${name}*/`
    );
    const only = stringTarget('app.css', MARKER);

    await injectIntoCssTargets([only.target], [MARKER], RULES, finalize);

    expect(finalize).toHaveBeenCalledTimes(1);
    expect(finalize.mock.calls[0]?.[1]).toBe('app.css');
    expect(only.state.source).toBe(`${RULES}/*app.css*/`);
  });

  test('finalizes once even with many marker-bearing targets', async () => {
    const finalize = vi.fn<(css: string, name: string) => Promise<string>>(async css => css);
    const targets = Array.from({ length: 200 }, (_, index) =>
      stringTarget(`chunk-${index}.css`, `c${index}${MARKER}`)
    );

    await injectIntoCssTargets(
      targets.map(entry => entry.target),
      [MARKER],
      RULES,
      finalize
    );

    expect(finalize).toHaveBeenCalledTimes(1);
    expect(targets.filter(entry => entry.state.source.includes(RULES))).toHaveLength(1);
    expect(targets.some(entry => entry.state.source.includes(MARKER))).toBe(false);
  });

  test('awaits asynchronous reads and writes', async () => {
    const state = { source: `a${MARKER}` };
    const target: CssInjectionTarget = {
      name: 'a.css',
      read: async () => state.source,
      write: async next => {
        await Promise.resolve();
        state.source = next;
      },
    };

    await injectIntoCssTargets([target], [MARKER], RULES, identityFinalize);

    expect(state.source).toBe(`a${RULES}`);
  });

  test('reads each target exactly once', async () => {
    const read = vi.fn<() => string>(() => `a${MARKER}`);
    const target: CssInjectionTarget = { name: 'a.css', read, write: () => {} };

    await injectIntoCssTargets([target], [MARKER], RULES, identityFinalize);

    expect(read).toHaveBeenCalledTimes(1);
  });

  test('reads the fallback target only once as well', async () => {
    const read = vi.fn<() => string>(() => '.a{outline:0}');
    const target: CssInjectionTarget = { name: 'index.css', read, write: () => {} };

    await injectIntoCssTargets([target], [MARKER], RULES, identityFinalize);

    expect(read).toHaveBeenCalledTimes(1);
  });

  test('surfaces a write failure rather than reporting success', async () => {
    const target: CssInjectionTarget = {
      name: 'a.css',
      read: () => `a${MARKER}`,
      write: () => {
        throw new Error('read-only output');
      },
    };

    await expect(injectIntoCssTargets([target], [MARKER], RULES, identityFinalize)).rejects.toThrow(
      'read-only output'
    );
  });

  test('handles an extremely large rule set across many targets', async () => {
    const rules = '.x{color:red}'.repeat(40_000);
    const targets = Array.from({ length: 50 }, (_, index) =>
      stringTarget(`chunk-${index}.css`, `c${index}${MARKER}`)
    );

    const handled = await injectIntoCssTargets(
      targets.map(entry => entry.target),
      [MARKER],
      rules,
      identityFinalize
    );

    expect(handled).toBe(true);
    expect(targets[0]?.state.source).toBe(`c0${rules}`);
    expect(targets.at(-1)?.state.source).toBe(`c49`);
  });
});
