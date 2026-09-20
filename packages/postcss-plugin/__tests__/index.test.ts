import fs from 'node:fs';
import os from 'node:os';
import path from 'path';

import postcss from 'postcss';
import { describe, expect, test, vi } from 'vitest';

import {
  resolveImportSourcesWithMetadata,
  resolveIncludeWithMetadata,
  resolveExclude,
  AUTO_DISCOVERY_EXCLUDES,
  DEFAULT_IMPORT_SOURCES,
  DEFAULT_INCLUDE_GLOB,
} from '../src/discovery';
import createPlugin from '../src/plugin';

describe('@stylexswc/postcss-plugin', () => {
  const fixturesDir = path.resolve(__dirname, '__fixtures__');
  const autoDiscoveryFixturesDir = path.resolve(__dirname, '__auto_discovery_fixtures__');

  /**
   * Creates a temporary copy of the auto-discovery fixtures directory and
   * sets up mock node_modules (stylex-custom-lib, non-stylex-lib, .cache).
   * Returns the temp directory path; caller must clean up with fs.rmSync.
   */
  function createAutoDiscoveryFixture(): string {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'stylex-postcss-auto-discovery-'));

    fs.cpSync(autoDiscoveryFixturesDir, tempDir, { recursive: true });

    // A dependency that uses react-strict-dom as a StyleX import source
    const stylexLibDir = path.join(tempDir, 'node_modules', 'stylex-custom-lib');
    fs.mkdirSync(stylexLibDir, { recursive: true });
    fs.writeFileSync(
      path.join(stylexLibDir, 'package.json'),
      JSON.stringify(
        {
          name: 'stylex-custom-lib',
          version: '1.0.0',
          main: 'index.js',
          dependencies: {
            'react-strict-dom': '^0.0.0',
          },
        },
        null,
        2
      ),
      'utf8'
    );
    fs.writeFileSync(
      path.join(stylexLibDir, 'index.js'),
      [
        "import { css } from 'react-strict-dom';",
        '',
        'export const styles = css.create({',
        '  lib: {',
        "    backgroundColor: 'orange',",
        '  },',
        '});',
        '',
      ].join('\n'),
      'utf8'
    );

    // A dependency that does NOT use StyleX
    const nonStylexLibDir = path.join(tempDir, 'node_modules', 'non-stylex-lib');
    fs.mkdirSync(nonStylexLibDir, { recursive: true });
    fs.writeFileSync(
      path.join(nonStylexLibDir, 'package.json'),
      JSON.stringify({ name: 'non-stylex-lib', version: '1.0.0', main: 'index.js' }, null, 2),
      'utf8'
    );
    fs.writeFileSync(path.join(nonStylexLibDir, 'index.js'), 'export const v = 1;\n', 'utf8');

    // A hidden file inside node_modules/.cache that should be excluded by auto-discovery
    const cachedNodeModulesDir = path.join(tempDir, 'node_modules', '.cache');
    fs.mkdirSync(cachedNodeModulesDir, { recursive: true });
    fs.writeFileSync(
      path.join(cachedNodeModulesDir, 'hidden-stylex.js'),
      [
        "import { css } from 'react-strict-dom';",
        '',
        'export const styles = css.create({',
        '  hidden: {',
        "    backgroundColor: 'hotpink',",
        '  },',
        '});',
        '',
      ].join('\n'),
      'utf8'
    );

    // Nested node_modules inside stylex-custom-lib (simulates transitive deps like
    // @stylexswc/design-system → @stylexswc/rs-compiler → @stylexswc/transform/tests).
    // These files must never be scanned during auto-discovery.
    const nestedNodeModulesDir = path.join(
      stylexLibDir,
      'node_modules',
      'transitive-dep',
      'tests',
      'fixtures'
    );
    fs.mkdirSync(nestedNodeModulesDir, { recursive: true });
    fs.writeFileSync(
      path.join(nestedNodeModulesDir, 'colors.stylex.js'),
      // This file uses TypeScript enum syntax that the RS compiler cannot parse.
      'enum Color { Red = "red", Blue = "blue" }\n',
      'utf8'
    );

    return tempDir;
  }

  async function runStylexPostcss(options = {}, inputCSS = '@stylex;') {
    // Create a new instance for each test as the plugin is stateful
    const stylexPostcssPlugin = createPlugin();

    const plugin = stylexPostcssPlugin({
      cwd: fixturesDir,
      include: ['**/*.js'],
      ...options,
    });

    const processor = postcss([plugin]);
    const result = await processor.process(inputCSS, {
      from: path.join(fixturesDir, 'input.css'),
    });

    return result;
  }

  async function runAutoDiscoveryPostcss(
    options: Record<string, unknown> | ((fixtureDir: string) => Record<string, unknown>) = {},
    inputCSS = '@stylex;'
  ): Promise<{ css: string; messages: postcss.Message[] }> {
    const fixtureDir = createAutoDiscoveryFixture();
    const stylexPostcssPlugin = createPlugin();
    const resolvedOptions = typeof options === 'function' ? options(fixtureDir) : options;

    const basePluginOptions = {
      cwd: fixtureDir,
      // Pass react-strict-dom as an import source so auto-discovery can find
      // stylex-custom-lib (which depends on react-strict-dom) and so the
      // RS compiler can transform files that use it.
      rsOptions: {
        dev: false,
        runtimeInjection: false,
        importSources: [{ from: 'react-strict-dom', as: 'css' }],
      },
    };

    const plugin = stylexPostcssPlugin({
      ...basePluginOptions,
      ...resolvedOptions,
    });

    const processor = postcss([plugin]);
    try {
      const result = await processor.process(inputCSS, {
        from: path.join(fixtureDir, 'input.css'),
      });
      return { css: result.css, messages: result.messages };
    } finally {
      fs.rmSync(fixtureDir, { recursive: true, force: true });
    }
  }

  // ─── Core plugin tests ───────────────────────────────────────────────────────

  test('extracts CSS from StyleX files', async () => {
    const result = await runStylexPostcss();

    expect(result.css).toMatchInlineSnapshot(`
      ".x1u857p9{background-color:green}
      .xrkmrrc{background-color:red}"
    `);

    // Check that messages contain dependency information
    expect(result.messages.length).toBeGreaterThan(0);
    expect(result.messages.some(m => m.type === 'dir-dependency')).toBe(true);
  });

  test('handles empty CSS input without @stylex rule', async () => {
    const result = await runStylexPostcss({}, '/* No stylex rule here */');

    expect(result.css).toMatchInlineSnapshot('"/* No stylex rule here */"');
    expect(result.messages).toHaveLength(0);
  });

  test('supports CSS layers', async () => {
    const result = await runStylexPostcss({ useCSSLayers: true });

    expect(result.css).toContain('@layer');
    expect(result.css).toMatchInlineSnapshot(`
      "
      @layer priority1;
      @layer priority1{
      .x1u857p9{background-color:green}
      .xrkmrrc{background-color:red}
      }"
    `);
  });

  test('handles exclude patterns', async () => {
    const result = await runStylexPostcss({
      exclude: ['**/styles-second.js'],
    });

    // Should not contain styles-second.js styles
    expect(result.css).not.toContain('green');

    expect(result.css).toMatchInlineSnapshot('".xrkmrrc{background-color:red}"');
  });

  test('respects string syntax for importSources', async () => {
    // Default importSources should not process any files
    const defaultResult = await runStylexPostcss({
      include: ['**/import-sources-*.js'],
    });

    expect(defaultResult.css).toBe('');

    // Custom importSources should process only import-sources-string.js
    const customResult = await runStylexPostcss({
      include: ['**/import-sources-*.js'],
      rsOptions: {
        dev: false,
        runtimeInjection: false,
        importSources: ['custom'],
      },
    });

    expect(customResult.css).toMatchInlineSnapshot('".x1t391ir{background-color:blue}"');
  });

  test('supports object syntax for importSources', async () => {
    const result = await runStylexPostcss({
      include: ['**/import-sources-object.js'],
      rsOptions: {
        dev: false,
        runtimeInjection: false,
        importSources: [{ as: 'css', from: 'react-strict-dom' }],
      },
    });

    expect(result.css).toMatchInlineSnapshot('".x1cu41gw{background-color:yellow}"');
  });

  test('skips files that do not match include/exclude patterns', async () => {
    const result = await runStylexPostcss({
      include: ['**/styles-second.js'],
    });

    // Should contain styles-second.js styles but not styles.js
    expect(result.css).not.toContain('red');

    expect(result.css).toMatchInlineSnapshot('".x1u857p9{background-color:green}"');
  });

  test('dedupes files matched by relative and absolute include patterns', async () => {
    const targetFile = path.join(fixturesDir, 'styles.js');
    const readFileSpy = vi.spyOn(fs, 'readFileSync');

    try {
      const result = await runStylexPostcss({
        include: ['styles.js', targetFile],
      });

      expect(result.css).toContain('background-color:red');

      const readsForTargetFile = readFileSpy.mock.calls.filter(([file]) => {
        return path.normalize(String(file)) === path.normalize(targetFile);
      });

      // File should only be read once even though it matched two include patterns
      expect(readsForTargetFile).toHaveLength(1);
    } finally {
      readFileSpy.mockRestore();
    }
  });

  test('auto-discovers include globs when include is omitted', async () => {
    const result = await runStylexPostcss({
      include: undefined,
    });

    expect(result.css).toContain('background-color:green');
    expect(result.css).toContain('background-color:red');
  });

  // ─── Auto-discovery integration ──────────────────────────────────────────────

  test('auto-discovers StyleX dependency directories', async () => {
    const result = await runAutoDiscoveryPostcss();

    // local-stylex.js (@stylexjs/stylex)
    expect(result.css).toContain('color:red');
    // local-rsd.js (react-strict-dom)
    expect(result.css).toContain('color:purple');
    // stylex-custom-lib (discovered dependency)
    expect(result.css).toContain('background-color:orange');

    expect(
      result.messages.some(
        message =>
          message.type === 'dir-dependency' && String(message.dir).includes('stylex-custom-lib')
      )
    ).toBe(true);
  });

  test('does not scan nested node_modules inside discovered dependency packages', async () => {
    // Regression: auto-discovered packages (e.g. @stylexswc/design-system) may have
    // their own node_modules with deeply-nested test fixtures that the RS compiler
    // cannot parse. Those files must never be reached.
    await expect(runAutoDiscoveryPostcss()).resolves.toMatchObject({
      css: expect.stringContaining('background-color:orange'),
    });

    // Additional assertion: the transitive dep's fixture file did NOT cause a parse error.
    // If it had been scanned the call above would have thrown.
  });

  test('does not auto-discover dependency directories when include is explicitly provided', async () => {
    const result = await runAutoDiscoveryPostcss({
      include: ['src/local-stylex.js'],
    });

    expect(result.css).toContain('color:red');
    expect(result.css).not.toContain('background-color:orange');
    expect(
      result.messages.some(
        message =>
          message.type === 'dir-dependency' && String(message.dir).includes('stylex-custom-lib')
      )
    ).toBe(false);
  });

  test('prefers explicit importSources over rsOptions.importSources for discovery', async () => {
    // Providing explicit importSources at plugin level that only includes @stylexjs/stylex
    // should prevent discovery of react-strict-dom-based dependencies
    const result = await runAutoDiscoveryPostcss({
      importSources: ['@stylexjs/stylex'],
      rsOptions: {
        dev: false,
        runtimeInjection: false,
        importSources: ['@stylexjs/stylex'],
      },
    });

    expect(result.css).toContain('color:red');
    // react-strict-dom source not active → no purple, no orange
    expect(result.css).not.toContain('color:purple');
    expect(result.css).not.toContain('background-color:orange');
  });

  test('falls back to default importSources when rsOptions omit importSources', async () => {
    const result = await runStylexPostcss({
      include: ['**/styles.js', '**/import-sources-string.js'],
      rsOptions: {
        dev: false,
        runtimeInjection: false,
      },
    });

    expect(result.css).toContain('background-color:red');
    expect(result.css).not.toContain('background-color:blue');
  });

  test('processes absolute include paths in node_modules even when node_modules is excluded', async () => {
    const result = await runAutoDiscoveryPostcss((fixtureDir: string) => ({
      include: [path.join(fixtureDir, 'node_modules/stylex-custom-lib/index.js')],
      exclude: ['**/node_modules/**'],
      rsOptions: {
        dev: false,
        runtimeInjection: false,
        importSources: [{ from: 'react-strict-dom', as: 'css' }],
      },
    }));

    expect(result.css).toContain('background-color:orange');
  });

  test('keeps specific node_modules excludes when absolute includes point to node_modules', async () => {
    const result = await runAutoDiscoveryPostcss((fixtureDir: string) => ({
      include: [path.join(fixtureDir, 'node_modules/**/*.js')],
      exclude: ['**/node_modules/**', '**/node_modules/.cache/**'],
      rsOptions: {
        dev: false,
        runtimeInjection: false,
        importSources: [{ from: 'react-strict-dom', as: 'css' }],
      },
    }));

    expect(result.css).toContain('background-color:orange');
    expect(result.css).not.toContain('background-color:hotpink');
  });

  test('logs discovery details in debug mode', async () => {
    const previousDebugValue = process.env.STYLEX_POSTCSS_DEBUG;
    process.env.STYLEX_POSTCSS_DEBUG = '1';
    const infoSpy = vi.spyOn(console, 'info').mockImplementation(() => {});

    try {
      await runAutoDiscoveryPostcss();

      const log = infoSpy.mock.calls.find(([arg]) =>
        String(arg).includes('Auto-discovery details')
      );

      expect(log).toBeDefined();
      expect(String(log?.[0])).toContain('"importSourcesSource"');
      expect(String(log?.[0])).toContain('"include"');
      expect(String(log?.[0])).toContain('"exclude"');
      expect(String(log?.[0])).toContain('"discoveredDependencyDirectories"');
    } finally {
      if (previousDebugValue == null) {
        delete process.env.STYLEX_POSTCSS_DEBUG;
      } else {
        process.env.STYLEX_POSTCSS_DEBUG = previousDebugValue;
      }
      infoSpy.mockRestore();
    }
  });

  // ─── Rebuild bookkeeping tests ───────────────────────────────────────────────

  /**
   * Makes a build directory for one test. The tests above use a glob on the
   * shared fixtures directory. A file that this fixture writes stays out of
   * that glob. The caller gets a writer for the one source file, a build
   * function, and a cleanup function.
   */
  function createRebuildFixture(fileName: string) {
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'stylex-postcss-rebuild-'));
    const stylesPath = path.join(dir, fileName);

    // The last mtime that this fixture wrote. Two writes can occur in the
    // same millisecond, and a `Date` removes the fraction of a millisecond.
    // Therefore each new mtime must start from the last mtime that this
    // fixture wrote, and not from the mtime on the disk.
    let lastMtimeMs = 0;

    // Writes the source file and increases its mtime. The build then sees the
    // change, also on a file system that keeps a low-resolution time.
    function writeStyles(contents: string) {
      fs.writeFileSync(stylesPath, contents, 'utf8');
      lastMtimeMs = Math.ceil(Math.max(fs.statSync(stylesPath).mtimeMs, lastMtimeMs)) + 1000;
      fs.utimesSync(stylesPath, new Date(lastMtimeMs), new Date(lastMtimeMs));
    }

    const processor = postcss([
      createPlugin()({
        cwd: dir,
        include: [fileName],
      }),
    ]);

    const input = { from: path.join(dir, 'input.css') };

    // Writes the source file but keeps the mtime that the last write set. The
    // map thus still agrees with the file. Only a build that reads the file
    // again can find the new rules.
    function writeStylesKeepingMtime(contents: string) {
      fs.writeFileSync(stylesPath, contents, 'utf8');
      fs.utimesSync(stylesPath, new Date(lastMtimeMs), new Date(lastMtimeMs));
    }

    return {
      writeStyles,
      writeStylesKeepingMtime,
      removeStyles: () => fs.rmSync(stylesPath, { force: true }),
      build: () => processor.process('@stylex;', input),
      cleanup: () => fs.rmSync(dir, { recursive: true, force: true }),
    };
  }

  // The lines of a source file with one rule. The rule sets the specified
  // background color.
  function stylesSourceLines(backgroundColor: string) {
    return [
      "import * as stylex from '@stylexjs/stylex';",
      '',
      'export const styles = stylex.create({',
      '  container: {',
      `    backgroundColor: '${backgroundColor}',`,
      '  },',
      '});',
    ];
  }

  // This is a source file that the compiler can parse.
  function stylesSource(backgroundColor: string) {
    return `${stylesSourceLines(backgroundColor).join('\n')}\n`;
  }

  // This is the same file without the two lines that close the object and the
  // call. The compiler cannot parse it.
  function unparsableStylesSource() {
    return `${stylesSourceLines('red').slice(0, -2).join('\n')}\n`;
  }

  /**
   * This test records the full rule. Each build that ends after a file changes
   * must show that change. The test cannot fail today, because `build()` has
   * no await. Two builds therefore cannot overlap. The test keeps its value as
   * a guard: if `build()` gets an await later, this test fails when the mtime
   * goes to the map too early. The test after this one shows the same defect
   * on the path that the synchronous build does have.
   */
  test('concurrent builds do not skip files mid transform', async () => {
    const fixture = createRebuildFixture('concurrent-styles.js');

    try {
      fixture.writeStyles(stylesSource('red'));
      await fixture.build();

      fixture.writeStyles(stylesSource('blue'));

      const [first, second] = await Promise.all([fixture.build(), fixture.build()]);

      expect(first.css).toContain('background-color:blue');
      expect(second.css).toContain('background-color:blue');
      expect(first.css).not.toContain('background-color:red');
      expect(second.css).not.toContain('background-color:red');
    } finally {
      fixture.cleanup();
    }
  });

  test('a file whose transform fails is transformed again on the next build', async () => {
    const fixture = createRebuildFixture('failing-styles.js');

    try {
      fixture.writeStyles(unparsableStylesSource());

      await expect(fixture.build()).rejects.toThrow('Failed to parse file');

      // The file did not change, thus its mtime is the same as the mtime that
      // the first build saw. The second build must read the file again,
      // because the first build did not collect its rules.
      await expect(fixture.build()).rejects.toThrow('Failed to parse file');

      // Repair the file, but keep the mtime. The third build must find the
      // new rules, and not an empty result.
      fixture.writeStylesKeepingMtime(stylesSource('red'));

      const repaired = await fixture.build();

      expect(repaired.css).toContain('background-color:red');
    } finally {
      fixture.cleanup();
    }
  });

  /**
   * The same claim on the path that watch mode actually takes. The plugin
   * latches `shouldSkipTransformError` after the first build that finishes, so
   * every later build swallows a transform error and logs a warning instead of
   * throwing. That build collected no rules, so it must write no mtime either
   * -- otherwise the file is skipped for as long as it is not edited, and its
   * classes stay missing after the real cause is repaired.
   */
  test('a file whose transform error is swallowed is transformed again on the next build', async () => {
    const fixture = createRebuildFixture('watch-mode-styles.js');
    const warnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});

    try {
      // The first build finishes, which is what turns the swallowing on.
      fixture.writeStyles(stylesSource('blue'));
      await fixture.build();

      fixture.writeStyles(unparsableStylesSource());

      const warned = await fixture.build();

      expect(warnSpy).toHaveBeenCalledWith(expect.stringContaining('Failed to transform'));
      expect(warned.css).not.toContain('background-color:red');

      // Repair the file and keep the mtime, which is what an unrelated later
      // build sees: only a build that reads the file again finds the rules.
      fixture.writeStylesKeepingMtime(stylesSource('red'));

      const repaired = await fixture.build();

      expect(repaired.css).toContain('background-color:red');
    } finally {
      warnSpy.mockRestore();
      fixture.cleanup();
    }
  });

  /**
   * The assembler options reach the stylesheet through the whole plugin path:
   * `rsOptions` -> `normalizeRsOptions` -> `toTransformedOptions` -> the
   * bundle. `useLegacyClassnamesSort` is asserted against the assembler
   * directly elsewhere, which measures the assembler rather than the wiring —
   * a plugin that dropped the option on the way would still pass that.
   *
   * Two rules that tie on priority are ordered by declaration without the
   * option and by class name with it, and the two names sort the other way
   * round, so the output says which rule ran.
   */
  test('useLegacyClassnamesSort reaches the stylesheet through the plugin', async () => {
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'stylex-postcss-sort-'));

    // `color` is declared second and hashes to a name that sorts first.
    fs.writeFileSync(
      path.join(dir, 'sorted-styles.js'),
      [
        "import * as stylex from '@stylexjs/stylex';",
        '',
        'export const styles = stylex.create({',
        "  a: { backgroundColor: 'blue', color: 'red' },",
        '});',
        '',
      ].join('\n')
    );

    const assemble = async (useLegacyClassnamesSort: boolean) => {
      const processor = postcss([
        createPlugin()({
          cwd: dir,
          include: ['sorted-styles.js'],
          rsOptions: { useLegacyClassnamesSort },
        }),
      ]);

      const result = await processor.process('@stylex;', { from: path.join(dir, 'input.css') });

      return result.css;
    };

    try {
      const byDeclaration = await assemble(false);
      const byClassName = await assemble(true);

      expect(byDeclaration.indexOf('.x1t391ir')).toBeLessThan(byDeclaration.indexOf('.x1e2nbdu'));
      expect(byClassName.indexOf('.x1e2nbdu')).toBeLessThan(byClassName.indexOf('.x1t391ir'));
    } finally {
      fs.rmSync(dir, { recursive: true, force: true });
    }
  });

  /**
   * A file edited from having rules to having none must lose the rules it
   * declared before, whether the compiler still reads it or not. Both routes
   * end in the stylesheet keeping classes no source declares.
   */
  test('a file that loses its rules loses them from the stylesheet', async () => {
    const fixture = createRebuildFixture('emptied-styles.js');

    try {
      fixture.writeStyles(stylesSource('red'));

      expect((await fixture.build()).css).toContain('background-color:red');

      // Still a StyleX file, and now with nothing in it.
      fixture.writeStyles(
        "import * as stylex from '@stylexjs/stylex';\n\nexport const styles = stylex.create({});\n"
      );

      expect((await fixture.build()).css).not.toContain('background-color:red');

      fixture.writeStyles(stylesSource('red'));

      expect((await fixture.build()).css).toContain('background-color:red');

      // No StyleX import at all, so the compiler is never asked about it.
      fixture.writeStyles('export const styles = {};\n');

      expect((await fixture.build()).css).not.toContain('background-color:red');
    } finally {
      fixture.cleanup();
    }
  });

  /**
   * A file deleted between two builds must lose its rules as well, which is
   * the third way a file stops declaring what it declared before. The other
   * two are beside this one.
   */
  test('a file deleted between builds loses its rules', async () => {
    const fixture = createRebuildFixture('deleted-styles.js');

    try {
      fixture.writeStyles(stylesSource('red'));

      expect((await fixture.build()).css).toContain('background-color:red');

      fixture.removeStyles();

      expect((await fixture.build()).css).not.toContain('background-color:red');
    } finally {
      fixture.cleanup();
    }
  });
});

