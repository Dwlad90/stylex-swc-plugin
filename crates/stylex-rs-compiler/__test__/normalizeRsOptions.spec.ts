import { expect, test } from 'vitest';
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

test('normalizeRsOptions fills defaults for missing fields', () => {
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
  expect(result).toEqual(defaultResult);
});

test('normalizeRsOptions preserves provided values', () => {
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
  expect(result).toEqual(expected);
});

test('normalizeRsOptions: handles empty input', () => {
  const result = normalizeRsOptions({});
  expect(result).toEqual(defaultResult);
});

test('normalizeRsOptions: ignores unrelated fields', () => {
  const input = { foo: 123, bar: 'baz' };
  // @ts-expect-error - input not suitable for normalizeRsOptions
  const result = normalizeRsOptions(input);
  // Unrelated keys pass through (spread-based), defaults are still applied
  expect(result.dev).toBe(false);
  expect(result.importSources).toEqual(['stylex', '@stylexjs/stylex']);
});

test('normalizeRsOptions: accepts string importSources', () => {
  const input = { importSources: ['foo', 'bar'] };
  const result = normalizeRsOptions(input);
  expect(result.importSources).toEqual(['foo', 'bar']);
});

test('normalizeRsOptions: accepts object importSources', () => {
  const input = { importSources: [{ as: 'x', from: 'y' }] };
  const result = normalizeRsOptions(input);
  expect(result.importSources).toEqual([{ as: 'x', from: 'y' }]);
});

test('check default values when input is empty', () => {
  const result = normalizeRsOptions({});
  expect(result).toEqual(defaultResult);
});

test('should throw when input is not provided', () => {
  expect(() => normalizeRsOptions()).toThrow(
    'Options must be an object, received null/undefined'
  );
});

test('should throw when input is null', () => {
  expect(() => normalizeRsOptions(null)).toThrow(
    'Options must be an object, received null/undefined'
  );
});

test('should return default values when input is a string', () => {
  // @ts-expect-error - input must be an object
  const result = normalizeRsOptions('string input');
  expect(result).toEqual(defaultResult);
});

test('normalizeRsOptions: importSources - valid npm string', () => {
  const input = { importSources: ['@scope/pkg', 'foo-bar'] };
  const result = normalizeRsOptions(input);
  expect(result.importSources).toEqual(['@scope/pkg', 'foo-bar']);
});

test('normalizeRsOptions: importSources - valid object with npm from', () => {
  const input = { importSources: [{ as: 'foo', from: '@scope/pkg' }] };
  const result = normalizeRsOptions(input);
  expect(result.importSources).toEqual([{ as: 'foo', from: '@scope/pkg' }]);
});

test('normalizeRsOptions: importSources - mixed valid', () => {
  const input = { importSources: ['@scope/pkg', { as: 'foo', from: 'validpath' }] };
  const result = normalizeRsOptions(input);
  expect(result.importSources).toEqual(['@scope/pkg', { as: 'foo', from: 'validpath' }]);
});

test('normalizeRsOptions: importSources - empty array', () => {
  const input = { importSources: [] };
  const result = normalizeRsOptions(input);
  expect(result.importSources).toEqual([]);
});

test('normalizeRsOptions: styleResolution - default input', () => {
  const input: StyleXOptions = {};
  const result = normalizeRsOptions(input);
  expect(result.styleResolution).toBe('property-specificity');
});

test('normalizeRsOptions: styleResolution - valid input', () => {
  const input: StyleXOptions = { styleResolution: 'application-order' };
  const result = normalizeRsOptions(input);
  expect(result.styleResolution).toBe('application-order');
});

test('normalizeRsOptions: styleResolution - valid input with legacy-expand-shorthands', () => {
  const input: StyleXOptions = { styleResolution: 'legacy-expand-shorthands' };
  const result = normalizeRsOptions(input);
  expect(result.styleResolution).toBe('legacy-expand-shorthands');
});

test('normalizeRsOptions: enableLegacyValueFlipping - true input', () => {
  const result = normalizeRsOptions({ enableLegacyValueFlipping: true });
  expect(result.enableLegacyValueFlipping).toBe(true);
});

test('normalizeRsOptions: enableLegacyValueFlipping - false input', () => {
  const result = normalizeRsOptions({ enableLegacyValueFlipping: false });
  expect(result.enableLegacyValueFlipping).toBe(false);
});

