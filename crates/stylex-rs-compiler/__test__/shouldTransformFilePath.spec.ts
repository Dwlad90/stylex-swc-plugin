// `shouldTransformFile` answers one question — given a path and a set of
// include/exclude patterns, is this file transformed? — so almost every case is
// a row in a table rather than a hand-written test body. The rows below are
// declared with `test.each`, which reports each one as its own named test, so a
// regression names the exact pattern that broke instead of a line number inside
// a block of twenty assertions.
//
// The handful of cases that are genuinely not table-shaped stay as ordinary
// tests: the ones asserting only a return *type*, and the ones whose whole point
// is calling the function repeatedly with one stateful RegExp.

import * as path from 'path';

import { describe, expect, test } from 'vitest';

import { shouldTransformFile } from '../dist/index';

const cwd = process.cwd();

type PatternList = Parameters<typeof shouldTransformFile>[1];

interface Case {
  /** Names the row in the test reporter. */
  title: string;
  /** Resolved against `cwd`, which is how callers pass paths in practice. */
  file?: string;
  /**
   * Passed through untouched. Only for rows that deliberately exercise a path
   * that is empty or outside the project, where joining would defeat the point.
   */
  rawFile?: string;
  include?: PatternList;
  exclude?: PatternList;
  expected: boolean;
}

const run = ({ file, rawFile, include, exclude }: Case) =>
  shouldTransformFile(rawFile ?? path.join(cwd, file ?? ''), include, exclude);

/**
 * Declares one test per row, named by the row’s `title`. Kept separate from
 * `describe` so each group keeps a literal title, which is both greppable and
 * what `vitest/valid-title` requires.
 */
const rows = (cases: Case[]) =>
  test.each(cases)('$title', row => {
    expect(run(row)).toBe(row.expected);
  });

describe('no patterns', () => {
  rows([
    { title: 'undefined include and exclude', file: 'src/Button.tsx', expected: true },
    {
      title: 'null include and exclude',
      file: 'src/Button.tsx',
      include: null,
      exclude: null,
      expected: true,
    },
    {
      title: 'empty include and exclude arrays',
      file: 'src/Button.tsx',
      include: [],
      exclude: [],
      expected: true,
    },
    {
      title: 'empty include array matches everything',
      file: 'any/file.ts',
      include: [],
      expected: true,
    },
    {
      title: 'empty exclude array excludes nothing',
      file: 'any/file.ts',
      exclude: [],
      expected: true,
    },
  ]);
});

describe('include patterns', () => {
  rows([
    {
      title: 'glob matches',
      file: 'src/components/Button.tsx',
      include: ['src/**/*.tsx'],
      expected: true,
    },
    {
      title: 'glob does not match',
      file: 'lib/components/Button.tsx',
      include: ['src/**/*.tsx'],
      expected: false,
    },
    {
      title: 'regex matches',
      file: 'src/components/Button.tsx',
      include: [/src\/.*\.tsx$/],
      expected: true,
    },
    {
      title: 'regex does not match',
      file: 'lib/components/Button.tsx',
      include: [/^src\/.*\.tsx$/],
      expected: false,
    },
    {
      title: 'several patterns, first matches',
      file: 'src/Button.tsx',
      include: ['src/**/*.tsx', 'app/**/*.tsx'],
      expected: true,
    },
    {
      title: 'several patterns, second matches',
      file: 'app/Button.tsx',
      include: ['src/**/*.tsx', 'app/**/*.tsx'],
      expected: true,
    },
    {
      title: 'several patterns, none match',
      file: 'lib/Button.tsx',
      include: ['src/**/*.tsx', 'app/**/*.tsx'],
      expected: false,
    },
    {
      title: 'root-level file with a bare glob',
      file: 'index.tsx',
      include: ['*.tsx'],
      expected: true,
    },
    {
      title: 'nested directories',
      file: 'src/nested/deep/Component.tsx',
      include: ['src/**/*.tsx'],
      expected: true,
    },
  ]);
});

