import test from 'ava';
import { normalizeRsOptions } from '../dist/index.js';
import { SourceMaps, PropertyValidationMode } from '../dist/transform.js';
import type { StyleXOptions } from '../dist/index.js';

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

test('normalizeRsOptions: preserves all TS-only fields together', t => {
  const include = ['src/**/*.tsx', 'app/**/*.ts'];
  const exclude = [/node_modules/, /\.test\./];
  const swcPlugins: Array<[string, Record<string, unknown>]> = [
    ['@swc/plugin-example', { foo: 'bar' }],
    ['@swc/plugin-other', {}],
  ];
  const result = normalizeRsOptions({ include, exclude, swcPlugins, dev: true });
  t.deepEqual(result.include, include);
  t.deepEqual(result.exclude, exclude);
  t.deepEqual(result.swcPlugins, swcPlugins);
  t.is(result.dev, true);
});

test('normalizeRsOptions: TS-only fields are preserved with RegExp instances', t => {
  const regexInclude = /src\/.*\.tsx$/;
  const regexExclude = /node_modules/;
  const result = normalizeRsOptions({
    include: [regexInclude],
    exclude: [regexExclude],
  });
  t.is(result.include![0], regexInclude);
  t.is(result.exclude![0], regexExclude);
});

test('normalizeRsOptions: preserves debugFilePath function', t => {
  // debugFilePath is not in the TS-only fields type, but it passes through
  // as a native option. Test that it is preserved.
  const debugFilePath = (filename: string) => filename;
  const result = normalizeRsOptions({ dev: true, debugFilePath });
  t.is(result.dev, true);
  t.is(result.debugFilePath, debugFilePath);
});

test('normalizeRsOptions: multiple boolean options override defaults correctly', t => {
  const result = normalizeRsOptions({
    dev: true,
    test: true,
    debug: true,
    enableFontSizePxToRem: true,
    enableMinifiedKeys: false,
    enableInlinedConditionalMerge: false,
  });
  t.is(result.dev, true);
  t.is(result.test, true);
  t.is(result.debug, true);
  t.is(result.enableFontSizePxToRem, true);
  t.is(result.enableMinifiedKeys, false);
  t.is(result.enableInlinedConditionalMerge, false);
});

// ── Edge cases and advanced scenarios ──────────────────────────────

test('normalizeRsOptions: classNamePrefix is preserved', t => {
  const result = normalizeRsOptions({ classNamePrefix: 'x' });
  t.is(result.classNamePrefix, 'x');
});

test('normalizeRsOptions: classNamePrefix with empty string', t => {
  const result = normalizeRsOptions({ classNamePrefix: '' });
  t.is(result.classNamePrefix, '');
});

test('normalizeRsOptions: aliases are preserved', t => {
  const aliases = { '@components/*': ['src/components/*'] };
  const result = normalizeRsOptions({ aliases });
  t.deepEqual(result.aliases, aliases);
});

test('normalizeRsOptions: definedStylexCssVariables are preserved', t => {
  const vars = { '--primary': 'blue', '--secondary': 'red' };
  const result = normalizeRsOptions({ definedStylexCssVariables: vars });
  t.deepEqual(result.definedStylexCssVariables, vars);
});

test('normalizeRsOptions: sourceMap values are preserved', t => {
  t.is(normalizeRsOptions({}).sourceMap, undefined);
  t.is(normalizeRsOptions({ sourceMap: SourceMaps.True }).sourceMap, SourceMaps.True);
  t.is(normalizeRsOptions({ sourceMap: SourceMaps.False }).sourceMap, SourceMaps.False);
  t.is(normalizeRsOptions({ sourceMap: SourceMaps.Inline }).sourceMap, SourceMaps.Inline);
});

test('normalizeRsOptions: propertyValidationMode overrides default', t => {
  t.is(normalizeRsOptions({}).propertyValidationMode, PropertyValidationMode.Silent);
  t.is(
    normalizeRsOptions({ propertyValidationMode: PropertyValidationMode.Throw })
      .propertyValidationMode,
    PropertyValidationMode.Throw
  );
  t.is(
    normalizeRsOptions({ propertyValidationMode: PropertyValidationMode.Warn })
      .propertyValidationMode,
    PropertyValidationMode.Warn
  );
  t.is(
    normalizeRsOptions({ propertyValidationMode: PropertyValidationMode.Silent })
      .propertyValidationMode,
    PropertyValidationMode.Silent
  );
});

test('normalizeRsOptions: mixed include patterns (string and RegExp)', t => {
  const include = ['src/**/*.ts', /components\/.*\.tsx$/];
  const result = normalizeRsOptions({ include });
  t.is(result.include!.length, 2);
  t.is(result.include![0], 'src/**/*.ts');
  t.true(result.include![1] instanceof RegExp);
});

test('normalizeRsOptions: mixed exclude patterns (string and RegExp)', t => {
  const exclude = ['node_modules/**', /\.test\./];
  const result = normalizeRsOptions({ exclude });
  t.is(result.exclude!.length, 2);
  t.is(result.exclude![0], 'node_modules/**');
  t.true(result.exclude![1] instanceof RegExp);
});

