/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { Position } from './position';
import { TokenParser } from '../core2';
import { type LengthPercentage } from './length-percentage';
declare class BasicShape {
  toString(): string;
}
export declare class Inset extends BasicShape {
  readonly top: LengthPercentage;
  readonly right: LengthPercentage;
  readonly bottom: LengthPercentage;
  readonly left: LengthPercentage;
  readonly round: null | undefined | LengthPercentage;
  constructor(
    top: LengthPercentage,
    right: LengthPercentage,
    bottom: LengthPercentage,
    left: LengthPercentage,
    round: null | undefined | LengthPercentage,
  );
  toString(): string;
  static get parser(): TokenParser<Inset>;
}
export type TCircleRadius = LengthPercentage | 'closest-side' | 'farthest-side';
export declare class Circle extends BasicShape {
  readonly radius: TCircleRadius;
  readonly position: null | undefined | Position;
  constructor(radius: TCircleRadius, position: null | undefined | Position);
  toString(): string;
  static get parser(): TokenParser<Circle>;
}
export declare class Ellipse extends BasicShape {
  readonly radiusX: TCircleRadius;
  readonly radiusY: TCircleRadius;
  readonly position: null | undefined | Position;
  constructor(
    radiusX: TCircleRadius,
    radiusY: TCircleRadius,
    position: null | undefined | Position,
  );
  toString(): string;
  static get parser(): TokenParser<Ellipse>;
}
type FillRule = 'nonzero' | 'evenodd';
type Point = Readonly<[LengthPercentage, LengthPercentage]>;
export declare class Polygon extends BasicShape {
  readonly fillRule: null | undefined | FillRule;
  readonly points: ReadonlyArray<Point>;
  constructor(points: this['points'], fillRule: this['fillRule']);
  toString(): string;
  static get parser(): TokenParser<Polygon>;
}
export declare class Path extends BasicShape {
  readonly fillRule: null | undefined | FillRule;
  readonly path: string;
  constructor(path: this['path'], fillRule: this['fillRule']);
  toString(): string;
  static get parser(): TokenParser<Path>;
}
