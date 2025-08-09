/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import type { CSSToken } from '@csstools/css-tokenizer';
export type TokenIterator = {
  nextToken: () => CSSToken;
  endOfFile: () => boolean;
};
export declare class TokenList {
  readonly tokenIterator: TokenIterator;
  readonly consumedTokens: Array<CSSToken>;
  currentIndex: number;
  isAtEnd: boolean;
  constructor(input: TokenIterator | string);
  consumeNextToken(): CSSToken | null;
  peek(): CSSToken | null;
  get first(): CSSToken | null;
  setCurrentIndex(newIndex: number): void;
  rewind(positions: number): void;
  get isEmpty(): boolean;
  getAllTokens(): ReadonlyArray<CSSToken>;
  slice(start: number, end: number): Array<CSSToken>;
}
