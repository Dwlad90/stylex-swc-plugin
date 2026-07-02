import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import * as rollup from 'rollup';
import type { UnpluginBuildContext, UnpluginContext, UnpluginContextMeta } from 'unplugin';
import { vi, describe, expect, test } from 'vitest';

import unplugin from '../src';
import stylexPlugin from '../src/rollup';

type TestPluginInstance = {
  buildStart?: (this: UnpluginBuildContext) => void;
  transform?: (this: UnpluginBuildContext & UnpluginContext, code: string, id: string) => unknown;
  webpack?: (compiler: unknown) => void;
  rspack?: (compiler: unknown) => void;
};

const stylexSource = `
  import * as stylex from '@stylexjs/stylex';
  const styles = stylex.create({ foo: { color: 'red' } });
  export default styles;
`;

function createMockContext(): Partial<UnpluginBuildContext & UnpluginContext> {
  return {
    addWatchFile: () => {},
    emitFile: () => '',
    getWatchFiles: () => [],
    parse: () => ({}) as ReturnType<UnpluginBuildContext['parse']>,
    error: () => {},
    warn: () => {},
  };
}

function createMockCssAsset(source: string) {
  return {
    source: () => ({
      toString: () => source,
    }),
  };
}

async function collectStyleXRules(pluginInstance: TestPluginInstance) {
  const mockContext = createMockContext();

  if (typeof pluginInstance.buildStart === 'function') {
    pluginInstance.buildStart.call(mockContext as UnpluginBuildContext);
  }

  if (typeof pluginInstance.transform !== 'function') {
    throw new Error('Transform is not a function');
  }

  await pluginInstance.transform.call(
    mockContext as UnpluginBuildContext & UnpluginContext,
    stylexSource,
    '/virtual/foo.js'
  );
}

async function runWebpackLikeCssInjection(framework: 'webpack' | 'rspack') {
  const transformCss = vi.fn(async (css: string, filePath: string | undefined) => {
    return `${css}\n/* transformed:${framework}:${filePath} */`;
  });
  const plugin = unplugin.raw(
    {
      useCssPlaceholder: true,
      transformCss,
      rsOptions: {
        runtimeInjection: false,
        dev: false,
      },
    },
    { framework } as UnpluginContextMeta
  );
  const pluginInstance = (Array.isArray(plugin) ? plugin[0] : plugin) as TestPluginInstance;

  if (!pluginInstance) {
    throw new Error('Plugin instance is undefined');
  }

  await collectStyleXRules(pluginInstance);

  type MockAssets = Record<string, ReturnType<typeof createMockCssAsset>>;
  let processAssetsCallback: ((assets: MockAssets) => Promise<void>) | undefined;
  const assets = {
    'app.css': createMockCssAsset('body{margin:0}\n@stylex;'),
  };
  const compilation = {
    hooks: {
      processAssets: {
        tapPromise: vi.fn((_options: unknown, callback: (assets: MockAssets) => Promise<void>) => {
          processAssetsCallback = callback;
        }),
      },
    },
    updateAsset: vi.fn((fileName: string, source: ReturnType<typeof createMockCssAsset>) => {
      assets[fileName as keyof typeof assets] = source;
    }),
    emitAsset: vi.fn(),
  };
  const compiler = {
    webpack: {
      Compilation: {
        PROCESS_ASSETS_STAGE_OPTIMIZE_SIZE: 0,
      },
      sources: {
        RawSource: class RawSource {
          #source: string;

          constructor(source: string) {
            this.#source = source;
          }

          source() {
            return {
              toString: () => this.#source,
            };
          }
        },
      },
    },
    hooks: {
      thisCompilation: {
        tap: vi.fn((_name: string, callback: (compilation: unknown) => void) =>
          callback(compilation)
        ),
      },
    },
  };

  const applyBundlerHook = pluginInstance[framework];

  if (typeof applyBundlerHook !== 'function') {
    throw new Error(`${framework} hook is not a function`);
  }

  applyBundlerHook(compiler);
  await processAssetsCallback?.(assets);

  return { assets, compilation, transformCss };
}

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
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'stylex-unplugin-test-'));

    const inputFile = path.join(tempDir, 'input.js');
    fs.writeFileSync(inputFile, stylexSource);

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

      let cssAsset, jsCode;
      for (const chunkOrAsset of output) {
        if (chunkOrAsset.type === 'asset' && chunkOrAsset.fileName.endsWith('.css')) {
          cssAsset = chunkOrAsset;
          break;
        } else if (chunkOrAsset.fileName.endsWith('input.js')) {
          jsCode = (chunkOrAsset as rollup.OutputChunk).code;
        }
      }

      expect(cssAsset).toBeDefined();
      expect(cssAsset?.source).toContain('color:red');
      const cssContent = cssAsset?.source.toString().trim();
      // CSS should be in compact format like .x1e2nbdu{color:red}
      expect(cssContent).toMatchSnapshot();
      expect(jsCode).toMatchSnapshot();
    } finally {
      fs.rmSync(tempDir, { recursive: true, force: true });
    }
  });

  test('webpack hook transforms StyleX CSS before placeholder injection', async () => {
    const { assets, compilation, transformCss } = await runWebpackLikeCssInjection('webpack');
    const finalCSS = assets['app.css'].source().toString();

    expect(transformCss).toHaveBeenCalledTimes(1);
    expect(transformCss.mock.calls[0]?.[1]).toBe('app.css');
    expect(compilation.updateAsset).toHaveBeenCalledTimes(1);
    expect(finalCSS).toContain('body{margin:0}');
    expect(finalCSS).toContain('color:red');
    expect(finalCSS).toContain('/* transformed:webpack:app.css */');
    expect(finalCSS).not.toContain('@stylex;');
  });

  test('rspack hook transforms StyleX CSS before placeholder injection', async () => {
    const { assets, compilation, transformCss } = await runWebpackLikeCssInjection('rspack');
    const finalCSS = assets['app.css'].source().toString();

    expect(transformCss).toHaveBeenCalledTimes(1);
    expect(transformCss.mock.calls[0]?.[1]).toBe('app.css');
    expect(compilation.updateAsset).toHaveBeenCalledTimes(1);
    expect(finalCSS).toContain('body{margin:0}');
    expect(finalCSS).toContain('color:red');
    expect(finalCSS).toContain('/* transformed:rspack:app.css */');
    expect(finalCSS).not.toContain('@stylex;');
  });

  test('transform error includes the file path and preserves cause', async () => {
    const plugin = unplugin.raw({}, { framework: 'rollup' });
    const pluginInstance = Array.isArray(plugin) ? plugin[0] : plugin;

    if (!pluginInstance) {
      throw new Error('Plugin instance is undefined');
    }

    let capturedError: Error | string | undefined;
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
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
        filePath
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
