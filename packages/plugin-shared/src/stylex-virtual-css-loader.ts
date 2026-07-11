import { getHashDigest } from 'loader-utils';

import type { InputCode, SourceMap } from './types';
import type { LoaderContext } from 'webpack';

export default function (
  this: LoaderContext<unknown>,
  inputCode: InputCode,
  inputSourceMap?: SourceMap
) {
  const callback = this.async();

  if (this._compiler?.options.mode === 'production') {
    // In production the real CSS replaces the chunk asset during
    // processAssets; the pass-through content is stable and cacheable.
    return callback(null, inputCode, inputSourceMap);
  }

  // Development only: the dummy content depends on the rules of the importing
  // module, which change without this module's resource changing — never
  // cache it, so HMR and Next.js navigation pick up style edits.
  this.cacheable(false);

  const data = new URLSearchParams(this.resourceQuery.slice(1));

  try {
    const stylex = data.get('stylex');

    if (stylex == null) {
      return callback(null, inputCode, inputSourceMap);
    }

    const hash = getHashDigest(Buffer.from(stylex), 'xxhash64', 'base62', 32);

    // A content-hashed dummy rule: any change to the module's StyleX rules
    // changes the hash, which invalidates HMR for the stylex chunk.
    const css = `/* StyleX HMR bundle: ${hash} */\n.stylex-hashed-${hash} {}\n`;

    callback(null, `${inputCode}\n${css}`);
  } catch (e) {
    callback(e as Error);
  }
}
