/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { type CalcConstant } from './calc-constant';
import { Percentage } from './common-types';
import { TokenParser } from '../token-parser';
import type { TokenDimension } from '@csstools/css-tokenizer';
type Addition = { type: '+'; left: CalcValue; right: CalcValue };
type Subtraction = { type: '-'; left: CalcValue; right: CalcValue };
type Multiplication = { type: '*'; left: CalcValue; right: CalcValue };
type Division = { type: '/'; left: CalcValue; right: CalcValue };
type Group = { type: 'group'; expr: CalcValue };
type CalcValue =
  | number
  | TokenDimension[4]
  | Percentage
  | CalcConstant
  | Addition
  | Subtraction
  | Multiplication
  | Division
  | Group;
export declare const valueParser: TokenParser<CalcValue>;
export declare class Calc {
  readonly value: CalcValue;
  constructor(value: this['value']);
  toString(): string;
  static get parser(): TokenParser<Calc>;
}
