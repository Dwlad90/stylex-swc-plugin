/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import type * as webpack from "webpack";

import { PLUGIN_NAME, type SupplementedLoaderContext } from "./constants";

export type WebpackLoaderOptions = {
  /**
   * Please never use this feature, it will be removed without further notice.
   */
  // unstable_keepOriginalCode?: boolean;
};

type WebpackLoaderParams = Parameters<
  webpack.LoaderDefinitionFunction<WebpackLoaderOptions>
>;

function stylexLoader(
  this: SupplementedLoaderContext<WebpackLoaderOptions>,
  inputCode: string,
) {
  const callback = this.async();

  // @ts-expect-error - tmp
  const { stylexPlugin } = this.getOptions();
  const logger = this._compiler?.getInfrastructureLogger(PLUGIN_NAME);

  stylexPlugin.transformCode(inputCode, this.resourcePath, logger).then(
    ({ code, map }: { code: string; map: string }) => {
      callback(null, code, map);
    },
    (error: Error) => {
      callback(error);
    },
  );
}

export default stylexLoader;
module.exports = stylexLoader;
