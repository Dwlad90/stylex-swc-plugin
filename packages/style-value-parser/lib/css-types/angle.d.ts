/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../token-parser';
export declare class Angle {
  readonly value: number;
  readonly unit: string;
  constructor(value: number, unit: this['unit']);
  toString(): string;
  static get parser(): TokenParser<Angle>;
}
