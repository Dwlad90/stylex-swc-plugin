import stylexBabelPlugin from '@stylexjs/babel-plugin';
import { transform as stylexTransform, normalizeRsOptions } from '@stylexswc/rs-compiler';
import type { StyleXOptions, TransformedOptions } from '@stylexswc/rs-compiler';

import type { TransformOptions, StyleXPluginOption } from './types';

// Creates a stateful bundler for processing StyleX rules using Babel.
export default function createBundler() {
  const styleXRulesMap = new Map();

  // Determines if the source code should be transformed based on the presence of StyleX imports.
  function shouldTransform(
    sourceCode: string,
    rsOptions?: StyleXPluginOption['rsOptions']
  ): boolean {
    const importSources = rsOptions?.importSources;

    if (!importSources) return false;

    return importSources.some(importSource => {
      // Already an object (e.g., { from: '@stylexjs/stylex' })
      if (typeof importSource !== 'string') {
        const fromTrimmed = importSource.from?.trim();

        if (!fromTrimmed) return false;

        return sourceCode.includes(importSource.from);
      }

      const importSourceTrimmed = importSource.trimStart();

      if (!importSourceTrimmed) return false;

      // JSON string edge-case: only attempt parse if it looks like a JSON object
      if (importSourceTrimmed[0] === '{') {
        try {
          const parsed = JSON.parse(importSourceTrimmed);
          if (typeof parsed.from === 'string') {
            const fromTrimmed = parsed.from.trim();

            if (!fromTrimmed) return false;

            return sourceCode.includes(fromTrimmed);
          }
        } catch {
          // Not valid JSON — fall through to plain string check
        }
      }

      // Standard string case, e.g. '@stylexjs/stylex'
      return sourceCode.includes(importSource);
    });
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
      const rsOptionsNormalized = normalizeRsOptions(rsOptions);

      transformResult = stylexTransform(id, sourceCode, rsOptionsNormalized);
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
  function bundle(transformedOptions: TransformedOptions) {
    const rules = Array.from(styleXRulesMap.values()).flat();

    const css = stylexBabelPlugin.processStylexRules(rules, transformedOptions);
    return css;
  }

  return {
    shouldTransform,
    transform,
    remove,
    bundle,
  };
}
