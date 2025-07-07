import test from 'ava';
import { normalizeRsOptions } from '../dist/index';
import type { StyleXOptions } from '../dist/index';

const defaultResult: StyleXOptions = {
  dev: false,
  enableFontSizePxToRem: false,
  enableInlinedConditionalMerge: true,
  importSources: ['stylex', '@stylexjs/stylex'],
  runtimeInjection: false,
  treeshakeCompensation: false,
  unstable_moduleResolution: {
    type: 'commonJS',
  },
  enableLogicalStylesPolyfill: false,
  enableMinifiedKeys: true,
};

test('normalizeRsOptions fills defaults for missing fields', t => {
  const input = {
    dev: undefined,
    enableFontSizePxToRem: undefined,
    runtimeInjection: undefined,
    treeshakeCompensation: undefined,
    importSources: undefined,
    unstable_moduleResolution: undefined,
    enableInlinedConditionalMerge: undefined,
  };
  const result = normalizeRsOptions(input);
  t.deepEqual(result, { ...defaultResult, dev: process.env.NODE_ENV === 'development' });
});

test('normalizeRsOptions preserves provided values', t => {
  const input = {
    dev: true,
    enableFontSizePxToRem: false,
    runtimeInjection: true,
    treeshakeCompensation: false,
    importSources: ['foo', 'bar'],
    unstable_moduleResolution: {
      type: 'esm',
      rootDir: '/tmp',
      themeFileExtension: '.css',
    },
    enableInlinedConditionalMerge: false,
  };
  const expected = {
    ...defaultResult,
    dev: true,
    enableFontSizePxToRem: false,
    runtimeInjection: true,
    treeshakeCompensation: false,
    importSources: ['foo', 'bar'],
    unstable_moduleResolution: {
      type: 'esm',
      rootDir: '/tmp',
      themeFileExtension: '.css',
    },
    enableInlinedConditionalMerge: false,
  };
  const result = normalizeRsOptions(input);
  t.deepEqual(result, expected);
});

test('normalizeRsOptions: handles empty input', t => {
  const result = normalizeRsOptions({});
  t.deepEqual(result, { ...defaultResult, dev: process.env.NODE_ENV === 'development' });
});

test('normalizeRsOptions: ignores unrelated fields', t => {
  const input = { foo: 123, bar: 'baz' };
  // @ts-expect-error - input not suitable for normalizeRsOptions
  const result = normalizeRsOptions(input);
  t.deepEqual(result, { ...defaultResult, dev: process.env.NODE_ENV === 'development' });
});

test('normalizeRsOptions: negative - invalid importSources type', t => {
  const input = { importSources: 123 };
  // @ts-expect-error - input not suitable for normalizeRsOptions
  const error = t.throws(() => normalizeRsOptions(input), {
    message: 'Given napi value is not an array on StyleXOptions.importSources',
  });
  t.truthy(error);
});

test('normalizeRsOptions: negative - invalid unstable_moduleResolution', t => {
  const input = { unstable_moduleResolution: 123 };
  // @ts-expect-error - input not suitable for normalizeRsOptions
  const error = t.throws(() => normalizeRsOptions(input), {
    message: 'Missing field `type` on StyleXOptions.unstable_moduleResolution',
  });
  t.truthy(error);
});

test('normalizeRsOptions: positive - accepts string importSources', t => {
  const input = { importSources: ['foo', 'bar'] };
  const expected = {
    ...defaultResult,
    importSources: ['foo', 'bar'],
    dev: process.env.NODE_ENV === 'development',
  };
  const result = normalizeRsOptions(input);
  t.deepEqual(result, expected);
});

test('normalizeRsOptions: positive - accepts object importSources', t => {
  const input = { importSources: [{ as: 'x', from: 'y' }] };
  const expected = {
    ...defaultResult,
    importSources: [{ as: 'x', from: 'y' }],
    dev: process.env.NODE_ENV === 'development',
  };
  const result = normalizeRsOptions(input);
  t.deepEqual(result, expected);
});

