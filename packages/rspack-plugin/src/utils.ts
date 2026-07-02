import { normalizeRsOptions, transform } from '@stylexswc/rs-compiler';

import { VIRTUAL_CSS_PATTERN } from './constants';

import type { LoaderContext } from '@rspack/core';
import type { SupplementedLoaderContext } from './types';
import type { StyleXOptions, StyleXTransformResult } from '@stylexswc/rs-compiler';
import type { Rule as StyleXRule } from '@stylexjs/babel-plugin';

export function stringifyRequest(loaderContext: LoaderContext<unknown>, request: string) {
  return JSON.stringify(
    loaderContext.utils.contextify(loaderContext.context || loaderContext.rootContext, request)
  );
}

export const isSupplementedLoaderContext = <T>(
  context: LoaderContext<T>
): context is SupplementedLoaderContext<T> => {
  return Object.prototype.hasOwnProperty.call(context, 'StyleXRspackContextKey');
};

export function generateStyleXOutput(
  resourcePath: string,
  inputSource: string,
  rsOptions: Partial<StyleXOptions>
): StyleXTransformResult {
  return transform(resourcePath, inputSource, normalizeRsOptions(rsOptions ?? {}));
}

/**
 * Extracts serialized StyleX rules from a css module identifier.
 *
 * Rspack module identifiers are `|`-separated segments
 * (e.g. `css|/path/to/stylex.virtual.css?from=...&stylex=[...]|<runtime>`),
 * so the query has to be isolated from the segment containing the virtual
 * css request before parsing.
 */
export function parseStylexRulesFromIdentifier(identifier: string): StyleXRule[] | null {
  if (!VIRTUAL_CSS_PATTERN.test(identifier)) {
    return null;
  }

  const queryMatch = identifier.match(/stylex\.virtual\.css\?([^|]*)/);
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
