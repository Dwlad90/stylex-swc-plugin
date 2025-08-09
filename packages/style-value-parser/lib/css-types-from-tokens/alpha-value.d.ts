/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../core2';
export declare class AlphaValue {
  readonly value: number;
  constructor(value: number);
  toString(): string;
  static parser: TokenParser<AlphaValue>;
}
