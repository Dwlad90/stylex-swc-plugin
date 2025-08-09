/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../core2';
export declare class CustomIdentifier {
  readonly value: string;
  constructor(value: string);
  toString(): string;
  static get parser(): TokenParser<CustomIdentifier>;
}
