/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../core2';
import type { TokenDimension } from '@csstools/css-tokenizer';
type Fraction = [number, '/', number];
type WordRule = 'color' | 'monochrome' | 'grid' | 'color-index';
type Length = TokenDimension[4];
type MediaRuleValue = Length | string | Fraction;
type MediaKeyword = {
  type: 'media-keyword';
  key: 'screen' | 'print' | 'all';
  not: boolean;
};
type MediaWordRule = { type: 'word-rule'; keyValue: WordRule };
type MediaRulePair = { type: 'pair'; key: string; value: MediaRuleValue };
type MediaNotRule = { type: 'not'; rule: MediaQueryRule };
type MediaAndRules = { type: 'and'; rules: ReadonlyArray<MediaQueryRule> };
type MediaOrRules = { type: 'or'; rules: ReadonlyArray<MediaQueryRule> };
type MediaQueryRule =
  | MediaKeyword
  | MediaWordRule
  | MediaRulePair
  | MediaNotRule
  | MediaAndRules
  | MediaOrRules;
export declare const mediaInequalityRuleParser: TokenParser<MediaRulePair>;
export declare class MediaQuery {
  queries: MediaQueryRule;
  constructor(queries: this['queries']);
  toString(): string;
  static normalize(rule: MediaQueryRule): MediaQueryRule;
  static get parser(): TokenParser<MediaQuery>;
}
