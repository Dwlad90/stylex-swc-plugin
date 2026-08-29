export type { ImportSourceInput, StyleXMetadata, StyleXModuleResolution, StyleXTransformResult, } from '../dist/transform';
import type { SourceMaps as NativeSourceMaps, StyleXOptions as NativeStyleXOptions, StyleXTransformResult } from '../dist/transform';
export declare const SourceMaps: Readonly<{
    True: NativeSourceMaps;
    False: NativeSourceMaps;
    Inline: NativeSourceMaps;
}>;
export declare const PropertyValidationMode: Readonly<{
    readonly Throw: 'throw';
    readonly Warn: 'warn';
    readonly Silent: 'silent';
}>;
/** StyleX compiler options (native options + TS-only fields). */
export interface StyleXOptions extends NativeStyleXOptions {
    include?: Array<string | RegExp>;
    exclude?: Array<string | RegExp>;
    inputSourceMap?: string;
    swcPlugins?: Array<[string, Record<string, unknown>]>;
}
export type UseLayersType = boolean | {
    before?: ReadonlyArray<string>;
    after?: ReadonlyArray<string>;
    prefix?: string;
};
export type TransformedOptions = Partial<Pick<StyleXOptions, 'legacyDisableLayers' | 'enableLTRRTLComments'> & {
    useLayers: UseLayersType;
}>;
/**
 * Normalize StyleX compiler options by applying defaults and merging
 * user-provided values. Uses a spread/defaults pattern: defaults are
 * applied first, then user-provided values overlay them
 * (undefined keys skipped).
 */
export declare function normalizeRsOptions(options?: StyleXOptions | null): StyleXOptions;
/**
 * Determine whether a file should be transformed based on include/exclude
 * patterns (glob strings or RegExp).
 */
export declare function shouldTransformFile(filePath: string, include?: Array<string | RegExp> | null, exclude?: Array<string | RegExp> | null): boolean;
/**
 * Transform source code with StyleX. When `options.swcPlugins` is set,
 * SWC plugins are applied first, then the native StyleX transform runs.
 */
export declare function transform(filename: string, code: string, options: StyleXOptions): StyleXTransformResult;
//# sourceMappingURL=index.d.ts.map