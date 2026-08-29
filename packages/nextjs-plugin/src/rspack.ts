import { existsSync } from 'node:fs';
import path from 'node:path';

import { exportAsCommonJs } from '@stylexswc/plugin-shared/cjs-interop';
import { INCLUDE_EXTENSIONS } from '@stylexswc/plugin-shared/constants';
import StyleXRspackPlugin, {
  DEFAULT_STYLEX_PACKAGES,
  buildVirtualCssPattern,
} from '@stylexswc/rspack-plugin';
import type { StyleXPluginOption } from '@stylexswc/rspack-plugin';
import withRspack from 'next-rspack';
import { warn } from 'next/dist/build/output/log';
import { lazyPostCSS } from 'next/dist/build/webpack/config/blocks/css';
import type { ConfigurationContext as WebpackConfigurationContext } from 'next/dist/build/webpack/config/utils';
import browserslist from 'next/dist/compiled/browserslist';
import type { NextConfig, WebpackConfigContext } from 'next/dist/server/config-shared';
import { getRspackCore } from 'next/dist/shared/lib/get-rspack';
import type { Processor as PostCSSProcessor } from 'postcss';
import type webpack from 'webpack';

/** Rspack-only; absent from the webpack `Configuration` type Next.js hands us */
type RspackPersistentCache = false | { type: 'persistent' | 'memory'; [key: string]: unknown };

type RspackExperiments = NonNullable<webpack.Configuration['experiments']> & {
  cache?: RspackPersistentCache;
};

type RspackConfiguration = Omit<webpack.Configuration, 'experiments'> &
  WebpackConfigurationContext & {
    experiments?: RspackExperiments;
  };

export interface StyleXNextRspackPluginOption extends StyleXPluginOption {
  /**
   * Whether the server compilers keep Rspack's persistent cache.
   *
   * Next.js 16 turns on `experiments.cache = { type: 'persistent' }` for
   * every `next-rspack` compiler. With a `proxy.ts`/`middleware.ts` entry
   * that cache degrades catastrophically in the server compilers: the build
   * spends its time in Rspack's native filesystem layer walking the whole
   * workspace, and warm builds are slower than cold ones (measured on the
   * example app: 1.6s -> 27s cold, 50s warm; unbounded in large monorepos).
   *
   * Left unset, the cache is disabled for the server compilers of a
   * **production build** when a proxy/middleware entry is detected. `next dev`
   * is left alone: it shows no such stall, and keeping its cache is worth
   * ~200ms on the first compile of a route. Set `true` to keep Next.js'
   * setting untouched, `false` to always disable it (dev included).
   *
   * Auto-detection also stands down when your own `webpack()` hook configures
   * `experiments.cache` itself, so an explicit choice there always wins.
   *
   * @default undefined (auto-detect, production builds only)
   */
  rspackServerPersistentCache?: boolean;
}

/**
 * Next.js resolves the proxy/middleware entry from `<root>` or `<root>/src`
 * @see https://nextjs.org/docs/app/api-reference/file-conventions/proxy
 */
const NEXTJS_PROXY_FILENAMES = ['proxy', 'middleware'] as const;
const NEXTJS_PROXY_DIRS = ['', 'src'] as const;
/**
 * Extensions are interpolated into a probe path, so anything that could escape
 * the project directory is rejected. Anchored and single-class: no backtracking.
 */
const SAFE_EXTENSION_PATTERN = /^[A-Za-z0-9]+$/;

/** Warning text and test assertions must not vary with the platform separator */
const toPosixPath = (value: string): string => value.split(path.sep).join('/');

/**
 * Stable stringification of `experiments.cache`, used to tell whether a user's
 * `webpack()` hook configured it. Returns `undefined` when the field is absent,
 * which callers must treat as "no choice made" rather than as a change.
 */
const snapshotPersistentCache = (cache: RspackPersistentCache | undefined): string | undefined => {
  if (cache === undefined) {
    return undefined;
  }

  try {
    return JSON.stringify(cache);
  } catch {
    // Circular or non-serializable value: it came from somewhere other than
    // Next.js' plain defaults, so treat it as a deliberate configuration.
    return '[unserializable]';
  }
};