test('normalizeRsOptions: enableLegacyValueFlipping - empty input', () => {
  const result = normalizeRsOptions({});
  expect(result.enableLegacyValueFlipping).toBe(false);
});

test('normalizeRsOptions: enableLTRRTLComments - true input', () => {
  const result = normalizeRsOptions({ enableLTRRTLComments: true });
  expect(result.enableLTRRTLComments).toBe(true);
});

test('normalizeRsOptions: enableLTRRTLComments - false input', () => {
  const result = normalizeRsOptions({ enableLTRRTLComments: false });
  expect(result.enableLTRRTLComments).toBe(false);
});

test('normalizeRsOptions: enableLTRRTLComments - empty input', () => {
  const result = normalizeRsOptions({});
  expect(result.enableLTRRTLComments).toBe(false);
});

test('normalizeRsOptions: true value for runtimeInjection', () => {
  const result = normalizeRsOptions({ runtimeInjection: true });
  expect(result.runtimeInjection).toBe(true);
});

test('normalizeRsOptions: false value for runtimeInjection', () => {
  const result = normalizeRsOptions({ runtimeInjection: false });
  expect(result.runtimeInjection).toBe(false);
});

test('normalizeRsOptions: string value for runtimeInjection', () => {
  const result = normalizeRsOptions({ runtimeInjection: '@test/runtime-injection' });
  expect(result.runtimeInjection).toBe('@test/runtime-injection');
});

test('normalizeRsOptions: include and exclude default to empty arrays', () => {
  const result = normalizeRsOptions({});
  expect(result.include).toEqual([]);
  expect(result.exclude).toEqual([]);
});

test('normalizeRsOptions: include and exclude are passed through', () => {
  const include = ['src/**/*.tsx'];
  const exclude = [/node_modules/];
  const result = normalizeRsOptions({ include, exclude });
  expect(result.include).toEqual(include);
  expect(result.exclude).toEqual(exclude);
});

test('normalizeRsOptions: swcPlugins default to empty array', () => {
  const result = normalizeRsOptions({});
  expect(result.swcPlugins).toEqual([]);
});

test('normalizeRsOptions: swcPlugins are passed through', () => {
  const swcPlugins: Array<[string, Record<string, unknown>]> = [
    ['@swc/plugin-example', { foo: 'bar' }],
  ];
  const result = normalizeRsOptions({ swcPlugins });
  expect(result.swcPlugins).toEqual(swcPlugins);
});

test('normalizeRsOptions: unstable_moduleResolution is passed through', () => {
  const result = normalizeRsOptions({
    unstable_moduleResolution: { type: 'esm', rootDir: '/app' },
  });
  expect(result.unstable_moduleResolution).toEqual({ type: 'esm', rootDir: '/app' });
});

test('normalizeRsOptions: preserves all TS-only fields together', () => {
  const include = ['src/**/*.tsx', 'app/**/*.ts'];
  const exclude = [/node_modules/, /\.test\./];
  const swcPlugins: Array<[string, Record<string, unknown>]> = [
    ['@swc/plugin-example', { foo: 'bar' }],
    ['@swc/plugin-other', {}],
  ];
  const result = normalizeRsOptions({ include, exclude, swcPlugins, dev: true });
  expect(result.include).toEqual(include);
  expect(result.exclude).toEqual(exclude);
  expect(result.swcPlugins).toEqual(swcPlugins);
  expect(result.dev).toBe(true);
});

test('normalizeRsOptions: TS-only fields are preserved with RegExp instances', () => {
  const regexInclude = /src\/.*\.tsx$/;
  const regexExclude = /node_modules/;
  const result = normalizeRsOptions({
    include: [regexInclude],
    exclude: [regexExclude],
  });
  expect(result.include![0]).toBe(regexInclude);
  expect(result.exclude![0]).toBe(regexExclude);
});

test('normalizeRsOptions: preserves debugFilePath function', () => {
  // debugFilePath is not in the TS-only fields type, but it passes through
  // as a native option. Test that it is preserved.
  const debugFilePath = (filename: string) => filename;
  const result = normalizeRsOptions({ dev: true, debugFilePath });
  expect(result.dev).toBe(true);
  expect(result.debugFilePath).toBe(debugFilePath);
});

test('normalizeRsOptions: multiple boolean options override defaults correctly', () => {
  const result = normalizeRsOptions({
    dev: true,
    test: true,
    debug: true,
    enableFontSizePxToRem: true,
    enableMinifiedKeys: false,
    enableInlinedConditionalMerge: false,
  });
  expect(result.dev).toBe(true);
  expect(result.test).toBe(true);
  expect(result.debug).toBe(true);
  expect(result.enableFontSizePxToRem).toBe(true);
  expect(result.enableMinifiedKeys).toBe(false);
  expect(result.enableInlinedConditionalMerge).toBe(false);
});

