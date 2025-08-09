/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../token-parser';
import { type LengthPercentage } from './length-percentage';
export type HorizontalKeyword = 'left' | 'center' | 'right';
export type VerticalKeyword = 'top' | 'center' | 'bottom';
export type Horizontal =
  | LengthPercentage
  | HorizontalKeyword
  | [HorizontalKeyword, LengthPercentage];
export type Vertical =
  | LengthPercentage
  | VerticalKeyword
  | [VerticalKeyword, LengthPercentage];
export declare class Position {
  readonly horizontal: null | undefined | Horizontal;
  readonly vertical: null | undefined | Vertical;
  constructor(
    horizontal: null | undefined | Horizontal,
    vertical: null | undefined | Vertical,
  );
  toString(): string;
  static get parser(): TokenParser<Position>;
}