describe('exclude patterns', () => {
  rows([
    {
      title: 'glob matches',
      file: 'src/Button.test.tsx',
      exclude: ['**/*.test.tsx'],
      expected: false,
    },
    {
      title: 'glob does not match',
      file: 'src/Button.tsx',
      exclude: ['**/*.test.tsx'],
      expected: true,
    },
    {
      title: 'regex matches',
      file: 'src/Button.test.tsx',
      exclude: [/\.test\.tsx$/],
      expected: false,
    },
    {
      title: 'regex does not match',
      file: 'src/Button.tsx',
      exclude: [/\.test\.tsx$/],
      expected: true,
    },
    {
      title: 'several patterns: .test is excluded',
      file: 'src/Button.test.tsx',
      exclude: ['**/*.test.tsx', '**/*.spec.tsx'],
      expected: false,
    },
    {
      title: 'several patterns: .spec is excluded',
      file: 'src/Button.spec.tsx',
      exclude: ['**/*.test.tsx', '**/*.spec.tsx'],
      expected: false,
    },
    {
      title: 'several patterns: a plain file survives',
      file: 'src/Button.tsx',
      exclude: ['**/*.test.tsx', '**/*.spec.tsx'],
      expected: true,
    },
    {
      title: 'a file with several dots still matches *.test.tsx',
      file: 'src/Button.component.test.tsx',
      exclude: ['**/*.test.tsx'],
      expected: false,
    },
  ]);
});

const COMBINED_INCLUDE = ['src/**/*.tsx'];
const COMBINED_EXCLUDE = ['**/*.test.tsx'];

// Note: glob doesn't support brace expansion, use separate patterns
const COMPLEX_INCLUDE = ['src/**/*.ts', 'src/**/*.tsx', 'app/**/*.tsx'];
const COMPLEX_EXCLUDE = ['**/*.test.*', '**/*.spec.*', '**/__mocks__/**'];

const MIXED_INCLUDE = ['src/**/*.tsx', /app\/.*\.tsx$/];
const MIXED_EXCLUDE = ['**/*.test.*', /\.(stories|spec)\./];

describe('include combined with exclude', () => {
  rows([
    {
      title: 'include matches and exclude does not',
      file: 'src/Button.tsx',
      include: COMBINED_INCLUDE,
      exclude: COMBINED_EXCLUDE,
      expected: true,
    },
    {
      title: 'include matches but exclude also matches',
      file: 'src/Button.test.tsx',
      include: COMBINED_INCLUDE,
      exclude: COMBINED_EXCLUDE,
      expected: false,
    },
    {
      title: 'include does not match',
      file: 'lib/Button.tsx',
      include: COMBINED_INCLUDE,
      exclude: COMBINED_EXCLUDE,
      expected: false,
    },
    {
      title: 'exclude takes precedence over include',
      file: 'src/__tests__/Button.tsx',
      include: ['src/**/*.tsx'],
      exclude: ['**/__tests__/**'],
      expected: false,
    },
    {
      title: 'same regex in include and exclude resolves to excluded',
      file: 'src/App.tsx',
      include: [/src\/.*\.tsx$/],
      exclude: [/src\/.*\.tsx$/],
      expected: false,
    },
    {
      title: 'glob include with regex exclude keeps the non-test file',
      file: 'src/utils/helper.ts',
      include: ['**/*.ts'],
      exclude: [/\.test\./],
      expected: true,
    },
    {
      title: 'glob include with regex exclude drops the test file',
      file: 'src/utils/helper.test.ts',
      include: ['**/*.ts'],
      exclude: [/\.test\./],
      expected: false,
    },
    {
      title: 'complex: src component is included',
      file: 'src/components/Button.tsx',
      include: COMPLEX_INCLUDE,
      exclude: COMPLEX_EXCLUDE,
      expected: true,
    },
    {
      title: 'complex: app page is included',
      file: 'app/pages/index.tsx',
      include: COMPLEX_INCLUDE,
      exclude: COMPLEX_EXCLUDE,
      expected: true,
    },
    {
      title: 'complex: test file is excluded',
      file: 'src/components/Button.test.tsx',
      include: COMPLEX_INCLUDE,
      exclude: COMPLEX_EXCLUDE,
      expected: false,
    },
    {
      title: 'complex: mock file is excluded',
      file: 'src/__mocks__/Button.tsx',
      include: COMPLEX_INCLUDE,
      exclude: COMPLEX_EXCLUDE,
      expected: false,
    },
    {
      title: 'complex: file outside include is excluded',
      file: 'lib/components/Button.tsx',
      include: COMPLEX_INCLUDE,
      exclude: COMPLEX_EXCLUDE,
      expected: false,
    },
    {
      title: 'mixed glob and regex: glob side matches',
      file: 'src/Button.tsx',
      include: MIXED_INCLUDE,
      exclude: MIXED_EXCLUDE,
      expected: true,
    },
    {
      title: 'mixed glob and regex: regex side matches',
      file: 'app/page.tsx',
      include: MIXED_INCLUDE,
      exclude: MIXED_EXCLUDE,
      expected: true,
    },
  ]);
});

