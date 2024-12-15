import postcss from 'postcss';
import createBuilder from './builder';

import type { StyleXPluginOption } from './types';

const PLUGIN_NAME = '@stylexswc/postcss-plugin';

const builder = createBuilder();

const isDev = process.env.NODE_ENV === 'development';

const plugin = ({
  cwd = process.cwd(),
  // By default reuses the Babel configuration from the project root.
  // Use `babelrc: false` to disable this behavior.
  rsOptions = {},
  include,
  exclude,
  useCSSLayers = false,
}: StyleXPluginOption) => {
  exclude = [
    // Exclude type declaration files by default because it never contains any CSS rules.
    '**/*.d.ts',
    '**/*.flow',
    ...(exclude ?? []),
  ];

  // Whether to skip the error when transforming StyleX rules.
  // Useful in watch mode where Fast Refresh can recover from errors.
  // Initial transform will still throw errors in watch mode to surface issues early.
  let shouldSkipTransformError = false;

  return {
    postcssPlugin: PLUGIN_NAME,
    plugins: [
      // Processes the PostCSS root node to find and transform StyleX at-rules.
      function (root: postcss.Root, result: postcss.Result) {
        const fileName = result.opts.from;

        // Configure the builder with the provided options
        builder.configure({
          include,
          exclude,
          cwd,
          rsOptions,
          useCSSLayers,
          isDev,
        });

        // Find the "@stylex" at-rule
        const styleXAtRule = builder.findStyleXAtRule(root);

        if (styleXAtRule == null) {
          return;
        }

        // Get dependencies to be watched for changes
        const dependencies = builder.getDependencies();

        // Add each dependency to the PostCSS result messages.
        // This watches the entire "./src" directory for "./src/**/*.{ts,tsx}"
        // to handle new files and deletions reliably in watch mode.
        for (const dependency of dependencies) {
          result.messages.push({
            plugin: PLUGIN_NAME,
            parent: fileName,
            ...dependency,
          });
        }

        // Build and parse the CSS from collected StyleX rules
        const css = builder.build({
          shouldSkipTransformError,
        });
        const parsed = postcss.parse(css, {
          from: fileName,
        });

        // Replace the "@stylex" rule with the generated CSS
        styleXAtRule.replaceWith(parsed);

        result.root = root;

        if (!shouldSkipTransformError) {
          // Build was successful, subsequent builds are for watch mode
          shouldSkipTransformError = true;
        }
      },
    ],
  };
};

plugin.postcss = true;

module.exports = plugin;
export default plugin;
