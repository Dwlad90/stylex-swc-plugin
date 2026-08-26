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

/**
 * What to do when placeholder mode finds no stylesheet to inject into.
 *
 * - `error`: fail the build wherever the plugin can prove the marker was part
 *   of it, which today is Vite; every other bundler reports a warning, because
 *   there a missing stylesheet is indistinguishable from a build that has no
 *   CSS at all
 * - `warn`: never fatal, report everywhere
 * - `ignore`: stay silent
 */
export type MissingCssPlaceholderReport = 'error' | 'warn' | 'ignore';

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
   * This keeps your stylesheet on the standard bundler CSS pipeline (PostCSS,
   * css-loader, LightningCSS, etc.) with the StyleX rules inside it.
   *
   * In a build the rules themselves are spliced in at the marker once the
   * bundle is assembled, since modules behind a dynamic import are transformed
   * long after the stylesheet is loaded. They are minified with the rest of the
   * output, but per-module steps such as PostCSS do not run over them; use
   * `transformCss` for that. The dev server has no bundle step and inlines the
   * rules, so there the whole pipeline sees them.
   *
   * Works with Vite, Webpack, Rspack, esbuild and Rollup. Farm is not
   * supported yet: its adapter never receives the hook that replaces the
   * marker.
   */
  useCssPlaceholder?: boolean | string;
  /**
   * How to report a build where `useCssPlaceholder` is on but no stylesheet in
   * the output can carry the StyleX rules.
   *
   * Defaults to `error`: nothing links a standalone stylesheet in placeholder
   * mode, so the styles would silently go missing at runtime.
   *
   * Lower it to `warn` when another plugin legitimately takes the stylesheet
   * out of the bundle -- inlining the CSS into JavaScript, for instance -- and
   * the build is expected to have no CSS asset left by the time the rules are
   * ready.
   *
   * Ignored when `useCssPlaceholder` is off, and never applied to SSR builds,
   * which have no stylesheet of their own by design.
   */
  onMissingCssPlaceholder?: MissingCssPlaceholderReport;
}
