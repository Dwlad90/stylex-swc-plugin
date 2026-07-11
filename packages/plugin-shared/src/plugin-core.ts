import path from 'path';
import stylexBabelPlugin from '@stylexjs/babel-plugin';

import {
  BUILD_INFO_STYLEX_KEY,
  DEFAULT_STYLEX_PACKAGES,
  NEXTJS_COMPILER_NAMES,
  VIRTUAL_ENTRYPOINT_CSS_PATTERN,
} from './constants';
import { mergeStyleXRulesInto, publishStyleXRules } from './nextjs-registry';
import {
  buildVirtualCssPattern,
  escapeRegExp,
  parseStylexRulesFromIdentifier,
  shouldProcessFile,
} from './utils';

import type { StyleXRulesMap } from './nextjs-registry';
import type { StyleXOptions, TransformedOptions } from '@stylexswc/rs-compiler';
import type { Rule as StyleXRule } from '@stylexjs/babel-plugin';
import type {
  CSSTransformer,
  CacheGroupOptions,
  StyleXLoaderOptions,
  StyleXPluginOption,
} from './types';

export type RegisterStyleXRules = (_resourcePath: string, _stylexRules: StyleXRule[]) => void;

const identityTransform: CSSTransformer = css => css;

let packageVersion = 'unknown';
try {
  // Resolved at runtime relative to the compiled module (dist/ and src/ are
  // both one level below package.json); only used to cache-bust chunk hashes
  // eslint-disable-next-line @typescript-eslint/no-require-imports
  packageVersion = (require('../package.json') as { version?: string }).version ?? 'unknown';
} catch {
  // chunk hashes fall back to option-only metadata
}

type ModuleWithBuildInfo = {
  buildInfo?: Record<string, unknown> | undefined;
};

type ModuleWithIdentifier = {
  identifier(): string;
};

type ChunkLike = {
  files: Iterable<string>;
};

/**
 * Bundler-agnostic core of the StyleX webpack/rspack plugins: option
 * normalization, rule bookkeeping (both transports), the Next.js
 * cross-compiler registry, and final CSS generation. The wrapper packages
 * extend it and implement only `apply()` with bundler-specific hook wiring.
 */
export class StyleXPluginCore {
  stylexRules: StyleXRulesMap = new Map();
  transformedOptions: TransformedOptions;

  loaderOption: StyleXLoaderOptions;
  cacheGroup?: CacheGroupOptions;
  transformCss: CSSTransformer;
  loaderOrder: NonNullable<StyleXPluginOption['loaderOrder']>;
  stylexPackages: string[];
  include: StyleXOptions['include'];
  exclude: StyleXOptions['exclude'];
  /** raw `carrierCss` option; resolved against compiler.context in apply() */
  carrierCss?: string;
  /** absolute path of the configured or packaged carrier, set by resolveCarrier() */
  carrierPath?: string;

  constructor({
    stylexImports = ['stylex', '@stylexjs/stylex'],
    useCSSLayers = false,
    rsOptions = {},
    nextjsMode = false,
    nextjsAppRouterMode = false,
    transformCss = identityTransform,
    extractCSS = true,
    loaderOrder = 'first',
    cacheGroup,
    stylexPackages = DEFAULT_STYLEX_PACKAGES,
    carrierCss,
  }: StyleXPluginOption = {}) {
    this.transformedOptions = {
      useLayers: useCSSLayers,
      legacyDisableLayers: rsOptions.legacyDisableLayers,
      enableLTRRTLComments: rsOptions.enableLTRRTLComments,
    };
    // include/exclude filtering happens before the loader runs (module rule
    // condition or loader-injection check); the loader receives them stripped
    // so the transform doesn't re-filter
    this.include = rsOptions.include;
    this.exclude = rsOptions.exclude;
    this.loaderOption = {
      stylexImports,
      rsOptions: {
        enableFontSizePxToRem: true,
        runtimeInjection: false,
        treeshakeCompensation: true,
        importSources: stylexImports,
        injectStylexSideEffects: loaderOrder !== 'last',
        ...rsOptions,
        include: undefined,
        exclude: undefined,
      },
      nextjsMode,
      nextjsAppRouterMode,
      extractCSS,
    };
    this.transformCss = transformCss;
    this.loaderOrder = loaderOrder;
    this.cacheGroup = cacheGroup;
    this.stylexPackages = stylexPackages;
    this.carrierCss = carrierCss;
  }

  /**
   * Resolves the configured carrier against `compiler.context`, falling back
   * to the exact packaged carrier path.
   * Call from apply() before installing the cache group.
   */
  resolveCarrier(context: string | undefined, defaultCarrierPath: string): void {
    this.carrierPath = this.carrierCss
      ? path.resolve(context ?? process.cwd(), this.carrierCss)
      : defaultCarrierPath;
  }

