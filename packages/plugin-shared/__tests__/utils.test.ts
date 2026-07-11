import path from 'path';
import { describe, expect, test } from 'vitest';

import { DEFAULT_STYLEX_PACKAGES, VIRTUAL_CSS_PATTERN } from '../src/constants';
import {
  buildVirtualCssPattern,
  isAllowlistedPackage,
  parseStylexRulesFromIdentifier,
  shouldProcessFile,
} from '../src/utils';

import type { Rule as StyleXRule } from '@stylexjs/babel-plugin';

describe('shouldProcessFile', () => {
  const project = path.join(path.sep, 'project');
  const inNodeModules = (...segments: string[]) => path.join(project, 'node_modules', ...segments);
  const defaults = { stylexPackages: DEFAULT_STYLEX_PACKAGES };

  test('scopes node_modules to the stylexPackages allowlist', () => {
    expect(shouldProcessFile(path.join(project, 'app', 'page.tsx'), defaults)).toBe(true);
    expect(shouldProcessFile(inNodeModules('react', 'index.js'), defaults)).toBe(false);
    expect(
      shouldProcessFile(inNodeModules('@stylexjs', 'open-props', 'lib', 'colors.js'), defaults)
    ).toBe(true);
    // pnpm layout: the final `node_modules/@stylexjs/...` segment still matches
    expect(
      shouldProcessFile(
        inNodeModules(
          '.pnpm',
          '@stylexjs+open-props@0.11.1',
          'node_modules',
          '@stylexjs',
          'open-props',
          'lib',
          'colors.js'
        ),
        defaults
      )
    ).toBe(true);
    expect(shouldProcessFile(path.join(project, 'app', 'styles.css'), defaults)).toBe(false);
  });

  test('matches exact stylexPackages entries only', () => {
    const custom = { stylexPackages: ['@stylexjs/', 'my-design-system'] };

    expect(shouldProcessFile(inNodeModules('my-design-system', 'tokens.js'), custom)).toBe(true);
    expect(shouldProcessFile(inNodeModules('my-design-system-extra', 'tokens.js'), custom)).toBe(
      false
    );
    expect(shouldProcessFile(inNodeModules('other-lib', 'tokens.js'), custom)).toBe(false);
  });
});

describe('isAllowlistedPackage', () => {
  test('requires a node_modules segment', () => {
    expect(isAllowlistedPackage(path.join(path.sep, 'app', 'page.tsx'), ['@stylexjs/'])).toBe(
      false
    );
  });
});

describe('buildVirtualCssPattern', () => {
  test('defaults to the packaged carrier + dummy pattern', () => {
    expect(buildVirtualCssPattern()).toBe(VIRTUAL_CSS_PATTERN);
  });

  test('a custom carrier path replaces the default carrier match', () => {
    const pattern = buildVirtualCssPattern('/project/src/styles/my-carrier.css');

    expect(pattern.test('/project/src/styles/my-carrier.css')).toBe(true);
    // dummies always belong to the chunk
    expect(pattern.test('/repo/plugin-shared/dist/stylex-virtual.css?from=App.js')).toBe(true);
    // the default packaged carrier no longer matches (avoids false positives)
    expect(pattern.test('/repo/webpack-plugin/dist/stylex.css')).toBe(false);
    // regex metacharacters in the path are escaped
    expect(pattern.test('/project/src/styles/myXcarrierXcss')).toBe(false);
  });
});

describe('parseStylexRulesFromIdentifier', () => {
  test('parses StyleX rules from `|`-segmented css module identifiers', () => {
    const rules: StyleXRule[] = [['x1abcd', { ltr: '.x1abcd{color:red}', rtl: null }, 3000]];
    const query = new URLSearchParams({
      from: 'app/layout.tsx',
      stylex: JSON.stringify(rules),
    });
    const identifier = `css|/repo/node_modules/@stylexswc/plugin-shared/dist/stylex-virtual.css?${query.toString()}|used-exports`;

    expect(parseStylexRulesFromIdentifier(identifier)).toEqual(rules);
    expect(parseStylexRulesFromIdentifier('css|/repo/app/global.css|used-exports')).toBeNull();
    expect(
      parseStylexRulesFromIdentifier('css|/repo/dist/stylex-virtual.css?from=app/layout.tsx')
    ).toBeNull();
  });

  test('ignores the carrier stylesheet', () => {
    expect(
      parseStylexRulesFromIdentifier('css|/repo/node_modules/@stylexswc/plugin-shared/dist/stylex.css|used-exports')
    ).toBeNull();
  });
});
