import { normalizeRsOptions, transform } from '@stylexswc/rs-compiler';

import type { StyleXOptions, StyleXTransformResult } from '@stylexswc/rs-compiler';
import type { SourceMap } from './types';

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
