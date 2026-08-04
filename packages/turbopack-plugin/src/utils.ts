import { SourceMaps, normalizeRsOptions, transform } from '@stylexswc/rs-compiler';
import type { StyleXOptions, StyleXTransformResult } from '@stylexswc/rs-compiler';

import type { SourceMap } from './types';

export function generateStyleXOutput(
  resourcePath: string,
  inputSource: string,
  rsOptions: Partial<StyleXOptions>,
  inputSourceMap?: SourceMap,
  sourceMapsEnabled?: boolean
): StyleXTransformResult {
  const options = normalizeRsOptions(rsOptions ?? {});

  // The bundler drops the emitted map when its own `devtool` is off, so
  // building one is pure waste — and costly waste now that the authored source
  // is inlined into it. An explicit `rsOptions.sourceMap` still wins.
  //
  // Only an explicit `false` disables it: Turbopack's loader context is a
  // partial webpack shim that may not define `this.sourceMap`, and treating
  // that `undefined` as "off" would silently strip every source map.
  if (sourceMapsEnabled === false && rsOptions?.sourceMap === undefined) {
    options.sourceMap = SourceMaps.False;
  }

  // Forward the previous loader's source map so debug source-map annotations
  // and the emitted map resolve to the original authored file instead of the
  // (possibly already transformed) loader input.
  if (inputSourceMap != null && options.inputSourceMap === undefined) {
    options.inputSourceMap =
      typeof inputSourceMap === 'string' ? inputSourceMap : JSON.stringify(inputSourceMap);
  }

  return transform(resourcePath, inputSource, options);
}
