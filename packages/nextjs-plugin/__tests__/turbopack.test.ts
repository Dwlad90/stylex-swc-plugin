import { INCLUDE_EXTENSIONS, INCLUDE_REGEXP } from '@stylexswc/plugin-shared';
import type {
  NextConfig,
  TurbopackLoaderItem,
  TurbopackRuleConfigCollection,
  TurbopackRuleConfigItem,
} from 'next/dist/server/config-shared';
import { describe, expect, test } from 'vitest';

import withStyleX from '../src/turbopack';

const LOADER_PATH = '@stylexswc/turbopack-plugin/loader';

const stylexLoader = (options: Record<string, unknown> = {}) => ({
  loader: LOADER_PATH,
  options,
});

function buildRules(
  nextConfig: NextConfig = {},
  pluginOptions?: Parameters<typeof withStyleX>[0]
): Record<string, TurbopackRuleConfigCollection> {
  return withStyleX(pluginOptions)(nextConfig).turbopack?.rules ?? {};
}

/** Reads the one rule that a glob holds, for the globs that hold only one. */
const ruleAt = (
  rules: Record<string, TurbopackRuleConfigCollection>,
  glob: string
): TurbopackRuleConfigItem => rules[glob] as TurbopackRuleConfigItem;

describe('turbopack rules', () => {
  test('registers one rule for every transformable extension', () => {
    const globs = Object.keys(buildRules());

    // Compared as sets, because the order of the rules does not matter.
    expect(new Set(globs)).toEqual(new Set(INCLUDE_EXTENSIONS.map(ext => `*.${ext}`)));
    expect(globs).toHaveLength(INCLUDE_EXTENSIONS.length);
  });

  // The bug this guards: the rules once listed only .ts, .tsx, .js and .jsx,
  // so StyleX that shipped in an .mjs or .cjs module never reached the loader.
  test.each(['mjs', 'cjs', 'mts', 'cts'])('covers the .%s extension', extension => {
    expect(buildRules()).toHaveProperty(`*.${extension}`);
  });

  test('gives the loader to every rule', () => {
    for (const rule of Object.values(buildRules())) {
      expect((rule as TurbopackRuleConfigItem).loaders).toEqual([stylexLoader()]);
    }
  });

  // Every glob that the plugin writes must name a path that the loader
  // accepts. The glob and the matcher come from one list, and this holds the
  // two together.
  test('names a glob that the path matcher agrees with', () => {
    for (const glob of Object.keys(buildRules())) {
      expect(glob.startsWith('*.')).toBe(true);

      // The glob '*.mjs' stands for a path such as '/src/module.mjs'.
      expect(INCLUDE_REGEXP.test(`/src/module${glob.slice(1)}`)).toBe(true);
    }
  });

  test('keeps the rest of the Next.js config', () => {
    const nextConfig = { reactStrictMode: true, transpilePackages: ['@stylexjs/open-props'] };

    expect(withStyleX()(nextConfig)).toMatchObject(nextConfig);
  });

  test('keeps the other Turbopack options', () => {
    const nextConfig = { turbopack: { resolveAlias: { underscore: 'lodash' } } };

    expect(withStyleX()(nextConfig).turbopack?.resolveAlias).toEqual({ underscore: 'lodash' });
  });

  test('accepts a missing config', () => {
    expect(Object.keys(withStyleX()().turbopack?.rules ?? {})).toHaveLength(
      INCLUDE_EXTENSIONS.length
    );
  });

  test('accepts a config whose turbopack field is absent', () => {
    expect(ruleAt(buildRules({ reactStrictMode: true }), '*.mjs').loaders).toEqual([
      stylexLoader(),
    ]);
  });
});

describe('loader options', () => {
  test('passes the plugin options to the loader', () => {
    const pluginOptions = { rsOptions: { dev: true }, stylexImports: ['stylex'] };

    expect(ruleAt(buildRules({}, pluginOptions), '*.mjs').loaders).toEqual([
      stylexLoader(pluginOptions),
    ]);
  });

  test('keeps loaderOrder out of the loader options', () => {
    const rules = buildRules({}, { rsOptions: { dev: true }, loaderOrder: 'last' });

    expect(ruleAt(rules, '*.mjs').loaders).toEqual([stylexLoader({ rsOptions: { dev: true } })]);
  });

  // Next.js rejects a rule whose options do not survive a JSON round trip.
  test('produces options that JSON can carry', () => {
    const rules = buildRules({}, { rsOptions: { dev: true }, stylexImports: ['stylex'] });

    expect(JSON.parse(JSON.stringify(rules))).toEqual(rules);
  });

  test('gives each rule its own loader options object', () => {
    const rules = buildRules({}, { rsOptions: { dev: true } });
    const first = ruleAt(rules, '*.js').loaders?.[0] as unknown as {
      options: Record<string, unknown>;
    };
    const second = ruleAt(rules, '*.mjs').loaders?.[0] as unknown as {
      options: Record<string, unknown>;
    };

    first.options.dev = 'changed';

    expect(second.options).toEqual({ rsOptions: { dev: true } });
  });
});

