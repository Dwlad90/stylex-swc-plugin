import { SourceMaps } from '@stylexswc/rs-compiler';
import type { StyleXOptions } from '@stylexswc/rs-compiler';

/**
 * The source-map decisions a bundler loader makes on the compiler's behalf.
 *
 * Kept in its own module — and out of the package entry — so a loader can pull
 * it in without also loading the plugin core and its webpack dependencies.
 */
export type ResolvedSourceMapOptions = Pick<StyleXOptions, 'sourceMap' | 'inlineSourcesContent'>;

/**
 * Decide `sourceMap` and `inlineSourcesContent` for a loader run.
 *
 * Both are resolved from the host's own source-map setting rather than left to
 * the compiler defaults, and both yield to an explicit `rsOptions` value. The
 * result is meant to be spread over `normalizeRsOptions(...)` output; a
 * `sourceMap` of `undefined` deliberately means "leave the compiler default
 * alone", which is safe because `normalizeRsOptions` has no default for it.
 *
 * @param rsOptions the caller's raw (un-normalized) options — explicit values
 *   here always win
 * @param sourceMapsEnabled the host's setting, i.e. webpack's `this.sourceMap`.
 *   `undefined` means the host never told us
 */
export function resolveSourceMapOptions(
  rsOptions: Partial<StyleXOptions>,
  sourceMapsEnabled?: boolean
): ResolvedSourceMapOptions {
  // The bundler drops the emitted map when its own `devtool` is off, so
  // building one is pure waste. Only an explicit `false` disables it:
  // `undefined` means the host never told us — Turbopack's loader context is a
  // partial webpack shim and may not define `this.sourceMap` — and treating
  // that as "off" would silently strip source maps from every file.
  const sourceMap =
    rsOptions.sourceMap ?? (sourceMapsEnabled === false ? SourceMaps.False : undefined);

  // `sourcesContent` embeds the entire authored file in the map. That is what
  // Chrome DevTools needs in development — without it, it re-fetches the source
  // over `webpack-internal://` and fails — but in a production build it is pure
  // payload, and it publishes the source to anyone who can reach the `.map`.
  //
  // Hence the default is the build mode rather than the compiler's own `true`.
  // The asymmetry with `sourceMap` above is deliberate: a map with no
  // `sourcesContent` still degrades gracefully — the bundler re-supplies the
  // text from the module source, and a production `sources` URL is fetchable
  // over http — whereas a missing map degrades to nothing at all. So "the host
  // never told us" keeps the map and drops the inlined text.
  const inlineSourcesContent =
    rsOptions.inlineSourcesContent ?? rsOptions.dev ?? process.env.NODE_ENV !== 'production';

  return { sourceMap, inlineSourcesContent };
}
