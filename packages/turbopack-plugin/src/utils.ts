import { resolveSourceMapOptions } from '@stylexswc/plugin-shared/source-map-options';
import { normalizeRsOptions, transform } from '@stylexswc/rs-compiler';
import type { StyleXOptions, StyleXTransformResult } from '@stylexswc/rs-compiler';

import type { SourceMap } from './types';

export function generateStyleXOutput(
  resourcePath: string,
  inputSource: string,
  rsOptions: Partial<StyleXOptions>,
  inputSourceMap?: SourceMap,
  sourceMapsEnabled?: boolean
): StyleXTransformResult {
  // Shared with the webpack/rspack loader: Turbopack's loader context is a
  // partial webpack shim, so the "host never told us" case this resolves is
  // reached far more often here than there.
  const options: StyleXOptions = {
    ...normalizeRsOptions(rsOptions ?? {}),
    ...resolveSourceMapOptions(rsOptions ?? {}, sourceMapsEnabled),
  };

  // Forward the previous loader's source map so debug source-map annotations
  // and the emitted map resolve to the original authored file instead of the
  // (possibly already transformed) loader input.
  if (inputSourceMap != null && options.inputSourceMap === undefined) {
    options.inputSourceMap =
      typeof inputSourceMap === 'string' ? inputSourceMap : JSON.stringify(inputSourceMap);
  }

  return transform(resourcePath, inputSource, options);
}
