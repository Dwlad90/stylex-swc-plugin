/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../token-parser';
import { TransformFunction } from '../css-types/transform-function';
export declare class Transform {
  readonly value: ReadonlyArray<TransformFunction>;
  constructor(value: ReadonlyArray<TransformFunction>);
  toString(): string;
  static get parse(): TokenParser<Transform>;
}
