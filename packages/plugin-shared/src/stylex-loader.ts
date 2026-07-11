import path from 'path';

import {
  BUILD_INFO_STYLEX_KEY,
  LOADER_TRANSFORMED_FLAG,
  PLUGIN_NAME,
  VIRTUAL_STYLEX_CSS_DUMMY_IMPORT_PATH,
} from './constants';
import { generateStyleXOutput, stringifyRequest } from './utils';

import type { InputCode, SourceMap, StyleXLoaderOptions } from './types';
import type { LoaderContext } from 'webpack';

// next/dist/... empty module shims and the `client-only`/`server-only`
// packages legitimately produce empty loader input — anchored so user files
// that merely contain these substrings (e.g. `empty-state.ts`) still warn
const skipWarnRegex = /[\\/](?:empty|client-only|server-only)(?:\.[cm]?js)?$/;

export default async function stylexLoader(
  this: LoaderContext<StyleXLoaderOptions>,
  inputCode: InputCode,
  inputSourceMap: SourceMap
) {
  const callback = this.async();

  const { stylexImports, rsOptions, extractCSS = true } = this.getOptions();

  const logger = this._compiler?.getInfrastructureLogger?.(PLUGIN_NAME);

  if (!inputCode) {
    if (!skipWarnRegex.test(this.resourcePath)) {
      logger?.warn(`stylex-loader: inputCode is empty for resource ${this.resourcePath}`);
    }

    return callback(null, inputCode, inputSourceMap);
  }

  const stringifiedInputCode = typeof inputCode === 'string' ? inputCode : inputCode.toString();

  // bail out early if already transformed
  // for some reason, a module might be passed to stylex-loader more than once,
  // happened with Next.js App Router
  if (stringifiedInputCode.includes(LOADER_TRANSFORMED_FLAG)) {
    return callback(null, stringifiedInputCode, inputSourceMap);
  }

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
      rsOptions,
      inputSourceMap
    );

    let parsedMap: SourceMap = undefined;

    if (map) {
      try {
        parsedMap = typeof map === 'string' ? JSON.parse(map) : map;
      } catch (error) {
        logger?.warn(
          `stylex-loader: failed to parse map for resource ${this.resourcePath}: ${(error as Error).message}`
        );
      }
    }

    // If metadata.stylex doesn't exist at all, we only need to return the transformed code
    if (
      !extractCSS ||
      !metadata ||
      !('stylex' in metadata) ||
      metadata.stylex == null ||
      !metadata.stylex.length
    ) {
      if (extractCSS) logger?.debug(`No stylex styles generated from ${this.resourcePath}`);
      // The code WAS transformed — stamp the flag so a second pass through
      // the loader chain never re-transforms it.
      return callback(null, `${code}\n${LOADER_TRANSFORMED_FLAG}`, parsedMap);
    }

    logger?.debug(`Read stylex styles from ${this.resourcePath}:`, metadata.stylex);

    // webpack rule transport: buildInfo is persisted in webpack's filesystem
    // cache alongside the module, so the plugin can re-collect rules on cached
    // rebuilds. rspack loaders don't expose a persistent `_module.buildInfo`
    // across the native boundary — there the identifier query below is the
    // transport and this write is a harmless no-op.
    const buildInfo = (this._module as { buildInfo?: Record<string, unknown> } | undefined)
      ?.buildInfo;

    if (buildInfo) {
      buildInfo[BUILD_INFO_STYLEX_KEY] = {
        resourcePath: this.resourcePath,
        stylexRules: metadata.stylex,
      };
    }

    // Relative to the compiler context so module identifiers (and therefore
    // chunk hashes) stay machine-independent across CI runners.
    const from = path.relative(this.rootContext, this.resourcePath);

    const urlParams = new URLSearchParams({
      from,
      stylex: JSON.stringify(metadata.stylex),
    });

    // Dummy virtual import picked up by the virtual css loader: routes the
    // rules into the stylex chunk and invalidates HMR in development.
    const virtualCssRequest = stringifyRequest(
      this,
      `${VIRTUAL_STYLEX_CSS_DUMMY_IMPORT_PATH}?${urlParams.toString()}`
    );
    const postfix = `\nimport ${virtualCssRequest};\n${LOADER_TRANSFORMED_FLAG}`;

    return callback(null, code + postfix, parsedMap);
  } catch (error) {
    return callback(error as Error);
  }
}
