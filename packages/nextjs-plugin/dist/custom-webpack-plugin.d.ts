/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */
import type { Compiler } from "webpack";
declare class StylexPlugin {
    filesInLastRun: any;
    filePath: any;
    dev: any;
    appendTo: any;
    filename: any;
    babelConfig: any;
    stylexImports: any[];
    babelPlugin: any;
    useCSSLayers: any;
    constructor({ dev, appendTo, filename, stylexImports, useCSSLayers, }?: any);
    apply(compiler: Compiler): void;
    transformCode(inputCode: string, filename: string, logger: any): Promise<{
        code: string;
        map: null;
    } | {
        code: string;
        map?: undefined;
    }>;
}
export default StylexPlugin;
//# sourceMappingURL=custom-webpack-plugin.d.ts.map