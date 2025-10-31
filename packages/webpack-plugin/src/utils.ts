import stylexPlugin, { normalizeRsOptions } from '@stylexswc/rs-compiler';

import type webpack from 'webpack';
import type { SupplementedLoaderContext } from './types';
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
  rsOptions: Partial<stylexPlugin.StyleXOptions>
): StyleXTransformResult {
  return stylexPlugin.transform(resourcePath, inputSource, normalizeRsOptions(rsOptions ?? {}));
}
