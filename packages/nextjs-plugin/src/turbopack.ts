import type { NextConfig, TurbopackLoaderItem } from 'next/dist/server/config-shared';
import type { StyleXPluginOption } from '@stylexswc/webpack-plugin';

const withStyleX =
  (
    pluginOptions?: Omit<
      StyleXPluginOption,
      'useCSSLayers' | 'nextjsMode' | 'transformCss' | 'extractCSS'
    >
  ) =>
  (nextConfig: NextConfig = {}): NextConfig => {
    const reactLoaders: TurbopackLoaderItem[] = [
      {
        loader: '@stylexswc/turbopack-plugin/loader',
        // @ts-expect-error - StyleXPluginOption is compatible with Next.js' Turbopack loader options, but types are not aligned yet
        options: {
          ...pluginOptions,
        },
      },
    ];

    return {
      ...nextConfig,
      turbopack: {
        ...nextConfig?.turbopack,
        rules: {
          '*.ts': {
            loaders: reactLoaders,
          },
          '*.tsx': {
            loaders: reactLoaders,
          },
          '*.js': {
            loaders: reactLoaders,
          },
          '*.jsx': {
            loaders: reactLoaders,
          },
        },
      },
    };
  };

export default withStyleX;

module.exports = withStyleX;
module.exports.default = withStyleX;
