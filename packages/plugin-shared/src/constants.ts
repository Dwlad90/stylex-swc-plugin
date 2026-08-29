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

/**
 * Every file extension that the StyleX loader transforms.
 *
 * This list is the one source for the set. A plugin that matches on a path
 * uses `INCLUDE_REGEXP`, which is built from this list. A plugin that must
 * give a glob to its bundler builds the glob from this list too. Nothing
 * repeats the set by hand, so no two plugins can disagree about it.
 *
 * The list is also the cheapest entry point of this package: read it from
 * `@stylexswc/plugin-shared/constants`, which loads no compiler.
 */
export const INCLUDE_EXTENSIONS = ['js', 'jsx', 'mjs', 'cjs', 'ts', 'tsx', 'mts', 'cts'] as const;

/**
 * Path form of `INCLUDE_EXTENSIONS`, built from the list itself. The two forms
 * therefore always agree. The pattern holds only literal names and one anchor,
 * so it reads a path in one pass and cannot backtrack.
 */
export const INCLUDE_REGEXP = new RegExp(`\\.(${INCLUDE_EXTENSIONS.join('|')})$`);

/**
 * Glob form of `INCLUDE_EXTENSIONS`, built from the list itself. All three
 * forms therefore always agree.
 *
 * CSS discovery and a PostCSS config both scan with a glob. A glob that names
 * fewer extensions than the bundler plugins compile makes StyleX compile and
 * the page then get no CSS for it.
 *
 * @param dir - directory to scan, such as `src` or `./app`. Give no directory
 *   to scan from wherever the glob is applied.
 * @returns a glob that matches every transformable file under `dir`
 */
export function buildIncludeGlob(dir?: string): string {
  const extensions = `**/*.{${INCLUDE_EXTENSIONS.join(',')}}`;

  return dir == null || dir === '' ? extensions : `${dir.replace(/\/+$/, '')}/${extensions}`;
}

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
