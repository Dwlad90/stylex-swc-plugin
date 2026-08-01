import { transform, normalizeRsOptions, shouldTransformFile } from '@stylexswc/rs-compiler';
import { createHash } from 'node:crypto';

import type { StyleXOptions } from '@stylexswc/rs-compiler';
import type { TransformerCreator, SyncTransformer } from '@jest/transform';
import type { Config } from '@jest/types';

type TransformerConfig = Config.TransformerConfig[1];

// Transformed output is only valid for the compiler that produced it, so the
// compiler version has to participate in the cache key.
const { version: COMPILER_VERSION } = require('@stylexswc/rs-compiler/package.json') as {
  version: string;
};

export interface JestTransformerConfig extends TransformerConfig {
  rsOptions?: StyleXOptions;
}

const process: SyncTransformer<JestTransformerConfig>['process'] = function process(
  sourceText,
  sourcePath,
  options
) {
  // Destructure rather than delete: `transformerConfig` is created once per run
  // and shared across every file Jest transforms, so mutating it here would
  // strip the patterns for all later files.
  const { include, exclude, ...compilerOptions } = options.transformerConfig.rsOptions ?? {};

  // Check if file should be transformed based on include/exclude patterns
  if (!shouldTransformFile(sourcePath, include, exclude)) {
    return { code: sourceText };
  }

  const { code } = transform(sourcePath, sourceText, normalizeRsOptions(compilerOptions));

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
    // Without this, upgrading the compiler leaves Jest replaying output that
    // the previous compiler produced, because nothing else in the key moves.
    .update(COMPILER_VERSION)
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
