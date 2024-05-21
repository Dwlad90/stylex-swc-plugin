/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */
import type * as webpack from "webpack";
import { type SupplementedLoaderContext } from "./constants";
export type WebpackLoaderOptions = {
    /**
     * Please never use this feature, it will be removed without further notice.
     */
    stylexPlugin?: {
        transformCode: (code: string, filePath: string, logger?: ReturnType<webpack.Compiler["getInfrastructureLogger"]>) => Promise<{
            code: string;
            map: string;
        }>;
    };
};
declare function stylexLoader(this: SupplementedLoaderContext<WebpackLoaderOptions>, inputCode: string): void;
export default stylexLoader;
//# sourceMappingURL=custom-webpack-loader.d.ts.map