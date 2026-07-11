import fs from 'fs';
import path from 'path';
import { describe, expect, test, vi } from 'vitest';

import { BUILD_INFO_STYLEX_KEY, LOADER_TRANSFORMED_FLAG } from '../src/constants';
import stylexLoader from '../src/stylex-loader';
import stylexVirtualCssLoader from '../src/stylex-virtual-css-loader';

import type { StyleXLoaderOptions } from '../src/types';
import type { LoaderContext } from 'webpack';

const FIXTURE_PATH = path.join(__dirname, '__fixtures__', 'Button.jsx');
const ROOT_CONTEXT = path.join(__dirname, '..');

type LoaderResult = {
  error: Error | null | undefined;
  code: string | Buffer | undefined;
  map: unknown;
};

function createStylexLoaderContext(options?: Partial<StyleXLoaderOptions>) {
  const result: LoaderResult = { error: undefined, code: undefined, map: undefined };
  const buildInfo: Record<string, unknown> = {};

  const context = {
    resourcePath: FIXTURE_PATH,
    rootContext: ROOT_CONTEXT,
    context: path.dirname(FIXTURE_PATH),
    getOptions: () => ({
      stylexImports: ['stylex', '@stylexjs/stylex'],
      rsOptions: { dev: false, runtimeInjection: false },
      nextjsMode: false,
      nextjsAppRouterMode: false,
      extractCSS: true,
      ...options,
    }),
    async:
      () =>
      (error: Error | null | undefined, code?: string | Buffer, map?: unknown) => {
        result.error = error;
        result.code = code;
        result.map = map;
      },
    utils: {
      // emulates webpack's contextify: absolute requests become relative
      contextify: (contextDir: string, request: string) => {
        const [file = '', query] = request.split('?');
        const relative = path.relative(contextDir, file);
        const prefixed = relative.startsWith('.') ? relative : `./${relative}`;

        return query == null ? prefixed : `${prefixed}?${query}`;
      },
    },
    _module: { buildInfo },
    _compiler: {
      options: { mode: 'development' },
      getInfrastructureLogger: () => ({ warn: vi.fn(), debug: vi.fn() }),
    },
  };

  return { context: context as unknown as LoaderContext<StyleXLoaderOptions>, result, buildInfo };
}

describe('stylex-loader', () => {
  const fixtureSource = fs.readFileSync(FIXTURE_PATH, 'utf8');

  test('appends a relative dummy import, stamps buildInfo and the transformed flag', async () => {
    const { context, result, buildInfo } = createStylexLoaderContext();

    await stylexLoader.call(context, fixtureSource, undefined);

    expect(result.error).toBeFalsy();
    const output = String(result.code);

    // transformed flag guards against double transformation
    expect(output).toContain(LOADER_TRANSFORMED_FLAG);

    // dummy import carries the rules and a context-relative `from`
    const importMatch = output.match(/import "([^"]+stylex-virtual\.css[^"]*)";/);
    expect(importMatch).not.toBeNull();

    const query = new URLSearchParams(importMatch![1]!.split('?')[1]);
    const from = query.get('from');

    expect(from).toBe(path.relative(ROOT_CONTEXT, FIXTURE_PATH));
    expect(path.isAbsolute(from!)).toBe(false);
    expect(JSON.parse(query.get('stylex')!)).not.toHaveLength(0);

    // request itself is contextified (relative), never absolute
    expect(path.isAbsolute(importMatch![1]!)).toBe(false);

    // webpack rule transport: rules stored on module buildInfo
    expect(buildInfo[BUILD_INFO_STYLEX_KEY]).toMatchObject({ resourcePath: FIXTURE_PATH });
  });

  test('bails out when the input was already transformed', async () => {
    const first = createStylexLoaderContext();
    await stylexLoader.call(first.context, fixtureSource, undefined);
    const transformed = String(first.result.code);

    const second = createStylexLoaderContext();
    await stylexLoader.call(second.context, transformed, undefined);

    expect(second.result.code).toBe(transformed);
    expect(second.buildInfo[BUILD_INFO_STYLEX_KEY]).toBeUndefined();
  });

  test('bails out when the input has no stylex imports', async () => {
    const { context, result, buildInfo } = createStylexLoaderContext();
    const source = 'export const answer = 42;\n';

    await stylexLoader.call(context, source, undefined);

    expect(result.code).toBe(source);
    expect(buildInfo[BUILD_INFO_STYLEX_KEY]).toBeUndefined();
  });

  test('skips the dummy import when extractCSS is disabled', async () => {
    const { context, result } = createStylexLoaderContext({ extractCSS: false });

    await stylexLoader.call(context, fixtureSource, undefined);

    expect(String(result.code)).not.toContain('stylex-virtual.css');
    expect(String(result.code)).not.toContain(LOADER_TRANSFORMED_FLAG);
  });
});

function createVirtualCssLoaderContext(mode: 'development' | 'production', resourceQuery: string) {
  const result: LoaderResult = { error: undefined, code: undefined, map: undefined };
  const cacheable = vi.fn();

  const context = {
    resourceQuery,
    cacheable,
    async:
      () =>
      (error: Error | null | undefined, code?: string | Buffer, map?: unknown) => {
        result.error = error;
        result.code = code;
        result.map = map;
      },
    _compiler: { options: { mode } },
  };

  return { context: context as unknown as LoaderContext<unknown>, result, cacheable };
}

describe('stylex-virtual-css-loader', () => {
  const stylexQuery = `?${new URLSearchParams({
    from: 'app/Button.jsx',
    stylex: JSON.stringify([['x1abcd', { ltr: '.x1abcd{color:red}', rtl: null }, 3000]]),
  }).toString()}`;

  test('appends a content-hashed dummy rule in development', () => {
    const { context, result, cacheable } = createVirtualCssLoaderContext(
      'development',
      stylexQuery
    );

    stylexVirtualCssLoader.call(context, '/* dummy */', undefined);

    expect(cacheable).toHaveBeenCalledWith(false);
    expect(String(result.code)).toMatch(/\.stylex-hashed-[A-Za-z0-9]+ \{\}/);
  });

  test('passes through in production', () => {
    const { context, result } = createVirtualCssLoaderContext('production', stylexQuery);

    stylexVirtualCssLoader.call(context, '/* dummy */', undefined);

    expect(result.code).toBe('/* dummy */');
  });

  test('passes through without a stylex query', () => {
    const { context, result } = createVirtualCssLoaderContext('development', '?foo=bar');

    stylexVirtualCssLoader.call(context, '/* dummy */', undefined);

    expect(result.code).toBe('/* dummy */');
  });
});
