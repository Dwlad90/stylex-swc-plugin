import { getHashDigest } from 'loader-utils';

import type webpack from 'webpack';
import { InputCode, SourceMap } from './types';

export default function (
  this: webpack.LoaderContext<unknown>,
  inputCode: InputCode,
  inputSourceMap?: SourceMap
) {
  const callback = this.async();
  const data = new URLSearchParams(this.resourceQuery.slice(1));

  try {
    const stylex = data.get('stylex');

    if (stylex == null) {
      callback(null, inputCode, inputSourceMap);
      return;
    }

    const isProd = this._compiler?.options.mode === 'production';

    // If we got stylex in the virtual css import, we need to disable the cache
    // to fix HMR and Next.js navigation
    this.cacheable(isProd);

    if (isProd) {
      // In production, we don't need to generate dummy css
      return callback(null, '');
    }

    // @ts-expect-error - since v3 getHashDigest supports xxhash64
    // https://github.com/webpack/loader-utils?tab=readme-ov-file#interpolatename
    const hash = getHashDigest(Buffer.from(stylex), 'xxhash64', 'base62', 32);

    const css = `
    /*
     * Temporary CSS placeholder - @toss/stylexswc-webpack-plugin
     * This will be replaced with actual CSS during asset injection
     *
     * StyleX Bundle ID: ${hash}
     * Generated Rules:
     * ${JSON.stringify(JSON.parse(stylex), null, 4)}
     *
     * Note: This content is for development reference only
     */
    `;

    callback(null, css);
  } catch (e) {
    callback(e as Error);
  }
}
