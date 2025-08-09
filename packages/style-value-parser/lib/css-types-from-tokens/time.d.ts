/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../core2';
export declare class Time {
  readonly value: number;
  readonly unit: 's' | 'ms';
  constructor(value: number, unit: 's' | 'ms');
  toString(): string;
  static UNITS: ReadonlyArray<'s' | 'ms'>;
  static get parser(): TokenParser<Time>;
}
