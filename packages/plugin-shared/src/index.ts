export {
  BUILD_INFO_STYLEX_KEY,
  DEFAULT_STYLEX_PACKAGES,
  INCLUDE_REGEXP,
  LOADER_TRANSFORMED_FLAG,
  NEXTJS_COMPILER_NAMES,
  PLUGIN_NAME,
  VIRTUAL_CSS_PATTERN,
  VIRTUAL_ENTRYPOINT_CSS_PATH,
  VIRTUAL_ENTRYPOINT_CSS_PATTERN,
  VIRTUAL_STYLEX_CSS_DUMMY_IMPORT_PATH,
  VIRTUAL_STYLEX_CSS_DUMMY_IMPORT_PATTERN,
  isNextJsCompilerName,
} from './constants';
export type { NextJsCompilerName } from './constants';

export { StyleXPluginCore } from './plugin-core';
export type { RegisterStyleXRules } from './plugin-core';

export { mergeStyleXRulesInto, publishStyleXRules } from './nextjs-registry';
export type { StyleXRulesMap } from './nextjs-registry';

export {
  buildVirtualCssPattern,
  escapeRegExp,
  generateStyleXOutput,
  isAllowlistedPackage,
  parseStylexRulesFromIdentifier,
  resolveLoaderPath,
  shouldProcessFile,
  stringifyRequest,
} from './utils';

export type {
  CSSTransformer,
  CacheGroupOptions,
  InputCode,
  SWCPluginRule,
  SourceMap,
  StyleXLoaderOptions,
  StyleXPluginOption,
} from './types';

import { resolveLoaderPath } from './utils';

/** Absolute path of the shared stylex-loader, resolvable by webpack/rspack. */
export const stylexLoaderPath = resolveLoaderPath('stylex-loader');

/** Absolute path of the shared virtual css loader, resolvable by webpack/rspack. */
export const stylexVirtualCssLoaderPath = resolveLoaderPath('stylex-virtual-css-loader');