// Turbopack runs the loaders of one rule from right to left, the same as
// webpack. The last place therefore runs first.
describe('loaderOrder', () => {
  const userRule: TurbopackRuleConfigItem = { loaders: ['my-loader'] };

  test('puts the StyleX loader last by default, so that it runs first', () => {
    const rules = buildRules({ turbopack: { rules: { '*.mjs': userRule } } });

    expect(ruleAt(rules, '*.mjs').loaders).toEqual(['my-loader', stylexLoader()]);
  });

  test('puts the StyleX loader first when loaderOrder is last', () => {
    const rules = buildRules(
      { turbopack: { rules: { '*.mjs': userRule } } },
      {
        loaderOrder: 'last',
      }
    );

    expect(ruleAt(rules, '*.mjs').loaders).toEqual([stylexLoader(), 'my-loader']);
  });

  test('uses the same order as an explicit first', () => {
    const config = { turbopack: { rules: { '*.mjs': userRule } } };

    expect(buildRules(config, { loaderOrder: 'first' })).toEqual(buildRules(config));
  });
});

// Turbopack reads a list of rules as a set of alternatives, not as a chain: it
// keeps the first item whose condition agrees and it discards the rest. A merge
// that only adds an item to the list would drop a loader.
describe('turbopack rules that the user already wrote', () => {
  const userRule: TurbopackRuleConfigItem = { loaders: ['my-loader'] };

  test('leaves a glob that StyleX does not claim alone', () => {
    expect(buildRules({ turbopack: { rules: { '*.svg': userRule } } })).toHaveProperty(
      '*.svg',
      userRule
    );
  });

  test('adds the loader to the rule instead of beside it', () => {
    const rules = buildRules({ turbopack: { rules: { '*.js': userRule } } });

    expect(ruleAt(rules, '*.js').loaders).toEqual(['my-loader', stylexLoader()]);
  });

  test('keeps the condition, type and rename fields', () => {
    const detailed: TurbopackRuleConfigItem = {
      as: '*.js',
      condition: 'browser',
      loaders: ['my-loader'],
      type: 'ecmascript',
    };

    expect(ruleAt(buildRules({ turbopack: { rules: { '*.cjs': detailed } } }), '*.cjs')).toEqual({
      ...detailed,
      loaders: ['my-loader', stylexLoader()],
    });
  });

  test('adds loaders to a rule that sets only a module type', () => {
    const rules = buildRules({ turbopack: { rules: { '*.cjs': { type: 'ecmascript' } } } });

    expect(ruleAt(rules, '*.cjs')).toEqual({
      type: 'ecmascript',
      loaders: [stylexLoader()],
    });
  });

  test('expands a loader name written in the short form', () => {
    const rules = buildRules({ turbopack: { rules: { '*.ts': ['my-loader'] } } });

    expect(rules['*.ts']).toEqual([{ loaders: ['my-loader', stylexLoader()] }]);
  });

  test('expands a loader with options written in the short form', () => {
    const item: TurbopackLoaderItem = { loader: 'my-loader', options: { flag: true } };
    const rules = buildRules({ turbopack: { rules: { '*.ts': [item] } } });

    expect(rules['*.ts']).toEqual([{ loaders: [item, stylexLoader()] }]);
  });

  test('adds the loader to every item of a list', () => {
    const list = ['loader-a', { loader: 'loader-b' }, userRule];
    const rules = buildRules({ turbopack: { rules: { '*.ts': list } } });

    expect(rules['*.ts']).toEqual([
      { loaders: ['loader-a', stylexLoader()] },
      { loaders: [{ loader: 'loader-b' }, stylexLoader()] },
      { loaders: ['my-loader', stylexLoader()] },
    ]);
  });

  test('closes a list whose items all carry a condition', () => {
    const list: TurbopackRuleConfigItem[] = [
      { condition: 'browser', loaders: ['browser-loader'] },
      { condition: 'node', loaders: ['node-loader'] },
    ];
    const rules = buildRules({ turbopack: { rules: { '*.mts': list } } });

    expect(rules['*.mts']).toEqual([
      { condition: 'browser', loaders: ['browser-loader', stylexLoader()] },
      { condition: 'node', loaders: ['node-loader', stylexLoader()] },
      { loaders: [stylexLoader()] },
    ]);
  });

  test('does not close a list that already has an item without a condition', () => {
    const list: TurbopackRuleConfigItem[] = [
      { condition: 'browser', loaders: ['browser-loader'] },
      { loaders: ['every-loader'] },
    ];

    expect(buildRules({ turbopack: { rules: { '*.mts': list } } })['*.mts']).toHaveLength(2);
  });

  test('turns an empty list into the StyleX rule', () => {
    expect(buildRules({ turbopack: { rules: { '*.cts': [] } } })['*.cts']).toEqual([
      { loaders: [stylexLoader()] },
    ]);
  });

  test('does not change the object the user passed in', () => {
    const rules = { '*.js': userRule };
    const turbopack = { rules };

    buildRules({ turbopack });

    expect(rules).toEqual({ '*.js': { loaders: ['my-loader'] } });
    expect(turbopack).toEqual({ rules: { '*.js': { loaders: ['my-loader'] } } });
  });
});

