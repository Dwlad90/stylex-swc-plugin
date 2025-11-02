import { PLUGIN_NAME } from './constants';
import { LoaderInterpolateOption } from 'loader-utils';
import { generateStyleXOutput } from './utils';

import type { InputCode, SourceMap, StyleXTurbopackLoaderOptions } from './types';
import type { LoaderContext } from 'webpack';

const skipWarnRegex = /empty|client-only/;

export default async function stylexTurbopackLoader(
  this: LoaderContext<LoaderInterpolateOption & StyleXTurbopackLoaderOptions>,
  inputCode: InputCode,
  inputSourceMap: SourceMap
) {
  const callback = this.async();

  const { stylexImports = ['stylex', '@stylexjs/stylex'], rsOptions } = this.getOptions();

  const logger = this._compiler?.getInfrastructureLogger(PLUGIN_NAME);

  if (!inputCode) {
    if (!skipWarnRegex.test(this.resourcePath)) {
      logger?.warn(
        `@stylexswc/webpack-plugin: inputCode is empty for resource ${this.resourcePath}`
      );
    }

    return callback(null, inputCode, inputSourceMap);
  }

  const stringifiedInputCode = typeof inputCode === 'string' ? inputCode : inputCode.toString();

  // bail out early if the input doesn't contain stylex imports
  if (
    !stylexImports?.some(importName =>
      typeof importName === 'string'
        ? stringifiedInputCode.includes(importName)
        : stringifiedInputCode.includes(importName.as) ||
          stringifiedInputCode.includes(importName.from)
    )
  ) {
    return callback(null, stringifiedInputCode, inputSourceMap);
  }

  try {
    const { code, map, metadata } = generateStyleXOutput(
      this.resourcePath,
      stringifiedInputCode,
      rsOptions
    );

    // If metadata.stylex doesn't exist at all, we only need to return the transformed code
    if (
      !metadata ||
      !('stylex' in metadata) ||
      metadata.stylex == null ||
      !metadata.stylex.length
    ) {
      return callback(null, code ?? undefined, map ?? undefined);
    }

    logger?.debug(`Read stylex styles from ${this.resourcePath}:`, metadata.stylex);

    // Return the transformed code with StyleX metadata processed.
    // The CSS styles have been extracted and compiled outside of this loader (for example, by a separate PostCSS step).
    return callback(null, code, map ?? undefined);
  } catch (error) {
    return callback(error as Error);
  }
}
