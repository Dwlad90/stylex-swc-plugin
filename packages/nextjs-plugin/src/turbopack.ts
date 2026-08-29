import { INCLUDE_EXTENSIONS } from '@stylexswc/plugin-shared/constants';
import type { StyleXPluginOption } from '@stylexswc/webpack-plugin';
import type {
  NextConfig,
  TurbopackLoaderItem,
  TurbopackLoaderOptions,
  TurbopackRuleConfigCollection,
  TurbopackRuleConfigItem,
} from 'next/dist/server/config-shared';

import { exportAsCommonJs } from './cjs-interop';

const LOADER_PATH = '@stylexswc/turbopack-plugin/loader';

/** Tells whether a loader entry is the StyleX loader, in either of its forms. */
const isStyleXLoader = (loader: TurbopackLoaderItem): boolean =>
  typeof loader === 'string' ? loader === LOADER_PATH : loader.loader === LOADER_PATH;

/**
 * A list item can be a loader name, or a loader with options, or a full rule.
 * The first two forms are short forms of the third. Expand them, so that the
 * merge below has one shape to work with.
 */
const toRuleItem = (
  item: TurbopackLoaderItem | TurbopackRuleConfigItem
): TurbopackRuleConfigItem =>
  typeof item === 'string' || 'loader' in item ? { loaders: [item] } : item;

const withStyleX =
  (
    pluginOptions?: Omit<
      StyleXPluginOption,
      'useCSSLayers' | 'nextjsMode' | 'transformCss' | 'extractCSS'
    >
  ) =>
  (nextConfig: NextConfig = {}): NextConfig => {
    // `loaderOrder` puts the loader in its place, so the loader itself does not
    // need it. Next.js also refuses loader options that JSON cannot carry.
    const { loaderOrder = 'first', ...loaderOptions } = pluginOptions ?? {};

    // Build a new loader entry for each rule. One shared entry would let a
    // change to one rule reach all of the others.
    const stylexLoader = (): TurbopackLoaderItem => ({
      loader: LOADER_PATH,
      // The plugin options match what the loader reads. Next.js types loader
      // options as plain JSON, which the wider option type does not satisfy.
      options: { ...loaderOptions } as TurbopackLoaderOptions,
    });

    const stylexRule = (): TurbopackRuleConfigItem => ({ loaders: [stylexLoader()] });

    /**
     * Adds the StyleX loader to one rule. Turbopack runs the loaders of a rule
     * from right to left, the same as webpack. The last place therefore runs
     * first, which is what `loaderOrder: 'first'` asks for.
     */
    const withStyleXLoader = (
      item: TurbopackLoaderItem | TurbopackRuleConfigItem
    ): TurbopackRuleConfigItem => {
      const rule = toRuleItem(item);
      const loaders = rule.loaders ?? [];

      // Do not add a second StyleX loader when the plugin runs twice over the
      // same config. A second pass would compile output that is already
      // compiled.
      if (loaders.some(isStyleXLoader)) {
        return rule;
      }

      return {
        ...rule,
        loaders:
          loaderOrder === 'last' ? [stylexLoader(), ...loaders] : [...loaders, stylexLoader()],
      };
    };

    const withStyleXRule = (
      userRule: TurbopackRuleConfigCollection | undefined
    ): TurbopackRuleConfigCollection => {
      if (userRule === undefined) {
        return stylexRule();
      }

      if (!Array.isArray(userRule)) {
        return withStyleXLoader(userRule);
      }

      // Turbopack reads a list as a set of alternatives, not as a chain. It
      // keeps the first item whose condition agrees and it discards the rest.
      // Add the loader to every item, so that StyleX runs whichever item
      // Turbopack keeps.
      const items = userRule.map(withStyleXLoader);

      // A condition can fail on all of the items. Close the list with a plain
      // StyleX item, so that the file still reaches the loader.
      return items.some(item => item.condition === undefined) ? items : [...items, stylexRule()];
    };

    // One rule for every extension the loader transforms. A shorter list would
    // let StyleX in the extensions it leaves out reach the browser uncompiled.
    const rules: Record<string, TurbopackRuleConfigCollection> = {
      ...nextConfig?.turbopack?.rules,
    };

    for (const extension of INCLUDE_EXTENSIONS) {
      const glob = `*.${extension}`;

      rules[glob] = withStyleXRule(rules[glob]);
    }

    return {
      ...nextConfig,
      turbopack: {
        ...nextConfig?.turbopack,
        rules,
      },
    };
  };

export default withStyleX;

exportAsCommonJs(typeof module === 'undefined' ? undefined : module, withStyleX);
