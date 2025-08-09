/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../core2';
type Unit = 'dpi' | 'dpcm' | 'dppx';
export declare class Resolution {
  readonly value: number;
  readonly unit: Unit;
  constructor(value: number, unit: Unit);
  toString(): string;
  static UNITS: ReadonlyArray<Unit>;
  static get parser(): TokenParser<Resolution>;
}
