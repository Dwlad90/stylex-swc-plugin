import { normalizeRsOptions, transform } from '@stylexswc/rs-compiler';

import type webpack from 'webpack';
import type { SourceMap, SupplementedLoaderContext } from './types';
import type { StyleXOptions, StyleXTransformResult } from '@stylexswc/rs-compiler';

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
  rsOptions: Partial<StyleXOptions>,
  inputSourceMap?: SourceMap
): StyleXTransformResult {
  const options = normalizeRsOptions(rsOptions ?? {});

  // Forward the previous loader's source map so debug source-map annotations
  // and the emitted map resolve to the original authored file instead of the
  // (possibly already transformed) loader input.
  if (inputSourceMap != null && options.inputSourceMap === undefined) {
    options.inputSourceMap =
      typeof inputSourceMap === 'string' ? inputSourceMap : JSON.stringify(inputSourceMap);
  }

  return transform(resourcePath, inputSource, options);
}
