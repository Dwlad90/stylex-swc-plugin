import path from 'path';
import { normalizeRsOptions, shouldTransformFile, transform } from '@stylexswc/rs-compiler';

import {
  INCLUDE_REGEXP,
  VIRTUAL_CSS_PATTERN,
  VIRTUAL_STYLEX_CSS_DUMMY_IMPORT_PATTERN,
} from './constants';

import type webpack from 'webpack';
import type { SourceMap } from './types';
import type { StyleXOptions, StyleXTransformResult } from '@stylexswc/rs-compiler';
import type { Rule as StyleXRule } from '@stylexjs/babel-plugin';

export function stringifyRequest(loaderContext: webpack.LoaderContext<unknown>, request: string) {
  return JSON.stringify(
    loaderContext.utils.contextify(loaderContext.context || loaderContext.rootContext, request)
  );
}

export function generateStyleXOutput(
  resourcePath: string,
  inputSource: string,
  rsOptions: Partial<StyleXOptions>,
  inputSourceMap?: SourceMap
): StyleXTransformResult {
  const options = normalizeRsOptions(rsOptions ?? {});

  // Forward the previous loader's source map so debug source-map annotations
  // and the emitted map resolve to the original authored file instead of the
  // (possibly already transformed) loader input.
  if (inputSourceMap != null && options.inputSourceMap === undefined) {
    options.inputSourceMap =
      typeof inputSourceMap === 'string' ? inputSourceMap : JSON.stringify(inputSourceMap);
  }

  return transform(resourcePath, inputSource, options);
}

export function isAllowlistedPackage(resourcePath: string, stylexPackages: string[]) {
  const nodeModulesSegment = `${path.sep}node_modules${path.sep}`;
  const nodeModulesEntries = path.normalize(resourcePath).split(nodeModulesSegment).slice(1);

  return stylexPackages.some(packageName => {
    const normalizedPackageName = path.normalize(packageName).replace(/[\\/]$/, '');

    return nodeModulesEntries.some(
      entry =>
        entry === normalizedPackageName || entry.startsWith(`${normalizedPackageName}${path.sep}`)
    );
  });
}

/**
 * Decides whether a file goes through the stylex-loader: JS/TS sources only,
 * node_modules excluded unless allowlisted via `stylexPackages`, then the
 * user's `include`/`exclude` options.
 */
export function shouldProcessFile(
  resourcePath: string,
  options: {
    stylexPackages: string[];
    include?: StyleXOptions['include'];
    exclude?: StyleXOptions['exclude'];
  }
): boolean {
  if (!resourcePath || !INCLUDE_REGEXP.test(resourcePath)) {
    return false;
  }

  const nodeModulesSegment = `${path.sep}node_modules${path.sep}`;

  if (resourcePath.includes(nodeModulesSegment)) {
    if (!isAllowlistedPackage(resourcePath, options.stylexPackages)) {
      return false;
    }
  }

  return shouldTransformFile(resourcePath, options.include, options.exclude);
}

/**
 * Extracts serialized StyleX rules from a css module identifier.
 *
 * Rspack module identifiers are `|`-separated segments
 * (e.g. `css|/path/to/stylex-virtual.css?from=...&stylex=[...]|<runtime>`),
 * so the query has to be isolated from the segment containing the dummy
 * css request before parsing. This is the rspack rule transport: rspack
 * loaders can't persist `module.buildInfo` across the native boundary, but
 * identifiers survive caching and carry the rules across compilations.
 */
export function parseStylexRulesFromIdentifier(identifier: string): StyleXRule[] | null {
  if (!VIRTUAL_STYLEX_CSS_DUMMY_IMPORT_PATTERN.test(identifier)) {
    return null;
  }

  const queryMatch = identifier.match(/stylex-virtual\.css\?([^|]*)/);
  const query = queryMatch?.[1];

  if (!query) {
    return null;
  }

  const params = new URLSearchParams(query);
  const stylex = params.get('stylex');

  if (stylex == null) {
    return null;
  }

  try {
    return JSON.parse(stylex) as StyleXRule[];
  } catch {
    return null;
  }
}

export function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

/**
 * Pattern matching the CSS modules that belong in the stylex chunk: the
 * per-module HMR dummy imports plus the carrier stylesheet. With a custom
 * `carrierCss` path the custom carrier REPLACES the default `stylex.css`
 * filename match (avoiding false positives on unrelated files of that name).
 */
export function buildVirtualCssPattern(carrierPath?: string): RegExp {
  if (!carrierPath) {
    return VIRTUAL_CSS_PATTERN;
  }

  return new RegExp(
    `${escapeRegExp(carrierPath)}|${VIRTUAL_STYLEX_CSS_DUMMY_IMPORT_PATTERN.source}`
  );
}

/**
 * Resolves a sibling loader module path. Loaders resolve as `.ts` only when
 * the package runs from source (e.g. vitest); published dist builds always
 * resolve the compiled `.js`.
 */
export function resolveLoaderPath(loaderName: string) {
  try {
    return require.resolve(`./${loaderName}`);
  } catch (error) {
    const isModuleNotFound =
      error instanceof Error && 'code' in error && error.code === 'MODULE_NOT_FOUND';

    if (!isModuleNotFound) {
      throw error;
    }

    return require.resolve(`./${loaderName}.ts`);
  }
}
