import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';

import * as rollup from 'rollup';
import type { UnpluginBuildContext, UnpluginContext, UnpluginContextMeta } from 'unplugin';
import { vi, describe, expect, test } from 'vitest';

import unplugin from '../src';
import stylexPlugin from '../src/rollup';
import type { UnpluginStylexRSOptions } from '../src/types';

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

async function runWebpackLikeCssInjection(
  framework: 'webpack' | 'rspack',
  initialAssets: Record<string, string> = { 'app.css': 'body{margin:0}\n@stylex;' },
  extraOptions: UnpluginStylexRSOptions = {},
  // Skipped to reach a build that carries the marker but produced no rules,
  // which still has to leave the marker out of the output.
  collectRules = true
) {
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
      ...extraOptions,
    },
    { framework } as UnpluginContextMeta
  );
  const pluginInstance = (Array.isArray(plugin) ? plugin[0] : plugin) as TestPluginInstance;

  if (!pluginInstance) {
    throw new Error('Plugin instance is undefined');
  }

  if (collectRules) {
    await collectStyleXRules(pluginInstance);
  }

  type MockAssets = Record<string, ReturnType<typeof createMockCssAsset>>;

  let processAssetsCallback: ((assets: MockAssets) => Promise<void>) | undefined;
  const assets: MockAssets = Object.fromEntries(
    Object.entries(initialAssets).map(([fileName, source]) => [
      fileName,
      createMockCssAsset(source),
    ])
  );
  const compilation = {
    hooks: {
      processAssets: {
        tapPromise: vi.fn((_options: unknown, callback: (assets: MockAssets) => Promise<void>) => {
          processAssetsCallback = callback;
        }),
      },
    },
    updateAsset: vi.fn((fileName: string, source: ReturnType<typeof createMockCssAsset>) => {
      assets[fileName] = source;
    }),
    emitAsset: vi.fn(),
    warnings: [] as Error[],
  };
  const compiler = {
    webpack: {
      Compilation: {
        PROCESS_ASSETS_STAGE_OPTIMIZE_SIZE: 0,
      },
      WebpackError: class WebpackError extends Error {},
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
    const plugin = unplugin.raw({}, { framework: 'rollup', versions: {} });
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
      await pluginInstance.buildStart.call(mockContext as UnpluginBuildContext);
    }

    expect(typeof pluginInstance.transform).toBe('function');

    const transform = pluginInstance.transform as Extract<
      typeof pluginInstance.transform,
      (...args: never[]) => unknown
    >;
    const result = await transform.call(
      mockContext as UnpluginBuildContext & UnpluginContext,
      'const noop = 1;',
      '/virtual/foo.js'
    );

    expect(result).toBeNull();
  });

  test('writes fallback CSS asset when no CSS bundle entry exists', async () => {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'stylex-unplugin-test-'));
    let bundle: rollup.RollupBuild | undefined;

    const inputFile = path.join(tempDir, 'input.js');
    fs.writeFileSync(inputFile, stylexSource);

    try {
      bundle = await rollup.rollup({
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
      await bundle?.close();
      fs.rmSync(tempDir, { recursive: true, force: true });
    }
  });

  test('webpack hook transforms StyleX CSS before placeholder injection', async () => {
    const { assets, compilation, transformCss } = await runWebpackLikeCssInjection('webpack');
    const finalCSS = assets['app.css']?.source().toString();

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
    const finalCSS = assets['app.css']?.source().toString();

    expect(transformCss).toHaveBeenCalledTimes(1);
    expect(transformCss.mock.calls[0]?.[1]).toBe('app.css');
    expect(compilation.updateAsset).toHaveBeenCalledTimes(1);
    expect(finalCSS).toContain('body{margin:0}');
    expect(finalCSS).toContain('color:red');
    expect(finalCSS).toContain('/* transformed:rspack:app.css */');
    expect(finalCSS).not.toContain('@stylex;');
  });

  test('webpack warns instead of emitting a stylesheet nothing links', async () => {
    const { assets, compilation } = await runWebpackLikeCssInjection('webpack', {});

    // Placeholder mode never links an emitted file, so emitting one here would
    // only hide the fact that the styles cannot be delivered.
    expect(compilation.emitAsset).not.toHaveBeenCalled();
    expect(Object.keys(assets)).toEqual([]);
    expect(compilation.warnings).toHaveLength(1);
    expect(compilation.warnings[0]?.message).toContain('no CSS asset contained the placeholder');
  });

  test('rspack warns instead of emitting a stylesheet nothing links', async () => {
    const { assets, compilation } = await runWebpackLikeCssInjection('rspack', {});

    expect(compilation.emitAsset).not.toHaveBeenCalled();
    expect(Object.keys(assets)).toEqual([]);
    expect(compilation.warnings).toHaveLength(1);
  });

  test('webpack stays silent about a missing target on request', async () => {
    const { compilation } = await runWebpackLikeCssInjection(
      'webpack',
      {},
      {
        onMissingCssPlaceholder: 'ignore',
      }
    );

    expect(compilation.warnings).toEqual([]);
  });

  test('webpack does not warn when the marker was replaced', async () => {
    const { compilation } = await runWebpackLikeCssInjection('webpack');

    expect(compilation.warnings).toEqual([]);
  });

  // A marker the rules never replaced is invalid CSS the browser is handed for
  // nothing, so the cleanup cannot depend on there being rules to inject.
  test.each(['webpack', 'rspack'] as const)(
    '%s leaves no marker behind when the build has no StyleX rules',
    async framework => {
      const { assets, compilation } = await runWebpackLikeCssInjection(
        framework,
        { 'app.css': 'body{margin:0}\n@stylex;' },
        {},
        false
      );

      expect(assets['app.css']?.source().toString()).toBe('body{margin:0}\n');
      // Nothing went missing, so there is nothing to report either.
      expect(compilation.warnings).toEqual([]);
    }
  );

  test.each(['webpack', 'rspack'] as const)(
    '%s strips the marker from a stylesheet that did not receive the rules',
    async framework => {
      const { assets } = await runWebpackLikeCssInjection(framework, {
        'app.css': 'body{margin:0}\n@stylex;',
        'other.css': '.other{outline:0}\n@stylex;',
      });

      expect(assets['app.css']?.source().toString()).toContain('color:red');
      // The rules belong in one stylesheet; a second copy would only duplicate
      // them, so the other marker is removed rather than filled.
      expect(assets['other.css']?.source().toString()).toBe('.other{outline:0}\n');
    }
  );

  test('warns that Farm does not support placeholder mode', () => {
    const warn = vi.spyOn(console, 'warn').mockImplementation(() => {});

    try {
      unplugin.raw({ useCssPlaceholder: true }, { framework: 'farm' } as UnpluginContextMeta);

      expect(warn).toHaveBeenCalledWith(expect.stringContaining('not supported under Farm'));
    } finally {
      warn.mockRestore();
    }
  });

  test('transform error includes the file path and preserves cause', async () => {
    const plugin = unplugin.raw({}, { framework: 'rollup', versions: {} });
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
      await pluginInstance.buildStart.call(mockContext as UnpluginBuildContext);
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
