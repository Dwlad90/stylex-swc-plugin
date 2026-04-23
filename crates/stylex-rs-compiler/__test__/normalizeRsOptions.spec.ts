import test from 'ava';
import { normalizeRsOptions } from '../dist/index';
import type { StyleXOptions } from '../dist/index';

const defaultResult: StyleXOptions = {
  dev: false,
  test: false,
  debug: false,
  enableFontSizePxToRem: false,
  enableInlinedConditionalMerge: true,
  enableLegacyValueFlipping: false,
  importSources: ['stylex', '@stylexjs/stylex'],
  runtimeInjection: false,
  treeshakeCompensation: false,
  enableLogicalStylesPolyfill: false,
  enableMinifiedKeys: true,
  styleResolution: 'property-specificity',
  enableLTRRTLComments: false,
  legacyDisableLayers: false,
  useRealFileForSource: true,
  enableMediaQueryOrder: true,
  enableDebugClassNames: false,
  propertyValidationMode: 'silent',
  include: [],
  exclude: [],
  swcPlugins: [],
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
  t.deepEqual(result, defaultResult);
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
  t.deepEqual(result, defaultResult);
});

test('normalizeRsOptions: ignores unrelated fields', t => {
  const input = { foo: 123, bar: 'baz' };
  // @ts-expect-error - input not suitable for normalizeRsOptions
  const result = normalizeRsOptions(input);
  // Unrelated keys pass through (spread-based), defaults are still applied
  t.is(result.dev, false);
  t.deepEqual(result.importSources, ['stylex', '@stylexjs/stylex']);
});

test('normalizeRsOptions: accepts string importSources', t => {
  const input = { importSources: ['foo', 'bar'] };
  const result = normalizeRsOptions(input);
  t.deepEqual(result.importSources, ['foo', 'bar']);
});

test('normalizeRsOptions: accepts object importSources', t => {
  const input = { importSources: [{ as: 'x', from: 'y' }] };
  const result = normalizeRsOptions(input);
  t.deepEqual(result.importSources, [{ as: 'x', from: 'y' }]);
});

test('check default values when input is empty', t => {
  const result = normalizeRsOptions({});
  t.deepEqual(result, defaultResult);
});

test('should throw when input is not provided', t => {
  const error = t.throws(() => normalizeRsOptions(), {
    message: 'Options must be an object, received null/undefined',
  });
  t.truthy(error);
});

test('should throw when input is null', t => {
  const error = t.throws(() => normalizeRsOptions(null), {
    message: 'Options must be an object, received null/undefined',
  });
  t.truthy(error);
});

test('should return default values when input is a string', t => {
  // @ts-expect-error - input must be an object
  const result = normalizeRsOptions('string input');
  t.deepEqual(result, defaultResult);
});

test('normalizeRsOptions: importSources - valid npm string', t => {
  const input = { importSources: ['@scope/pkg', 'foo-bar'] };
  const result = normalizeRsOptions(input);
  t.deepEqual(result.importSources, ['@scope/pkg', 'foo-bar']);
});

test('normalizeRsOptions: importSources - valid object with npm from', t => {
  const input = { importSources: [{ as: 'foo', from: '@scope/pkg' }] };
  const result = normalizeRsOptions(input);
  t.deepEqual(result.importSources, [{ as: 'foo', from: '@scope/pkg' }]);
});

test('normalizeRsOptions: importSources - mixed valid', t => {
  const input = { importSources: ['@scope/pkg', { as: 'foo', from: 'validpath' }] };
  const result = normalizeRsOptions(input);
  t.deepEqual(result.importSources, ['@scope/pkg', { as: 'foo', from: 'validpath' }]);
});

test('normalizeRsOptions: importSources - empty array', t => {
  const input = { importSources: [] };
  const result = normalizeRsOptions(input);
  t.deepEqual(result.importSources, []);
});

test('normalizeRsOptions: styleResolution - default input', t => {
  const input: StyleXOptions = {};
  const result = normalizeRsOptions(input);
  t.is(result.styleResolution, 'property-specificity');
});

test('normalizeRsOptions: styleResolution - valid input', t => {
  const input: StyleXOptions = { styleResolution: 'application-order' };
  const result = normalizeRsOptions(input);
  t.is(result.styleResolution, 'application-order');
});

test('normalizeRsOptions: styleResolution - valid input with legacy-expand-shorthands', t => {
  const input: StyleXOptions = { styleResolution: 'legacy-expand-shorthands' };
  const result = normalizeRsOptions(input);
  t.is(result.styleResolution, 'legacy-expand-shorthands');
});

test('normalizeRsOptions: enableLegacyValueFlipping - true input', t => {
  const result = normalizeRsOptions({ enableLegacyValueFlipping: true });
  t.is(result.enableLegacyValueFlipping, true);
});

test('normalizeRsOptions: enableLegacyValueFlipping - false input', t => {
  const result = normalizeRsOptions({ enableLegacyValueFlipping: false });
  t.is(result.enableLegacyValueFlipping, false);
});

test('normalizeRsOptions: enableLegacyValueFlipping - empty input', t => {
  const result = normalizeRsOptions({});
  t.is(result.enableLegacyValueFlipping, false);
});

test('normalizeRsOptions: enableLTRRTLComments - true input', t => {
  const result = normalizeRsOptions({ enableLTRRTLComments: true });
  t.is(result.enableLTRRTLComments, true);
});

test('normalizeRsOptions: enableLTRRTLComments - false input', t => {
  const result = normalizeRsOptions({ enableLTRRTLComments: false });
  t.is(result.enableLTRRTLComments, false);
});

test('normalizeRsOptions: enableLTRRTLComments - empty input', t => {
  const result = normalizeRsOptions({});
  t.is(result.enableLTRRTLComments, false);
});

test('normalizeRsOptions: true value for runtimeInjection', t => {
  const result = normalizeRsOptions({ runtimeInjection: true });
  t.is(result.runtimeInjection, true);
});

test('normalizeRsOptions: false value for runtimeInjection', t => {
  const result = normalizeRsOptions({ runtimeInjection: false });
  t.is(result.runtimeInjection, false);
});

test('normalizeRsOptions: string value for runtimeInjection', t => {
  const result = normalizeRsOptions({ runtimeInjection: '@test/runtime-injection' });
  t.is(result.runtimeInjection, '@test/runtime-injection');
});

test('normalizeRsOptions: include and exclude default to empty arrays', t => {
  const result = normalizeRsOptions({});
  t.deepEqual(result.include, []);
  t.deepEqual(result.exclude, []);
});

test('normalizeRsOptions: include and exclude are passed through', t => {
  const include = ['src/**/*.tsx'];
  const exclude = [/node_modules/];
  const result = normalizeRsOptions({ include, exclude });
  t.deepEqual(result.include, include);
  t.deepEqual(result.exclude, exclude);
});

test('normalizeRsOptions: swcPlugins default to empty array', t => {
  const result = normalizeRsOptions({});
  t.deepEqual(result.swcPlugins, []);
});

test('normalizeRsOptions: swcPlugins are passed through', t => {
  const swcPlugins: Array<[string, Record<string, unknown>]> = [
    ['@swc/plugin-example', { foo: 'bar' }],
  ];
  const result = normalizeRsOptions({ swcPlugins });
  t.deepEqual(result.swcPlugins, swcPlugins);
});

test('normalizeRsOptions: unstable_moduleResolution is passed through', t => {
  const result = normalizeRsOptions({
    unstable_moduleResolution: { type: 'esm', rootDir: '/app' },
  });
  t.deepEqual(result.unstable_moduleResolution, { type: 'esm', rootDir: '/app' });
});