// A config can pass through the plugin more than once: plugin composition, or
// a config file that wraps an already wrapped config.
describe('a config that the plugin already changed', () => {
  test('does not add a second loader to a plain rule', () => {
    const once = withStyleX()({});

    expect(withStyleX()(once).turbopack?.rules).toEqual(once.turbopack?.rules);
  });

  test('does not add a second loader beside a rule the user wrote', () => {
    const config = { turbopack: { rules: { '*.js': { loaders: ['my-loader'] } } } };
    const once = withStyleX()(config);

    expect(withStyleX()(once).turbopack?.rules).toEqual(once.turbopack?.rules);
  });

  test('does not add a second closing item to a list of conditions', () => {
    const config = {
      turbopack: { rules: { '*.mts': [{ condition: 'browser' as const, loaders: ['a'] }] } },
    };
    const once = withStyleX()(config);

    expect(withStyleX()(once).turbopack?.rules).toEqual(once.turbopack?.rules);
  });

  test('sees the loader written as a plain name', () => {
    const rules = buildRules({ turbopack: { rules: { '*.js': [LOADER_PATH] } } });

    expect(rules['*.js']).toEqual([{ loaders: [LOADER_PATH] }]);
  });

  test('stays stable over many passes', () => {
    const once = withStyleX()({ turbopack: { rules: { '*.js': { loaders: ['my-loader'] } } } });
    let config = once;

    for (let pass = 0; pass < 20; pass += 1) {
      config = withStyleX()(config);
    }

    expect(config.turbopack?.rules).toEqual(once.turbopack?.rules);
  });
});

describe('unusual input', () => {
  test('handles a rule whose loaders list is empty', () => {
    const rules = buildRules({ turbopack: { rules: { '*.js': { loaders: [] } } } });

    expect(ruleAt(rules, '*.js').loaders).toEqual([stylexLoader()]);
  });

  test('keeps a deep condition tree whole', () => {
    const condition = {
      all: [{ any: ['browser' as const, 'edge-light' as const] }, { not: 'foreign' as const }],
    };
    const rules = buildRules({ turbopack: { rules: { '*.js': { condition, loaders: ['a'] } } } });

    expect(ruleAt(rules, '*.js').condition).toEqual(condition);
  });

  test('keeps a glob named like an object member', () => {
    const rules = buildRules({
      turbopack: { rules: { constructor: { loaders: ['a'] }, toString: { loaders: ['b'] } } },
    });

    expect(rules).toHaveProperty('constructor', { loaders: ['a'] });
    expect(rules).toHaveProperty('toString', { loaders: ['b'] });
    expect(ruleAt(rules, '*.js').loaders).toEqual([stylexLoader()]);
  });

  test('does not let a rule named __proto__ reach the prototype', () => {
    const rules = buildRules({
      turbopack: { rules: JSON.parse('{"__proto__": {"loaders": ["evil"]}}') },
    });

    expect(({} as { loaders?: unknown }).loaders).toBeUndefined();
    expect(ruleAt(rules, '*.js').loaders).toEqual([stylexLoader()]);
  });

  test('handles a very long list of rules for one glob', () => {
    const list = Array.from({ length: 5_000 }, (_unused, index) => ({
      condition: 'browser' as const,
      loaders: [`loader-${index}`],
    }));
    const rules = buildRules({ turbopack: { rules: { '*.js': list } } });
    const merged = rules['*.js'] as TurbopackRuleConfigItem[];

    // Every item keeps its own loader, and the list gains one closing item.
    expect(merged).toHaveLength(5_001);
    expect(merged[0]?.loaders).toEqual(['loader-0', stylexLoader()]);
    expect(merged[4_999]?.loaders).toEqual(['loader-4999', stylexLoader()]);
    expect(merged[5_000]).toEqual({ loaders: [stylexLoader()] });
  });

  test('handles many globs that StyleX does not claim', () => {
    const userRules = Object.fromEntries(
      Array.from({ length: 2_000 }, (_unused, index) => [
        `*.ext${index}`,
        { loaders: [`loader-${index}`] },
      ])
    );
    const rules = buildRules({ turbopack: { rules: userRules } });

    expect(Object.keys(rules)).toHaveLength(2_000 + INCLUDE_EXTENSIONS.length);
    expect(rules['*.ext1999']).toEqual({ loaders: ['loader-1999'] });
  });

  test('carries large loader options through without loss', () => {
    const aliases = Object.fromEntries(
      Array.from({ length: 1_000 }, (_unused, index) => [`@a${index}/*`, [`/src/a${index}/*`]])
    );
    const rules = buildRules({}, { rsOptions: { aliases } });
    const loader = ruleAt(rules, '*.js').loaders?.[0] as unknown as {
      options: { rsOptions: unknown };
    };

    expect(loader.options.rsOptions).toEqual({ aliases });
  });
});
