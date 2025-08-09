/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

export declare class SubString {
  readonly string: string;
  startIndex: number;
  endIndex: number;
  constructor(str: string);
  startsWith(str: string): boolean;
  get first(): string;
  get(relativeIndex: number): string;
  toString(): string;
  get isEmpty(): boolean;
}