// ── Edge cases and advanced scenarios ──────────────────────────────

test('normalizeRsOptions: classNamePrefix is preserved', () => {
  const result = normalizeRsOptions({ classNamePrefix: 'x' });
  expect(result.classNamePrefix).toBe('x');
});

test('normalizeRsOptions: classNamePrefix with empty string', () => {
  const result = normalizeRsOptions({ classNamePrefix: '' });
  expect(result.classNamePrefix).toBe('');
});

test('normalizeRsOptions: aliases are preserved', () => {
  const aliases = { '@components/*': ['src/components/*'] };
  const result = normalizeRsOptions({ aliases });
  expect(result.aliases).toEqual(aliases);
});

test('normalizeRsOptions: definedStylexCssVariables are preserved', () => {
  const vars = { '--primary': 'blue', '--secondary': 'red' };
  const result = normalizeRsOptions({ definedStylexCssVariables: vars });
  expect(result.definedStylexCssVariables).toEqual(vars);
});

test('normalizeRsOptions: sourceMap values are preserved', () => {
  expect(normalizeRsOptions({}).sourceMap).toBe(undefined);
  expect(normalizeRsOptions({ sourceMap: SourceMaps.True }).sourceMap).toBe(SourceMaps.True);
  expect(normalizeRsOptions({ sourceMap: SourceMaps.False }).sourceMap).toBe(SourceMaps.False);
  expect(normalizeRsOptions({ sourceMap: SourceMaps.Inline }).sourceMap).toBe(SourceMaps.Inline);
});

test('normalizeRsOptions: propertyValidationMode overrides default', () => {
  expect(normalizeRsOptions({}).propertyValidationMode).toBe(PropertyValidationMode.Silent);
  expect(normalizeRsOptions({ propertyValidationMode: PropertyValidationMode.Throw })
      .propertyValidationMode).toBe(PropertyValidationMode.Throw);
  expect(normalizeRsOptions({ propertyValidationMode: PropertyValidationMode.Warn })
      .propertyValidationMode).toBe(PropertyValidationMode.Warn);
  expect(normalizeRsOptions({ propertyValidationMode: PropertyValidationMode.Silent })
      .propertyValidationMode).toBe(PropertyValidationMode.Silent);
});

test('normalizeRsOptions: mixed include patterns (string and RegExp)', () => {
  const include = ['src/**/*.ts', /components\/.*\.tsx$/];
  const result = normalizeRsOptions({ include });
  expect(result.include!.length).toBe(2);
  expect(result.include![0]).toBe('src/**/*.ts');
  expect(result.include![1] instanceof RegExp).toBe(true);
});

test('normalizeRsOptions: mixed exclude patterns (string and RegExp)', () => {
  const exclude = ['node_modules/**', /\.test\./];
  const result = normalizeRsOptions({ exclude });
  expect(result.exclude!.length).toBe(2);
  expect(result.exclude![0]).toBe('node_modules/**');
  expect(result.exclude![1] instanceof RegExp).toBe(true);
});

test('normalizeRsOptions: explicit false values are not stripped', () => {
  const result = normalizeRsOptions({
    dev: false,
    test: false,
    debug: false,
    runtimeInjection: false,
    treeshakeCompensation: false,
  });
  expect(result.dev).toBe(false);
  expect(result.test).toBe(false);
  expect(result.debug).toBe(false);
  expect(result.runtimeInjection).toBe(false);
  expect(result.treeshakeCompensation).toBe(false);
});

test('normalizeRsOptions: explicit 0 / empty string are not stripped', () => {
  const result = normalizeRsOptions({ classNamePrefix: '', runtimeInjection: '' });
  expect(result.classNamePrefix).toBe('');
  expect(result.runtimeInjection).toBe('');
});

test('normalizeRsOptions: undefined fields do not clobber defaults', () => {
  const result = normalizeRsOptions({
    dev: undefined,
    test: undefined,
    enableFontSizePxToRem: undefined,
    styleResolution: undefined,
  });
  expect(result.dev).toBe(false);
  expect(result.test).toBe(false);
  expect(result.enableFontSizePxToRem).toBe(false);
  expect(result.styleResolution).toBe('property-specificity');
});

