import path from 'node:path';

import { describe, expect, it } from '@jest/globals';
import type { StyleXOptions } from '@stylexswc/rs-compiler';

// Deliberately the built CommonJS artifact rather than `../src`: this package
// ships as a Jest transformer loaded by Jest via `require`, so the artifact is
// the real public contract.
//
// The `as typeof import('../src/index')` is what keeps this from being an
// untyped call site. `@swc/jest` strips types without checking them, so an
// untyped `require` would leave every assertion below compiling against `any`
// — renaming an export or changing an arity would sail past `pnpm typecheck`
// and only surface as a runtime TypeError. The cast costs nothing at runtime,
// still loads `dist`, and makes `tsgo` check each call against the real
// signatures. Source-versus-declaration drift is a different question, and is
// covered by `attw` in `check:artifacts`.
// oxlint-disable-next-line typescript/no-require-imports
const { createTransformer } = require('../dist/index.js') as typeof import('../src/index');

/**
 * Jest hands `process` a fully populated options object; these tests supply
 * only the field under test. Narrowed once, here, rather than at each call.
 */
const asTransformOptions = (partial: { transformerConfig: { rsOptions?: StyleXOptions } }) =>
  partial as unknown as Parameters<ReturnType<typeof createTransformer>['process']>[2];

const STYLEX_SOURCE = `
import * as stylex from '@stylexjs/stylex';

const styles = stylex.create({
  root: { color: 'red' },
});

export default styles;
`;

/** Jest resolves include/exclude patterns relative to the working directory. */
const underCwd = (relative: string) => path.join(process.cwd(), relative);

const optionsWith = (rsOptions: StyleXOptions = {}) =>
  asTransformOptions({ transformerConfig: { rsOptions } });

describe('createTransformer', () => {
  it('exposes the Jest transformer contract', () => {
    const transformer = createTransformer();

    expect(typeof transformer.process).toBe('function');
    expect(typeof transformer.processAsync).toBe('function');
    expect(typeof transformer.getCacheKey).toBe('function');
  });
});

describe('process', () => {
  it('transforms a StyleX source file', () => {
    const { process: transform } = createTransformer();
    const filePath = underCwd('src/Button.tsx');

    const { code } = transform(STYLEX_SOURCE, filePath, optionsWith());

    expect(code).not.toBe(STYLEX_SOURCE);
    // `stylex.create` is compiled away into static class names.
    expect(code).not.toContain('stylex.create');
  });

  it('returns the source unchanged when a file is excluded', () => {
    const { process: transform } = createTransformer();
    const filePath = underCwd('src/Button.tsx');

    const { code } = transform(STYLEX_SOURCE, filePath, optionsWith({ exclude: ['src/**'] }));

    expect(code).toBe(STYLEX_SOURCE);
  });

  it('returns the source unchanged when a file falls outside include', () => {
    const { process: transform } = createTransformer();
    const filePath = underCwd('lib/Button.tsx');

    const { code } = transform(STYLEX_SOURCE, filePath, optionsWith({ include: ['src/**'] }));

    expect(code).toBe(STYLEX_SOURCE);
  });

  it('transforms a file that matches include', () => {
    const { process: transform } = createTransformer();
    const filePath = underCwd('src/Button.tsx');

    const { code } = transform(STYLEX_SOURCE, filePath, optionsWith({ include: ['src/**'] }));

    expect(code).not.toBe(STYLEX_SOURCE);
  });

  // Jest reuses one transformer instance, and one options object, for every
  // file in a run. Filtering must therefore survive earlier files.
  it('keeps applying exclude after an included file was transformed', () => {
    const { process: transform } = createTransformer();
    const options = optionsWith({ exclude: ['src/ignored/**'] });

    const transformed = transform(STYLEX_SOURCE, underCwd('src/Button.tsx'), options);
    expect(transformed.code).not.toBe(STYLEX_SOURCE);

    const skipped = transform(STYLEX_SOURCE, underCwd('src/ignored/Button.tsx'), options);
    expect(skipped.code).toBe(STYLEX_SOURCE);
  });

  it('keeps applying include after an included file was transformed', () => {
    const { process: transform } = createTransformer();
    const options = optionsWith({ include: ['src/**'] });

    const transformed = transform(STYLEX_SOURCE, underCwd('src/Button.tsx'), options);
    expect(transformed.code).not.toBe(STYLEX_SOURCE);

    const skipped = transform(STYLEX_SOURCE, underCwd('lib/Button.tsx'), options);
    expect(skipped.code).toBe(STYLEX_SOURCE);
  });
});

describe('processAsync', () => {
  it('produces the same result as process', async () => {
    const { process: transform, processAsync } = createTransformer();
    const filePath = underCwd('src/Button.tsx');

    const sync = transform(STYLEX_SOURCE, filePath, optionsWith());
    const async = await processAsync(STYLEX_SOURCE, filePath, optionsWith());

    expect(async.code).toBe(sync.code);
  });
});

describe('getCacheKey', () => {
  const filePath = underCwd('src/Button.tsx');

  it('is deterministic for identical inputs', () => {
    const { getCacheKey } = createTransformer();

    expect(getCacheKey(STYLEX_SOURCE, filePath, optionsWith())).toBe(
      getCacheKey(STYLEX_SOURCE, filePath, optionsWith())
    );
  });

  it('changes when the source changes', () => {
    const { getCacheKey } = createTransformer();

    expect(getCacheKey(STYLEX_SOURCE, filePath, optionsWith())).not.toBe(
      getCacheKey(`${STYLEX_SOURCE}\n// edited`, filePath, optionsWith())
    );
  });

  it('changes when the file path changes', () => {
    const { getCacheKey } = createTransformer();

    expect(getCacheKey(STYLEX_SOURCE, filePath, optionsWith())).not.toBe(
      getCacheKey(STYLEX_SOURCE, underCwd('src/Card.tsx'), optionsWith())
    );
  });

  it('changes when the transformer config changes', () => {
    const { getCacheKey } = createTransformer();

    expect(getCacheKey(STYLEX_SOURCE, filePath, optionsWith())).not.toBe(
      getCacheKey(STYLEX_SOURCE, filePath, optionsWith({ dev: true }))
    );
  });

  // The key mixes in the size and mtime of the native addon so that rebuilding
  // the Rust crate — which does not move `package.json` — still invalidates the
  // cache. Finding that addon relies on NAPI-RS naming it
  // `rs-compiler.<triple>.node`. If that convention ever changes, the stamp
  // degrades to an empty string and the staleness returns with no other signal,
  // so the convention itself is what is asserted here.
  it('can locate the native addon its cache key is stamped from', () => {
    const addonPath = Object.keys(require.cache).find(id => {
      const base = path.basename(id);
      return base.startsWith('rs-compiler.') && base.endsWith('.node');
    });

    expect(addonPath).toBeDefined();
  });
});
