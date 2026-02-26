import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import rollup from 'rollup';
import type { UnpluginBuildContext, UnpluginContext } from 'unplugin';

import unplugin from '../src';
import stylexPlugin from '../src/rollup';

describe('@stylexswc/unplugin', () => {
  test('ignores files without StyleX imports', async () => {
    const plugin = unplugin.raw({}, { framework: 'rollup' });
    const pluginInstance = Array.isArray(plugin) ? plugin[0] : plugin;

    if (!pluginInstance) {
      throw new Error('Plugin instance is undefined');
    }

    const mockContext: Partial<UnpluginBuildContext & UnpluginContext> = {
      addWatchFile: () => {},
      emitFile: () => '',
      getWatchFiles: () => [],
      parse: () => ({}) as ReturnType<UnpluginBuildContext['parse']>,
      error: () => {},
      warn: () => {},
    };

    if (typeof pluginInstance.buildStart === 'function') {
      pluginInstance.buildStart.call(mockContext as UnpluginBuildContext);
    }

    if (typeof pluginInstance.transform === 'function') {
      const result = await pluginInstance.transform.call(
        mockContext as UnpluginBuildContext & UnpluginContext,
        'const noop = 1;',
        '/virtual/foo.js'
      );
      expect(result).toBeNull();
    } else {
      throw new Error('Transform is not a function');
    }
  });

  test('writes fallback CSS asset when no CSS bundle entry exists', async () => {
    const tempDir = fs.mkdtempSync(
      path.join(os.tmpdir(), 'stylex-unplugin-test-'),
    );

    const inputFile = path.join(tempDir, 'input.js');
    const source = `
      import * as stylex from '@stylexjs/stylex';
      const styles = stylex.create({ foo: { color: 'red' } });
      export default styles;
    `;
    fs.writeFileSync(inputFile, source);

    try {
      const bundle = await rollup.rollup({
        input: inputFile,
        external: ['@stylexjs/stylex'],
        plugins: [
          stylexPlugin({
            rsOptions: {
              runtimeInjection: false,
              dev: false,
            },
          }),
        ],
      });

      const { output } = await bundle.generate({
        format: 'esm',
        dir: tempDir,
      });

      let cssAsset;
      for (const chunkOrAsset of output) {
        if (chunkOrAsset.type === 'asset' && chunkOrAsset.fileName.endsWith('.css')) {
          cssAsset = chunkOrAsset;
          break;
        }
      }

      expect(cssAsset).toBeDefined();
      expect(cssAsset?.source).toContain('color:red');
      const cssContent = cssAsset?.source.toString().trim();
      // CSS should be in compact format like .x1e2nbdu{color:red}
      expect(cssContent).toMatch(/^\.[a-z0-9]+\{color:red\}$/i);
    } finally {
      fs.rmSync(tempDir, { recursive: true, force: true });
    }
  });

  test('transform error includes the file path and preserves cause', async () => {
    const plugin = unplugin.raw({}, { framework: 'rollup' });
    const pluginInstance = Array.isArray(plugin) ? plugin[0] : plugin;

    if (!pluginInstance) {
      throw new Error('Plugin instance is undefined');
    }

    let capturedError: Error | string | undefined;
    const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {});
    const mockContext: Partial<UnpluginBuildContext & UnpluginContext> = {
      addWatchFile: () => {},
      emitFile: () => '',
      getWatchFiles: () => [],
      parse: () => ({}) as ReturnType<UnpluginBuildContext['parse']>,
      error: (msg: unknown) => {
        capturedError = msg as Error | string;
      },
      warn: () => {},
    };

    if (typeof pluginInstance.buildStart === 'function') {
      pluginInstance.buildStart.call(mockContext as UnpluginBuildContext);
    }

    // This code uses stylex.create with a non-static value that will cause
    // the rs-compiler to fail during transformation
    const badCode = `
      import * as stylex from '@stylexjs/stylex';
      const val = globalThis.dynamic;
      const styles = stylex.create({ root: { color: val.nested.deep() } });
    `;
    const filePath = '/path/to/MyComponent.tsx';

    if (typeof pluginInstance.transform === 'function') {
      await pluginInstance.transform.call(
        mockContext as UnpluginBuildContext & UnpluginContext,
        badCode,
        filePath,
      );
    }

    consoleSpy.mockRestore();

    expect(capturedError).toBeDefined();
    expect(capturedError).toBeInstanceOf(Error);
    const errorMessage = (capturedError as Error).message;
    expect(errorMessage).toContain(filePath);
    expect(errorMessage).toContain('StyleX transformation error');

    // Original error should be preserved as the cause
    expect((capturedError as Error).cause).toBeDefined();
  });
});
