import path from 'path';
import browserslist from 'next/dist/compiled/browserslist';
import { warn } from 'next/dist/build/output/log';
import { lazyPostCSS } from 'next/dist/build/webpack/config/blocks/css';
import { getRspackCore } from 'next/dist/shared/lib/get-rspack';
import StyleXRspackPlugin, {
  DEFAULT_STYLEX_PACKAGES,
  buildVirtualCssPattern,
} from '@stylexswc/rspack-plugin';
import withRspack from 'next-rspack';

import type { NextConfig, WebpackConfigContext } from 'next/dist/server/config-shared';
import type { StyleXPluginOption } from '@stylexswc/rspack-plugin';
import type webpack from 'webpack';
import type { Processor as PostCSSProcessor } from 'postcss';
import type { ConfigurationContext as WebpackConfigurationContext } from 'next/dist/build/webpack/config/utils';

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
    // Ignore
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
  (pluginOptions?: StyleXPluginOption) =>
  (nextConfig: NextConfig = {}): NextConfig => {
    // Scoped per `withStyleX(...)` call rather than module-level, so it doesn't
    // leak across unrelated Next.js configs sharing this process (e.g. a
    // monorepo building multiple apps, or repeated calls in tests).
    let count = 0;

    // The App Router cross-compiler rule registry lives on `globalThis`, so
    // the client/server/edge-server compilers must share one build process.
    // Only enforced when the registry is in use — Pages Router builds
    // (`nextjsAppRouterMode: false`) keep the user's setting.
    const useAppRouterRegistry = pluginOptions?.nextjsAppRouterMode ?? true;

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
      webpack(
        config: webpack.Configuration & WebpackConfigurationContext,
        ctx: WebpackConfigContext
      ) {
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

        if (typeof nextConfig.webpack === 'function') {
          config = nextConfig.webpack(config, ctx);
        }

        const { buildId, dev, isServer } = ctx;

        count += 1;

        if (pluginOptions?.rsOptions?.debug || process.env.STYLEX_DEBUG) {
          warn(
            `@stylexswc/nextjs-plugin/rspack: rspack config #${count} (buildId=${buildId}, server=${isServer}, env=${dev ? 'dev' : 'prod'})`
          );
        }

        config.optimization ||= {};
        config.optimization.splitChunks ||= {};
        config.optimization.splitChunks.cacheGroups ||= {};

        const extractCSS = pluginOptions?.extractCSS ?? true;

        // Resolved once and shared by the css rule test below and the plugin
        // (via `carrierCss`), so the two can never disagree about the carrier
        // location when `compiler.context` differs from the project dir
        const carrierPath = pluginOptions?.carrierCss
          ? path.resolve(ctx.dir, pluginOptions.carrierCss)
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
              }) as unknown as NonNullable<webpack.Configuration['plugins']>[number]
            );
          }
        }

        // Packages in transpilePackages ship untransformed source (Next requirement
        // for StyleX-authoring packages), so they are exactly the node_modules
        // packages the stylex-loader must process
        const stylexPackages = Array.from(
          new Set([
            ...(pluginOptions?.stylexPackages ?? DEFAULT_STYLEX_PACKAGES),
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
            ...pluginOptions,
            // Pre-resolved absolute path: the plugin's chunk pattern and the
            // css rule above can never disagree about the carrier location
            ...(pluginOptions?.carrierCss ? { carrierCss: carrierPath } : {}),
            // Computed values always win: `dev` must reflect this Next.js
            // build, and stylexPackages merges in transpilePackages
            stylexPackages,
            rsOptions: {
              ...pluginOptions?.rsOptions,
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

                    if (typeof pluginOptions?.transformCss === 'function') {
                      return pluginOptions.transformCss(result.css, filePath);
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

module.exports = withStyleX;
module.exports.default = withStyleX;