test('normalizeRsOptions: negative - importSources with invalid object', t => {
  const input = { importSources: [{ foo: {} }] };
  // @ts-expect-error - input not suitable for normalizeRsOptions
  const error = t.throws(() => normalizeRsOptions(input), {
    message: 'Import path does not match required pattern on StyleXOptions.importSources',
  });
  t.truthy(error);
});

test('check default values when input is empty', t => {
  const result = normalizeRsOptions({});
  t.deepEqual(result, { ...defaultResult, dev: process.env.NODE_ENV === 'development' });
});

test('should throw when input is not provided', t => {
  // @ts-expect-error - input must be an object
  const error = t.throws(() => normalizeRsOptions(), {
    message: 'Cannot convert undefined or null to object',
  });
  t.truthy(error);
});

test('should throw when input is null', t => {
  // @ts-expect-error - input must be an object
  const error = t.throws(() => normalizeRsOptions(null), {
    message: 'Cannot convert undefined or null to object',
  });
  t.truthy(error);
});

test('should return default values when input is a string', t => {
  // @ts-expect-error - input must be an object
  const result = normalizeRsOptions('string input');
  t.deepEqual(result, { ...defaultResult, dev: process.env.NODE_ENV === 'development' });
});

test('normalizeRsOptions: importSources - valid npm string', t => {
  const input = { importSources: ['@scope/pkg', 'foo-bar', 'a'.repeat(214)] };
  const expected = {
    ...defaultResult,
    importSources: ['@scope/pkg', 'foo-bar', 'a'.repeat(214)],
    dev: process.env.NODE_ENV === 'development',
  };
  const result = normalizeRsOptions(input);
  t.deepEqual(result, expected);
});

test('normalizeRsOptions: importSources - invalid npm string (too long)', t => {
  const input = { importSources: ['a'.repeat(215)] };
  const error = t.throws(() => normalizeRsOptions(input), {
    message: 'Import path is too long (max 214 characters) on StyleXOptions.importSources',
  });
  t.truthy(error);
});

test('normalizeRsOptions: importSources - valid object with npm from', t => {
  const input = { importSources: [{ as: 'foo', from: '@scope/pkg' }] };
  const expected = {
    ...defaultResult,
    importSources: [{ as: 'foo', from: '@scope/pkg' }],
    dev: process.env.NODE_ENV === 'development',
  };
  const result = normalizeRsOptions(input);
  t.deepEqual(result, expected);
});

test('normalizeRsOptions: importSources - invalid object with bad from', t => {
  const input = { importSources: [{ as: 'foo', from: 'not valid path!' }] };
  const error = t.throws(() => normalizeRsOptions(input), {
    message: 'Import path does not match required pattern on StyleXOptions.importSources',
  });
  t.truthy(error);
});

test('normalizeRsOptions: importSources - mixed valid/invalid', t => {
  const input = { importSources: ['@scope/pkg', { as: 'foo', from: 'not valid path!' }] };
  const error = t.throws(() => normalizeRsOptions(input), {
    message: 'Import path does not match required pattern on StyleXOptions.importSources',
  });
  t.truthy(error);
});

test('normalizeRsOptions: importSources - mixed valid', t => {
  const input = { importSources: ['@scope/pkg', { as: 'foo', from: 'validpath' }] };
  const result = normalizeRsOptions(input);
  t.deepEqual(result, {
    ...defaultResult,
    importSources: ['@scope/pkg', { as: 'foo', from: 'validpath' }],
    dev: process.env.NODE_ENV === 'development',
  });
});

test('normalizeRsOptions: importSources - empty array', t => {
  const input = { importSources: [] };
  const expected = {
    ...defaultResult,
    importSources: [],
    dev: process.env.NODE_ENV === 'development',
  };
  const result = normalizeRsOptions(input);
  t.deepEqual(result, expected);
});

test('normalizeRsOptions: importSources - array with null/undefined', t => {
  const input = { importSources: [null, undefined, '@scope/pkg'] };
  // @ts-expect-error - input not suitable for normalizeRsOptions
  const error = t.throws(() => normalizeRsOptions(input), {
    message: 'Cannot convert undefined or null to object',
  });
  t.truthy(error);
});
