import postcss from 'postcss';
import createBuilder from './builder';
import {
  resolveExclude,
  resolveImportSourcesWithMetadata,
  resolveIncludeWithMetadata,
} from './discovery';

import type { StyleXPluginOption } from './types';

function isDebugEnabled(): boolean {
  const value = String(process.env.STYLEX_POSTCSS_DEBUG ?? '').toLowerCase();
  return value === '1' || value === 'true' || value === 'yes';
}

function createPlugin() {
  const PLUGIN_NAME = '@stylexswc/postcss-plugin';

  const builder = createBuilder();

  const isDev = process.env.NODE_ENV === 'development';

  const plugin = ({
    // By default reuses the Babel configuration from the project root.
    cwd = process.cwd(),
    rsOptions = {},
    include,
    exclude,
    useCSSLayers = false,
    importSources,
  }: StyleXPluginOption) => {
    const importSourcesResolution = resolveImportSourcesWithMetadata({
      importSources,
      rsOptions,
    });
    const effectiveImportSources = importSourcesResolution.importSources;

    const includeResolution = resolveIncludeWithMetadata({
      cwd,
      include,
      importSources: effectiveImportSources,
    });
    const effectiveInclude = includeResolution.include;

    const effectiveExclude = resolveExclude({
      include,
      exclude,
    });

    const excludeWithDefaults = [
      // Exclude type declaration files by default because it never contains any CSS rules.
      '**/*.d.ts',
      '**/*.flow',
      ...effectiveExclude,
    ];

    if (isDebugEnabled()) {
      console.info(
        `[${PLUGIN_NAME}] Auto-discovery details:\n${JSON.stringify(
          {
            cwd,
            importSourcesSource: importSourcesResolution.source,
            importSources: effectiveImportSources,
            include: effectiveInclude,
            includeWasExplicit: includeResolution.hasExplicitInclude,
            discoveredDependencyDirectories:
              includeResolution.discoveredDependencyDirectories,
            exclude: excludeWithDefaults,
          },
          null,
          2,
        )}`,
      );
    }

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
            include: effectiveInclude,
            exclude: excludeWithDefaults,
            cwd,
            rsOptions: {
              ...rsOptions,
              importSources:
                effectiveImportSources.length > 0
                  ? (effectiveImportSources as typeof rsOptions.importSources)
                  : rsOptions?.importSources ?? ['@stylexjs/stylex', 'stylex'],
              styleResolution: rsOptions?.styleResolution ?? 'property-specificity',
            },
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
              type: 'dependency',
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

  return plugin;
}

export default createPlugin;
