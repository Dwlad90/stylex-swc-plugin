/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { Length } from './length';
import { TokenParser } from '../core2';
import { Angle } from './angle';
import type { LengthPercentage } from './length-percentage';
export declare class TransformFunction {
  static get parser(): TokenParser<TransformFunction>;
}
export declare class Matrix extends TransformFunction {
  readonly a: number;
  readonly b: number;
  readonly c: number;
  readonly d: number;
  readonly tx: number;
  readonly ty: number;
  constructor(
    a: number,
    b: number,
    c: number,
    d: number,
    tx: number,
    ty: number,
  );
  toString(): string;
  static get parser(): TokenParser<Matrix>;
}
export declare class Matrix3d extends TransformFunction {
  readonly args: Readonly<
    [
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
    ]
  >;
  constructor(args: this['args']);
  toString(): string;
  static get parser(): TokenParser<Matrix3d>;
}
export declare class Perspective extends TransformFunction {
  readonly length: Length;
  constructor(length: Length);
  toString(): string;
  static get parser(): TokenParser<Perspective>;
}
export declare class Rotate extends TransformFunction {
  readonly angle: Angle;
  constructor(angle: Angle);
  toString(): string;
  static get parser(): TokenParser<Rotate>;
}
export declare class RotateXYZ extends TransformFunction {
  readonly x: Angle;
  readonly axis: 'X' | 'Y' | 'Z';
  constructor(x: this['x'], axis: this['axis']);
  toString(): string;
  static get parser(): TokenParser<RotateXYZ>;
}
export declare class Rotate3d extends TransformFunction {
  readonly x: number;
  readonly y: number;
  readonly z: number;
  readonly angle: Angle;
  constructor(x: number, y: number, z: number, angle: Angle);
  toString(): string;
  static get parser(): TokenParser<Rotate3d>;
}
export declare class Scale extends TransformFunction {
  readonly sx: number;
  readonly sy: void | number;
  constructor(sx: this['sx'], sy?: null | undefined | this['sy']);
  toString(): string;
  static get parser(): TokenParser<Scale>;
}
export declare class Scale3d extends TransformFunction {
  readonly sx: number;
  readonly sy: number;
  readonly sz: number;
  constructor(sx: this['sx'], sy: this['sy'], sz: this['sz']);
  toString(): string;
  static get parser(): TokenParser<Scale3d>;
}
export declare class ScaleAxis extends TransformFunction {
  readonly s: number;
  readonly axis: 'X' | 'Y' | 'Z';
  constructor(s: this['s'], axis: this['axis']);
  toString(): string;
  static get parser(): TokenParser<ScaleAxis>;
}
export declare class Skew extends TransformFunction {
  readonly ax: Angle;
  readonly ay: void | Angle;
  constructor(ax: this['ax'], ay?: null | undefined | this['ay']);
  toString(): string;
  static get parser(): TokenParser<Skew>;
}
export declare class SkewAxis extends TransformFunction {
  readonly a: Angle;
  readonly axis: 'X' | 'Y';
  constructor(a: this['a'], axis: this['axis']);
  toString(): string;
  static get parser(): TokenParser<SkewAxis>;
}
export declare class Translate extends TransformFunction {
  readonly tx: LengthPercentage;
  readonly ty: void | LengthPercentage;
  constructor(tx: this['tx'], ty?: null | undefined | this['ty']);
  toString(): string;
  static get parser(): TokenParser<Translate>;
}
export declare class Translate3d extends TransformFunction {
  readonly tx: LengthPercentage;
  readonly ty: LengthPercentage;
  readonly tz: Length;
  constructor(tx: LengthPercentage, ty: LengthPercentage, tz: Length);
  toString(): string;
  static get parser(): TokenParser<Translate3d>;
}
export declare class TranslateAxis extends TransformFunction {
  readonly t: LengthPercentage;
  readonly axis: 'X' | 'Y' | 'Z';
  constructor(t: LengthPercentage, axis: 'X' | 'Y' | 'Z');
  toString(): string;
  static get parser(): TokenParser<TranslateAxis>;
}
