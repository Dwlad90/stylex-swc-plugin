import path from 'path';

import type { Rule as StyleXRule } from '@stylexjs/babel-plugin';
import { SourceMaps } from '@stylexswc/rs-compiler';
import { describe, expect, test } from 'vitest';

import { DEFAULT_STYLEX_PACKAGES, VIRTUAL_CSS_PATTERN } from '../src/constants';
import {
  buildVirtualCssPattern,
  generateStyleXOutput,
  isAllowlistedPackage,
  parseStylexRulesFromIdentifier,
  shouldProcessFile,
} from '../src/utils';

describe('shouldProcessFile', () => {
  const project = path.join(path.sep, 'project');
  const inNodeModules = (...segments: string[]) => path.join(project, 'node_modules', ...segments);
  const defaults = { stylexPackages: DEFAULT_STYLEX_PACKAGES };

  test('scopes node_modules to the stylexPackages allowlist', () => {
    expect(shouldProcessFile(path.join(project, 'app', 'page.tsx'), defaults)).toBe(true);
    expect(shouldProcessFile(inNodeModules('react', 'index.js'), defaults)).toBe(false);
    expect(
      shouldProcessFile(inNodeModules('@stylexjs', 'open-props', 'lib', 'colors.js'), defaults)
    ).toBe(true);
    // pnpm layout: the final `node_modules/@stylexjs/...` segment still matches
    expect(
      shouldProcessFile(
        inNodeModules(
          '.pnpm',
          '@stylexjs+open-props@0.11.1',
          'node_modules',
          '@stylexjs',
          'open-props',
          'lib',
          'colors.js'
        ),
        defaults
      )
    ).toBe(true);
    expect(shouldProcessFile(path.join(project, 'app', 'styles.css'), defaults)).toBe(false);
  });

  test('matches exact stylexPackages entries only', () => {
    const custom = { stylexPackages: ['@stylexjs/', 'my-design-system'] };

    expect(shouldProcessFile(inNodeModules('my-design-system', 'tokens.js'), custom)).toBe(true);
    expect(shouldProcessFile(inNodeModules('my-design-system-extra', 'tokens.js'), custom)).toBe(
      false
    );
    expect(shouldProcessFile(inNodeModules('other-lib', 'tokens.js'), custom)).toBe(false);
  });
});

describe('isAllowlistedPackage', () => {
  test('requires a node_modules segment', () => {
    expect(isAllowlistedPackage(path.join(path.sep, 'app', 'page.tsx'), ['@stylexjs/'])).toBe(
      false
    );
  });
});

describe('buildVirtualCssPattern', () => {
  test('defaults to the packaged carrier + dummy pattern', () => {
    expect(buildVirtualCssPattern()).toBe(VIRTUAL_CSS_PATTERN);
  });

  test('a custom carrier path replaces the default carrier match', () => {
    const pattern = buildVirtualCssPattern('/project/src/styles/my-carrier.css');

    expect(pattern.test('/project/src/styles/my-carrier.css')).toBe(true);
    // dummies always belong to the chunk
    expect(pattern.test('/repo/plugin-shared/dist/stylex-virtual.css?from=App.js')).toBe(true);
    // the default packaged carrier no longer matches (avoids false positives)
    expect(pattern.test('/repo/webpack-plugin/dist/stylex.css')).toBe(false);
    // regex metacharacters in the path are escaped
    expect(pattern.test('/project/src/styles/myXcarrierXcss')).toBe(false);
  });
});

describe('parseStylexRulesFromIdentifier', () => {
  test('parses StyleX rules from `|`-segmented css module identifiers', () => {
    const rules: StyleXRule[] = [['x1abcd', { ltr: '.x1abcd{color:red}', rtl: null }, 3000]];
    const query = new URLSearchParams({
      from: 'app/layout.tsx',
      stylex: JSON.stringify(rules),
    });
    const identifier = `css|/repo/node_modules/@stylexswc/plugin-shared/dist/stylex-virtual.css?${query.toString()}|used-exports`;

    expect(parseStylexRulesFromIdentifier(identifier)).toEqual(rules);
    expect(parseStylexRulesFromIdentifier('css|/repo/app/global.css|used-exports')).toBeNull();
    expect(
      parseStylexRulesFromIdentifier('css|/repo/dist/stylex-virtual.css?from=app/layout.tsx')
    ).toBeNull();
  });

  test('ignores the carrier stylesheet', () => {
    expect(
      parseStylexRulesFromIdentifier(
        'css|/repo/node_modules/@stylexswc/plugin-shared/dist/stylex.css|used-exports'
      )
    ).toBeNull();
  });
});

/** `sourcesContent` of an emitted map, or `undefined` when it carries none. */
function contentOf(map: string | undefined): string[] | undefined {
  return (JSON.parse(map!) as { sourcesContent?: string[] }).sourcesContent;
}

