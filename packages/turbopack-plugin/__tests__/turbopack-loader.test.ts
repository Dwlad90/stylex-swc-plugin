import { describe, expect, test } from 'vitest';

import stylexTurbopackLoader from '../src/turbopack-loader';
import type { StyleXTurbopackLoaderOptions } from '../src/types';

type LoaderOptions = Partial<StyleXTurbopackLoaderOptions>;

// The source text holds no `stylex`, so only a configured import source can
// select it, and only a compiler that knows that source can transform it.
const CUSTOM_SOURCE_MODULE = [
  "import * as css from 'design-system';",
  "export const styles = css.create({ red: { color: 'red' } });",
].join('\n');

/** Runs the loader once and returns the code it handed back. */
async function runLoader(options: LoaderOptions, sourceCode: string): Promise<string> {
  let output: unknown;

  const context = {
    resourcePath: '/project/src/styles.js',
    getOptions: () => options,
    async:
      () =>
      (error: Error | null | undefined, code?: unknown): void => {
        if (error) {
          throw error;
        }

        output = code;
      },
  };

  await stylexTurbopackLoader.call(
    context as unknown as ThisParameterType<typeof stylexTurbopackLoader>,
    sourceCode,
    undefined
  );

  return String(output);
}

describe('stylexTurbopackLoader', () => {
  const rsOptions = { dev: false, runtimeInjection: false };

  test('transforms a module that imports from a source set in stylexImports', async () => {
    const code = await runLoader(
      { stylexImports: ['design-system'], rsOptions },
      CUSTOM_SOURCE_MODULE
    );

    expect(code).not.toContain('css.create');
  });

  test('transforms a module that imports from a source set in rsOptions', async () => {
    const code = await runLoader(
      { rsOptions: { ...rsOptions, importSources: ['design-system'] } },
      CUSTOM_SOURCE_MODULE
    );

    expect(code).not.toContain('css.create');
  });

  test('returns a module that mentions no import source unchanged', async () => {
    const sourceCode = 'export const answer = 42;';

    await expect(runLoader({ rsOptions }, sourceCode)).resolves.toBe(sourceCode);
  });
});