/**
 * Probes for Next.js' proxy/middleware entry with a bounded set of synchronous
 * `existsSync` calls — `webpack()` is a synchronous API, so this cannot be
 * deferred, and the result is memoized per config by the caller.
 *
 * @param dir - project root (`ctx.dir`)
 * @param pageExtensions - normalized `ctx.config.pageExtensions`, merged into
 *   the built-in superset; malformed values are skipped
 * @returns the entry's path relative to `dir` with POSIX separators, or
 *   `undefined` when the project has no proxy/middleware entry
 */
const findNextjsProxyEntry = (dir: string, pageExtensions?: string[]): string | undefined => {
  // Detection only gates a cache optimization, so a superset of Next.js'
  // `pageExtensions` is deliberate: a false positive costs build cache, a
  // false negative costs a hanging build. The shared list is that superset, and
  // it stays correct when a new extension joins it.
  const extensions = new Set(
    [...INCLUDE_EXTENSIONS, ...(pageExtensions ?? [])].filter(extension =>
      SAFE_EXTENSION_PATTERN.test(extension)
    )
  );

  for (const subDir of NEXTJS_PROXY_DIRS) {
    for (const filename of NEXTJS_PROXY_FILENAMES) {
      for (const extension of extensions) {
        const candidate = path.join(dir, subDir, `${filename}.${extension}`);

        if (existsSync(candidate)) {
          return toPosixPath(path.relative(dir, candidate));
        }
      }
    }
  }

  return undefined;
};

type CssExtractPluginClass = {
  new (_options: { filename: string; chunkFilename: string; ignoreOrder: boolean }): unknown;
  loader: string;
};

// Adopted from https://github.com/vercel/next.js/blob/1f1632979c78b3edfe59fd85d8cce62efcdee688/packages/next/build/webpack-config.ts#L60-L72
const getSupportedBrowsers = (dir: string, isDevelopment: boolean) => {
  try {
    return browserslist.loadConfig({
      path: dir,
      env: isDevelopment ? 'development' : 'production',
    });
  } catch {
    // Ignore: browserslist config is optional, fall back to the caller's default.
    return undefined;
  }
};

/**
 * Resolves CssExtractRspackPlugin from the exact module instance Next.js uses
 * (`next-rspack/rspack-core`), so the `instanceof` dedup check below cannot be
 * defeated by a second copy of @rspack/core
 */
const getCssExtractPlugin = (): CssExtractPluginClass => {
  const rspackCore = getRspackCore();

  const plugin = rspackCore.rspack?.CssExtractRspackPlugin ?? rspackCore.CssExtractRspackPlugin;

  if (!plugin) {
    throw new Error(
      '@stylexswc/nextjs-plugin/rspack: CssExtractRspackPlugin not found in next-rspack/rspack-core.'
    );
  }

  return plugin as CssExtractPluginClass;
};

// Adopt from Next.js' getGlobalCssLoader
// https://github.com/vercel/next.js/blob/d61b0761efae09bd9cb1201ff134ed8950d9deca/packages/next/src/build/webpack/config/blocks/css/loaders/global.ts#L7
function getStyleXVirtualCssLoader(
  ctx: WebpackConfigContext,
  cssExtractPlugin: CssExtractPluginClass,
  postcss: () => Promise<unknown>
) {
  const loaders: webpack.RuleSetUseItem[] = [];

  // Adopt from Next.js' getClientStyleLoader
  // https://github.com/vercel/next.js/blob/56d35ede8ed2ab25fa8e29583d4e81e3e76a0e29/packages/next/src/build/webpack/config/blocks/css/loaders/global.ts#L18
  if (!ctx.isServer) {
    loaders.push({
      loader: cssExtractPlugin.loader,
      options: {
        publicPath: `${ctx.config.assetPrefix}/_next/`,
        esModule: false,
      },
    });
  }

  // We don't actually use postcss-loader or css-loader to run against
  // the stylex css (which doesn't exist yet).
  // We use this loader to run against the virtual dummy css.
  loaders.push({
    // https://github.com/vercel/next.js/blob/0572e218afe130656be53f7367bf18c4ea389f7d/packages/next/build/webpack/config/blocks/css/loaders/global.ts#L29-L38
    loader: require.resolve('next/dist/build/webpack/loaders/css-loader/src'),
    options: {
      // https://github.com/vercel/next.js/blob/88a5f263f11cb55907f0d89a4cd53647ee8e96ac/packages/next/build/webpack/config/blocks/css/index.ts#L142-L147
      postcss,
      importLoaders: 1,
      modules: false,
    },
  });

  return loaders;
}

