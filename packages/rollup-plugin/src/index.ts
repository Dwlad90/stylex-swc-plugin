import {
  normalizeRsOptions,
  shouldTransformFile,
  transform as stylexTransform,
} from '@stylexswc/rs-compiler';
import type { Rule } from '@stylexjs/babel-plugin';
import { transform } from 'lightningcss';
import type { CustomAtRules, TransformOptions } from 'lightningcss';
import type { Plugin, TransformResult, TransformPluginContext } from 'rollup';
import browserslist from 'browserslist';
import { browserslistToTargets } from 'lightningcss';
import stylexBabelPlugin from '@stylexjs/babel-plugin';
import crypto from 'crypto';

import type { StyleXOptions, TransformedOptions, UseLayersType } from '@stylexswc/rs-compiler';

function replaceFileName(original: string, css: string) {
  if (!original.includes('[hash]')) {
    return original;
  }
  const hash = crypto.createHash('sha256').update(css).digest('hex').slice(0, 8);
  return original.replace(/\[hash\]/g, hash);
}

export type PluginOptions = {
  rsOptions?: StyleXOptions;
  fileName?: string;
  useCSSLayers?: UseLayersType;
  lightningcssOptions?: Omit<TransformOptions<CustomAtRules>, 'code' | 'filename' | 'visitor'>;
  extractCSS?: boolean;
};

export default function stylexPlugin({
  rsOptions = {},
  fileName = 'stylex.css',
  useCSSLayers = false,
  lightningcssOptions,
  extractCSS = true,
}: PluginOptions = {}): Plugin {
  let stylexRules: Record<string, Rule[]> = {};

  const transformedOptions: TransformedOptions = {
    useLayers: useCSSLayers,
    enableLTRRTLComments: rsOptions?.enableLTRRTLComments,
    legacyDisableLayers: rsOptions?.legacyDisableLayers,
  };

  return {
    name: 'rollup-plugin-stylex',
    buildStart() {
      stylexRules = {};
    },
    generateBundle() {
      const rules: Array<Rule> = Object.values(stylexRules).flat();
      if (rules.length > 0) {
        const collectedCSS = stylexBabelPlugin.processStylexRules(rules, transformedOptions);
        // Process the CSS using lightningcss
        const { code } = transform({
          targets: browserslistToTargets(browserslist('>= 1%')),
          ...lightningcssOptions,
          filename: 'stylex.css',
          code: Buffer.from(collectedCSS),
        });

        // Convert the Buffer back to a string
        const processedCSS = code.toString();

        this.emitFile({
          fileName: replaceFileName(fileName, processedCSS),
          source: processedCSS,
          type: 'asset',
        });
      }
    },
    shouldTransformCachedModule({ code: _code, id, meta }) {
      stylexRules[id] = meta.stylex;
      return false;
    },
    async transform(
      this: TransformPluginContext,
      inputCode: string,
      id: string
    ): Promise<null | TransformResult> {
      // Check if file should be transformed based on include/exclude patterns
      const shouldTransform = shouldTransformFile(id, rsOptions?.include, rsOptions?.exclude);

      if (!shouldTransform) {
        return null;
      } else {
        rsOptions.include = undefined;
        rsOptions.exclude = undefined;
      }

      const normalizedRsOptions = normalizeRsOptions(rsOptions ?? {});

      if (
        !normalizedRsOptions.importSources?.some(importName =>
          typeof importName === 'string'
            ? inputCode.includes(importName)
            : inputCode.includes(importName.from)
        )
      ) {
        // In rollup, returning null from any plugin phase means
        // "no changes made".
        return null;
      }

      // The combined map of all previous plugins lets the compiler resolve
      // debug source-map annotations to the original authored file. The
      // compiler only reads it for those annotations (`debug` +
      // `enableDebugDataProp`); plain map chaining is handled by Rollup
      // itself when the plugin returns its own map. Fetching it
      // unconditionally is expensive: with no previous maps Rollup
      // synthesizes a hi-res map of the whole module, which then gets
      // stringified, re-parsed and cloned per module. Rollup resets the
      // sourcemap chain when `getCombinedSourcemap` is used, so returning
      // the chained map is the expected contract.
      const needsInputSourceMap =
        (normalizedRsOptions.debug ?? normalizedRsOptions.dev) === true &&
        normalizedRsOptions.enableDebugDataProp !== false;

      if (needsInputSourceMap && normalizedRsOptions.inputSourceMap === undefined) {
        try {
          const combinedMap = this.getCombinedSourcemap();

          if (combinedMap?.mappings) {
            normalizedRsOptions.inputSourceMap = JSON.stringify(combinedMap);
          }
        } catch {
          // No usable source map for this module - annotations fall back to
          // locating positions in the source text.
        }
      }

      const result = stylexTransform(id, inputCode, normalizedRsOptions);

      if (result == null) {
        console.warn('stylex: transformAsync returned null');
        return { code: inputCode };
      }
      const { code, map, metadata } = result;
      if (code == null) {
        console.warn('stylex: transformAsync returned null code');
        return { code: inputCode };
      }

      if (this.meta.watchMode) {
        const ast = this.parse(code);
        for (const stmt of ast.body) {
          if (stmt.type === 'ImportDeclaration') {
            const resolved = await this.resolve(stmt.source.value?.toString() || '', id);
            if (resolved && !resolved.external) {
              const result = await this.load(resolved);
              if (result && result.meta && 'stylex' in result.meta) {
                stylexRules[resolved.id] = result.meta.stylex;
              }
            }
          }
        }
      }

      if (
        extractCSS &&
        !normalizedRsOptions.runtimeInjection &&
        metadata.stylex != null &&
        metadata.stylex.length > 0
      ) {
        stylexRules[id] = metadata.stylex;
      }

      return { code, map: map, meta: metadata };
    },
  };
}
