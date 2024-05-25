import type * as webpack from "webpack";

import { PLUGIN_NAME, type SupplementedLoaderContext } from "./constants";

export type WebpackLoaderOptions = {
  /**
   * Please never use this feature, it will be removed without further notice.
   */
  stylexPlugin?: {
    transformCode: (
      code: string,
      filePath: string,
      logger?: ReturnType<webpack.Compiler["getInfrastructureLogger"]>,
    ) => Promise<{ code: string; map: string }>;
  };
};

type WebpackLoaderParams = Parameters<
  webpack.LoaderDefinitionFunction<WebpackLoaderOptions>
>;

function stylexLoader(
  this: SupplementedLoaderContext<WebpackLoaderOptions>,
  inputCode: string,
) {
  const callback = this.async();

  const { stylexPlugin } = this.getOptions();
  const logger = this._compiler?.getInfrastructureLogger(PLUGIN_NAME);

  stylexPlugin?.transformCode(inputCode, this.resourcePath, logger).then(
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
