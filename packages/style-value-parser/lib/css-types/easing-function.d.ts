/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../token-parser';
export declare class EasingFunction {
  static get parser(): TokenParser<EasingFunction>;
}
export declare class LinearEasingFunction extends EasingFunction {
  readonly points: ReadonlyArray<number>;
  constructor(points: ReadonlyArray<number>);
  toString(): string;
  static get parser(): TokenParser<LinearEasingFunction>;
}
export declare class CubicBezierEasingFunction extends EasingFunction {
  readonly points: [number, number, number, number];
  constructor(points: [number, number, number, number]);
  toString(): string;
  static get parser(): TokenParser<CubicBezierEasingFunction>;
}
type TCubicBezierKeyword = 'ease' | 'ease-in' | 'ease-out' | 'ease-in-out';
export declare class CubicBezierKeyword extends EasingFunction {
  readonly keyword: TCubicBezierKeyword;
  constructor(keyword: TCubicBezierKeyword);
  toString(): string;
  static get parser(): TokenParser<CubicBezierKeyword>;
}
export declare class StepsEasingFunction extends EasingFunction {
  readonly steps: number;
  readonly start: 'start' | 'end';
  constructor(steps: number, start: 'start' | 'end');
  toString(): string;
  static get parser(): TokenParser<StepsEasingFunction>;
}
export declare class StepsKeyword extends EasingFunction {
  readonly keyword: 'step-start' | 'step-end';
  constructor(keyword: 'step-start' | 'step-end');
  toString(): string;
  static get parser(): TokenParser<StepsKeyword>;
}
