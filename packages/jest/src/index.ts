import { createHash } from 'node:crypto';

import type { TransformerCreator, SyncTransformer } from '@jest/transform';
import type { Config } from '@jest/types';
import { transform, normalizeRsOptions, shouldTransformFile } from '@stylexswc/rs-compiler';
import type { StyleXOptions } from '@stylexswc/rs-compiler';

type TransformerConfig = Config.TransformerConfig[1];

// Transformed output is only valid for the compiler that produced it, so the
// compiler version has to participate in the cache key. This package is
// published as CommonJS and Jest loads it with `require`, so reading the
// manifest synchronously is the correct thing to do here.
// eslint-disable-next-line typescript/no-require-imports
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
  // The parts are separated by NUL rather than concatenated: `update` appends
  // raw bytes, so without a delimiter a source ending in the path prefix would
  // hash identically to a different (source, path) pair.
  const hash = createHash('sha256');
  for (const part of [
    sourceText,
    sourcePath,
    JSON.stringify(options.transformerConfig),
    // Without this, upgrading the compiler leaves Jest replaying output that
    // the previous compiler produced, because nothing else in the key moves.
    COMPILER_VERSION,
  ]) {
    hash.update(part);
    hash.update('\0');
  }

  return hash.digest('hex');
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
