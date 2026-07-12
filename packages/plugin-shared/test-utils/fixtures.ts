import path from 'path';

export const FIXTURES_ROOT = path.join(__dirname, '..', 'test-fixtures');

export const BASIC_FIXTURE_ROOT = path.join(FIXTURES_ROOT, 'basic');
export const BASIC_FIXTURE_ENTRY = path.join(BASIC_FIXTURE_ROOT, 'entry.js');

/**
 * Entry variant that additionally imports `plain.css`, a foreign (non-StyleX)
 * stylesheet. Used by the cache-group funnel tests: a widened cacheGroup (no
 * `test`) pulls it into the stylex chunk, and its declaration must survive
 * the asset finalization.
 */
export const PLAIN_CSS_FIXTURE_ENTRY = path.join(BASIC_FIXTURE_ROOT, 'entry-with-plain-css.js');

/** Declaration contributed by the fixture's `plain.css` (unminified builds). */
export const PLAIN_CSS_DECLARATION = 'outline-color: rgb(1, 2, 3)';

/**
 * CSS declarations that must appear in the emitted stylex asset when the
 * `basic` fixture compiles: `App.js` and `Button.js` each contribute rules,
 * so all four appearing proves cross-module rule aggregation.
 */
export const BASIC_FIXTURE_EXPECTED_CSS = [
  'color:red',
  'background-color:blue',
  'padding-top:4px',
  // the default enableFontSizePxToRem converts the authored 16px
  'font-size:1rem',
];

/** Alias used by the fixture entry for the per-bundler carrier stylesheet. */
export const CARRIER_ALIAS = 'stylex-carrier.css';

/**
 * Finds the emitted stylex chunk stylesheet in a map of output file name to
 * file content (e.g. a memfs volume snapshot of the output directory).
 */
export function findStylexCss(files: Record<string, string>, chunkName: string): string | null {
  const fileName = Object.keys(files).find(
    name => name.includes(chunkName) && name.endsWith('.css')
  );

  if (fileName == null) {
    return null;
  }

  return files[fileName]!;
}
