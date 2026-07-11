export const PLUGIN_NAME = 'stylex';

/**
 * Appended by the stylex-loader after a successful transform so a module that
 * is fed through the loader chain more than once (observed with Next.js App
 * Router) is never transformed twice.
 */
export const LOADER_TRANSFORMED_FLAG = '/* [@stylexswc] stylex-loader transformed */';

/**
 * Physical carrier stylesheet. Consumers import it once at their application
 * entrypoint; the plugin replaces the emitted asset content with the
 * extracted StyleX CSS during `processAssets`.
 */
export const VIRTUAL_ENTRYPOINT_CSS_PATH = require.resolve('./stylex.css');

/**
 * Physical target of the per-module dummy imports appended by the
 * stylex-loader. Only used to invalidate HMR in development; carries the
 * serialized rules in its resource query.
 */
export const VIRTUAL_STYLEX_CSS_DUMMY_IMPORT_PATH = require.resolve('./stylex-virtual.css');

/** Matches both the carrier and the dummy imports (splitChunks cacheGroup test). */
export const VIRTUAL_CSS_PATTERN = /stylex\.css|stylex-virtual\.css/;

/**
 * Matches only the carrier stylesheet by path. The wrapper plugins ship their
 * own `stylex.css` copy (npm `exports` cannot point across packages), so this
 * is a filename pattern rather than an exact-path comparison.
 */
export const VIRTUAL_ENTRYPOINT_CSS_PATTERN = /[\\/]stylex\.css$/;

/** Matches only the per-module HMR dummy imports. */
export const VIRTUAL_STYLEX_CSS_DUMMY_IMPORT_PATTERN = /stylex-virtual\.css/;

export const INCLUDE_REGEXP = /\.[cm]?[jt]sx?$/;

/**
 * Key under which the stylex-loader stores extracted rules on
 * `module.buildInfo`. webpack persists `buildInfo` in its filesystem cache, so
 * rules survive cached rebuilds where the loader doesn't re-run.
 */
export const BUILD_INFO_STYLEX_KEY = '~stylexswc_stylex_rules';

/**
 * node_modules packages that ship untransformed StyleX source and must go
 * through the stylex-loader even though node_modules is excluded by default
 */
export const DEFAULT_STYLEX_PACKAGES = ['@stylexjs/'];

// https://github.com/vercel/next.js/blob/canary/packages/next/src/shared/lib/constants.ts
export const NEXTJS_COMPILER_NAMES = {
  client: 'client',
  server: 'server',
  edgeServer: 'edge-server',
} as const;

export type NextJsCompilerName = (typeof NEXTJS_COMPILER_NAMES)[keyof typeof NEXTJS_COMPILER_NAMES];

export function isNextJsCompilerName(name: string | undefined): name is NextJsCompilerName {
  if (name == null) {
    return false;
  }

  return Object.values(NEXTJS_COMPILER_NAMES).includes(name as NextJsCompilerName);
}
