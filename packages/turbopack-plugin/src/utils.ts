import { normalizeRsOptions, transform } from '@stylexswc/rs-compiler';

import type { StyleXOptions, StyleXTransformResult } from '@stylexswc/rs-compiler';

export function generateStyleXOutput(
  resourcePath: string,
  inputSource: string,
  rsOptions: Partial<StyleXOptions>
): StyleXTransformResult {
  return transform(resourcePath, inputSource, normalizeRsOptions(rsOptions ?? {}));
}