test('normalizeRsOptions: explicit false values are not stripped', t => {
  const result = normalizeRsOptions({
    dev: false,
    test: false,
    debug: false,
    runtimeInjection: false,
    treeshakeCompensation: false,
  });
  t.is(result.dev, false);
  t.is(result.test, false);
  t.is(result.debug, false);
  t.is(result.runtimeInjection, false);
  t.is(result.treeshakeCompensation, false);
});

test('normalizeRsOptions: explicit 0 / empty string are not stripped', t => {
  const result = normalizeRsOptions({ classNamePrefix: '', runtimeInjection: '' });
  t.is(result.classNamePrefix, '');
  t.is(result.runtimeInjection, '');
});

test('normalizeRsOptions: undefined fields do not clobber defaults', t => {
  const result = normalizeRsOptions({
    dev: undefined,
    test: undefined,
    enableFontSizePxToRem: undefined,
    styleResolution: undefined,
  });
  t.is(result.dev, false);
  t.is(result.test, false);
  t.is(result.enableFontSizePxToRem, false);
  t.is(result.styleResolution, 'property-specificity');
});

test('normalizeRsOptions: enableLogicalStylesPolyfill default and override', t => {
  t.is(normalizeRsOptions({}).enableLogicalStylesPolyfill, false);
  t.is(normalizeRsOptions({ enableLogicalStylesPolyfill: true }).enableLogicalStylesPolyfill, true);
});

test('normalizeRsOptions: enableMediaQueryOrder default and override', t => {
  t.is(normalizeRsOptions({}).enableMediaQueryOrder, true);
  t.is(normalizeRsOptions({ enableMediaQueryOrder: false }).enableMediaQueryOrder, false);
});

test('normalizeRsOptions: legacyDisableLayers default and override', t => {
  t.is(normalizeRsOptions({}).legacyDisableLayers, false);
  t.is(normalizeRsOptions({ legacyDisableLayers: true }).legacyDisableLayers, true);
});

test('normalizeRsOptions: useRealFileForSource default and override', t => {
  t.is(normalizeRsOptions({}).useRealFileForSource, true);
  t.is(normalizeRsOptions({ useRealFileForSource: false }).useRealFileForSource, false);
});

test('normalizeRsOptions: enableDebugClassNames default and override', t => {
  t.is(normalizeRsOptions({}).enableDebugClassNames, false);
  t.is(normalizeRsOptions({ enableDebugClassNames: true }).enableDebugClassNames, true);
});

test('normalizeRsOptions: many swcPlugins are passed through', t => {
  const swcPlugins: Array<[string, Record<string, unknown>]> = [
    ['@swc/plugin-a', { opt: 1 }],
    ['@swc/plugin-b', { opt: 2 }],
    ['@swc/plugin-c', {}],
  ];
  const result = normalizeRsOptions({ swcPlugins });
  t.is(result.swcPlugins!.length, 3);
  t.deepEqual(result.swcPlugins, swcPlugins);
});

test('normalizeRsOptions: unstable_moduleResolution with all fields', t => {
  const result = normalizeRsOptions({
    unstable_moduleResolution: {
      type: 'commonJS',
      rootDir: '/project',
      themeFileExtension: '.stylex.ts',
    },
  });
  t.deepEqual(result.unstable_moduleResolution, {
    type: 'commonJS',
    rootDir: '/project',
    themeFileExtension: '.stylex.ts',
  });
});

test('normalizeRsOptions: unstable_moduleResolution with minimal fields', t => {
  const result = normalizeRsOptions({
    unstable_moduleResolution: { type: 'esm' },
  });
  t.is(result.unstable_moduleResolution!.type, 'esm');
  t.is(result.unstable_moduleResolution!.rootDir, undefined);
});

test('normalizeRsOptions: all defaults are correct', t => {
  const result = normalizeRsOptions({});
  // Verify every default value
  t.is(result.dev, false);
  t.is(result.test, false);
  t.is(result.debug, false);
  t.is(result.enableFontSizePxToRem, false);
  t.is(result.runtimeInjection, false);
  t.is(result.treeshakeCompensation, false);
  t.is(result.enableInlinedConditionalMerge, true);
  t.is(result.enableLogicalStylesPolyfill, false);
  t.is(result.enableMinifiedKeys, true);
  t.is(result.enableLegacyValueFlipping, false);
  t.is(result.enableLTRRTLComments, false);
  t.is(result.legacyDisableLayers, false);
  t.is(result.useRealFileForSource, true);
  t.is(result.enableMediaQueryOrder, true);
  t.is(result.enableDebugClassNames, false);
  t.is(result.propertyValidationMode, 'silent');
  t.is(result.styleResolution, 'property-specificity');
  t.deepEqual(result.importSources, ['stylex', '@stylexjs/stylex']);
  t.deepEqual(result.include, []);
  t.deepEqual(result.exclude, []);
  t.deepEqual(result.swcPlugins, []);
});

test('normalizeRsOptions: number input treated as empty object', t => {
  // @ts-expect-error - testing invalid input
  const result = normalizeRsOptions(42);
  t.deepEqual(result, defaultResult);
});

test('normalizeRsOptions: boolean input treated as empty object', t => {
  // @ts-expect-error - testing invalid input
  const result = normalizeRsOptions(true);
  t.deepEqual(result, defaultResult);
});
