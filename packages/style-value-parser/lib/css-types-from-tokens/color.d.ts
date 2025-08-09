/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../core2';
import { Angle } from './angle';
import { Percentage } from './common-types';
export declare class Color {
  static get parser(): TokenParser<Color>;
}
export declare class NamedColor extends Color {
  readonly value: string;
  constructor(value: string);
  toString(): string;
  static parser: TokenParser<NamedColor>;
}
export declare class HashColor extends Color {
  readonly value: string;
  constructor(value: string);
  toString(): string;
  get r(): number;
  get g(): number;
  get b(): number;
  get a(): number;
  static get parser(): TokenParser<HashColor>;
}
export declare class Rgb extends Color {
  readonly r: number;
  readonly g: number;
  readonly b: number;
  constructor(r: number, g: number, b: number);
  toString(): string;
  static get parser(): TokenParser<Rgb>;
}
export declare class Rgba extends Color {
  readonly r: number;
  readonly g: number;
  readonly b: number;
  readonly a: number;
  constructor(r: number, g: number, b: number, a: number);
  toString(): string;
  static get parser(): TokenParser<Rgba>;
}
export declare class Hsl extends Color {
  readonly h: Angle;
  readonly s: Percentage;
  readonly l: Percentage;
  constructor(h: Angle, s: Percentage, l: Percentage);
  toString(): string;
  static get parser(): TokenParser<Hsl>;
}
export declare class Hsla extends Color {
  readonly h: Angle;
  readonly s: Percentage;
  readonly l: Percentage;
  readonly a: number;
  constructor(h: Angle, s: Percentage, l: Percentage, a: number);
  toString(): string;
  static get parser(): TokenParser<Hsla>;
}
export declare class Lch extends Color {
  readonly l: number;
  readonly c: number;
  readonly h: Angle | number;
  readonly alpha: null | undefined | number;
  constructor(l: this['l'], c: this['c'], h: this['h'], alpha?: this['alpha']);
  toString(): string;
  static get parser(): TokenParser<Lch>;
}
export declare class Oklch extends Color {
  readonly l: number;
  readonly c: number;
  readonly h: Angle;
  readonly alpha: null | undefined | number;
  constructor(
    l: number,
    c: number,
    h: Angle,
    alpha?: null | undefined | number,
  );
  toString(): string;
  static get parser(): TokenParser<Lch>;
}
export declare class Oklab extends Color {
  readonly l: number;
  readonly a: number;
  readonly b: number;
  readonly alpha: null | undefined | number;
  constructor(
    l: number,
    a: number,
    b: number,
    alpha?: null | undefined | number,
  );
  toString(): string;
  static get parser(): TokenParser<Oklab>;
}
