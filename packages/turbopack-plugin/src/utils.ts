import stylexPlugin, { normalizeRsOptions } from '@stylexswc/rs-compiler';

import type { StyleXTransformResult } from '@stylexswc/rs-compiler';

export function generateStyleXOutput(
  resourcePath: string,
  inputSource: string,
  rsOptions: Partial<stylexPlugin.StyleXOptions>
): StyleXTransformResult {
  return stylexPlugin.transform(resourcePath, inputSource, normalizeRsOptions(rsOptions ?? {}));
}