  /** Matches everything belonging in the stylex chunk (carrier + dummies). */
  getVirtualCssPattern(): RegExp {
    return buildVirtualCssPattern(this.carrierPath);
  }

  /** Matches only the carrier stylesheet (for sideEffects forcing). */
  getCarrierPattern(): RegExp {
    if (this.carrierPath) {
      return new RegExp(`${escapeRegExp(this.carrierPath)}$`);
    }

    return VIRTUAL_ENTRYPOINT_CSS_PATTERN;
  }

  getChunkName(defaultChunkName: string): string {
    if (
      typeof this.cacheGroup === 'object' &&
      this.cacheGroup != null &&
      'name' in this.cacheGroup &&
      typeof this.cacheGroup.name === 'string'
    ) {
      return this.cacheGroup.name;
    }

    return defaultChunkName;
  }

  shouldProcessFile(resourcePath: string): boolean {
    return shouldProcessFile(resourcePath, {
      stylexPackages: this.stylexPackages,
      include: this.include,
      exclude: this.exclude,
    });
  }

  /**
   * Resolved lazily against the actual build mode instead of a module-load-time
   * NODE_ENV snapshot, so `dev` reflects this compilation, not whatever env var
   * was set when the plugin module was first `require`'d.
   */
  resolveDevOption(mode: string | undefined): void {
    this.loaderOption.rsOptions.dev ??= mode !== 'production';
  }

  /**
   * Installs the dedicated splitChunks cacheGroup that funnels the carrier and
   * dummy CSS modules into a single named chunk, throwing early when
   * `optimization.splitChunks` is disabled.
   */
  assertAndInstallCacheGroup(
    optimization: {
      splitChunks?: false | { cacheGroups?: Record<string, CacheGroupOptions> };
    },
    packageName: string,
    chunkName: string
  ): void {
    if (!optimization.splitChunks) {
      throw new Error(
        [
          'You don\'t have "optimization.splitChunks" enabled.',
          `"optimization.splitChunks" should be enabled for "${packageName}" to function properly.`,
        ].join(' ')
      );
    }

    optimization.splitChunks.cacheGroups ??= {};
    const defaultCacheGroup: CacheGroupOptions = {
      name: chunkName,
      test: this.getVirtualCssPattern(),
      type: 'css/mini-extract',
      chunks: 'all',
      enforce: true,
    };

    optimization.splitChunks.cacheGroups[chunkName] =
      typeof this.cacheGroup === 'object' && this.cacheGroup != null
        ? { ...defaultCacheGroup, ...this.cacheGroup }
        : defaultCacheGroup;
  }

  /**
   * Serialized plugin metadata mixed into chunk hashes so option changes
   * (which change the generated CSS) invalidate long-term caching.
   */
  buildChunkHashMeta(packageName: string): string {
    return JSON.stringify({
      name: packageName,
      packageVersion,
      loaderOption: this.loaderOption,
      transformedOptions: this.transformedOptions,
    });
  }

  /**
   * webpack rule transport: the stylex-loader stores extracted rules on
   * `module.buildInfo`, which webpack persists in its filesystem cache, so
   * cached rebuilds (where the loader doesn't re-run) still surface rules
   * here. Call from `compilation.hooks.finishModules`.
   */
  collectFromBuildInfo(modules: Iterable<ModuleWithBuildInfo>): void {
    const recollected: StyleXRulesMap = new Map();

    for (const mod of modules) {
      const buildInfo = mod.buildInfo;

      if (!buildInfo || !(BUILD_INFO_STYLEX_KEY in buildInfo)) {
        continue;
      }

      const stylexBuildInfo = buildInfo[BUILD_INFO_STYLEX_KEY];

      if (
        typeof stylexBuildInfo === 'object' &&
        stylexBuildInfo != null &&
        'resourcePath' in stylexBuildInfo &&
        'stylexRules' in stylexBuildInfo &&
        typeof stylexBuildInfo.resourcePath === 'string'
      ) {
        recollected.set(
          stylexBuildInfo.resourcePath,
          stylexBuildInfo.stylexRules as readonly StyleXRule[]
        );
      }
    }

    this.stylexRules = recollected;
  }

  /**
   * rspack rule transport: rebuilds the rules map from the dummy-import
   * queries embedded in module identifiers. Identifiers survive caching and
   * carry the rules across compilations, and rebuilding (instead of merging)
   * drops rules of files that were deleted or stopped importing stylex.
   */
  replaceFromChunkModuleIdentifiers(modules: Iterable<ModuleWithIdentifier>): void {
    const recollected: StyleXRulesMap = new Map();

    for (const mod of modules) {
      const identifier = mod.identifier();
      const rules = parseStylexRulesFromIdentifier(identifier);

      if (rules != null) {
        recollected.set(identifier, rules);
      }
    }

    this.stylexRules = recollected;
  }

