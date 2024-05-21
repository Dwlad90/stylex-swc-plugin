/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */
import type { Configuration } from "webpack";
import type { NextConfig } from "next";
import type { ConfigurationContext } from "next/dist/build/webpack/config/utils";
import type { WebpackConfigContext } from "next/dist/server/config-shared";
declare function StylexNextJSPlugin({ rootDir, filename, ...pluginOptions }: any): (nextConfig?: NextConfig) => {
    transpilePackages: string[];
    webpack(config: Configuration & ConfigurationContext, options: WebpackConfigContext): Configuration & ConfigurationContext;
    exportPathMap?: ((defaultMap: import("next/dist/server/config-shared").ExportPathMap, ctx: {
        dev: boolean;
        dir: string;
        outDir: string | null;
        distDir: string;
        buildId: string;
    }) => import("next/dist/server/config-shared").ExportPathMap | Promise<import("next/dist/server/config-shared").ExportPathMap>) | undefined;
    i18n?: import("next/dist/server/config-shared").I18NConfig | null | undefined;
    eslint?: import("next/dist/server/config-shared").ESLintConfig | undefined;
    typescript?: import("next/dist/server/config-shared").TypeScriptConfig | undefined;
    headers?: (() => Promise<import("next/dist/lib/load-custom-routes").Header[]>) | undefined;
    rewrites?: (() => Promise<import("next/dist/lib/load-custom-routes").Rewrite[] | {
        beforeFiles: import("next/dist/lib/load-custom-routes").Rewrite[];
        afterFiles: import("next/dist/lib/load-custom-routes").Rewrite[];
        fallback: import("next/dist/lib/load-custom-routes").Rewrite[];
    }>) | undefined;
    redirects?: (() => Promise<import("next/dist/lib/load-custom-routes").Redirect[]>) | undefined;
    excludeDefaultMomentLocales?: boolean | undefined;
    trailingSlash?: boolean | undefined;
    env?: Record<string, string | undefined> | undefined;
    distDir?: string | undefined;
    cleanDistDir?: boolean | undefined;
    assetPrefix?: string | undefined;
    cacheHandler?: string | undefined;
    cacheMaxMemorySize?: number | undefined;
    useFileSystemPublicRoutes?: boolean | undefined;
    generateBuildId?: (() => string | Promise<string | null> | null) | undefined;
    generateEtags?: boolean | undefined;
    pageExtensions?: string[] | undefined;
    compress?: boolean | undefined;
    analyticsId?: string | undefined;
    poweredByHeader?: boolean | undefined;
    images?: Partial<import("next/dist/shared/lib/image-config").ImageConfigComplete> | undefined;
    devIndicators?: {
        buildActivity?: boolean | undefined;
        buildActivityPosition?: "bottom-right" | "bottom-left" | "top-right" | "top-left" | undefined;
    } | undefined;
    onDemandEntries?: {
        maxInactiveAge?: number | undefined;
        pagesBufferLength?: number | undefined;
    } | undefined;
    amp?: {
        canonicalBase?: string | undefined;
    } | undefined;
    deploymentId?: string | undefined;
    basePath?: string | undefined;
    sassOptions?: {
        [key: string]: any;
    } | undefined;
    productionBrowserSourceMaps?: boolean | undefined;
    optimizeFonts?: boolean | undefined;
    reactProductionProfiling?: boolean | undefined;
    reactStrictMode?: boolean | null | undefined;
    publicRuntimeConfig?: {
        [key: string]: any;
    } | undefined;
    serverRuntimeConfig?: {
        [key: string]: any;
    } | undefined;
    httpAgentOptions?: {
        keepAlive?: boolean | undefined;
    } | undefined;
    outputFileTracing?: boolean | undefined;
    staticPageGenerationTimeout?: number | undefined;
    crossOrigin?: "anonymous" | "use-credentials" | undefined;
    swcMinify?: boolean | undefined;
    compiler?: {
        reactRemoveProperties?: boolean | {
            properties?: string[] | undefined;
        } | undefined;
        relay?: {
            src: string;
            artifactDirectory?: string | undefined;
            language?: "typescript" | "javascript" | "flow" | undefined;
            eagerEsModules?: boolean | undefined;
        } | undefined;
        removeConsole?: boolean | {
            exclude?: string[] | undefined;
        } | undefined;
        styledComponents?: boolean | import("next/dist/server/config-shared").StyledComponentsConfig | undefined;
        emotion?: boolean | import("next/dist/server/config-shared").EmotionConfig | undefined;
        styledJsx?: boolean | {
            useLightningcss?: boolean | undefined;
        } | undefined;
    } | undefined;
    output?: "standalone" | "export" | undefined;
    skipMiddlewareUrlNormalize?: boolean | undefined;
    skipTrailingSlashRedirect?: boolean | undefined;
    modularizeImports?: Record<string, {
        transform: string | Record<string, string>;
        preventFullImport?: boolean | undefined;
        skipDefaultConversion?: boolean | undefined;
    }> | undefined;
    logging?: {
        fetches?: {
            fullUrl?: boolean | undefined;
        } | undefined;
    } | undefined;
    experimental?: import("next/dist/server/config-shared").ExperimentalConfig | undefined;
};
export default StylexNextJSPlugin;
//# sourceMappingURL=index.d.ts.map