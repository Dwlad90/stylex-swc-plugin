import type { StyleXOptions, UseLayersType } from '@stylexswc/rs-compiler';
import type { LoaderContext } from 'webpack';
import type webpack from 'webpack';

type WebpackCacheGroupEntry = NonNullable<
  Exclude<
    Exclude<webpack.Configuration['optimization'], undefined>['splitChunks'],
    undefined | false
  >['cacheGroups']
>[string];

// Exclude the shorthand members explicitly instead of Extract-ing by shape:
// a weak-type match ({ name?: unknown }) silently changes meaning if webpack
// ever reshapes the union (e.g. the function form has an apparent `name`).
type WebpackCacheGroupObject = Exclude<
  WebpackCacheGroupEntry,
  false | string | RegExp | ((..._args: never) => unknown)
>;

export type CacheGroupOptions =
  (Omit<WebpackCacheGroupObject, 'name'> & { name?: string }) | string | RegExp | false;

type AsyncFnParams = Parameters<ReturnType<LoaderContext<unknown>['async']>>;

export type InputCode = AsyncFnParams['1'];
export type SourceMap = AsyncFnParams['2'];

export type CSSTransformer = (
  _css: string,
  _filePath: string | undefined
) => string | Buffer | Promise<string | Buffer>;

export interface StyleXPluginOption {
  /**
   * stylex options passed to the StyleX Rust compiler
   *
   * @see https://stylexjs.com/docs/api/configuration/babel-plugin/
   */
  rsOptions?: Partial<StyleXOptions>;
  /**
   * Specify where stylex will be imported from
   *
   * @default ['stylex', '@stylexjs/stylex']
   */
  stylexImports?: StyleXOptions['importSources'];
  /**
   * Whether to use CSS layers
   *
   * @default false
   */
  useCSSLayers?: UseLayersType;
  /**
   * Next.js Mode
   *
   * @default false
   */
  nextjsMode?: boolean;
  /**
   * Next.js App Router Mode
   *
   * Enables the cross-compiler rule registry so styles collected by the
   * server and edge-server compilers are merged into the client CSS asset.
   *
   * @default false
   */
  nextjsAppRouterMode?: boolean;

  /**
   * Enable other CSS transformation
   *
   * Since the StyleX plugin only injects CSS after all loaders, you can not
   * use postcss-loader. With this you can invoke `postcss()` here.
   */
  transformCss?: CSSTransformer;

  /**
   * Whether to extract CSS
   *
   * @default true
   */
  extractCSS?: boolean;

  /**
   * Loader execution order
   *
   * Determines when the StyleX transformation is applied relative to other loaders:
   * - 'first': StyleX processes source code before any other loaders (recommended)
   * - 'last': StyleX processes after all other loaders have run
   *
   * The webpack plugin injects its loader per-module; the rspack plugin
   * registers a static module rule, so ordering maps to `Rule.enforce`
   * ('first' -> 'pre', 'last' -> 'post').
   *
   * @default 'first'
   */
  loaderOrder?: 'first' | 'last';
  /**
   * Customizes the cache group configuration for extracted CSS chunks.
   *
   * When provided, this REPLACES the plugin's default cache group entirely
   * (standard `splitChunks` semantics apply — e.g. omitting `test` matches
   * every module, which funnels all extracted CSS into the stylex chunk).
   * Only a static string `name` is supported and it is defaulted when omitted,
   * so the plugin can locate the emitted chunk. String and RegExp shorthand
   * values are treated as `test`. `false` disables the plugin's cache group
   * entirely (extracted styles then have no CSS asset to land in and the
   * build warns — prefer `extractCSS: false` to opt out of extraction).
   * Include `type: 'css/mini-extract'`, `chunks` and `enforce` yourself when
   * you need them.
   *
   * @see https://webpack.js.org/plugins/split-chunks-plugin/#splitchunkscachegroups
   */
  cacheGroup?: CacheGroupOptions;

  /**
   * node_modules packages that must be processed by the stylex-loader.
   *
   * By default, node_modules is excluded even if a module imports StyleX, so
   * only source that ships already-untransformed StyleX (e.g. component
   * libraries) needs to opt in here. List path fragments (e.g. '@stylexjs/',
   * 'my-design-system') for packages that ship untransformed StyleX source.
   *
   * @default ['@stylexjs/']
   */
  stylexPackages?: string[];

  /**
   * Path to a custom carrier stylesheet that receives the extracted StyleX
   * CSS — the file you import once at your app entrypoint. Absolute, or
   * relative to `compiler.context`.
   *
   * Replaces the default carrier shipped with the plugin
   * (`<plugin package>/stylex.css`): useful when a file named `stylex.css`
   * elsewhere in your project would collide with the default filename
   * pattern, or when you want the carrier to live in your own source tree.
   *
   * @example './src/styles/stylex-carrier.css'
   */
  carrierCss?: string;
}

export type StyleXLoaderOptions = {
  stylexImports: StyleXOptions['importSources'];
  rsOptions: Partial<StyleXOptions>;
  nextjsMode: boolean;
  nextjsAppRouterMode: boolean;
  extractCSS?: boolean;
};

export type SWCPluginRule = {
  class_name: string;
  style: { ltr: string; rtl?: null | string };
  priority: number;
};
