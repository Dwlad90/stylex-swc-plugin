import { transform, normalizeRsOptions, shouldTransformFile } from '@stylexswc/rs-compiler';
import { createHash } from 'crypto';

import type { StyleXOptions } from '@stylexswc/rs-compiler';
import type { TransformerCreator, SyncTransformer } from '@jest/transform';
import type { Config } from '@jest/types';

type TransformerConfig = Config.TransformerConfig[1];

export interface JestTransformerConfig extends TransformerConfig {
  rsOptions?: StyleXOptions;
}

const process: SyncTransformer<JestTransformerConfig>['process'] = function process(
  sourceText,
  sourcePath,
  options
) {
  const rsOptions = options.transformerConfig.rsOptions ?? {};

  // Check if file should be transformed based on include/exclude patterns
  const shouldTransform = shouldTransformFile(sourcePath, rsOptions.include, rsOptions.exclude);

  if (!shouldTransform) {
    return { code: sourceText };
  } else {
    rsOptions.include = undefined;
    rsOptions.exclude = undefined;
  }

  const { code } = transform(sourcePath, sourceText, normalizeRsOptions(rsOptions));

  return { code };
};

const processAsync: SyncTransformer<JestTransformerConfig>['processAsync'] =
  async function processAsync(sourceText, sourcePath, options) {
    return process(sourceText, sourcePath, options);
  };

const getCacheKey: SyncTransformer<JestTransformerConfig>['getCacheKey'] = function getCacheKey(
  sourceText,
  sourcePath,
  options
) {
  return createHash('sha256')
    .update(sourceText)
    .update(sourcePath)
    .update(JSON.stringify(options.transformerConfig))
    .digest('hex');
};

const createTransformer: TransformerCreator<
  SyncTransformer<JestTransformerConfig>,
  JestTransformerConfig
> = () => {
  return {
    process,
    processAsync,
    getCacheKey,
  };
};

module.exports = { createTransformer };
export default { createTransformer };
