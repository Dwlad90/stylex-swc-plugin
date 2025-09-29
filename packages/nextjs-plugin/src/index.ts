import nextMiniCssExtractPluginExports from 'next/dist/build/webpack/plugins/mini-css-extract-plugin';
import { warn } from 'next/dist/build/output/log';
import browserslist from 'next/dist/compiled/browserslist';
import { lazyPostCSS } from 'next/dist/build/webpack/config/blocks/css';
import StyleXWebpackPlugin, { VIRTUAL_CSS_PATTERN } from '@toss/stylexswc-webpack-plugin';

import type { NextConfig, WebpackConfigContext } from 'next/dist/server/config-shared';
import type { StyleXPluginOption } from '@toss/stylexswc-webpack-plugin';
import type webpack from 'webpack';
import type { Processor as PostCSSProcessor } from 'postcss';
import type { ConfigurationContext as WebpackConfigurationContext } from 'next/dist/build/webpack/config/utils';

/** Next.js' precompilation add "__esModule: true", but doesn't add an actual default exports */
const NextMiniCssExtractPlugin: (typeof import('next/dist/build/webpack/plugins/mini-css-extract-plugin'))['default'] =
  nextMiniCssExtractPluginExports;

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

const getNextMiniCssExtractPlugin = (isDev: boolean) => {
  // Use own MiniCssExtractPlugin to ensure HMR works
  // v9 has issues when using own plugin in production
  // v10.2.1 has issues when using built-in plugin in development since it
  // doesn't bundle HMR files
  // v12.1.7 finaly fixes the issue by adding the missing hmr/hotModuleReplacement.js file
  if (isDev) {
    try {
      // Check if hotModuleReplacement exists
      require.resolve('next/dist/compiled/mini-css-extract-plugin/hmr/hotModuleReplacement');
      return NextMiniCssExtractPlugin;
    } catch {
      warn(
        'Next.js built-in mini-css-extract-plugin is broken, will fallback to "mini-css-extract-plugin"'
      );
      // eslint-disable-next-line @typescript-eslint/no-require-imports
      return require('mini-css-extract-plugin');
    }
  }
  // Always use Next.js built-in MiniCssExtractPlugin in production
  return NextMiniCssExtractPlugin;
};

// Adopt from Next.js' getGlobalCssLoader
// https://github.com/vercel/next.js/blob/d61b0761efae09bd9cb1201ff134ed8950d9deca/packages/next/src/build/webpack/config/blocks/css/loaders/global.ts#L7
function getStyleXVirtualCssLoader(
  ctx: WebpackConfigContext,
  MiniCssExtractPlugin: typeof NextMiniCssExtractPlugin,
  postcss: () => Promise<unknown>
) {
  const loaders: webpack.RuleSetUseItem[] = [];

  // Adopt from Next.js' getClientStyleLoader
  // https://github.com/vercel/next.js/blob/56d35ede8ed2ab25fa8e29583d4e81e3e76a0e29/packages/next/src/build/webpack/config/blocks/css/loaders/global.ts#L7
  if (!ctx.isServer) {
    // https://github.com/vercel/next.js/blob/56d35ede8ed2ab25fa8e29583d4e81e3e76a0e29/packages/next/src/build/webpack/config/blocks/css/loaders/global.ts#L18
    // https://github.com/vercel/next.js/blob/56d35ede8ed2ab25fa8e29583d4e81e3e76a0e29/packages/next/src/build/webpack/config/blocks/css/loaders/client.ts#L3
    loaders.push({
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      loader: (MiniCssExtractPlugin as any).loader,
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

let count = 0;

const withStyleX =
  (pluginOptions?: StyleXPluginOption) =>
  (nextConfig: NextConfig = {}): NextConfig => {
    return {
      ...nextConfig,
      webpack(
        config: webpack.Configuration & WebpackConfigurationContext,
        ctx: WebpackConfigContext
      ) {
        if (typeof nextConfig.webpack === 'function') {
          config = nextConfig.webpack(config, ctx);
        }

        const { buildId, dev, isServer } = ctx;

        if (pluginOptions?.rsOptions?.debug || process.env.STYLEX_DEBUG) {
          console.log(
            [
              '!!!GETTING WEBPACK CONFIG!!!',
              '======================',
              `Count: ${++count}`,
              `Build ID: ${buildId}`,
              `Server: ${isServer}`,
              `Env: ${dev ? 'dev' : 'prod'}`,
            ].join('\n')
          );
        }

        // For some reason, Next 11.0.1 has `config.optimization.splitChunks`
        // set to `false` when webpack 5 is enabled.
        config.optimization ||= {};
        config.optimization.splitChunks ||= {};
        config.optimization.splitChunks.cacheGroups ||= {};

        const extractCSS = pluginOptions?.extractCSS ?? true;

        config.plugins ??= [];

        let lazyPostCSSPromise: Promise<{
          postcss: typeof import('postcss');
          postcssWithPlugins: PostCSSProcessor;
        }> | null = null;
        const postcss = () => {
          lazyPostCSSPromise ||= lazyPostCSS(
            ctx.dir,
            getSupportedBrowsers(ctx.dir, ctx.dev),
            undefined,
            false
          );
          return lazyPostCSSPromise;
        };

        if (extractCSS) {
          const MiniCssExtractPlugin = getNextMiniCssExtractPlugin(ctx.dev);
          // Based on https://github.com/vercel/next.js/blob/88a5f263f11cb55907f0d89a4cd53647ee8e96ac/packages/next/build/webpack/config/helpers.ts#L12-L18
          const cssRules = (
            config.module?.rules?.find(
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
            ) as webpack.RuleSetRule
          ).oneOf;
          // Here we matches virtual css file emitted by StyleXPlugin
          cssRules?.unshift({
            test: VIRTUAL_CSS_PATTERN,
            use: getStyleXVirtualCssLoader(ctx, MiniCssExtractPlugin, postcss),
          });

          // StyleX need to emit the css file on both server and client, both during the
          // development and production.
          // However, Next.js only add MiniCssExtractPlugin on client + production.
          //
          // To simplify the logic at our side, we will add MiniCssExtractPlugin based on
          // the "instanceof" check (We will only add our required MiniCssExtractPlugin if
          // Next.js hasn't added it yet).
          // This also prevent multiple MiniCssExtractPlugin being added (which will cause
          // RealContentHashPlugin to panic)
          if (!config.plugins.some((plugin: unknown) => plugin instanceof MiniCssExtractPlugin)) {
            // HMR reloads the CSS file when the content changes but does not use
            // the new file name, which means it can't contain a hash.
            const filename = ctx.dev ? 'static/css/[name].css' : 'static/css/[contenthash].css';

            // Logic adopted from https://git.io/JtdBy
            config.plugins.push(
              new MiniCssExtractPlugin({
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
              })
            );
          }
        }

        config.plugins.push(
          new StyleXWebpackPlugin({
            ...pluginOptions,
            rsOptions: {
              ...pluginOptions?.rsOptions,
              dev: ctx.dev,
            },
            // Enforce nextjsMode to true
            nextjsMode: true,
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
          })
        );

        return config;
      },
    };
  };

export default withStyleX;

module.exports = withStyleX;
module.exports.default = withStyleX;
