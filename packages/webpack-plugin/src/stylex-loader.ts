import { VIRTUAL_CSS_PATH, PLUGIN_NAME } from './constants';
import loaderUtils from 'loader-utils';
import { generateStyleXOutput, isSupplementedLoaderContext, stringifyRequest } from './utils';

import type {
  InputCode,
  SourceMap,
  StyleXWebpackLoaderOptions,
  SupplementedLoaderContext,
} from './types';

export default async function stylexLoader(
  this: SupplementedLoaderContext<StyleXWebpackLoaderOptions>,
  inputCode: InputCode,
  inputSourceMap: SourceMap
) {
  const callback = this.async();

  const {
    stylexImports,
    rsOptions,
    nextjsMode,
    transformer,
    extractCSS = true,
  } = this.getOptions();

  const logger = this._compiler?.getInfrastructureLogger(PLUGIN_NAME);

  if (!inputCode) {
    if (!this.resourcePath.includes('empty')) {
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

  if (!isSupplementedLoaderContext(this)) {
    return callback(new Error('stylex-loader: loader context is not SupplementedLoaderContext!'));
  }

  try {
    const { code, map, metadata } = generateStyleXOutput(
      this.resourcePath,
      stringifiedInputCode,
      rsOptions,
      transformer
    );

    // If metadata.stylex doesn't exist at all, we only need to return the transformed code
    if (
      !extractCSS ||
      !metadata ||
      !('stylex' in metadata) ||
      metadata.stylex == null ||
      !metadata.stylex.length
    ) {
      if (extractCSS) logger?.debug(`No stylex styles generated from ${this.resourcePath}`);
      return callback(null, code ?? undefined, map ?? undefined);
    }

    logger?.debug(`Read stylex styles from ${this.resourcePath}:`, metadata.stylex);

    this.StyleXWebpackContextKey.registerStyleXRules(this.resourcePath, metadata.stylex);

    const serializedStyleXRules = JSON.stringify(metadata.stylex);

    const urlParams = new URLSearchParams({
      from: this.resourcePath,
      stylex: serializedStyleXRules,
    });

    if (!nextjsMode) {
      // Normal webpack mode

      // We generate a virtual css file that looks like it is relative to the source
      const virtualFileName = loaderUtils.interpolateName(
        this,
        '[path][name].[hash:base64:8].stylex.virtual.css',
        { content: serializedStyleXRules }
      );

      const virtualCssRequest = stringifyRequest(
        this,
        `${virtualFileName}!=!${VIRTUAL_CSS_PATH}?${urlParams.toString()}`
      );
      const postfix = `\nimport ${virtualCssRequest};`;

      return callback(null, code + postfix, map ?? undefined);
    }

    // Next.js App Router doesn't support inline matchResource and inline loaders
    // So we adapt Next.js' "external" css import approach instead
    const virtualCssRequest = stringifyRequest(this, `${VIRTUAL_CSS_PATH}?${urlParams.toString()}`);
    const postfix = `\nimport ${virtualCssRequest};`;

    return callback(null, code + postfix, map ?? undefined);
  } catch (error) {
    return callback(error as Error);
  }
}
