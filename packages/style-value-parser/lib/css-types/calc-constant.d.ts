/**
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *
 */

import { TokenParser } from '../token-parser';
export type CalcConstant = 'pi' | 'e' | 'infinity' | '-infinity' | 'NaN';
export declare const allCalcConstants: ReadonlyArray<CalcConstant>;
export declare const calcConstant: TokenParser<CalcConstant>;
