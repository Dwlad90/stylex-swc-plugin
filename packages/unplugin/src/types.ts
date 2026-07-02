import type { StyleXOptions } from '@stylexswc/rs-compiler';
import type { TransformedOptions } from '@stylexswc/rs-compiler';

/**
 * Transforms extracted StyleX CSS before it is emitted or injected.
 *
 * `filePath` identifies the CSS destination and is bundler-specific:
 * - webpack/rspack/rollup injection: the output asset name (e.g. `app.css`)
 * - esbuild disk writes: the absolute path of the written file
 * - Vite placeholder replacement: the id of the CSS module being loaded
 * - generated assets: the configured `fileName`, with `[hash]` left unresolved
 *   (the hash is computed from the transformed CSS, so it cannot be known earlier)
 * - `undefined` when no destination is known
 *
 * Buffer results are decoded as UTF-8. Results are memoized per `filePath`
 * while the input CSS is unchanged, so the callback must be a pure function
 * of its arguments.
 */
export type CSSTransformer = (
  css: string,
  filePath: string | undefined
) => string | Buffer | Promise<string | Buffer>;

export interface UnpluginStylexRSOptions {
  fileName?: string;
  useCSSLayers?: TransformedOptions['useLayers'];
  pageExtensions?: string[];
  rsOptions?: StyleXOptions;
  extractCSS?: boolean;
  /**
   * Transform the extracted StyleX CSS before it is emitted or injected.
   *
   * This matches the webpack plugin API and receives the generated CSS plus the
   * target CSS asset path when one is known.
   */
  transformCss?: CSSTransformer;
  /**
   * Enable CSS injection into CSS files via placeholder marker.
   *
   * - When set to `true`, the plugin will look for the default `@stylex;` marker
   * - When set to a string, the plugin will use that string as the marker
   *
   * Example with default marker:
   * ```css
   * @stylex;
   * ```
   * ```js
   * { useCssPlaceholder: true }
   * ```
   *
   * Example with custom marker:
   * ```css
   * /* INJECT_STYLEX_HERE *\/
   * ```
   * ```js
   * { useCssPlaceholder: '/* INJECT_STYLEX_HERE *\/' }
   * ```
   *
   * This allows StyleX CSS to go through the standard bundler CSS pipeline
   * (PostCSS, css-loader, LightningCSS, etc.) for processing.
   *
   * Works with all bundlers: Vite, Webpack, Rspack, esbuild, Rollup, Farm.
   */
  useCssPlaceholder?: boolean | string;
}
