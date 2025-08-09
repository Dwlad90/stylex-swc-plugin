/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../core2';
export declare const UNITS_BASED_ON_FONT: any;
export declare const UNITS_BASED_ON_VIEWPORT: any;
export declare const UNITS_BASED_ON_CONTAINER: any;
export declare const UNITS_BASED_ON_ABSOLUTE_UNITS: any;
export declare class Length {
  readonly value: number;
  readonly unit: string;
  constructor(value: number, unit: string);
  toString(): string;
  static UNITS: ReadonlyArray<string>;
  static get parser(): TokenParser<Length>;
}
