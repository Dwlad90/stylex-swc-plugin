import stylexBabelPlugin from '@stylexjs/babel-plugin';
import { StyleXOptions, transform as stylexTransform } from '@stylexswc/rs-compiler';

import type { TransformOptions, StyleXPluginOption } from './types';

// Creates a stateful bundler for processing StyleX rules using Babel.
export default function createBundler() {
  const styleXRulesMap = new Map();

  // Determines if the source code should be transformed based on the presence of StyleX imports.
  function shouldTransform(sourceCode: string) {
    return sourceCode.includes('stylex');
  }

  // Transforms the source code using Babel, extracting StyleX rules and storing them.
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
      transformResult = stylexTransform(id, sourceCode, rsOptions);
    } catch (error) {
      if (shouldSkipTransformError) {
        console.warn(
          `[@stylexswc/postcss-plugin] Failed to transform "${id}": ${(error as Error).message}`
        );

        return transformResult;
      }

      throw error;
    }

    const { code, map, metadata } = transformResult;

    const stylex = metadata.stylex;
    if (stylex != null && stylex.length > 0) {
      styleXRulesMap.set(id, stylex);
    }

    return { code, map, metadata };
  }

  // Removes the stored StyleX rules for the specified file.
  function remove(id: string) {
    styleXRulesMap.delete(id);
  }

  //  Bundles all collected StyleX rules into a single CSS string.
  function bundle({ useCSSLayers }: Pick<StyleXPluginOption, 'useCSSLayers'>) {
    const rules = Array.from(styleXRulesMap.values()).flat();

    const css = stylexBabelPlugin.processStylexRules(rules, !!useCSSLayers);

    return css;
  }

  return {
    shouldTransform,
    transform,
    remove,
    bundle,
  };
}