const EXCEPT_OPEN_PROPS = [/node_modules(?!\/@stylexjs\/open-props)/];
const EXCEPT_STYLEXJS_SCOPE = [/node_modules(?!\/@stylexjs)/];

describe('regex lookaround', () => {
  rows([
    {
      title: 'negative lookahead excludes an unrelated node_modules package',
      file: 'node_modules/some-package/index.js',
      exclude: EXCEPT_OPEN_PROPS,
      expected: false,
    },
    {
      title: 'negative lookahead spares the named package',
      file: 'node_modules/@stylexjs/open-props/index.js',
      exclude: EXCEPT_OPEN_PROPS,
      expected: true,
    },
    {
      title: 'negative lookahead leaves src untouched',
      file: 'src/index.tsx',
      exclude: EXCEPT_OPEN_PROPS,
      expected: true,
    },
    {
      title: 'scope lookahead excludes an unrelated package',
      file: 'node_modules/some-package/index.js',
      exclude: EXCEPT_STYLEXJS_SCOPE,
      expected: false,
    },
    {
      title: 'scope lookahead spares @stylexjs/stylex',
      file: 'node_modules/@stylexjs/stylex/index.js',
      exclude: EXCEPT_STYLEXJS_SCOPE,
      expected: true,
    },
    {
      title: 'scope lookahead spares @stylexjs/open-props',
      file: 'node_modules/@stylexjs/open-props/index.js',
      exclude: EXCEPT_STYLEXJS_SCOPE,
      expected: true,
    },
    {
      title: 'positive lookahead includes only .test before .tsx',
      file: 'src/Button.test.tsx',
      include: [/.*\.test(?=\.tsx$)/],
      expected: true,
    },
    {
      title: 'positive lookahead rejects a plain component',
      file: 'src/Button.tsx',
      include: [/.*\.test(?=\.tsx$)/],
      expected: false,
    },
    // Negative lookahead at the start stands in for a lookbehind: it checks
    // whether the string begins with src/, rather than looking backwards from a
    // match found mid-string.
    {
      title: 'leading negative lookahead spares paths under src/',
      file: 'src/components/Button.tsx',
      exclude: [/^(?!src\/).*\.tsx$/],
      expected: true,
    },
    {
      title: 'leading negative lookahead excludes paths outside src/',
      file: 'lib/components/Button.tsx',
      exclude: [/^(?!src\/).*\.tsx$/],
      expected: false,
    },
  ]);
});

const REACT_INCLUDE = ['src/**/*.ts', 'src/**/*.tsx'];
const REACT_EXCLUDE = [
  '**/*.test.ts',
  '**/*.test.tsx',
  '**/*.spec.ts',
  '**/*.spec.tsx',
  '**/*.stories.ts',
  '**/*.stories.tsx',
  '**/__tests__/**',
  '**/__mocks__/**',
];

const MONOREPO_INCLUDE = [
  'packages/*/src/**/*.ts',
  'packages/*/src/**/*.tsx',
  'apps/*/src/**/*.ts',
  'apps/*/src/**/*.tsx',
];
const MONOREPO_EXCLUDE = ['**/*.test.*', '**/node_modules/**'];

const NEXT_INCLUDE = ['app/**/*.ts', 'app/**/*.tsx', 'components/**/*.ts', 'components/**/*.tsx'];
const NEXT_EXCLUDE = ['**/*.test.*', '**/__tests__/**', 'app/api/**'];

const BUILD_INCLUDE = ['**/*.ts', '**/*.tsx'];
const BUILD_EXCLUDE = [
  '**/node_modules/**',
  '**/dist/**',
  '**/build/**',
  '**/.next/**',
  '**/coverage/**',
];