// ─── Discovery unit tests ────────────────────────────────────────────────────

describe('discovery', () => {
  describe('resolveImportSourcesWithMetadata', () => {
    test('returns explicit importSources when provided', () => {
      const result = resolveImportSourcesWithMetadata({
        importSources: ['my-stylex-lib'],
      });
      expect(result.source).toBe('postcss-option');
      expect(result.importSources).toEqual(['my-stylex-lib']);
    });

    test('infers from rsOptions.importSources and merges with defaults', () => {
      const result = resolveImportSourcesWithMetadata({
        rsOptions: {
          importSources: ['my-stylex-lib'],
        },
      });
      expect(result.source).toBe('rs-options');
      expect(result.importSources).toEqual(
        expect.arrayContaining([...DEFAULT_IMPORT_SOURCES, 'my-stylex-lib'])
      );
    });

    test('falls back to defaults when nothing is provided', () => {
      const result = resolveImportSourcesWithMetadata({});
      expect(result.source).toBe('defaults');
      expect(result.importSources).toEqual(DEFAULT_IMPORT_SOURCES);
    });

    test('explicit importSources takes precedence over rsOptions', () => {
      const result = resolveImportSourcesWithMetadata({
        importSources: ['explicit-lib'],
        rsOptions: {
          importSources: ['rs-lib'],
        },
      });
      expect(result.source).toBe('postcss-option');
      expect(result.importSources).toEqual(['explicit-lib']);
    });

    test('deduplicates importSources', () => {
      const result = resolveImportSourcesWithMetadata({
        importSources: ['@stylexjs/stylex', '@stylexjs/stylex', 'stylex'],
      });
      expect(result.importSources).toEqual(['@stylexjs/stylex', 'stylex']);
    });

    test('handles object-form importSources in rsOptions', () => {
      const result = resolveImportSourcesWithMetadata({
        rsOptions: {
          importSources: [{ from: 'react-strict-dom', as: 'css' }],
        },
      });
      expect(result.source).toBe('rs-options');
      expect(result.importSources).toEqual(
        expect.arrayContaining([{ from: 'react-strict-dom', as: 'css' }])
      );
    });
  });

  describe('resolveIncludeWithMetadata', () => {
    test('returns explicit include when provided', () => {
      const result = resolveIncludeWithMetadata({
        cwd: '/tmp/fake-project',
        include: ['src/**/*.ts'],
        importSources: DEFAULT_IMPORT_SOURCES,
      });
      expect(result.hasExplicitInclude).toBe(true);
      expect(result.include).toEqual(['src/**/*.ts']);
      expect(result.discoveredDependencyDirectories).toEqual([]);
    });

    test('auto-discovers when include is omitted', () => {
      const result = resolveIncludeWithMetadata({
        cwd: '/tmp/fake-project',
        importSources: DEFAULT_IMPORT_SOURCES,
      });
      expect(result.hasExplicitInclude).toBe(false);
      expect(result.include).toContain(DEFAULT_INCLUDE_GLOB);
    });

    test('auto-discovers when include is empty array', () => {
      const result = resolveIncludeWithMetadata({
        cwd: '/tmp/fake-project',
        include: [],
        importSources: DEFAULT_IMPORT_SOURCES,
      });
      expect(result.hasExplicitInclude).toBe(false);
      expect(result.include).toContain(DEFAULT_INCLUDE_GLOB);
    });

    test('deduplicates discovered include patterns', () => {
      const result = resolveIncludeWithMetadata({
        cwd: '/tmp/fake-project',
        include: [DEFAULT_INCLUDE_GLOB, DEFAULT_INCLUDE_GLOB],
        importSources: DEFAULT_IMPORT_SOURCES,
      });
      const defaultGlobCount = result.include.filter(p => p === DEFAULT_INCLUDE_GLOB).length;
      expect(defaultGlobCount).toBe(1);
    });
  });

  describe('resolveExclude', () => {
    test('returns user excludes when include is explicit', () => {
      const result = resolveExclude({
        include: ['src/**/*.ts'],
        exclude: ['**/*.test.ts'],
      });
      expect(result).toEqual(['**/*.test.ts']);
    });

    test('adds auto-discovery excludes when include is omitted', () => {
      const result = resolveExclude({
        exclude: ['**/*.test.ts'],
      });
      expect(result).toEqual(expect.arrayContaining([...AUTO_DISCOVERY_EXCLUDES, '**/*.test.ts']));
    });

    test('adds auto-discovery excludes when include is empty', () => {
      const result = resolveExclude({
        include: [],
        exclude: [],
      });
      expect(result).toEqual(expect.arrayContaining(AUTO_DISCOVERY_EXCLUDES));
    });

    test('returns empty when include is explicit and no excludes', () => {
      const result = resolveExclude({
        include: ['src/**/*.ts'],
      });
      expect(result).toEqual([]);
    });

    test('deduplicates auto-discovery excludes with user excludes', () => {
      const result = resolveExclude({
        exclude: ['node_modules/**'],
      });
      const nodeModulesCount = result.filter(p => p === 'node_modules/**').length;
      expect(nodeModulesCount).toBe(1);
    });
  });
});
