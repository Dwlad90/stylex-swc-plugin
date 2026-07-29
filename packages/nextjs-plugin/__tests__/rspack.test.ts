import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest';

import withStyleX from '../src/rspack';

import type { NextConfig, WebpackConfigContext } from 'next/dist/server/config-shared';
import type { StyleXNextRspackPluginOption } from '../src/rspack';
import type webpack from 'webpack';

const { rspackPluginOptions, warn } = vi.hoisted(() => ({
  rspackPluginOptions: [] as Array<Record<string, unknown>>,
  warn: vi.fn(),
}));

vi.mock('next/dist/build/output/log', () => ({ warn }));
vi.mock('next-rspack', () => ({
  default: (config: NextConfig) => config,
}));
vi.mock('@stylexswc/rspack-plugin', () => ({
  default: class StyleXRspackPlugin {
    constructor(options: Record<string, unknown>) {
      rspackPluginOptions.push(options);
    }
  },
  DEFAULT_STYLEX_PACKAGES: ['@stylexjs/stylex'],
  buildVirtualCssPattern: vi.fn(),
}));

const originalNextRspack = process.env.NEXT_RSPACK;
const temporaryDirectories: string[] = [];

const createProject = (entry: string): string => {
  const projectDirectory = mkdtempSync(path.join(tmpdir(), 'stylex-nextjs-plugin-'));
  const entryPath = path.join(projectDirectory, entry);

  temporaryDirectories.push(projectDirectory);
  mkdirSync(path.dirname(entryPath), { recursive: true });
  writeFileSync(entryPath, '');

  return projectDirectory;
};

const createContext = (
  dir: string,
  overrides: Partial<Pick<WebpackConfigContext, 'dev' | 'isServer' | 'nextRuntime'>> = {},
  pageExtensions = ['tsx', 'ts', 'jsx', 'js']
): WebpackConfigContext =>
  ({
    buildId: 'test-build',
    config: {
      pageExtensions,
    },
    defaultLoaders: {
      babel: {},
    },
    dev: false,
    dir,
    isServer: true,
    totalPages: 0,
    webpack: {},
    ...overrides,
  }) as WebpackConfigContext;

const createRspackConfigRunner = (
  nextConfig: NextConfig = {},
  pluginOptions: StyleXNextRspackPluginOption = { extractCSS: false }
): ((context: WebpackConfigContext) => webpack.Configuration) => {
  const wrappedConfig = withStyleX({
    extractCSS: false,
    ...pluginOptions,
  })(nextConfig);

  const configureWebpack = wrappedConfig.webpack;

  if (typeof configureWebpack !== 'function') {
    throw new TypeError('Expected the Rspack wrapper to provide a webpack configuration hook.');
  }

  return context =>
    configureWebpack(
      {
        experiments: {
          cache: {
            type: 'persistent',
          },
        },
      },
      context
    ) as webpack.Configuration;
};

const applyRspackConfig = (
  nextConfig: NextConfig,
  context: WebpackConfigContext,
  pluginOptions?: StyleXNextRspackPluginOption
): webpack.Configuration => createRspackConfigRunner(nextConfig, pluginOptions)(context);

beforeEach(() => {
  process.env.NEXT_RSPACK = 'true';
  rspackPluginOptions.splice(0);
  warn.mockClear();
});

afterEach(() => {
  for (const directory of temporaryDirectories.splice(0)) {
    rmSync(directory, { force: true, recursive: true });
  }

  if (originalNextRspack === undefined) {
    delete process.env.NEXT_RSPACK;
  } else {
    process.env.NEXT_RSPACK = originalNextRspack;
  }
});

describe('@stylexswc/nextjs-plugin/rspack persistent cache', () => {
  test('uses the normalized Next.js page extensions when detecting a proxy entry', () => {
    const projectDirectory = createProject('proxy.custom');
    const config = applyRspackConfig(
      {},
      createContext(projectDirectory, { isServer: true }, ['custom'])
    );

    expect(config.experiments?.cache).toBe(false);
    expect(warn).toHaveBeenCalledOnce();
  });

  test('does not forward wrapper-only cache options to the Rspack plugin', () => {
    const projectDirectory = createProject('proxy.ts');

    applyRspackConfig(
      {
        pageExtensions: ['ts'],
      },
      createContext(projectDirectory),
      {
        rspackServerPersistentCache: false,
      }
    );

    expect(rspackPluginOptions).toHaveLength(1);
    expect(rspackPluginOptions[0]).not.toHaveProperty('rspackServerPersistentCache');
  });

  test('keeps the persistent cache for production client compilers', () => {
    const projectDirectory = createProject('proxy.ts');
    const config = applyRspackConfig(
      {
        pageExtensions: ['ts'],
      },
      createContext(projectDirectory, { isServer: false })
    );

    expect(config.experiments?.cache).toEqual({ type: 'persistent' });
    expect(warn).not.toHaveBeenCalled();
  });

  test('keeps the persistent cache for development server compilers by default', () => {
    const projectDirectory = createProject('proxy.ts');
    const config = applyRspackConfig(
      {
        pageExtensions: ['ts'],
      },
      createContext(projectDirectory, { dev: true })
    );

    expect(config.experiments?.cache).toEqual({ type: 'persistent' });
    expect(warn).not.toHaveBeenCalled();
  });

  test('keeps the persistent cache when explicitly enabled', () => {
    const projectDirectory = createProject('proxy.ts');
    const config = applyRspackConfig(
      {
        pageExtensions: ['ts'],
      },
      createContext(projectDirectory),
      {
        rspackServerPersistentCache: true,
      }
    );

    expect(config.experiments?.cache).toEqual({ type: 'persistent' });
    expect(warn).not.toHaveBeenCalled();
  });

  test('disables the persistent cache in development when explicitly disabled', () => {
    const projectDirectory = createProject('proxy.ts');
    const config = applyRspackConfig(
      {
        pageExtensions: ['ts'],
      },
      createContext(projectDirectory, { dev: true }),
      {
        rspackServerPersistentCache: false,
      }
    );

    expect(config.experiments?.cache).toBe(false);
    expect(warn).toHaveBeenCalledOnce();
  });

  test('keeps the persistent cache when no proxy or middleware entry exists', () => {
    const projectDirectory = createProject('app.ts');
    const config = applyRspackConfig(
      {
        pageExtensions: ['ts'],
      },
      createContext(projectDirectory)
    );

    expect(config.experiments?.cache).toEqual({ type: 'persistent' });
    expect(warn).not.toHaveBeenCalled();
  });

  test('warns once when both server compilers use the same wrapped config', () => {
    const projectDirectory = createProject(path.join('src', 'middleware.ts'));
    const runRspackConfig = createRspackConfigRunner({
      pageExtensions: ['ts'],
    });

    runRspackConfig(createContext(projectDirectory, { nextRuntime: 'nodejs' }));
    runRspackConfig(createContext(projectDirectory, { nextRuntime: 'edge' }));

    expect(warn).toHaveBeenCalledOnce();
    expect(warn).toHaveBeenCalledWith(expect.stringContaining('src/middleware.ts'));
  });
});