describe('real-world configurations', () => {
  rows([
    {
      title: 'React: component is transformed',
      file: 'src/components/Button.tsx',
      include: REACT_INCLUDE,
      exclude: REACT_EXCLUDE,
      expected: true,
    },
    {
      title: 'React: test is skipped',
      file: 'src/components/Button.test.tsx',
      include: REACT_INCLUDE,
      exclude: REACT_EXCLUDE,
      expected: false,
    },
    {
      title: 'React: story is skipped',
      file: 'src/components/Button.stories.tsx',
      include: REACT_INCLUDE,
      exclude: REACT_EXCLUDE,
      expected: false,
    },
    {
      title: 'monorepo: package source is transformed',
      file: 'packages/ui/src/Button.tsx',
      include: MONOREPO_INCLUDE,
      exclude: MONOREPO_EXCLUDE,
      expected: true,
    },
    {
      title: 'monorepo: app source is transformed',
      file: 'apps/web/src/App.tsx',
      include: MONOREPO_INCLUDE,
      exclude: MONOREPO_EXCLUDE,
      expected: true,
    },
    {
      title: 'monorepo: package test is skipped',
      file: 'packages/ui/src/Button.test.tsx',
      include: MONOREPO_INCLUDE,
      exclude: MONOREPO_EXCLUDE,
      expected: false,
    },
    {
      title: 'Next.js: app page is transformed',
      file: 'app/page.tsx',
      include: NEXT_INCLUDE,
      exclude: NEXT_EXCLUDE,
      expected: true,
    },
    {
      title: 'Next.js: component is transformed',
      file: 'components/Header.tsx',
      include: NEXT_INCLUDE,
      exclude: NEXT_EXCLUDE,
      expected: true,
    },
    {
      title: 'Next.js: api route is skipped',
      file: 'app/api/route.ts',
      include: NEXT_INCLUDE,
      exclude: NEXT_EXCLUDE,
      expected: false,
    },
    {
      title: 'build dirs: source is transformed',
      file: 'src/index.tsx',
      include: BUILD_INCLUDE,
      exclude: BUILD_EXCLUDE,
      expected: true,
    },
    {
      title: 'build dirs: node_modules is skipped',
      file: 'node_modules/react/index.ts',
      include: BUILD_INCLUDE,
      exclude: BUILD_EXCLUDE,
      expected: false,
    },
    {
      title: 'build dirs: dist is skipped',
      file: 'dist/bundle.js',
      include: BUILD_INCLUDE,
      exclude: BUILD_EXCLUDE,
      expected: false,
    },
  ]);
});

const TSX_UNDER_SRC = ['src/**/*.tsx'];
const TS_AND_TSX_UNDER_SRC = ['src/**/*.ts', 'src/**/*.tsx'];

