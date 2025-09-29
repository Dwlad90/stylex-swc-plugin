import { transform, normalizeRsOptions } from '@toss/stylexswc-rs-compiler';
import { createHash } from 'crypto';

import type { StyleXOptions } from '@toss/stylexswc-rs-compiler';
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
  const { code } = transform(
    sourcePath,
    sourceText,
    normalizeRsOptions(options.transformerConfig.rsOptions ?? {})
  );

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
