"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.PropertyValidationMode = exports.SourceMaps = void 0;
exports.normalizeRsOptions = normalizeRsOptions;
exports.shouldTransformFile = shouldTransformFile;
exports.transform = transform;
const path = __importStar(require("path"));
const picomatch_1 = __importDefault(require("picomatch"));
const transform_1 = __importDefault(require("../dist/transform"));
// const enums are erased by TypeScript — provide runtime values
// so ESM consumers can import them. Typed as the native enum rather than as
// string literals: without that, `{ sourceMap: SourceMaps.False }` fails to
// typecheck, which defeats the point of exporting the values at all.
exports.SourceMaps = Object.freeze({
    True: 'True',
    False: 'False',
    Inline: 'Inline',
});
exports.PropertyValidationMode = Object.freeze({
    Throw: 'throw',
    Warn: 'warn',
    Silent: 'silent',
});
/**
 * Default values for StyleX options.
 * Every field that has a sensible default is listed here.
 *
 * `maxEvaluationDepth` is deliberately absent. Its default lives in the
 * compiler, which falls back to the `STYLEX_MAX_EVALUATION_DEPTH` environment
 * variable before reaching for it -- so a value listed here would be sent on
 * every call and make that variable unreachable.
 */
const defaultOptions = {
    dev: false,
    test: false,
    debug: false,
    enableFontSizePxToRem: false,
    runtimeInjection: false,
    treeshakeCompensation: false,
    enableInlinedConditionalMerge: true,
    enableLogicalStylesPolyfill: false,
    enableMinifiedKeys: true,
    enableLegacyValueFlipping: false,
    enableLTRRTLComments: false,
    legacyDisableLayers: false,
    useRealFileForSource: true,
    inlineSourcesContent: true,
    emitSourceMapColumns: true,
    enableMediaQueryOrder: true,
    enableDebugClassNames: false,
    propertyValidationMode: 'silent',
    styleResolution: 'property-specificity',
    importSources: ['stylex', '@stylexjs/stylex'],
};
// ── normalizeRsOptions ──────────────────────────────────────────────
/** Strip keys whose value is `undefined` so they don't clobber defaults. */
function definedEntries(obj) {
    return Object.fromEntries(Object.entries(obj).filter(([, v]) => v !== undefined));
}
/**
 * Normalize StyleX compiler options by applying defaults and merging
 * user-provided values. Uses a spread/defaults pattern: defaults are
 * applied first, then user-provided values overlay them
 * (undefined keys skipped).
 */
function normalizeRsOptions(options) {
    if (options == null) {
        throw new TypeError('Options must be an object, received null/undefined');
    }
    // Non-object input (string, number, etc.) — treat as empty options
    const inputOptions = typeof options === 'object' ? options : {};
    const definedOptions = definedEntries(inputOptions);
    // Spread defaults then user values (undefined keys already stripped)
    const result = {
        ...defaultOptions,
        ...definedOptions,
        include: definedOptions.include ?? [],
        exclude: definedOptions.exclude ?? [],
        swcPlugins: definedOptions.swcPlugins ?? [],
    };
    return result;
}
// ── shouldTransformFile ─────────────────────────────────────────────
/**
 * Determine whether a file should be transformed based on include/exclude
 * patterns (glob strings or RegExp).
 */
function shouldTransformFile(filePath, include, exclude) {
    const relativePath = path.relative(process.cwd(), filePath).split(path.sep).join('/');
    if (include && include.length > 0) {
        if (!include.some(p => matchPattern(relativePath, p))) {
            return false;
        }
    }
    if (exclude && exclude.length > 0) {
        if (exclude.some(p => matchPattern(relativePath, p))) {
            return false;
        }
    }
    return true;
}
/** Match a file path against a single pattern (glob string or RegExp). */
function matchPattern(filePath, pattern) {
    if (pattern instanceof RegExp) {
        // Reset lastIndex to avoid nondeterministic results with /g or /y flags
        pattern.lastIndex = 0;
        return pattern.test(filePath);
    }
    if (typeof pattern !== 'string' || pattern === '') {
        return false;
    }
    return picomatch_1.default.isMatch(filePath, pattern, { dot: true });
}
// ── transform ───────────────────────────────────────────────────────
/**
 * Transform source code with StyleX. When `options.swcPlugins` is set,
 * SWC plugins are applied first, then the native StyleX transform runs.
 */
function transform(filename, code, options) {
    // Apply include/exclude filter before transforming
    if (!shouldTransformFile(filename, options.include, options.exclude)) {
        return {
            code,
            metadata: { stylex: [] },
            map: undefined,
        };
    }
    let transformedCode = code;
    let inputSourceMap = options.inputSourceMap;
    if (options.swcPlugins?.length) {
        // oxlint-disable-next-line typescript/no-require-imports
        const swc = require('@swc/core');
        // Always request an external map: it chains `inputSourceMap` and the
        // plugins' own edits, so the positions handed to the native transform
        // keep describing the code it actually receives. The native transform
        // owns emission of the final map per `options.sourceMap` (incl. inline).
        // The intermediate map becomes the native transform's `orig`, and a
        // chained map keeps whatever `sourcesContent` it arrives with — so it has
        // to carry the authored text from here.
        const result = swc.transformSync(transformedCode, {
            filename,
            sourceMaps: true,
            inlineSourcesContent: options.inlineSourcesContent ?? true,
            inputSourceMap,
            jsc: {
                parser: { syntax: 'typescript', tsx: true },
                target: 'es2022',
                experimental: { plugins: options.swcPlugins },
            },
        });
        transformedCode = result.code;
        if (result.map) {
            inputSourceMap = result.map;
        }
    }
    // Strip TS-only fields before passing to native transform
    const { swcPlugins: _swcPlugins, include: _include, exclude: _exclude, ...nativeOptions } = options;
    return transform_1.default.transform(filename, transformedCode, {
        ...nativeOptions,
        inputSourceMap,
    });
}
