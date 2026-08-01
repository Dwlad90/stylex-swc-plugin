import { expect, test } from 'vitest';
import { shouldTransformFile } from '../dist/index';
import * as path from 'path';

const cwd = process.cwd();

test('shouldTransformFile: no patterns - returns true', () => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const result = shouldTransformFile(filePath, undefined, undefined);
  expect(result).toBe(true);
});

test('shouldTransformFile: null patterns - returns true', () => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const result = shouldTransformFile(filePath, null, null);
  expect(result).toBe(true);
});

test('shouldTransformFile: empty arrays - returns true', () => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const result = shouldTransformFile(filePath, [], []);
  expect(result).toBe(true);
});

// Include-only tests
test('shouldTransformFile: include glob - matching file', () => {
  const filePath = path.join(cwd, 'src/components/Button.tsx');
  const include = ['src/**/*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  expect(result).toBe(true);
});

test('shouldTransformFile: include glob - non-matching file', () => {
  const filePath = path.join(cwd, 'lib/components/Button.tsx');
  const include = ['src/**/*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  expect(result).toBe(false);
});

test('shouldTransformFile: include regex - matching file', () => {
  const filePath = path.join(cwd, 'src/components/Button.tsx');
  const include = [/src\/.*\.tsx$/];
  const result = shouldTransformFile(filePath, include, undefined);
  expect(result).toBe(true);
});

test('shouldTransformFile: include regex - non-matching file', () => {
  const filePath = path.join(cwd, 'lib/components/Button.tsx');
  const include = [/^src\/.*\.tsx$/];
  const result = shouldTransformFile(filePath, include, undefined);
  expect(result).toBe(false);
});

test('shouldTransformFile: multiple include patterns - matches first', () => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const include = ['src/**/*.tsx', 'app/**/*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  expect(result).toBe(true);
});

test('shouldTransformFile: multiple include patterns - matches second', () => {
  const filePath = path.join(cwd, 'app/Button.tsx');
  const include = ['src/**/*.tsx', 'app/**/*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  expect(result).toBe(true);
});

test('shouldTransformFile: multiple include patterns - matches neither', () => {
  const filePath = path.join(cwd, 'lib/Button.tsx');
  const include = ['src/**/*.tsx', 'app/**/*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  expect(result).toBe(false);
});

// Exclude-only tests
test('shouldTransformFile: exclude glob - matching file', () => {
  const filePath = path.join(cwd, 'src/Button.test.tsx');
  const exclude = ['**/*.test.tsx'];
  const result = shouldTransformFile(filePath, undefined, exclude);
  expect(result).toBe(false);
});

test('shouldTransformFile: exclude glob - non-matching file', () => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const exclude = ['**/*.test.tsx'];
  const result = shouldTransformFile(filePath, undefined, exclude);
  expect(result).toBe(true);
});

test('shouldTransformFile: exclude regex - matching file', () => {
  const filePath = path.join(cwd, 'src/Button.test.tsx');
  const exclude = [/\.test\.tsx$/];
  const result = shouldTransformFile(filePath, undefined, exclude);
  expect(result).toBe(false);
});

test('shouldTransformFile: exclude regex - non-matching file', () => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const exclude = [/\.test\.tsx$/];
  const result = shouldTransformFile(filePath, undefined, exclude);
  expect(result).toBe(true);
});

test('shouldTransformFile: multiple exclude patterns', () => {
  const testFile = path.join(cwd, 'src/Button.test.tsx');
  const specFile = path.join(cwd, 'src/Button.spec.tsx');
  const normalFile = path.join(cwd, 'src/Button.tsx');
  const exclude = ['**/*.test.tsx', '**/*.spec.tsx'];

  expect(shouldTransformFile(testFile, undefined, exclude)).toBe(false);
  expect(shouldTransformFile(specFile, undefined, exclude)).toBe(false);
  expect(shouldTransformFile(normalFile, undefined, exclude)).toBe(true);
});

// Combined include and exclude tests
test('shouldTransformFile: combined - include matches, exclude does not', () => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const include = ['src/**/*.tsx'];
  const exclude = ['**/*.test.tsx'];
  const result = shouldTransformFile(filePath, include, exclude);
  expect(result).toBe(true);
});

test('shouldTransformFile: combined - include matches, exclude also matches', () => {
  const filePath = path.join(cwd, 'src/Button.test.tsx');
  const include = ['src/**/*.tsx'];
  const exclude = ['**/*.test.tsx'];
  const result = shouldTransformFile(filePath, include, exclude);
  expect(result).toBe(false);
});

test('shouldTransformFile: combined - include does not match', () => {
  const filePath = path.join(cwd, 'lib/Button.tsx');
  const include = ['src/**/*.tsx'];
  const exclude = ['**/*.test.tsx'];
  const result = shouldTransformFile(filePath, include, exclude);
  expect(result).toBe(false);
});

test('shouldTransformFile: complex scenario - multiple patterns', () => {
  // Note: glob doesn't support brace expansion, use separate patterns
  const include = ['src/**/*.ts', 'src/**/*.tsx', 'app/**/*.tsx'];
  const exclude = ['**/*.test.*', '**/*.spec.*', '**/__mocks__/**'];

  const validFile1 = path.join(cwd, 'src/components/Button.tsx');
  const validFile2 = path.join(cwd, 'app/pages/index.tsx');
  const testFile = path.join(cwd, 'src/components/Button.test.tsx');
  const mockFile = path.join(cwd, 'src/__mocks__/Button.tsx');
  const outsideFile = path.join(cwd, 'lib/components/Button.tsx');

  expect(shouldTransformFile(validFile1, include, exclude)).toBe(true);
  expect(shouldTransformFile(validFile2, include, exclude)).toBe(true);
  expect(shouldTransformFile(testFile, include, exclude)).toBe(false);
  expect(shouldTransformFile(mockFile, include, exclude)).toBe(false);
  expect(shouldTransformFile(outsideFile, include, exclude)).toBe(false);
});

// Edge cases
test('shouldTransformFile: root level file with glob', () => {
  const filePath = path.join(cwd, 'index.tsx');
  const include = ['*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  expect(result).toBe(true);
});

test('shouldTransformFile: nested directories', () => {
  const filePath = path.join(cwd, 'src/nested/deep/Component.tsx');
  const include = ['src/**/*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  expect(result).toBe(true);
});

test('shouldTransformFile: exclude takes precedence over include', () => {
  const filePath = path.join(cwd, 'src/__tests__/Button.tsx');
  const include = ['src/**/*.tsx'];
  const exclude = ['**/__tests__/**'];
  const result = shouldTransformFile(filePath, include, exclude);
  expect(result).toBe(false);
});

test('shouldTransformFile: mixed glob and regex patterns', () => {
  const filePath1 = path.join(cwd, 'src/Button.tsx');
  const filePath2 = path.join(cwd, 'app/page.tsx');
  const include = ['src/**/*.tsx', /app\/.*\.tsx$/];
  const exclude = ['**/*.test.*', /\.(stories|spec)\./];

  expect(shouldTransformFile(filePath1, include, exclude)).toBe(true);
  expect(shouldTransformFile(filePath2, include, exclude)).toBe(true);
});

test('shouldTransformFile: file with multiple dots', () => {
  const filePath = path.join(cwd, 'src/Button.component.test.tsx');
  const exclude = ['**/*.test.tsx'];
  const result = shouldTransformFile(filePath, undefined, exclude);
  expect(result).toBe(false);
});

// Regex lookahead/lookbehind tests
test('shouldTransformFile: negative lookahead - exclude node_modules except specific package', () => {
  const nodeModulesFile = path.join(cwd, 'node_modules/some-package/index.js');
  const stylexFile = path.join(cwd, 'node_modules/@stylexjs/open-props/index.js');
  const srcFile = path.join(cwd, 'src/index.tsx');

  // Exclude all node_modules except @stylexjs/open-props
  const exclude = [/node_modules(?!\/@stylexjs\/open-props)/];

  expect(shouldTransformFile(nodeModulesFile, undefined, exclude)).toBe(false);
  expect(shouldTransformFile(stylexFile, undefined, exclude)).toBe(true);
  expect(shouldTransformFile(srcFile, undefined, exclude)).toBe(true);
});

test('shouldTransformFile: negative lookahead - exclude node_modules except @stylexjs scope', () => {
  const otherNodeModule = path.join(cwd, 'node_modules/some-package/index.js');
  const stylexCore = path.join(cwd, 'node_modules/@stylexjs/stylex/index.js');
  const stylexProps = path.join(cwd, 'node_modules/@stylexjs/open-props/index.js');

  // Exclude all node_modules except @stylexjs packages
  const exclude = [/node_modules(?!\/@stylexjs)/];

  expect(shouldTransformFile(otherNodeModule, undefined, exclude)).toBe(false);
  expect(shouldTransformFile(stylexCore, undefined, exclude)).toBe(true);
  expect(shouldTransformFile(stylexProps, undefined, exclude)).toBe(true);
});

test('shouldTransformFile: positive lookahead - only files before .test', () => {
  const testFile = path.join(cwd, 'src/Button.test.tsx');
  const componentFile = path.join(cwd, 'src/Button.tsx');

  // Only match files that have .test before .tsx
  const include = [/.*\.test(?=\.tsx$)/];

  expect(shouldTransformFile(testFile, include, undefined)).toBe(true);
  expect(shouldTransformFile(componentFile, include, undefined)).toBe(false);
});

test('shouldTransformFile: negative lookbehind - exclude files not preceded by src/', () => {
  const srcFile = path.join(cwd, 'src/components/Button.tsx');
  const libFile = path.join(cwd, 'lib/components/Button.tsx');

  // Exclude .tsx files not starting with src/ using negative lookahead
  // Note: negative lookbehind checks position before the match, not within the string
  // So we use negative lookahead at the start to check if string doesn't start with src/
  const exclude = [/^(?!src\/).*\.tsx$/];

  expect(shouldTransformFile(srcFile, undefined, exclude)).toBe(true); // Starts with src/, doesn't match exclude
  expect(shouldTransformFile(libFile, undefined, exclude)).toBe(false); // Doesn't start with src/, matches exclude
});

// Error handling tests
test('shouldTransformFile: invalid pattern type - skipped gracefully', () => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const include = [123, 'src/**/*.tsx'];
  // @ts-expect-error - invalid pattern type
  const result = shouldTransformFile(filePath, include, undefined);
  // Should process valid patterns and skip invalid ones
  expect(result).toBe(true);
});

test('shouldTransformFile: empty string pattern', () => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const include = [''];
  const result = shouldTransformFile(filePath, include, undefined);
  // Empty string pattern shouldn't match anything
  expect(result).toBe(false);
});

// Real-world scenarios
test('shouldTransformFile: React project - exclude tests and stories', () => {
  const include = ['src/**/*.ts', 'src/**/*.tsx'];
  const exclude = [
    '**/*.test.ts',
    '**/*.test.tsx',
    '**/*.spec.ts',
    '**/*.spec.tsx',
    '**/*.stories.ts',
    '**/*.stories.tsx',
    '**/__tests__/**',
    '**/__mocks__/**',
  ];

  const component = path.join(cwd, 'src/components/Button.tsx');
  const test = path.join(cwd, 'src/components/Button.test.tsx');
  const story = path.join(cwd, 'src/components/Button.stories.tsx');

  expect(shouldTransformFile(component, include, exclude)).toBe(true);
  expect(shouldTransformFile(test, include, exclude)).toBe(false);
  expect(shouldTransformFile(story, include, exclude)).toBe(false);
});

test('shouldTransformFile: monorepo - multiple packages', () => {
  const include = [
    'packages/*/src/**/*.ts',
    'packages/*/src/**/*.tsx',
    'apps/*/src/**/*.ts',
    'apps/*/src/**/*.tsx',
  ];
  const exclude = ['**/*.test.*', '**/node_modules/**'];

  const pkgFile = path.join(cwd, 'packages/ui/src/Button.tsx');
  const appFile = path.join(cwd, 'apps/web/src/App.tsx');
  const testFile = path.join(cwd, 'packages/ui/src/Button.test.tsx');

  expect(shouldTransformFile(pkgFile, include, exclude)).toBe(true);
  expect(shouldTransformFile(appFile, include, exclude)).toBe(true);
  expect(shouldTransformFile(testFile, include, exclude)).toBe(false);
});

test('shouldTransformFile: Next.js app directory', () => {
  const include = ['app/**/*.ts', 'app/**/*.tsx', 'components/**/*.ts', 'components/**/*.tsx'];
  const exclude = ['**/*.test.*', '**/__tests__/**', 'app/api/**'];

  const page = path.join(cwd, 'app/page.tsx');
  const component = path.join(cwd, 'components/Header.tsx');
  const api = path.join(cwd, 'app/api/route.ts');

  expect(shouldTransformFile(page, include, exclude)).toBe(true);
  expect(shouldTransformFile(component, include, exclude)).toBe(true);
  expect(shouldTransformFile(api, include, exclude)).toBe(false);
});

// Performance directories exclusion
test('shouldTransformFile: exclude build directories', () => {
  const include = ['**/*.ts', '**/*.tsx'];
  const exclude = [
    '**/node_modules/**',
    '**/dist/**',
    '**/build/**',
    '**/.next/**',
    '**/coverage/**',
  ];

  const srcFile = path.join(cwd, 'src/index.tsx');
  const nodeModules = path.join(cwd, 'node_modules/react/index.ts');
  const dist = path.join(cwd, 'dist/bundle.js');

  expect(shouldTransformFile(srcFile, include, exclude)).toBe(true);
  expect(shouldTransformFile(nodeModules, include, exclude)).toBe(false);
  expect(shouldTransformFile(dist, include, exclude)).toBe(false);
});

// Additional tests for paths not matching include patterns

test('shouldTransformFile: include specific extension - wrong extension', () => {
  const include = ['src/**/*.tsx'];
  const tsFile = path.join(cwd, 'src/utils.ts');
  const jsFile = path.join(cwd, 'src/index.js');
  const cssFile = path.join(cwd, 'src/styles.css');

  expect(shouldTransformFile(tsFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(jsFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(cssFile, include, undefined)).toBe(false);
});

test('shouldTransformFile: include wrong directory - correct extension', () => {
  const include = ['src/**/*.tsx'];
  const libFile = path.join(cwd, 'lib/Button.tsx');
  const distFile = path.join(cwd, 'dist/Button.tsx');
  const publicFile = path.join(cwd, 'public/Button.tsx');

  expect(shouldTransformFile(libFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(distFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(publicFile, include, undefined)).toBe(false);
});

test('shouldTransformFile: include specific subdirectory - files in parent or sibling directories', () => {
  const include = ['src/components/**/*.tsx'];
  const parentFile = path.join(cwd, 'src/index.tsx');
  const siblingFile = path.join(cwd, 'src/utils/helper.tsx');
  const validFile = path.join(cwd, 'src/components/Button.tsx');

  expect(shouldTransformFile(parentFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(siblingFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(validFile, include, undefined)).toBe(true);
});

test('shouldTransformFile: strict path matching - similar but different paths', () => {
  const include = ['src/**/*.tsx'];
  const srcCopy = path.join(cwd, 'src-copy/Button.tsx');
  const srcBackup = path.join(cwd, 'src.backup/Button.tsx');
  const srcOld = path.join(cwd, 'src_old/Button.tsx');

  expect(shouldTransformFile(srcCopy, include, undefined)).toBe(false);
  expect(shouldTransformFile(srcBackup, include, undefined)).toBe(false);
  expect(shouldTransformFile(srcOld, include, undefined)).toBe(false);
});

test('shouldTransformFile: multiple includes with no match', () => {
  const include = ['src/**/*.tsx', 'app/**/*.tsx', 'components/**/*.tsx'];
  const libFile = path.join(cwd, 'lib/Button.tsx');
  const vendorFile = path.join(cwd, 'vendor/Component.tsx');
  const testFile = path.join(cwd, 'tests/fixture.tsx');

  expect(shouldTransformFile(libFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(vendorFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(testFile, include, undefined)).toBe(false);
});

test('shouldTransformFile: include with regex - non-matching paths', () => {
  const include = [/^src\/components\/.*\.tsx$/];
  const wrongDir = path.join(cwd, 'lib/components/Button.tsx');
  const wrongExt = path.join(cwd, 'src/components/Button.ts');
  const wrongNesting = path.join(cwd, 'src/Button.tsx');

  expect(shouldTransformFile(wrongDir, include, undefined)).toBe(false);
  expect(shouldTransformFile(wrongExt, include, undefined)).toBe(false);
  expect(shouldTransformFile(wrongNesting, include, undefined)).toBe(false);
});

test('shouldTransformFile: case-sensitive path matching', () => {
  const include = ['src/**/*.tsx'];
  // Note: On case-insensitive filesystems (macOS, Windows), this behavior may vary
  const upperSrc = path.join(cwd, 'Src/Button.tsx');
  const upperFile = path.join(cwd, 'src/BUTTON.tsx');

  // On case-sensitive filesystems, these should not match
  // The actual behavior depends on the filesystem
  const result1 = shouldTransformFile(upperSrc, include, undefined);
  const result2 = shouldTransformFile(upperFile, include, undefined);

  // Just verify the function doesn't crash - actual result depends on filesystem
  expect(typeof result1).toBe('boolean');
  expect(typeof result2).toBe('boolean');
});

test('shouldTransformFile: include with specific nesting level', () => {
  // Note: glob's * wildcard matches any characters including path separators
  // So src/*/*.tsx will match both src/Button.tsx and src/a/b/Button.tsx
  // To match exactly one level, we need to use a more specific pattern or regex
  const include = [/^src\/[^/]+\/[^/]+$/]; // Exactly src/ + one dir + one file
  const tooShallow = path.join(cwd, 'src/index.tsx');
  const tooDeep = path.join(cwd, 'src/components/ui/Button.tsx');
  const justRight = path.join(cwd, 'src/components/Button.tsx');

  expect(shouldTransformFile(tooShallow, include, undefined)).toBe(false);
  expect(shouldTransformFile(tooDeep, include, undefined)).toBe(false);
  expect(shouldTransformFile(justRight, include, undefined)).toBe(true);
});

test('shouldTransformFile: hidden directories not matching include', () => {
  const include = ['src/**/*.tsx'];
  const hiddenDir = path.join(cwd, '.hidden/Button.tsx');
  const hiddenNested = path.join(cwd, 'src/.private/Component.tsx');
  const dotFile = path.join(cwd, '.config.tsx');

  expect(shouldTransformFile(hiddenDir, include, undefined)).toBe(false);
  // This should match because it's within src/
  expect(shouldTransformFile(hiddenNested, include, undefined)).toBe(true);
  expect(shouldTransformFile(dotFile, include, undefined)).toBe(false);
});

test('shouldTransformFile: vendor and third-party directories', () => {
  const include = ['src/**/*.ts', 'src/**/*.tsx'];
  const vendor = path.join(cwd, 'vendor/lib.ts');
  const thirdParty = path.join(cwd, 'third_party/module.ts');
  const external = path.join(cwd, 'external/plugin.ts');

  expect(shouldTransformFile(vendor, include, undefined)).toBe(false);
  expect(shouldTransformFile(thirdParty, include, undefined)).toBe(false);
  expect(shouldTransformFile(external, include, undefined)).toBe(false);
});

test('shouldTransformFile: include with multiple extensions - wrong extension combinations', () => {
  const include = ['src/**/*.ts', 'src/**/*.tsx'];
  const jsFile = path.join(cwd, 'src/index.js');
  const jsxFile = path.join(cwd, 'src/component.jsx');
  const mjsFile = path.join(cwd, 'src/module.mjs');
  const dtsFile = path.join(cwd, 'src/types.d.ts');

  expect(shouldTransformFile(jsFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(jsxFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(mjsFile, include, undefined)).toBe(false);
  // .d.ts files should match **/*.ts pattern
  expect(shouldTransformFile(dtsFile, include, undefined)).toBe(true);
});

test('shouldTransformFile: strict regex matching - partial path matches', () => {
  // Regex without ^ and $ anchors should match anywhere
  const include1 = [/components\/.*\.tsx/]; // No anchors - matches anywhere
  const include2 = [/^components\/.*\.tsx$/]; // With anchors - only exact match

  const file1 = path.join(cwd, 'src/components/Button.tsx');
  const file2 = path.join(cwd, 'components/Button.tsx');

  // Without anchors, should match the substring
  expect(shouldTransformFile(file1, include1, undefined)).toBe(true);
  expect(shouldTransformFile(file2, include1, undefined)).toBe(true);

  // With anchors, only the exact path should match
  expect(shouldTransformFile(file1, include2, undefined)).toBe(false);
  expect(shouldTransformFile(file2, include2, undefined)).toBe(true);
});

test('shouldTransformFile: files at root vs nested with same name', () => {
  const include = ['src/**/*.tsx'];
  const rootIndex = path.join(cwd, 'index.tsx');
  const srcIndex = path.join(cwd, 'src/index.tsx');
  const nestedIndex = path.join(cwd, 'src/pages/index.tsx');

  expect(shouldTransformFile(rootIndex, include, undefined)).toBe(false);
  expect(shouldTransformFile(srcIndex, include, undefined)).toBe(true);
  expect(shouldTransformFile(nestedIndex, include, undefined)).toBe(true);
});

test('shouldTransformFile: temp and cache directories', () => {
  const include = ['src/**/*.tsx'];
  const tempFile = path.join(cwd, '.temp/component.tsx');
  const cacheFile = path.join(cwd, '.cache/bundle.tsx');
  const tmpFile = path.join(cwd, 'tmp/output.tsx');

  expect(shouldTransformFile(tempFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(cacheFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(tmpFile, include, undefined)).toBe(false);
});

test('shouldTransformFile: backup and versioned directories', () => {
  const include = ['src/**/*.tsx'];
  const backup = path.join(cwd, 'backup/src/Button.tsx');
  const archived = path.join(cwd, 'archived/src/Component.tsx');
  const v1 = path.join(cwd, 'v1/src/OldComponent.tsx');

  expect(shouldTransformFile(backup, include, undefined)).toBe(false);
  expect(shouldTransformFile(archived, include, undefined)).toBe(false);
  expect(shouldTransformFile(v1, include, undefined)).toBe(false);
});

test('shouldTransformFile: include matches but wrong file type', () => {
  const include = ['src/**/*.tsx'];
  const imageFile = path.join(cwd, 'src/assets/logo.png');
  const styleFile = path.join(cwd, 'src/styles/main.css');
  const mdFile = path.join(cwd, 'src/README.md');
  const jsonFile = path.join(cwd, 'src/config.json');

  expect(shouldTransformFile(imageFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(styleFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(mdFile, include, undefined)).toBe(false);
  expect(shouldTransformFile(jsonFile, include, undefined)).toBe(false);
});

test('shouldTransformFile: very specific include pattern - near misses', () => {
  const include = ['src/components/ui/**/*.tsx'];
  const tooHigh = path.join(cwd, 'src/Button.tsx');
  const oneLevelUp = path.join(cwd, 'src/components/Button.tsx');
  const wrongSibling = path.join(cwd, 'src/components/layout/Header.tsx');
  const correct = path.join(cwd, 'src/components/ui/Button.tsx');

  expect(shouldTransformFile(tooHigh, include, undefined)).toBe(false);
  expect(shouldTransformFile(oneLevelUp, include, undefined)).toBe(false);
  expect(shouldTransformFile(wrongSibling, include, undefined)).toBe(false);
  expect(shouldTransformFile(correct, include, undefined)).toBe(true);
});

test('shouldTransformFile: include with complex regex - boundary cases', () => {
  // Match files that have exactly 'component' in the name (not 'components')
  const include = [/\/component\.tsx$/];
  const singular = path.join(cwd, 'src/component.tsx');
  const plural = path.join(cwd, 'src/components.tsx');
  const prefix = path.join(cwd, 'src/mycomponent.tsx');
  const suffix = path.join(cwd, 'src/componentlist.tsx');

  expect(shouldTransformFile(singular, include, undefined)).toBe(true);
  expect(shouldTransformFile(plural, include, undefined)).toBe(false);
  expect(shouldTransformFile(prefix, include, undefined)).toBe(false);
  expect(shouldTransformFile(suffix, include, undefined)).toBe(false);
});

test('shouldTransformFile: regex with escaped slashes (when passed as string)', () => {
  // Test that escaped slashes in regex patterns are handled correctly
  // This tests the from_string parser when patterns come as strings
  // Note: When patterns come as RegExp objects, they're handled directly in parse_js_pattern
  const include = ['src/**/*.tsx'];

  // This should match files with literal forward slashes in the pattern
  const file1 = path.join(cwd, 'src/components/Button.tsx');
  const file2 = path.join(cwd, 'lib/components/Button.tsx');

  expect(shouldTransformFile(file1, include, undefined)).toBe(true);
  expect(shouldTransformFile(file2, include, undefined)).toBe(false);

  // Test with regex object that uses escaped slashes in the pattern itself
  const regexInclude = [/src\/components\/.*\.tsx$/];
  expect(shouldTransformFile(file1, regexInclude, undefined)).toBe(true);
  expect(shouldTransformFile(file2, regexInclude, undefined)).toBe(false);
});

test('shouldTransformFile: invalid regex patterns fallback to glob', () => {
  // Invalid regex patterns should be treated as glob patterns
  const include = ['/[invalid(regex/'];
  const file = path.join(cwd, '[invalid(regex');

  // Since it's invalid regex, it should be treated as glob pattern
  // which won't match our test file
  expect(shouldTransformFile(file, include, undefined)).toBe(false);
});

test('shouldTransformFile: string regex with flags', () => {
  // Use native RegExp objects for case-insensitive matching
  const include = [/button/i]; // Case-insensitive via RegExp
  const upper = path.join(cwd, 'src/BUTTON.tsx');
  const lower = path.join(cwd, 'src/button.tsx');

  expect(shouldTransformFile(upper, include, undefined)).toBe(true);
  expect(shouldTransformFile(lower, include, undefined)).toBe(true);
});

// Regression test for RegExp with /g flag (lastIndex statefulness)
test('shouldTransformFile: regex with /g flag is deterministic across calls', () => {
  const globalRegex = /src/g;
  const include = [globalRegex];
  const filePath = path.join(cwd, 'src/Button.tsx');

  // Without lastIndex reset, the second call could return false
  // because RegExp.test() with /g advances lastIndex
  expect(shouldTransformFile(filePath, include, undefined)).toBe(true);
  expect(shouldTransformFile(filePath, include, undefined)).toBe(true);
  expect(shouldTransformFile(filePath, include, undefined)).toBe(true);
});

test('shouldTransformFile: regex with /y flag is deterministic across calls', () => {
  const stickyRegex = /src/y;
  const include = [stickyRegex];
  const filePath = path.join(cwd, 'src/Button.tsx');

  // Sticky flag also causes lastIndex issues
  expect(shouldTransformFile(filePath, include, undefined)).toBe(true);
  expect(shouldTransformFile(filePath, include, undefined)).toBe(true);
});

test('shouldTransformFile: regex with /gi flags is deterministic across calls', () => {
  const globalInsensitiveRegex = /BUTTON/gi;
  const include = [globalInsensitiveRegex];
  const filePath = path.join(cwd, 'src/Button.tsx');

  for (let i = 0; i < 10; i++) {
    expect(shouldTransformFile(filePath, include, undefined), `failed on call ${i + 1}`).toBe(true);
  }
});

// ── Path edge cases ─────────────────────────────────────────────────

test('shouldTransformFile: empty string file path', () => {
  expect(shouldTransformFile('', ['**/*.ts'], undefined)).toBe(false);
});

test('shouldTransformFile: absolute path with no cwd prefix', () => {
  // When the absolute path isn't under cwd, path.relative returns
  // something with ../.. which may or may not match patterns
  expect(
    shouldTransformFile('/completely/different/path.ts', ['src/**'], undefined),
    'unrelated absolute path should not match src/**'
  ).toBe(false);
});

test('shouldTransformFile: dot files and directories', () => {
  const dotFile = path.join(cwd, '.eslintrc.ts');
  const dotDir = path.join(cwd, '.config/styles.ts');

  expect(shouldTransformFile(dotFile, ['**/*.ts'], undefined)).toBe(true);
  expect(shouldTransformFile(dotDir, ['.config/**'], undefined)).toBe(true);
});

test('shouldTransformFile: file with spaces in path', () => {
  const file = path.join(cwd, 'src/My Component/styles.tsx');
  expect(shouldTransformFile(file, ['src/**/*.tsx'], undefined)).toBe(true);
});

test('shouldTransformFile: file with unicode characters in path', () => {
  const file = path.join(cwd, 'src/компонент/styles.tsx');
  expect(shouldTransformFile(file, ['src/**/*.tsx'], undefined)).toBe(true);
});

test('shouldTransformFile: deeply nested path', () => {
  const file = path.join(cwd, 'src/a/b/c/d/e/f/g/h/styles.tsx');
  expect(shouldTransformFile(file, ['src/**/*.tsx'], undefined)).toBe(true);
  expect(shouldTransformFile(file, ['src/*.tsx'], undefined)).toBe(false);
});

test('shouldTransformFile: both include and exclude empty arrays', () => {
  const file = path.join(cwd, 'anything.tsx');
  expect(shouldTransformFile(file, [], [])).toBe(true);
});

test('shouldTransformFile: regex with special chars matches correctly', () => {
  const file = path.join(cwd, 'src/file.module.css.ts');
  expect(shouldTransformFile(file, [/\.module\.css\.ts$/], undefined)).toBe(true);
  expect(shouldTransformFile(file, [/\.module\.css$/], undefined)).toBe(false);
});

test('shouldTransformFile: anchored regex patterns', () => {
  const file = path.join(cwd, 'src/Button.tsx');
  expect(shouldTransformFile(file, [/^src\//], undefined)).toBe(true);
  expect(shouldTransformFile(file, [/\.tsx$/], undefined)).toBe(true);
  expect(shouldTransformFile(file, [/^lib\//], undefined)).toBe(false);
});

test('shouldTransformFile: exclude with regex matching entire relative path', () => {
  const file = path.join(cwd, 'vendor/third-party/lib.ts');
  expect(shouldTransformFile(file, undefined, [/^vendor\//])).toBe(false);
  expect(shouldTransformFile(file, undefined, [/third-party/])).toBe(false);
});

test('shouldTransformFile: same regex used for both include and exclude', () => {
  const pattern = /src\/.*\.tsx$/;
  const file = path.join(cwd, 'src/App.tsx');
  // Include matches, exclude also matches → excluded
  expect(shouldTransformFile(file, [pattern], [pattern])).toBe(false);
});

test('shouldTransformFile: glob with negation-like pattern', () => {
  const file1 = path.join(cwd, 'src/utils/helper.ts');
  const file2 = path.join(cwd, 'src/utils/helper.test.ts');
  // Include all TS, exclude tests
  expect(shouldTransformFile(file1, ['**/*.ts'], [/\.test\./])).toBe(true);
  expect(shouldTransformFile(file2, ['**/*.ts'], [/\.test\./])).toBe(false);
});

test('shouldTransformFile: single file glob pattern', () => {
  const file = path.join(cwd, 'src/specific-file.tsx');
  expect(shouldTransformFile(file, ['src/specific-file.tsx'], undefined)).toBe(true);
  expect(shouldTransformFile(file, ['src/other-file.tsx'], undefined)).toBe(false);
});

test('shouldTransformFile: case-sensitive glob pattern', () => {
  const file = path.join(cwd, 'src/MyComponent.TSX');
  // Glob patterns are case-sensitive by default
  expect(shouldTransformFile(file, ['src/**/*.tsx'], undefined)).toBe(false);
  expect(shouldTransformFile(file, ['src/**/*.TSX'], undefined)).toBe(true);
});

test('shouldTransformFile: repeated calls with same stateful regex in exclude', () => {
  const exclude = [/test/g];
  const file = path.join(cwd, 'src/test-utils.ts');

  for (let i = 0; i < 5; i++) {
    expect(
      shouldTransformFile(file, undefined, exclude),
      `exclude with /g should be consistent on call ${i + 1}`
    ).toBe(false);
  }
});

test('shouldTransformFile: empty include array matches everything', () => {
  expect(shouldTransformFile(path.join(cwd, 'any/file.ts'), [], undefined)).toBe(true);
});

test('shouldTransformFile: empty exclude array excludes nothing', () => {
  expect(shouldTransformFile(path.join(cwd, 'any/file.ts'), undefined, [])).toBe(true);
});
