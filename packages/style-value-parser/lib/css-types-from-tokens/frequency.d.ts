/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../core2';
export declare class Frequency {
  readonly value: number;
  readonly unit: 'Hz' | 'KHz';
  constructor(value: number, unit: 'Hz' | 'KHz');
  toString(): string;
  static UNITS: ReadonlyArray<'Hz' | 'KHz'>;
  static get parser(): TokenParser<Frequency>;
}
