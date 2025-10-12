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
  const include = [
    'app/**/*.ts',
    'app/**/*.tsx',
    'components/**/*.ts',
    'components/**/*.tsx',
  ];
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