test('normalizeRsOptions: enableLogicalStylesPolyfill default and override', () => {
  expect(normalizeRsOptions({}).enableLogicalStylesPolyfill).toBe(false);
  expect(normalizeRsOptions({ enableLogicalStylesPolyfill: true }).enableLogicalStylesPolyfill).toBe(true);
});

test('normalizeRsOptions: enableMediaQueryOrder default and override', () => {
  expect(normalizeRsOptions({}).enableMediaQueryOrder).toBe(true);
  expect(normalizeRsOptions({ enableMediaQueryOrder: false }).enableMediaQueryOrder).toBe(false);
});

test('normalizeRsOptions: legacyDisableLayers default and override', () => {
  expect(normalizeRsOptions({}).legacyDisableLayers).toBe(false);
  expect(normalizeRsOptions({ legacyDisableLayers: true }).legacyDisableLayers).toBe(true);
});

test('normalizeRsOptions: useRealFileForSource default and override', () => {
  expect(normalizeRsOptions({}).useRealFileForSource).toBe(true);
  expect(normalizeRsOptions({ useRealFileForSource: false }).useRealFileForSource).toBe(false);
});

test('normalizeRsOptions: enableDebugClassNames default and override', () => {
  expect(normalizeRsOptions({}).enableDebugClassNames).toBe(false);
  expect(normalizeRsOptions({ enableDebugClassNames: true }).enableDebugClassNames).toBe(true);
});

test('normalizeRsOptions: many swcPlugins are passed through', () => {
  const swcPlugins: Array<[string, Record<string, unknown>]> = [
    ['@swc/plugin-a', { opt: 1 }],
    ['@swc/plugin-b', { opt: 2 }],
    ['@swc/plugin-c', {}],
  ];
  const result = normalizeRsOptions({ swcPlugins });
  expect(result.swcPlugins!.length).toBe(3);
  expect(result.swcPlugins).toEqual(swcPlugins);
});

test('normalizeRsOptions: unstable_moduleResolution with all fields', () => {
  const result = normalizeRsOptions({
    unstable_moduleResolution: {
      type: 'commonJS',
      rootDir: '/project',
      themeFileExtension: '.stylex.ts',
    },
  });
  expect(result.unstable_moduleResolution).toEqual({
    type: 'commonJS',
    rootDir: '/project',
    themeFileExtension: '.stylex.ts',
  });
});

test('normalizeRsOptions: unstable_moduleResolution with minimal fields', () => {
  const result = normalizeRsOptions({
    unstable_moduleResolution: { type: 'esm' },
  });
  expect(result.unstable_moduleResolution!.type).toBe('esm');
  expect(result.unstable_moduleResolution!.rootDir).toBe(undefined);
});

test('normalizeRsOptions: all defaults are correct', () => {
  const result = normalizeRsOptions({});
  // Verify every default value
  expect(result.dev).toBe(false);
  expect(result.test).toBe(false);
  expect(result.debug).toBe(false);
  expect(result.enableFontSizePxToRem).toBe(false);
  expect(result.runtimeInjection).toBe(false);
  expect(result.treeshakeCompensation).toBe(false);
  expect(result.enableInlinedConditionalMerge).toBe(true);
  expect(result.enableLogicalStylesPolyfill).toBe(false);
  expect(result.enableMinifiedKeys).toBe(true);
  expect(result.enableLegacyValueFlipping).toBe(false);
  expect(result.enableLTRRTLComments).toBe(false);
  expect(result.legacyDisableLayers).toBe(false);
  expect(result.useRealFileForSource).toBe(true);
  expect(result.enableMediaQueryOrder).toBe(true);
  expect(result.enableDebugClassNames).toBe(false);
  expect(result.propertyValidationMode).toBe('silent');
  expect(result.styleResolution).toBe('property-specificity');
  expect(result.importSources).toEqual(['stylex', '@stylexjs/stylex']);
  expect(result.include).toEqual([]);
  expect(result.exclude).toEqual([]);
  expect(result.swcPlugins).toEqual([]);
});

test('normalizeRsOptions: number input treated as empty object', () => {
  // @ts-expect-error - testing invalid input
  const result = normalizeRsOptions(42);
  expect(result).toEqual(defaultResult);
});

test('normalizeRsOptions: boolean input treated as empty object', () => {
  // @ts-expect-error - testing invalid input
  const result = normalizeRsOptions(true);
  expect(result).toEqual(defaultResult);
});
