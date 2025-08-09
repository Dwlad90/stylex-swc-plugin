/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import type { LengthPercentage } from '../css-types/length-percentage';
import { TokenParser } from '../token-parser';
export declare class BorderRadiusIndividual {
  horizontal: LengthPercentage;
  vertical: LengthPercentage;
  constructor(horizontal: LengthPercentage, vertical?: LengthPercentage);
  toString(): string;
  static get parse(): TokenParser<BorderRadiusIndividual>;
}
export declare class BorderRadiusShorthand {
  horizontalTopLeft: LengthPercentage;
  horizontalTopRight: LengthPercentage;
  horizontalBottomRight: LengthPercentage;
  horizontalBottomLeft: LengthPercentage;
  verticalTopLeft: LengthPercentage;
  verticalTopRight: LengthPercentage;
  verticalBottomRight: LengthPercentage;
  verticalBottomLeft: LengthPercentage;
  constructor(
    horizontalTopLeft: LengthPercentage,
    horizontalTopRight: LengthPercentage,
    horizontalBottomRight: LengthPercentage,
    horizontalBottomLeft: LengthPercentage,
    verticalTopLeft: LengthPercentage,
    verticalTopRight: LengthPercentage,
    verticalBottomRight: LengthPercentage,
    verticalBottomLeft: LengthPercentage,
  );
  toString(): string;
  static get parse(): TokenParser<BorderRadiusShorthand>;
}
