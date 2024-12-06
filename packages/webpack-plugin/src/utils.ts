import stylexPlugin from '@stylexswc/rs-compiler';

import type webpack from 'webpack';
import type { Rule } from '@stylexjs/babel-plugin';
import type { SWCPluginRule, StyleXWebpackLoaderOptions, SupplementedLoaderContext } from './types';
import type { StyleXTransformResult } from '@stylexswc/rs-compiler';

export function stringifyRequest(loaderContext: webpack.LoaderContext<unknown>, request: string) {
  return JSON.stringify(
    loaderContext.utils.contextify(loaderContext.context || loaderContext.rootContext, request)
  );
}

export const isSupplementedLoaderContext = <T>(
  context: webpack.LoaderContext<T>
): context is SupplementedLoaderContext<T> => {
  return Object.prototype.hasOwnProperty.call(context, 'StyleXWebpackContextKey');
};

export function generateStyleXOutput(
  resourcePath: string,
  inputSource: string,
  rsOptions: Partial<stylexPlugin.StyleXOptions>,
  transformer: StyleXWebpackLoaderOptions['transformer']
): StyleXTransformResult {
  if (transformer === 'swc') {
    const metadata = { stylex: [] };
    let metadataStr = '[]';

    const code = inputSource.replace(
      /\/\/*__stylex_metadata_start__(?<metadata>.+)__stylex_metadata_end__/,
      (...args) => {
        metadataStr = args.at(-1)?.metadata.split('"__stylex_metadata_end__')[0];

        return '';
      }
    );

    try {
      metadata.stylex = JSON.parse(metadataStr)?.map(
        (rule: SWCPluginRule) => [rule.class_name, rule.style, rule.priority] as Rule
      );
    } catch (e) {
      console.error('Error parsing StylexX metadata', e);
    }

    const map = undefined;

    return { code, map, metadata };
  }

  return stylexPlugin.transform(resourcePath, inputSource, rsOptions);
}