describe('generateStyleXOutput', () => {
  const resourcePath = path.join(path.sep, 'project', 'app', 'page.tsx');
  const source = `import stylex from '@stylexjs/stylex';

export const styles = stylex.create({ default: { color: 'red' } });
`;
  const rsOptions = { unstable_moduleResolution: { type: 'commonJS' } } as const;

  test('emits a map when the bundler has source maps on', () => {
    const { map } = generateStyleXOutput(
      resourcePath,
      source,
      { ...rsOptions, dev: true },
      undefined,
      true
    );

    expect(map).toBeDefined();
    expect(contentOf(map)).toStrictEqual([source]);
  });

  test('skips map generation when the bundler has source maps off', () => {
    // webpack discards the map when `devtool` is false, so building one — and
    // inlining the whole source into it — is wasted work.
    const { map } = generateStyleXOutput(resourcePath, source, { ...rsOptions }, undefined, false);

    expect(map).toBeUndefined();
  });

  test('still transforms the code when map generation is skipped', () => {
    const { code, metadata } = generateStyleXOutput(
      resourcePath,
      source,
      { ...rsOptions },
      undefined,
      false
    );

    expect(code).not.toContain('stylex.create');
    expect(metadata.stylex.length).toBeGreaterThan(0);
  });

  // `maxEvaluationDepth` reaches the compiler through the same `rsOptions` spread
  // every other option does, so a wrapper does not enumerate it and nothing here
  // had exercised that. The pair below is what proves the option survives
  // `normalizeRsOptions` and lands on the transform: one source, two verdicts,
  // decided only by the option.
  //
  // 40 levels is between the compiler's default ceiling of 32 and the raised one,
  // and every level adds `+ 1` so the folded value can only be reached by the
  // full descent -- a tower folding to the same answer at any height would pass
  // whether 40 levels were evaluated or one.
  const deeplyNested = (levels: number) => {
    let expression = 'BASE';

    for (let index = 0; index < levels; index += 1) {
      expression = `(${expression} + 1)`;
    }

    return `import stylex from '@stylexjs/stylex';

const BASE = 5;

export const styles = stylex.create({ default: { zIndex: ${expression} } });
`;
  };

  test('rsOptions.maxEvaluationDepth raises the compiler ceiling', () => {
    const { code, metadata } = generateStyleXOutput(
      resourcePath,
      deeplyNested(40),
      { ...rsOptions, maxEvaluationDepth: 256 },
      undefined,
      false
    );

    expect(code).not.toContain('stylex.create');
    // 5 + 40, so the value is one only the whole descent produces.
    expect(metadata.stylex.map(([, rule]) => rule.ltr).join('')).toContain('z-index:45');
  });

  test('the same source refuses at the default ceiling', () => {
    // The guard on the case above: without it, a `maxEvaluationDepth` that was
    // silently dropped would look identical to one that worked.
    expect(() =>
      generateStyleXOutput(resourcePath, deeplyNested(40), { ...rsOptions }, undefined, false)
    ).toThrow(/too deeply nested/);
  });

  test('an explicit rsOptions.sourceMap wins over the bundler setting', () => {
    const { map } = generateStyleXOutput(
      resourcePath,
      source,
      { ...rsOptions, sourceMap: SourceMaps.True },
      undefined,
      false
    );

    expect(map).toBeDefined();
  });

  test('an explicit rsOptions.sourceMap: False stays off when the bundler is on', () => {
    const { map } = generateStyleXOutput(
      resourcePath,
      source,
      { ...rsOptions, sourceMap: SourceMaps.False },
      undefined,
      true
    );

    expect(map).toBeUndefined();
  });

  test('defaults to emitting a map when the caller omits the flag', () => {
    // Back-compat: the parameter was added after the fact, and callers that
    // predate it must keep their previous behavior.
    const { map } = generateStyleXOutput(resourcePath, source, { ...rsOptions });

    expect(map).toBeDefined();
  });

  describe('inlineSourcesContent', () => {
    // The whole authored file goes into the map. Needed for DevTools in dev,
    // pure payload — and a source disclosure — in a production build.
    test('a development build inlines the source', () => {
      const { map } = generateStyleXOutput(
        resourcePath,
        source,
        { ...rsOptions, dev: true },
        undefined,
        true
      );

      expect(contentOf(map)).toStrictEqual([source]);
    });

    test('a production build does not', () => {
      const { map } = generateStyleXOutput(
        resourcePath,
        source,
        { ...rsOptions, dev: false },
        undefined,
        true
      );

      // The map is still emitted — only the embedded text is dropped.
      expect(map).toBeDefined();
      expect(contentOf(map)).toBeUndefined();
    });

    test('an explicit inlineSourcesContent wins over the build mode', () => {
      const { map } = generateStyleXOutput(
        resourcePath,
        source,
        { ...rsOptions, dev: false, inlineSourcesContent: true },
        undefined,
        true
      );

      expect(contentOf(map)).toStrictEqual([source]);
    });

    test('an explicit false is honoured in development too', () => {
      const { map } = generateStyleXOutput(
        resourcePath,
        source,
        { ...rsOptions, dev: true, inlineSourcesContent: false },
        undefined,
        true
      );

      expect(contentOf(map)).toBeUndefined();
    });
  });

  test('an undefined bundler flag keeps maps on', () => {
    // Turbopack's loader context is a partial webpack shim and may leave
    // `this.sourceMap` undefined. Only an explicit `false` may disable maps —
    // treating "the host never said" as "off" would silently strip every map.
    const { map } = generateStyleXOutput(
      resourcePath,
      source,
      { ...rsOptions },
      undefined,
      undefined
    );

    expect(map).toBeDefined();
  });
});
