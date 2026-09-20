import stylexBabelPlugin from '@stylexjs/babel-plugin';
import { transform as stylexTransform, normalizeRsOptions } from '@stylexswc/rs-compiler';
import type { StyleXOptions, TransformedOptions } from '@stylexswc/rs-compiler';

import type { TransformOptions } from './types';

// Creates a stateful bundler for processing StyleX rules using Babel.
export default function createBundler() {
  const styleXRulesMap = new Map();

  // Transforms the source code, extracting StyleX rules and storing them.
  //
  // `collected` says whether the rules of this file reached the bundler. A
  // transform error that `shouldSkipTransformError` swallows leaves them
  // uncollected, and the caller has to know: a file recorded as built without
  // its rules is skipped by every later build, so its classes go missing until
  // the file is edited again.
  function transform(
    id: string,
    sourceCode: string,
    rsOptions: StyleXOptions,
    options: TransformOptions
  ) {
    const { shouldSkipTransformError } = options;

    let transformResult: ReturnType<typeof stylexTransform> = {
      code: sourceCode,
      map: undefined,
      metadata: { stylex: [] },
    };

    try {
      const rsOptionsNormalized = normalizeRsOptions(rsOptions);

      transformResult = stylexTransform(id, sourceCode, rsOptionsNormalized);
    } catch (error) {
      if (shouldSkipTransformError) {
        console.warn(
          `[@stylexswc/postcss-plugin] Failed to transform "${id}": ${(error as Error).message}`
        );

        return { ...transformResult, collected: false };
      }

      throw error;
    }

    const { code, map, metadata } = transformResult;

    const stylex = metadata.stylex;
    if (stylex != null && stylex.length > 0) {
      styleXRulesMap.set(id, stylex);
    } else {
      // A file edited from having rules to having none keeps the old ones
      // otherwise, and the stylesheet then carries classes no source declares.
      styleXRulesMap.delete(id);
    }

    return { code, map, metadata, collected: true };
  }

  // Removes the stored StyleX rules for the specified file.
  function remove(id: string) {
    styleXRulesMap.delete(id);
  }

  //  Bundles all collected StyleX rules into a single CSS string.
  function bundle(transformedOptions: TransformedOptions) {
    const rules = Array.from(styleXRulesMap.values()).flat();

    const css = stylexBabelPlugin.processStylexRules(rules, transformedOptions);
    return css;
  }

  return {
    transform,
    remove,
    bundle,
  };
}
