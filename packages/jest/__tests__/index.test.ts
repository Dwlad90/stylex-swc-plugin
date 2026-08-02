import path from 'node:path';

import { describe, expect, it } from '@jest/globals';

// Deliberately the built CommonJS artifact rather than `../src`: this package
// ships as a Jest transformer loaded by Jest via `require`, so the artifact is
// the real public contract.
// oxlint-disable-next-line typescript/no-require-imports
const { createTransformer } = require('../dist/index.js');

type TransformOptions = {
  transformerConfig: { rsOptions?: Record<string, unknown> };
};

const STYLEX_SOURCE = `
import * as stylex from '@stylexjs/stylex';

const styles = stylex.create({
  root: { color: 'red' },
});

export default styles;
`;

/** Jest resolves include/exclude patterns relative to the working directory. */
const underCwd = (relative: string) => path.join(process.cwd(), relative);

const optionsWith = (rsOptions: Record<string, unknown> = {}): TransformOptions => ({
  transformerConfig: { rsOptions },
});

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
});
