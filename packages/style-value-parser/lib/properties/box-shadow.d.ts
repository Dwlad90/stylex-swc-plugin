/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../token-parser';
import { Length } from '../css-types/length';
import { Color } from '../css-types/color';
export declare class BoxShadow {
  readonly offsetX: Length;
  readonly offsetY: Length;
  readonly blurRadius: Length;
  readonly spreadRadius: Length;
  readonly color: Color;
  readonly inset: boolean;
  constructor(
    offsetX: Length,
    offsetY: Length,
    blurRadius: Length,
    spreadRadius: Length,
    color: Color,
    inset: boolean,
  );
  static get parse(): TokenParser<BoxShadow>;
}
export declare class BoxShadowList {
  readonly shadows: ReadonlyArray<BoxShadow>;
  constructor(shadows: ReadonlyArray<BoxShadow>);
  static get parse(): TokenParser<BoxShadowList>;
}