  /** Publish this compiler's rules for the Next.js App Router client merge. */
  publishNextjsRegistry(registryKey: string | undefined, compilerName: string | undefined): void {
    if (this.loaderOption.nextjsMode && this.loaderOption.nextjsAppRouterMode) {
      publishStyleXRules(registryKey, compilerName, this.stylexRules);
    }
  }

  /** Merge rules published by the other Next.js compilers (client compiler only). */
  mergeNextjsRegistry(registryKey: string | undefined, compilerName: string | undefined): void {
    if (this.loaderOption.nextjsMode && this.loaderOption.nextjsAppRouterMode) {
      mergeStyleXRulesInto(registryKey, compilerName, this.stylexRules);
    }
  }

  getStyleXCSS(): string | null {
    if (this.stylexRules.size === 0) {
      return null;
    }

    // Take styles for the modules that were included in the last compilation.
    const allRules: StyleXRule[] = Array.from(this.stylexRules.values()).flat();

    return stylexBabelPlugin.processStylexRules(allRules, this.transformedOptions);
  }

  /**
   * Whether this compiler is the one responsible for emitting the stylex CSS
   * asset. In Next.js multi-compiler builds only the client compilation emits
   * it — the server and edge-server compilers merely collect rules and
   * publish them to the cross-compiler registry, so ending without a stylex
   * chunk is their normal state, not a failure.
   */
  private isEmittingCompiler(compilerName: string | undefined): boolean {
    if (!this.loaderOption.nextjsMode) {
      return true;
    }

    return compilerName === NEXTJS_COMPILER_NAMES.client;
  }

  /**
   * Actionable message for the silent-failure case: styles were extracted but
   * there is no CSS asset to receive them, so they would vanish from the
   * output without a trace.
   */
  private buildMissingCarrierWarning(chunkName: string, carrierHint?: string): string {
    return [
      `StyleX rules were extracted from ${this.stylexRules.size} module(s), but no "${chunkName}" CSS asset was emitted to receive them — the styles will be MISSING from the output.`,
      `Make sure the carrier stylesheet is imported once at your app entrypoint${carrierHint ? ` (${carrierHint})` : ''} and that a css rule (css-loader + extract plugin) covers it.`,
    ].join(' ');
  }

  /**
   * Generates the final stylesheet and REPLACES the carrier chunk's CSS asset
   * content with it. Callbacks keep this structural so both webpack's and
   * rspack's `Compilation` satisfy it without type gymnastics.
   */
  async finalizeStylexAsset({
    assets,
    getNamedChunk,
    updateAsset,
    createSource,
    chunkName,
    compilerName,
    carrierHint,
    warn,
  }: {
    assets: Record<string, unknown>;
    getNamedChunk: (_name: string) => ChunkLike | null | undefined;
    updateAsset: (_name: string, _source: unknown) => void;
    createSource: (_css: string | Buffer) => unknown;
    chunkName: string;
    /** `compiler.name`; gates the missing-carrier warning in Next.js builds */
    compilerName?: string;
    /** import specifier suggested in the missing-carrier warning */
    carrierHint?: string;
    warn?: (_message: string) => void;
  }): Promise<void> {
    const stylexChunk = getNamedChunk(chunkName);
    const expectsCssAsset =
      this.loaderOption.extractCSS !== false &&
      this.stylexRules.size > 0 &&
      this.isEmittingCompiler(compilerName);

    if (stylexChunk == null) {
      if (expectsCssAsset) {
        warn?.(this.buildMissingCarrierWarning(chunkName, carrierHint));
      }

      return;
    }

    // Let's find the css file that belongs to the stylex chunk
    const stylexChunkFiles = new Set(stylexChunk.files);
    const cssAssetNames = Object.keys(assets).filter(
      assetName => stylexChunkFiles.has(assetName) && assetName.endsWith('.css')
    );

    if (cssAssetNames.length === 0) {
      if (expectsCssAsset) {
        warn?.(this.buildMissingCarrierWarning(chunkName, carrierHint));
      }

      return;
    }
    if (cssAssetNames.length > 1) {
      warn?.(
        'Multiple CSS assets found for the stylex chunk. This should not happen. Please report this issue.'
      );
    }

    const cssAsset = cssAssetNames[0];
    const stylexCSS = this.getStyleXCSS();

    if (stylexCSS == null || cssAsset == null) {
      return;
    }

    const finalCss = await this.transformCss(stylexCSS, cssAsset);

    updateAsset(cssAsset, createSource(finalCss));
  }
}
