import path from 'path';
import postcss from 'postcss';
import createPlugin from '../src/plugin';

describe('@stylexswc/postcss-plugin', () => {
  const fixturesDir = path.resolve(__dirname, '__fixtures__');

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
        // importSources: ['custom'],
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
});
