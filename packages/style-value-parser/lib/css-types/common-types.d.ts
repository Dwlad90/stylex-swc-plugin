/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../token-parser';
export type CSSWideKeyword = 'inherit' | 'initial' | 'unset' | 'revert';
export declare const cssWideKeywords: TokenParser<CSSWideKeyword>;
export declare const inherit: TokenParser<'inherit'>;
export declare const initial: TokenParser<'initial'>;
export declare const unset: TokenParser<'unset'>;
export declare const revert: TokenParser<'revert'>;
export declare const auto: TokenParser<string>;
export declare class CssVariable {
  readonly name: string;
  constructor(name: string);
  toString(): string;
  static parse: TokenParser<CssVariable>;
}
export declare class Percentage {
  readonly value: number;
  constructor(value: number);
  toString(): string;
  static get parser(): TokenParser<Percentage>;
}
export declare const numberOrPercentage: TokenParser<number | Percentage>;