describe('paths outside the include set', () => {
  rows([
    {
      title: 'wrong extension: .ts',
      file: 'src/utils.ts',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'wrong extension: .js',
      file: 'src/index.js',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'wrong extension: .css',
      file: 'src/styles.css',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'wrong directory: lib',
      file: 'lib/Button.tsx',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'wrong directory: dist',
      file: 'dist/Button.tsx',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'wrong directory: public',
      file: 'public/Button.tsx',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'subdirectory include: parent file does not match',
      file: 'src/index.tsx',
      include: ['src/components/**/*.tsx'],
      expected: false,
    },
    {
      title: 'subdirectory include: sibling file does not match',
      file: 'src/utils/helper.tsx',
      include: ['src/components/**/*.tsx'],
      expected: false,
    },
    {
      title: 'subdirectory include: the target file matches',
      file: 'src/components/Button.tsx',
      include: ['src/components/**/*.tsx'],
      expected: true,
    },
    {
      title: 'similar directory: src-copy',
      file: 'src-copy/Button.tsx',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'similar directory: src.backup',
      file: 'src.backup/Button.tsx',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'similar directory: src_old',
      file: 'src_old/Button.tsx',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'several includes, no match: lib',
      file: 'lib/Button.tsx',
      include: ['src/**/*.tsx', 'app/**/*.tsx', 'components/**/*.tsx'],
      expected: false,
    },
    {
      title: 'several includes, no match: vendor',
      file: 'vendor/Component.tsx',
      include: ['src/**/*.tsx', 'app/**/*.tsx', 'components/**/*.tsx'],
      expected: false,
    },
    {
      title: 'several includes, no match: tests',
      file: 'tests/fixture.tsx',
      include: ['src/**/*.tsx', 'app/**/*.tsx', 'components/**/*.tsx'],
      expected: false,
    },
    {
      title: 'anchored regex include: wrong directory',
      file: 'lib/components/Button.tsx',
      include: [/^src\/components\/.*\.tsx$/],
      expected: false,
    },
    {
      title: 'anchored regex include: wrong extension',
      file: 'src/components/Button.ts',
      include: [/^src\/components\/.*\.tsx$/],
      expected: false,
    },
    {
      title: 'anchored regex include: wrong nesting',
      file: 'src/Button.tsx',
      include: [/^src\/components\/.*\.tsx$/],
      expected: false,
    },
    // `*` in a glob spans path separators here, so exact nesting needs a regex.
    {
      title: 'exact nesting regex: too shallow',
      file: 'src/index.tsx',
      include: [/^src\/[^/]+\/[^/]+$/],
      expected: false,
    },
    {
      title: 'exact nesting regex: too deep',
      file: 'src/components/ui/Button.tsx',
      include: [/^src\/[^/]+\/[^/]+$/],
      expected: false,
    },
    {
      title: 'exact nesting regex: exactly right',
      file: 'src/components/Button.tsx',
      include: [/^src\/[^/]+\/[^/]+$/],
      expected: true,
    },
    {
      title: 'hidden directory outside src',
      file: '.hidden/Button.tsx',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'hidden directory inside src still matches',
      file: 'src/.private/Component.tsx',
      include: TSX_UNDER_SRC,
      expected: true,
    },
    { title: 'dotfile at root', file: '.config.tsx', include: TSX_UNDER_SRC, expected: false },
    {
      title: 'vendor directory',
      file: 'vendor/lib.ts',
      include: TS_AND_TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'third_party directory',
      file: 'third_party/module.ts',
      include: TS_AND_TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'external directory',
      file: 'external/plugin.ts',
      include: TS_AND_TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'ts/tsx include rejects .js',
      file: 'src/index.js',
      include: TS_AND_TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'ts/tsx include rejects .jsx',
      file: 'src/component.jsx',
      include: TS_AND_TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'ts/tsx include rejects .mjs',
      file: 'src/module.mjs',
      include: TS_AND_TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'ts/tsx include accepts .d.ts',
      file: 'src/types.d.ts',
      include: TS_AND_TSX_UNDER_SRC,
      expected: true,
    },
    {
      title: 'root index is outside src/**',
      file: 'index.tsx',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'src index is inside src/**',
      file: 'src/index.tsx',
      include: TSX_UNDER_SRC,
      expected: true,
    },
    {
      title: 'nested index is inside src/**',
      file: 'src/pages/index.tsx',
      include: TSX_UNDER_SRC,
      expected: true,
    },
    {
      title: 'temp directory',
      file: '.temp/component.tsx',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'cache directory',
      file: '.cache/bundle.tsx',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    { title: 'tmp directory', file: 'tmp/output.tsx', include: TSX_UNDER_SRC, expected: false },
    {
      title: 'backup directory',
      file: 'backup/src/Button.tsx',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'archived directory',
      file: 'archived/src/Component.tsx',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'versioned directory',
      file: 'v1/src/OldComponent.tsx',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'non-code file: image',
      file: 'src/assets/logo.png',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'non-code file: stylesheet',
      file: 'src/styles/main.css',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'non-code file: markdown',
      file: 'src/README.md',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'non-code file: json',
      file: 'src/config.json',
      include: TSX_UNDER_SRC,
      expected: false,
    },
    {
      title: 'deep include: file too high',
      file: 'src/Button.tsx',
      include: ['src/components/ui/**/*.tsx'],
      expected: false,
    },
    {
      title: 'deep include: file one level up',
      file: 'src/components/Button.tsx',
      include: ['src/components/ui/**/*.tsx'],
      expected: false,
    },
    {
      title: 'deep include: sibling directory',
      file: 'src/components/layout/Header.tsx',
      include: ['src/components/ui/**/*.tsx'],
      expected: false,
    },
    {
      title: 'deep include: the target file',
      file: 'src/components/ui/Button.tsx',
      include: ['src/components/ui/**/*.tsx'],
      expected: true,
    },
  ]);
});

describe('regex matching semantics', () => {
  rows([
    // `/component.tsx$` must not match `components.tsx`, `mycomponent.tsx`, …
    {
      title: 'word boundary: exact filename matches',
      file: 'src/component.tsx',
      include: [/\/component\.tsx$/],
      expected: true,
    },
    {
      title: 'word boundary: plural does not match',
      file: 'src/components.tsx',
      include: [/\/component\.tsx$/],
      expected: false,
    },
    {
      title: 'word boundary: prefixed name does not match',
      file: 'src/mycomponent.tsx',
      include: [/\/component\.tsx$/],
      expected: false,
    },
    {
      title: 'word boundary: suffixed name does not match',
      file: 'src/componentlist.tsx',
      include: [/\/component\.tsx$/],
      expected: false,
    },
    {
      title: 'unanchored regex matches a nested path',
      file: 'src/components/Button.tsx',
      include: [/components\/.*\.tsx/],
      expected: true,
    },
    {
      title: 'unanchored regex matches a root path',
      file: 'components/Button.tsx',
      include: [/components\/.*\.tsx/],
      expected: true,
    },
    {
      title: 'anchored regex rejects a nested path',
      file: 'src/components/Button.tsx',
      include: [/^components\/.*\.tsx$/],
      expected: false,
    },
    {
      title: 'anchored regex accepts the exact path',
      file: 'components/Button.tsx',
      include: [/^components\/.*\.tsx$/],
      expected: true,
    },
    {
      title: 'glob include with slashes matches under src',
      file: 'src/components/Button.tsx',
      include: ['src/**/*.tsx'],
      expected: true,
    },
    {
      title: 'glob include with slashes rejects lib',
      file: 'lib/components/Button.tsx',
      include: ['src/**/*.tsx'],
      expected: false,
    },
    {
      title: 'regex with escaped slashes matches under src',
      file: 'src/components/Button.tsx',
      include: [/src\/components\/.*\.tsx$/],
      expected: true,
    },
    {
      title: 'regex with escaped slashes rejects lib',
      file: 'lib/components/Button.tsx',
      include: [/src\/components\/.*\.tsx$/],
      expected: false,
    },
    // An unparsable regex string is treated as a glob, which then fails to match.
    {
      title: 'invalid regex string falls back to glob and does not match',
      file: '[invalid(regex',
      include: ['/[invalid(regex/'],
      expected: false,
    },
    {
      title: 'case-insensitive flag matches uppercase',
      file: 'src/BUTTON.tsx',
      include: [/button/i],
      expected: true,
    },
    {
      title: 'case-insensitive flag matches lowercase',
      file: 'src/button.tsx',
      include: [/button/i],
      expected: true,
    },
    {
      title: 'escaped dots match the full compound extension',
      file: 'src/file.module.css.ts',
      include: [/\.module\.css\.ts$/],
      expected: true,
    },
    {
      title: 'escaped dots reject a partial compound extension',
      file: 'src/file.module.css.ts',
      include: [/\.module\.css$/],
      expected: false,
    },
    {
      title: 'anchored prefix matches',
      file: 'src/Button.tsx',
      include: [/^src\//],
      expected: true,
    },
    {
      title: 'anchored suffix matches',
      file: 'src/Button.tsx',
      include: [/\.tsx$/],
      expected: true,
    },
    {
      title: 'anchored prefix for a different directory does not match',
      file: 'src/Button.tsx',
      include: [/^lib\//],
      expected: false,
    },
    {
      title: 'exclude regex anchored to the relative path',
      file: 'vendor/third-party/lib.ts',
      exclude: [/^vendor\//],
      expected: false,
    },
    {
      title: 'exclude regex matching mid-path',
      file: 'vendor/third-party/lib.ts',
      exclude: [/third-party/],
      expected: false,
    },
  ]);
});

describe('path edge cases', () => {
  rows([
    { title: 'empty file path', rawFile: '', include: ['**/*.ts'], expected: false },
    // `path.relative` yields a `../..` path here, which must not match `src/**`.
    {
      title: 'absolute path outside cwd',
      rawFile: '/completely/different/path.ts',
      include: ['src/**'],
      expected: false,
    },
    { title: 'dotfile', file: '.eslintrc.ts', include: ['**/*.ts'], expected: true },
    { title: 'dot directory', file: '.config/styles.ts', include: ['.config/**'], expected: true },
    {
      title: 'spaces in the path',
      file: 'src/My Component/styles.tsx',
      include: ['src/**/*.tsx'],
      expected: true,
    },
    {
      title: 'unicode in the path',
      file: 'src/компонент/styles.tsx',
      include: ['src/**/*.tsx'],
      expected: true,
    },
    {
      title: 'deeply nested path matches a globstar',
      file: 'src/a/b/c/d/e/f/g/h/styles.tsx',
      include: ['src/**/*.tsx'],
      expected: true,
    },
    {
      title: 'deeply nested path does not match a single-level glob',
      file: 'src/a/b/c/d/e/f/g/h/styles.tsx',
      include: ['src/*.tsx'],
      expected: false,
    },
    { title: 'both arrays empty', file: 'anything.tsx', include: [], exclude: [], expected: true },
    {
      title: 'single-file glob matches that file',
      file: 'src/specific-file.tsx',
      include: ['src/specific-file.tsx'],
      expected: true,
    },
    {
      title: 'single-file glob rejects another file',
      file: 'src/specific-file.tsx',
      include: ['src/other-file.tsx'],
      expected: false,
    },
    // Glob patterns are case-sensitive by default.
    {
      title: 'glob case sensitivity: lowercase pattern misses uppercase extension',
      file: 'src/MyComponent.TSX',
      include: ['src/**/*.tsx'],
      expected: false,
    },
    {
      title: 'glob case sensitivity: matching case succeeds',
      file: 'src/MyComponent.TSX',
      include: ['src/**/*.TSX'],
      expected: true,
    },
  ]);
});

describe('malformed and empty patterns', () => {
  test('a non-pattern entry is skipped and the valid pattern still applies', () => {
    const filePath = path.join(cwd, 'src/Button.tsx');
    const include = [123, 'src/**/*.tsx'];
    // @ts-expect-error - invalid pattern type
    const result = shouldTransformFile(filePath, include, undefined);
    expect(result).toBe(true);
  });

  test('an empty string pattern matches nothing', () => {
    const filePath = path.join(cwd, 'src/Button.tsx');
    const result = shouldTransformFile(filePath, [''], undefined);
    expect(result).toBe(false);
  });
});

describe('filesystem case sensitivity', () => {
  // Case folding depends on the filesystem (macOS and Windows fold, Linux does
  // not), so only the contract — a boolean, no crash — can be asserted here.
  test.each([
    { title: 'uppercase directory', file: 'Src/Button.tsx' },
    { title: 'uppercase filename', file: 'src/BUTTON.tsx' },
  ])('$title returns a boolean whatever the filesystem does', ({ file }) => {
    expect(typeof shouldTransformFile(path.join(cwd, file), ['src/**/*.tsx'], undefined)).toBe(
      'boolean'
    );
  });
});

// These cannot become table rows: the assertion is about calling the function
// repeatedly with one RegExp instance. `test`/`exec` on a `/g` or `/y` regex
// advances `lastIndex`, so a second call returns false unless the callee resets
// it. One row per call would construct a fresh pattern and prove nothing.
describe('stateful regex flags stay deterministic across calls', () => {
  test('/g flag in include', () => {
    const include = [/src/g];
    const filePath = path.join(cwd, 'src/Button.tsx');

    expect(shouldTransformFile(filePath, include, undefined)).toBe(true);
    expect(shouldTransformFile(filePath, include, undefined)).toBe(true);
    expect(shouldTransformFile(filePath, include, undefined)).toBe(true);
  });

  test('/y flag in include', () => {
    const include = [/src/y];
    const filePath = path.join(cwd, 'src/Button.tsx');

    expect(shouldTransformFile(filePath, include, undefined)).toBe(true);
    expect(shouldTransformFile(filePath, include, undefined)).toBe(true);
  });

  test('/gi flags in include', () => {
    const include = [/BUTTON/gi];
    const filePath = path.join(cwd, 'src/Button.tsx');

    for (let i = 0; i < 10; i++) {
      expect(shouldTransformFile(filePath, include, undefined), `failed on call ${i + 1}`).toBe(
        true
      );
    }
  });

  test('/g flag in exclude', () => {
    const exclude = [/test/g];
    const filePath = path.join(cwd, 'src/test-utils.ts');

    for (let i = 0; i < 5; i++) {
      expect(
        shouldTransformFile(filePath, undefined, exclude),
        `exclude with /g should be consistent on call ${i + 1}`
      ).toBe(false);
    }
  });
});
