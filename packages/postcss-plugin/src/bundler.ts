import stylexBabelPlugin from '@stylexjs/babel-plugin';
import { transform as stylexTransform, normalizeRsOptions } from '@stylexswc/rs-compiler';
import type { StyleXOptions } from '@stylexswc/rs-compiler';

import type { TransformOptions, StyleXPluginOption } from './types';

// Creates a stateful bundler for processing StyleX rules using Babel.
export default function createBundler() {
  const styleXRulesMap = new Map();

  // Determines if the source code should be transformed based on the presence of StyleX imports.
  function shouldTransform(sourceCode: string, rsOptions?: StyleXPluginOption['rsOptions']) {
    const { importSources } = rsOptions ?? {};

    let parsedImportSources: StyleXOptions['importSources'] | undefined;

    try {
      parsedImportSources = importSources?.map(importSource => {
        const a = typeof importSource === 'string' ? JSON.parse(importSource) : importSource;

        return a;
      });
    } catch (error) {
      parsedImportSources = importSources;
    }

    const shouldTransform = parsedImportSources?.some(importSource => {
      if (typeof importSource === 'string') {
        return sourceCode.includes(importSource);
      }
      return sourceCode.includes(importSource.from);
    });

    // if (importSourcesExtend != null) {
    //   shouldTransform ||= importSourcesExtend.some(importSource => {
    //     if (typeof importSource === 'string') {
    //       return sourceCode.includes(importSource);
    //     }

    //     return sourceCode.includes(importSource.from);
    //   });
    // }

    return shouldTransform;
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
  function bundle({
    useCSSLayers,
    enableLTRRTLComments,
  }: Pick<StyleXPluginOption, 'useCSSLayers'> &
    Pick<NonNullable<StyleXPluginOption['rsOptions']>, 'enableLTRRTLComments'>) {
    const rules = Array.from(styleXRulesMap.values()).flat();

    // @ts-expect-error - type is not up to date and will be fixed in the future
    const css = stylexBabelPlugin.processStylexRules(rules, {
      useLayers: useCSSLayers,
      enableLTRRTLComments,
    });
    return css;
  }

  return {
    shouldTransform,
    transform,
    remove,
    bundle,
  };
}
