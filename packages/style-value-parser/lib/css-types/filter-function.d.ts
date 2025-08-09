/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../token-parser';
import { Length } from './length';
import { Angle } from './angle';
export declare class FilterFunction {
  toString(): string;
  static get parser(): TokenParser<FilterFunction>;
}
export declare class BlurFilterFunction extends FilterFunction {
  readonly radius: Length;
  constructor(radius: Length);
  toString(): string;
  static get parser(): TokenParser<BlurFilterFunction>;
}
export declare class BrightnessFilterFunction extends FilterFunction {
  readonly percentage: number;
  constructor(percentage: number);
  toString(): string;
  static get parser(): TokenParser<BrightnessFilterFunction>;
}
export declare class ContrastFilterFunction extends FilterFunction {
  readonly amount: number;
  constructor(amount: number);
  toString(): string;
  static get parser(): TokenParser<ContrastFilterFunction>;
}
export declare class GrayscaleFilterFunction extends FilterFunction {
  readonly amount: number;
  constructor(amount: number);
  toString(): string;
  static get parser(): TokenParser<GrayscaleFilterFunction>;
}
export declare class HueRotateFilterFunction extends FilterFunction {
  readonly angle: Angle;
  constructor(angle: Angle);
  toString(): string;
  static get parser(): TokenParser<HueRotateFilterFunction>;
}
export declare class InverFilterFunction extends FilterFunction {
  readonly amount: number;
  constructor(amount: number);
  toString(): string;
  static get parser(): TokenParser<InverFilterFunction>;
}
export declare class OpacityFilterFunction extends FilterFunction {
  readonly amount: number;
  constructor(amount: number);
  toString(): string;
  static get parser(): TokenParser<OpacityFilterFunction>;
}
export declare class SaturateFilterFunction extends FilterFunction {
  readonly amount: number;
  constructor(amount: number);
  toString(): string;
  static get parser(): TokenParser<SaturateFilterFunction>;
}
export declare class SepiaFilterFunction extends FilterFunction {
  readonly amount: number;
  constructor(amount: number);
  toString(): string;
  static get parser(): TokenParser<SepiaFilterFunction>;
}
