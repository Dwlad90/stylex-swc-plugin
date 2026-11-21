import crypto from 'node:crypto';

import { shouldProcessSource } from '@stylexswc/plugin-shared/module-selection';
import { toTransformedOptions } from '@stylexswc/plugin-shared/transformed-options';
import {
  normalizeRsOptions,
  shouldTransformFile,
  processStylexRules,
  transform as stylexTransform,
} from '@stylexswc/rs-compiler';
import type { StyleXOptions, StyleXRule as Rule, UseLayersType } from '@stylexswc/rs-compiler';
import browserslist from 'browserslist';
import { transform } from 'lightningcss';
import type { CustomAtRules, TransformOptions } from 'lightningcss';
import { browserslistToTargets } from 'lightningcss';
import type { Plugin, TransformResult, TransformPluginContext } from 'rollup';

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

  const transformedOptions = toTransformedOptions(useCSSLayers, rsOptions);

  // The options are read once, when the plugin is made, as for every Rollup
  // plugin. The plugin applies the include and exclude filter itself, so the
  // compiler gets the options without it. Nothing changes them later, so each
  // module reads the same options.
  const { include, exclude } = rsOptions;
  const compilerRsOptions = normalizeRsOptions({
    ...rsOptions,
    include: undefined,
    exclude: undefined,
  });

  // The compiler reads an input source map only for debug annotations
  // (`debug` and `enableDebugDataProp`). Plain map chaining is done by Rollup
  // when the plugin returns its own map. The map is fetched only when it is
  // needed, because with no previous maps Rollup makes a hi-res map of the
  // whole module, which is then stringified, parsed again and cloned. Rollup
  // resets the source map chain when `getCombinedSourcemap` is used, so the
  // plugin returns the chained map.
  const fetchCombinedMap =
    (compilerRsOptions.debug ?? compilerRsOptions.dev) === true &&
    compilerRsOptions.enableDebugDataProp !== false &&
    compilerRsOptions.inputSourceMap === undefined;

  return {
    name: 'rollup-plugin-stylex',
    buildStart() {
      stylexRules = {};
    },
    generateBundle() {
      const rules: Array<Rule> = Object.values(stylexRules).flat();
      if (rules.length > 0) {
        const collectedCSS = processStylexRules(rules, transformedOptions);
        // Process the CSS using lightningcss
        const { code } = transform({
          targets: browserslistToTargets(browserslist()),
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
      // A cached module that gave no rules has no `stylex` key in its meta.
      if (Array.isArray(meta.stylex)) {
        stylexRules[id] = meta.stylex;
      }
      return false;
    },
    async transform(
      this: TransformPluginContext,
      inputCode: string,
      id: string
    ): Promise<null | TransformResult> {
      // Check if file should be transformed based on include/exclude patterns
      if (!shouldTransformFile(id, include, exclude)) {
        return null;
      }

      if (!shouldProcessSource(inputCode, compilerRsOptions)) {
        // In rollup, returning null from any plugin phase means
        // "no changes made".
        return null;
      }

      // A module that gets a source map gets its own copy of the options, so
      // its map does not reach the next module.
      let moduleRsOptions = compilerRsOptions;

      if (fetchCombinedMap) {
        try {
          const combinedMap = this.getCombinedSourcemap();

          if (combinedMap?.mappings) {
            moduleRsOptions = { ...compilerRsOptions, inputSourceMap: JSON.stringify(combinedMap) };
          }
        } catch {
          // No usable source map for this module - annotations fall back to
          // locating positions in the source text.
        }
      }

      const result = stylexTransform(id, inputCode, moduleRsOptions);

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
        !compilerRsOptions.runtimeInjection &&
        metadata.stylex != null &&
        metadata.stylex.length > 0
      ) {
        stylexRules[id] = metadata.stylex;
      }

      return { code, map: map, meta: metadata };
    },
  };
}
