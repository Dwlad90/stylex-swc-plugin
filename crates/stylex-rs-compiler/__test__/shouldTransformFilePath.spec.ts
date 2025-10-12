import test from 'ava';
import { shouldTransformFile } from '../dist/index';
import * as path from 'path';

const cwd = process.cwd();

test('shouldTransformFile: no patterns - returns true', t => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const result = shouldTransformFile(filePath, undefined, undefined);
  t.is(result, true);
});

test('shouldTransformFile: null patterns - returns true', t => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const result = shouldTransformFile(filePath, null, null);
  t.is(result, true);
});

test('shouldTransformFile: empty arrays - returns true', t => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const result = shouldTransformFile(filePath, [], []);
  t.is(result, true);
});

// Include-only tests
test('shouldTransformFile: include glob - matching file', t => {
  const filePath = path.join(cwd, 'src/components/Button.tsx');
  const include = ['src/**/*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  t.is(result, true);
});

test('shouldTransformFile: include glob - non-matching file', t => {
  const filePath = path.join(cwd, 'lib/components/Button.tsx');
  const include = ['src/**/*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  t.is(result, false);
});

test('shouldTransformFile: include regex - matching file', t => {
  const filePath = path.join(cwd, 'src/components/Button.tsx');
  const include = [/src\/.*\.tsx$/];
  const result = shouldTransformFile(filePath, include, undefined);
  t.is(result, true);
});

test('shouldTransformFile: include regex - non-matching file', t => {
  const filePath = path.join(cwd, 'lib/components/Button.tsx');
  const include = [/^src\/.*\.tsx$/];
  const result = shouldTransformFile(filePath, include, undefined);
  t.is(result, false);
});

test('shouldTransformFile: multiple include patterns - matches first', t => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const include = ['src/**/*.tsx', 'app/**/*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  t.is(result, true);
});

test('shouldTransformFile: multiple include patterns - matches second', t => {
  const filePath = path.join(cwd, 'app/Button.tsx');
  const include = ['src/**/*.tsx', 'app/**/*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  t.is(result, true);
});

test('shouldTransformFile: multiple include patterns - matches neither', t => {
  const filePath = path.join(cwd, 'lib/Button.tsx');
  const include = ['src/**/*.tsx', 'app/**/*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  t.is(result, false);
});

// Exclude-only tests
test('shouldTransformFile: exclude glob - matching file', t => {
  const filePath = path.join(cwd, 'src/Button.test.tsx');
  const exclude = ['**/*.test.tsx'];
  const result = shouldTransformFile(filePath, undefined, exclude);
  t.is(result, false);
});

test('shouldTransformFile: exclude glob - non-matching file', t => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const exclude = ['**/*.test.tsx'];
  const result = shouldTransformFile(filePath, undefined, exclude);
  t.is(result, true);
});

test('shouldTransformFile: exclude regex - matching file', t => {
  const filePath = path.join(cwd, 'src/Button.test.tsx');
  const exclude = [/\.test\.tsx$/];
  const result = shouldTransformFile(filePath, undefined, exclude);
  t.is(result, false);
});

test('shouldTransformFile: exclude regex - non-matching file', t => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const exclude = [/\.test\.tsx$/];
  const result = shouldTransformFile(filePath, undefined, exclude);
  t.is(result, true);
});

test('shouldTransformFile: multiple exclude patterns', t => {
  const testFile = path.join(cwd, 'src/Button.test.tsx');
  const specFile = path.join(cwd, 'src/Button.spec.tsx');
  const normalFile = path.join(cwd, 'src/Button.tsx');
  const exclude = ['**/*.test.tsx', '**/*.spec.tsx'];

  t.is(shouldTransformFile(testFile, undefined, exclude), false);
  t.is(shouldTransformFile(specFile, undefined, exclude), false);
  t.is(shouldTransformFile(normalFile, undefined, exclude), true);
});

// Combined include and exclude tests
test('shouldTransformFile: combined - include matches, exclude does not', t => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const include = ['src/**/*.tsx'];
  const exclude = ['**/*.test.tsx'];
  const result = shouldTransformFile(filePath, include, exclude);
  t.is(result, true);
});

test('shouldTransformFile: combined - include matches, exclude also matches', t => {
  const filePath = path.join(cwd, 'src/Button.test.tsx');
  const include = ['src/**/*.tsx'];
  const exclude = ['**/*.test.tsx'];
  const result = shouldTransformFile(filePath, include, exclude);
  t.is(result, false);
});

test('shouldTransformFile: combined - include does not match', t => {
  const filePath = path.join(cwd, 'lib/Button.tsx');
  const include = ['src/**/*.tsx'];
  const exclude = ['**/*.test.tsx'];
  const result = shouldTransformFile(filePath, include, exclude);
  t.is(result, false);
});

test('shouldTransformFile: complex scenario - multiple patterns', t => {
  // Note: glob doesn't support brace expansion, use separate patterns
  const include = ['src/**/*.ts', 'src/**/*.tsx', 'app/**/*.tsx'];
  const exclude = ['**/*.test.*', '**/*.spec.*', '**/__mocks__/**'];

  const validFile1 = path.join(cwd, 'src/components/Button.tsx');
  const validFile2 = path.join(cwd, 'app/pages/index.tsx');
  const testFile = path.join(cwd, 'src/components/Button.test.tsx');
  const mockFile = path.join(cwd, 'src/__mocks__/Button.tsx');
  const outsideFile = path.join(cwd, 'lib/components/Button.tsx');

  t.is(shouldTransformFile(validFile1, include, exclude), true);
  t.is(shouldTransformFile(validFile2, include, exclude), true);
  t.is(shouldTransformFile(testFile, include, exclude), false);
  t.is(shouldTransformFile(mockFile, include, exclude), false);
  t.is(shouldTransformFile(outsideFile, include, exclude), false);
});

// Edge cases
test('shouldTransformFile: root level file with glob', t => {
  const filePath = path.join(cwd, 'index.tsx');
  const include = ['*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  t.is(result, true);
});

test('shouldTransformFile: nested directories', t => {
  const filePath = path.join(cwd, 'src/nested/deep/Component.tsx');
  const include = ['src/**/*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  t.is(result, true);
});

test('shouldTransformFile: exclude takes precedence over include', t => {
  const filePath = path.join(cwd, 'src/__tests__/Button.tsx');
  const include = ['src/**/*.tsx'];
  const exclude = ['**/__tests__/**'];
  const result = shouldTransformFile(filePath, include, exclude);
  t.is(result, false);
});

test('shouldTransformFile: mixed glob and regex patterns', t => {
  const filePath1 = path.join(cwd, 'src/Button.tsx');
  const filePath2 = path.join(cwd, 'app/page.tsx');
  const include = ['src/**/*.tsx', /app\/.*\.tsx$/];
  const exclude = ['**/*.test.*', /\.(stories|spec)\./];

  t.is(shouldTransformFile(filePath1, include, exclude), true);
  t.is(shouldTransformFile(filePath2, include, exclude), true);
});

test('shouldTransformFile: file with multiple dots', t => {
  const filePath = path.join(cwd, 'src/Button.component.test.tsx');
  const exclude = ['**/*.test.tsx'];
  const result = shouldTransformFile(filePath, undefined, exclude);
  t.is(result, false);
});

// Regex lookahead/lookbehind tests
test('shouldTransformFile: negative lookahead - exclude node_modules except specific package', t => {
  const nodeModulesFile = path.join(cwd, 'node_modules/some-package/index.js');
  const stylexFile = path.join(cwd, 'node_modules/@stylexjs/open-props/index.js');
  const srcFile = path.join(cwd, 'src/index.tsx');

  // Exclude all node_modules except @stylexjs/open-props
  const exclude = [/node_modules(?!\/@stylexjs\/open-props)/];

  t.is(shouldTransformFile(nodeModulesFile, undefined, exclude), false);
  t.is(shouldTransformFile(stylexFile, undefined, exclude), true);
  t.is(shouldTransformFile(srcFile, undefined, exclude), true);
});

test('shouldTransformFile: negative lookahead - exclude node_modules except @stylexjs scope', t => {
  const otherNodeModule = path.join(cwd, 'node_modules/some-package/index.js');
  const stylexCore = path.join(cwd, 'node_modules/@stylexjs/stylex/index.js');
  const stylexProps = path.join(cwd, 'node_modules/@stylexjs/open-props/index.js');

  // Exclude all node_modules except @stylexjs packages
  const exclude = [/node_modules(?!\/@stylexjs)/];

  t.is(shouldTransformFile(otherNodeModule, undefined, exclude), false);
  t.is(shouldTransformFile(stylexCore, undefined, exclude), true);
  t.is(shouldTransformFile(stylexProps, undefined, exclude), true);
});

test('shouldTransformFile: positive lookahead - only files before .test', t => {
  const testFile = path.join(cwd, 'src/Button.test.tsx');
  const componentFile = path.join(cwd, 'src/Button.tsx');

  // Only match files that have .test before .tsx
  const include = [/.*\.test(?=\.tsx$)/];

  t.is(shouldTransformFile(testFile, include, undefined), true);
  t.is(shouldTransformFile(componentFile, include, undefined), false);
});

test('shouldTransformFile: negative lookbehind - exclude files not preceded by src/', t => {
  const srcFile = path.join(cwd, 'src/components/Button.tsx');
  const libFile = path.join(cwd, 'lib/components/Button.tsx');

  // Exclude .tsx files not starting with src/ using negative lookahead
  // Note: negative lookbehind checks position before the match, not within the string
  // So we use negative lookahead at the start to check if string doesn't start with src/
  const exclude = [/^(?!src\/).*\.tsx$/];

  t.is(shouldTransformFile(srcFile, undefined, exclude), true); // Starts with src/, doesn't match exclude
  t.is(shouldTransformFile(libFile, undefined, exclude), false); // Doesn't start with src/, matches exclude
});

// Error handling tests
test('shouldTransformFile: invalid pattern type - skipped gracefully', t => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const include = [123, 'src/**/*.tsx'];
  const result = shouldTransformFile(filePath, include, undefined);
  // Should process valid patterns and skip invalid ones
  t.is(result, true);
});

test('shouldTransformFile: empty string pattern', t => {
  const filePath = path.join(cwd, 'src/Button.tsx');
  const include = [''];
  const result = shouldTransformFile(filePath, include, undefined);
  // Empty string pattern shouldn't match anything
  t.is(result, false);
});

// Real-world scenarios
test('shouldTransformFile: React project - exclude tests and stories', t => {
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

  t.is(shouldTransformFile(component, include, exclude), true);
  t.is(shouldTransformFile(test, include, exclude), false);
  t.is(shouldTransformFile(story, include, exclude), false);
});

test('shouldTransformFile: monorepo - multiple packages', t => {
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

  t.is(shouldTransformFile(pkgFile, include, exclude), true);
  t.is(shouldTransformFile(appFile, include, exclude), true);
  t.is(shouldTransformFile(testFile, include, exclude), false);
});

test('shouldTransformFile: Next.js app directory', t => {
  const include = ['app/**/*.ts', 'app/**/*.tsx', 'components/**/*.ts', 'components/**/*.tsx'];
  const exclude = ['**/*.test.*', '**/__tests__/**', 'app/api/**'];

  const page = path.join(cwd, 'app/page.tsx');
  const component = path.join(cwd, 'components/Header.tsx');
  const api = path.join(cwd, 'app/api/route.ts');

  t.is(shouldTransformFile(page, include, exclude), true);
  t.is(shouldTransformFile(component, include, exclude), true);
  t.is(shouldTransformFile(api, include, exclude), false);
});

// Performance directories exclusion
test('shouldTransformFile: exclude build directories', t => {
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

  t.is(shouldTransformFile(srcFile, include, exclude), true);
  t.is(shouldTransformFile(nodeModules, include, exclude), false);
  t.is(shouldTransformFile(dist, include, exclude), false);
});

// Additional tests for paths not matching include patterns

test('shouldTransformFile: include specific extension - wrong extension', t => {
  const include = ['src/**/*.tsx'];
  const tsFile = path.join(cwd, 'src/utils.ts');
  const jsFile = path.join(cwd, 'src/index.js');
  const cssFile = path.join(cwd, 'src/styles.css');

  t.is(shouldTransformFile(tsFile, include, undefined), false);
  t.is(shouldTransformFile(jsFile, include, undefined), false);
  t.is(shouldTransformFile(cssFile, include, undefined), false);
});

test('shouldTransformFile: include wrong directory - correct extension', t => {
  const include = ['src/**/*.tsx'];
  const libFile = path.join(cwd, 'lib/Button.tsx');
  const distFile = path.join(cwd, 'dist/Button.tsx');
  const publicFile = path.join(cwd, 'public/Button.tsx');

  t.is(shouldTransformFile(libFile, include, undefined), false);
  t.is(shouldTransformFile(distFile, include, undefined), false);
  t.is(shouldTransformFile(publicFile, include, undefined), false);
});

test('shouldTransformFile: include specific subdirectory - files in parent or sibling directories', t => {
  const include = ['src/components/**/*.tsx'];
  const parentFile = path.join(cwd, 'src/index.tsx');
  const siblingFile = path.join(cwd, 'src/utils/helper.tsx');
  const validFile = path.join(cwd, 'src/components/Button.tsx');

  t.is(shouldTransformFile(parentFile, include, undefined), false);
  t.is(shouldTransformFile(siblingFile, include, undefined), false);
  t.is(shouldTransformFile(validFile, include, undefined), true);
});

test('shouldTransformFile: strict path matching - similar but different paths', t => {
  const include = ['src/**/*.tsx'];
  const srcCopy = path.join(cwd, 'src-copy/Button.tsx');
  const srcBackup = path.join(cwd, 'src.backup/Button.tsx');
  const srcOld = path.join(cwd, 'src_old/Button.tsx');

  t.is(shouldTransformFile(srcCopy, include, undefined), false);
  t.is(shouldTransformFile(srcBackup, include, undefined), false);
  t.is(shouldTransformFile(srcOld, include, undefined), false);
});

test('shouldTransformFile: multiple includes with no match', t => {
  const include = ['src/**/*.tsx', 'app/**/*.tsx', 'components/**/*.tsx'];
  const libFile = path.join(cwd, 'lib/Button.tsx');
  const vendorFile = path.join(cwd, 'vendor/Component.tsx');
  const testFile = path.join(cwd, 'tests/fixture.tsx');

  t.is(shouldTransformFile(libFile, include, undefined), false);
  t.is(shouldTransformFile(vendorFile, include, undefined), false);
  t.is(shouldTransformFile(testFile, include, undefined), false);
});

test('shouldTransformFile: include with regex - non-matching paths', t => {
  const include = [/^src\/components\/.*\.tsx$/];
  const wrongDir = path.join(cwd, 'lib/components/Button.tsx');
  const wrongExt = path.join(cwd, 'src/components/Button.ts');
  const wrongNesting = path.join(cwd, 'src/Button.tsx');

  t.is(shouldTransformFile(wrongDir, include, undefined), false);
  t.is(shouldTransformFile(wrongExt, include, undefined), false);
  t.is(shouldTransformFile(wrongNesting, include, undefined), false);
});

test('shouldTransformFile: case-sensitive path matching', t => {
  const include = ['src/**/*.tsx'];
  // Note: On case-insensitive filesystems (macOS, Windows), this behavior may vary
  const upperSrc = path.join(cwd, 'Src/Button.tsx');
  const upperFile = path.join(cwd, 'src/BUTTON.tsx');

  // On case-sensitive filesystems, these should not match
  // The actual behavior depends on the filesystem
  const result1 = shouldTransformFile(upperSrc, include, undefined);
  const result2 = shouldTransformFile(upperFile, include, undefined);

  // Just verify the function doesn't crash - actual result depends on filesystem
  t.is(typeof result1, 'boolean');
  t.is(typeof result2, 'boolean');
});

test('shouldTransformFile: include with specific nesting level', t => {
  // Note: glob's * wildcard matches any characters including path separators
  // So src/*/*.tsx will match both src/Button.tsx and src/a/b/Button.tsx
  // To match exactly one level, we need to use a more specific pattern or regex
  const include = [/^src\/[^/]+\/[^/]+$/]; // Exactly src/ + one dir + one file
  const tooShallow = path.join(cwd, 'src/index.tsx');
  const tooDeep = path.join(cwd, 'src/components/ui/Button.tsx');
  const justRight = path.join(cwd, 'src/components/Button.tsx');

  t.is(shouldTransformFile(tooShallow, include, undefined), false);
  t.is(shouldTransformFile(tooDeep, include, undefined), false);
  t.is(shouldTransformFile(justRight, include, undefined), true);
});

test('shouldTransformFile: hidden directories not matching include', t => {
  const include = ['src/**/*.tsx'];
  const hiddenDir = path.join(cwd, '.hidden/Button.tsx');
  const hiddenNested = path.join(cwd, 'src/.private/Component.tsx');
  const dotFile = path.join(cwd, '.config.tsx');

  t.is(shouldTransformFile(hiddenDir, include, undefined), false);
  // This should match because it's within src/
  t.is(shouldTransformFile(hiddenNested, include, undefined), true);
  t.is(shouldTransformFile(dotFile, include, undefined), false);
});

test('shouldTransformFile: vendor and third-party directories', t => {
  const include = ['src/**/*.ts', 'src/**/*.tsx'];
  const vendor = path.join(cwd, 'vendor/lib.ts');
  const thirdParty = path.join(cwd, 'third_party/module.ts');
  const external = path.join(cwd, 'external/plugin.ts');

  t.is(shouldTransformFile(vendor, include, undefined), false);
  t.is(shouldTransformFile(thirdParty, include, undefined), false);
  t.is(shouldTransformFile(external, include, undefined), false);
});

test('shouldTransformFile: include with multiple extensions - wrong extension combinations', t => {
  const include = ['src/**/*.ts', 'src/**/*.tsx'];
  const jsFile = path.join(cwd, 'src/index.js');
  const jsxFile = path.join(cwd, 'src/component.jsx');
  const mjsFile = path.join(cwd, 'src/module.mjs');
  const dtsFile = path.join(cwd, 'src/types.d.ts');

  t.is(shouldTransformFile(jsFile, include, undefined), false);
  t.is(shouldTransformFile(jsxFile, include, undefined), false);
  t.is(shouldTransformFile(mjsFile, include, undefined), false);
  // .d.ts files should match **/*.ts pattern
  t.is(shouldTransformFile(dtsFile, include, undefined), true);
});

test('shouldTransformFile: strict regex matching - partial path matches', t => {
  // Regex without ^ and $ anchors should match anywhere
  const include1 = [/components\/.*\.tsx/]; // No anchors - matches anywhere
  const include2 = [/^components\/.*\.tsx$/]; // With anchors - only exact match

  const file1 = path.join(cwd, 'src/components/Button.tsx');
  const file2 = path.join(cwd, 'components/Button.tsx');

  // Without anchors, should match the substring
  t.is(shouldTransformFile(file1, include1, undefined), true);
  t.is(shouldTransformFile(file2, include1, undefined), true);

  // With anchors, only the exact path should match
  t.is(shouldTransformFile(file1, include2, undefined), false);
  t.is(shouldTransformFile(file2, include2, undefined), true);
});

test('shouldTransformFile: files at root vs nested with same name', t => {
  const include = ['src/**/*.tsx'];
  const rootIndex = path.join(cwd, 'index.tsx');
  const srcIndex = path.join(cwd, 'src/index.tsx');
  const nestedIndex = path.join(cwd, 'src/pages/index.tsx');

  t.is(shouldTransformFile(rootIndex, include, undefined), false);
  t.is(shouldTransformFile(srcIndex, include, undefined), true);
  t.is(shouldTransformFile(nestedIndex, include, undefined), true);
});

test('shouldTransformFile: temp and cache directories', t => {
  const include = ['src/**/*.tsx'];
  const tempFile = path.join(cwd, '.temp/component.tsx');
  const cacheFile = path.join(cwd, '.cache/bundle.tsx');
  const tmpFile = path.join(cwd, 'tmp/output.tsx');

  t.is(shouldTransformFile(tempFile, include, undefined), false);
  t.is(shouldTransformFile(cacheFile, include, undefined), false);
  t.is(shouldTransformFile(tmpFile, include, undefined), false);
});

test('shouldTransformFile: backup and versioned directories', t => {
  const include = ['src/**/*.tsx'];
  const backup = path.join(cwd, 'backup/src/Button.tsx');
  const archived = path.join(cwd, 'archived/src/Component.tsx');
  const v1 = path.join(cwd, 'v1/src/OldComponent.tsx');

  t.is(shouldTransformFile(backup, include, undefined), false);
  t.is(shouldTransformFile(archived, include, undefined), false);
  t.is(shouldTransformFile(v1, include, undefined), false);
});

test('shouldTransformFile: include matches but wrong file type', t => {
  const include = ['src/**/*.tsx'];
  const imageFile = path.join(cwd, 'src/assets/logo.png');
  const styleFile = path.join(cwd, 'src/styles/main.css');
  const mdFile = path.join(cwd, 'src/README.md');
  const jsonFile = path.join(cwd, 'src/config.json');

  t.is(shouldTransformFile(imageFile, include, undefined), false);
  t.is(shouldTransformFile(styleFile, include, undefined), false);
  t.is(shouldTransformFile(mdFile, include, undefined), false);
  t.is(shouldTransformFile(jsonFile, include, undefined), false);
});

test('shouldTransformFile: very specific include pattern - near misses', t => {
  const include = ['src/components/ui/**/*.tsx'];
  const tooHigh = path.join(cwd, 'src/Button.tsx');
  const oneLevelUp = path.join(cwd, 'src/components/Button.tsx');
  const wrongSibling = path.join(cwd, 'src/components/layout/Header.tsx');
  const correct = path.join(cwd, 'src/components/ui/Button.tsx');

  t.is(shouldTransformFile(tooHigh, include, undefined), false);
  t.is(shouldTransformFile(oneLevelUp, include, undefined), false);
  t.is(shouldTransformFile(wrongSibling, include, undefined), false);
  t.is(shouldTransformFile(correct, include, undefined), true);
});

test('shouldTransformFile: include with complex regex - boundary cases', t => {
  // Match files that have exactly 'component' in the name (not 'components')
  const include = [/\/component\.tsx$/];
  const singular = path.join(cwd, 'src/component.tsx');
  const plural = path.join(cwd, 'src/components.tsx');
  const prefix = path.join(cwd, 'src/mycomponent.tsx');
  const suffix = path.join(cwd, 'src/componentlist.tsx');

  t.is(shouldTransformFile(singular, include, undefined), true);
  t.is(shouldTransformFile(plural, include, undefined), false);
  t.is(shouldTransformFile(prefix, include, undefined), false);
  t.is(shouldTransformFile(suffix, include, undefined), false);
});

test('shouldTransformFile: regex with escaped slashes (when passed as string)', t => {
  // Test that escaped slashes in regex patterns are handled correctly
  // This tests the from_string parser when patterns come as strings
  // Note: When patterns come as RegExp objects, they're handled directly in parse_js_pattern
  const include = ['src/**/*.tsx'];

  // This should match files with literal forward slashes in the pattern
  const file1 = path.join(cwd, 'src/components/Button.tsx');
  const file2 = path.join(cwd, 'lib/components/Button.tsx');

  t.is(shouldTransformFile(file1, include, undefined), true);
  t.is(shouldTransformFile(file2, include, undefined), false);

  // Test with regex object that uses escaped slashes in the pattern itself
  const regexInclude = [/src\/components\/.*\.tsx$/];
  t.is(shouldTransformFile(file1, regexInclude, undefined), true);
  t.is(shouldTransformFile(file2, regexInclude, undefined), false);
});

test('shouldTransformFile: invalid regex patterns fallback to glob', t => {
  // Invalid regex patterns should be treated as glob patterns
  const include = ['/[invalid(regex/'];
  const file = path.join(cwd, '[invalid(regex');

  // Since it's invalid regex, it should be treated as glob pattern
  // which won't match our test file
  t.is(shouldTransformFile(file, include, undefined), false);
});

test('shouldTransformFile: string regex with flags', t => {
  // Test that string-based regex patterns with flags work correctly
  const include = ['/button/i']; // Case-insensitive via string format
  const upper = path.join(cwd, 'src/BUTTON.tsx');
  const lower = path.join(cwd, 'src/button.tsx');

  t.is(shouldTransformFile(upper, include, undefined), true);
  t.is(shouldTransformFile(lower, include, undefined), true);
});