const withStyleX =
  (pluginOptions?: StyleXNextRspackPluginOption) =>
  (nextConfig: NextConfig = {}): NextConfig => {
    const { rspackServerPersistentCache, ...rspackPluginOptions } = pluginOptions ?? {};
    // Scoped per `withStyleX(...)` call rather than module-level, so it doesn't
    // leak across unrelated Next.js configs sharing this process (e.g. a
    // monorepo building multiple apps, or repeated calls in tests).
    let count = 0;
    // The server and edge-server compilers both run `webpack()`; the notice
    // belongs in the log once per config, not once per compiler.
    let persistentCacheNoticeShown = false;
    // Both server compilers use the same project root and normalized config, so
    // the bounded synchronous probe runs once. Keyed by its inputs so the memo
    // cannot hand back an answer computed for different arguments.
    let proxyEntryCacheKey: string | undefined;
    let detectedProxyEntry: string | undefined;
    const getNextjsProxyEntry = (dir: string, pageExtensions?: string[]) => {
      const cacheKey = JSON.stringify([dir, pageExtensions ?? []]);

      if (cacheKey !== proxyEntryCacheKey) {
        proxyEntryCacheKey = cacheKey;
        detectedProxyEntry = findNextjsProxyEntry(dir, pageExtensions);
      }

      return detectedProxyEntry;
    };

    // The App Router cross-compiler rule registry lives on `globalThis`, so
    // the client/server/edge-server compilers must share one build process.
    // Only enforced when the registry is in use — Pages Router builds
    // (`nextjsAppRouterMode: false`) keep the user's setting.
    const useAppRouterRegistry = rspackPluginOptions.nextjsAppRouterMode ?? true;

    if (useAppRouterRegistry && nextConfig.experimental?.webpackBuildWorker) {
      warn(
        '@stylexswc/nextjs-plugin/rspack: disabling "experimental.webpackBuildWorker" — the StyleX cross-compiler rule registry requires all compilers to run in a single process.'
      );
    }

    // `withRspack` switches Next.js to the Rspack bundler for this config
    // (sets NEXT_RSPACK); applied to the final config object so users don't
    // have to compose `next-rspack` themselves
    return withRspack({
      ...nextConfig,
      ...(useAppRouterRegistry
        ? {
            experimental: {
              ...nextConfig.experimental,
              webpackBuildWorker: false,
            },
          }
        : {}),
      webpack(config: RspackConfiguration, ctx: WebpackConfigContext) {
        if (!process.env.NEXT_RSPACK) {
          throw new Error(
            [
              '@stylexswc/nextjs-plugin/rspack requires Next.js to run with Rspack.',
              'Run `next dev`/`next build` without the `--webpack` or `--turbopack` flags',
              '(set NEXT_RSPACK=true for `next start`),',
              'or use `@stylexswc/nextjs-plugin` for the default webpack bundler.',
            ].join(' ')
          );
        }

        // Snapshotted around the user's hook so an `experiments.cache` they set
        // themselves wins over the auto-detected workaround below. Serialized
        // rather than compared by reference: hooks commonly mutate the existing
        // object in place, which reference equality would miss.
        const cacheBeforeUserHook = snapshotPersistentCache(config.experiments?.cache);

        if (typeof nextConfig.webpack === 'function') {
          config = nextConfig.webpack(config, ctx);
        }

        const cacheAfterUserHook = snapshotPersistentCache(config.experiments?.cache);
        // An absent value is not a choice: a hook that returns a fresh config
        // object without `experiments` must not disable the workaround.
        const userConfiguredCache =
          cacheAfterUserHook !== undefined && cacheAfterUserHook !== cacheBeforeUserHook;

        const { buildId, dev, isServer } = ctx;

        count += 1;

        const debugEnabled = Boolean(
          rspackPluginOptions.rsOptions?.debug || process.env.STYLEX_DEBUG
        );

        if (debugEnabled) {
          warn(
            `@stylexswc/nextjs-plugin/rspack: rspack config #${count} (buildId=${buildId}, server=${isServer}, env=${dev ? 'dev' : 'prod'})`
          );
        }

        // Upstream workaround: Next.js 16 enables Rspack's persistent cache for
        // every compiler, and a proxy/middleware entry makes it pathological in
        // the server compilers (see `rspackServerPersistentCache`). Scoped to
        // the server compilers so the client keeps its cache, and to production
        // builds — `next dev` shows no such stall, and keeping its cache is
        // worth ~200ms on first compile of a route.
        if (isServer && rspackServerPersistentCache !== true) {
          const autoDetect = !dev && rspackServerPersistentCache !== false && !userConfiguredCache;
          const proxyEntry = autoDetect
            ? getNextjsProxyEntry(ctx.dir, ctx.config.pageExtensions)
            : undefined;
          // An explicit `false` disables the cache everywhere, dev included, and
          // outranks a cache configured in the user's own `webpack()` hook
          const disableCache = rspackServerPersistentCache === false || proxyEntry != null;

          // Only meaningful where auto-detection would otherwise have run: in
          // dev the cache is left alone regardless of the user's hook
          if (
            debugEnabled &&
            !dev &&
            userConfiguredCache &&
            rspackServerPersistentCache === undefined
          ) {
            warn(
              [
                '@stylexswc/nextjs-plugin/rspack: leaving "experiments.cache" alone —',
                'it was configured by the "webpack" hook in your Next.js config.',
              ].join(' ')
            );
          }

          if (disableCache) {
            config.experiments ??= {};
            config.experiments.cache = false;

            if (!persistentCacheNoticeShown) {
              persistentCacheNoticeShown = true;
              warn(
                proxyEntry
                  ? [
                      "@stylexswc/nextjs-plugin/rspack: disabling Rspack's persistent cache for the",
                      `server compilers — "${proxyEntry}" makes it pathologically slow on Next.js 16`,
                      '(builds can appear to hang). Set "rspackServerPersistentCache: true" to keep it.',
                    ].join(' ')
                  : [
                      "@stylexswc/nextjs-plugin/rspack: disabling Rspack's persistent cache for the",
                      'server compilers — "rspackServerPersistentCache: false" was set.',
                    ].join(' ')
              );
            }
          }
        }

        config.optimization ||= {};
        config.optimization.splitChunks ||= {};
        config.optimization.splitChunks.cacheGroups ||= {};

        const extractCSS = rspackPluginOptions.extractCSS ?? true;

        // Resolved once and shared by the css rule test below and the plugin
        // (via `carrierCss`), so the two can never disagree about the carrier
        // location when `compiler.context` differs from the project dir
        const carrierPath = rspackPluginOptions.carrierCss
          ? path.resolve(ctx.dir, rspackPluginOptions.carrierCss)
          : require.resolve('@stylexswc/rspack-plugin/stylex.css');

        config.plugins ??= [];

        let lazyPostCSSPromise: Promise<{
          postcss: typeof import('postcss');
          postcssWithPlugins: PostCSSProcessor;
        }> | null = null;
        const postcss = () => {
          lazyPostCSSPromise ||= lazyPostCSS(
            ctx.dir,
            getSupportedBrowsers(ctx.dir, ctx.dev),
            nextConfig?.experimental?.disablePostcssPresetEnv,
            nextConfig?.experimental?.useLightningcss
          );
          return lazyPostCSSPromise;
        };

        if (extractCSS) {
          const CssExtractPlugin = getCssExtractPlugin();
          // Based on https://github.com/vercel/next.js/blob/88a5f263f11cb55907f0d89a4cd53647ee8e96ac/packages/next/build/webpack/config/helpers.ts#L12-L18
          const cssContainerRule = config.module?.rules?.find(
            rule =>
              typeof rule === 'object' &&
              rule !== null &&
              Array.isArray(rule.oneOf) &&
              rule.oneOf.some(
                setRule =>
                  setRule &&
                  setRule.test instanceof RegExp &&
                  typeof setRule.test.test === 'function' &&
                  setRule.test.test('filename.css')
              )
          ) as webpack.RuleSetRule | undefined;
          const cssRules = cssContainerRule?.oneOf;

          if (!cssRules) {
            throw new Error(
              [
                "@stylexswc/nextjs-plugin/rspack: could not find Next.js' css oneOf rules",
                'in the Rspack config. StyleX CSS extraction cannot be wired up —',
                'this likely indicates an incompatible Next.js version. Please report this issue.',
              ].join(' ')
            );
          }

          // Here we matches virtual css file emitted by StyleXPlugin
          // (carrier + HMR dummies; honors a custom `carrierCss` path)
          cssRules.unshift({
            test: buildVirtualCssPattern(carrierPath),
            use: getStyleXVirtualCssLoader(ctx, CssExtractPlugin, postcss),
          });

          // StyleX needs to emit the css file on both server and client, both during
          // the development and production.
          // However, Next.js only adds CssExtractRspackPlugin on the client.
          //
          // The instanceof check prevents multiple extract plugins from being added
          // (which would cause RealContentHashPlugin to panic)
          if (!config.plugins.some((plugin: unknown) => plugin instanceof CssExtractPlugin)) {
            // HMR reloads the CSS file when the content changes but does not use
            // the new file name, which means it can't contain a hash.
            const filename = ctx.dev ? 'static/css/[name].css' : 'static/css/[contenthash].css';

            config.plugins.push(
              new CssExtractPlugin({
                filename,
                chunkFilename: filename,
                // Next.js guarantees that CSS order "doesn't matter", due to imposed
                // restrictions:
                // 1. Global CSS can only be defined in a single entrypoint (_app)
                // 2. CSS Modules generate scoped class names by default and cannot
                //    include Global CSS (:global() selector).
                //
                // While not a perfect guarantee (e.g. liberal use of `:global()`
                // selector), this assumption is required to code-split CSS.
                //
                // As for StyleX, the CSS is always atomic (so classes are always unique),
                // and StyleX Plugin will always sort the css based on media query and pseudo
                // selector.
                //
                // If this warning were to trigger, it'd be unactionable by the user,
                // but likely not valid -- so just disable it.
                ignoreOrder: true,
              }) as NonNullable<webpack.Configuration['plugins']>[number]
            );
          }
        }

        // Packages in transpilePackages ship untransformed source (Next requirement
        // for StyleX-authoring packages), so they are exactly the node_modules
        // packages the stylex-loader must process
        const stylexPackages = Array.from(
          new Set([
            ...(rspackPluginOptions.stylexPackages ?? DEFAULT_STYLEX_PACKAGES),
            ...(nextConfig.transpilePackages ?? []),
          ])
        );

        config.plugins.push(
          new StyleXRspackPlugin({
            // Built-in Next.js defaults come first so user options can
            // override them (e.g. `nextjsAppRouterMode: false` for the Pages
            // Router, where each compiler sees the complete rule set)
            nextjsMode: true,
            nextjsAppRouterMode: true,
            ...rspackPluginOptions,
            // Pre-resolved absolute path: the plugin's chunk pattern and the
            // css rule above can never disagree about the carrier location
            ...(rspackPluginOptions.carrierCss ? { carrierCss: carrierPath } : {}),
            // Computed values always win: `dev` must reflect this Next.js
            // build, and stylexPackages merges in transpilePackages
            stylexPackages,
            rsOptions: {
              ...rspackPluginOptions.rsOptions,
              dev: ctx.dev,
            },
            ...(extractCSS
              ? {
                  async transformCss(css, filePath) {
                    const { postcssWithPlugins } = await postcss();

                    const result = await postcssWithPlugins.process(css, {
                      from: filePath,
                      map: {
                        inline: false,
                        annotation: false,
                      },
                    });

                    if (typeof rspackPluginOptions.transformCss === 'function') {
                      return rspackPluginOptions.transformCss(result.css, filePath);
                    }

                    return result.css;
                  },
                }
              : { transformCss: undefined }),
          }) as unknown as NonNullable<webpack.Configuration['plugins']>[number]
        );

        return config;
      },
    });
  };

export default withStyleX;

exportAsCommonJs(typeof module === 'undefined' ? undefined : module, withStyleX);
