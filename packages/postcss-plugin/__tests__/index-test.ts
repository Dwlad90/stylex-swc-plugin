import fs from 'node:fs';
import os from 'node:os';
import path from 'path';
import postcss from 'postcss';
import createPlugin from '../src/plugin';
import {
  resolveImportSourcesWithMetadata,
  resolveIncludeWithMetadata,
  resolveExclude,
  AUTO_DISCOVERY_EXCLUDES,
  DEFAULT_IMPORT_SOURCES,
  DEFAULT_INCLUDE_GLOB,
} from '../src/discovery';

describe('@stylexswc/postcss-plugin', () => {
  const fixturesDir = path.resolve(__dirname, '__fixtures__');
  const autoDiscoveryFixturesDir = path.resolve(__dirname, '__auto_discovery_fixtures__');

  /**
   * Creates a temporary copy of the auto-discovery fixtures directory and
   * sets up mock node_modules (stylex-custom-lib, non-stylex-lib, .cache).
   * Returns the temp directory path; caller must clean up with fs.rmSync.
   */
  function createAutoDiscoveryFixture(): string {
    const tempDir = fs.mkdtempSync(
      path.join(os.tmpdir(), 'stylex-postcss-auto-discovery-')
    );

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
    options:
      | Record<string, unknown>
      | ((fixtureDir: string) => Record<string, unknown>) = {},
    inputCSS = '@stylex;'
  ): Promise<{ css: string; messages: postcss.Message[] }> {
    const fixtureDir = createAutoDiscoveryFixture();
    const stylexPostcssPlugin = createPlugin();
    const resolvedOptions =
      typeof options === 'function' ? options(fixtureDir) : options;

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
    } as Parameters<ReturnType<typeof createPlugin>>[0]);

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
    expect(result.messages.length).toBe(0);
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
    const readFileSpy = jest.spyOn(fs, 'readFileSync');

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
          message.type === 'dir-dependency' &&
          String(message.dir).includes('stylex-custom-lib')
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
          message.type === 'dir-dependency' &&
          String(message.dir).includes('stylex-custom-lib')
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
    const infoSpy = jest.spyOn(console, 'info').mockImplementation(() => {});

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
      expect(result).toEqual(
        expect.arrayContaining([...AUTO_DISCOVERY_EXCLUDES, '**/*.test.ts'])
      );
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
