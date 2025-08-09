/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../core2';
export declare class Flex {
  readonly fraction: number;
  constructor(fraction: number);
  toString(): string;
  static get parser(): TokenParser<Flex>;
}
